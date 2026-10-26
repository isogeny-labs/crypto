# PROTOCOL: cell G3, the C3 census (H-BINSTD-c72284 Stage 2)

Status: **prepared, not run, not frozen.** The freeze happens when the director
records the output of `g3census manifest` and the sha256 of every file in
`src/` next to this file (RUN_CARD step 1) before the first round-1 row. Until
then nothing here is a preregistration. Deviations from the spec of
EXP-BINSTD-372679 are listed in [AMENDMENT.md](AMENDMENT.md); read it first.

Spec: `crypto-autoresearcher/ledger/hypotheses/H-BINSTD-c72284.yaml` and
`experiments/EXP-BINSTD-372679/specification.yaml`. Audit that reopened it:
`/home/user/crypto-work/frobenius_audit_20261026.json`, claim C3, verdict
"unsupported".

## 1. Question and claim classes

Question. On a linear sigma-stable factor base V = ker g(sigma) of F_{2^n}
(Koblitz toys, b = 1), does any *sound* use of the Frobenius orbit structure
lower the per-target cost of one complete decomposition search below the plain
single-instance search, on the same solver?

| # | sentence this study may publish | class | established by |
|:--|:--|:--|:--|
| S1 | ord_n(2) = 8, 11, 5 for n = 17, 23, 31; stable dimensions at 131 and 163 are {0,1,130,131} and {0,1,162,163} | exact computation | `gf.rs` tests (already pass) |
| S2 | The gauge-fixed orbit system has |V^m / S_m| points up to the stated tie sliver; arm A is a bijective reparameterisation of V | theorem plus exact count | `counting_identity_of_the_three_domains_n7` (domain counts equal an independent Rust predicate) |
| S3 | Arms A, B, G, C, Cpoly return the exact oracle's verdict, and every SAT model is a valid decomposition | exact computation | oracle comparison on every target of every run (O-ARTIFACT otherwise) |
| S4 | The literal per-instance orbit clauses (arm U) change satisfiability on planted targets | exact computation | arm U vs oracle on planted SAT targets |
| S5 | On the named solver, in the named cells, the median paired conflict ratio A/C and B/C is (or is not) below 0.95 | **comparative** | this protocol, sections 4 to 8 |
| S6 | The excess conflicts of A and B over C grow with n or with l (additive vs multiplicative overhead) | comparative, low resolution | section 6 diagnostic |
| S7 | Anything about other solvers, other encodings, n >= 41, K-163, ECC2K-130, cost per relation of the whole pipeline, orbit unions at m >= 3 | **out of scope: not established** | |

No sentence may state a speedup, an exponent move, a K-163 or ECC2K-130
verdict, or a lane closure. A "relabelling" outcome is recorded as a
relabelling (AGENTS.md section 6), never as an improvement.

## 2. Instrument (identical for every arm)

* Solver: the repository's own native CDCL with native XOR rows and
  Gauss-Jordan propagation, `src/cryptanalysis/sat.rs`, compiled into the
  census binary by path (`#[path]`), so the sources are bit-identical to the
  library's. Its sha256 is recorded at the freeze. WDSat and CaDiCaL (the
  solvers the spec names) are C/C++ and are not usable under the
  native-Rust-only rule; WDSat's source exists only inside a sibling run
  directory. This is the principal scope limit (AMENDMENT A1).
* Encodings: `src/enc.rs`. Equations are the chained Semaev summation
  polynomial S3(x1,x2,x3) = (x1 x2 + x3 (x1 + x2))^2 + x1 x2 x3 + b (identical
  to `binary_semaev_s3`, checked against the group law by
  `group_law_and_s3_identity_exact`). m = 2 is S3(x1,x2,xR) = 0. m = 3 is the
  chain S3(x1,x2,X) = S3(X,x3,xR) = 0 with a free n-bit X. The system is native
  XOR rows plus AND-definition clauses, as in the repository's
  `semaev_sat` family. Field elements are symbolic sums of linear forms
  times constants, so squaring is free and equations are n XOR rows.
