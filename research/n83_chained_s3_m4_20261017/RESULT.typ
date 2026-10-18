#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 10pt)
#set heading(numbering: "1.")
= Four-summand chained-S3 entry point for retained factor bases

*Status:* Native solver integration and small-degree correctness validation.
Degree-83 natural relation yield, useful rank, and runtime remain unmeasured.
The pre-merge release suite passed, but inherited ecbench and quartic compile
errors block validation after merging the current PR #2100 head. The merged
head remains unverified pending a base fix and rerun.

The full-width chained-S3 circuit supports four summands, but the index-calculus
decomposition and dispatch guards previously admitted only five or six. This
change enables m = 4 while retaining finite-domain preflight, solver caps,
original-field checks, group lifting, and relation-rank checks.

#image("diagram.svg", width: 100%)

== Checked properties

Release library tests exercise four-, five-, and six-summand witnesses on
degree-7 known-answer fixtures. They replay sums in the original group and
check signed-Frobenius and ordered-domain modes, incomplete-orbit rejection,
exact rank, answer verification, and cap handling. Exact commands and statuses
are in `VALIDATION.md`.

The degree-83 m = 4 construction preflight gives 63,578 maximum circuit
variables. The 2,001-representative orbit domain adds 21,012 variables,
697,808 clauses, and 2,656 XOR rows. These are derived bounds, not measured
solve resources.

== Remaining work

The retained-base adapter must authenticate both the K = 2,048 explicit base
and the 2,001-column structured base and run a common bounded m = 4 query
corpus. Independent group replay, exact rank, complete cold costs, and matched
references are required before comparing quality. The existing N83
prebuilt-base problem record remains open.

Source: `src/cryptanalysis/koblitz_index_calculus.rs`,
`src/cryptanalysis/binary_semaev_chain_sat.rs`, and
`research/n83_prebuilt_factor_base_20261016/REVIEW.md`.
