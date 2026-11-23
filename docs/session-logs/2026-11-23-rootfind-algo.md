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
