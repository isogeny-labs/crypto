# Table B replay on the committed tree

Re-run of `scripts/replay_B.sh` by the coordinating session on 2026-10-24,
with `ic` built from the tree this directory is committed in. The earlier
replay (`raw/replay_B/`) used binaries built while other agents' edits were
still uncommitted.

- `ic` sha256: `09a58b513935438fbc74f461c02de11a95aea0371b865245caca4b9db0593802` (`cargo build --release --bin ic`, branch
  `claude/ecdlp-leads-analysis-l7f87y` on main `a0b65d9f5`).
- `compare.json` (`scripts/compare_B.py`): 13 of 13 instances identical to
  the archive, 1,257 runs; the archive re-pricing identity holds.
- Against `raw/replay_B/`, only wall-clock fields and per-host calibration
  timings differ. Counts, ratios and recovered logarithms are identical.