* Budgets, applied identically: conflict cap 20 000 000 (m = 2) and
  40 000 000 (m = 3), deadline 900 s (m = 2) and 14 400 s (m = 3) per solve.
  A solve that hits either is `UNKNOWN`, censored, and never evidence.
* Metric source: solver counters `conflicts`, `decisions`, `propagations`,
  `word_ops()` (bitop/v1), plus wall clock and VmHWM beside every row.
* Seeds: every (cell, target, arm, seed-slot) run uses a variable renaming and
  initial saved phases derived from `MASTER_SEED = 2026102703` by splitmix
  hashing; the derivation is in `run.rs::run_one` and the seed is printed in
  every row. Seed slots 1 and 2 are the primary pair, 3 and 4 the replicate.

### Arms

| arm | what it is | role |
|:--|:--|:--|
| C | plain single instance, normal-basis V coordinates, S_m lex break on the u-bits, priority on the u-bits | reference (c) |
| Cpoly | same, V coordinates and XOR rows in the polynomial basis | basis control (spec: "(c) in both bases") |
| A | x_i = sigma^{s_i}(rho_i), rho_i the necklace-minimal representative of its sigma-orbit in V (n-1 lex comparators per slot), s_i one-hot over Z/n, s_i = 0 forced when rho_i is sigma-fixed, S_m lex break on rho; priority on r and h | spec arm (a) |
| B | target t = sum_j h_j sigma^j(xR) with h one-hot, tuple gauge-fixed by "x_1 is the lex-minimal rotation of every slot" (m n - 1 comparators) and x_2 <= x_3; priority on u and h | spec arm (b), non-degenerate gauge |
| G | C plus the comparator circuits of B on the same coordinate bits, each made vacuous by a free selector | **generic size-matched symmetry-breaking control** (sound, no pruning) |
| U | C plus the comparators of B enforced against the fixed target | literal per-instance orbit clauses, **unsound by design**, tripwire |
| Cnobreak | C without the S_m break | positive control P1, expected C/Cnobreak ~ 1/m! |
| Bnogauge | B without the gauge | positive control P2, expected B/Bnogauge ~ 1/n |
| C@k | arm C with seed slot k | A/A noise floor (C@2 vs C@1) and replicates |

Arm names with `@k` select the seed slot; the encoding is the same.

The gauge for B is on the tuple, not on j. Gauging j = 0 makes B literally
equal to C (tested: `j0_embedding_equals_plain`) and its ratio identically 1,
which would make the spec's "excess (b) - (c) grows with n" vacuous
(AMENDMENT A3).

## 3. Populations

Cells (n, m, l), each with the lexicographically first V = ker g(sigma) of that
dimension (`vsel` 0): (17,2,8), (17,2,9), (23,2,11), (23,2,12), (31,2,15),
(31,2,16), (31,3,10), (31,3,11). All satisfy m (l-1) <= n. Added: the m = 3
ladder (31,3,5) and (31,3,6), the V-sensitivity cells `vsel` 1 at n = 17 and
23, and the ordinary-curve null (below).

Targets, per cell:

* **UNSAT targets (primary):** 50 for m = 2, split 25 on E_{a=0} and 25 on
  E_{a=1} (b = 1; S3 does not depend on a, so the two curves are two disjoint
  abscissa pools of one instance family, a blocking factor and not two
  structures). m = 3: see section 8. A target is a random abscissa, accepted
  if its sigma-orbit has not been used (conjugate targets are the same problem
  and would be pseudo-replicates) and the exact oracle says UNSAT.
* **Incidental SAT targets:** up to 5 per curve per cell, kept when met while
  sampling. Used for per-relation cost and soundness.
* **Planted SAT targets:** 10 per curve per cell (spec: 20 per cell):
  x_i in V with random lifts to E, xR = x(sum). The oracle must confirm SAT.
* **Labels come from the oracle, not from a solver:** m = 2 enumerates x1 in V
  and solves the quadratic S3 = 0 for x2 (`Oracle::count_m2_ordered`); m = 3
  builds the set D2 of X roots over V^2 once per cell and tests each x3.
  Verified exhaustively against brute force at n = 7
  (`oracle_matches_bruteforce_n7`).

