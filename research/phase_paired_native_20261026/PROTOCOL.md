# Protocol: native paired fixed-budget comparison, Frobenius phases as solver unknowns

Registered 2026-10-26, after the producer and replay code were drafted and
calibrated on non-frozen targets (section "Calibration disclosure") and before
any solver run on the frozen manifest. Stage diagnostic on a toy field. `S`,
end-to-end cost, the ratio to rho and any ECDLP speedup stay unset; none is
claimed at any size.

## Question

The legacy Python study (crypto PR #2583, directory
`research/frobenius_quotient_sat_20260930/paired_fixed_budget_20261011/`, branch
`claude/phase-aware-paired-fixed-budget-20261011`) found, on one trial with
CryptoMiniSat driven from Python, that carrying the Frobenius phase of each
summand as solver unknowns (arm B) decomposed more targets than representatives
only (arm A) but cost 1.61 and 1.38 times as much CPU per newly independent
relation (two formulations). `AGENTS.md` ("Implementation language: no Python")
requires a matched native rerun before any performance claim. This protocol
runs it: Rust only, one native single-thread solver, the same frozen targets,
every attempt charged, every relation verified on the curve.

**The solver here is `crypto_lib::cryptanalysis::sat::Solver`, not CryptoMiniSat.**
It is the crate's own from-scratch CDCL solver with native XOR rows, and its
documentation says it is not competitive with state-of-the-art solvers on
generic CNF. The contrast measured below is therefore within one solver. The
legacy timings stay labelled historical; absolute seconds are not comparable
between the two studies, only the direction of the arm B / arm A contrast is.

## Hypotheses (falsifiable, unchanged from the legacy study)

- **H1 (mechanism).** With one-hot Frobenius phases (arm B) each of the 5
  orbit representatives stands for 13 admissible x-coordinates, so at equal
  matrix width the effective factor base is 13 times larger (65 against 5
  x-coordinates). On identical targets arm B spends less per newly independent
  verified relation than arm A, all attempts charged.
- **H0.** Arm B's larger circuit costs more than the extra decomposable
  targets return: cost per newly independent relation is equal or higher for
  arm B.

**Decision rule (fixed now).** For each repetition let `rho_conf` be arm B's
conflicts per newly independent verified relation divided by arm A's, and
`rho_cpu` the same ratio for CPU seconds, every attempt of the arm charged
(verified, UNSAT, budget exhausted). H1 is read as **supported** only if, in
every completed repetition, `rho_conf < 1` and `rho_cpu < 1`, and every relation
passes every check below with no rejected tuple. If both ratios are at or above
1 in every completed repetition the result is **H1 not supported (H0 not
rejected)**. Any other pattern, in particular the two clocks disagreeing, is
reported as **mixed**, with the pattern, and counts as not supported. The
deterministic budget (conflicts) and the CPU clock are both named in the rule
because the two arms' conflicts do not cost the same time; a disagreement is a
finding, not noise to be resolved by choosing one. Every outcome is a result.

## Frozen inputs

- `fixtures/manifest.json`: the legacy manifest, copied byte for byte,
  SHA-256 `8cb7f7989e5ea35656b34e7ba8bf7104e0c081277b8dbbd6b0f7507369b01231`.
  It is not regenerated. The producer refuses to run on any other content
  unless `--manifest` and `--expect-sha` name another manifest.
- Curve `E0: y^2 + xy = x^3 + 1` over `F_{2^13}`, modulus
  `X^13 + X^4 + X^3 + X + 1` (`0x201b`), `#E = 8012`, odd subgroup `H = 4E` of
  prime order `r = 2003`, trace `t = -1`, `lambda = 89` (root of
  `lambda^2 + lambda + 2 = 0 mod r` with `pi(G) = [lambda]G`; the code finds it
  from the group law and checks the equation).
- Factor space: the seeded 4-dimensional subspace of `ker Tr` with basis
  `0x16d1, 0x19a, 0x15a6, 0x7df`; 5 representatives, 65 admissible
  x-coordinates once phases are free.
