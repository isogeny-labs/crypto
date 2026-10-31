# Draft protocol: prime-field shared-coefficient SAT discriminator

**Status:** design complete enough for review, **not frozen and not executable
yet**. No affected data may be taken until the native instrument, external
solver versions and hashes, preflight runtime, power justification and analysis
script are filled into `protocol.draft.json`, the record validates, is frozen,
and is committed.

## Question

On one exact prime-field decomposition predicate, does any practical
CDCL/XOR/finite-field solver show lower complete-work growth than exact
enumeration and F4 on unseen natural targets, while returning the same model
status and surviving planted, UNSAT, null and A/A controls?

The experiment tests a solver/representation family. It cannot prove a
subexponential ECDLP algorithm. A failing arm refutes only that frozen
representation, solver version, parameter ladder and budget.

## Semantic instance, fixed before representation

Use the odd-arity divisor-coefficient construction from
`research/prime_one_shot_20261029`:

- nonsingular `E/F_p` with known prime whole-group order for the first panel;
- selected distinct-abscissa chart;
- norm identity and target equation;
- quotient/remainder membership tower;
- `F_p` coefficient/rationality constraints;
- squarefreeness and coprimality witnesses;
- root extraction, sign recovery, curve membership and independent group-law
  replay.

One native Rust generator emits a canonical typed polynomial/Boolean IR.
Representation exporters consume that IR. An arm may not regenerate the
mathematical instance independently.

### Fixed ladder

| rung | arity `m` | membership depth `t` | target classes |
|---:|---:|---:|---|
| 1 | 3 | 3 | natural, planted, exact UNSAT |
| 2 | 5 | 4 | natural, planted, exact UNSAT |
| 3 | 7 | 5 | natural, planted, exact UNSAT |
| 4 | 9 | 6 | natural, planted, exact UNSAT |

There are eight independent held-out instances per target class per rung: 96
instances. Seeds are derived from the frozen protocol hash. Prime/curve/base
generation must finish before arm labels are assigned and is independent of
the solver arm.

The largest exactly enumerable prefix of the ladder is fixed in preflight.
Exact enumeration remains an attempted row at larger rungs and may time out;
those timeouts stay in the export.

## Arms

| id | role | representation/backend | purpose |
|---|---|---|---|
| `enum-ref` | reference | canonical exact enumeration | complete model-set oracle at small sizes and search-space denominator |
| `f4-base` | baseline | identical polynomial IR to msolve/F4 | established algebraic solver reference |
| `cdcl-cnf` | candidate | parsimonious CNF to CaDiCaL | ordinary CDCL hypothesis |
| `xor-cnf` | candidate | CNF-XOR to CryptoMiniSat | preserve parity structure |
| `qf-ff` | candidate | QF_FF to cvc5 | finite-field SMT control |
| `cdcl-aa` | A/A | same binary, input and seed as `cdcl-cnf`, fresh process/layout | runtime/instruction noise floor |
| `planted-positive` | positive | same CaDiCaL path on the planted class | detector and witness-extraction power |
| `matched-null` | negative | same CaDiCaL path on a support/model-count matched curve-free predicate | distinguish elliptic structure from generic sparse-system behavior |

The candidate is a solver-plus-faithful-export bundle. The experiment does not
attribute a difference between `cdcl-cnf` and `xor-cnf` solely to solver code;
that would require a later crossed solver-by-encoding design.

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

Mutation controls change one coefficient, omit one rationality constraint and
corrupt one extracted root. The canonical evaluator must reject all three.

### Negative control

For each natural instance, generate a curve-free sparse system with the same:

- field and bit length;
- variable/equation counts and degree multiset;
- monomial-support incidence graph;
- selected chart and field equations;
- exact model count on enumerable cells, or the same planted/UNSAT class at
  larger cells.

Randomize coefficients subject to the planted model where applicable. If the
model-count match cannot be obtained within the preregistered generator budget,
the pair is invalid; do not substitute an easier null after observing solver
behavior.

### A/A

`cdcl-aa` uses a fresh process and independently randomized memory layout but
the same CNF bytes and solver seed as `cdcl-cnf`. Its retired-instruction and
wall-time spread is the noise floor. A second diagnostic A/A with a
protocol-derived variable renaming may be reported, but it is not the primary
A/A because variable order can alter CDCL search.