Ordinary-curve null (n = 17, l = 8, m = 2, `--b 2`, b = t not in F_2): here the
Frobenius equivariance fails, so arm B must disagree with the plain truth
somewhere, while C and A (a bijection) must not. The cell is a **detector
check for unsoundness**, not a cost comparison (AMENDMENT A4); the summary
reports "detector works" only if B disagrees and C, A never do.

## 4. Controls

| control | arm(s) | expectation written before data | how a miss is read |
|:--|:--|:--|:--|
| negative / noise floor (A/A) | C@2 vs C@1, paired per target | median ratio within the bootstrap noise of 1; sd of the log ratio s_AA sets the minimum detectable effect | an interval for a candidate effect that does not clear the A/A interval is noise |
| negative / size control | G vs C | median ratio >= 0.95 (vacuous circuits cannot prune); G tests whether adding n(n-1) aux variables and clauses of this shape by itself moves conflicts | if G reproduces a sub-0.95 A or B ratio, that ratio is not symmetry |
| basis control | Cpoly vs C | reported with interval, no direction predicted | if Cpoly/C < 0.95 the reference basis is not neutral and a candidate must also beat Cpoly |
| positive P1 | C vs Cnobreak | C/Cnobreak ~ 1/m! (0.5 at m = 2): detected iff the 95% upper bound is < 0.95 | miss: cell void (O-ARTIFACT), detector blind |
| positive P2 | B vs Bnogauge (n = 17 and 23 only) | ~ 1/n | same |
| soundness tripwire | U on planted SAT targets | at least one planted target per cell turns UNSAT (spec: H1 literal clauses change satisfiability) | none flipping anywhere contradicts S4 and is the headline |
| ordinary-curve unsoundness detector | B vs oracle on b = t | B disagrees somewhere | detector failed: B0 untrustworthy |
| same instrument | all | one binary, one solver source hash, one budget, one seed rule, arms interleaved per target (order shuffled per target from the master seed), arm labels in the rows | |

Positive controls run on the same code path (same builder, loader, solver,
counters). P1's effect (m!) is far above the 0.95 threshold; the original H1
claimed n^{-m}, and P2 is a real factor-n quotient on the same circuitry. They
show the detector sees large symmetry gains. They do not show it sees a 5%
one: that is the MDE of section 5.

## 5. Metrics, resolution and power

Primary (spec): conflicts per UNSAT target to a complete UNSAT answer. The
unit is the (target, seed-slot); the inference unit is the target (cluster
bootstrap resamples targets with their slots, 10 000 resamples, percentile 95%
interval of the median ratio).

Co-primary (AGENTS.md section 5a): solver `word_ops` at encoder-work weights
w = 0, 0.1, 1 (`sops`, `ops01`, `ops`); an arm is ranked against C only if the
median ratio is on one side of 1 at all three weights, otherwise "unresolved
inside the band". Unpriced native work listed for every arm: encoder literals
written (counted, `encode_ops`), the table sigma^j(xR) for B (n squarings, not
counted, bounded by n^3 bit operations, about 3e4 at n = 31, negligible against
the search), the V basis reduction (once per cell, not counted). Wall time and
peak RSS are reported as practicality notes only.

Secondary: per-relation cost = total cost over all targets of a kind divided by
SAT targets found. By S3 all sound arms find the same SAT targets, so the
per-relation ratio equals the per-target ratio of the totals; it cannot show a
yield difference. The n-fold conjugate yield (claim C4) is an equivariance
fact, not measured here.

Resolution. For N paired targets with sd(log ratio) = s, the minimum
detectable median ratio at 80% power and two-sided 5% is
`exp(-2.8 * 1.2533 * s / sqrt(N))`.

| s \ N | 10 | 20 | 25 | 50 |
|:--|--:|--:|--:|--:|
| 0.05 | 0.946 | 0.962 | 0.966 | 0.975 |
| 0.10 | 0.895 | 0.925 | 0.932 | 0.952 |
| 0.20 | 0.801 | 0.855 | 0.869 | 0.906 |
| 0.30 | 0.717 | 0.790 | 0.810 | 0.862 |