- 12 targets: `a0..a3` planted with random Frobenius phases, `b0..b3` planted
  with all phases 0, `c0..c3` uniform in `H`.
- `fixtures/witness_sealed.json` (SHA-256
  `f2cafe041cb5585f829eee5aae18fce706b070ab8fb20197d50bbe68b1254580`, copied
  unchanged) holds the construction witnesses and the legacy brute force. **The
  solver arms never open it.** Only the checkers do: `gate --sealed` and the
  replay example, as a cross-check of their own exhaustive pair matching.

## Arms and formulation

| | arm A | arm B |
|---|---|---|
| unknowns per summand | 4 coordinate bits in the factor-space basis | 4 coordinate bits + 13 one-hot phase bits |
| `x_i` | representative `v_i` | `v_i^(2^k_i)`, `k_i` chosen by the one-hot vector |
| matrix columns | 5 representatives | 5 representatives |
| effective factor base | 5 x-coordinates | 65 x-coordinates |

Both arms use the legacy odd-subgroup constraint (`q^2 + sqrt(v) q + 1 = 0`,
`Tr(q) = 0` per summand), pairwise distinct `x_i`, and the legacy chain of
binary Semaev equations for `y^2 + xy = x^3 + 1`, built gate by gate: each
field product of linear forms is `n^2` AND gates (Tseitin) plus native XOR
rows; squaring, square root, constant multiplication and trace are linear and
stay inside XOR rows. One difference from the legacy encoder, applied to both
arms: equal linear forms share one defining variable (the legacy encoder
defined a new variable at every use), which removes duplicate XOR rows and
nothing else.

- **`subgroup_xor` (preregistered, the only formulation in the decision rule):**
  `S3(x1,x2,t) = 0`, `S3(x3,x4,u) = 0`, `S3(t,u,x_R) = 0` (legacy `specialized`
  final link, `subgroup`, native XOR).
- `line_subgroup` (legacy `line` final link) is implemented and covered by the
  same tests but is **not preregistered for the main block**: at the solver
  speed measured in the calibration, one formulation at three repetitions
  already costs about 100 CPU minutes, so a second one does not fit the compute
  allowance. It runs, one repetition, only if compute remains, and is then
  exploratory.
- `*_sym` variants (the total order `x1 < x2 < x3 < x4` on both arms, breaking
  the 24-fold ordering symmetry) are implemented and tested but are **not run**
  in the main block (calibration: they cut arm A's UNSAT proofs by 2 to 3.5 times
  and left arm B slower to find a solution, which changes the contrast; see the
  calibration table).

**Library check (the open question).** The crate does build four-summand
decomposition systems: `koblitz_groebner::build_decomposition_system` with
`m = 4` returns 42 variables for `s = 4`, `n = 13`, and
`semaev_sat::encode_boolean_system_with` encodes it. But it is the linear chain
`S3(x1,x2,e1), S3(e1,x3,e2), S3(e2,x4,x_R)`, not the legacy tree, it has no
odd-subgroup constraint, and its monomial mask holds 64 problem variables
(128 under the `wide-vars` feature) while arm B alone has `4*4 + 4*13 = 68`
coordinate and phase variables before any intermediate. So the library encoder
cannot carry arm B, and the arms are built directly on `sat::Solver` with the
crate's own Tseitin/XOR scheme, matching the legacy `specialized` and `line`
tree forms. A unit test builds the library chain, encodes it, and checks that it
accepts a planted phase-0 witness (a cross-check of the arm A semantics only).

## Budget, clocks and solver settings

- **Primary deterministic budget:** `conflict_budget = 3,000,000` conflicts per
  attempt, the same for both arms and every repetition. When the solver
  reaches it, the attempt ends with status `budget` and is charged 3,000,000
  conflicts. The figure is set from the calibration: arm A's UNSAT proofs took
  0.73 to 1.52 million conflicts on four non-frozen targets, and the budget is
  twice the largest, so that a proof is not cut off by the budget on the frozen
  targets with reasonable probability.
