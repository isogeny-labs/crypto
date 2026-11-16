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

## Addendum A: instrument and pre-data amendments (pinned before any claim cell)

**Instrument.** `p1tool/` (standalone Rust, `src/main.rs` sha256
`0d9fa76e0381167b8d3c6d4ec8033a8345204db9f7ad99190ff5828171954edd`; development
binary sha256 `4ba83d9a5ab271ddbec38ea0e263d97457dab73997cfcc80f6abb33b98f8250f`,
rustc 1.97.0), msolve 0.6.5 (`/usr/bin/msolve` sha256 `c2722288…f759fa`), thin
runner `calibration/msolve_cell.sh` (`ulimit -v`, `timeout`, `nice -n 10`). Details,
equations and variable counts: `p1tool/README.md`.

**Positive controls, done during the build (instrument validation)
[pcc:unverified]:**
- A, emitter fidelity: on round 7's instances, 12 of 12 systems emitted by
  `p1tool` are byte-identical (`cmp`) to those of the unmodified `genrr.sing` and
  `gen.sing`; the Singular regeneration matches all 5 hashes of
  `round7_rr/SYSTEMS.sha256`.
- B, measurement: RR-Kummer m3 t4 gives D = 4, widest 2,989 × 2,801, qdim 1;
  T-Kummer m3 t4 (k0, k1) gives D = 6, widest 9,037 × 7,077, qdim 3; equal to
  `round7_rr/rows.tsv` and `../rows.tsv`. Rule 1's control condition is therefore
  met before any claim cell.

Commands, per cell:

```
p1tool emit --system rr|t --kind dickson|kummer --p P --a A --b B --m M --t T --seed S --out C.ms
VLIMIT=1572864 TLIMIT=1200 calibration/msolve_cell.sh C.ms C.log      # one job at a time
p1tool parse C.log
```

**Pre-data amendments (from the build, before any claim cell):**

1. **Curves fixed.** T24: a = 3,607,823, b = 6,826,282, n = 14,683,681 (BSGS; a
   naive count agrees). T31: a = 289,962,109, b = 1,299,273,082,
   n = 2,046,768,077 (BSGS and Hasse uniqueness). Both from `p1tool curve --seed 1`.
2. **Kummer comparison arm on a prime-order curve.** Round 7's p0 curve has order
   786,627, which is not prime. It stays as the positive control (B above). The
   RR-K and T-K comparison arms of rule 3 use the prime-order curve over p0
   a = 658,649, b = 132,295, n = 785,623, at the same cells as the Dickson arms.
3. **Seeds and a skip rule.** A cell needs a planted instance whose base holds m
   distinct admissible abscissae with exactly one sign pattern summing to R; at
   t = 3 the base has at most 8 abscissae and many seeds have none. Seeds are the
   integers S = 1, 2, …, 50 in order (not hash-derived); each cell takes the first
   two admissible seeds, every skipped seed is recorded with its reason, and a cell
   with no admissible seed in 1..50 is "not admissible" (never imputed). Build-time
   emit-only checks: m3 t4 and m3 t5 are admissible at seeds 1 and 2 on both
   primes; m3 t3 seed 1 is not; m5 t3 at T24 is admissible only at seed 2; at T31
   at neither seed 1 nor 2.
4. **The T arm is round 7's cherry-tree chain system with the membership tower,
   run through msolve.** The proposal named `pkm_tower_pilot --kinds dickson-torus`,
   which is a different system (the caterpillar S₃ chain under its own F4 engine).
5. **Variable counts corrected.** At m = 3: N_RR = 5t − 2 (18 at t = 4) and
   N_T = 3t + 1 (13 at t = 4); the proposal's N = (2m − 1)t + 2m (26) is wrong.
   This changes no decision rule.
6. **Disclosure: the predictions are not blind.** The development smoke runs at
   tiny Dickson primes p ∈ {1151, 1279, 1999} (finished 21:35Z, two minutes after
   the protocol commit at 21:33Z, and read before this addendum) gave
   D_RR-D = 4, widest 2,801, and D_T-D = 6, widest 7,129, at m3 t4. These match the
   proposal's predictions, which were written before them. The claim cells at T24
   and T31 decide; the smoke values decide nothing.
7. **Quotient dimension** may exceed 1 when an instance has a second decomposition
   (2 of the 11 smoke RR cells, qdim 2); rule 1 requires only qdim ≥ 1.
