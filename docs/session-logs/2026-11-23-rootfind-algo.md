# 2026-11-23: root finding, algorithmic (counted-cost) improvement

## What was done
- Surveyed about 75 prior root-finding studies with a delegated read-only
  survey, cross-checked against key directories.
- Wrote `research/rootfind_algo_survey_20261123/SURVEY.md` (merged in
  #4931): incumbents, levers tried, why the remaining levers are
  constants. Added an addendum on the binary F4 direction (this
  follow-up PR).

## Decisions made
- Proposed no new constant-factor lever as an "algorithmic improvement".
  - On prime fields the decomposition is generic: S ≈ √(2A)·c,
    exponent → 1/2 (BSGS).
  - Binary m = 2 on a subspace is generic too (exponent 2/3).
  - The char-2 S3 bilinear reformulation is still 2^d work per target.
  - Large primes are a measured negative here.
- The owner chose binary F4, then "all in parallel": the matched F4 gate,
  a new degree lever, genus 2–3, and this close-out.

## Open threads
- Three parallel follow-ups: the program-B matched F4/F5/F6 gate, a
  preregistered degree-lever probe, and genus 2–3 root finding. Each
  lands in its own PR.

## Errors
- The research-kb binary and the lead-atlas checkout are absent from this
  container; the survey read `research/` directly.
- The #4931 merge landed before the addendum push; this PR carries the
  addendum.

## 21:05Z: degree-lever direction published

### What was done
- The parallel workflow (three directions, each with a red-team review) finished. Published the degree-lever direction: `examples/rootfind_degree_lever_20261123.rs` and `research/rootfind_degree_lever_20261123/` (README, PROTOCOL, raw, analysis, report.pdf, figure, the worker's own SESSION_LOG.md).
- A fix-up pass applied the red-team review: a G0-failure box (62/72 preregistered, relaxed post hoc to d_F plus closed dims), A/A reworded as uninformative, the "wrong sign" hedged as one integer step, the window-base sentence fixed, and the source-hash gap and host-load censoring recorded.

### Decisions made
- Negative result `[pcc:unverified]`: the symmetrised frame does not lower the d_F slope at m=3 over n=9–18. Its Macaulay matrix is smaller by a shrinking constant only. No leaderboard or benchmarks row.

### Open threads
- The G1 selftest output was not saved (no binary survives); rerun after a build.
- The other two directions (matched F4 gate, genus 2–3 sibling) are being corrected per their red-team reviews, then published as separate PRs.

### Errors
- G0 failed as preregistered (see the README). There was no cargo build because the disk was at about 750 MB.