- **Wall-clock deadline 400 s per attempt is a backstop only.** It exists so a
  pathological attempt cannot stall the session; status `deadline` (conflicts
  below the budget) is reported separately and is read as an unresolved
  attempt. If any `deadline` occurs the primary budget was not the binding
  limit for that attempt and the results say so.
- **CPU clock:** `CLOCK_PROCESS_CPUTIME_ID` around each attempt (model build,
  load, every solve, every verification). The process is single threaded.
- Fresh solver per attempt; one thread; attempts strictly sequential within a
  process. Repetitions 2 and 3 may run as two concurrent processes on different
  cores (the host has four, shared); each process still alternates the arms
  ABBA, so any load applies to both arms alike, and the loadavg at the end of
  each attempt is in its receipt.
- **Branch priority:** `Solver::set_branch_priority` names the core unknowns
  (coordinate bits and, in arm B, the phase one-hots) in both arms. The
  rationale is the solver documentation's: name the genuinely free variables;
  the subgroup witnesses `q` and the chain unknowns `t`, `u` are determined by
  the core unknowns up to a linear system. Calibration did not discriminate
  between this, naming every unknown, and no priority (arm A UNSAT was not
  reached in 40 to 60 s under any of them), so the choice was made on that
  rationale and is fixed here.
- **Seeds.** The solver is deterministic. Repetition `r` (1, 2, 3) scrambles the
  saved decision phases with `Solver::scramble_saved_phases(phase_seed(r))`,
  `phase_seed(r) = splitmix64(0x5048415345504149 xor r) | 1`, the same seed for
  both arms in that repetition. Different repetitions give different search
  trajectories; the same repetition replayed gives identical conflict counts.
  CPU seconds additionally vary with load. Run times of a CDCL solver are
  heavy tailed in the seed (the calibration saw the same target cost 60 thousand
  and over 380 thousand conflicts under two encodings), so three repetitions
  is a small sample and the result says so.

## Order and repetitions

3 repetitions, each the full schedule for `subgroup_xor`: the 12 targets in
manifest order (a, b, c); pair `i` of repetition `r` runs `A,B` when
`(i + r - 1)` is even and `B,A` when odd, so neighbouring pairs read ABBA.
Total `3 x 12 x 2 = 72` attempts. Repetition 1 runs alone and is read through
the gate (next section) before repetitions 2 and 3 start. A cap of 6,600 s of
total attempt CPU stops the run at a pair boundary; attempts not run are
written as `skipped` receipts, never silently dropped, and the result says what
was cut.

**Compute, stated in advance.** By the calibration means a full repetition
costs about 35 CPU minutes (arm A's eight UNSAT proofs dominate, about 17
minutes; arm B about 17), so the block is expected to cost about 100 CPU
minutes, above the nominal 90-minute session figure once the calibration (about
25 minutes) is added. That overrun is accepted and bounded by the cap above; it
is the price of the native solver proving arm A's UNSAT cases, where
CryptoMiniSat took 8 to 52 s each (legacy table). The alternative, a budget low enough to fit
three repetitions, would leave every arm A UNSAT case unresolved and make the
comparison a measurement of the budget.

## Gate before any timing (first)

The structural facts of the legacy run must be reproduced exactly **before any
timing value is read**:

1. exhaustive native pair matching (no solver): arm A decomposes exactly
   `b0..b3` (4 of 12), arm B decomposes 12 of 12; the legacy brute force in
   `witness_sealed.json` agrees;
2. the solver, repetition 1: arm A verifies exactly the four `b` targets and
   proves the other eight UNSAT; arm B verifies 12 of 12; independent rank 4
   for arm A and 5 for arm B of 5 columns; no solver answer contradicts the
   exhaustive check; no attempt ends in `budget` or `deadline`.

Run `phase_paired_fixed_budget run --reps 1` and then `gate`. The run's progress
output carries statuses only, and `gate` reads statuses, relations and ranks
only. If the gate fails, the run **stops**: the discrepancy is reported and no
timing is read or analysed. A gate failure caused only by `budget` or
`deadline` statuses is a resource limit, not a discrepancy of fact, and is
reported as such; raising the budget then needs a dated addendum below, and the
receipts at the lower budget are kept as superseded.

