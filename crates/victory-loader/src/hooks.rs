//! Opt-in compatibility hooks for known title builds.
//!
//! LOCAL (nvidia2): the title watchdog switch, ported from our branch nvidia
//! (trustno1-coder/VIctory-NVIDIA 033be27), which copied it from
//! vitutu1/victory-perf `crates/victory-loader/src/hooks.rs`, itself a port
//! from ZigZagBoi1/ScarlettPort (`runtime/victory`, GPL-3.0-or-later). The
//! render-scale hooks that file also had are not ported.
//!
//! Only per-build offsets and short byte signatures live here (asset policy:
//! nothing of the title itself). A hook applies only to the build whose hash
//! it names, and only where the bytes are exactly the ones it replaces.

use windows_sys::Win32::System::Diagnostics::Debug::FlushInstructionCache;
use windows_sys::Win32::System::Memory::{PAGE_EXECUTE_READWRITE, VirtualProtect};
use windows_sys::Win32::System::Threading::GetCurrentProcess;

/// First 16 hex digits of the BLAKE3 of the 1.0.2.1 executable.
const BUILD_1_0_2_1: &str = "b99f876b74d4a77f";

/// `VICTORY_NO_WATCHDOG=1` (off by default): disarm the title's watchdog.
///
/// On a PC where a load keeps the title's main thread from advancing for
/// about 30 s (16 GB of RAM, paging), the watchdog ends the process on
/// purpose by writing to address 0: an access violation at the two places
/// patched below, which earlier runs logged as crashes. With the switch the
/// title waits instead. Only `1` turns it on (the nvidia branch took any
/// value).
pub(crate) const NO_WATCHDOG_ENV: &str = "VICTORY_NO_WATCHDOG";

/// Whether [`NO_WATCHDOG_ENV`] asks for the watchdog to be disarmed.
pub(crate) fn no_watchdog() -> bool {
    std::env::var_os(NO_WATCHDOG_ENV).is_some_and(|v| v == "1")
}

/// One code patch: `rva` must hold exactly `expect` before `new` is written.
struct Patch<'a> {
    what: &'static str,
    rva: usize,
    expect: &'a [u8],
    new: &'a [u8],
}

/// The title's watchdog thread ends the process on purpose when its main
/// thread has not advanced for about 30 one-second ticks. Two places do it:
/// the handler it calls (`mov dword ptr [0], imm32`) and, when the handler
/// returns or is unset, a fallback right after the call (`mov dword ptr [0],
/// 0; int3`). Jumping over the whole block returns the loop to its normal
/// path; the second patch makes the handler harmless if something else calls
/// it.
const WATCHDOG: &[Patch<'static>] = &[
    Patch {
        what: "watchdog: skip the call and the fallback fault",
        rva: 0x2215c27,
        // mov rax, [rip + disp32] (the handler pointer)
        expect: &[0x48, 0x8B, 0x05, 0x1A, 0x07, 0xFA, 0x04],
        // jmp +0x18 (to the loop's normal continuation), then NOPs
        new: &[0xEB, 0x18, 0x90, 0x90, 0x90, 0x90, 0x90],
    },
    Patch {
        what: "watchdog: handler's deliberate fault",
        rva: 0x23cf683,
        expect: &[0xC7, 0x04, 0x25, 0, 0, 0, 0, 0x21, 0xD4, 0x8B, 0x4D],
        // an eleven-byte NOP
        new: &[0x66, 0x66, 0x2E, 0x0F, 0x1F, 0x84, 0, 0, 0, 0, 0],
    },
];

/// Whether `hash` is the build the patches were made for.
fn known_build(hash: &blake3::Hash) -> bool {
    hash.to_hex().starts_with(BUILD_1_0_2_1)
}

/// Writes `p.new` over `bytes` if they are exactly `p.expect`.
fn patch_bytes(bytes: &mut [u8], p: &Patch<'_>) -> bool {
    if bytes != p.expect || p.new.len() != p.expect.len() {
        return false;
    }
    bytes.copy_from_slice(p.new);
    true
}

