# Candidate links toward novel prime-field index calculus

**Additive implementation follow-up:** the [C1–C3 bounded reference](implementation/README.md)
now provides a structured predicate, exact prefix witnesses and reusable local
membership diagrams. The proposal snapshot below and its original PDF remain
historical; full summation messages, general novelty and an IC improvement are
still unestablished.

This is a proposal and finite-control record, not a new IC algorithm, theorem,
speedup, unknown-scalar recovery or scientific-state promotion. Three candidate
combinations are specified below. Their global novelty is **not established**.
The earlier final-slot archive oracle is an existing generic technique; it is
not the proposed algebraic advance here.

## The useful connection: polynomial values versus exact zero support

PKM supplies small-degree factor-base membership chains. Semaev supplies a
summation constraint. Weighted counting supplies a possible exact contraction
and witness-extraction route, subject to its coefficient algebra, locality,
encoded size and precision assumptions. The missing bridge is an efficient
representation of the *exact rational, subgroup-valid zero predicate*.

A low-degree polynomial-value matrix is not that predicate matrix. The new
native controls distinguish them using exact modular elimination:

- For four fixed prime/domain fixtures, the value matrix x−y has rank 2,
  while its entrywise exact-zero matrix has full rank in the tested domain.
- For y²=x³+2x+3 over F17, the 11 rational abscissae give S3 value rank 3,
  but the tested zero-filter slices have rank 10 or 11.
- Restricting to the **order-11 subgroup**, independently enumerated by [11]P=O,
  leaves 10 finite signed points and five abscissae {3,8,12,14,15}. Every tested
  target abscissa gives value rank 3 and zero-filter rank 5 over F17.

These are exact finite software controls, not a general rank-growth theorem.
Neither matrix rank is the useful relation-row rank over the scalar field F11.
A full-rank slice obstructs a low-rank linear factorization of that slice; it
neither rules out compact functional predicates nor refutes general counting
algorithms. The degree/certificate warnings in the reviewed Hexagon notes are
related cautions, not the source of this separate rank control.