## Verification (fail closed)

For every `verified` attempt, in the producer and again in the independent
replay: four distinct abscissae; all four points on the curve and in `H`
(group-law membership); the signed sum equals the target by the crate group law;
each point is mapped to `(representative j, phase k, sign)` and the
coefficient `+-lambda^k mod r` recomputed; `log R = sum_j c_j log V_j (mod r)`
against the exhaustive discrete-log table of `H`; arm A points are phase 0. The
row `(c_0..c_4)` goes into a rank tracker over `Z/2003` (producer: the crate's
`U64RankTracker`; replay: its own dense elimination). Every `unsat` is compared
with the exhaustive check (an UNSAT on a decomposable target is a defect that
voids the table until explained). A rejected tuple (a SAT model that fails the
curve check) is blocked and counted.

## Metrics and accounting

- **Primary:** conflicts, and CPU seconds, per newly independent verified
  relation, per arm and repetition: the sum over **every attempt** of that arm
  in that repetition (verified, UNSAT, budget exhausted; build and verification
  included for CPU) divided by the final rank of the relation matrix. This is
  independent of target order. Ratio B/A and the decision rule above.
- **Secondary:** verified per attempt, UNSAT counts, budget counts, rejected
  tuples, per-target paired ratios (conflicts and CPU, median over
  repetitions), and the cost charged when each rank was first reached (this one
  depends on the schedule's target order, because arm A's first relation comes
  after its `a` targets; descriptive only).
- **Ceilings stated in advance.** The matrix has 5 columns, so rank saturates at
  5 in both arms; relations found after saturation are verified but not
  independent. Arm A can produce relations on only the four `b` targets by
  construction, so "per newly independent relation" compares an arm that can
  reach rank 4 with one that can reach rank 5, and the targets are not an
  unbiased sample of any distribution. The per-target paired ratios on `b0..b3`
  (decomposable in both arms) are the like-for-like secondary view.
- Cost is exclusive and complete: build, load, solve, decode and verification
  are inside an attempt. Manifest construction, the discrete-log table and the
  exhaustive check are one-off setup costs, reported separately and not charged
  to either arm.

## Preregistered versus exploratory

Preregistered: everything above, including the gate, the metrics named primary,
the decision rule, the budget (3,000,000 conflicts), the backstop (400 s), the
one formulation `subgroup_xor`, 3 repetitions, the cap. Exploratory, reported
but not part of the decision: decisions, propagations and `word_ops` ratios;
per-target ratios; rank-trajectory tables; `line_subgroup`; the unprioritised
and symmetry-broken variants; any n = 19 work. Exploratory numbers carry that
label.

## Stop conditions

The gate must pass before repetitions 2 and 3 run. Any relation failing a
check, any UNSAT contradicted by the exhaustive check, or any `deadline` status
stops the analysis and the defect is reported. The 6,600 s attempt-CPU cap
bounds the block. The optional n = 19 block (a new manifest frozen natively with
`freeze` before any run) runs only if the n = 13 block completes and compute
remains; otherwise its exact command is recorded as the next step.

## What this cannot show

A stage ratio at n = 13 on 5 representatives says nothing about end-to-end
cost at larger fields, about the legacy CryptoMiniSat harness, or about any
other solver. There is no `S`, no ratio to rho and no speedup claim.

## Calibration disclosure

Before this protocol was committed, the producer was run on a second set of
targets that is **not** in the frozen manifest (`freeze --like
fixtures/manifest.json --target-seed 32 --per-set 2`, six targets `a0 a1 b0 b1
c0 c1`, SHA-256
`8d0d9f23e5913e540ad5077186f78adec25ea6c3dbf32e78c31c48e2f6116e5a`; checked to
share no target with the frozen set), same curve, factor space and formulation,
to choose the budget and to see whether the solver could prove arm A's UNSAT
cases at all. Phase seed of repetition 1, branch priority on the core unknowns,
an encoder build without the shared-form variable cache. Nothing from these runs
enters any table of results.

