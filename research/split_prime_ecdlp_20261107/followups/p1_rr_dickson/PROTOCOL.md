# P1: Riemann–Roch norm-divisibility system with a Dickson-torus membership tower (protocol, frozen 2026-11-07)

Proposal `IDEA-20261107-RR-DICKSON-TORUS`
(`../../ideas/idea-generator.md` lines 41–135). Class: **stage diagnostic,
algebraic track** (AGENTS.md "Tracks of index calculus"). No pipeline `S`.

## Question

round7_rr measured that the separator-free Riemann–Roch system (RR) has a lower
F4 solving degree than the cherry-tree chain system (T) at m = 3, t = 4 with a
**Kummer** membership tower (D_RR = 4 vs D_T = 6, widest matrix 2,801 vs 7,077;
`research/ic_tree_gluing_20261103/round7_rr/README.md` [pcc:unverified]).
NIST-shape primes (p ≡ 3 mod 4, 2^t | p + 1) have no Kummer tower, only the
**Dickson torus** tower. Does the RR advantage transfer to the Dickson tower?

## Arms and instances

| arm | system | tower | prime |
|---|---|---|---|
| RR-D | RR | Dickson, `U | D_{2^t}(X) − c` | T24 = 14,680,063 = 7·2^21 − 1; T31 = 2,046,820,351 = 61·2^25 − 1 |
| T-D | T (cherry tree) | Dickson | T24, T31 |
| RR-K | RR | Kummer, `U | X^{2^t} − c` | p0 = 786,433 = 3·2^18 + 1 (round7_rr's prime and instance) |
| T-K | T | Kummer | p0 |

- Curves: one prime-order curve y² = x³ + ax + b per Dickson prime, the first
  one found from seed `P1/curve/<p>` (j ∉ {0, 1728}, exact order checked); the
  Kummer arms use round7_rr's own instance so they double as the positive control.
- Targets: 2 planted targets per cell (seeds `P1/target/<p>/<m>/<t>/<i>`), each a
  sum of m base points, so a solution exists; the generator checks that the planted
  solution zeroes every equation before writing the system.
- Cells: m = 3 at t = 3, 4, 5 on T24 and T31 (both systems); m = 5 at t = 3 on T24
  (both systems); Kummer m = 3 at t = 3, 4, 5 on p0 (both systems).
- Engine and limits (round7_rr's): `msolve -g 2 -t 1 -v 2`, msolve 0.6.5,
  `ulimit -v 1572864` (1.5 GiB), 1,200 s per job, one job at a time.
  A job that dies at the memory cap is "cap"; one past 1,200 s is "timeout";
  both are recorded with the highest degree reached and never imputed.

## Metrics (per job, exact)

D (F4 solving degree: the highest degree of a matrix F4 reduced), widest matrix
(columns, and rows × columns), quotient dimension (must be ≥ 1: the planted
relation), variable count N and input degrees, status. Wall seconds descriptive.
Per cell the decided value is the maximum over its 2 targets (D is expected to be
target-independent; a disagreement between targets is reported).

## Predictions (from the proposal, frozen)

D_RR-D(m3, t4) = 4; D_T-D(m3, t4) = 6; D_RR-D(m3, t5) = 5; |D_RR-D − D_RR-K| ≤ 1 at
every shared (m, t) (proposer's probability 0.75).

## Decision rules (frozen; first applicable outcome per line)

1. **Control failure (P1 moot):** RR-K does not reproduce round7_rr's D at m3 t4
   within ±1, or any arm's quotient dimension is 0 (planted relation lost).
2. **Killed:** D_RR-D ≥ D_T-D in every comparable completed Dickson cell.
3. **Transfer fails:** |D_RR-D − D_RR-K| ≥ 2 in any completed shared (m, t).
4. **Trade, not a gain:** D_RR-D < D_T-D somewhere, but the RR widest matrix
   is larger in every such cell.
5. **Stage-level positive at m = 3:** D_RR-D < D_T-D and widest RR < widest T in at
   least one m = 3 Dickson cell, on both primes.
6. **Repository criterion met (algebraic track, stage level):** rule 5 holds
   and the m = 5, t = 3 cell also has D and widest matrix smaller for RR-D than
   for T-D. This would be a stage-level result only; it moves no pipeline `S` and
   needs the validation gate before any leaderboard mention.
If neither m = 5 arm completes within the limits, rule 6 is "undecided", never
"failed".

## Controls

- Positive: RR-K and T-K reproduce round7_rr; emitter fidelity against round7's
  Singular generators (instrument validation, Addendum A).
- Negative / nuisance: two targets per cell; two Dickson primes; curve seed recorded.
- Torus condition checked for every base element (Dickson arms).

## Cost accounting

Stage diagnostic: only D and widest matrix are compared, arm against arm, on the
same instance. No end-to-end cost, no rho comparison (AGENTS.md "Research progress
and communication").

## Sources checked

KB unavailable. `research/ic_tree_gluing_20261103/round7_rr/README.md`,
`research/ic_tree_gluing_20261103/STATUS.md`, `research/pkm_dickson_torus_20261015/`,
`research/prime_one_shot_20261029/`, `problems/tree-glued-pdp-separator-coupling.md`,
`../../ideas/idea-generator.md`. Lean: `dickson_tower_faithful` and
`kummer_tower_faithful` in `aburan28/crypto-lean` PR #3 (the towers impose exactly
the membership the systems claim).

## Addendum A: instrument (pinned before any claim cell)

Pending the builder's report.
