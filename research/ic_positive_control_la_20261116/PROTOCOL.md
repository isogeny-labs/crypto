# Positive control, linear algebra: coordinate Wiedemann and factor-base re-balance

Frozen 2026-11-16 before any measured run (`frozen.sha256`). Every number this
study produces is `[pcc:unverified]` until a validator replays it.

## 1. Question

After the Cardano–Lucas port (`research/ic_positive_control_lucas_20261116`,
arm M) the linear algebra is 54–74% of the genus-3 positive control's native
multiplications. A diagnostic split at p = 32771 (N = 1933 core columns,
nr = 1983 rows, one curve, not part of any verdict) gave: Krylov sequence
2.32e7 (dominated by the random diagonal `D` of `AᵀDA`), Berlekamp–Massey
7.6e6, reconstruction of the whole solution vector 1.53e7.

Only one coordinate of the solution is needed, the target's logarithm. This
study measures two solver changes and one re-tuning against the matched
negation rho on the §5a band.

## 2. Arms (same binary, same instances, same seeds)

| arm | splitting test | linear algebra | factor base `nb = ceil(cc · p^{2/3})` |
|---|---|---|---|
| M | Cardano–Lucas + Cardano roots | baseline Wiedemann on `AᵀDA` | cc = 2 |
| N | as M | coordinate Wiedemann on `AᵀDA` (unit projection at the target column, answer read from the Krylov sequence; no reconstruction) | cc = 2 |
| S | as M | coordinate Wiedemann on a random square row subset `A'` (no `AᵀA`, no `D`) | cc = 2 |
| S* | as M | as S | cc = cc* from the training sweep (section 4) |
| R | negation-map rho on the order-`r` subgroup | – | – |

A coordinate answer is certified by group arithmetic (`[t]G = T` twice);
a failure falls back in the same attempt to the baseline solver with its
cost counted (`la_fallbacks`). M, N and S share every relation (the solver
runs after collection), so their relation work and group operations are
identical up to the verification multiplications.

## 3. Instances and seeds (test)

As the earlier two studies: primes the first prime ≥ `2^7 … 2^16`;
`curve_seed0 = 1`, `max_h = 64`; curves per prime 5 (≤ 2053), 3 (4099, 8209),
2 (16411, 32771), 1 (65537); 4 seeds per curve and arm.

## 4. Training sweep for cc* (before the test run, on other curves)

- Curves `curve_seed0 = 5001` (disjoint from the test curves), primes 4099,
  8209 and 16411, 2 curves × 2 seeds, arm S, `cc ∈ {0.7, 1.0, 1.4, 2.0}`.
- Score: geometric mean over (prime, curve, seed) of S's `total_w01`
  (w = 0.1 GAE per native multiplication), each divided by the same
  instance's R `total_w01`.
- cc* = the grid value with the lowest score; ties to the larger cc. cc* is
  written into `TRAINING.md` and committed before the test run starts.

## 5. Gates

- G1: M's rows reproduce the earlier study's M rows (`C_lucas_roots`) in
  `group_ops`, `rel_mul`, `la_mul`, `trials`, `relations`.
- G2: N and S have M's `trials` and `relations` on every (p, curve, seed).
- G3: every log certified in every arm.
- Unit tests `coordinate_wiedemann_returns_the_target_coordinate` and
  `la_modes_certify_and_cut_la_work` pass at the frozen commit.

## 6. Metrics

- Component: LA native multiplications, N/M and S/M (paired on the same
  relations); fallback count.
- End to end: arm total ÷ R total at w = 0, 0.1, 1 with the two-stage
  bootstrap (curves, then seeds; 2000 resamples; 95%).
- Native work including additions (`native_all`), reported beside every
  multiplication figure, since additions are priced at 0 in the band.

## 7. Predictions (before data)

- P1: S/M LA multiplications in [0.25, 0.45] at every p ≥ 4099; N/M in
  [0.55, 0.80].
- P2: fallbacks in ≤ 5% of S and N runs.
- P3: at p = 32771, S/R at w = 0.1 in [1.8, 3.0] (M was 3.69).
- P4: cc* < 2.
- P5: §5a verdict still unresolved for every arm (no arm below R at all
  three prices).

## 8. Decision rule

- The solver change advances the component if G1–G3 hold and S/M LA
  multiplications ≤ 0.6 at every p ≥ 4099.
- S* is reported as the new end-to-end operating point only if its training
  score beats S's (cc = 2) score; otherwise S is.
- Distance to rho is reported per price as the distance remaining.

## 9. Deviations from the lab defaults (as the earlier study, PROTOCOL §8)

Local run (no AWS identity in this container, so not isolab-eks); counts are
deterministic in the seed. S3 checkpoint and raw-data upload unavailable. No
Conductor client.