A public [S3 rank repository](https://github.com/Anvexis/S3-Low-Rank-Discovery)
already reports low ranks of polynomial-value matrices. Its README blob
`5c06af4be6e1bec5d38ae0433b4beab2d8b6d939` was read; its timings were not
reproduced or adopted. Low value rank is therefore not claimed novel here.
The quadratic dependence of S3 on either input already explains a rank-at-most-
three value slice by the usual polynomial evaluation basis, without requiring
j=0. Fast evaluation of a chosen pair does not by itself locate all zero pairs.

## C1 — PKM membership chains inside an exact predicate counter

**Proposed combination:** replace the F4 decision/grid-recovery stage by a
counter/witness engine on the actual PKM membership-and-summation network.
Use an auxiliary-isogeny fibre as the primary candidate; Kummer and Dickson
are controls. Keep exact finite-field arithmetic, point lifts, signs and
subgroup checks inside the accepted relation semantics.

**Links:** [PKM 2016, §§3.1–3.4](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
[weighted counting](https://hexagonmath.org/2610.00594v1), and
[locality qualifications](https://hexagonmath.org/2610.00613v1).
The existing implementation already has separate PKM F4 and exact tensor
engines; integrating structured membership into the latter is the candidate.
See `src/cryptanalysis/ic_framework/pkm_semaev.rs`,
`crates/hashed-ecdl-tensor/src/encoding.rs` and `width.rs`.

**What would be new:** a demonstrated encoding/contraction advantage for the
exact constrained PDP, rather than another low-degree presentation or a generic
last-slot lookup. No such advantage is implemented or established by this note.

**Decisive control:** compare the complete Boolean acceptance predicate, its
actual graph/cutsets and zero-support slices against exhaustive small-field
relations. Record encoded bits/edges, factor scopes, coefficient precision,
intermediate storage and all construction costs. A word variable over Fp costs
log₂p bits; low polynomial degree does not make it a constant-size Boolean node.
The source weighted theorem uses a Q-algebra; Fp arithmetic is not automatically
an admissible coefficient algebra. Boolean predicates with rational weights
still require an encoding and bit-cost audit. Finite-field zero tests such as
1−F^(p−1), or additive-character decompositions, can introduce large degree,
many modes or extension coefficients; their cost must be included. Approximate
cancellation is not an exact zero test.

**Existing obstruction retained:** the repo's Dickson/torus tower study records
the same leading-form obstruction as its split controls. Changing p−1 to p+1
alone is not a proposed cure. The current source-compatible tensor engine's
finite-size fallback and independent complexity audit also remain open.

## C2 — Rational-chart-aware prefix witness extraction

**Proposed combination:** attach count-guided self-reduction to C1, operating
on the complete rational PDP rather than recovering an algebraic-closure
solution by scanning the entire factor-base grid. Chart-aware rational maps,
point lifts and subgroup restrictions form part of each accepted assignment.

**Links:** prefix self-reduction in
[the factoring note](https://hexagonmath.org/2610.00600v1), the PKM systems,
and [Kosters–Yeo's distinction between algebraic summation constraints and
rational relations](https://arxiv.org/html/1503.08001v3).
The repo's existing `pkm_semaev.rs` explicitly documents grid recovery after
F4 consistency over the algebraic closure. General zero-dimensional extraction
and count-guided search are prior techniques; this is a proposed combination.

**What would be new:** a bounded, complete witness path for these structured
prime-field systems that avoids the full grid and preserves the IC acceptance
policy. It remains unimplemented.

**Decisive control:** compare all accepted small-field tuples with direct group
addition, including repeated points and intermediate identity/pole cases.
Saturation by a denominator removes its zero chart; it is only complete when
all omitted charts are explicitly handled. Require rational coordinates and
actual subgroup membership. A cofactor projection must not silently move an
output away from its advertised PKM abscissa set. Keep count multiplicities
explicit; exhausted precision/work is unknown, not zero. Recover and replay
signed points before recording a labelled relation.

## C3 — Target-parametric message reuse within one IC solve

**Proposed combination:** cache only target-independent membership messages,
and represent the boundary dependence on the public query point compactly.
Evaluate those messages across the many random uG+vQ queries of one IC solve.
Weights may track witness support or bounded elimination-cost objectives.

**Links:** the weighted/locality sources above and the existing symbolic
`prepare` stage in `pkm_semaev.rs`. That stage already specializes coefficients
per target, so caching symbolic coefficients alone is prior work.

**What would be new:** exact compressed contraction messages that remain small
through the zero predicate and reduce cold total work across actual queries.
No such representation or benefit has been demonstrated here.

**Decisive control:** record symbolic degree, coefficient bits, live storage,
cache construction and per-query specialization/witness cost. Compare all
small-field targets with exact independent enumeration. Reject a representation
that is merely a table of all p target values, all base tuples, or known logs.
Charge cache setup once in the cold single-DLP total; do not claim free reuse
across unrelated DLPs. Measure target-bearing useful rank, not only hit count.

## Required cost hurdle, not a claimed complexity result

Let r be the prime subgroup order and B the factor-base unknown count after
known sign identifications. As a *heuristic screening model only*, suppose the
finite-target decomposition probability scales as q≈B^m/(m!r) in the unsaturated
regime, and let η represent the useful-rank fraction. Neither heuristic is
proved for the proposed bases. Rank loss, overlap or bias can invalidate it.
A planning expression is

    T_cold ≈ S_aux + S_messages + O~(B) + [B/(q η)] T_dec + O~(B²) + T_verify.

T_dec must include failed decisions and witness recovery, not just successful
contractions. The quadratic term is a stated sparse-linear-algebra planning
assumption, not a measured solver bound. Every setup term still needs admission.
Writing B=r^β, T_dec=r^α and η=r^(−δ), the relation-work exponent in this model
is 1−(m−1)β+α+δ, while the stated linear-algebra term has exponent 2β.
Both, and setup, must lie below 1/2 to meet a strict rho-exponent target.

For the **hypothetical** choice m=4 and B=r^(1/5), this demands
α+δ<1/10, equivalently T_dec/η=o(r^(1/10)), in addition to acceptable setup
and verification. This is a hurdle a future solver must meet, not an achieved
rate or forecast. The native arithmetic control checks the equality boundary,
usefulness penalty and an oversized-base linear-algebra failure case. A model
passing this screen is not evidence of an ECDLP improvement.

## Search scope and evidence boundaries

Reviewed: the prior 22-paper snapshot and cached primary texts; the sources
linked above; `research/prime_ic_program_20261029/README.md`;
`research/pkm_dickson_torus_20261015/README.md`; factor-base encyclopedia;
existing PKM, hashed tensor and partial-relation code/audit notes. Live searches
combined prime-field IC, tensor rank, weighted counting, character sums,
saturation and summation polynomials. The public rank README was read as prior
art, not validated cryptanalytic evidence. The Oxford page for the 2018
prime-field-DLP paper returned HTTP 403; its full text was not reviewed, and
novelty relative to that work is unresolved. No new Lean proof was checked.
Search absence is not a novelty proof. The conditional hardness result in
Kosters–Yeo concerns varying summation arity under its assumptions; it does not
establish hardness of this fixed-m structured prime-field proposal. Its binary
trace/first-fall results are not transferred to Fp.

`proposals.json` records source links, obligations and failure tests. The native
controls are in `tools/complexity-catalog/src/prime_ic.rs`. All nine catalog/
control tests, strict Clippy and formatting pass; raw outputs are in `checks/`.
The finite controls and planning arithmetic are development checks, not a
performance experiment or general theorem. No changes are made to prior
findings, frozen runs, source-proof statuses or frontier/leaderboard graphs.
The previous generic oracle PR is separate. These proposed solver combinations,
all larger-parameter studies and any independent-target IC campaign remain open.
