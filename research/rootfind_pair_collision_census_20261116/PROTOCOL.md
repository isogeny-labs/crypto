# Negation-folded pair-sum collision census

Frozen 2026-11-16 before implementation or execution. This is a bounded
mechanism screen, not a timing comparison or a target-log experiment.

## Question and exact workload

The scalar `PairTable::build_negation_folded` retains the first pair for each
nonzero sum abscissa. Can later pairs with that same abscissa supply independent
homogeneous relation rows after the signed Frobenius column fold?

The three fixed public curve/base cells are `K_0` over `GF(2^13)` with standard
subspace dimension 5, `K_0/GF(2^19)` with dimension 7, and
`K_0/GF(2^23)` with dimension 8. They use the exact
`build_standard_subspace_factor_base` and `koblitz_factor_base` constructors,
`ColumnFold::SignedFrobeniusOrbit`, and the curve constructor's largest-prime
subgroup order `r`. The executable records the complete `ICV1` curve ID,
modulus, group order, `r`, cofactor, enumerated point/base digest, column count,
and exact pair count. No other cell may be substituted into this record.

## Enumeration and controls

For each sorted base abscissa, use its first point `i` and every signed point
`j` above that and subsequent abscissae, in the scalar builder's exact order.
Compute `S=P_i+P_j`; skip infinity. Key by `x(S)`. Retain the first witness
for each key. On a later witness, compare full sums:

- `S_current=S_first`: form `P_i+P_j-P_first_i-P_first_j=O`.
- `S_current=-S_first`: form `P_i+P_j+P_first_i+P_first_j=O`.

Reject any other case. Check every candidate by full group addition, then map
the four signed point indices to `col_of`/`coef_of` modulo `r`; count zero and
nonzero vectors separately. Compute incremental modular rank of nonzero rows.
The first-witness table's entry and insertion-attempt counts must equal the
production scalar `PairTable` counts. A deliberately perturbed nontrivial
candidate must fail the full-group check; if none exists, record the control
as inapplicable. All duplicate candidates, including zero rows, are recorded
in aggregate. Raw rows are retained for replay.

Stop invalid on any control failure. Per cell ceiling: 200,000 pair additions,
120 seconds, 512 MiB resident memory. If a ceiling is hit, mark that cell
censored and retain its partial record; never classify it as a zero-rank
result. Run each cell once, with no seed or adaptive parameter choice.

## Decision and cost boundary

Positive independent rank in any cell justifies a separate integration and
same-target comparison protocol, with harvesting, verification, rank updates,
base construction, decomposition, elimination, and target descent all charged.
Zero rank across all completed cells rejects this specific harvesting path for
these cells only. A control failure makes the observation invalid. Homogeneous
rows do not themselves name the target logarithm, and this screen makes no
speedup or ECDLP cost claim.

## Provenance and artifacts

Primary current source: `src/cryptanalysis/ic_boundary.rs` (negation-folded
builder and signed column fold), `src/cryptanalysis/koblitz_index_calculus.rs`
(standard subspace), and
`papers/rootfinding-decomposition/rootfinding_decomposition.tex` (pair-table
algorithm). Adjacent established implementation:
`PairSumTable::build_folded_within_harvest` and `free_relations` in
`koblitz_index_calculus.rs`; this census does not claim that duplicate-sum
harvesting is new. Shared KB revision `9141e386254340a9bdba3036f2bb706a`,
last refresh 2026-11-14T21:44:48Z, predates this branch. Relevant source IDs:
`17a1b67993e4b781bbf1198fd2598c2f`,
`d6a1186b61a04985b87e930cae476f05`, and
`e808ee120fcb3a6265261193039c6353`.

The executable is `examples/rootfind_pair_collision_census.rs`. Each cell's
write-once run directory under `reports/run-<UTC-start>/` retains command,
source commit and dirty state, Rust version and host, stdout JSON, stderr,
exit status, and elapsed resource observations. The report cites those raw
files, contains a quantitative SVG, its source data, and an inspected PDF.
This protocol is frozen in git before the first census execution; later
changes use an explicit amendment rather than modifying this file.
