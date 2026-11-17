# Correction: the bootstrap drew only odd-numbered rounds (2026-10-29)

Found by the independent validator
([`../../../benchmarks/20261029-ic-aimed-gather/validator-report.json`](../../../benchmarks/20261029-ic-aimed-gather/validator-report.json),
findings F1–F5) after this session's summary was published, and confirmed
before any result was shown as validated.

## The defect

Both sessions' summaries (`../summarise.sh`, `summarise.sh`) bootstrap the
median with the LCG `s → (s·1103515245 + 12345) mod 2^31`, the resample
index being `s mod n`. jq's numbers are doubles. The product exceeds 2^53
on the first step and is rounded, the rounded states are almost always
even, and with an even number of rounds `s mod n` is then almost always
even. **99.6% of the resample draws fell on odd-numbered rounds** in both
sessions (`as_run_odd_round_share` in `summary-corrected.json`). The
published "95% bootstrap intervals" were therefore intervals over about
half the rounds, and in this session over three of the six orders only
(odd rounds use permutations `a q c`, `q a c` and `c a q`).

The 5-pair A/A of the first session has an odd `n` and is unaffected.
Medians, counts, identity and isolation figures are unaffected: only the
intervals are.

## The correction

`correct_bootstrap.sh` recomputes both sessions from the raw records with
the same constants, seed, resample count and percentile indices, the step
done exactly and (since the second correction below) the index taken from
the high bits (the step splits the multiplier as 16838·65536 + 20077 so every product
stays below 2^53; the first states match an integer reference). It writes
`summary-corrected.json` with the as-run intervals beside the corrected
ones. No record and no earlier summary is changed.

| session | metric | A/B 95% as run | A/B 95% corrected | A/A 95% corrected | margin | verdict corrected (as run) |
|:--|:--|:--|:--|:--|--:|:--|
| 2026-10-29 | collect | 1.090 – 1.160 | **1.068 – 1.136** | 0.977 – 1.026 | +0.042 | **established** (established) |
| 2026-10-29 | whole set-up | 1.021 – 1.116 | 1.020 – 1.082 | 0.946 – 1.015 | +0.005 | rule met only after the fix; not claimed (not established) |
| 2026-10-23 | collect | 1.065 – 1.160 | 1.082 – 1.149 | 0.944 – 1.087 | −0.005 | not established (not established) |
| 2026-10-23 | whole set-up | 1.018 – 1.068 | 1.018 – 1.080 | 0.953 – 1.121 | −0.103 | not established (not established) |

## Second correction: the first correction was parity-stratified

Found in review of #2858 (and as finding F16 of the validator's second
pass). With the step done exactly, `s mod 2^31` alternates parity, so
`s mod n` for an even `n` made every resample draw exactly `n/2` odd and
`n/2` even rounds: a bootstrap stratified by round parity, not independent
draws with replacement. `correct_bootstrap.sh` now takes the index from the
generator's high bits, `floor(s·n / 2^31)`; the per-resample count of odd
rounds then ranges 11–32 at `n = 42`, as independent draws do. The table
above uses it. The first correction's intervals stay in
`summary-corrected.json` as `ci95_stratified`; no bound moved by more than
0.003 between the two, and no verdict changed.

## How the result is read

- **Collect phase, 2026-10-29: established.** It holds under the as-run,
  the stratified and the corrected generator, by 0.042 under the corrected one, and the
  validator found it under 500 of 500 independent seeds.
- **Whole set-up, 2026-10-29: marginal, not claimed.** The verdict flips
  with the generator, the margin is 0.005, and the generator was fixed after
  the data were seen. Reported as is.
- **2026-10-23:** neither verdict changes.
- The README's earlier explanation of the lopsided A/B interval ("the
  per-round ratios cluster just above the median") was wrong: the odd
  rounds' collect median is 1.137 and the even rounds' 1.072, and the
  interval only saw the odd ones.
