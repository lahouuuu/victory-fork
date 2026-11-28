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

## Session 6 — stretched bodies: diagnosed with VICTORY_UPCOPY_CHECK, VICTORY_FETCH_CHECK measured; cause not found (2026-11-27)
- Branch / start commit / end commit: nvidia / 5f4eb4b (= origin/nvidia) / the commit that
  adds this entry. No code changed. Nothing pushed.
- Task given: remove the stretched bodies and clothes at the least fps cost. PM decisions on
  the way: an existing fix before new code; a diagnostic run first (VICTORY_UPCOPY_CHECK=1),
  then P (base command) and Q (P + the switch the diagnostic names); Q2 (VICTORY_RELAY_COPY=0)
  and new code only if Q fails. After Q the PM stopped the measurements (fps first): no Q2, no
  new code.
- Page file: the start check found the OLD setting (C: 32,768-49,152 MB, commit limit 47.9 GB),
  and it was still so after a first restart (04:50). After the PM set it again and restarted
  (04:56): C: only, 49,152-65,536 MB, 49,152 MB in use, commit limit 63.9 GB. Read only.
  P and Q ran on that boot; the diagnostic runs (D, D2) on the old setting.
- What is known of the cause, read in the code (crates/victory-gpu):
  - The relay's second stage (`relay_exact`, host.rs) makes a read of title memory wait only
    for flights that DECLARED writes there; the title's 174 MB heap is declared written as a
    whole (host.rs, `fill_is_titles`), so CPU reads there do wait.
  - Shaders that store through views they pick from the heap by a computed index (37 compute
    programs; the skinning passes write so, card_steps.rs) write buffers nobody declared: the
    result stays on the card and a later refresh of its 64 KiB sends older bytes over it
    (picked.rs). `VICTORY_PICKED_WRITES=1|2` (off) declares buffers found written that way (2:
    lands them too); off by default: hashing a 32 MB slice a frame plus the buffers found, and
    what is found late stays on the card until the next declared write.
  - With `VICTORY_RELAY_COPY=1` (set by play-nvidia.ps1) the fetch of card copies runs on the
    copy queue beside later submissions, so a piece can come down half written; when its
    compare does not flag it again the CPU side keeps those bytes until a refresh sends them
    up (host/torn.rs). `VICTORY_FETCH_CHECK=1` (off) hashes every fetched piece and fetches
    again a piece that is not what its flight ended with. Off by default "until a run has
    shown how often a piece is caught and what the hashes cost" (3-4 ms for 96 MB, on the
    threads that land). It does nothing without RELAY_COPY=1.
  - `VICTORY_RELAY_COPY=0` (not run): fetches go over the drawing queue behind what was sent
    meanwhile; its comment measures 48 ms of a 120 ms frame for the landing thread then.
  - `VICTORY_UPCOPY_CHECK=1` (diagnostics): compares what goes up to card copies with what the
    card holds, and at rest whole copies; each finding names why (never declared there, or
    declared and last fetched N frames ago). Its own comment: not for timed runs.
  - Searched for an existing fix: avatarmanz issue 13 and fxceless issue 89 (read on GitHub):
    nobody reports it fully fixed; one tester reports `VICTORY_FETCH_CHECK=1` with RELAY=2
    fixes it on Linux; the usual workaround is RELAY=1 CARD_STEPS=0 (Session 5's -Quality).
- Diagnostic (base command + VICTORY_UPCOPY_CHECK=1, not timed; first room window estimated
  from the picture settling, the room with many characters from when the user said):

  | D2 | first room (110-220 s, est.) | corridor | room with many characters (180 s) |
  |---|---|---|---|
  | sends that put older values over the card's | 315 | 1,624 | 4,083 |
  | words overwritten | 1.7 M | 8.6 M | 26.3 M |
  | cause, of the 590 sends the log names | 100 % "declared; last fetched 0-1 frames ago" | same | same |
  | cause "never declared / no shader declared" | 0 | 0 | 0 |

  Most of them in one 25 MB buffer. At rest, copies whose card side differs held title memory
  equal to the CPU side (the card had newer bytes). About 10x more in the room with many
  characters than in the first room, as the user sees. So the diagnostic named
  VICTORY_FETCH_CHECK (cause 2), not PICKED_WRITES.
- Crashes (not seen in Sessions 3-5; both in the same code: a list's prelude, the copies of
  depth textures, `Host::submit` host.rs:5605 / `prelude()`):
  - D (first diagnostic launch, 04:28, about 140 s in, during the arrival cutscene):
    unhandled exception 0xc0000005 in the NVIDIA driver's user-mode DLL (a read at 0x36),
    called from `Host::submit` (frames named from our build's PDB). The title's own crash
    reporter then tried to upload a report; the layer blocked the connection. D2, same
    settings, cutscene skipped: no crash.
  - P (first launch, about 129 s in, at the end of the arrival cutscene, before the user took
    control): "host GPU failure: no host list for a state prelude", the device refused a
    command list with E_OUTOFMEMORY (0x8007000E); free RAM 0.07 GB, commit 39.5 / 63.9 GB (not
    at the limit), the process's private memory up 1.6 GB in 3 s. P2, same settings: no crash;
    Q: no crash.
  - Also in Windows' log at the same time, unrelated: the motherboard's RGB lighting service
    crashing (its own DLL).
- Tests (5f4eb4b, same code as Session 5):
  - fmt: FAIL, 2,788 diffs (Session 5: 2,788).
  - clippy: FAIL, 38 errors (Session 5: 38); clippy all-features: FAIL, 36. This time every
    failing crate was reported in one run (4 crates).
  - test: 927 passed, 0 failed (Session 5: 927).
  - policy: pass (265 files).
- Measurements (base command = `-NoSmoothMotion` + VICTORY_NO_WATCHDOG=1, THIN=0,
  PIPELINE_WAIT_MS=150, PIPELINE_WORKERS=4, TEXTURE_STAGE_MB=512, TEXTURE_STAGE_MS=50; Series
  S, world at 540p; same boot, 14.1-14.5 GB committed at each launch, no restart between runs;
  first room 3 min from when the user said (Q: 2.5 min, the user went on), then the room with
  many characters (P 3 min; Q about 75 s, the window was closed before the end)):

  | | P first room | P many characters | Q first room | Q many characters |
  |---|---|---|---|---|
  | mean fps | 7.3 | 8.9 | 8.0 | 8.3 |
  | median ms/frame (p90) | 133 (186) | 123 (142) | 125 (154) | 117 (133) |
  | samples > 300 ms | 2 % | 0 % | 0 % | 0 % |
  | median draws/frame | 4,824 | 3,752 | 4,707 | 4,782 |
  | split CPU preparing / GPU / copying back (ms) | 72 / 0 / 38 | 67 / 0 / 35 | 72 / 0 / 37 | 72 / 0 / 39 |
  | lowest free RAM; peak commit, process / system | 0.38; 21.9 / 41.2 GB | 0.21; 22.7 / 41.8 | 0.27; 21.9 / 41.3 | 0.45; 22.0 / 41.4 |
  | peak GPU memory | 7.7 GB | 8.4 | 7.4 | 7.5 |
  | stretching (the user) | seen from afar on arrival, calm once inside | a little: less often and on fewer objects than before | the same as P | the same as P |
  | crash | P: once (above), P2 none | none | none | none |

  P and Q weighted over both places: 8.09 and 8.09 fps. Q's fetch check: about 690 pieces
  checked a frame, 0.3-0.8 of them caught half written and fetched again.
- Conclusion: VICTORY_FETCH_CHECK does what it says (it catches real half-written pieces) and
  costs nothing measurable here (one run each), but the stretching stays as it was. The
  stretching is NOT mainly caused by pieces fetched half written; the remaining cause is not
  known; set aside (PM: fps first). The user also saw, in the room with many characters, a
  guard's arms in a wrong colour (light arms on a dark-skinned character): a wrong or stale
  texture rather than stretching.
- Recommended command (branch nvidia, diag build): the base command plus FETCH_CHECK (kept: it
  costs nothing and catches real wrong pieces):
  `.\scripts\play-nvidia.ps1 -Exe <title exe> -NoSmoothMotion -More @{VICTORY_NO_WATCHDOG='1';
  VICTORY_THIN='0'; VICTORY_PIPELINE_WAIT_MS='150'; VICTORY_PIPELINE_WORKERS='4';
  VICTORY_TEXTURE_STAGE_MB='512'; VICTORY_TEXTURE_STAGE_MS='50'; VICTORY_FETCH_CHECK='1'}`
- What did NOT work or was taken back out: FETCH_CHECK against the stretching (above). Q2 and
  new code not tried (PM decision).
- Open problems found (not fixed):
  - Stretched bodies: cause unknown after this session (not half-written fetches; the
    diagnostic found no undeclared writes in the sends it names, but names at most 4 sends per
    buffer per line).
  - Two crashes in a list's prelude (depth texture copies), one in the NVIDIA driver, one
    E_OUTOFMEMORY with free RAM near 0; each once in two launches; cause not looked into.
  - Free RAM falls to 0.1-0.4 GB in every run (16 GB for 22 GB of process memory).
- Questions for the user / PM: none open.
- Suggested next task: fps first: try the settings of brothernog/rota that our base already
  reads (VICTORY_CARD_ALL and VICTORY_MEMORY_CARD_MB in shadow.rs, VICTORY_MEMORY_MAP in
  copystat.rs, VICTORY_DYNAMIC_SLOTS in descriptor/host.rs; the values are in that fork's
  apps/victory/src/tuned.rs, not in our tree). Later: the newer nvidia/main (9d6300f: DLSS,
  frame generation) audited and measured; the stretching and the prelude crashes.

## Session 7 — fps: brothernog/rota's settings already in our base, release vs diag build (2026-11-27)
- Branch / start commit / end commit: nvidia / fab61f9 (= origin/nvidia) / the commit that
  adds this entry. No code changed. Nothing pushed.
- Task given: the fastest command on this PC with rota's settings (VICTORY_CARD_ALL=1,
  VICTORY_MEMORY_CARD_MB=1024, VICTORY_MEMORY_MAP=0, VICTORY_CARD_STEPS=1,
  VICTORY_DYNAMIC_SLOTS=499999), read in OUR tree first; whether a release build can be run.
- Start state (read only): page file C: only, 49,152-65,536 MB, commit limit 63.9 GB.
- What the switches do in our tree, and what play-nvidia.ps1 already sets:
  - `VICTORY_CARD_ALL=1` (shadow.rs `card_all`, off): the least size of a title buffer whose
    copy gets a side in the card's memory goes to 0 (the script's VICTORY_CARD_BUFFERS_MIN_MB=1
    gives 1 MB: logged `card_buffers_min_kb=1024` without it, `0` with it), while the card has
    room; a copy refused there gets two sides in the PC's memory. So small buffers on the
    card, as rota describes.
  - `VICTORY_MEMORY_CARD_MB` (shadow.rs `by_address`): the most of the card's memory for copies
    of memory that shaders read by address (ray structures); default 4,096, rota 1,024; past
    it those copies stay in the PC's memory. Not set by the script.
  - `VICTORY_MEMORY_MAP=0` (copystat.rs, diagnostics builds only): leaves out the walk of the
    address space done with every sixth memory line, which "holds up other threads' memory
    calls for its length (89 ms for the 6.2 GB pool)". On by default in diag; absent in release.
  - `VICTORY_CARD_STEPS` (host/card_steps.rs): 1 = constant buffers of descriptor tables done
    on the card behind the flights, 2 = the title's buffer copies too. The script sets 2.
  - `VICTORY_DYNAMIC_SLOTS` (descriptor/host.rs `dynamic_slots`): on NVIDIA's view heap capped
    at 1,000,000 the value asked is clamped to at most half the heap: the script's 524,288 and
    rota's 499,999 both give 499,999 (Session 6's log: `slots=499999 title_in_place=500000`).
    The script's value kept: no difference.
