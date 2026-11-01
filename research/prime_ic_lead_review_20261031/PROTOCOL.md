# Draft protocol: arity-three prime-field CDCL(T) exponent screen

**Status:** design complete enough for review, **not frozen and not executable
yet**. No affected data may be taken until the native instrument, the exact
CaDiCaL external-propagator source, theory-explanation checker, external solver
versions and hashes, preflight runtime, power justification and analysis script
are filled into `protocol.draft.json`, the record validates, is frozen, and is
committed.

## Question

At fixed arity `m=3`, does hybrid CDCL(T) with an `F_p` theory, or cvc5
`QF_FF`, achieve point-decomposition work `2^(e s)` with the upper corrected
95% interval for `e` strictly below `0.5`, while returning the exact model
status and surviving enumeration, planted, UNSAT, curve-free-null and A/A
controls?

The experiment tests a solver/representation family. It cannot prove a
subexponential ECDLP algorithm. A failing arm refutes only that frozen
representation, solver version, parameter ladder and budget.

## Semantic instance, fixed before representation

Use a nonsingular `E/F_p` with known prime whole-group order and the corrected
arity-three predicate:

- three disjoint one-hot windows of rational factor-base points;
- exactly-one and forbidden-word clauses in the Boolean skeleton;
- two chained `S3` relations through an intermediate point and the quadratic
  lambda lift; no expanded `S4` polynomial;
- the nonzero-displacement chart, with every zero-displacement exceptional case
  represented explicitly;
- the guarded three-summand/disjoint-window rationality argument, so no
  additional rationality circuit is present at `m=3`;
- sign recovery, curve membership and independent group-law replay.

The lambda equation `(2t*lambda-s0)^2=f(x)` has discriminant
`16*t^2*f(x)`. For nonzero `t` it gives a rational lambda exactly when `x`
has a rational curve lift. The construction and its exceptional branch are
specified in [`DEC-20261031-cd6460.md`](DEC-20261031-cd6460.md).

One native Rust generator emits a canonical typed polynomial/Boolean IR.
Representation exporters consume that IR. An arm may not regenerate the
mathematical instance independently.

### Fixed ladder

| rungs | arity `m` | scale `s` | approximate field bits | target classes |
|---:|---:|---:|---:|---|
| 1--9 | 3 | every integer 6--14 | 18--42 | natural, planted, exact UNSAT |

There are eight independent held-out instances per target class per rung: 216
instances. `s` is the declared scale and the manifest records the actual
`log2(p)/3` after prime rounding. Seeds are derived from the frozen protocol
hash. Prime/curve/base/window generation must finish before arm labels are
assigned and is independent of the solver arm.

Canonical two-window enumeration is attempted at every rung and is expected to
scale as `2^(2s)` up to polynomial factors. The largest exactly enumerable
prefix is fixed in preflight; later timeouts stay in the export. A continuation
may extend `s=15..27` (approximately 45--81-bit fields) under a separately
frozen amendment, never by changing this first ladder after data.

## Arms

| id | role | representation/backend | purpose |
|---|---|---|---|
| `enum-ref` | reference | canonical exact two-window enumeration | complete model-set oracle at small sizes and `e=2` cost control |
| `f4-base` | baseline | identical polynomial IR to msolve/F4 | established algebraic solver reference |
| `cdcl-t-fp` | candidate | CaDiCaL Boolean skeleton plus external `F_p` theory propagator | primary hybrid hypothesis |
| `qf-ff` | candidate | `QF_FF` to cvc5 | independent finite-field solver route |
| `cnf-cost` | control | complete pure-CNF bit-blast | expose `Theta(log^2 p)` multiplication expansion; not a theorem route |
| `cdcl-t-aa` | A/A | same binary, input, theory and seed as `cdcl-t-fp`, fresh process/layout | runtime/instruction noise floor |
| `planted-positive` | positive | same CDCL(T) path on the planted class | detector, theory-explanation and witness-extraction power |
| `matched-null` | negative | same CDCL(T) path on a support/model-count matched curve-free predicate | distinguish elliptic structure from generic sparse-system behavior |

