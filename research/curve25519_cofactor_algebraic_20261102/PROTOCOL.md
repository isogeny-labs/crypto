# Cofactor-aware algebraic oracle (`pkm-semaev:lifts=1`) — protocol

Frozen 2026-11-02, before any data in this directory.

## Problem

`pkm-semaev` solves `S₃(x_i, x_j, x(R)) = 0` for the target abscissa exactly,
so it finds only `R = ±P_i ± P_j` in `E(F_p) ≅ Z/8 × Z/ℓ` (Curve25519
miniatures, `p ≡ 5 mod 8`). The base points are physical (not
cofactor-projected), so pair sums spread over the 8 cosets of the order-ℓ
subgroup and only the pairs in R's own coset count. Measured hit rates are
`~1/3 – 1/5` of the exact projected yield λ of `Q-A`
(`research/curve25519_algebraic_ladder_20261102/analysis/support.jsonl`).

## The lift contract (read from the code, then followed)

`collect_and_solve_relations_scaled_touching`
(`src/cryptanalysis/ic_boundary.rs`) turns an oracle's `Relation::Summands(idx)`
into the row `Σ_i coef_of[i]·x_{col_of[i]} − h·b·d = h·a (mod r)`, with the
target `R = [a]G + [b]Q`. The oracle's returned indices are **not** compared
with `R` by the pipeline; the row is the projection `[h]Σ P_i = [h]R`, which
holds for any `Σ P_i = R + K` with `K ∈ E[h]` because `[h]K = O`. The fiber
oracles (`ic_framework/fiber.rs`, mode `lifts`) therefore (i) compute
`E(F_q)[h]` in `prepare` from `[r]P` over base points
(`cofactor_fiber`), charged to set-up, (ii) per target add each `K`
(charged), (iii) run the inner decomposition against the lift `R + K` and
check the recovered points sum to the lift **exactly** (`lift_abscissae`),
(iv) return the first hit's indices unchanged.
`pkm-semaev:lifts=1` does the same: lifts `R + K` with an abscissa not
yet tried (x(R+K) = x(−R−K); `K = O` first), one F4 system per lift, the first
liftable relation wins; every F4 call and every lift addition is charged.
With `h = 1` (fiber of size 1) the oracle is the unchanged blind oracle.

## Verification of the contract (unit test, `cargo test --release --lib pkm_semaev`)

`cofactor_lifts_match_the_brute_force_fiber`, on a 10007-field curve with
`#E = h·r`, `h ≥ 4`, `E[h]` rational: (a) the verdict equals the brute-force
set "R + K decomposes for some `K ∈` E[h] (all points killed by h, found by
enumeration, independent of `cofactor_fiber`)"; (b) every returned relation
has `Σ − R ∈ E[h]`, `[h]Σ = [h]R`, and satisfies the pipeline row
`Σ coef·x = h·a (mod r)` with the column unknowns `x_c = log_G [h]P_c`
computed by discrete-log table; (c) the same with `guess=1`; (d) `h = 1`
gives identical relations and identical F4 call counts with and without
`lifts=1`.

## Measurement

Instrument: `curve25519_family f4cost <k> <form> <t> <guess> <targets> <seed>
<cap_s> 2 full <lifts>` (`lifts` moved to position 11 when the m/mode arguments of the m = 3 harness landed first; edit made before any data), the oracle called directly on random subgroup targets
`[a]G`. k = 20 miniature. Cells (each run twice, `lifts=0` and `lifts=1`,
**same seed**, hence the same 200 targets):

| form | t | guess | seed |
|:--|:-:|:-:|:-:|
| isogeny | 4 | 0 | 2026111204 |
| isogeny | 5 | 0 | 2026111205 |
| isogeny | 6 | 0 | 2026111206 |
| bits | 5 | 3 | 2026111205 |
| bits | 6 | 3 | 2026111206 |

200 targets, per-cell cap 600 s (a capped cell is reported with its
`targets_done`, not dropped). Single process, runs interleaved by pair
(lifts=0 then lifts=1) on one host; the measured quantities are operation
counts (F4 multiply-adds, calls, hits), which do not depend on host load.
A "hit" is a relation the oracle returned. With lifts, a hit is verified by
the pipeline's rule (`[8]Σ = [8]R`); `hits_exact` counts those with `Σ = R`.

Reported per cell and arm: targets done, hit rate (with Poisson 95% interval),
F4 calls per target, F4 multiply-adds per target, multiply-adds per hit
(per relation), and the **ratio of multiply-adds per relation, lifts /
no-lifts**. Fiber set-up group operations are reported separately (one-off,
not per target). F4 multiply-adds are native, unpriced work (AGENTS.md §5a);
this study compares two arms of one oracle in the same unit and makes no
cross-method claim, no rho comparison and no leaderboard row.

## Preregistered predictions

- **L1.** Hit rate rises by a factor of 3–5 (never more than 8, the number of
  lifts), because only about 1 pair sum in 8 lies in R's coset and a lift
  moves the target to another coset; the measured blind rate is already
  1/3–1/5 of λ, so the lifted rate should approach λ (the exact projected
  yield).
- **L2.** F4 calls per target rise by at most 4× (a refuted target costs up to
  8 calls; a hit stops early).
- **L3.** Multiply-adds per relation therefore fall by a factor of about
  `(hit-rate gain) / (call gain)` — i.e. lifts / no-lifts ratio below 1 (a
  win) if the hit-rate gain exceeds the call gain, above 1 otherwise. No
  prediction is made that it is below 1.

## Statistical power (stated before data)

At k = 20, λ for these cells is about 0.1 % – 1 % (support.jsonl), so 200
targets give 0 – a few hits per cell without lifts. The hit-rate ratio is
therefore reported with its Poisson interval and is weak evidence; the
multiply-adds per target and per call are exact counts and carry the cost
side. If a cell has zero hits in an arm, the ratio per relation is reported
as undefined, not imputed. If time allows, a supplementary run with more
targets (same code, labelled as such, own seeds) is added without changing
the preregistered cells.

## Classification (decided before data)

- **Improvement** only if the lifts / no-lifts multiply-adds-per-relation
  ratio is below 1 in the cells with at least 3 hits in both arms, and every
  lifted relation verifies. Otherwise a **yield gain at a cost** (hit rate up,
  per-relation cost not down) or **no gain**. The gap to matched rho is not
  measured here (no rho arm); it is unchanged from the ladder study.