## Variable ledger

| factor | treatment | rule |
|---|---|---|
| mathematical instance | fixed within pair | one canonical IR hash supplied to every backend |
| solver/representation bundle | crossed | the independent variable |
| rung and target class | crossed | all declared cells, no favorable subset |
| curve, target and null draw | randomized | protocol-hash seeds, independent of arm |
| compiler, wrapper and exporter | fixed | one commit and one binary hash |
| external solver | fixed within arm | version, binary hash and flags frozen |
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
2. **Retired instructions** (`counted`): from the first target-dependent IR
   compilation instruction through solver exit and independent verification,
   including parse/serialization and proof/model extraction. Reusable
   target-independent setup is reported separately.
3. **Peak resident bytes** (`counted`): maximum RSS over the complete interval.
4. **Growth slope** (`derived`): robust slope of `log2(retired instructions)`
   against `log2(exact candidate-space size)` across completed rungs, with a
   paired bootstrap interval over instances.

### Mechanism diagnostics

- CDCL: decisions, conflicts, propagations, restarts, learned clauses/bytes;
- XOR: XOR propagations/eliminations plus the CDCL counters;
- F4: solving degree, rows, columns, nonzeros and multiply-adds;
- QF_FF: backend-native conflicts/lemmas/field reductions where emitted;
- all arms: input variables, constraints, bytes, setup instructions, solve
  instructions, verification instructions and failure class.

Report SAT work under

`W(w) = decisions + conflicts + w * propagations`, `w in {0, 0.1, 1}`.

This is a sensitivity band, not a conversion to group additions. A conclusion
that reverses within the band is unresolved.

Wall time is secondary, only from L2 rows, and is read against the A/A interval.
No stage count is promoted to a complete ECDLP or rho comparison.

## Sample size and analysis

The current draft specifies 24 instances per rung (eight per target class),
three measured fresh-process replicates and one warm-up. Before freezing:

1. time one complete call per arm and rung on scratch instances;
2. separate fixed from per-instance cost;
3. run the full analysis on scratch output;
4. justify power for a 15% slope reduction and a 1.5x paired instruction ratio
   under the observed log-ratio variance;
5. retain 96 instances unless power requires more—never reduce power to fit a
   preferred budget.

The draft currently marks power as not established. That is a freeze blocker,
not permission to take pilot data on the target cells. Scratch preflight uses
separate disclosed instances.

## Decision rules

The family has three candidate solver bundles; use Holm correction at family
alpha `0.05`.

### `sat_screen_advanced`

At least one preregistered SAT bundle must satisfy all of:

1. every positive control at every attempted rung verifies;
2. no wrong answer or unproved UNSAT;
3. completion on all four rungs for the natural class;
4. upper 95% interval for its growth slope at most `0.85` across the three
   largest rungs;
5. complete retired instructions below F4 on both largest jointly completed
   rungs, with corrected 95% paired intervals below ratio 1 and differences
   outside the A/A interval.

### `sat_screen_not_advanced`

Use this verdict only if either:

- the lower 95% slope interval is at least `0.95` on three consecutive rungs;
  or
- every SAT bundle is censored before F4 on both largest jointly attempted
  rungs under identical limits.

### `inconclusive`

All other censoring, failed positive controls, missing perf counters, unmatched
nulls, invalid isolation for clock claims or insufficient power is
`inconclusive`. Wrong answers are reported explicitly and invalidate that arm;
they are not folded into a runtime loss.

The structure comparison (`cdcl-cnf` versus `matched-null`) is separate. An
elliptic-structure effect requires a corrected paired instruction ratio at most
`0.8` on the three largest rungs with an interval excluding 1. Absence of that
effect does not by itself refute a generic SAT-engine improvement.

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
- omitting rationality, extraction, setup, proof or verification costs from one
  arm;
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
- raw row per execution, including failures;
- one `experiment.export/v1` passing `check-export --verify-files`;
- masked analysis output, unmask key hash and flow table;
- independent replay/review record;
- run report, quantitative graphs and inspected PDF.

The formal draft is [protocol.draft.json](protocol.draft.json). It is expected
to remain `draft` until every freeze blocker named above is resolved.