The candidate is a solver-plus-faithful-theory bundle. CaDiCaL owns Boolean
search; the `F_p` theory owns field arithmetic and must emit a checkable
explanation clause for every propagated literal or conflict. Theory calls,
field operations and explanation checking are charged. The quotient-ring
relations `z^2-z` avoid carries algebraically, but they do not make a pure CNF
field multiplication constant-size.

## Controls

### Exact semantics

At every enumerable cell, compare the complete projected model set after
canonicalization. SAT/UNSAT agreement alone is insufficient. At larger cells,
every SAT witness is checked by the canonical IR evaluator and independent
group law; every claimed UNSAT requires a checked proof object when the backend
supports one. An unproved UNSAT is `unknown`, not `unsat`.

### Positive control

Each planted instance contains a decomposition generated before solver input
serialization. The solver receives only the public curve, target and system.
Every backend must return a valid model whose replay sums to the target; it need
not return the originally planted tuple if another valid model exists. A miss
invalidates that backend/rung's negative reading.

Mutation controls change one field coefficient, admit one forbidden word,
force two windows to share an abscissa, corrupt one theory-explanation clause
and corrupt one extracted root. The canonical evaluator/explanation checker
must reject all five.

### Negative control

For each natural instance, generate a curve-free sparse system with the same:

- field and bit length;
- variable/equation counts and degree multiset;
- monomial-support incidence graph;
- selected chart, one-hot/window shape and field equations;
- exact model count on enumerable cells, or the same planted/UNSAT class at
  larger cells.

Randomize coefficients subject to the planted model where applicable. If the
model-count match cannot be obtained within the preregistered generator budget,
the pair is invalid; do not substitute an easier null after observing solver
behavior.

### A/A

`cdcl-t-aa` uses a fresh process and independently randomized memory layout but
the same Boolean bytes, theory instance and solver seed as `cdcl-t-fp`. Its
retired-instruction and wall-time spread is the noise floor. A second
diagnostic A/A with a protocol-derived variable renaming may be reported, but
it is not the primary A/A because variable order can alter CDCL search.

## Variable ledger

| factor | treatment | rule |
|---|---|---|
| mathematical instance | fixed within pair | one canonical IR hash supplied to every backend |
| solver/representation bundle | crossed | the independent variable |
| rung and target class | crossed | all declared cells, no favorable subset |
| curve, target and null draw | randomized | protocol-hash seeds, independent of arm |
| compiler, wrapper and exporter | fixed | one commit and one binary hash |
| external solver/theory | fixed within arm | solver version, propagator source, binary hash, flags and explanation checker frozen |
| threads | fixed | one thread in all arms |
| arm order | interleaved | randomized within instance/rung round |
| warm-up | fixed | one discarded warm-up per binary/size class |
| host, kernel, frequency, SMT, NUMA, pressure | measured | isolab L2 receipt on every row |
| process layout | randomized | fresh process per replicate |
| spot interruption | measured | no scientific retry; interrupted rows retained; restored clocks invalid |
| analyst | masked | method labels withheld until the frozen analysis completes |

## Metrics

### Primary

1. **Verified status/model set** (`exact`): canonical projected models on exact
   cells; otherwise verified SAT witness, proof-checked UNSAT, unknown or
   timeout.
2. **Complete counted work** (`counted`): retired instructions from the first
   target-dependent IR compilation instruction through solver exit and
   independent verification, including parse/serialization, every theory call
   and explanation check, and proof/model extraction. Theory field operations
   are also reported as a separate exact counter vector. Reusable target-
   independent setup is reported separately.
3. **Peak resident bytes** (`counted`): maximum RSS over the complete interval.
4. **PDP exponent `e`** (`derived`): robust slope of
   `log2(complete counted work)` directly against declared scale `s`, with a
   paired bootstrap interval over instances. Candidate-space size is not the
   normalization axis.

### Mechanism diagnostics

- CDCL(T): decisions, conflicts, Boolean propagations, restarts, learned
  clauses/bytes, theory calls, theory propagations/conflicts, field additions,
  multiplications and inversions, and explanation clauses checked;
- pure CNF: variables, clauses and gates by field operation, including the
  measured `Theta(log^2 p)` multiplication expansion;
- F4: solving degree, rows, columns, nonzeros and multiply-adds;
- QF_FF: backend-native conflicts/lemmas/field reductions where emitted;
- all arms: input variables, constraints, bytes, setup instructions, solve
  instructions, verification instructions and failure class.