N needed to resolve 0.95 itself: 12 at s = 0.05, 47 at s = 0.10, 106 at 0.15,
188 at 0.20. **At N = 50 the 0.95 threshold is at the edge of resolution
unless the A/A sd is at most about 0.10.** s is unknown until the pilot. If the
pilot A/A gives s > 0.10 for a cell, the cell cannot establish "ratio >= 0.95"
(its interval will straddle), and the rule returns O-INCONCLUSIVE, not
O-RELABEL; the only claims such a cell supports are the large ones (A or B much
cheaper, or much dearer). Using seed slots 1+2 (and 3+4) per arm reduces the
within-target noise by about sqrt(2). This is a resolution statement about the
design, not a prediction of the outcome.

Multiplicity. Sixteen cell-arm tests are read. The "structure" label needs
the lower tail to clear 0.95 in the primary slots and again in the independent
replicate slots, with G and Cpoly not reproducing it; the chance of a false
structure label from noise alone is then about 16 x 0.025^2 = 1e-2. The
"relabel" label is an intersection claim over cells and needs no correction.

Cheapest discriminator: n = 17, l = 8, arms C, C@2, A, B, G, Cnobreak on 50
UNSAT targets (seconds of CPU).

Stopping rule: fixed N per cell as in section 3; no optional stopping; a cell
that cannot finish within the budget is cut by the section 8 gates (decided
from pilot rows only, before any round-1 row of that cell) and recorded as
such.

## 6. Overhead diagnostic (S6)

Per cell, mean conflicts of C and the excess of A and B over C. For two cells
with the same n and m and adjacent l, C grows by r_C = C(l+1)/C(l) (about 2 at
m = 2 and 4 at m = 3 if conflicts scale as 2^{(m-1) l}, the section 8 model; the
measured r_C is used) and the excess by r_E. "Additive-consistent" is r_E < sqrt(r_C); otherwise
"multiplicative-consistent". With two l values per n this is a two-point
comparison and the dependence on n has three points (17, 23, 31): it can
distinguish an excess that is flat in l from one that tracks C, and cannot fit a
functional form in n. State it that way.

## 7. Decision rule (frozen at the freeze; implemented in `stats.rs::decide`)

Per cell, evaluated on UNSAT-truth targets with both arms UNSAT:

1. **O-ARTIFACT** if any sound arm (C, Cpoly, Cnobreak, A, B, Bnogauge, G)
   disagrees with the oracle, or any SAT certificate fails, or P1 (or P2 where
   run) is not detected (upper bound >= 0.95). The cell is void.
2. **O-IMPEDIMENT** if any arm-slot has more than 10% censored UNSAT targets,
   or fewer than 10 paired targets. Never evidence.
3. For X in {A, B}: **candidate** if the 95% interval upper bound of the median
   conflict ratio X/C (primary slots) is below 0.95. A candidate is
   **confirmed** (O-STRUCTURE) only if all of: the replicate slots also have
   upper bound below 0.95; G/C upper bound is at least 0.95; Cpoly/C upper
   bound is at least 0.95; the A/A lower bound exceeds the candidate's upper
   bound. A candidate that is not confirmed gives O-INCONCLUSIVE with the
   failed conditions listed.
4. **O-RELABEL** if no candidate and both lower bounds are at least 0.95.
5. Otherwise **O-INCONCLUSIVE** (an interval straddles 0.95).

Cross-cell reading: the spec's O-TRIPWIRE headline applies if arm U flips zero
planted SAT targets in every cell. O-RELABEL in all cells is "consistent with
the relabelling null in these cells on this solver"; it is not a proof and not
a statement about other solvers or encodings. An O-STRUCTURE cell is a
heuristic-class lead needing independent replication on a second build before
any wording stronger than "observed once".

The analysis is the frozen script `summarize`; arm labels are not masked
because the rule is mechanical and fixed before data (stated deviation from the
skill checklist).

## 8. Sizes, runtime and the gates

