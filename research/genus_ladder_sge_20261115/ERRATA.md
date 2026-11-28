# Errata (from the validator replay, `VALIDATION_REPORT.md`, 2026-11-21)

The validated files are left as frozen; corrections live here. [pcc:unverified]

- **E1, narrative figures in RESULTS.md** (the conclusions they serve still hold):
  merge multiplications over `la_mul` are up to 3.84e-4, not "at most 2.5e-4"; merge
  additions are 0.46 % to 2.86 % of `la_add`, not "0.7 % to 2.3 %"; the `la_add` ratio is
  0.36 to 0.66, not "0.39 to 0.63".
- **E2, source freeze.** `frozen.sha256`'s entry for `genus_ladder.rs` fails at later
  commits: the only difference is `pub(crate) fn sge_reduce` (commit c9749cbffa); the
  frozen blob exists at a552dc2ee8.
- **E3, bootstrap order.** `paired.py`'s bootstrap endpoints depend on row order (up to
  2.3e-3 relative); point estimates do not. Sort rows by key before re-running it.
- Not replayed: genus 5 and the producer's raw `sge_g34.jsonl` (in the artifact store,
  unreachable from the validator's host).
