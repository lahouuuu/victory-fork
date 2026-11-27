# SESSIONS.md — Claude Code work log

Written ONLY by Claude Code sessions, append-only. The project-manager chat
reads it and never edits it. Never paste anything from the game here (see
CLAUDE.md, rule #1).

## Entry template (copy for each session, newest at the bottom)

```
## Session <id> — <short title> (<date>)
- Branch / start commit / end commit:
- Task given:
- What was done (files, switches added):
- Tests: fmt / clippy / clippy all-features / test / policy -> pass/fail
- Measurements (run numbers, scene, before -> after), or "none":
- What did NOT work or was taken back out:
- Open problems found (not fixed):
- Questions for the user / PM:
- Suggested next task:
```

---

<!-- Session entries start below this line. -->

## Session 1 — setup and baseline measurement on the user's PC (2026-11-26)
- Branch / start commit / end commit: main / 085ee7c / the commit that adds this file.
  Checked at start: main = origin/main = 085ee7c. upstream/main has 47 commits we do not
  have (newest fc63ce1); nothing merged.
- Task given: install the toolchain, check config, run the CLAUDE.md checks, release build,
  read the machine's specs and virtual memory, measure with the run 253 switch set. No code
  changes.
- What was done (files, switches added): no code or config file of the repo changed.
  Toolchain 1.98.1 was already installed. `cargo xtask install-hooks` run. `victory.toml`
  created at the repo root (git-ignored, all paths outside the repo; `victory config` OK,
  cache and data in the default %LOCALAPPDATA%\victory folders). `cargo build --release -p
  victory` OK. This commit adds CLAUDE.md, SESSIONS.md, PROMPTS.md, PM_HANDOFF.md (the last
  one as received).
- Tests: on 085ee7c as it is, without changes:
  - fmt: FAIL, 1,476 diffs in 69 files (lines over max_width = 100). CI does not run fmt.
  - clippy: FAIL, 8 errors, all in victory-gpu (dead code `placed_filler_twins`, field
    `depth_written` never read, manual `is_multiple_of` x3, collapsible `if`,
    `contains_key` + `insert`, `sort_unstable_by_key`). CI runs clippy, so it fails there too.
  - clippy all-features: FAIL, 10 errors of the same kinds in victory-gpu.
  - test: first run 2 FAILED in victory-gpu (`indirect::tests::warp_budget_saturates_...`,
    `inplace::tests::warp_a_title_buffer_in_place_...`, both on WARP); second run all pass
    (604 tests). Flaky, not investigated.
  - policy: pass (187 files).
  Not fixed: the user asked not to spend time on what does not help frame rate.
- Machine: AMD Ryzen 7 3700X (8 cores / 16 threads), NVIDIA RTX 3060 12 GB (driver 621.24),
  16 GB DDR4-3200, Windows 11 Pro build 26300. Shader cache on an NVMe SSD.
- Virtual memory: commit limit 56.9 GB = 16 GB RAM + page file on C: (SSD, 16 to 32 GB)
  + page file on a second disk (HDD, system managed, 25 GB). Peak page file use seen:
  2.3 GB on C:, 2.9 GB on the HDD. The limit was never approached; no change made or
  recommended (an earlier advice to enlarge it was withdrawn: the crashes are not 1455).
  Before run 3 the user closed other applications and rebooted: 15.8 GB committed, 9.4 GB
  free before launch (24 GB committed before runs 1-2).
- Command (PowerShell, from the repo root):
  `$env:VICTORY_LEVELS_ON_DEMAND='1'; $env:VICTORY_ASYNC_SUBMIT='1'; $env:VICTORY_QUEUE2='1';
  $env:VICTORY_INDIRECT_WINDOW='1'; $env:VICTORY_TEXTURE_NO_WAIT='1'; $env:VICTORY_MERGE_GAP='64';
  $env:VICTORY_MERGE_RUNS='4096'; $env:VICTORY_STAGING_POOL='0'; $env:VICTORY_COPY_PLAIN='1';
  $env:VICTORY_JOIN_PARTS='1'; $env:VICTORY_BACK_OWN_VIEW='1'; $env:VICTORY_INDIRECT_GATHER='1';
  $env:VICTORY_CARD_REHASH_RUNS='1'; .\target\release\victory.exe run <game dir>\<title exe> --execute`
  Sampled every 2 s from outside the process: window caption (fps, ms/frame, draws/frame),
  process and system commit, free RAM, the process's dedicated GPU memory.
- Measurements (frame rate from the window caption, 2 s samples):
  - Run 1 (empty shader cache, other apps open): menus 30 fps. Opening cutscene 1 to 3 fps
    (mean 1.7, 300 to 1,600 ms a frame), picture mostly black (draws left out while their
    pipelines are built), audio stopped, subtitles piled up. Crashed about 7 min into the
    cutscene. At the crash: process 18.8 GB committed, system 47 / 56.9 GB, free RAM 0.4 GB,
    GPU 6.6 GB.
  - Run 2 (cache partly filled, other apps open): never reached the cutscene; crashed during
    loading about 6 min after starting the game (936 s after launch). System 44.6 / 56.9 GB,
    free RAM 0.2 to 0.6 GB, GPU 4.9 GB.
  - Run 3 (after reboot, other apps closed, same command): no crash, closed by the user.
    Menus and loading 30 fps. Opening cutscene: about 35 min of wall time, mean 4.4 fps
    (0 to 16), median 314 ms a frame, median 1,227 draws a frame. First room: about 5 min,
    mean 1.1 fps (0 to 4), median 1,055 ms a frame (360 to 3,100), median 3,970 draws a
    frame (300 to 9,500); the player could move. The next scripted cutscene then started on
    its own. Peaks: process 23 GB committed, system 46.9 / 56.9 GB, free RAM down to 0.1 GB,
    GPU 9.4 GB. Shader translation was still going on in the room (the cache had never
    reached it before), so the room figure is a first-visit figure, not a warm one.
  - Against the README (RTX 4070, i7-14700KF, 32 GB): cutscene 5 to 8.5 fps there vs 4.4
    mean here (same range); first room 8 to 10 fps there vs about 1 fps here on a first
    visit. Not comparable until the room is measured with a warm cache.
- What did NOT work or was taken back out: nothing changed in the repo. Computer control of
  the game window was declined by the user, so the user drove the menus by hand.
- Open problems found (not fixed):
  - Two crashes in runs 1 and 2, both "unhandled exception 0xc0000005 ... write 0x0: nothing
    mapped there" in the title's own code (two nearby places; the second's stack passes through
    the first's), not in the layer. Not error 1455, no device removal. Free RAM was near 0
    both times, and run 3 (with about 8 GB more free at launch) did not crash: memory pressure
    is a possible cause, NOT proven. The title's crash reporter then tried to upload; the layer
    blocked it, as intended.
  - On a 16 GB PC free RAM sits at 0.1 to 1 GB in the cutscene and the room: heavy paging.
  - 47 commits of upstream to merge (stability, the water fix, the run 279 switch set).
  - fmt and clippy fail on 085ee7c (above); two WARP tests are flaky.
- Questions for the user / PM: none open.
- Suggested next task: Session 2, merge the 47 upstream commits, then measure the first
  room again with a warm cache (a second visit) with the same switch set, and with the run
  279 set.

## Session 2 — upstream integrated (squashed, without diagnostics/); measurements not reached (2026-11-26)
- Branch / start commit / end commit: main / 57e685d / bc5db4c (upstream) + the commit that
  adds this entry. Nothing pushed.
- Task given: bring in upstream/main, run the checks, build, then measure the first room with
  a warm cache: A = run 253 set, B = run 279 set, C = the better of the two + Series S profile.
- What was done (files, switches added):
  - At start upstream/main had 66 commits we did not have (988aa41), not 47 (fc63ce1) as the
    prompt said; the user chose all 66.
  - A first `git merge` (542c10c, local only) was undone with `git reset --keep 57e685d`:
    the run logs it brought under diagnostics/ (22 files) hold the title's internal file and
    folder names (about 57 lines a log) and, in 10 of them, a personal online identifier of
    upstream's tester. That fails rule #1 although `cargo xtask policy` passed. A real merge
    would put them in our history even if deleted afterwards.
  - PM decision (option 1): `git merge --squash 988aa41`, all of diagnostics/ removed from the
    index and the folder (its README.md too), one commit bc5db4c "Upstream up to 988aa41,
    squashed, without diagnostics/". `git ls-files diagnostics` is empty; main's history holds
    no upstream commit; the tree equals upstream/main apart from diagnostics/ and our 4 .md.
  - Upstream's changes to docs/asset-policy.md ("Run logs") and xtask/src/policy.rs that allow
    diagnostics/ are kept as they are (simpler future integrations).
  - No code of ours changed, no switch added.
- Tests (57e685d before -> bc5db4c after):
  - fmt: FAIL 1,476 diffs / 69 files -> FAIL 1,688 diffs / 73 files (upstream does not run fmt).
  - clippy: FAIL 8 errors -> PASS (fixed upstream in 97f8dbc).
  - clippy all-features: FAIL 10 errors -> PASS.
  - test: 2 flaky WARP tests -> 611 passed, 0 failed, 13 ignored, on two runs in a row.
  - policy: pass -> pass (190 files).
  - `cargo build --release -p victory`: OK.
- Switch sets:
  - A (run 253): `VICTORY_LEVELS_ON_DEMAND=1 VICTORY_ASYNC_SUBMIT=1 VICTORY_QUEUE2=1
    VICTORY_INDIRECT_WINDOW=1 VICTORY_TEXTURE_NO_WAIT=1 VICTORY_MERGE_GAP=64
    VICTORY_MERGE_RUNS=4096 VICTORY_STAGING_POOL=0 VICTORY_COPY_PLAIN=1 VICTORY_JOIN_PARTS=1
    VICTORY_BACK_OWN_VIEW=1 VICTORY_INDIRECT_GATHER=1 VICTORY_CARD_REHASH_RUNS=1`
  - B (run 279, read from upstream's diagnostics/README.md at 988aa41, read only): A plus
    `VICTORY_CARD_WINDOWS_MB=2048 VICTORY_CARD_BUFFERS_MIN_KB=64 VICTORY_CARD_BUFFERS_MB=1280
    VICTORY_TEXTURE_BACK=1 VICTORY_CARD_HASH_LEN=1` (TEXTURE_BACK = the water fix: present).
- Measurements: none of A, B or C was taken. Four warm-up launches with set A, none reached the
  first room (the merge changes the shader translator, and Session 1 had never warmed the room):
  1. After a fresh boot, other apps closed: opening cutscene 0 to 23 fps (mean 5.3 over 261
     samples), then about 10 min on a still screen (2 draws a frame, 30 fps) while the PC was
     left alone and possibly slept; closed normally, no crash.
  2. Ended by itself 16 s after start, while booting: no line in crashes.log, no Windows
     event, the layer's log stops mid-load with no error. Cause not known.
  3. The game loaded the last save by itself; about 15 min at 30 fps on a near-empty screen
     (10 to 13 draws), then crashed at the end of loading, 1,019 s after start: unhandled
     exception 0xc0000005, write to address 0 in the title's own code, the same place and
     stack as Session 1's second launch. Peaks: process 18 GB, system 39.9 / 57.9 GB, free RAM
     near 0, GPU 7.1 GB. Not error 1455.
  4. White screen at 30 fps for several minutes (the window caption lost its frame counter),
     then loading; ended at 805 s by a normal close ("did not finish suspending in time"),
     not a crash. Peaks: process 20.3 GB, system 42.0 / 57.9 GB, free RAM near 0, GPU 7.9 GB.
  Side by side with Session 1 (run 3, set A, before the merge): cutscene 4.4 fps mean there,
  5.3 here in launch 1; first room about 1 fps on a first visit there, not reached here.
- Recommended command: none measured. Set A stays the one used, unproven better or worse than B.
- What did NOT work or was taken back out: the real merge 542c10c (above).
- Open problems found (not fixed):
  - The load of the last save crashes in the title's own code (0xc0000005, write to 0): now 2
    of the 3 launches that loaded a save directly. Free RAM is near 0 each time on this 16 GB
    PC; memory pressure is a possible cause, not proven.
  - A silent exit 16 s after start with nothing logged.
  - The README (from upstream) points to diagnostics/README.md for the run 279 set and to run
    logs; that folder does not exist here.
  - diagnostics/ is left out here by PM decision, while docs/asset-policy.md and
    xtask/src/policy.rs (from upstream) allow it: a future integration must keep leaving it out.
  - fmt still fails (1,688 diffs).
- Questions for the user / PM: whether to keep trying to reach the room on main (each try is
  15 to 20 min and may crash at the save load) before comparing other bases.
- Suggested next task: measure A and B in the first room once a launch gets there (start a new
  game instead of loading the save, if the save load keeps crashing); then C (Series S).

## Session 3 — trial of the trustno1-coder/VIctory-NVIDIA base on the user's PC (2026-11-26)
- Branch / start commit / end commit: nvidia (new, local) / 033be27 of that repository /
  the commit that adds our 4 files with this entry. main untouched (040e82e = origin/main at
  start). Nothing pushed. Remote `nvidia` added; 033be27 is its newest commit (none newer).
- Task given: try that base on a separate branch: audit, checks, diag build, whether its
  resolution patches fit the user's copy, then measure in the first room (D = its
  `play-nvidia.ps1 -NoSmoothMotion`; later PM additions: E = D with three switches changed,
  F = the better of D/E at 480p; translate shaders ahead of time before E).
- What was done (files, switches added): no code changed. Our 4 files copied from main
  (PM_HANDOFF.md as it is) plus this entry. The scripts used for the runs live outside the repo.
- Audit, rule #1: `cargo xtask policy` passes (265 files). No logs, pictures or captures in the
  tree; one binary, the bcpack tables, already in main (from the original VIctory). No game file
  names, no personal identifiers (the save code's user id is a made-up constant). To note for
  the PM: their code, README and commit messages name the title about ten times, and the code
  quotes about 80 of the title's shader names (main already has 70 of the same kind, inherited).
- Audit, safety: `scripts/play-nvidia.ps1` downloads nothing and changes no system setting; it
  sets VICTORY_* in its own PowerShell session and, without `-NoSmoothMotion`, copies the
  build to `target\<build>\victory_sm.exe`. The `play-S-*` / `play-X-*` scripts only call it.
  The three build.rs only hash sources and pass link flags. No new crate in Cargo.lock. New
  process launches are tests or `victory.exe` starting itself (`warm`). The title's network
  stays blocked. Nothing doubtful found.
- Tests (on 033be27 + our files, nothing fixed):
  - fmt: FAIL, 2,788 diffs.
  - clippy and clippy all-features: FAIL, 38 errors in victory-gpu, victory-loader and
    victory-formats (dead code, manual `is_multiple_of` x9, `transmute` without annotations x4,
    operator precedence x2, and others).
  - test: 927 passed, 0 failed, 45 ignored.
  - policy: pass.
  - `cargo build --locked --profile diag -p victory --features diagnostics`: OK.
- Compatibility: the fork's patches apply only to the executable whose BLAKE3 starts with
  b99f876b74d4a77f (its "1.0.2.1"); the layer reports exactly that build hash for the user's
  copy, and at start logs the render-scale hooks and both watchdog hooks as applied. They fit.
- FINDING, the crashes of Sessions 1 and 2: the fork's `crates/victory-loader/src/hooks.rs`
  describes the title's watchdog: when its main thread has not advanced for about 30 one-second
  ticks it ends the process on purpose by writing to address 0. The two places it patches are
  the two exact places of all four "crashes" in our crashes.log. So those were the watchdog
  firing during long loads on this 16 GB PC, not faults. The fork's `VICTORY_NO_WATCHDOG=1`
  (opt-in, not set by `play-nvidia.ps1`) disarms it; with it, no launch of this session ended
  early. Used in every run below, with the user's approval.
- Shaders ahead of time (PM addition, before E): `target\diag\victory.exe shaders` (no paths:
  it walks the configured game folder) with exactly E's variables: 25,776 shaders in the cache,
  209 not translatable (198 containers with no program the translator takes, 11 unsupported
  forms), 558 s. The shader cache key holds the translator's own switches' values
  (VICTORY_BVH_MISS, VICTORY_NAN_SCRUB, ...); VICTORY_THIN is not one of them.
  `warm --used`: 0 pipelines known, 1 s: it needs a book written by a run with
  `VICTORY_PIPELINE_BOOK=1`, which the play script does not set. The host pipeline library
  (411 MB, about 2,500 started early) is what keeps pipelines between runs.
- Measurements: PC restarted or freshly booted and other apps closed before the warm-up, D and
  E (F ran in the same boot as E, 11.3 GB free at launch). Series S profile, output 1920x1080,
  world drawn at 960x540 (D, E) or 853x480 (F), then the title's own upscaler. Sampled every
  2 s from outside: window caption (fps, ms/frame, draws/frame), process and system commit,
  free RAM, the process's dedicated GPU memory. Hitches: the layer logs no per-frame times and
  PresentMon needs admin rights, so they are counted as the share of 2 s samples whose mean
  frame time is over 100 ms / over 300 ms: a proxy, not frame counts. Values used, checked in
  each run's log (the script applies `-More` after its own values):

  | | D | E | F |
  |---|---|---|---|
  | VICTORY_THIN | 1 (8 threads) | 0 | 0 |
  | VICTORY_PIPELINE_WAIT_MS | 10000 | 150 | 150 |
  | pipeline workers | 10 (default) | 4 | 4 |
  | render height | 540 | 540 | 480 |
  | room, mean fps | 5.1 | 6.0 | 6.9 |
  | room, median ms/frame (p90) | 182 (619) | 154 (269) | 123 (365) |
  | room, samples > 100 ms / > 300 ms | 99 % / 31 % | 100 % / 10 % | 96 % / 18 % |
  | room, median draws/frame | 5,265 | 5,950 | 4,683 |
  | cutscene or arrival, mean fps | 2.3 (124 samples over 2 s) | 9.4 (often 11-18) | 12.3 |
  | peak commit, process / system | 21.1 / 40.4 GB | 18.9 / 35.9 GB | 18.8 / 36.0 GB |
  | lowest free RAM | 0.11 GB | 0.20 GB | 0.05 GB |
  | peak GPU memory | 6.2 GB | 5.9 GB | 5.7 GB |
  | crash | no | no | no |

  Warm-up (D's settings, first visit): room 5.3 fps mean, 10 fps in light views. In E a frame
  of 146 ms was 74.5 ms preparing on the CPU, 41.7 ms on the GPU and 31.4 ms copying back;
  about 290 draws a frame were dropped without a host pipeline (none being made: the skipped
  ray-tracing programs) and about 5,600 textures waited to be made.
  Picture (the user): the same in D, E and F, "very ugly", 480p and 540p not told apart; walls
  and furniture appear late. In F, after the user left the game alone for a while, animations
  and lip movement looked right in conversations; whether that is the longer wait is not known.
  Caveats: E differs from D by three switches AND the shaders translated ahead of time, so the
  two effects are mixed. D's room window is my estimate (the user did not say when it began);
  E's and F's begin where the user said. Views differ (draw counts), so F's small gain over E
  is within what the view can explain: the frame is bound by the CPU, not by resolution.
- Compared with main (Sessions 1-2, run 253 set, no watchdog switch): room about 1 fps on a
  first visit (Session 1) and never reached in Session 2's four launches; cutscene 4.4 to
  5.3 fps. This base with E: room 6 fps, cutscene about 9 to 12 fps, every run reached the
  room without an early end. Clearly better on this PC, still far from playable.
- Recommended command (Series S, world at 540p, no Smooth Motion; branch nvidia, diag build):
  `.\scripts\play-nvidia.ps1 -Exe <title exe> -NoSmoothMotion -More @{VICTORY_NO_WATCHDOG='1';
  VICTORY_THIN='0'; VICTORY_PIPELINE_WAIT_MS='150'; VICTORY_PIPELINE_WORKERS='4'}`
- What did NOT work or was taken back out: `warm --used` (nothing recorded to warm); a
  PresentMon capture for per-frame times (needs admin; not pursued).
- Open problems found (not fixed):
  - Free RAM falls to 0.05-0.2 GB in every run: 16 GB is short for about 19-21 GB of commit.
  - Textures and props arrive late (thousands of textures waiting); cause not looked into.
  - Series S with the world at 1080p (`play-S-1080p`) stops at start per that base's README
    (on 12 GB cards too). The user wants a sharp 1080p picture: 720p is the next thing to test.
  - main has no watchdog switch: its "crashes" will go on there.
  - fmt and clippy fail on this base (above).
- Questions for the user / PM: whether to continue on this base (its history is another
  fork's, 633 commits) or bring its watchdog switch and E's settings to main.
- Suggested next task: measure E's set at 720p (`-Resolution 720`) in the same place, and record
  a pipeline book (`VICTORY_PIPELINE_BOOK=1`) so `warm --used` can work; then decide the base.

## Session 4 — avatar-nvidia: avatarmanz 2de99801 + trustno1 033be27 squashed, measured against nvidia (2026-11-26)
- Branch / start commit / end commit: avatar-nvidia (new, local, from nvidia 20537b5) / 20537b5 /
  the commit that adds this entry. Commits: ceb95a3 (the squash, one parent 20537b5), 4547204
  (tests and notes after the squash), then this entry with PM_HANDOFF.md as received. main
  (040e82e) and nvidia (20537b5) not moved. Nothing pushed.
- Task given: rebuild the NVIDIA fixes on avatarmanz/VIctory-7900XTX frozen at 2de99801 without
  its history (PM option 2: one squashed commit, conflicts settled for the NVIDIA fixes, each one
  noted), PORT_* off by default, audit before building, checks no worse than Session 3, diag
  build, then measure in the first room: G = nvidia build, H = avatar-nvidia, I = H + the five
  PORT_*, J = the better at 720p; all with Session 3's recommended set and
  VICTORY_PIPELINE_BOOK=1. Time limit 4 h for the integration (the PM later allowed more for the
  measurements).
- Why not a cherry-pick: the PORT_* commits (86dc109, 4b82f6c, 9c23e8d, 9997d25, 4cb90be,
  3920dc2, 9d723e8 and its fixes, 673be0e, a609a2f, a39509f) build on about 60 earlier avatarmanz
  commits; cherry-picking them alone did not apply. trustno1 forked avatarmanz at 0a43566 and
  added 13 commits; those 13 were replayed on 2de99801.
- What was done (files, switches added): the tree of avatarmanz 2de99801 with trustno1's 13
  commits replayed on it, committed as one commit on top of 20537b5; our 4 .md files kept as on
  nvidia. No file removed. Conflicts and what was kept:
  - Interface plane (trustno1 7997a20): avatarmanz's own two-plane compose kept (compose.rs,
    queue.rs, host.rs; `VICTORY_PLANES`, on by default); trustno1's compose plumbing and its
    duplicate module dropped. The `VICTORY_BVH_MISS` shader-cache key kept. README item 9 now
    says so: the switch `play-nvidia.ps1` sets for the plane is not read on this tree.
  - Loader (afd84de): both module lists kept. Render scale: `VICTORY_RENDER_SCALE` is applied by
    NVIDIA's hooks.rs (with `VICTORY_OWN_UPSCALER`); avatarmanz's own branch in resolution.rs
    for the same switch removed, since both patch the same upscaler (comment `LOCAL
    (avatar-nvidia)` there). VICTORY_NO_WATCHDOG: NVIDIA's (avatarmanz has none).
  - 77e13a9 (faces, ray tracing off, picture fixes): ambient.rs keeps both helpers (avatarmanz's
    pacing and NVIDIA's `most()`); shader cache decode reads NVIDIA's half views; pipeline/make.rs
    keeps NVIDIA's single wait; emit/lower.rs adds NVIDIA's `nan_scrub()` (VICTORY_NAN_SCRUB:
    NVIDIA's, avatarmanz has none); emit exports are the union; descriptor/host.rs NVIDIA's
    VICTORY_FAIR_DUE doc, avatarmanz's view-address write; draw.rs takes avatarmanz's TablePlan
    plus NVIDIA's half-view loop, NVIDIA's "tables past the host heap" loop dropped (avatarmanz's
    snapshot check covers it), relook pacing NVIDIA's `VICTORY_RELOOK_MS` with avatarmanz's
    frame-time pacing when it is known.
  - README conflicts: trustno1's text. d3cd650 and 033be27 applied cleanly.
  - 4547204: two test shaders and the tables test updated to the merged signatures; the
    switch-doc check (xtask) skips CLAUDE.md, SESSIONS.md, PROMPTS.md, PM_HANDOFF.md.
  - PORT_* switches (VICTORY_PORT_CONSTANTS, _DESCRIPTORS, _CBV, _GUARD, _JOURNAL): off unless
    set, as in avatarmanz. Nothing new added by this session.
- Audit, rule #1 (before building): `cargo xtask policy` passes. avatarmanz 2de99801 adds no
  logs, images, captures, IDs or machine paths; no file removed. The bcpack tables binary is
  the one already in main. It adds two more of the title's shader names and two mentions of
  the title's name, of the same kind as the base already has (Session 3). Safety: no new
  build.rs, no new crate; new process launches are tests or the `warm` workers; the title's
  network stays blocked.
- Tests (on 4547204 = this tree's code):
  - fmt: FAIL, 3,179 diffs (Session 3: 2,788; avatarmanz 2de99801 alone: 3,159). Not fixed.
  - clippy: FAIL, 3 errors (Session 3: 38): `transmute` without annotations x2 in victory-loader descriptor_sites.rs, an unneeded `&mut` in victory-gpu watch.rs. Clippy stops at crates that fail, so further lints may be hidden. Not fixed.
  - clippy all-features: FAIL, the same 3 errors.
  - test: pass, 1,039 passed, 0 failed (Session 3: 927). The port-7 test flaky once earlier (below).
  - policy: pass, 283 files.
  - `cargo build --locked --profile diag -p victory --features diagnostics`: OK.
- Shaders ahead of time (avatar build, its own cache folder via VICTORY_CACHE_DIR so nvidia's
  stays as it was for G): 25,787 translated, 198 not translatable, 662 s with PORT_* off, 886 s
  with them on (nvidia, Session 3: 25,776 / 209 / 558 s). Pipelines ahead of time: G's book
  (83,559 title pipelines, 2,533 drawn with) copied to the avatar cache, `warm --used`: 2,533
  made, 7 without a host pipeline, none ended its process; 429 s for H's variables, 17 s for
  I's (the driver had most already).
- Conditions: the game folder was MOVED from the HDD to the SSD just before G (the user's
  choice: the HDD was at 100 % during scans); G, G2, H, H2, I all read it from the SSD. One boot
  for all runs (G was its first launch; the user did not restart between runs after that).
  Task Manager was open beside the game during G only. Page file: 16-32 GB on the SSD plus
  system-managed 26 GB on the HDD; not changed (PM). Every run: `play-nvidia.ps1 -NoSmoothMotion`
  (Series S, output 1920x1080, world at 960x540) plus VICTORY_NO_WATCHDOG=1, VICTORY_THIN=0,
  VICTORY_PIPELINE_WAIT_MS=150, VICTORY_PIPELINE_WORKERS=4, VICTORY_PIPELINE_BOOK=1. From G2 on,
  also sampled (read only): CPU of the Windows antivirus and of the indexer, physical disk
  reads and busy time. Room window: 3 min from when the user said they entered (G2: from when
  the picture settled; the user did not say and later went on to the next cutscene). The user
  moved about in G2, H and H2 and stood still in I.
- Measurements (first room, 3 min each; frame split = median of the layer's own frame lines):

  | | G (nvidia, first launch) | G2 (nvidia) | H (avatar, first launch) | H2 (avatar) | I (avatar + 5 PORT_*) |
  |---|---|---|---|---|---|
  | mean fps | 0.86 | 7.2 | 0.63 | 2.0 | 2.9 |
  | median ms/frame (p90) | 2,656 (6,925) | 113 (609) | 2,030 (9,727) | 441 (817) | 335 (444) |
  | samples > 300 ms / > 1 s | 95 % / 71 % | 18 % / 3 % | 100 % / 81 % | 100 % / 6 % | 90 % / 1 % |
  | median draws/frame | 5,143 | 4,558 | 5,833 | 5,095 | 4,625 |
  | frame split: CPU preparing / GPU / copying back (ms) | 300-360 / 0-270 / 21-95 | 64 / ~0 / 32 | 460 / 0 / 32 | 76 / 0 / 23 | 58 / 0 / 26 |
  | victory CPU, share of the machine | - | 36 % | 29 % | 13 % | 15 % |
  | lowest / mean free RAM | 0.32 / 0.75 GB | 1.11 / 1.40 GB | 0.58 / 1.13 GB | 0.49 / 0.96 GB | 1.19 / 1.76 GB |
  | peak commit, process / system | 19.4 / 36.4 GB | 19.1 / 37.0 GB | 23.1 / 41.3 GB | 21.8 / 39.7 GB | 20.0 / 38.1 GB |
  | peak GPU memory | 5.9 GB | 5.6 GB | 8.4 GB | 8.1 GB | 6.8 GB |
  | antivirus CPU mean / max | - | 0.1 / 3.5 % | 0.4 / 4.1 % | 0.3 / 3.1 % | 0.4 / 5.8 % |
  | indexer CPU max | - | 0.3 % | 0.3 % | 0.3 % | 0.3 % |
  | HDD (holds a page file) busy | - | 98 % | 100 % | 100 % | 77 % |
  | crash | no (stopped by me) | no | no | no | no |

  - G is set aside (PM): first launch after the move. G2, same build and command, is back at
    Session 3's E (6.0 fps, 154 ms; split 75 / 42 / 31). G2 is the reference.
  - H was a cold start for the avatar build: "no host pipeline library yet", 88 pipelines still
    being made in the room (`warm` fills the driver's cache, not the layer's library). H2 is
    the same run with the library H saved (208 MB; nvidia's is 438 MB). I ran third and loaded
    232 MB; 3 of its frame lines over the whole run had pipelines being made.
  - H2 and I: the layer's own parts of a frame add up to about 100 ms, the frame takes 330-450
    ms, and the process uses about a third of the CPU G2 uses: it waits for something the frame
    line does not count. More memory (H2: +2.7 GB process, +2.4 GB GPU over G2) on a PC that
    already pages to an HDD is one candidate; a wait in avatarmanz's code is another. Not looked
    into (one task per session).
  - I vs H2: better (2.9 vs 2.0 fps, p90 444 vs 817 ms), but the user stood still in I. No crash.
    The log has 1,301 warnings from the descriptor port (a view record in the title's heap it
    could not match); none in H2.
  - Picture (the user): in I, as usual. In H2 and several other runs (which ones not said), a
    character's arms were drawn without textures, and while walking textures in the room
    sometimes stretched wrongly. Cutscenes do not play right, at times not at all. Not tied to
    one build by these runs.
  - J (720p) not run: the PM's condition was "only if H/I go well"; both are far below G2.
  - During G2's loading the HDD was busy 65 % with the game not reading from it, and at 98-100 %
    in the room. Page file peaks since boot: 2.1 GB on the SSD, 4.2 GB on the HDD. Free space
    on C: (read only, for the PM): 269 GB.
- Recommended command: unchanged from Session 3, on branch nvidia (diag build): G2 is the best
  measured. avatar-nvidia is not recommended for playing.
- What did NOT work or was taken back out: a direct cherry-pick of the PORT_* commits (above).
  `warm --used` does not fill the layer's host pipeline library, so the first launch of a build
  on a new cache is still cold (H).
- Open problems found (not fixed):
  - avatar-nvidia: frames 3-4x slower than nvidia with most of the time outside what the frame
    line counts; more process and GPU memory.
  - The first launch after moving the game, or of a build on a new cache, is 7-10x slower in the
    room than the second (G, H).
  - The page file on the HDD is busy at 77-100 % during play.
  - fmt fails on the merged base (3,159 diffs in avatarmanz alone); clippy, see above. The test
    `watch::port7_tests::no_write_is_lost_with_port_7` failed once in a full run and passed 3 of
    3 alone (flaky).
  - The interface-plane switch `play-nvidia.ps1` sets has no effect on avatar-nvidia.
- Questions for the user / PM: keep avatar-nvidia as a branch to study, or go back to nvidia
  as the working base?
- Suggested next task: on nvidia, the page file entirely on the SSD (the PM's next session),
  then G2's run again, then J (720p) on nvidia.

## Session 5 — nvidia remeasured: page file on the SSD, 720p, -Quality, texture staging (2026-11-27)
- Branch / start commit / end commit: nvidia / 20537b5 (= origin/nvidia) / the commit that
  adds this entry. Switched from avatar-nvidia (5649892) keeping the PM's PM_HANDOFF.md
  byte for byte. avatar-nvidia and main not moved. Nothing pushed.
- Task given: no code change; on nvidia's diag build with Session 3's recommended command and
  VICTORY_PIPELINE_BOOK=1, measure in the first room: K = as is (vs Session 4's G2), L = K at
  720p, M = the better + `-Quality`, N = the best + fxceless/victory-fork#1268's texture set.
- What was done (files, switches added): no code changed. The Session 4 entry copied as it is
  from avatar-nvidia 5649892 (SESSIONS.md now equals that commit's, plus this entry). The diag
  build was rebuilt (target held avatar-nvidia's): same size as Session 3's, 14,238,720 bytes.
  nvidia's own cache (shaders, 438 MB host pipeline library) was warm and untouched since G2.
- Start state checked (read only): page file only on C: (SSD), 32,768-49,152 MB, in force
  since the 00:14 boot; commit limit 47.9 GB (Session 4: 57.9). The user did not restart
  between runs (one boot for K, L, M, N); about 15-17 GB committed before each launch.
- What the texture set does (read in crates/victory-gpu/src/draw.rs before N):
  `VICTORY_TEXTURE_STAGE_MB=512` = bytes of textures made per frame (default on this 12 GB card:
  192 MB); `VICTORY_TEXTURE_STAGE_MS=50` = time a frame's texture pass may take (default 250
  ms); `VICTORY_TEXTURE_CATCHUP=1` = the default already (with 1,024 or more due, both limits
  x3). So N stages more bytes per frame but stalls a frame for much less time.
- `-Quality` values (M), checked: `play-nvidia.ps1` sets VICTORY_RELAY=1, VICTORY_CARD_STEPS=0,
  VICTORY_TEXTURE_NO_WAIT=0, and `-More` (our five switches) does not touch them. The log
  confirms the relay stage: its relay line reads `exact=false` in M (stage 1) and `exact=true`
  in G2, K, L, N (stage 2, VICTORY_RELAY=2; that line names "VICTORY_RELAY=1" in every run, it
  is the line's fixed text). CARD_STEPS and TEXTURE_NO_WAIT have no log line: taken from the
  script.
- Tests (20537b5, this tree's code; same as Session 3):
  - fmt: FAIL, 2,788 diffs (Session 3: 2,788).
  - clippy: FAIL, 38 errors counted with `--keep-going` (Session 3: 38); a plain run stops at
    the first failing crates and shows 5. clippy all-features: FAIL, the same 5 in a plain run.
  - test: 927 passed, 0 failed (Session 3: 927).
  - policy: pass (265 files).
- Measurements (first room, 3 min from when the user said they entered; Series S profile,
  output 1920x1080; frame split = median of the layer's own frame lines; "textures waiting" =
  the layer's count at the first and last frame line of the window; sampled read only every
  2 s like Session 4, with the antivirus, indexer and disks):

  | | G2 (S4, page file on HDD+SSD) | K | L (720p) | M (-Quality) | N (K + textures) |
  |---|---|---|---|---|---|
  | mean fps | 7.2 | **7.6** | 5.9 | 6.0 | 7.3 |
  | median ms/frame (p90) | 113 (609) | 131 (**160**) | 153 (404) | 160 (258) | 131 (260) |
  | samples > 300 ms / > 1 s | 18 % / 3 % | **2 % / 0 %** | 11 % / 6 % | 6 % / 0 % | 6 % / 3 % |
  | median draws/frame | 4,558 | 5,047 | 5,282 | 4,587 | 4,784 |
  | split CPU preparing / GPU / copying back (ms) | 64 / ~0 / 32 | 76 / 0 / 35 | 89 / 1.5 / 38 | 61 / **85** / 27 | 75 / 0 / 33 |
  | lowest / mean free RAM | 1.11 / 1.40 GB | 0.34 / 0.89 | 0.16 / 0.78 | 0.25 / 0.73 | 0.32 / 0.76 |
  | peak commit, process / system (limit) | 19.1 / 37.0 (57.9) | 21.9 / 42.6 (47.9) | 22.9 / **44.3** | 21.3 / 42.6 | 21.8 / 42.3 |
  | peak GPU memory | 5.6 GB | 7.7 | 8.3 | 7.3 | 7.6 |
  | HDD busy / SSD busy | 98 % / 17 % | **0 %** / 62 % | 0 % / 89 % | 0 % / 71 % | 0 % / 65 % |
  | textures waiting in the room, first -> last | 15,413 -> 1,653 | 16,127 -> 2,996 | 16,623 -> 3,794 | 13,591 -> 4,420 | 13,534 -> **1,539** |
  | black loading screen | - | 40 s | 40 s | 49 s | 54 s |
  | arrival cutscene: mean fps, median ms, > 1 s | - | 3.2, 465, 29 % | 3.2, 335, 4 % | skipped by the user | **4.2, 255, 0 %** |
  | arrival cutscene: textures waiting, first -> last | - | 18,262 -> 16,429 (peak 21,897) | 17,487 -> 17,126 | - | 18,837 -> 14,238 |
  | launch to room | > 8 min | about 3 min 20 | 2 min 40 | 2 min 30 | 2 min 40 |
  | crash / freeze | no | no | no | no | no |

  Antivirus 0.7-1.1 % mean, at most 8 %; indexer at most 1 %: not a factor. The system commit
  came within 3.6 GB of the new 47.9 GB limit (L) but never reached it; no freeze was seen.
- Reading the table:
  - K vs G2: about the same mean fps, but long frames almost gone (p90 609 -> 160 ms, > 300 ms
    18 % -> 2 %), the HDD idle, the room reached in a third of the time. Two things changed at
    once (page file only on the SSD, and another boot with 2-3 GB more committed at start), so
    the gain is not given to the page file alone. More memory used (process +2.8 GB, GPU +2 GB)
    with the same build and settings: not explained.
  - L (720p): -1.7 fps, more hitches, the highest memory. The user: clearly sharper than K.
    Better to the eye but not kept for now (fps, memory); to try again once memory is settled.
  - M (-Quality): -1.6 fps; the GPU part grows from 0 to 85 ms a frame (the submitting thread
    waits for the card, as the script's comment says). Stretched bodies and clothes: REDUCED,
    NOT GONE (the user); worse the more characters are on screen (a scene with many characters
    in one room stretches most). A character's arms again drawn in the wrong colour. Kept
    values: VICTORY_RELAY=1, VICTORY_CARD_STEPS=0, VICTORY_TEXTURE_NO_WAIT=0 (above). For
    Session 6, which is to fix this in code.
  - N's base: the PM's rule was M if it lost 1.5 fps or less against K, else K. M lost 1.58
    (7.63 -> 6.05): K was taken, by a small margin.
  - N vs K: in the room about the same (-0.35 fps, one run each; more hitches: > 300 ms 6 % vs
    2 %, p90 260 vs 160), textures drain faster (1,539 left after 3 min vs 2,996). In the arrival
    cutscene clearly smoother (4.2 vs 3.2 fps, no frame over 1 s vs 29 %). The cutscene started
    on time (the user); textures of buildings and eyes in it still came late. In the room, the
    user saw nothing arriving late.
- Picture, the user (all runs at 540p unless said): blurry and grainy, textures messy (K); the
  edge of a window visible through the user's character that should hide it (K); bodies and
  clothes stretched (all; less with -Quality); a character's arms untextured or wrongly
  coloured (M, and earlier in Session 4); textures stretching and the character's edges
  merging with the background at times, mostly when the camera moves (N).
- Recommended command (branch nvidia, diag build): K, unchanged from Session 3:
  `.\scripts\play-nvidia.ps1 -Exe <title exe> -NoSmoothMotion -More @{VICTORY_NO_WATCHDOG='1';
  VICTORY_THIN='0'; VICTORY_PIPELINE_WAIT_MS='150'; VICTORY_PIPELINE_WORKERS='4'}`
  It has the most fps and the fewest long frames measured. What it gives up: a sharper picture
  (720p: -1.7 fps), fewer stretched bodies (-Quality: -1.6 fps), smoother cutscenes and faster
  texture arrival (N's set: -0.35 fps in the room, more hitches there). Add
  `VICTORY_TEXTURE_STAGE_MB='512'; VICTORY_TEXTURE_STAGE_MS='50'` inside `-More` to prefer
  cutscenes; one run each, the room difference is within what one run can tell.
- What did NOT work or was taken back out: nothing taken out. M's arrival cutscene not
  measured (skipped by the user).
- Open problems found (not fixed):
  - Stretched bodies and clothes, worse with many characters; -Quality only reduces it.
  - A character's arms untextured or wrongly coloured; a window edge seen through the user's
    character; character edges merging with the background when the camera moves.
  - Arrival cutscene: 14,000-17,000 textures still waiting at its end in every run.
  - The commit limit dropped from 57.9 to 47.9 GB with the new page file; L came within 3.6 GB.
  - Same build and settings use about 2-3 GB more process and GPU memory than in Session 4.
  - fmt and clippy fail on this base (above, as in Session 3).
- Questions for the user / PM: whether to raise the page file's maximum (L's margin), and
  whether to take N's texture set into the recommended command for smoother cutscenes.
- Suggested next task: Session 6 as planned (the stretched bodies in code, starting from what
  VICTORY_RELAY=1 / VICTORY_CARD_STEPS=0 change); then L (720p) again once memory is settled.
