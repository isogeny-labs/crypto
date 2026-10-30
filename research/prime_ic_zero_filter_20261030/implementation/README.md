# C1–C3 structured predicate implementation reference

2026-10-30. This additive follow-up implements bounded native references for the
three [proposals](../README.md). Their general research goals remain **partial**.
It establishes software behavior on explicit fixtures, not a novel theorem,
fast algebraic solver, useful relation-rank gain or ECDLP speedup.

| Requested direction | Implemented and checked | Remaining gap |
|---|---|---|
| C1: exact structured relation predicates | Rational two-isogeny fibre circuit, actual subgroup constraint and complete point-addition charts inside one Boolean network | General PKM auxiliary-isogeny chains and a demonstrated advantageous contraction |
| C2: count-guided extraction | Exact counts and interface-bit self-reduction; every witness rechecked for membership and sum | The current functional leaf still enumerates inputs; grid-free arithmetic recovery is not a bound on total solver work |
| C3: reusable target messages | Shared reduced decision diagram for local membership; immutable arithmetic network with a pinned public target boundary | Compact contracted message for the complete summation predicate and verified cold-total benefit |

The API is [`structured.rs`](../../../crates/hashed-ecdl-tensor/src/structured.rs).
`Prepared::new` constructs one network; `count` and `witness` accept public target
points. No scalar labels, logs, precomputed relation sums or successful-target
counts enter the network. `MembershipMessage` is shared with `Arc` across slots,
networks and queries. Existing hash, selector, archive and relation APIs retain
their behavior. The existing correctness workflow already checks the crate.

## Exact accepted language and exceptional charts

The special family has a rational two-torsion point (t,0). Write u=x−t,
A=3t and B=3t²+a. The local predicate selects the finite nonkernel fibres of
the usual quotient abscissa u+A+B/u at a supplied set of distinct rational
image abscissae. Its circuit tests the product of the quadratic fibre equations,
u≠0, rational canonical coordinates, the curve equation and [r]P=O. It performs
no cofactor projection. This is a standard rational-two-torsion quotient family,
not a construction for every prime-order curve or arbitrary PKM auxiliary map.
No new polynomial identity or general map theorem is claimed.

Relations are **ordered signed tuples, with repetitions and inverse pairs
allowed**. They are deliberately different from the archive oracle's sorted,
inverse-pair-free multisets. Counts cannot be interchanged with those receipts
or converted to useful matrix rank. Complete existing point-addition charts
retain identity, inverse and doubling cases, including identity intermediate
sums. Kernel points are explicitly outside this particular factor base; they
are not lost through an unchecked saturation. Target identity is included as a
software chart control, not an independent-target DLP workload.

`witness` explicitly uses `Counter::witness_by_counts`, then replays rational
membership and the group sum. A missing witness means an exact zero count only
when the counter completed. Precision, work, storage and invalid-input errors
propagate. Replay uses native group arithmetic separate from the contraction
algorithm, but it is not a second independent arithmetic implementation.

## Reusable local messages: two constructors

`MembershipMessage::compile` is an exhaustive finite control. It visits every
coordinate bit encoding and reduces equal decision subgraphs. Its setup grows
with the coordinate space and is not a proposed scalable compiler.

`MembershipMessage::from_fibres` instead solves each quadratic fibre, extracts
both rational ordinate signs, filters actual subgroup membership, deduplicates
points and builds the same Boolean membership function as a reduced decision
diagram. Root extraction uses the existing checked square-root context and an
explicit caller random source. Setup, candidate checks, diagram construction,
per-query traversal and vector sizes have explicit meters. Duplicate zero roots
can be checked twice before deduplication, and remain in the candidate counter.
The compiler enumerates membership points, not all field encodings or tuples.
No universal compression, contraction-width or sub-rho guarantee follows.

This is an engineering control for **local membership reuse**. It does not
implement the proposed compact target-dependent *full summation message*.
Existing coefficient specialization, exact self-reduction, decision diagrams,
quadratic-root solving and point-set membership are prior techniques. Global
novelty of a prime-field IC combination is still unestablished.

## Finite controls and resource accounting