Structural sizes, from the build-only test `all_census_cells_build_all_arms`
(no solve, deterministic): at (31, 2, 16) arm C has 304 variables and 31 XOR
rows; A 3 235 and 93; B 3 684 and 108; G 2 225 and 61. At (31, 3, 10): C 1 802
and 103; A 5 660 and 196; B 6 652 and 228. At (17, 2, 8): C 88 and 17; A 949 and
51; B 1 094 and 60.

Conflict scaling used for the estimate (a **code-reasoning estimate with
unmeasured constants**): the XOR engine turns the problem, once the first
summand is fixed, into an affine system in the second, so a complete UNSAT
search visits about kappa x 2^{(m-1) l} nodes, not 2^{m l}. Anchor: the
sibling record EV-FROB-b6e1e9 / union_n23.json shows WDSat with XOR-Gauss at
660 conflicts for a 2^7 first summand (kappa about 5) and 93 858 conflicts for
a 14-bit system without XOR (5.7 x 2^14). kappa in [1, 20] is the stated
uncertainty; the per-conflict time is taken from the `sat.rs` doc comment
(about 80 us at 767 variables, 52 rows) scaled by size: 50 us for C at
m = 2, 250 us for A and B, 100 us for G; 150 us for C and 800 us for B at m = 3.

| cell | K = 5 x 2^{(m-1) l} | s per target, all primary arms | N | central CPU-h |
|:--|--:|--:|--:|--:|
| 17,2,8 | 1.3e3 | 2 | 50 | 0.03 |
| 17,2,9 | 2.6e3 | 4 | 50 | 0.06 |
| 23,2,11 | 1.0e4 | 14 | 50 | 0.2 |
| 23,2,12 | 2.0e4 | 28 | 50 | 0.4 |
| 31,2,15 | 1.6e5 | 220 | 50 | 3.1 |
| 31,2,16 | 3.3e5 | 440 | 50 | 6.1 |
| 31,3,5 / 6 | 5e3 / 2e4 | 17 / 67 | 25 | 0.1 / 0.5 |
| 31,3,10 | 5.2e6 | 5.7e3 (C, C@2, B) | 10 | 16 |
| 31,3,11 | 2.1e7 | 2.3e4 | 10 | 63 |

Replicate slots 3 and 4 add about 80% at n = 31. The central total for every
cell except (31,3,11) is about 35 CPU-hours, about 9 hours on 4 cores, beyond
the 8-hour wall budget of the spec. The band for kappa and per-conflict time is
roughly 0.2x to 12x. Therefore the plan is gated:

* **G0 (pilot, round 0, excluded from inference):** 3 to 5 targets per cell at
  n = 17, 23, one n = 31 m = 2 cell, and one m = 3 ladder cell. `g3census gate`
  prints conflicts and ns per conflict per arm. Compute kappa and the per-arm
  time, and re-evaluate the table.
* **G1:** n = 17 and 23 cells run in full unconditionally.
* **G2:** each n = 31, m = 2 cell runs at N = 50 if the re-estimated CPU-hours
  for the cell are at most 8, else at N = floor(8 x 3600 / seconds-per-target)
  with a floor of 20; below 20 the cell is declared unresolvable and recorded
  as O-IMPEDIMENT(budget). The replicate slots (3, 4) run for a cell only if a
  primary candidate (interval upper bound < 0.95) or an unresolved interval
  exists; n = 17 and 23 always run them.
* **G3 (m = 3):** the ladder (31,3,5), (31,3,6) runs with all arms including A,
  and fits kappa_3 and the per-conflict times. (31,3,10) runs with arms C, C@2,
  B only (A and G dropped, AMENDMENT A7) at N = floor(12 x 3600 / seconds per
  target), at most 10 and at least 6; below 6 it is O-IMPEDIMENT(budget).
  (31,3,11) runs only if the re-estimate for N = 6 is at most 12 CPU-hours,
  expected not to be met. At N = 10 and s = 0.1 the MDE is 0.895 (table above):
  the m = 3 cell can establish "B is not much cheaper than C", not "B/C >= 0.95".
  Stating the m = 3 result as a 0.95-band test would be an overclaim.

Memory: well under 1 GB per process (largest encoding about 7e3 variables with
dense XOR rows of 110 words); peak RSS is recorded per row.

## 9. Variable ledger