/// Applies the watchdog patches whose bytes match. Returns how many applied;
/// for another build, says so and patches nothing.
///
/// # Safety
/// `base` is the mapped image whose original BLAKE3 is `hash`, and no title
/// code is running yet.
pub(crate) unsafe fn disarm_watchdog(base: usize, hash: &blake3::Hash) -> usize {
    if !known_build(hash) {
        tracing::warn!(
            build = &hash.to_hex()[..16],
            expected = BUILD_1_0_2_1,
            "{NO_WATCHDOG_ENV}: not the build the watchdog patches are for; nothing patched"
        );
        return 0;
    }
    WATCHDOG
        .iter()
        // SAFETY: forwarded from this function's contract.
        .filter(|p| unsafe { apply(base, p) })
        .count()
}

/// # Safety
/// See [`disarm_watchdog`].
unsafe fn apply(base: usize, p: &Patch<'_>) -> bool {
    let at = (base + p.rva) as *mut u8;
    let len = p.expect.len();
    let mut old = 0;
    // SAFETY: the offset is inside the mapped .text of this exact build;
    // the protection of those bytes only, restored below.
    if unsafe { VirtualProtect(at as _, len, PAGE_EXECUTE_READWRITE, &mut old) } == 0 {
        tracing::warn!(
            what = p.what,
            "hook skipped: the bytes could not be made writable"
        );
        return false;
    }
    // SAFETY: writable now, inside the image, and no title code runs.
    let done = patch_bytes(unsafe { std::slice::from_raw_parts_mut(at, len) }, p);
    // SAFETY: as above.
    unsafe {
        VirtualProtect(at as _, len, old, &mut old);
        FlushInstructionCache(GetCurrentProcess(), at as _, len);
    }
    if done {
        tracing::info!(what = p.what, "hook applied");
    } else {
        tracing::warn!(
            what = p.what,
            "hook skipped: the bytes are not the expected ones"
        );
    }
    done
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Synthetic text: each patch's expected bytes between filler bytes of
    /// our own; the patch applies there, leaves the filler, and only once.
    #[test]
    fn the_patches_apply_where_their_bytes_are() {
        for p in WATCHDOG {
            let mut text = vec![0xCCu8; p.expect.len() + 16];
            let at = 8;
            text[at..at + p.expect.len()].copy_from_slice(p.expect);
            assert!(
                patch_bytes(&mut text[at..at + p.expect.len()], p),
                "{}",
                p.what
            );
            assert_eq!(&text[at..at + p.new.len()], p.new);
            assert!(
                text[..at]
                    .iter()
                    .chain(&text[at + p.new.len()..])
                    .all(|&b| b == 0xCC)
            );
            // The bytes are no longer the expected ones: a second time does nothing.
            assert!(!patch_bytes(&mut text[at..at + p.expect.len()], p));
        }
    }

    /// Other bytes (another build, or code moved) are left as they are.
    #[test]
    fn other_bytes_are_not_touched() {
        for p in WATCHDOG {
            let mut bytes = vec![0x90u8; p.expect.len()];
            let before = bytes.clone();
            assert!(!patch_bytes(&mut bytes, p));
            assert_eq!(bytes, before);
            assert_eq!(
                p.new.len(),
                p.expect.len(),
                "{}: a patch keeps the length",
                p.what
            );
        }
    }

    /// Only the 1.0.2.1 build is patched.
    #[test]
    fn only_the_known_build_is_patched() {
        assert!(!known_build(&blake3::hash(b"not the title")));
        let hex = BUILD_1_0_2_1.to_string() + &"0".repeat(48);
        assert!(known_build(&blake3::Hash::from_hex(hex).unwrap()));
    }

    /// The switch is off unless it says `1`.
    #[test]
    fn the_switch_reads_only_one() {
        // (Other tests do not read this variable.)
        // SAFETY: set and removed in this test alone.
        unsafe { std::env::remove_var(NO_WATCHDOG_ENV) };
        assert!(!no_watchdog());
        unsafe { std::env::set_var(NO_WATCHDOG_ENV, "0") };
        assert!(!no_watchdog());
        unsafe { std::env::set_var(NO_WATCHDOG_ENV, "1") };
        assert!(no_watchdog());
        unsafe { std::env::remove_var(NO_WATCHDOG_ENV) };
    }
}
