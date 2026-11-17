# Protocol: cubic-GHS genus-3 index calculus against the strongest rho on `E0(F_{q^3})`

2026-10-23. Frozen before any measured data of this study (the only prior
run of the instrument is a smoke run at `q = 2^12`, not recorded here).
Item 2 of the extension-field programme.

## Question

Round 4 of the IC improvement rounds (`research/ic_improve4_20261015/01-genus3-relations/`)
lowered the cost of genus-3 double-large-prime index calculus after the
cubic GHS transfer by about 0.45× in total since the last rho comparison
(`research/ic_next10_20261014/07-cubic-ghs-genus3/`, which found IC 2.29×
an ideal negation rho at `q = 2^15`). No round since has had a rho arm.
**At which `q` in `2^14 .. 2^20` is the round-4 pipeline below the
strongest rho on the same instances, in the same unit and the same
binary, and how far is it from rho where it is not?**

Scope: the structured class the transfer applies to (`E0 / F_{q^3}`,
`q` prime, `#E0 ≡ 0 (mod 4)`, a cubic GHS cover by a genus-3 hyperelliptic
curve over `F_q`). Not prime-order curves and not generic curves over
`F_{q^3}`. Theory predicts `Õ(q^{4/3})` for genus-3 double large primes
against `q^{3/2}` for rho (Gaudry–Thomé–Thériault–Diem; Diem), so a
crossover is the expected outcome; the measurement locates it.

## Baseline (the current verified IC pipeline)

Round 4's arm `w:lpsgebw2@cx+C1C2`: R1 relation primitives, C1 field-level
splitting test and root finding, C2 32 lockstep walks with one batched
inversion, large primes as columns, structured Gaussian elimination, block
Wiedemann (2, 1); `kappa_R2(b) = exp(0.2669 − 0.0941 (b − 20))`,
`cx = 16` for `b ≤ 17`, else 8. This study changes nothing in it.

## Instrument

`examples/ghs3_vs_rho_20261023.rs`, a copy of round 4's
`ic_improve4_01_genus3_rel.rs` with additions that do not touch the IC
path (header of the file). Driver: [`run.sh`](run.sh). Instances: round 4's
(`seed = 20261014·1000 + 10 b + curve`), `b = 14 .. 20`, curves 0–2, one IC
target per curve (round 4's).

**Unit.** `M_q + ⌈log₂ q⌉·I_q + M_ℓ` (round 4's), with the sensitivity
`c_ℓ = 9` (`M_ℓ` = 9 `M_q`) reported beside it.

**The rho reference.**

- **Walk:** the library's `ic_boundary::rho_walk_with(NegationClasses,
  RhoWalk::negation())`, the walk of ecbench's `rho.negation`, on `E0`
  through a `CountedGroup` whose operations are the instrument's counted
  `F_{q^3}` affine additions. 8 targets per curve (the IC target first),
  every answer checked.
- **Price per group operation (`GAE`):** 32 affine additions sharing one
  inversion (Montgomery's trick), with Karatsuba `F_{q^3}` products (6 `M_q`)
  and 5-product squares, every sum asserted equal to the instrument's
  addition. This is cheaper than the instrument's own addition (schoolbook,
  one inversion each), so it favours rho; both are recorded.
- **`R_ideal`** = `√(πℓ/4) × GAE` (an ideal negation rho with no walk
  overhead) — the **primary** reference, the most favourable to rho.
- **`R_meas`** = per-curve mean of measured group operations × `GAE`.
- **Symmetry:** the instance check records `j(E0)` and whether it lies in
  `F_q`. If it does on any instance, negation may not be the strongest
  rho; such an instance is excluded from every claim and reported.

## Gates and controls (numeric, fixed now)

| check | accept when |
|---|---|
| gate 0 (instrument unchanged) | every IC cost of every arm equals round 4's recorded `results/run/b<b>_c<c>_w.jsonl` value exactly |
| correctness | every IC and rho answer correct |
| rho calibration | at each `b`, pooled mean over the 24 runs of (group operations / `√(πℓ/4)`) in [0.85, 1.30] |
| symmetry | `j(E0) ∉ F_q` on the instance |

If gate 0 or correctness fails, nothing is reported as a result. If the rho
calibration fails at a size, `R_meas` at that size is withheld (`R_ideal`
stands, being a model).

## Decision rule

At size `b`, **"IC below the strong rho"** holds iff on **every** one of
the three curves the IC cost (`+C1C2`, `c_ℓ = 1`) is below **both**
`R_ideal` and `R_meas`, **at every weight of the §5a band**:

- IC unpriced: `hash_ops_uncharged` (relation-phase and matrix-assembly
  hash-table operations);
- rho unpriced: `canonicalisations_uncharged` plus two table operations per
  distinguished point (for `R_ideal`: one canonicalisation per ideal
  operation, no table operations);
- each priced at `w ∈ {0, 0.1, 1}` GAE per operation.

Otherwise the size is reported as "IC above rho" (all curves above at all
weights) or "unresolved". The `c_ℓ = 9` result is reported separately and
labelled as a sensitivity.

**Distance remaining.** At every size, the per-curve ratio IC / `R_ideal`
and its geometric mean. **Crossover:** least-squares fit of
`log₂(IC / R_ideal)` against `b` over all 21 instances, the root and a
bootstrap 95% interval (resampling curves within size, 10,000 resamples).

## What would count against the prediction

- No size at which the rule holds, and a fitted slope of `log₂ ratio` that
  is not negative.
- A crossover interval that lies above `b = 20` reports the gap as
  remaining and is not a failure of the instrument.

## Validation gate

The evidence directory gets a `VALIDATION.json` only after an independent
replay of sampled jobs (IC and rho answers and counts reproduced
bit-for-bit) and an independent recompute of the headline ratios from
`raw/`, by validator agents that did not write the analysis.

---

## Deviations log (appended 2026-10-23; the text above is unchanged and was committed before any data)

1. **Execution only.** The first run (card `CARD-ghs3-vs-rho-20261023-1`) was killed by the agent
   harness's 30-minute background limit, not by the tool or the card's timeout; every child
   process died with it. Two jobs had completed (`raw/b20_c1.jsonl`, `raw/b20_c2.jsonl`); killed
   jobs left no record (a job writes its line only when it finishes). `run.sh` skips a job whose
   record exists, and every job is deterministic in `(b, curve)`, so the run is resumed with the
   **same bound binary** (sha256 `7847d102…`, not rebuilt) and no count changes. The resume is a
   new card launched detached from the agent harness. The two completed records are kept as
   written; the validator's replay covers them like every other job.
2. **Execution only (second interruption).** The container was reclaimed at about 07:35-08:18 UTC while the agent session was idle; every process died (uptime reset), the disk survived. The resume card `CARD-ghs3-vs-rho-20261023-2` wrote no receipt. Completed records are still only `b20_c1` and `b20_c2` (killed jobs write nothing). A third card resumes with the same binary; the session is kept active by a harness-tracked waiter until the run ends.