| factor | treatment |
|:--|:--|
| instrument: commit, rustc -Vv, build flags, binary sha256, solver source sha256 | fixed, recorded at the freeze |
| host: CPU model, governor, SMT, co-tenants (other agents share these 4 CPUs) | measured per run (`lscpu`, load average before and after); wall time is a practicality note only, so contention affects no primary metric (conflicts and word ops are counted) |
| process pinning | fixed: one process per core, `taskset -c k` |
| memory layout, ASLR | uncontrolled and irrelevant to the counted metrics |
| arm order and warm-up | randomised per target (shuffled from the master seed); no warm-up effects on counts |
| targets | sampled independently of arm, distinct sigma-orbits; labels from the oracle |
| factor base V | fixed per cell (lexicographically first); sensitivity cells use `vsel` 1; **conclusions are conditional on one V per cell** |
| curve a | blocking factor (two disjoint abscissa pools of one instance family) |
| seeds | derived from the master seed; slots 1,2 primary, 3,4 replicate; C@2 vs C@1 is the A/A (independent renaming and phases, same encoding) |
| spot preemption | not used; if a process dies the rows file is incomplete and the cell is rerun from scratch under a new directory |
| analyst | the decision rule is mechanical; `summarize` is frozen before round 1 |

Comparison designs: A vs C, B vs C, G vs C, Cpoly vs C differ in exactly one
declared factor (the encoding, and for Cpoly the basis); A vs C also differs in
the instance size, which is part of the arm, not a nuisance. C vs Cnobreak and
B vs Bnogauge differ in one clause group. C@2 vs C@1 is the A/A.

## 10. What does not count

* Any ratio read from fewer than 10 paired targets, from censored runs, or
  from a void cell.
* A ratio produced after changing the priority set, the lex break, the budget,
  N, or the threshold. These are amendments with both versions reported.
* Conflicts without the operation counts: a conflict ratio below 1 with the
  op ratio above 1 in the band is "a count lowered without the total falling".
* Wall-clock as the headline.
* Reading Cnobreak or Bnogauge as arms to beat: they are detector checks.
* Citing KN-FIND-47da4e as evidence about linear stable bases (its object is an
  orbit union of a non-stable subspace, its baseline is the sum of n systems).
* Any statement about WDSat or CaDiCaL behaviour.

## 11. What the design cannot resolve

* Other solvers. A different clause-learning or XOR strategy, or a
  circulant-aware solver, could behave differently; the conclusion is for this
  solver only.
* Other orbit encodings: coordinate blocks, invariant-ring variables,
  Faugere-Svartz character blocks inside an F4, a gauge on j instead of the
  tuple, non-lexicographic canonical forms. Only two encodings are measured.
* The 0.95 threshold at N = 50 unless s <= about 0.10, and at N <= 10 not at all.
* A single V per cell, and n = 17, 23, 31 only: nothing about n = 41, 43 or
  131, and nothing about the exponent in n.
* The branching-priority sensitivity: all arms get priority on their own free
  variables. The sibling record found default order 2.5 to 15 times worse for
  the orbit arm; a no-priority replicate at n = 17 is a secondary run, not part
  of the rule.
* m >= 4 and chained systems beyond S4; m = 3 only at one or two cells with
  small N.
* The per-relation cost of the whole index calculus; the n-fold conjugate
  relation yield (C4) and equivariant elimination (C5) are separate claims
  not touched here.

## 12. Cross-reference to the audit

| audit item | handled by |
|:--|:--|
| C3 unsupported: Stage 2 never run | this protocol (prepared) |
| denominator mismatch with 47da4e | baseline is plain single-instance C, not n^{m-1} systems; the exact class count D(n,m) = C(n+m-1,m)/n enters only as the domain-count identity in the tests |
| object mismatch (union vs stable V) | stable V only |
| metric: per UNSAT target only | adds op counts at three weights, SAT-target cost and per-relation totals |
| solver dependence | single solver; stated as the main limit |
| ordering tuned for one arm | every arm gets priority on its own free variables; no-priority replicate secondary |
| m >= 3 never run | ladder plus one or two spec cells, gated |
