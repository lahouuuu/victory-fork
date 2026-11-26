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
