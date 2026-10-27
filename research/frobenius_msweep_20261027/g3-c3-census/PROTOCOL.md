# PROTOCOL: cell G3, the C3 census (H-BINSTD-c72284 Stage 2)

Status: **prepared, not run, not frozen** (revision 2 after independent
review; the changes are listed in section 14). The freeze happens at RUN_CARD
step 1: the frozen set (this file, AMENDMENT.md, RUN_CARD.md, `src/`,
`Cargo.toml`, `protocol.json`) is **committed**, `git status --porcelain` over
the cell and `src/cryptanalysis/sat.rs` must be empty, and the commit, the
file hashes, the binary sha256 and the toolchain are written to a separate
`FREEZE.txt` (not into this file, which is itself hashed). Until then nothing
here is a preregistration. The working tree this was written in is not a
frozen committed set. Deviations from the spec of
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
* Budgets, applied identically to every arm of a cell: conflict cap
  20 000 000 and deadline 900 s per solve for every m = 2 cell and for the
  m = 3 ladder cells (31,3,5), (31,3,6) (the `run` defaults, which the card
  passes implicitly); 40 000 000 and 14 400 s for (31,3,10) and (31,3,11)
  (passed explicitly). The pilot (round 0) uses a hard cap of 500 000 conflicts
  and 300 s so that a heavy arm is censored, not run away. The conflict cap is
  the intended censor rule because counts are deterministic; the deadline is a
  safety valve and is load dependent on shared CPUs. `g3census gate` prints,
  per cell and arm, the smallest conflict count among censored rows: a value
  below the cap marks a deadline censor, which is reported as a load artifact.
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
| G | C plus the comparator circuits of B on the same coordinate bits, **every** comparator (including x_2 <= x_3 at m = 3) made vacuous by a free selector | **generic size-matched symmetry-breaking control** (sound, no pruning). The review found that an enforced x_2 <= x_3 in coordinate order contradicts C's own u-lex break and made G unsound at m = 3 (about 48% of unordered triples inadmissible); fixed, and tested on every sorted triple at (7,3,3) and 120 sampled triples at (31,5,3) |
| U | C plus the comparators of B enforced against the fixed target | literal per-instance orbit clauses, **unsound by design**, tripwire |
| Cnobreak | C without the S_m break | informative control P1 (model expectation, section 4); not an arm to beat |
| Bnogauge | B without the gauge | informative control P2 (model expectation, section 4) |
| Chalf | C plus one unit clause **per slot** pinning that slot's top u-bit to 0 (m unit clauses; all slots in the lower half of V's coordinate order), **UNSAT targets only** (`--unsat-only-arms`; it loses solutions by design and is tested never to invent one) | positive control P3: search space retained is at most 1/2 under any enumeration model (see section 4 arithmetic); revision 3 replaced the slot-1-only pin, which retains 3/4 (m = 2) or 7/8 (m = 3) of the sorted space |
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
| positive P3 (voiding) | Chalf vs C, UNSAT targets, cells where Chalf runs (n = 17, 23, 31 m = 2, and the m = 3 ladder) | expected range 0.125 to 0.5 (arithmetic below), not a point value. Detected iff the 95% upper bound is < 0.95 | miss at >= 6 paired targets: cell void (O-ARTIFACT), the detector cannot see a >= 2x search-space reduction |
| informative P1 | C vs Cnobreak | model expectation in [0.5, 1.0] at m = 2 and [0.25, 1.0] at m = 3: once the first summand is fixed XOR-Gauss solves the second affinely, so a complete UNSAT proof enumerates first-summand branches and a lex break on the Gauss-solved slot may prune little (the review's argument, accepted). The earlier 1/m! was a symmetry-count prior, not a derivation | reported with interval; a miss (upper bound >= 0.95) is read "control uninformative: this solver does not exploit the S_m break", **never** a void and never a stop |
| informative P2 | B vs Bnogauge (n = 17 and 23 only) | <= 1; about 1/n only if the solver exploits the circulant gauge | same as P1 |
| soundness tripwire | U on planted SAT targets | at least one planted target per cell turns UNSAT (spec: H1 literal clauses change satisfiability) | none flipping anywhere contradicts S4 and is the headline |
| ordinary-curve unsoundness detector | B vs oracle on b = t | B disagrees somewhere | detector failed: B0 untrustworthy |
| same instrument | all | one binary, one solver source hash, one budget, one seed rule, arms interleaved per target (order shuffled per target from the master seed), arm labels in the rows | |

P3 arithmetic (revision 3). C's lex break forces u_1 <= u_2 (<= u_3), so slot 1
is the lexicographically smallest and its top bit is 1 only if every slot is in
the upper half. Pinning slot 1 alone therefore keeps 3/4 of the sorted space at
m = 2 and 7/8 at m = 3 (N = 1024 count: 393472 of 524800 = 0.7497 pairs; 0.8746
triples) and, with a seed sd(log) of 0.2 to 0.3 at N = 25, would not clear 0.95
on a sound instrument. Chalf pins the top bit of **every** slot: the retained
fraction is (1/2)^m of the ordered tuples, 1/4 (m = 2) and 1/8 (m = 3) of the
sorted space; if the solver instead enumerates the branches of one slot, half of
them are removed (0.5); the solver's actual enumeration order is not known
(nothing has been run). Expectation: Chalf/C in [0.125, 0.5] if cost follows
the search space, and anything below 0.95 counts as detected. The pilot gate
(RUN_CARD step 3) is numeric: pilot median Chalf/C must be <= 0.8 over the
pooled pilot UNSAT targets of the cells that run Chalf (at least 3 targets), or
the instrument is stopped for review; the 0.8 is the top of the section 13
range, chosen so a half-search-space effect with seed noise is not misread.

Controls run on the same code path (same builder, loader, solver, counters).
Only P3 has an effect derived from the model and is allowed to void a cell. P1
and P2 are unmeasured expectations on this solver (nothing has been run), so
they inform and never stop the campaign. P3 shows the detector sees a 2x change;
it does not show it sees a 5% one: that is the MDE of section 5, computed from
the sd of the A/C and B/C log ratios of the pilot (which include the
encoding-by-target interaction), not from the A/A sd alone. The A/A sd is a
floor for it.

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

Not analysed: per-relation and SAT-target cost. The rows of incidental and
planted SAT targets are recorded (verdicts, certificates, counters) and used
for soundness (S3) and the tripwire (S4) only; no cost statement is made from
them. By S3 all sound arms find the same SAT targets, so a per-relation ratio
would equal the per-target ratio of the totals and cannot show a yield
difference. The n-fold conjugate yield (claim C4) is an equivariance fact, not
measured here.

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

Planning versus decision sd. The table uses s; the relevant s is that of the
A/C and B/C log ratios in the pilot (seed noise plus encoding-by-target
interaction), at least the A/A sd. The SUMMARY `MDE` column is computed from
sd(log) and the number of **targets** (not of slot pairs: the two slots of a
target are not independent targets), and the decision-relevant minimum is read
from the A/C and B/C rows, not from s_AA alone.

Multiplicity. Sixteen cell-arm tests are read. The replicate slots re-run the
same targets and encodings with new renamings and phases, so they replicate
solver noise only, not target sampling: a favourable target sample reproduces
in slots 3 and 4. The chance that a "structure" label arises from target
sampling alone is therefore of order 16 x 0.025 = 0.4 before the G, Cpoly and
A/A conditions, not 16 x 0.025^2; those conditions and the 0.95 band reduce it
by an amount that is not computed here. Treat any single O-STRUCTURE cell as an
observation to replicate on fresh targets (a second run with `--seed-offset`),
not as a finding. The "relabel" label is an intersection claim over cells and
needs no correction.

Cheapest discriminator: n = 17, l = 8, arms C, C@2, A, B, G, Chalf, Cnobreak on 50
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

1. **O-ARTIFACT** if any sound arm (C, Cpoly, Cnobreak, A, B, Bnogauge, G,
   and Chalf on its UNSAT-only rows) disagrees with the oracle, or any SAT
   certificate fails, or P3 (Chalf/C, where run, >= 6 paired targets) is not
   detected (upper bound >= 0.95). The cell is void. P1 and P2 never void.
2. **O-IMPEDIMENT** if any arm-slot has more than 1% censored UNSAT targets (in
   practice any censored target at N < 100: dropping the hardest targets of one
   arm biases its median ratio down by about 5 percentile points, about 6% at
   sd(log) = 0.3, the size of the band), or if the cell has fewer than 10
   paired UNSAT targets for A and B (rule 2a), or, for a B and C only cell
   (rule 2b), fewer than 6.
2b. **O-BOUNDED** for a B and C only m = 3 cell with at least 6 paired targets
   (the (31,3,10) and (31,3,11) spec cells, AMENDMENT A7): the label carries the
   median B/C ratio and its 95% interval and states "excludes B/C below
   <lower bound>". It makes no structure claim (no A, no G, no replicate slots)
   and is never an O-RELABEL; at N = 6 to 10 the interval is wide and the
   statement is the large-gain exclusion only. The ladder cells (31,3,5),
   (31,3,6) carry all arms and are labelled by rules 3 to 5 at N = 25 (rule
   2a's minimum 10 applies).
3. For X in {A, B}: **candidate** if the 95% interval upper bound of the median
   conflict ratio X/C (primary slots) is below 0.95. A candidate is
   **confirmed** (O-STRUCTURE) only if all of (each of G, Cpoly and A/A is a
   required control: if it is absent or has fewer than 10 paired targets the
   candidate is not confirmed, fail closed): the replicate slots also have
   upper bound below 0.95; G/C upper bound is at least 0.95; Cpoly/C upper
   bound is at least 0.95; the A/A lower bound exceeds the candidate's upper
   bound. A candidate that is not confirmed gives O-INCONCLUSIVE with the
   failed conditions listed.
4. **O-RELABEL** if no candidate, both lower bounds are at least 0.95, and P3
   (Chalf/C) is present with at least 6 paired targets and detected (upper
   bound below 0.95); otherwise O-INCONCLUSIVE ("P3 absent or undersized").
   Dropping `--unsat-only-arms Chalf` from a cell therefore cannot yield a null label.
5. Otherwise **O-INCONCLUSIVE** (an interval straddles 0.95).

Cross-cell reading: `summarize` prints the O-TRIPWIRE headline (how many of the
cells that ran arm U on planted targets show at least one planted SAT target
flipped to UNSAT; the spec's tripwire is contradicted only if none does) and a
per-cell flag. O-RELABEL in all cells is "consistent with
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
| 31,2,15 | 1.6e5 | 220 (+15% SAT, U, Chalf) | 50 | 3.1 (3.6) |
| 31,2,16 | 3.3e5 | 440 (+15%) | 50 | 6.1 (7.0) |
| 31,3,5 / 6 | 5e3 / 2e4 | 17 / 67 | 25 | 0.1 / 0.5 |
| 31,3,10 | 5.2e6 | 5.7e3 (C, C@2, B) | 10 | 16 |
| 31,3,11 | 2.1e7 | 2.3e4 | 10 | 63 |

Replicate slots 3 and 4 add about 80% at n = 31. The central total for every
cell except (31,3,11), with the SAT, U and Chalf additions, is about 40 CPU-hours, about 9 hours on 4 cores, beyond
the 8-hour wall budget of the spec. The band for kappa and per-conflict time is
roughly 0.2x to 12x. Therefore the plan is gated:

* **G0 (pilot, round 0, excluded from inference, hard cap 500 000 conflicts and
  300 s, targets from `--seed-offset 1000` so that pilot targets are disjoint
  from the round-1 target streams and cannot leak into inference):** 3 to 5 targets per cell at
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
  Its label is O-BOUNDED (rule 2b, minimum N = 6, reconciled with this
  section); stating the m = 3 result as a 0.95-band test would be an overclaim.

Counts of targets and arms priced. The table above prices the UNSAT targets
of the primary arms only. The full cost adds: (i) satisfiable targets, which
the card caps at n = 31 (`--sat-keep 2 --planted 4`; the tripwire needs one
planted flip, and U flips about 1 - 1/n of planted targets), each costing about
half an UNSAT solve per arm in the model, about +12%; (ii) arm U on the planted
targets, a full UNSAT-style solve of a G-sized system, +2 to +4%; (iii) Chalf,
about half of a C solve per target, +6%; (iv) the m = 3 spec cells run no SAT
or planted targets. The central total in the RUN_CARD includes these (about 40
CPU-hours).

Evidence that the estimate may be badly low. During review a correctness-only
scratch run at (31,3,5) (planted SAT targets, arms C, Cnobreak, A, B, G, a
5 000 000-conflict cap and no deadline) had used 10 CPU-minutes and 1.7 GB
resident with 12 targets by 5 arms unfinished, against a predicted 17 s per
target for all arms. It was not attributed to an arm (probing was stopped). This
is one unreplicated observation, but it is in the direction the unmeasured
constants could take. `sat.rs` never compacts forgotten clauses out of its
arena, so resident memory grows with conflicts.

Memory and wall gates (pilot, `g3census gate`):

* **G4 (memory):** `max_rss_mb` and `max_conflicts` per arm. Projected RSS at
  the real cap = `max_rss_mb` x (cap / `max_conflicts`) in the worst (linear)
  case. Four processes run at once; each projected RSS must be at most 3 GB. If
  not, lower that cell's `--conflicts` until it is, and record the cap as the
  censor rule (a lower cap makes more rows censored and the cell more likely an
  O-IMPEDIMENT, which is the honest outcome).
* **G5 (re-estimate trigger):** if any arm's pilot mean conflicts or ns per
  conflict is more than 2x the section 8 table for the pilot cell, or any pilot
  row is censored, recompute every ladder cost from the pilot numbers before
  launching anything, and cut cells by the G2 and G3 rules on the recomputed
  numbers. The ladder never launches on the table above alone.

## 9. Variable ledger

| factor | treatment |
|:--|:--|
| instrument: commit (frozen set committed, `git status --porcelain` empty), `rustc -Vv`, `cargo -V`, RUSTFLAGS, binary sha256, solver source sha256 | fixed, recorded in `FREEZE.txt` at the freeze |
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

* Any ratio read from fewer than 10 paired targets (the O-BOUNDED statement of
  a B-only m = 3 cell from 6 targets is an interval and a large-gain exclusion,
  not a 0.95 test), from censored runs, or from a void cell.
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
* Target sampling reproducibility of a favourable sample: replicate slots
  re-run the same targets, so only a fresh `--seed-offset` run tests it.
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

## 13. Expectations written before data (not results)

Priors recorded here so that a later reader can see what was expected, with the
caveat that nothing has been run and these are reasoning, not measurement:

* Direction. No gain of order n or n^m from relabelling (free C_n action,
  counting identity). A and B pay roughly 10x more variables and XOR rows.
  Expected medians: A/C in 0.9 to 3, B/C in 0.9 to 2, G/C near 1, Cpoly/C near 1.
* Controls. P3 Chalf/C in 0.125 to 0.5 (all-slot pin; up to 0.8 allowed before the pilot gate stops). P1 in [0.5, 1.0] (m = 2).
  P2 <= 1, near 1/n only if the solver exploits the gauge. U flips most planted
  SAT targets (about 1 - 1/n).
* Labels. Probability that some cell carries an O-STRUCTURE label: about 7% (the
  most plausible source is solver exploitation of the circulant gauge in B at
  n = 31, which G and fresh-target replication are there to remove). About 40%
  per n = 31 m = 2 cell that the interval straddles 0.95 (O-INCONCLUSIVE),
  because CDCL seed noise on conflict counts is often 0.2 to 0.3 in log.
  (31,3,10) cut for budget about 35%, (31,3,11) about 80%.
* Outcome I would bet on: O-RELABEL at n = 17 and 23; relabel or inconclusive at
  n = 31; O-BOUNDED at m = 3. Nothing here can move the lead beyond "no large
  gain from these two encodings on this solver".

## 14. Revision 2 (changes after independent review)

G made sound at m = 3 and tested on single-solution triples; chunked and
replicate rows keyed by target (xr); B-only m = 3 cells get a rule (O-BOUNDED)
and the minimum N is reconciled (10 full, 6 B-only); P1 demoted to
informative, P3 (Chalf, derivable) added as the voiding control; memory and
re-estimate gates, hard-capped pilot and disjoint pilot targets; censoring
tolerance 1%; MDE from targets; replicate-slot multiplicity restated; tripwire
headline, per-cell flag and S6 for A and B implemented; per-relation promise
dropped; freeze procedure rewritten (committed set, FREEZE.txt, binary hash).

## 15. Revision 3 (changes after the round-1 review of revision 2)

* Freeze mechanics made executable: tests write to the OS temp dir (the
  tracked `target/test-*` fixtures are removed), the cell has a `.gitignore`
  for `target/` and `reports/run-*/`, RUN_CARD builds with `CARGO_TARGET_DIR`
  outside the tree, and steps 1a/1b are exact commands (README.md is in the
  hash list). `protocol.json` still has to be authored and validated with the
  sibling `crypto-autoresearcher` checker; the schema was not available in the
  preparation checkout, so this is a pre-freeze step that blocks the run if it
  fails (RUN_CARD step 1a).
* P3 (Chalf) re-derived: pin on every slot, expectation a range 0.125 to 0.5,
  numeric pilot gate (section 4).
* Decision rule rule 3 fails closed on missing or under-10-target G, Cpoly or
  A/A (new unit test).
* Budget: gate G1 at n = 17 and 23 is not exempt from an abort: the first
  censored UNSAT row in any cell stops that cell's remaining targets (RUN_CARD
  step 4 note); a censored pilot row is a lower bound only and triggers G5.

## 16. Amendment 2026-10-27 (round-2 review of revision 3)

* Rule 4 (O-RELABEL) now fails closed without its positive control: P3
  required with >= 6 paired targets and hi < 0.95, else O-INCONCLUSIVE. New
  unit test `relabel_requires_the_positive_control`. AMENDMENT A19.
* Cargo.lock is ignored by the root `.gitignore` and is not committed: the
  freeze `git add` no longer lists it (it exited 1); its sha256 stays in
  FREEZE.txt. RUN_CARD step 1 corrected.
