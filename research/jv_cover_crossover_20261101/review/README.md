# Independent review of the JV cover crossover study (2026-11-01)

**Reviewed:** [`../`](../), merged as PR #3194 at `main` 4def6153d1.

**Reviewers:** two independent, breakthrough-tier agents, each in its own session. Neither read the other's report. Neither saw the main session's conclusions.

| Review | Report | Verdict |
|---|---|---|
| Validator | [`validator/REPORT.md`](validator/REPORT.md), [`validation_report.yaml`](validator/validation_report.yaml) | **passed.** The receipts are admissible. The claim stands only as stated: against rhodp, in the harness unit, at w = 0. |
| Red team | [`red_team/REPORT.md`](red_team/REPORT.md), [`analysis.txt`](red_team/analysis.txt) | **Refuted as a claim about Pollard rho.** It survives only as a claim about the study's non-competitive reference. |

Graph: [`forest.svg`](forest.svg) / [`forest.png`](forest.png). PDF: [`REVIEW.pdf`](REVIEW.pdf).

## Where the two reviews agree

**Integrity (measured).** The data and computations hold up:
- Every number in the study reproduces from the raw records, by independent Rust code.
- The study's `fit/` tool rebuilds byte-identical outputs.
- The frozen protocol hash matches.
- A cover run (p = 1301, seed 3) replays bit-for-bit.
- The solved log was checked by a disjoint group-law implementation.

**Reproduction failure explained (measured).** The 0.949× drop in multiplication count is entirely the `JV_LIN` default changing from sparse to fast (commit `e6ff48ea72`). With `JV_LIN=sparse`, the current binary reproduces the predecessor's records bit-for-bit. The per-prime verdicts are unaffected. Under the old linear stage, the 1451 verdict would become "not resolved".

**Calibration confounded (derived from code).** `rho_e_dp` polls its stop flag only every 2^20 steps per walker. With 4 walkers, this adds about 0.20 S-units at p = 251, which explains most of the +0.28 calibration gap. At the new primes the effect is ≤ 0.0014. The red team's clean null gives 1.266 ± 0.059 (n = 120), against a theoretical 1.273. So rhodp is unbiased at the new primes, and that reference stands.

**The claim is unit- and reference-bound (derived).**
- **Weak reference.** rhodp has no negation map; it is the plain walk.
- **Generous unit.** The study charges 331 F_p multiplications per rho step: schoolbook arithmetic with a per-step inversion. The binary's own tuned price is 0.416 of that, and the cover's count has no comparable slack.
- **Repricing the rho step at the tuned price reverses the verdict** at every prime, giving 2.2–3.5×.
- **§5a.** Pricing the cover's own emitted lower bound `f4_native_ops` at any w > 0 puts the cover above every reference.
- **Thin statistics.** With 3 seeds, a t-interval resolves nothing. The 1451 upper end is 0.970, only 3% below 1.

## What can be said

Only one statement is supported:
- On the JV weak class, at p = 1451 and 1601, the cover's counted F_p multiplications were 0.78× and 0.59× those of the harness's no-negation, schoolbook, per-step-inversion r-adding walk on the same instances.
- This is at w = 0, with F4 native work and F_p additions left unpriced.

This is **not** a result about Pollard rho.

**Distance remaining** to the matched, negation-map rho (red team, derived). This is the cover's cost as a multiple of that rho's cost:

| p | Group additions | F_p multiplications (textbook prices) |
|---|---|---|
| 1301 | 1.48 [1.24, 1.76] | 3.54 |
| 1451 | 1.22 [1.04, 1.44] | 2.92 |
| 1601 | 0.92 [0.72, 1.24], not resolved | 2.19 |

**Trend.** The p^−2 trend is supported: slope −2.10 [−2.21, −1.99]. Extrapolated, the fair crossover is at p ≈ 2300–2400 (model). That is beyond the implementation's u64 limit of p ≤ 2039.

**Scope.** The class is the Joux–Vitse weak class (ePrint 2011/020). The result bears on no prime-order or deployed curve, and on no row of IC leaderboard tables A–C. Nothing here enters the leaderboard or `benchmarks/`.

## Next steps (both reviews)

1. **Change the reference.** Make the pooled negation-map rho (≥ 50 runs) the primary reference. Report S both in group additions and in textbook F_p multiplications. This needs no new cover runs.
2. **Fix the stop poll.** Make `rho_e_dp` poll its stop flag per DP or every 2^14 steps, or calibrate single-threaded.
3. **More seeds.** Run ≥ 8 cover seeds per prime before stating any verdict at 1601.
4. **Complete the §5a count.** The cover arm should emit its F_p additions and a complete F4 native count, so the §5a band covers both arms.
5. **Improve the cover.**
   - At p = 1601, cut F_p multiplications 2.2×.
   - Or lift the u64 limit to reach p ≈ 2400.

## How this was made

- **Numbers:** computed by the reviewers' own Rust tools: `validator/recompute.rs`, `validator/verify_log.rs`, `red_team/analysis/`.
- **Controls:** 62 red-team runs, in `red_team/controls/`.
- **Validator report:** the validator's harness blocked it from writing `validator/REPORT.md`, so the main session placed the agent's returned text there unedited.
- **Graph:** `forest.svg` is a hand-laid plot of the numbers in the two reports. `awk` was used for layout only. The PNG and PDF are LibreOffice renders.
