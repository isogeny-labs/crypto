# Addendum 2 (2026-10-29, after the PR review): provenance of the registered sources

This addendum supersedes the sentence in `AMENDMENT-1-NOTE.md` that says the original source of `digit_fall.rs` "is not kept". That note and `AMENDMENT-1.md` are left as written; nothing historical is rewritten.

**What was recovered.**
- **`registered/digit_fall.rs.v0`.** I rebuilt the registered source by reversing, step by step, the one patch of Amendment 1 (the `plant(...)` function and its use) on the committed `digit_fall.rs`. Its sha256 is `d811d6f8…eb2`, **equal to the hash in `REGISTRATION.sha256`**. A match to a 256-bit hash taken before any counted run is the evidence that it is the registered text, not an approximation.
- **`registered/run.sh.v0`.** I rewrote the registered launcher from the heredoc that created it. Its sha256 is `8726b4eb…6b4`, **equal to `REGISTRATION.sha256`**.
- **`registered/run.sh.v1`.** The committed launcher at the time panels B-E ran. Its sha256 is `ab2d4ebc…3a`, equal to the line recorded for it in `REGISTRATION-AMENDMENT-1.sha256` (that line was written after the run, as `AMENDMENT-1-NOTE.md` says).

**What this does and does not establish for panel A.**
- **Established.** Panel A (60 draws) was run with a binary whose hash is in `BINARY.sha256` (`b7fc4110…ffb`), built from the source whose hash is registered. That source is now committed and verifies.
- **Reproduced.** Rebuilding `registered/digit_fall.rs.v0` and rerunning panel A's `s = 3` and `s = 5` cells (12 draws each, registered seeds) reproduces the committed records line for line, except for `secs`. (The amended binary reproduced the `s = 3` cell earlier.)
- **Not established.** The rebuilt binary's sha256 is **not** equal to `BINARY.sha256` (rustc embeds build paths, so a rebuild is not bit-identical). So the binary that produced panel A is identified by its recorded hash and by behaviour on two cells; the binary file itself is not committed and a bit-identical rebuild was not achieved. Panel A's other cells (`s = 2, 4, 6, 7`) were not rerun here.

**Other changes in this commit (review items 1 and 3).**
- **`run.sh`** now takes a required, non-existing output directory and never writes to the committed `results/`. `test_run_sh.sh` checks this: a failing substitute binary and an existing output directory both leave the committed records untouched, and the commands issued match the launchers that produced the records (`analysis/run_sh_test.txt`).
- **Panel D** has 8 independent instances (4 at `s = 4`, 4 at `s = 5`) and 4 replay checks: the first-fall-only `s = 4` records share seeds, primes, curves and targets with the closure `s = 4` records. README, `analysis/`, `report.pdf` and the PR description now say so, and `derive.mjs` separates replays from instances. No new data was generated.
