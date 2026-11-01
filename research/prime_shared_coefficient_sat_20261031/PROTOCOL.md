# Staged protocol: shared-coefficient SAT/F4 discriminator

Status: **E0 complete; the E1 SAT and msolve subgates are frozen but unrun;
cvc5 1.4.2 is proof-ineligible; E2--E5 remain draft**. This document was
written before any affected SAT, XOR, QF_FF or F4 study output. The E0 toy
correctness result was already visible and is not used as performance
evidence. The exact freezes are [E1_PROTOCOL.md](E1_PROTOCOL.md) and
[E1_COMPARATOR_PROTOCOL.md](E1_COMPARATOR_PROTOCOL.md).

## Question and scope

On one exact selected-chart prime-field decomposition predicate, does any
preregistered CDCL/CNF, CNF-XOR or QF_FF bundle show lower complete-work growth
than exact enumeration and msolve/F4 on unseen natural targets?

The experiment can advance or reject a solver bundle on the declared ladder.
It cannot prove or disprove subexponential prime-field ECDLP. A surviving
bundle must still pass complete relation collection and a family theorem.

## Hypotheses

H1: shared `U` coefficients across the quotient/remainder tower expose repeated
local implications, allowing at least one SAT bundle to prune a growing
fraction of the exact coefficient space.

H2: any reduction is caused by the elliptic/shared-coefficient structure, not
only the sparse support graph.

Predictions fixed for E3:

- H1 advance threshold: corrected upper 95% interval for complete-work slope
  at most `0.85` over the three largest rungs, plus a retired-instruction win
  over F4 on the two largest jointly completed rungs outside A/A spread.
- H2 structure threshold: candidate/matched-null paired instruction ratio at
  most `0.8` on the three largest rungs with corrected 95% interval below 1.
- Evidence against advancement: lower 95% slope interval at least `0.95` on
  three consecutive rungs, or every SAT bundle censoring before F4 on both
  largest jointly attempted rungs.

## Stage E0: canonical correctness gate (complete)

Inputs:

- historical F17 remainder witnesses for `(m,t)=(3,4),(5,4),(7,4)`;
- all 18 affine targets on `y^2=x^3+2x+2` over F17 for `(m,t,c)=(3,3,1)`.

Implementations compared:

1. coefficient enumeration over monic `A`, constant nonzero `C`, norm
   division, split roots and canonical extraction;
2. independent affine curve enumeration, triple group addition, and linear
   interpolation of `(A,C)` followed by independent root-product `U`.

Primary metric: exact equality of the projected
`(a0,a1,c0,u0,u1,u2)` model set for each target. The two paths are
algorithmically separate but same-author and share basic finite-field and
polynomial primitives; E1 therefore still requires external backend and
certificate checks. Controls: three historical known answers, one `a0+1`
mutation per fixture, malformed schema/spec/assignment tests, strict parse
round trip, deterministic exporter tests.

Fixed result: 18 targets, 6 SAT, 12 UNSAT, 6 models in each implementation,
18 exact set matches. No timing was collected. See `results/e0-panel.json`.

## Stage E1: representation-equivalence gate (subgates frozen, unrun)

Implement parsimonious finite-field arithmetic lowerings from the canonical IR
to CNF and CNF-XOR. Do not benchmark until this gate passes.

The shared functional circuit, strict exporters, projected-model runner and
proof-mutation controls are implemented. The CaDiCaL/CNF and
CryptoMiniSat/CNF-XOR panel is frozen separately. A strict msolve
parametrization parser, F17 extractor and fixed-control runner are also
implemented and frozen. Neither panel has run because the checkpoint launch
gate remains unavailable.

Pinned cvc5 1.4.2 cannot emit a trust-free, independently checkable
finite-field UNSAT proof: complete proof checking stops at trusted `THEORY_FF`
rewrites and Alethe output rejects `CONST_FINITE_FIELD`. It can be only an
optional SAT-model cross-check and cannot close an UNSAT cell.

Fixed tiny panel:

- all 18 E0 F17 targets;
- the three `t=4` known witnesses at `m=3,5,7`;
- one deterministic mutation of every known witness;
- malformed/truncated/trailing-input cases.

Arms: canonical evaluator, exact enumeration, msolve, optional cvc5 QF_FF,
CaDiCaL CNF and CryptoMiniSat CNF-XOR. Compare projected semantic variables,
not auxiliary models. SAT models must pass canonical evaluation, root
extraction and group replay. msolve algebraic-closure parametrizations must be
filtered to Fp and replayed; their empty sets remain uncertified. UNSAT is
accepted only with a certificate checked by the pinned official checker;
`unknown`, timeout or unchecked UNSAT remains unknown.

Pass rule: every completed backend has exactly the same projected model set on
enumerable cases, every positive control is detected, every mutation is
rejected, and all verifier mutations fail. Any mismatch blocks E2 and is
reported by target/backend; it is never repaired after looking at E3 data.

## Stage E2: scratch preflight and freeze

Use disjoint protocol-hash seeds and only the two smallest scratch rungs. Four
instances of each class (natural, planted, exact-UNSAT) per scratch rung give
24 instances. These rows may select:

- one pinned version, image digest and flag set per backend;
- one common timeout and memory ceiling;
- fixed finite-field bit sizes and curve-generation rules for each E3 rung;
- CNF field representation and XOR extraction rule;
- whether valid retired-instruction counters are available;
- the final sample size needed for 80% power at family alpha `0.05`, Holm over
  the three candidate bundles, for a 15% slope reduction and 1.5x paired
  instruction ratio under observed variance.