- Release build: play-nvidia.ps1 takes `-Build release`. Built (`cargo build --locked --release
  -p victory`, 9,968,128 bytes; the old one was main's from Session 1). In it the ray-tracing
  skip list (VICTORY_SKIP_PIPELINES, rt_off_hashes.txt) does not exist (pipeline.rs, under the
  diagnostics feature), so the 1,034 shaders the diag build skips run; the script warns that
  rooms then go white with the view angle. Diag-only costs: the copystat lines and the memory
  map walk (above), timeline spans, extra timing.
- Tests (fab61f9, same code as Session 6):
  - fmt: FAIL, 2,788 diffs (Session 6: 2,788).
  - clippy: FAIL, 38 errors; clippy all-features: FAIL, 36 (Session 6: 38 / 36).
  - test: 927 passed, 0 failed (Session 6: 927).
  - policy: pass (265 files).
  - `cargo build --locked --release -p victory`: OK.
- Measurements (Session 6's recommended command = R0; Series S, world at 540p; same boot,
  15.3-16.2 GB committed at each launch; first room 3 min from when the user said, then the
  room with many characters 3 min; frame split = median of the layer's frame lines):

  | | R0 | R1 = R0 + CARD_ALL=1, MEMORY_CARD_MB=1024, MEMORY_MAP=0 | R2 = R1 + CARD_STEPS=1 | R3 = R1, release build |
  |---|---|---|---|---|
  | first room: mean fps, median ms (p90) | 7.1, 136 (177) [67 s only] | **9.1, 101 (161)** | 8.6, 120 (143) | 7.9, 123 (152) [window estimated] |
  | room with many characters: mean fps, median ms (p90) | 7.8, 126 (158) | 8.4, 124 (154) | 9.4, 115 (144) | 8.6, 111 (163) [117 s, a cutscene began] |
  | weighted over both | 7.6 | **8.8** | 9.0 | 8.2 |
  | samples > 300 ms (room / many) | 4 % / 5 % | 2 % / 0 % | 0 % / 0 % | 0 % / 0 % |
  | split CPU / GPU / copy back, first room (ms) | 77 / 0 / 32 | **64 / 0 / 28** | 64 / 26 / 30 | 82 / 0 / 37 |
  | victory CPU, share of the machine | 44-47 % | 43-46 % | 40-45 % | 61-65 % |
  | textures waiting at the end of the first room | 5,547 | 2,024 | 3,822 | 196 |
  | lowest free RAM; peak commit system | 0.21; 43.8 GB | 0.19; 44.2 | 0.32; 44.5 | 0.02; 43.6 |
  | peak GPU memory | 8.3 GB | 8.5 | 8.4 | 7.8 |
  | picture (the user) | as usual | a face black for a while, then drawn | NEW: part of the walls black in the room with many characters only (joints right), not seen before | looks better, less stretching |
  | crash | no | no | no | no |

  A first R0 launch was not used: the user went on without saying when they entered the room
  and closed the window. In R0 the user left the first room after 67 s.
- Reading: R1 is about 1.2 fps over R0 (one run each), with less CPU preparing (64 vs 77 ms)
  and copying back. R2 is 0.25 fps over R1 (within one run's noise), brings the GPU wait back
  (10-26 ms, as -Quality did) and a new visible fault (black walls): not kept. R3 is slower
  than R1 (8.2 vs 8.8; the CPU more busy, likely the ray-tracing passes the diag build skips),
  but the user found its picture better with less stretching and it had far fewer textures
  waiting: a picture trade, not a speed one, and its windows are less exact.
- Recommended command (branch nvidia, diag build): R1:
  `.\scripts\play-nvidia.ps1 -Exe <title exe> -NoSmoothMotion -More @{VICTORY_NO_WATCHDOG='1';
  VICTORY_THIN='0'; VICTORY_PIPELINE_WAIT_MS='150'; VICTORY_PIPELINE_WORKERS='4';
  VICTORY_TEXTURE_STAGE_MB='512'; VICTORY_TEXTURE_STAGE_MS='50'; VICTORY_FETCH_CHECK='1';
  VICTORY_CARD_ALL='1'; VICTORY_MEMORY_CARD_MB='1024'; VICTORY_MEMORY_MAP='0'}`
  The fastest without a new serious fault. What it gives up: R3's picture (release build,
  about -0.6 fps).
- What did NOT work or was taken back out: VICTORY_CARD_STEPS=1 (black walls). The release
  build as a speed gain.
- Open problems found (not fixed):
  - R2: part of the walls black in one room with VICTORY_CARD_STEPS=1.
  - The release build runs the ray-tracing passes the diag build skips; that it looked better
    (less stretching) was not expected and is one user's look at one run.
  - Free RAM still falls to 0.02-0.3 GB in every run.
- Questions for the user / PM: whether the release build's look (less stretching, textures
  in time) is worth about 0.6 fps; it would also need R1's settings checked there (MEMORY_MAP
  does not exist in it).
