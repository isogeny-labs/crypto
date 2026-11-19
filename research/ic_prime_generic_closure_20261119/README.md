# Generic prime-field index calculus: what is closed, and why

2026-11-19. A closure note, not a new measurement. It joins this session's studies and
the repository's earlier ones into one argument, so effort is not re-spent on routes that
the evidence already rules out. Scope (owner rules): generic prime curves, one target
charged cold, no multi-target methods, no weak families. Every figure is
**[pcc:unverified]** with the standing of its source; no leaderboard row is claimed.

## Correction (same day): the pair table is not the incumbent

The repository's best generic prime-field rows are the k-sum family, not the m = 2 pair
table modelled below: `ic_ladder_wwalk_ksum_20261115` 1.8–1.9× matched rho at 24–48 bits;
`ic_ksum_sibling_20261118` 4.6–5.3×; `ic_ksum_prune_20261115` 5.0–5.7× (all [pcc:unverified],
w = 0). Those studies already label the track "generic decomposition, bounded by √r;
cannot beat rho or BSGS". `../ic_prime_pair_constants_20261119` reproduced the ordering:
k-sum m = 3–5 at small |F| is 6.6–9.8× rho on the size-model curves against 12.7–20.1× for
the best m = 2 arm. So the section below explains why the m = 2 pair table is far from rho;
the k-sum rows are closer but bounded by the same √r birthday argument (last paragraph of
the next section).

## The cost of the m = 2 pair-table row, measured

`prime-abscissa` base, `mitm-prime-blocked:m=2`, `walk` targets, target-pinned
`incremental-gauss` ([`../ic_fb_size_model_20261119`](../ic_fb_size_model_20261119/README.md),
336 records verified, predictions M1–M6 held):

    GAE ≈ 0.98·|F|²  (pair table)  +  1.2·N/(4|F|)  (walk trials)

within 11 % at six sizes on two curves. The optimum is |F| ≈ N^{1/3}/2 and the best cost
grows as N^{2/3}, so S grows as N^{1/6}. Distance to `rho.negation` at the optimum:
14.6× (24 bits), 24.4× (28 bits) at w = 0, and it widens with the curve.

## Why no device inside this structure reaches rho

| route | finding | source |
|:--|:--|:--|
| choose \|F\| | optimum S ∝ N^{1/6}; measured ratio 1.57 from 24 to 28 bits (model 1.59) | `ic_fb_size_model_20261119` |
| prune the pair table to T sums | cost ≈ T + 0.28·\|F\|·N/T ≥ 1.05·√(\|F\|·N) > rho for every \|F\| ≥ 1 (derivation from the fitted constants) | same, README |
| relation solving (sketches, structural-rank gate, mixed peel, nested dissection, sort-merge) | no counted change; the target-pinned solver already reads the minimum rows (Dulmage–Mendelsohn: zero slack on 128/128 streams) | `ic_sketch_mixed_solver_20261115` |
| filters on the decomposition test | the test is native work only; it moves S only at w > 0 and shifts the optimum up one size | `ic_fb_size_model_20261119` (M6) |
| residual / large-prime collector (res2, res4) | an r-adding walk that stores every point; each relation is a birthday collision; exponent matches rho (0.46–0.48) and its cost equals rho's on a structureless carrier. Driving its overheads out turns it into rho: relabelling, not an IC gain | `ic_prime_residual_lp_20261026`, `ic_planner_exhaustive_20261026`, `src/cryptanalysis/ic_framework/residual.rs` |
| extension fields E(F_{p³}) (Gaudry, double large primes) | 605–2,501× rho at 27–39-bit groups; fitted crossover 2^171 (2^156–2^188), 2^245 with theory exponents, none if linear algebra keeps its fitted p^{1.52} | `ic_ext_field_crossover_20261119` |

The common reason: with m-term decompositions (k-sum included) over a factor base of points with small
abscissa, a relation is a meet-in-the-middle event between a stored set and a walk. Its
best case is a birthday bound, which is rho's; the factor-base bookkeeping only adds
constant and exponent overhead on top.

## What remains open (and is small)

- Constants of the m = 2 pair-table row: measured in `../ic_prime_pair_constants_20261119`.
  No existing m = 2 variant lowers the table or trial constant at the optimal |F| (every
  arm ties or is dearer; deferred-guard is cheaper only at w = 1).
- A structurally different decomposition with a cheaper-than-|F| membership test per trial
  for a base of size near √N. None is known for generic prime curves (the
  Petit–Kosters–Messeng family is in `src/cryptanalysis/ic_framework/pkm_semaev.rs`).

## Not covered here

Binary and Koblitz curves, where decomposition is a polynomial system and the solver is
the cost centre (`../ic_binary_unsat_cost_20261119`, `../ic_binary_generic_incumbent_20261119`).
Weak families and multi-target methods are out of scope by owner rule.
