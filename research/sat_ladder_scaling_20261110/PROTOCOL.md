# F4: SAT on the bit-level x-only ladder — conflict-count scaling and clause reuse (protocol, frozen 2026-11-10)

Moonshot M2 (main session) and A1 (`research/prime_moonshot_20261110/ideas-generator.md`).
Prior art: `research/prime_field_smt_20261005` (cvc5 `QF_BV`/`QF_FF`: `unknown` at 20 bits in
120 s on the decomposition system), `research/pdp_ml_sat_ceiling_20261027`. This study measures
the **scaling exponent** of a CDCL solver on the *ladder* formulation (the scalar's bits are the
only free inputs), which no prior study measured, and whether learned clauses amortise across
targets (advice; falls under `S·T² ≳ n`). Class: boundary diagnostic.

## 1. Encoding
Curve `E/F_p` of prime order `r`, `n = ⌈log₂ p⌉ ∈ {10, 12, 14, 16, 18, 20}`, one prime-order curve
per `n` (`p` = largest prime below `2^n`, `(a,b)` from seed `F4/curve/<n>`), `G` fixed.
Unknown: `k` as `⌈log₂ r⌉` Boolean variables. Circuit: projective x-only Montgomery ladder on
`(X : Z)` from `x(G)`, each step the standard differential add + double (`≈ 10` modular
multiplications), Tseitin-encoded with schoolbook multipliers, adders, and modular reduction by
quotient variables (`prod = q·p + rem`, `rem < p`); final constraint `X ≡ x_Q · Z (mod p)`,
`Z ≠ 0`. Every SAT model is decoded and checked: `[k]G = Q` exactly (fatal otherwise).

## 2. Solver and arms
CaDiCaL (vendored through a Rust crate; fallback `varisat`, recorded) with default options.
- **Arm FRESH:** new solver per target.
- **Arm REUSE:** one incremental solver per curve, target `x_Q` bits as assumptions, 20 targets in
  a seeded random order; learned clauses persist.
20 targets per `n` (seed `F4/target/<n>/<i>`, `k` uniform in `[1, r−1]`). Conflict budget
`10^7` per solve (censored rows are reported as censored, never imputed). Metrics: **conflicts**
(primary), decisions, propagations; wall seconds descriptive only (the host is shared).

## 3. Predictions (frozen)
P1. Exponent `α` of median FRESH conflicts in `n` (`log₂ conflicts ≈ α n + c`, fitted on the
uncensored `n`): `α ∈ [0.85, 1.15]` (exhaustive-like); rho is `0.5`.
P2. REUSE / FRESH median conflict ratio at each `n` ≥ 0.5.
P3. All models decode to the true `k`.
Prior that `α` upper CI `< 0.6`: 0.01.

## 4. Decision rules
1. **Instrument failure:** P3 fails, or fewer than three `n` are uncensored.
2. **Lead alive:** `α` upper 95 % CI `< 0.6` with `n = 18` or `20` uncensored, **or** REUSE ratio
   `< 0.2` at two consecutive `n`. Then replicate with a second solver and 40 targets.
3. **Closed:** otherwise. Reading: CDCL on the ladder circuit is exhaustive-like and clause reuse
   does not amortise below the generic bound at these sizes.

## 5. Route and outputs
Local host; counts are the result. `results/run.json`, README, SVG (log₂ conflicts vs `n`, both
arms, censoring marked), REPORT.pdf, session-log entry. Task card after `FREEZE.sha256`.

## 6. Amendment and disclosure before the full run (2026-11-10)
Predictions and rules were committed (`c8bbc80e1a`) before the tool existed. The implementer's
development runs (not committed, not scored): at `n = 4, 5, 6` with 3 targets, FRESH median
conflicts 6,222 / 9,856 / 185,353 and REUSE 315 / 1,024 / 52,295 (ratios 0.051 / 0.104 / 0.282),
`α_FRESH ≈ 2.45`; at `n = 10, 12` every solve was censored at 2·10⁴ and a single `n = 10` target
did not finish at 2·10⁵ conflicts. Conflicts grow about 5× per bit, so the frozen grid
`n ∈ {10, …, 20}` at 10⁷ conflicts would be censored throughout and trip rule 1 by construction:
a grid-design error, not a measurement. **Amendment (grid and budget only):** `n ∈ {4, 5, 6, 7,
8, 9}`, 10 targets per `n`, conflict budget `4·10⁶`, `--threads 2`; one prime-order curve per `n`
as before. P1–P3 and rules 1–3 are unchanged (rule 1's "fewer than three `n` uncensored" now
refers to this grid; rule 2's thresholds are unchanged, and if its clause-reuse clause fires on
these sizes the replication it prescribes is run and its reading stated with `α_REUSE`). Instrument
facts fixed by the tool: CaDiCaL 3.0.1 via `cadical-sys 0.8.0`; propagations are
`propagations.search`; x-only decoding accepts `k' ∈ {k, r − k}` (P3 reads `x([k']G) = x_Q`);
the reduction is `prod + q·c = q·2^n + rem`, `c = 2^n − p`, `rem < p` as no carry out of
`rem + c`; the ladder starts from `(O, G)` over all `nk` bits with `k < r` as a comparator;
7 tests pass and the `n = 6` planted test is `#[ignore]`d (≈ 3·10⁵ conflicts). Expectation stated
before the run: `α_FRESH` well above 1 and P1 failing upward; the amended grid is for measuring
the exponent, not for reaching rho.