The primary fixture is y²=x³+1 over F5, full cardinality6, subgroup order3,
cofactor2 and generator(0,1), with kernel abscissa4 and image set{1}.
The accepted base is {(0,1),(0,4)}. For all three subgroup targets, the ordered
two-point counts are 2,1,1 and the three-point counts are 2,3,3, in the order
identity, generator, negative generator. Direct brute-force addition agrees
with both network variants and all returned prefix witnesses.

The second fixture is y²=x³+x over F19, full cardinality20, subgroup order5,
cofactor4 and generator(5,4), kernel abscissa0 and image set{7,9}. All signed
base-pair/target predicates are checked using the network witness program.
Both fields also compare root-based and exhaustive messages with direct
membership for **every single rational image abscissa**, including empty
fibres, branch points and noncanonical inputs. Independent inverse-based
evaluation checks the map on rational points. The second-field control is
predicate evaluation, not a larger-field contraction or scaling experiment.

The primary root-based message checks four signed lift candidates and retains
eight nonterminal nodes. The exact constructed graph edge counts are:

| Arity | Arithmetic membership circuit | Shared local membership diagram |
|---|---:|---:|
| 2 | 108288 | 31828 |
| 3 | 161286 | 46596 |

See [structure.svg](structure.svg). These are finite representation sizes,
not time, calibrated group costs or an end-to-end gain. Diagnostic step counts
in raw test output exclude shared message/network construction from each query
and include reference/replay work; they are **not** a cold IC cost model.
Setup meters are separate. No benchmark or performance ledger is updated.

The successful aggregate test uses a fresh 500-million-step envelope per
count-plus-witness correctness query. The production default remains100million.
Earlier checks hit both a shared100million envelope and a per-query100million
envelope on the three-point reference. Those genuine resource errors are kept
in `checks/attempt*.txt.gz`, alongside the initial integer-type compile error.
They are neither zero relations nor a mathematical obstruction. A second
development compile error (ambiguous shift type in the new root constructor)
was fixed before final checks. No runtime/resource guard was weakened.

## Reproduce and interpret

```sh
cargo test --release --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml -- --nocapture
cargo clippy --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --all-targets -- -D warnings
cargo +1.78.0 check --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --all-targets
cargo fmt --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --check
```

Raw final outputs and exact source digests are retained in `checks/` and
`sources.sha256`. These are local development controls on a shared x86 Linux
container, not an isolated performance experiment. No cloud job, timed research
panel, unknown-target solve, independent-host replay or Lean/source-proof audit
was run. The earlier root-library test at a different revision is not evidence
for this module. No new general mathematical result is asserted or formalized.

The [successful-work obstruction](../../../problems/factor-base-shape-to-successful-work.md)
and [general tensor audit](../../../problems/hashed-ecdl-tensor-backend.md)
remain open. The next implementation obligation is a bounded exact full
summation-boundary message with all chart branches, compared to this reference;
setup, precision, storage, failed queries and useful relation rank must be
retained before any scaling or cold-total claim. Existing canonical performance,
frontier and leaderboard graphs have no new qualified measurements.

## Sources and access limits

Reviewed the existing proposal and source snapshot, native PKM preparation/grid
recovery, tensor encoders/counter, prime-IC program, isogeny-fibre report and
Dickson/torus negative study. The sibling lead atlas matches existing
algebraic-membership, shared-coefficient and preprocessing leads; these methods
are not claimed newly discovered. Primary context:

- [PKM2016, §§3.1–3.4](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf): structured membership, not a generic transfer theorem.
- [Hexagon2610.00594v1](https://hexagonmath.org/2610.00594v1): weighted counting, whose coefficient-algebra hypotheses are not inherited by these Boolean development checks.
- [Hexagon2610.00600v1](https://hexagonmath.org/2610.00600v1): exact prefix-count self-reduction.
- [Hexagon2610.00613v1](https://hexagonmath.org/2610.00613v1): locality, encoded description and precision qualifications.
- [Kosters–Yeo1503.08001v3](https://arxiv.org/html/1503.08001v3): algebraic summation constraints do not automatically give rational relations.

The cached primary texts and previously pinned reviews were used. Fresh web
tool requests returned no content; a supplemental Stanford isogeny lecture page
returned503. The earlier inaccessible2018 prime-field paper remains unreviewed.
No missing source or search failure establishes novelty. Conductor is absent
(exit127); the documented warn-and-allow fallback applies and no reservation is
claimed. Repository report-evidence and research-visuals skills govern this
report and its inspected PDF.
