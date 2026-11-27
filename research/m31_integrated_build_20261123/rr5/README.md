# Streaming RR5 public queries

Tracking: combined build PR #5120, 2026-11-23 Pacific.

## Implemented scope

The existing distinct-abscissa RR5 extractor now has three streaming entry
points: `extract_visit`, `extract_cofactor_visit`, and
`extract_hybrid677_visit`. Each coefficient solution first passes quartic factorization,
signed factor-base lift, duplicate filter and literal curve replay. Only then
may a caller consume the row. Returning `true` stops the algebraic traversal;
returning `false` continues. Emitted rows are also retained as a bounded prefix
in `ResultRow`. Consumer stopping returns `ConsumerStopped`, with the native
traversal explicitly `complete=false` and `stopped_early=true`.

The caller-driven query adapter uses the existing `RelationAdapter` for
authentic right-hand sides, exact rank and scalar replay. Its inputs are the
existing ordinary M31 curve, public base points, public target, and public
coefficients `(a,b)` forming `[a]G+[b]T`. There is no scalar hint, pair table,
tuple scan or new seeded query generator. `QueryPipeline::submit(a,b,...)`
uses the explicit monic-cofactor presentation and sends each signed row to
the adapter immediately. It stops as soon as the adapter obtains and replays
the target scalar. The existing batch API, B677
protocol, excluded charts and frozen results retain their meanings.

## Prior work checked

- `research/prime31_four_summand_20261122/MATHEMATICS.md`, Sections 3, 7, 8:
  the chart, excluded cases, and algebraic branch-completeness contract.
- `research/prime31_four_summand_20261122/native/extractor.rs`: current
  extraction buffers all coefficient solutions before signed lifting.
- The same study's `native/relation_adapter.rs`: signed replay, canonical
  duplicate handling, exact modular rank, target functional and scalar replay.
- `research/prime31_rr5_public_relations_20261123/native/solver.rs`, read from
  the current Git tree: existing seeded public-query solver consumes rows
  after extraction. This addition does not copy that loop.
- `src/cryptanalysis/f4_fp.rs`: the existing visitor marks callback stops
  `stopped_early=true`, `complete=false`.
- Parent review covers the primary Semaev and Faugere sources. No novelty or
  complexity advantage is asserted for streaming integration.

Sources checked (KB): no KB MCP is loaded; `scripts/research-kb status`
reports no executable. `conductor check` also reports command not found.
The parent task reserves the files listed above; no external task record is
fabricated.

## Query receipts and boundaries

Each query receipt retains its public labels and constructed point, consumed
row receipts, rank before/after, recovered scalar if any, extraction result,
consumer error, query arithmetic and cumulative adapter work. `AlreadyRecovered`
does no further query arithmetic. `QueryExhausted` means that the supported
distinct-four RR5 chart was exhausted; it is not an absence claim over excluded
decompositions. Identity query points R and query points whose abscissa lies in the base
remain `UnsupportedChart`, giving pipeline state `Incomplete`.

Resource refusal, deadline and output-cap stops retain their specific extractor
status; rejected signed lifts do not overwrite those reasons. Successful target
recovery is a statement about the adapter's target functional, while exhaustive
extraction is a separate property. The receipt keeps those states separate.

Deadlines are cooperative. Construction, F4 and root work check the shared
deadline; an in-progress callback cannot be preempted. Adapter setup and public
query arithmetic are not covered by the extractor's clock. A hard elapsed-time
limit requires an outer process supervisor. `phase_accounting_complete=false`
is retained because these counters do not price the complete cold solve.

## Development checks

Use unchanged `PRIME31`, p=2147483647, for all new field-dependent controls.
The ten new M31 checks compare streamed and batch signed sets, require callback
stop to remain incomplete, check multiple-row continuation, output and native
allocation caps, unsupported and empty charts, false-RHS rejection, duplicates,
and input identity. Two public labelled queries feed the existing rank consumer
and require literal scalar replay before full column rank. These are bounded unit controls, not
timing experiments, natural relation-yield studies or cold-cost comparisons.

Local builds and unit checks are authorized development work under AGENTS
section 9a. Registered B677 extraction remains a separate admitted scientific
run, and is not launched by this work.

The initial release library gate returned exit 0 with 42 tests passing,
including all ten new M31 tests and the unchanged legacy regression suite.
The latter retains its historical smaller-field controls; these are not
substituted for the new M31 checks. Raw log and exit are in
`evidence/native-tests-initial.log` and `.exit`. [pcc:unverified]

The disclosed test base is `[2,3,5,11]G` with target `7G`. Public labels
`(21,0)` and `(7,2)` give the same query point `21G`; their two valid relation
equations isolate the target at rank 2 of 5. This is a constructed correctness
control, not a generic unknown-target trial or a relation-yield result.

## Source-pin review before the build

Two original-source whole-file guards are stale on the current main tree.
The reviewed sources are byte-equal to commit `0a3932b280`, whose PR #5099
audit is `scaling-b677/payload-integration/SOURCE_REVIEW.md`. A fresh Git diff
against the frozen sources confirms the described additions: two cfg(test)
map accessors in one selected range of `ic_boundary.rs`, plus outside-range
changes; the selected `Fp` range in `semaev_prime_poly.rs` is unchanged.

The explicit refreshed hashes are:

| File | SHA-256 |
|---|---|
| `src/cryptanalysis/ic_boundary.rs` | `7a7ff906dd2206b236a31155f0bdaa2d753b890b2edd79a67aaeb5cb52d72311` |
| `src/cryptanalysis/semaev_prime_poly.rs` | `e77ebb1576469b24e399cf2cd62059b80607eb65eeaa486847782e6f11714f9b` |

All equality assertions, literal unique anchors, interval ordering, verbatim
source copying and readback checks remain in place. This refresh does not
accept an arbitrary latest revision or modify the production solver.
