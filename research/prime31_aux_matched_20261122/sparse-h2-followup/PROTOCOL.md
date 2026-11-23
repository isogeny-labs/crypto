# Sparse H2 and fair staged Prime31 comparison

**Prospective protocol amendment. No experiment has been launched.**

Tracking: [PR #4937](https://github.com/isogeny-labs/crypto/pull/4937), following
merged [PR #4923](https://github.com/isogeny-labs/crypto/pull/4923). This directory
adds v2 semantics. The parent 55-cell and isogeny-choice 230-cell panels, their
receipts, source snapshots and protocol text remain historical evidence.

## Requirements and evidence

| Requirement | Implementation / evidence | Admission still required |
|---|---|---|
| Same H2 fiber, sparse first quartic | `native/src/kernel_fibers.rs`, `sparse_h2_contract.rs` | Native qualification receipt; measured solving cost |
| Fair plain algorithm setup | `plain_reference.rs`, independent `audit.rs`, ACCOUNTING.md | Actual process and study-overhead receipts |
| Screening distinct from recovery | v2 cell stage and parser; `staged_panel_contract.rs` | Canonical freeze and explicit stage selection |
| Paired varied streams | Five fixed seeds on the same public target | Selected candidate execution; no independent-target inference |
| Applicable IC and rho controls | `pair_control.rs`, BASELINE_REVIEW.md, three retained rho implementations | Matched admitted execution and complete cost coverage |
| Reproducible source and replay | Source inventory, copied qualifiers, independent arithmetic audit | Source/image binding and signed validation where applicable |

## Fixed problem and controls

Use exactly the original public problem: field p=2147483647, curve
`y²=x³+847514846x+2087444724`, prime order n=2147446831,
G=(625037715,2100627251), Q=(984837385,1142774582), and arity three. The target
scalar is never an input to any collector. The original/native-isogenous/
transported-isogenous target arms retain their previous definitions and map.

The four auxiliary kernels and full 1024-element fibers retain the fixed coset
scalar one. The frozen fixture has usable columns 518/506/518 for Aplus,
508/508/508 for H0, 512/519/512 for H1, and 515/533/515 for H2, in
original/native/transported order. These are retained correctness inputs,
not newly measured yield. New cosets require a separate admitted reference
preparation and independently checked fixture; v2 plain bindings currently
reject every coset scalar other than one.

Keep the source-pinned historical counted negation reference, counted v6, and
tuned one-thread prime-fast rho. Add an explicit same-base pair-table backend
for auxiliary and plain controls. Its maintained negation-folded batched table
and group arithmetic are reused from the pinned production source. The adapter
specifies a cancellation-first witness policy and batched complement traversal;
it is identified as such rather than relabeled as an unchanged complete solver.
The larger cross-repository baselines and their applicability limits are in
BASELINE_REVIEW.md. No global fastest-method claim follows from this finite set.

## Sparse H2 factorization

On Aminus: `y²=x³−x`, let H2=<2K,U>, where K has order1024 and U=(0,0) is
independent of its order-two point. H2 contains all four rational two-torsion
points. Quotient those first through the normalized maps x−1/x and u+4/u:

\[
\delta_x(x)=\frac{(x^2+1)^2}{x(x^2-1)},\qquad
x^4-zx^3+2x^2+zx+1=0.
\]

The residual generator is delta(2K), of order256. Eight further degree-two
quotients give total degree4·256=1024. The existing quadratic and quartic
representations consume these ten steps without a new polynomial solver.
The new factorization must equal the old complete kernel, V, numerator,
denominator, fiber polynomial, final curve and endpoint. Only intermediate
equations and construction work may differ. See MATHEMATICS.md and the fixed
identity certificate; the latter is evaluated with proof=null, not a Lean proof.

For three summands, finite-chart variable counts remain31 quadratic,16 quartic
and4 dense. A five-term input block does not establish lower solving degree,
fewer F4 operations, more relations or shorter recovery time. Those are questions
for the admitted comparison.

## Stage definitions and prospective inventories

`prime31-isogeny-choices staged-panel ABSOLUTE_NEW_DIRECTORY` creates inputs
only. The executable validates every generated cell. It creates:

| Pool | Streams | Prepared inputs | Natural-query ceiling | Status |
|---|---:|---:|---:|---|
| Screening | seed4188 |77|64|Unadmitted pilot |
| Complete-recovery candidates | Five fixed seeds |385|65536|Unselected prospective pool |

Each stream contains35 auxiliary polynomial cells (28 old routes plus7 sparse
H2 routes),12 same-base auxiliary pair-table cells,12 plain polynomial cells,
12 plain pair-table cells and6 rho controls. Sparse and standard dense H2 cells
retain separate setup paths even though their expanded polynomial is equal.
Pair-table controls use the standard auxiliary construction; the report must
show its setup separately from its query phase.

Seeds are4188,104729,130363,169087,224737. Every configuration within one
stream receives the same seed, including failed or negative queries. These
are different search streams for **one fixed public target**, not five targets.
Do not use these streams as timing repeats or independent target-success data.
Timing repeats and A/A controls require their own prospective admission.

Order: generate71 IC configurations in kernel/target/route/presentation order,
insert one successive rho after each11 IC cells, then rotate the77-cell sequence
left by9·stream modulo77 and reverse odd streams. The exact ordered cell hashes
are retained in each index. Every pool starts at attempted=0,completed=0 and
selected_for_execution=0. No receipt may be inferred from input generation.

The pilot measures setup, complete/unknown query outcomes, first-hit rows, rank
growth and censoring. A replayed scalar is accepted if it appears early. It is
not required to wait for full factor-base rank. Recovery attempts use the same
correct stopping condition and a larger logical ceiling, with the same physical
resource envelope. The ceiling65536 is finite; it is not an adequacy guarantee.

## Selection after the pilot

Before pilot execution, freeze its explicit admitted subset and all actual
budget, deadline and source/image receipts. Preserve the entire requested pool
as open if it cannot fit the inherited budget; do not silently replace it.

After pilot receipts exist, freeze a concrete recovery selection before any
recovery cell. Retain the standard/sparse H2 representation pairs as the primary
question, their same-base pair-table and plain controls, and all three matched
rho controls for each included destination. Additional auxiliary configurations
may be selected by replayed pilot row count, then completed-query count, then
the fixed index order. That ordering is a search-allocation heuristic, not a
performance ranking. Record all rejected/unselected configurations and why;
unknown, capped and failed pilot outcomes remain in the report.

No shortlist is recorded in this PR because the pilot has not run. The new
`run-staged.sh` requires a published admitted selection that names each exact
cell/hash, canonical protocol hash, original absolute deadline and, for the
recovery stage, a pilot receipt hash. It then delegates to the existing wrapper
with its publication, checkpoint, budget, source and process limits intact.
Changing a status string does not supply the required external admission.

## Unchanged physical bounds

One cell at a time, one affinity-bound logical CPU, one computational thread,
4096MiB, zero retries. The hard outer interval is120seconds (TERM at115, then
five seconds grace). Collection is at most110seconds and shortened by remaining
cold time. Each query has at most2000milliseconds shared across its charts or
pair probes. Pair-table setup is bounded by the outer envelope, not the query
deadline; a killed build can leave its internal partial counters unavailable.

Polynomial caps remain degree1024,32 pairs per step,4096 solutions,1000000
signed-lift attempts,48 variables,100000 input terms, Auto domain roots and
sibling batching. Both stages retain first_hit_per_query=true. The generated
rho inputs freeze 2000000 steps and pair each rho seed with its query-stream
seed; the published input hashes and admitted selection bind those exact
values. The general driver parser permits a positive rho ceiling up to
10000000 steps, while the mathematical auditor checks the declared public
target and recovered scalar without independently verifying rho seed or walk
accounting. These general interfaces do not authorize substitution into this
registered panel. The v2 parser and auditor enforce the stage, first-hit and
polynomial limits above; the existing v1 parser keeps its historical contract.

The inherited aggregate lane ceiling remains USD5, including previous builds,
failed/interrupted attempts and idle infrastructure. Current remaining headroom
is unknown and therefore blocks spending. Do not launch another account/pool,
raise quota, relabel retries or fabricate a deadline/budget receipt. The
prepared385-cell recovery pool is not a commitment to execute all its cells.

## Accounting and acceptance

See ACCOUNTING.md for exact timer boundaries. Plain controls consume a frozen
public column count, then perform their own target mapping/validation,
construction, indexing, optional pair-table setup, queries and recovery.
Independent audit reconstructs the auxiliary reference outside the algorithm.
Historical reference-preparation cost is unavailable; preserve that fact as
null rather than pricing it at zero. The study's total expenditure still
includes reference preparation and all controls.

Record algorithm setup/online intervals, outer wall/CPU/RSS where the harness
actually observes them, scoped group/field counters, table native work, F4
work, replay and linear algebra. Price counted field operations under the
current `bitop/v1` rules and report the required0/0.1/1 GAE band for other native
work. Do not double-price inversion powers or sum overlapping ledgers. Current
counters are partial; they do not supply a complete common operation total.

Complete recovery requires dG=Q on the original target and, when transported,
on the destination too, plus independent replay of natural query coefficients,
signed witnesses, rows and recovery. An accepted row or rank increase alone
does not meet that criterion. All first-hit policies may follow different
valid row trajectories; compare recovery/cost outcomes without demanding the
same first witness across unrelated oracles.

Retain every input, source/image/binary identity, stdout/stderr, exit status,
deadline/OOM/setup failure, incomplete stream and raw accounting field. A missing
counter is unavailable, not zero. External L2 and prospective warmup/A/A admission
are required before timing ratios; diagnostic clocks cannot certify themselves.

## Development and current execution dependency

Focused native tests, fixed identity issuance, parser checks and panel generation
are development work. They produce no Prime31 natural-query campaign or comparative
solving-cost result. Retain copied immutable binaries and distinct
build/pre-call/post-call hashes. The source closure preserves the earlier pinned
solver revision33b3bc3c7485df0fe80aae41e2093cf940d87ff3; two further exact snapshots
avoid changing that implementation as main advances. The identity issuer is
separately inventoried from the current source.

The fresh checkpoint gate reported no upload token or AWS profile. This session
has source-publication access through the GitHub connector, but no authenticated
IsoLab/Actions submission, live budget/capacity receipt or readable completed
canonical protocol freeze. The failed gate is retained in evidence. No local
benchmark substitutes for the requested admitted execution.
