# CLAUDE.md — rules for every Claude Code session in this repo

Claude Code reads this file automatically at the start of every session.

## What this project is

A fork of VIctory: a compatibility layer (Rust, Windows, Direct3D 12) that runs
an Xbox Series title natively on Windows from files the user supplies.
Read `README.md` (status, known problems, run history), `docs/architecture.md`
and `docs/asset-policy.md` before touching code you have not seen yet.

## Rule #1 — asset policy (never break this)

Nothing from the game is ever stored in this repository, in any form:
no assets, no executable bytes, no strings or tables from it, no decompiled or
disassembled code, no shader dumps, no screenshots, no converted data.
That includes `SESSIONS.md`, commit messages and code comments.
Allowed: our own code, build hashes, per-build offsets under our own names,
byte-pattern signatures, small constants we write, synthetic test data.
Full list: `docs/asset-policy.md`. `cargo xtask policy` is authoritative.

Game data, cache and saves live OUTSIDE the repo (paths in `victory.toml`).
Never copy anything from those folders into the repo.

## Session protocol

**At the start**
1. Run `git status` and `git log --oneline -5`. Check that the branch and the
   last commit match what the prompt says. If they do not, stop and report it.
2. Read the last 3 entries of `SESSIONS.md`.

**While working**
- One task per session, the one in the prompt. If you find another problem,
  write it down in the report instead of fixing it.
- Follow the repo's conventions: new speed/behaviour changes go behind a
  `VICTORY_*` environment switch, off by default, described in a comment where
  it is read. Approximations of console behaviour are marked `shortcut:`.
- Never claim a gain without a measurement. Give run numbers, scene, frame
  times before/after, and say when results are mixed or unproven.
- If you need an answer from the user, stop and ask one clear question.

**Before committing** (all must pass, same as CI)
```
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
cargo xtask policy
```
Commit on the working branch named in the prompt. **Do not push**; the user
pushes after review.

**At the end**
Append one entry to `SESSIONS.md` using the template at the top of that file.

## Files you must not edit

- `PM_HANDOFF.md`: it belongs to the project-manager chat. Read-only for you.
- Older entries of `SESSIONS.md`: append only, never rewrite history.