Report the Boolean native-work sensitivity band
`W(w)=decisions+conflicts+w*propagations`, `w in {0,0.1,1}`, next to—not in
place of—the complete counted work and theory ledger. A conclusion that exists
only after assigning zero cost to theory work is invalid.

Wall time is secondary, only from L2 rows, and is read against the A/A interval.
No stage count is promoted to a complete ECDLP or rho comparison.

## Sample size and analysis

The current draft specifies 24 instances per rung (eight per target class),
three measured fresh-process replicates and one warm-up. Before freezing:

1. time one complete call per arm and rung on scratch instances;
2. separate fixed from per-instance cost;
3. run the full analysis on scratch output;
4. justify power to distinguish an exponent below `0.5` from the boundary and
   a 1.5x paired instruction ratio under the observed log-ratio variance;
5. retain 216 instances unless power requires more—never reduce power to fit a
   preferred budget.

The draft currently marks power as not established. That is a freeze blocker,
not permission to take pilot data on the target cells. Scratch preflight uses
separate disclosed instances.

## Decision rules

The family has two candidate solver bundles; use Holm correction at family
alpha `0.05`.

### `sat_screen_advanced`

At least one preregistered finite-field solver bundle must satisfy all of:

1. every positive control at every attempted rung verifies;
2. no wrong answer or unproved UNSAT;
3. enough completed natural cells to meet the frozen power calculation, with
   no favorable-subset choice;
4. Holm-corrected upper 95% interval for `e` strictly below `0.5` over the
   preregistered `s=6..14` ladder;
5. the exponent conclusion lies outside A/A spread and is unchanged when all
   theory work and independent verification are included.

### `sat_screen_not_advanced`

Use this verdict only when the corrected lower 95% interval for `e` is at least
`0.5` over the preregistered ladder with valid controls and sufficient power.

### `inconclusive`

All other censoring, failed positive controls, missing perf counters, unmatched
nulls, invalid isolation for clock claims or insufficient power is
`inconclusive`. Wrong answers are reported explicitly and invalidate that arm;
they are not folded into a runtime loss.

The structure comparison (`cdcl-t-fp` versus `matched-null`) is separate. An
elliptic-structure effect requires a corrected paired instruction ratio at most
`0.8` on the three largest completed rungs with an interval excluding 1.
Absence of that effect does not by itself refute a generic SAT-engine
improvement.

## Stopping and budgets

- Fixed cells and fixed instance count; no stopping when a favorable slope
  appears.
- Per-row timeout and memory limits are identical across arms after preflight.
- Infrastructure failures may retry once; solver timeouts and nonzero solver
  exits are outcomes and never retry.
- If a row is restored after interruption, counted units remain valid and wall
  time is invalid.
- The positive-control block runs first. A failed detector stops affected
  negative readings but all attempted rows remain in the export.

## Inadmissible moves

- changing primes, curves, cells, seeds, target class or the null after seeing
  solver output;
- comparing SAT on a selected chart with F4 on an unselected/repeated-root
  chart;
- omitting theory calls, field operations, explanation checking, extraction,
  setup, proof or verification costs from one arm;
- calling pure CNF and a field-theory propagator the same solver model;
- fitting against candidate-space size instead of `s`, or using a non-strict
  threshold at `e=0.5`;
- treating `unknown`, timeout or unproved UNSAT as no solution;
- quoting a candidate-only completion as a speedup;
- replacing one-target evidence with a multi-target average;
- calling a finite slope a proof of subexponential complexity;
- selecting propagation weights after seeing which one wins.

## Required artifacts

- frozen `protocol.json` and generated seed list;
- canonical instance IRs and every representation, all hashed;
- native wrapper and analysis source/hash;
- solver binaries, versions, flags and image digest;
- exact external-propagator source binding, theory-operation ledger and
  independently checked explanation clauses;
- raw row per execution, including failures;
- one `experiment.export/v1` passing `check-export --verify-files`;
- masked analysis output, unmask key hash and flow table;
- independent replay/review record;
- run report, quantitative graphs and inspected PDF.

The formal draft is [protocol.draft.json](protocol.draft.json). It is expected
to remain `draft` until every freeze blocker named above is resolved.