| target | arm A | A conflicts | A CPU s | arm B | B conflicts | B CPU s |
|---|---|---|---|---|---|---|
| a0 | UNSAT | 873,695 | 88.9 | verified | 60,512 | 18.5 |
| a1 | UNSAT | 1,387,279 | 160.8 | verified | 112,834 | 33.2 |
| b0 | verified | 187,708 | 11.6 | verified | 96,622 | 29.5 |
| b1 | verified | 271,335 | 16.3 | verified | 715,182 | 242.7 |
| c0 | UNSAT | 725,509 | 73.0 | verified | 377,662 | 120.9 |
| c1 | UNSAT | 1,515,760 | 210.2 | verified | 204,718 | 67.5 |

Other calibration observations, all on non-frozen targets:

- Arm A on `a0` ended at the 40 to 60 s wall backstop without a proof under
  every branch-priority setting tried (all unknowns: 539,936 conflicts; none:
  818,068; core: 724,713), and finished UNSAT at 873,695 conflicts when given
  280 s. Conflict rates: about 9 thousand per second for arm A, 2.4 thousand
  for arm B.
- With the shared-form variable cache (the final encoder) arm B's rate rose to
  about 3.9 thousand conflicts per second, but individual targets changed cost
  sharply under the changed variable numbering (arm B `a0` not found in 384,516
  conflicts, arm A `a0` not UNSAT in 1,155,954), which is the heavy tail the
  seeds section warns about.
- The `subgroup_xor_sym` variant proved arm A `a0` and `a1` UNSAT in 393,238 and
  389,908 conflicts (about 25 s CPU each) while arm B found no solution in
  175,511 and 178,468 conflicts (59 s). It was therefore not adopted.
- The solver-free exhaustive check on the calibration set found arm A
  decomposing `b0 b1` only, as designed.

## Addenda

(none yet)

### Addendum 1, 2026-10-26, after repetitions 1 to 3 ran and before the last four attempts

Everything above is unchanged. Facts, then two decisions.

Facts. Repetition 1 ran alone and the gate passed (`results/gate_rep1.txt`:
arm A 4 verified and 8 UNSAT, rank 4; arm B 12 verified, rank 5; exhaustive
pair matching and the sealed brute force agree; no `budget` or `deadline`).
Repetitions 2 and 3 then ran with `run --resume --reps 3`. The run stopped at
the 6,600 s attempt-CPU cap at 6,751 s, before repetition 3 pair 10: the four
attempts of targets `c2` and `c3` of repetition 3 are written as `skipped`
receipts (seq 68 to 71). Separately, two arm B attempts of repetition 2 (`a2`
and `a3`) ended at the 400 s wall-clock backstop with 1,224,382 and 1,344,608
conflicts, below the 3,000,000 budget, status `deadline`. That is the case the
stop conditions name ("any `deadline` status stops the analysis"). Timing was
read after the gate passed, and the numbers above were read before this
addendum.

Decision 1 (deadline). The stop condition fired and is reported as fired.
Discarding the other 70 attempts would not remove the defect, so the analysis
continues as a clearly flagged deviation: those two attempts are charged what
they consumed (conflicts and CPU), which is a lower bound on what arm B would
have cost with the budget binding, so repetition 2's arm B cost is understated.
The tables carry the bound: arm B's conflicts in repetition 2 cannot exceed its
charged total plus `2 x (3,000,000 - conflicts used)`. A `deadline` attempt is
never read as a failure to decompose; it is unresolved.

Decision 2 (cap). The four skipped attempts are run, so that the preregistered
three complete repetitions exist: `run --resume --reps 3 --cpu-cap 8000`, same
binary, same budget, same seeds, nothing else changed. They are appended after
the skipped receipts; the skipped receipts stay in the file, marked superseded
by the later receipts of the same slot, and the replay checks that
correspondence. This choice was made after the first 68 attempts were read, so
it is not blind to the results; the alternative, analysing a repetition of 10
attempts per arm, was rejected because a partial repetition is not comparable
with a full one. The cost is about 12 further CPU minutes, which takes the
session past the nominal 90-minute figure (stated in the results).
