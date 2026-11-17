#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false)
= ECDLP corpus for machine-assisted algebra
Design and implementation scope, 27 October 2026 (client date).

This report describes a proposed dataset and its evidence contract. It reports
no experiment result, speed comparison, solved campaign or mathematical verdict.
The primary design is `docs/ml-dataset/DESIGN.md`, committed before implementation
at `a5062b8ce4f`. The pilot identifier is
`ML-ECDLP-DATASET-PILOT-20261027`.

#image("design-flow.svg", width: 100%)

The generator crosses Koblitz, ordinary binary and prime curves with paired
twists, arities 2 through 15, field sizes 23 through 31 and 47 through 61, and
multiple generators and target keys. At prime n=61 it uses the Mersenne prime
2305843009213693951. The default plan derives 165,150,720 method cells from
24 × 14 × 3 × 2 × 16 × 8 × 64 × 10. These are planned cells, not completed
solutions. Capability and failure records must preserve every requested axis.

== What the network can learn
Useful supervised targets include decomposition existence under an exact
factor base; which candidate relation will add rank; which residual pair closes
a large-prime cycle; which polynomial row or branch will simplify the current
system; and which SAT decision or restart policy reduces independently counted
remaining work. Sequence models can consume solver prefixes; graph models can
consume variable, monomial, clause and relation incidence graphs.

Representation learning can test whether invariant features transfer across
generators, twists, basis choices and field sizes. Predictions should be checked
against sign and Frobenius actions explicitly. Runtime and solve-probability
models must retain timeouts, unsupported cases and capped searches as censored
outcomes. A selected successful subset would teach the wrong distribution.

#pagebreak()
== Labels and mathematical boundaries
A constructed scalar d with Q = dG supplies a known answer. It does not show
that an algorithm recovered d from G and Q. A verified PDP witness shows that
a specified signed factor-base sum equals its target. It does not alone solve
the ECDLP. An independent recovery label requires an algorithm that never reads
the construction witness, plus separate scalar-multiplication replay.

First-fall and last-fall degrees are properties of stated polynomial systems
and degree-closure spaces. A solver's maximum degree or first observed drop is
a separate measured quantity. Models can suggest a low-degree identity,
syzygy or counterexample. A checker must establish its algebraic hypotheses,
membership and ranks before it becomes a proof or disproof. Finite examples do
not establish a universal degree bound.

== Preservation and cost measurement
Each cell binds its source commit, executable, protocol, plan, inputs, output
artifacts and resource limits. S3 publication requires a byte-for-byte download
and SHA-256 check, including the completion receipt. Interrupted attempts are
preserved. Disjoint shards and external Job reconciliation prevent duplicate
campaign ownership; local process locks alone cannot do that across hosts.

Typed native event counts, hardware instructions, cycles, CPU time, memory,
artifact bytes and diagnostic wall time remain separate. Hardware counters
retain raw and scaled readings, time enabled/running, inheritance and errors.
Unavailable or interrupted readings remain null. Private native solver states
that have no event hook are explicit trace gaps. Saving emitted artifacts is
not a claim that every internal machine state has been observed.

== Training split and validation
Keep scalars, seeds, future pivots, recovered logs and future learned clauses
out of input features. Partition equivalent presentations, signs, Frobenius
orbits, related generators and paired twists together. Start conservatively
with field-size components, then merge components linked by embeddings or
other cross-size constructions. Compare learned policies with frozen classical
baselines, ablations and label-permutation controls under a separate approved
training and evaluation protocol.

== Sources and remaining work
The technical map is grounded in the repository's native Koblitz index-calculus,
chained S3 SAT, matrix F4/F5, geometric F6, root, PKM and large-prime modules.
Definitions and caveats for closure-based degree arguments were read at
#link("https://arxiv.org/html/1505.02532")[arXiv:1505.02532].
PKM bibliographic context was checked against the University of Birmingham
record for #link("https://research.birmingham.ac.uk/en/publications/algebraic-approaches-for-the-elliptic-curve-discrete-logarithm-pr/")[Algebraic approaches for the elliptic curve discrete logarithm problem].
The full publisher paper was not retrieved.

The campaign must still establish verified native CLI behavior, bounded cloud
execution, complete requested backend coverage, streaming trace completeness,
consumer/export compatibility and durable generation waves. Those acceptance
items remain distinct from this design record and from a successful pilot.