Preflight seeds and cells cannot enter E3. A single preflight amendment is
allowed before freeze and must list every changed field. After freeze, no
solver, flags, field sizes, target class, null generator, timeout, metric,
threshold or exclusion changes are admissible.

Current freeze blockers: completion of both frozen E1 launches; independently
checked UNSAT coverage; a successful durable checkpoint; exact E2 field/curve
cells; protocol-derived seed list; detector and power receipts; valid `perf`
access; analysis script/hash; and an authorized isolated execution
environment. Pinned development builds of msolve, cvc5, CaDiCaL,
CryptoMiniSat and both proof checkers exist locally, but no study input has run.
The current host lacks `perf` access, an authorized checkpoint credential and
an EKS image carrying this custom toolchain.

## Stage E3: held-out solver tournament

### Design

Rungs: `(m,t)=(3,3),(5,4),(7,5),(9,6)`.

Target classes: natural, planted and exact-UNSAT. Eight independently generated
instances per class/rung give 96 semantic instances. One support/model-matched
null is generated per instance. Sample size remains provisional until the E2
power gate passes; if increased, it is changed before protocol freeze.

Arms:

- `enum-ref`: exact enumeration/reference;
- `f4-base`: msolve F4;
- `cdcl-cnf`: CaDiCaL on parsimonious CNF;
- `xor-cnf`: CryptoMiniSat on CNF-XOR;
- `qf-ff`: cvc5 QF_FF;
- `cdcl-aa`: byte-identical fresh-process A/A;
- `planted-positive`: detector control through the same CDCL path;
- `matched-null`: curve-free coefficient randomization preserving field,
  variable/equation/degree multiset, monomial-support incidence graph, chart,
  class and exact model count where enumerable.

One warmup and three measured fresh processes per arm/instance. Arm order is
protocol-hash randomized and interleaved within instance/replicate. Solvers are
single-threaded. The planned isolated backend is EKS Spot with two allocated
CPUs; memory is fixed after E2 and equal across arms.

### Measurements

Primary:

1. verified status and complete projected model set;
2. complete target-dependent retired instructions, from IR compilation through
   export, solving, model/proof parsing and independent verification;
3. peak RSS of wrapper and descendants;
4. Theil--Sen slope of `log2(instructions)` on
   `log2(exact candidate-space size)`.

Secondary: CPU cycles; wall time only for valid L2-isolated rows; decisions,
conflicts, propagations, restarts and learned clauses; XOR eliminations; F4
solving degree, rows, columns, nonzeros and multiply-adds. SAT native-work is a
sensitivity band
`decisions + conflicts + w*propagations`, `w in {0,0.1,1}`; it is never
converted into group additions.

Record CPU/kernel/governor/turbo/SMT/NUMA, binary and image hashes, isolation
receipt, pressure/steal, run CPU, thread count, setup/solve/verification
instructions and every failure.

### Analysis and uncertainty

Use paired medians and paired bootstrap 95% intervals over held-out instances;
Holm-correct the three candidate bundles. Fit Theil--Sen slopes and bootstrap
by instance, preserving target/null and replicate pairing. Show every instance,
timeout and exclusion. Do not impute censored work. Compare every claimed
difference with the A/A distribution. Analysis labels remain masked until the
frozen script has emitted results.

### Stopping, retries and exclusions

- Fixed cells and fixed `n`; never stop on a favorable slope.
- Identical per-row timeout and memory limits after E2.
- Infrastructure failures may retry once. Timeout, OOM, solver exit and wrong
  answer are outcomes and never retry.
- A failed planted detector invalidates the corresponding negative reading.
- A SAT model failing canonical/group replay is `wrong_answer`.
- Unchecked UNSAT is `unknown`.
- Invalid instruction counters exclude only instruction comparisons and are
  listed. Wall time below L2 or after restore is descriptive only.
- A null that misses its frozen match is excluded only from H2 and listed.

Verdicts are `sat_screen_advanced`, `sat_screen_not_advanced`,
`wrong_answer`, `invalid_measurement`, `within_control_distribution`, or
`inconclusive`.

## Stage E4: complete index-calculus escalation

Run only for a bundle advanced by E3. Freeze a prime/curve ladder and compare
against the strongest repository collector and rho baseline. Count factor-base
construction, target-independent and per-target compilation, relation yield,
all failed attempts, extraction, verification, duplicate removal, sparse
matrix construction/solve, individual logarithm, memory and precision. Require
independent scalar replay of recovered logs. A local solver win that disappears
in relation yield or linear algebra does not advance.

## Stage E5: theorem gate

Only after an E3/E4 signal, identify the load-bearing invariant (for example a
bounded-width elimination order, cut rank or propagation certificate), prove
it for the growing family, and insert it into relation-collection and linear
algebra complexity. Empirical slopes alone are not an asymptotic proof.

## Required artifacts

Frozen protocol and seed list; canonical IR and every export with hashes;
solver binaries/versions/flags/image digest; raw row per attempt; proof/model
objects and checker logs; isolation and detector receipts; masked analysis and
unmask record; complete export verifier; independent replay; report with
quantitative graphs. No aggregate is acceptable without auditable raw rows.