- Suggested next task: confirm R1 vs R0 with a second run each (one run each so far); then
  the release build with R1's settings again, the first room watched from the same entry.

## Session 8 — trial of trustno1's new base (48ad7859, "foxhound") on branch nvidia2 (2026-11-27)
- Branch / start commit / end commit: nvidia2 (new, local, from nvidia 1657259) / 5a41a8b
  ("trustno1 48ad7859, squashed": the tree of 48ad7859 in one commit, our CLAUDE.md, SESSIONS.md,
  PROMPTS.md from nvidia on top, PM_HANDOFF.md as received) / the commit that adds this entry.
  None of trustno1's commits is in our history. nvidia (1657259), main (040e82e) and
  avatar-nvidia (5649892) not moved. Nothing pushed.
- Start state: nvidia = origin/nvidia = 1657259; page file C: only, 49,152-65,536 MB.
  trustno1-coder/VIctory-NVIDIA had 29 commits newer than 48ad7859 (up to d12eb1b, 05:40 the same
  day): not taken (PM). 744 commits from our base 033be27 to 48ad7859; 117 files changed against
  nvidia's tree.
- Task given: try that version beside nvidia and compare it with our best command: audit first,
  checks, diag build, shaders ahead of time and a warm-up, then S0 (nvidia, Session 7's R1), S1
  (their launcher, Series S, no DLSS), S2 (S1 + R1's settings), and from a PM addition S3 (their
  default profile: Series X 2560x1440, no DLSS) and S4 (S3 with DLSS Quality, NVIDIA's files
  allowed through their Get-NVIDIA-DLSS.cmd only).
- Audit, rule #1 (before any build): `cargo xtask policy` passes (275 files of theirs; 279 with
  ours). No logs, pictures, captures, dumps or executables in the tree; no personal identifiers
  (the save code's user id is the same made-up constant); machine paths only as examples
  (`D:\Games\MyDump`). No file removed. To note for the PM: more of the title's names than
  before (58 distinct shader names against 39 in nvidia; the title's executable name in 8 files,
  0 in nvidia), of the same kind as earlier bases. Their policy.rs now allows `.py` (for
  scripts/faultbook.py). Removed by them: crates/victory-loader/src/hooks.rs (with it
  VICTORY_NO_WATCHDOG, VICTORY_RENDER_SCALE, VICTORY_OWN_UPSCALER), scripts/rt_off_hashes.txt,
  the play-S-*/play-X-* launchers, and our 4 .md files (put back).
- Audit, safety: no new build.rs and no new crate in Cargo.lock. The title's network stays
  blocked (net.rs unchanged: only loopback). Get-NVIDIA-DLSS.cmd and `victory get-dlss` (and a
  question at `victory play` start, only when stdin is a terminal) run scripts/get-nvidia-sdks.ps1,
  which downloads only from github.com/NVIDIA-RTX/Streamline releases (v2.14.16), takes 8 DLLs and
  their licences out of the zip into nvidia-sdk\ in the repository (git-ignored), no registry,
  no install, no admin. Play-VIctory.cmd names a prebuilt `victory-3d-rtx-pareto-residency.exe`
  that is not in the tree. Their launcher writes victory-settings.txt in the repository (F1
  menu, git-ignored) and logs to %LOCALAPPDATA%\victory\logs. With the PM's permission
  Get-NVIDIA-DLSS.cmd was run once before S4 (41 s, about 70 MB); the 8 DLLs carry valid
  signatures of NVIDIA Corporation; `git status` shows nothing new (nvidia-sdk/ ignored).
- Their launcher: scripts/play-nvidia.ps1 only calls scripts/play-foxhound.ps1. Defaults: release
  build, Series X with a native 2560x1440 scene, DLSS Quality when NVIDIA's files are there, no
  frame cap (VICTORY_FRAME_INTERVAL_US=1000), VICTORY_THIN=1, RELAY=2, RELAY_COPY=1, CARD_STEPS=2,
  CARD_BUFFERS_MIN_KB=64, TEXTURE_STAGE_MS=16, TEXTURE_CATCHUP=0, VICTORY_AMBIENT_SPECULAR=1 (a
  reflection estimate), VICTORY_EARLY_DEPTH=1, VICTORY_PIPELINE_BOOK=1 and others. `-SeriesS` is
  a 1280x720 scene (no render scale switch any more); no `-NoSmoothMotion` and no `-Resolution`.
  The "BAR windows under 24 GB of RAM" setting was not found in 48ad7859: what depends on the
  PC's memory is residency.rs (`limited_host_ram`: 32 GiB or less, true here), which makes the
  caches leaner; VICTORY_SEND_BAR and VICTORY_WC_BAR exist, off, not set by the script.
- Resizable BAR (read only, `nvidia-smi -q -d MEMORY`): BAR1 total 256 MiB: OFF on this card.
- Tests (nvidia2 = 48ad7859's code, nothing fixed):
  - fmt: FAIL, 3,363 diffs (nvidia: 2,788).
  - clippy: FAIL, 42 errors counted with `--keep-going` (nvidia: 38); a plain run stops at the
    first failing crate (1 error shown).
  - test: 1,027 passed, 2 FAILED, 46 ignored (`--no-fail-fast`; nvidia: 927 passed, 0 failed):
    `indirect::tests::host_list_budget_starts_empty_at_each_submission` and
    `indirect::tests::warp_batch_charges_every_execution` (left 0, right 134,217,728), failing
    every time, not flaky.
  - policy: pass. diag build (`--profile diag --features diagnostics`) and release build: OK.
- Shaders ahead of time (release build, its own new cache folder, with the switches their script
  sets for S1, read from the script itself): 25,760 translated, 225 not translatable (198
  containers without a program it takes, 22 wave64 programs with no wave32 lowering, 5 others),
  263 s. Warm-up launch W (S1's settings, played to the room with many characters, not timed):
  black screen during the arrival cutscene; heads drawn as eyes and teeth only in the first
  room, then drawn after a while (late textures); about 12-13 fps at the end.
- nvidia's diag build kept outside the repo for S0 (copy of 1657259's), nvidia's warm cache
  untouched (nvidia2 used its own).
- Measurements (one boot, 15.7-17.7 GB committed at each launch; first room 3 min from when the
  room was drawn after the user said so, then the room with many characters 3 min; "mean fps"
  is the mean of the window caption's fps, "median ms" the median of its ms/frame: the mean is
  pulled up by a few very fast stretches, so both are given):

  | | S0 nvidia, R1 | S1 nvidia2, Series S 720p | S2 = S1 + R1's settings | S3 nvidia2, Series X 1440p | S4 = S3 + DLSS Quality |
  |---|---|---|---|---|---|
  | first room: mean fps, median ms (p90) | 7.4, 123 (215) | 7.3, 107 (631) | 5.9, 221 (238) | 5.8, 207 (271) | 6.7, 181 (274) [86 s] |
  | first room, samples > 300 ms | 5 % | 28 % | 0 % | 8 % | 3 % |
  | many characters: mean fps, median ms (p90) | 9.6, 111 (131) | **11.5, 92 (106)** | 17.1, 88 (389) | 8.6, 107 (4,411) [a later place, see below] | 10.0, 98 (624) |
  | many characters, samples > 300 ms / > 1 s | 0 % / 0 % | 2 % / 0 % | 16 % / 2 % | 33 % / 24 % | 16 % / 5 % |
  | mean fps over both | 8.5 | **9.4** | 11.5 | 7.1 | 8.9 |
  | split CPU / GPU / copy back, many characters (ms) | 61 / 0 / 32 | 63 / 2 / 27 | 70 / 2 / 26 | 67 / 13 / 15 | 57 / 0 / 14 |
  | lowest free RAM | 0.05 GB | 0.24 | **0.00** | **0.00** | **0.00** |
  | peak process private / system commit | 23.0 / 44.6 GB | 22.2 / 42.9 | 24.0 / 44.5 | 27.1 / 50.3 | 27.4 / 50.7 |
  | peak GPU memory | 8.3 GB | 9.1 | 9.3 | 10.9 | 10.6 |
  | textures waiting at the end, many characters | 4,239 | 2,874 | 8,179 | 16,985 (rising) | 18,578 (rising) |
  | crash | no | no | no | no | no |

  R1's settings in S2 (those nvidia2 reads and its script does not set): VICTORY_PIPELINE_WAIT_MS=
  150, PIPELINE_WORKERS=4, TEXTURE_STAGE_MB=512, FETCH_CHECK=1, CARD_ALL=1, MEMORY_CARD_MB=1024,
  MEMORY_MAP=0 (diag only; S1-S4 ran the release build). S3/S4 also had those, VICTORY_THIN=0 and
  VICTORY_NO_WATCHDOG=1 (no longer read by nvidia2). No crash in the 8 launches of nvidia2 without
  the watchdog switch. S3: the user went past the room with many characters; its second window
  is the next place (toward a desk), as S2's window ran into it too. S4: a first launch ran
  unattended (the game went on by itself through the cutscenes) and was not used; in the one
  used, the user left the first room after 86 s.
- Picture (the user):
  - S0: fine.
  - S1: everything drawn in play, textures sharper (posters on the walls readable), NO stretching
    seen in the room with many characters, some shadow faults, eyes too white under some angles
    (pupils hidden as by a white layer), one character's glasses now right but her face's colour
    changed; cutscenes badly drawn; frame times took a while to settle in the first room.
  - S2: walls of a corridor black seen from inside it (fine from outside: a lighting fault), a
    character earlier drawn wrong now right, the same white eyes; black screen in the arrival
    cutscene (skipped).
  - S3: purple and blue textures in the first room, then drawn right; a guard's arms glitching
    again. S4: sharp; eyes too white under some angles; one character's head still wrong.
- Reading: nvidia2 in Series S (S1) is faster than nvidia (S0) in the room with many characters
  (median 92 vs 111 ms, about +20 %; mean 11.5 vs 9.6 fps) and about level in the first room
  (median 107 vs 123 ms, but with many more long frames there: p90 631 vs 215). Over both, mean
  9.4 vs 8.5 fps. One run each. It renders at 720p where nvidia renders at 540p, and the user
  saw no stretching. R1's settings do not help nvidia2 (S2: the first room twice as slow by the
  median, free RAM at 0, a new lighting fault). The Series X 1440p profile (S3, S4) runs out of
  memory on this PC (16 GB): free RAM 0, a quarter of S3's frames over a second, textures piling
  up; DLSS (S4) does not change that.
- Recommended command: S1, on branch nvidia2 (release build, its own cache folder):
  `.\scripts\play-nvidia.ps1 -Exe <title exe> -SeriesS -Dlss off -More @{VICTORY_THIN='0'}`
  (cache folder: VICTORY_CACHE_DIR to a folder of its own; a warm-up launch first). What it costs:
  black or badly drawn cutscenes, eyes too white under some angles, more long frames while the
  first room settles; and the base has 2 failing tests and no watchdog switch. If the PM wants
  no new fault, the alternative is S0 (nvidia, Session 7's command, 8.5 fps).
- What did NOT work or was taken back out: R1's settings on nvidia2 (S2). The Series X 1440p
  profile, with or without DLSS, on 16 GB. A first S4 launch (unattended).
- Open problems found (not fixed):
  - nvidia2: two unit tests fail; VICTORY_NO_WATCHDOG gone (the title's watchdog could end a run
    again in long stalls; none seen); cutscenes black or badly drawn; eyes too white.
  - RAM: every nvidia2 run but S1 reached 0 GB free.
  - Resizable BAR is off on this PC (BAR1 256 MiB).
- Questions for the user / PM: take S1 (faster, sharper, no stretching seen, worse cutscenes) or
  stay on S0 (no new fault)? Whether to turn on Resizable BAR in the BIOS (a system setting, the
  user's to change), since nvidia2 has BAR-based options.
- Suggested next task: a second run of S0 and S1 to confirm the gap; then S1 with the newer
  trustno1 commits (d12eb1b) if the PM wants them.

## Session 9 — VICTORY_NO_WATCHDOG ported to nvidia2; S1 confirmed (T1, T2) (2026-11-27)
- Branch / start commit / end commit: nvidia2 (now the working branch, PM decision) / 0268984
  (= origin/nvidia2) / a94ad08 (the port) + the commit that adds this entry. Nothing pushed.
- Task given: put the title-watchdog switch of branch nvidia back into nvidia2 (same name, off by
  default, applied only to the build its hash names), then two runs of Session 8's S1 with it.
- What was done (files, switches added):
  - Looked for an equivalent first: none in nvidia2. The offsets are nowhere in its tree; the
    "watchdog" in crates/victory-loader/src/threads.rs is the layer's own, at process exit.
  - New `crates/victory-loader/src/hooks.rs` (watchdog only; nvidia's render-scale hooks not
    ported), `mod hooks` in lib.rs, the call in windows.rs right after the executable is
    mapped and before any title code runs (the executable only, not the DLLs).
  - `VICTORY_NO_WATCHDOG=1` (off by default; only "1" turns it on, where nvidia took any value):
    if the original executable's BLAKE3 begins with the 1.0.2.1 build's, the two places where the
    title's watchdog writes to address 0 are patched, each only if its bytes are exactly the
    expected ones (else "hook skipped" in the log). Another build: a log line saying nothing was
    patched. The log says "hook applied" for each and "title watchdog hooks (VICTORY_NO_WATCHDOG)
    applied=N". Per-build offsets and short byte signatures under our names only (rule #1),
    the same as nvidia's. Credit in the commit: our nvidia branch (trustno1 033be27), from
    vitutu1/victory-perf, a port from ZigZagBoi1/ScarlettPort (GPL-3.0-or-later).
  - Four synthetic tests (bytes of our own): the patches apply where their bytes are and only
    once, other bytes are left untouched, only the known build hash passes, the switch reads
    only "1".
- Tests (a94ad08 against Session 8's 0268984):
  - fmt: FAIL, 3,363 diffs (Session 8: 3,363; none in the files of this change).
  - clippy: FAIL, 42 errors with `--keep-going` (Session 8: 42; none in hooks.rs; the loader's 4
    are the old ones in write_sites.rs); all-features: 40.
  - test: 1,031 passed (the 4 new ones included), the same 2 known failures in
    victory-gpu's indirect tests, 46 ignored (`--no-fail-fast`).
  - policy: pass (280 files). Release build: OK.
- Measurements (S1's command + VICTORY_NO_WATCHDOG=1: nvidia2's scripts\play-nvidia.ps1, release
  build, `-SeriesS -Dlss off`, VICTORY_THIN=0, its own cache folder; Series S, 1280x720 scene;
  one boot, 16.2-17.0 GB committed at each launch; first room from when the user said, the
  room with many characters from when the user said; the game takes the player on by itself,
  so the windows are shorter than 3 min where noted). In both runs the log said
  `applied=2`. No crash, no early end.

  | | S1 (Session 8) | T1 | T2 |
  |---|---|---|---|
  | first room: mean fps, median ms (p90) | 7.3, 107 (631) | 8.0, 111 (595) [90 s] | 9.6, 100 (131) [74 s] |
  | first room, samples > 300 ms | 28 % | 16 % | 0 % |
  | many characters: mean fps, median ms (p90) | 11.5, 92 (106) | 13.0, 74 (96) [127 s] | 10.6, 86 (203) [174 s] |
  | many characters, samples > 300 ms | 2 % | 0 % | 5 % |
  | mean fps over both | 9.4 | 10.9 | 10.3 |
  | split CPU / GPU / copy back, many characters (ms) | 63 / 2 / 27 | 57 / 3 / 25 | 63 / 2 / 26 |
  | lowest free RAM; peak commit system | 0.24; 42.9 GB | 0.42; 42.9 | 0.00; 44.4 |
  | peak GPU memory | 9.1 GB | 9.3 | 9.1 |

  S1 is confirmed: 9.4 to 10.9 fps over both places in three runs. The switch itself does not
  change frame times (it only removes a deliberate fault); T1/T2's small gain over S1 is more
  likely the fuller cache and shorter windows.
- Picture (the user): the arrival cutscene still badly drawn, with big white flashes, "as if it
  starts before the game is ready" (T1); loaded a little faster in T2 but still ugly, and in play
  "not at the level of the console versions"; eyes still too white under some angles.
- Recommended command (branch nvidia2, release build, its own cache folder, a warm-up launch
  first): `.\scripts\play-nvidia.ps1 -Exe <title exe> -SeriesS -Dlss off -More @{VICTORY_THIN='0';
  VICTORY_NO_WATCHDOG='1'}`
- What did NOT work or was taken back out: nothing.
- Open problems found (not fixed): cutscenes drawn before they are ready (white flashes, black
  screens); eyes too white; free RAM still reaches 0 GB at times (T2); the 2 failing tests of
  the base.
- Questions for the user / PM: none open.
- Suggested next task: the cutscenes (they start while pipelines and textures are still being
  made), or the newer trustno1 commits (up to d12eb1b) on nvidia2.
- Addendum to Sessions 8 and 9 (found after the Session 9 commit, while checking a free launch):
  - CORRECTION to Session 8: S4 (Series X 1440p + DLSS Quality) did crash, at about t = 650 s
    (16:21:26), just after its measured window (456-636 s) and seconds before it was to be
    closed: unhandled exception 0xc0000005 in the NVIDIA driver's user-mode DLL (a read at 0xc),
    recorded in crashes.log; its launcher ended with code 5, not the 255 of a forced close. Session
    8's table says "no" for S4: that is wrong. S4's numbers stand (taken before the crash). So
    both Series X 1440p profiles ran out of memory and S4 also ended in a driver fault.
  - Session 9, T2: the user adds that the arrival cutscene showed more than before ("more things
    visible"), though still far from the console versions.
  - A free launch after Session 9 (recommended command, not measured) ended normally (window
    closed); no crash.

## Session 10 — nvidia2 updated to trustno1 ca9c75c; V1, U (not run), W = AMD FSR 3 frame generation (2026-11-27)
- Branch / start commit / end commit: nvidia2 / f9e3b0d (= origin/nvidia2) / 183d75b ("trustno1
  48ad7859..ca9c75c, squashed") + the commit that adds this entry. Nothing pushed.
- Task given: bring nvidia2 up to trustno1's newest commit in one commit, audit, checks, then V1
  (Session 9's command on the new build), U (V1 + VICTORY_AMBIENT_SPECULAR=1) and W (the better of
  V1/U + AMD FSR 3 frame generation 2x, AMD's DLLs allowed from AMD's GitHub only).
- Update: trustno1-coder/VIctory-NVIDIA's newest commit at the fetch was ca9c75c (2026-11-27 10:12
  -0800), one after 29d8dbf (AMD FSR 3 frame generation): taken. The changes 48ad7859..ca9c75c (31
  commits, 33 files) applied on nvidia2 with `git apply --3way`: no conflict; none of them touches
  our VICTORY_NO_WATCHDOG (a94ad08) or our .md files. The resulting tree is exactly ca9c75c's plus
  our files, minus the two removed below. None of trustno1's commits is in our history.
- Audit, rule #1 and safety (before building): asset policy passes (283 files). No binary, log,
  image, capture or executable added; no machine path or personal identifier. REMOVED before the
  commit: PLAY_FRAMEGEN.cmd and scripts/play-gta6-framegen.ps1, a one-click launcher that names
  one of the title's data files to recognise its folder, searches every drive for it, copies the
  saves outside the repository and %LOCALAPPDATA%\victory (to D:\ or the user profile), reads
  the registry (read only) and runs the NVIDIA download script. No new build.rs, no new crate;
  victory-hle's windows-sys asks three more API sets (device and HID access for a new DualSense
  module, GDI). FSR 3 (crates/victory-gpu/src/streamline/fsr.rs) downloads nothing: it loads
  AMD's two DLLs from VICTORY_FIDELITYFX_DIR (else amd-sdk\FidelityFX\bin in the repository) and
  checks their signatures. Their launcher now makes FSR 3 frame generation "ready and off" at
  every start (VICTORY_FSR_FRAMEGEN=1, FRAMEGEN_X=0; without the DLLs it stays off) and sets
  VICTORY_TEXTURE_ROOM_PERCENT=86 (80 with frame generation on).
- Tests (183d75b; Session 9: fmt 3,363, clippy 42, 1,031 passed + 2 known failures):
  - fmt: FAIL, 3,462 diffs. The 99 more are all in files the update changed (1,316 -> 1,415 in
    those files; the others unchanged at 2,047): trustno1's new code, not ours. Not fixed.
  - clippy: FAIL, 42 (`--keep-going`); all-features: 40. Same as Session 9.
  - test: 1,057 passed, the same 2 known failures (victory-gpu indirect tests), 46 ignored.
  - policy: pass. Release build: OK.
- Shaders ahead of time (the translator changed: emit/lower.rs, shader_cache.rs): 25,760
  translated, 225 not, 275 s. Two warm-up launches (the first one stopped in the first room by
  a misunderstanding; the second went to the room with many characters).
- U not run, and why: the three light switches were already on in T1, T2 and V1, set by
  play-foxhound.ps1 (which play-nvidia.ps1 calls): the launch logs read VICTORY_AMBIENT=0.8,
  VICTORY_AMBIENT_AIR=1.5, VICTORY_AMBIENT_SPECULAR=1. So U = V1. Also, in T1's log the
  reflection stand-in (ambient.rs, VICTORY_AMBIENT_SPECULAR) never ran: all 38 of its 10 s lines
  say 0 frames and 0 dispatches followed (the title's miss pass it follows is among the
  ray-tracing passes this base leaves out); the diffuse stand-in was made. That is why the eyes
  and floors still take the sky's light. Not changed (no code task).
- AMD's DLLs for W (with the PM's permission): amd_fidelityfx_loader_dx12.dll (39,184 bytes) and
  amd_fidelityfx_framegeneration_dx12.dll (29,679,376 bytes) from GPUOpen-LibrariesAndSDKs/
  FidelityFX-SDK tag v2.4.0, Kits/FidelityFX/signedbin/ (the release ships them in the source
  tree; only these two files fetched), into %LOCALAPPDATA%\victory\fidelityfx (outside the
  repository). Both signed by Advanced Micro Devices (valid); their git blob hashes match AMD's
  tree. The layer's log: "AMD's FidelityFX DLLs loaded (signatures valid)".
- W, how it is started and what went wrong first: FSR 3 frame generation needs the line of
  submissions (VICTORY_THIN=1; V1 has 0) and makes the title render through its FidelityFX path at
  VICTORY_DLSS_MODE's scale. A first start (scale unset: dlaa, 1.0 of the 1080p output) stopped
  16 s in on the title's own breakpoint (0x80000003, in title code, during start-up); the title's
  crash reporter's upload was blocked by the layer. With VICTORY_DLSS_MODE=quality (0.667: V1's
  1280x720 scene) it ran to the end. (Also: play-nvidia.ps1 only passes @args on, which loses a
  switch given in a splatted table; typed on a command line it is fine.)
- Measurements (nvidia2 183d75b, release, Series S, 1280x720 scene, VICTORY_NO_WATCHDOG=1 applied=2
  in each; one boot; 16.9-17.5 GB committed at each launch; first room 3 min from when the user
  said (W: 103 s, the user went on), then the room with many characters 3 min from leaving the
  first room (the user walked through it toward a desk both times)):

  | | T1 (Session 9) | T2 (Session 9) | V1 | W (V1 + FSR 3 2x, THIN=1) |
  |---|---|---|---|---|
  | first room: fps rendered / shown | 8.0 | 9.6 | 7.1 / 7.1 | 7.6 / **14.8** |
  | first room: median ms (p90), > 300 ms | 111 (595), 16 % | 100 (131), 0 % | 125 (665), 17 % | 117 (406), 11 % |
  | many characters: fps rendered / shown | 13.0 | 10.6 | 11.7 / 11.7 | 11.6 / **23.2** |
  | many characters: median ms (p90), > 300 ms | 74 (96), 0 % | 86 (203), 5 % | 83 (104), 0 % | 84 (105), 0 % |
  | rendered fps over both (shown) | 10.9 | 10.3 | 9.4 | 10.1 (20.0) |
  | split CPU / GPU / copy back, many characters (ms) | 57 / 3 / 25 | 63 / 2 / 26 | 60 / 2 / 25 | - |
  | lowest free RAM; peak commit system | 0.42; 42.9 | 0.00; 44.4 | 0.34; 43.4 | 0.14; 43.9 GB |
  | peak GPU memory | 9.3 GB | 9.1 | 8.3 | 7.9 |
  | crash | no | no | no | no (first W start: the title's breakpoint at 16 s, see above) |

  The update brings no visible rendered-fps gain: V1 is within S1/T1/T2's 9.4-10.9. W costs no
  rendered fps and shows twice as many frames (the window caption gives both). One run each.
- Picture and feel (the user): V1: eyes a little better, still white under some angles; glasses,
  floor and haze "fine", but still not the picture expected of the game. W: "at times much
  smoother"; the controller feels a little slow, the same as in V1 (frame generation adds no lag
  the user felt); no trails or smearing seen.
- Recommended command, rendered fps first (branch nvidia2, release build, its own cache folder):
  V1, i.e. Session 9's: `.\scripts\play-nvidia.ps1 -Exe <title exe> -SeriesS -Dlss off -More
  @{VICTORY_THIN='0'; VICTORY_NO_WATCHDOG='1'}`.
  Frame generation is worth it to the eye: W renders as fast and shows about twice the frames,
  with no lag or trails the user noticed. Command (AMD's two DLLs in a folder outside the
  repository named by VICTORY_FIDELITYFX_DIR):
  `$env:VICTORY_FIDELITYFX_DIR = "$env:LOCALAPPDATA\victory\fidelityfx"` then
  `.\scripts\play-nvidia.ps1 -Exe <title exe> -SeriesS -Dlss off -FrameGen -FgType fsr -More
  @{VICTORY_THIN='1'; VICTORY_NO_WATCHDOG='1'; VICTORY_DLSS_MODE='quality'}`
- What did NOT work or was taken back out: U (identical to V1); W at the default scale (dlaa) on
  Series S (the title's breakpoint at start); the two one-click launcher files (removed).
- Open problems found (not fixed): the reflection stand-in never runs (white eyes); the first
  room still starts with many long frames; fmt +99 from the update; the 2 known failing tests.
- Questions for the user / PM: whether to take W's command as the one to play with.
- Suggested next task: make the reflection stand-in run (or find which pass it should follow in
  this base), since it is what the white eyes and floors come from.

## Session 11 — the long-play memory leak fix (trustno1's PR #58) on nvidia2; 20-minute runs X0, X1 (2026-11-27)
- Branch / start commit / end commit: nvidia2 / 9f692dc (= origin/nvidia2 after the PM's push; at the
  first check origin/nvidia2 was still f9e3b0d, reported) / 8e19745, a74d115, d6389d7, 05ae124 (the
  fix) + the commit that adds this entry. Nothing pushed.
- Task given: bring in the fix of foxhound v0.2.0-pre.3's "memory leak in long play" (the layer kept
  a new copy of views the title had only filled with different leftover bytes), then measure 20
  minutes of play before (X0) and after (X1).
- Where the fix was taken from: trustno1 already had it, after ca9c75c: PR #58 (merge 502f54c, 11:45
  -0800 on 2026-11-27), "placed views ignore padding words (views leak, confetti on faces)". The PM
  chose trustno1's commits over foxhound's (closer to our base). Picked with `git cherry-pick -x`, no
  conflict, nothing else of trustno1's (not the "MFG Unlocker" downloaded at start, nor the other
  commits after ca9c75c):
  - 4eead67 -> 8e19745: placed views copy only the named fields of the resource description; its
    padding (words 1 and 13) is zero, so the same view is no longer a new shape each time (from
    JMAN730/project-flamingo b1e0cd1d, per its message).
  - 89fd936 -> a74d115: table entries of views no resource releases go in a queue, oldest freed past
    VICTORY_OWNERLESS_VIEWS (default 1,048,576; 0 keeps all). ON BY DEFAULT, unlike our rule for new
    switches: it is the core of the fix asked for; noted, not changed.
  - 144aca0 -> d6389d7: the view description's unused words (per dimension) are zero too.
  - 8bff6c2 -> 05ae124 (test only, the 4th commit of that PR, taken with the user's yes): our test
    `descriptor::tests::views_at_new_addresses_add_no_entries` expected the padding words kept and
    failed every time without it.
  Their messages say the shape table filled after about half an hour of play, after which every
  placed view became a kept entry (34.9 million, 16.7 GB after 70 minutes on Series S).
- Audit, rule #1 and safety: only crates/victory-gpu/src/descriptor.rs and resource.rs change; no URL,
  path, identifier, long hex, process launch, file access or library load added. Policy passes.
- Tests (05ae124; Session 10: fmt 3,462, clippy 42, 1,057 passed + 2 known failures):
  - fmt: FAIL, 3,471 diffs: the 9 more are all in the picked code (descriptor.rs, resource.rs).
  - clippy: FAIL, 42 (`--keep-going`); all-features 40. Same.
  - test: 1,060 passed, the same 2 known failures, 46 ignored. (Before 8bff6c2 the descriptor test
    above failed, and `watch::tests::kernel_reads_into_guarded_blocks_after_opening` failed once in a
    full run and passed alone: flaky.)
  - policy: pass. Release build: OK. 9f692dc's release build kept outside the repository for X0.
- Measurements: Session 9's recommended command (`-SeriesS -Dlss off`, VICTORY_THIN=0,
  VICTORY_NO_WATCHDOG=1, release, its own cache folder), Series S, 1280x720 scene; the same boot
  (restarted before X0; X1 not restarted, 13.3 / 13.4 GB committed at launch); sampled every 2 s,
  folded into minutes from when the user was in play. X0: the first room, the room with many
  characters, the rest of the prologue, then the next mission (from minute 12). X1: the game
  started at its checkpoint, the next mission directly (a boat, then a farm in marshes with much
  vegetation from about minute 8): the places differ after minute 12 only partly.

```
  min | private GB X0 / X1 (each # = 0.25 GB over 20 GB)            | ms/frame median X0 / X1
    1 | X0 21.31 #####          X1 20.33 #              |  114 /   90
    2 | X0 21.30 #####          X1 20.27 #              |  107 /   92
    3 | X0 21.68 #######        X1 20.46 ##             |  102 /   93
    4 | X0 21.58 ######         X1 20.46 ##             |   93 /   92
    5 | X0 21.55 ######         X1 20.42 ##             |   78 /   97
    6 | X0 22.53 ##########     X1 20.48 ##             |  167 /  100
    7 | X0 23.18 #############  X1 20.64 ###            |  165 /  102
    8 | X0 23.34 #############  X1 21.01 ####           |  125 /  117
    9 | X0 21.94 ########       X1 20.83 ###            |  104 /  116
   10 | X0 23.10 ############   X1 21.20 #####          |  147 /  121
   11 | X0 22.87 ###########    X1 21.05 ####           |  172 /  117
   12 | X0 22.45 ##########     X1 21.13 #####          |   92 /  117
   13 | X0 22.50 ##########     X1 21.24 #####          |  100 /  117
   14 | X0 22.59 ##########     X1 21.29 #####          |   96 /  107
   15 | X0 22.51 ##########     X1 21.61 ######         |   96 /  125
   16 | X0 22.60 ##########     X1 21.71 #######        |   95 /  121
   17 | X0 22.46 ##########     X1 21.71 #######        |   94 /  118
   18 | X0 22.47 ##########     X1 21.71 #######        |   95 /  130
   19 | X0 22.55 ##########     X1 21.60 ######         |  100 /  113
   20 | X0 22.91 ############   X1 21.75 #######        |   95 /  128
```

  | | X0 (before) | X1 (after) |
  |---|---|---|
  | private memory, minute 1 -> 20 | 21.31 -> 22.91 GB (+1.6) | 20.33 -> 21.75 GB (+1.4) |
  | system commit, minute 1 -> 20 (limit 63.9) | 39.86 -> 41.57 GB | 38.91 -> 40.18 GB |
  | lowest free RAM | 0.00 GB (minutes 6, 8) | 0.04 GB |
  | peak GPU memory | 9.37 GB | 8.58 GB |
  | median ms/frame, minutes 13-20 | 94-100 | 107-130 (the vegetation) |
  | crash | no | no |

- Conclusion: neither run shows a leak in 20 minutes: memory rises 1.4-1.6 GB over the first
  minutes and then holds (X0 22.5-22.9 GB from minute 12, X1 21.6-21.75 GB from minute 15); frame
  times do not grow minute after minute (X1's slower minutes are the marsh's 6,500-7,500 draws a
  frame against about 4,000 on the boat). So the fix could not be shown at work here: by its own
  commit messages the leak starts once the shape table is full, after about half an hour of play;
  20 minutes are too short to see it. X1 holds about 1 GB less memory than X0 throughout, but the
  places differ. Not proven either way.
- What did NOT work or was taken back out: nothing taken out.
- Open problems found (not fixed): a run of 40 minutes or more is needed to see the leak and the fix;
  VICTORY_OWNERLESS_VIEWS is on by default; fmt +9 from the picked code; the flaky watch test.
- Questions for the user / PM: whether to run a 45-minute X0/X1 pair to see the leak.
- Suggested next task: that longer pair (same place, the game loaded at the same checkpoint for
  both).

## Session 12 — fps: sweep of the CPU-side switches that are off by default in nvidia2 (2026-11-28)
- Branch / start commit / end commit: nvidia2 / ef42499 (= origin/nvidia2) / the commit that adds this
  entry. No code changed. Nothing pushed.
- Task given: find which of 16 off-by-default switches (memory tracking, submitting thread) raise
  rendered fps on this PC with no new visible fault; then one run of the winners with frame generation.
- The 16 switches, read in the code (crates/victory-gpu, crates/victory-hle):
  - Already on in our base command (play-nvidia.ps1 calls play-foxhound.ps1, which sets them), so not
    tested: VICTORY_SYNC_IN_PLACE, VICTORY_HEAP_SYNC_AT_SUBMIT, VICTORY_HEAP_SYNC_FIRST,
    VICTORY_SPANS_QUICK, VICTORY_SYNC_FINE, VICTORY_SOFT_VIEWS=copies, VICTORY_WC_NO_BACK,
    VICTORY_PRELUDE_UPLOADS.
  - Left out: VICTORY_PORT_CONSTANTS (renderer port #1, the title's constants heap used in place;
    docs/README-foxhound.md: "on a 12 GB card the GPU then waits (slower overall)", this card's
    size); VICTORY_FAST_CORES (asks for the fast class of cores; only acts on processors with two
    kinds, this one has one).
  - Tested (6):
    - VICTORY_FAULT_UNLOCKED=1 (watch.rs): a write fault opens a guarded block without the guards'
      lock (its comment: 252 of 281 ms in the fault handler were waits for that lock); the guards'
      notes are settled by the next lock holder.
    - VICTORY_GUARD_HOT=50 (watch.rs): a block written again within 50 ms of its last rest stays
      open that long (about 330 guard faults a frame); copied at every refresh while it rests.
    - VICTORY_LAZY_VIEWS=1 (xmem.rs): the title's page-mapping calls map a view only when something
      touches it (365,000 calls in 4 minutes of streaming per its comment).
    - VICTORY_MAP_RUNS=1 (xmem.rs): one view per run of consecutive pages instead of one per page;
      a later change inside a run maps the rest again.
    - VICTORY_INDIRECT_MERGE=1 (indirect/merge.rs): identical indirect executions in a row joined
      (a `shortcut:`: what a binding reads is taken as the run's first found it).
    - VICTORY_HELPER_PRIORITY=2 (cores.rs): the threads a frame waits for at the highest thread
      priority. The +1 fps its comment measured (8.6 -> 9.6 on a 6-core) is `helpers_raised`, which
      is on by default (helpers above normal); 1 would leave them so, so 2 was the test.
- Method: release build, Session 9's recommended command (`-SeriesS -Dlss off`, VICTORY_THIN=0,
  VICTORY_NO_WATCHDOG=1, its own cache folder) + one switch at a time; the checkpoint the game loads
  at start (a cutscene at the load, left to play), standing still; each run unattended by a script
  (outside the repo): launch, the world's first frames (over 1,000 draws), 240 s, then 120 s
  measured, game closed. Same boot, 14.4-17.0 GB committed at each launch. The first frame of each
  run's window was the same phase of the scene in every run (median draws 3,437-3,574). The game
  shows a dialog at start that needs an OK; once it waited (the first Y3), which only delays.
  The first Y3 had a browser open on the PC during its window: set aside and run again.

  | run | switch | mean fps | median ms (p90) | CPU / GPU / copy back (ms) | peak private / commit | vs bases' mean 11.10 |
  |---|---|---|---|---|---|---|
  | Y0a | base | 10.77 | 89 (131) | 66 / 3 / 28 | 20.8 / 40.0 GB | |
  | Y1 | FAULT_UNLOCKED=1 | 11.47 | 86 (104) | 59 / 2 / 26 | 20.8 / 40.2 | +0.37 |
  | Y2 | GUARD_HOT=50 | 9.43 | 104 (130) | 76 / 0 / 31 | 20.9 / 42.2 | -1.67 |
  | Y3 | LAZY_VIEWS=1 | 11.39 | 86 (104) | 58 / 8 / 26 | 20.9 / 40.5 | +0.29 |
  | Y4 | MAP_RUNS=1 | 11.23 | 87 (112) | 63 / 5 / 26 | 21.1 / 40.6 | +0.13 |
  | Y5 | INDIRECT_MERGE=1 | 11.37 | 86 (112) | 58 / 3 / 27 | 21.0 / 40.6 | +0.27 |
  | Y6 | HELPER_PRIORITY=2 | 11.59 | 85 (105) | 52 / 6 / 25 | 20.8 / 40.3 | +0.49 |
  | Y0b | base | 11.42 | 86 (107) | 55 / 5 / 26 | 20.7 / 40.2 | |
  | Yfinal | FAULT_UNLOCKED=1 + HELPER_PRIORITY=2 | 11.49 | 86 (114) | 58 / 4 / 26 | 21.1 / 40.4 | +0.39 |
  | Yfinal + FG | the same + FSR 3 2x (THIN=1, quality scale) | 10.16 rendered, 20.59 shown | 98 (103) | | 20.9 / 40.4 | |

  No crash in any run. Peak GPU memory 8.1-8.5 GB (7.8 with frame generation).
- Reading: the two bases differ by 0.65 fps (10.77 / 11.42): the noise of one 2-minute run here.
  By the PM's rule (>= 0.3 over the bases' mean, no new visible fault) FAULT_UNLOCKED (+0.37) and
  HELPER_PRIORITY=2 (+0.49) win; LAZY_VIEWS (+0.29) and INDIRECT_MERGE (+0.27) just miss; MAP_RUNS
  does nothing measurable; GUARD_HOT is clearly worse (-1.67 fps, more CPU preparing). Together
  (Yfinal) 11.49, which is the second base (11.42) within 0.1: the gains are within the noise and
  NOT proven. With frame generation the rendered rate falls by about 1.3 fps (THIN=1 is needed)
  and the shown rate is about twice it.
- Picture (the user, Yfinal and Yfinal + FG): no new fault; "quite pretty from some angles" but not
  the console picture; glasses and eyes reflect too much light; some textures still glitch (as a
  guard's arms did); models load late; the sky shows through the player character's head at times
  and small blocky jerks: also seen without frame generation, so faults of the layer, not of FSR 3
  or of the switches. With frame generation: the caption reads about 12 rendered / 25 shown.
- Also noted (the user): at each checkpoint load the player character stands still before and after
  the cutscene while the scene is still loading (the console has it ready): a loading-speed problem
  of the layer, not looked into.
- Tests (ef42499, same code as Session 11): fmt FAIL 3,471 diffs (Session 11: 3,471); clippy FAIL 42,
  all-features 40 (same); test 1,060 passed, the same 2 known failures, 46 ignored; policy pass.
- Recommended command (branch nvidia2, release build, its own cache folder), rendered fps:
  `.\scripts\play-nvidia.ps1 -Exe <title exe> -SeriesS -Dlss off -More @{VICTORY_THIN='0';
  VICTORY_NO_WATCHDOG='1'; VICTORY_FAULT_UNLOCKED='1'; VICTORY_HELPER_PRIORITY='2'}`
  (the two new switches by the PM's rule; their gain is within this sweep's noise).
  With frame generation (AMD's DLLs outside the repository):
  `$env:VICTORY_FIDELITYFX_DIR = "$env:LOCALAPPDATA\victory\fidelityfx"` then
  `.\scripts\play-nvidia.ps1 -Exe <title exe> -SeriesS -Dlss off -FrameGen -FgType fsr -More
  @{VICTORY_THIN='1'; VICTORY_NO_WATCHDOG='1'; VICTORY_DLSS_MODE='quality';
  VICTORY_FAULT_UNLOCKED='1'; VICTORY_HELPER_PRIORITY='2'}`
- What did NOT work or was taken back out: VICTORY_GUARD_HOT (slower); the first Y3 (browser open).
- Open problems found (not fixed): the run-to-run noise (0.65 fps) is larger than every gain but
  GUARD_HOT's loss: deciding between these switches needs longer or repeated runs; characters and
  scenery still load late; the sky through the player character's head; the reflection stand-in.
- Questions for the user / PM: whether to confirm the two winners with three runs each.
- Suggested next task: the CPU preparing itself (about 55-65 ms of a frame): where it goes, from a
  diag build's frame split, rather than more switches.

## Session 13 — where the CPU preparing goes, from the logs of Sessions 9-12 (no game launched) (2026-11-28)
- Branch / start commit / end commit: nvidia2 / 3759fb5 (= origin/nvidia2) / the commit that adds this
  entry. No code changed, the game not launched. Nothing pushed. (Session 12's three-runs-each
  confirmation was proposed but not run: the PM replaced it with this session.)
- Task given: from the existing run logs, split the ~55-65 ms of CPU preparing per frame, explain
  the three largest items, and propose one change for Session 14.
- Sources: the layer's own 5-second lines in the logs of Session 12's runs (Y0a, Y0b, Y1, Y3-Y6,
  Yfinal; Y2 left out, its switch made it slower) and of Sessions 9-10 (T1, T2, V1, W), each taken
  over its measured windows (medians). All are nvidia2 release builds, Series S, 1280x720 scene, the
  same command. The logs stay outside the repository. Two lines are used:
  - the frame line: "N preparing: copies N ms ...; N on the GPU; N copying back" (preparing = the
    submitting thread's time preparing submissions, its waits for the card taken out);
  - the submitting thread's line: everything that thread does per frame, waits included.
- Frame line, median of the 8 Session 12 runs: 86.8 ms a frame = 58.2 preparing (of it 27.0 the
  copies/refreshes) + 4.8 waiting for the card + 26.0 copying back (the landing threads' fetch and
  merge). Sessions 9-10 runs: preparing 59-69 ms, copying back 26-28 ms (more draws there).
- The submitting thread per frame (median of the 8 runs): 72.8 ms in its submissions + 7.0 ms after
  sending = 78 ms of work:

  | item (the log's words) | ms a frame | % of 78 ms |
  |---|---|---|
  | lists' steps: refreshes of host copies (buffers and textures, on several threads) | 26.1 | 33 % |
  | copies in the card's memory (what changed goes up; compares planned) | 10.9 | 14 % |
  | planning the steps done on the card | 8.8 | 11 % |
  | lists' steps: the rest (copies and fills, one-at-a-time refreshes before shader writes) | 7.8 | 10 % |
  | asking the write watch (which pages the title wrote) | 7.0 | 9 % |
  | after the sending (flight noted, landed lists reset and given back) | 7.0 | 9 % |
  | the title's bytes put in under flights' writes | 4.0 | 5 % |
  | the heaps' records (begun + waited for at the end) | 2.5 | 3 % |
  | sending the lists | 2.4 | 3 % |
  | waiting for flights | 1.8 | 2 % |

  Counts in the same window (Y0b): 6,133 refreshes a frame in 45 flushes, 5,256 of them found to
  copy nothing; textures refreshed: 18.4 ms a frame summed over all threads ("reading, hashing and
  detiling"); copies in the card's memory: 3,664 copies (1.9 GB), 44.1 MB sent up a frame in 44,293
  ranges (about 1 KB each), 6,960 pieces compared on the card, 662 fetched; steps done on the card:
  6,783 fills a frame planned.
- What these logs cannot split: inside the refreshes, reading vs hashing vs detiling, and buffers
  vs textures on the thread's own clock (only the all-thread texture sum is given); the recording of
  the title's lists, which happens on the title's and the translating threads, not on the submitting
  thread (where the ~13 ms per 1,000 draws is spent is not in these lines); the title's own CPU work;
  the card's own working time (its clock line counts time with work queued, ~99 %, waits included);
  inside the landing threads' 26 ms beyond "merging what was fetched 8.3 ms" and "read-backs and
  lists 5.4 ms" (Y0b).
- The three largest items, read in the code (crates/victory-gpu/src/host.rs, shadow.rs,
  host/card_steps.rs):
  1. Refreshes (26 ms). Before the card runs the title's lists, every range of title memory those
     lists read is brought up to date in our copy of it (`Batch::flush`): the write watch says which
     pages the title wrote, each touched copy is checked (`nothing_new`) and updated, textures are
     read again, hashed and converted from the console's tiled layout. Spread on several threads,
     but the thread waits for the slowest. Costly here because there are thousands of small ranges a
     frame (6,133), most with nothing new, and texture detiling is byte work on an 8-core 2019 CPU
     with DDR4 shared with the title.
  2. Copies in the card's memory (11 ms). Buffers kept on the card (`card_before`): the bytes that
     changed on our side since the last submission are sent up, and a compare of the ranges the
     shaders may write is planned to run after the lists. 44,293 separate ranges a frame, about 1 KB
     each: the cost is per range (bookkeeping, a copy command each), not per byte.
  3. Planning the steps done on the card (8.8 ms). `card_steps::plan`: about 6,800 fills of constant
     buffers a frame are done on the card behind earlier flights instead of waiting for them; for
     each submission the plan works out which fills depend on which (ranges read and written,
     joined in rounds) so their order stays right. Pure bookkeeping on the submitting thread,
     before anything is sent.
- Proposal for Session 14 (one change): speed up `card_steps::plan` for the common case. Most fills
  read constants no other taken step writes; a fast path that sorts the submission's written ranges
  once and admits a fill with one binary search, falling back to today's rounds only when a fill
  meets a write, would leave the order unchanged. Estimated gain: 5-7 of the 8.8 ms on the
  submitting thread, about 6-8 % of an 87 ms frame, i.e. +0.6 to +0.9 fps (not proven). Risk:
  medium, the order of steps done on the card (a mistake shows as wrong constants: flicker, wrong
  colours, a device error); card_steps.rs has synthetic tests of `plan` to extend, behind a new
  switch, off by default. Work: 3-4 h. Measure: one launch at Session 12's checkpoint (2 min still),
  the switch on, compared with the 8 Session 12 runs (planning 8.8 ms, preparing 58.2 ms, 11.1 fps),
  reading the planning item of the split line first (it must drop) and the fps second.
  Larger but harder: the 44,293 small sends a frame (join neighbouring ranges before sending; more
  bytes, fewer commands) and the refreshes found to copy nothing (5,256 a frame).
- Tests: no code changed; `cargo xtask policy`: pass (283 files).
- Questions for the user / PM: whether Session 14 takes the planning fast path or the joining of the
  small sends.
