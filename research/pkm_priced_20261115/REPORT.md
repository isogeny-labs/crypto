# Priced symmetric m = 4 PKM oracle: results (2026-11-15)

Frozen protocol `PROTOCOL.md` (commit `b0e59c47`, before any frozen-size data). Run on a RunPod CPU pod
(`results/RUN.json`). Every number is measured unless labelled derived, and all are unverified assertions by the
producing session [pcc:unverified]. This is Stage 0 only: no cold pipeline, no measured yield, so no leaderboard row and
no validation gate. Each priced S below is derived from a measured per-target cost and a derived yield.

## Run

- **Attempts:** three RunPod pods (cpu5g, 8 vCPU, rust:1.97-bookworm). [pcc:unverified]
  - Attempts 1 and 2 failed to build and produced no data. The source archive lacked first the path dependencies, then
    the `include_str!` data files.
  - Attempt 3 (pod `hf59kkl8qq5vjd`, 07:37–07:53Z) completed. The build exited 0, every step returned 0, and
    `results/SHA256SUMS` verifies.
- **Archive check:** `cargo check --release --locked` passed locally on that exact archive before attempt 3.
- **Cost:** about $0.12 for all three attempts.

## Stage 0 (p = 786433, 24 targets per N, F4 on 8 / 4 of them)

| | N = 12 | N = 16 |
|---|---|---|
| curve: subgroup order r, liftable points of V | 19, 6 of 8 | 89, 8 of 16 |
| F4 stage, field ops per target | 400,958 | 1.0790 × 10¹⁰ |
| grid-built S₅, mulmod per target | 2,951,786 | 2,951,787 |
| grid build / F4 | **7.362** | 2.74 × 10⁻⁴ |
| parametric S₅ per target, mulmod | 3,960 | 3,960 |
| parametric precompute, once per curve | 2.632 × 10⁷ (8.9 grid builds) | same |
| coefficient check, parametric against grid | 12,375 compared, **0 mismatches** | 12,375, **0 mismatches** |
| planted decomposition vanishes in both modes | yes | yes |

**Derived priced S / rho-neg at w = 0.1 GAE**, from the summary mode with amortisation over 100 solves: [pcc:unverified]

| | N = 12 | N = 16 |
|---|---|---|
| F4 only (as the leaderboard quotes) | 2.10 × 10⁷ | 1.00 × 10⁹ |
| grid build plus F4 | 1.76 × 10⁸ | 1.00 × 10⁹ |
| parametric build plus F4 | 2.13 × 10⁷ | 1.00 × 10⁹ |

## Against the frozen predictions and kill rules

- **(a) Grid build / F4 at N = 12:** 7.36, against a prediction of 1.7–2.3 (band [1, 4]). It is outside the band and above it. [pcc:unverified]
  Kill 1 (< 0.2×) does not fire. The per-target S₅ build was the larger stage at this size, and it is **not priced** in
  the leaderboard's 2.75 × 10⁵× component ratio.
- **(b) Parametric build:** 3,960 mulmod per target is 0.99% of F4, within the predicted ≤ 1%. The equality is exact on
  every target and on the planted target, so kill 2 does not fire. The precompute breaks even after about 9 targets
  (prediction: within 100), so kill 3 does not fire.
- **(c) Priced S/rho:**
  - At N = 12 it is 2.1 × 10⁷ (parametric), which is 20–100× above the predicted 2 × 10⁵–10⁶. The curve's derived
    yield is 3.06 × 10⁻⁴, below the lead's 7.13 × 10⁻⁴ (only 6 of 8 points of V lift). The protocol reports the result
    with this yield; the remaining factor is not explained here.
  - At N = 16 it is about 1.0 × 10⁹, the upper edge of the predicted 10⁷–10⁹.
  - Kill 4 (< 10⁴) does not fire.

## Reading

- **Accounting correction to an existing row.** Built per target, the S₅ stage multiplies the N = 12 per-target cost by [pcc:unverified]
  8.4. Relabelling is not an improvement, but the leaderboard's symmetric PKM entry should say that its ratio excludes
  this stage.
- **Real component improvement.** The parametric precompute replaces a 2.95 × 10⁶ per-target build with 3,960 operations
  (745× per target) plus a one-off 2.6 × 10⁷ set-up. With it, the end-to-end cost at N = 12 is within 1% of the F4-only
  figure.
- **At N = 16** F4 dominates by a factor of 3,600, so the build question is moot. The priced distance to rho there is
  about 10⁹.
- **Gap to rho.** These are derived distances to matched rho-neg, not results that beat it: about 2 × 10⁷ at N = 12 and
  about 10⁹ at N = 16. They are reported as distance remaining, and Stage 1 (cold pipeline, measured yield) is required
  before any row exists.

Graph: `cost_by_stage.svg`. Raw outputs: `results/`. The two ecbench `records.jsonl` files (11.7 MB each) are kept in S3
and referenced by SHA-256 in `results/sess/*/records.jsonl.pointer`.
