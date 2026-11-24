# Integrated M31 graph, structured solver, and RR5 build

Tracking: [PR #5120](https://github.com/isogeny-labs/crypto/pull/5120).
Started 2026-11-23 Pacific; UTC session receipts retain their exact timestamps.

## Requested deliverables

| Component | Implementation scope | Correctness gate | Performance gate |
| --- | --- | --- | --- |
| Graph comparison | Separate-process eager/lazy forest, pivot and MST arms; bounded matrix-free projection tape | Same admitted stream, signed relation replay, cross-arm exact ranks and target outcomes; dense tape row-space and forward/transpose controls | Frozen matched replays with complete phase accounting and resource receipts |
| Structured solver | M31 constrained three-summand expanded S4 versus chained S3; explicit cancellation chart; certified selector normal forms | Exact polynomial certificate replay and complete signed solution-set controls against an independent finite-base oracle | Actual admitted F4 matrix/degree/cost comparison, preserving incomplete outcomes |
| RR5 streaming | Deliver algebraic coefficient solutions through quartic root recovery, signed lift and literal point replay to the existing target adapter immediately | Batch/stream equality, stop semantics, false-RHS and duplicate controls, public-label target-functional recovery | Registered B677 extraction and fully charged cold comparison remain separate |

The original acceptance criteria for the larger research program remain in
`docs/ACTIVE.md`. This source build does not redefine them. New elliptic-curve
controls use the prime coordinate field p=2147483647. Relation matrices use
the admitted curve's subgroup order, not p.

The generic quadratic-resultant certificate uses the repository's fixed-M61
identity-record format. It is auxiliary algebraic bookkeeping and does not
replace any M31 curve control or supply a formal proof.

## Source reuse and identities

The ordinary `prime31_algebraic_ic::PRIME31` fixture has
`a=847514846`, `b=2087444724`, `n=2147446831`, and
`G=(625037715,2100627251)`. Its source is
`research/prime31_ic_20261115/native/lib.rs`. The separate registered
curve `a=-3,b=21` has a different subgroup order and generator. An M31 label
alone is insufficient to identify a curve or join a stream. Existing graph
capture admission supplies the full public identity.

The new code reuses the repository's graph filters, F4 solver, finite-field
and point arithmetic, and signed relation adapter. Frozen protocols, source
snapshots, captures, and historical results are not rewritten. The existing
public-query solver in `research/prime31_rr5_public_relations_20261123` is
prior work; the new RR5 API is an incremental consumer rather than a second
seeded query loop.

## Sources checked

Sources checked (KB): the KB MCP is unavailable in this runtime and
`scripts/research-kb status` exits 127 because the executable is absent.
Repository sources, active lane PRs and their current handoffs supplied the
fallback. Conductor is also unavailable (`command not found`); the three
workers use disjoint parent-owned paths and do not create separate PRs.

- [Semaev, Summation polynomials (2004)](https://eprint.iacr.org/2004/031):
  theorem 1 and S3/resultant construction, read through the author's
  [full-text copy](https://www.researchgate.net/publication/2869464_Summation_Polynomials_and_the_Discrete_Logarithm_Problem_on_Elliptic_Curves)
  after the IACR PDF returned HTTP 403. Algebraic-closure roots require
  rational signed lifts and literal curve replay for our output contract.
- [Faugere, F4 (1999)](https://wstein.org/129-05/refs/faugere_f4.pdf),
  section 2: symbolic preprocessing and batched linear reduction. Input
  equation degree is not substituted for observed solving degree.
- [Gaudry, Theriault and Thome, double-large-prime technical report](https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf):
  relation-graph reduction is separate from relation collection and final
  matrix solving. The hyperelliptic performance conclusions are not
  transferred to the ordinary elliptic M31 fixtures.
- Existing `src/cryptanalysis/ic_graph`, `large_prime_filter.rs`, F4 source,
  and the `prime31_four_summand_20261122` mathematical/source contracts.

## Execution boundaries

Local builds, unit tests, static checks and reading existing records are
development work under AGENTS section 9a. No paid experiment, natural
collector rerun or claimed timing comparison is part of these development
controls. Matched scientific runs require the maintained IsoLab admission,
checkpoint transport, published source/image, finite remaining budget and
complete result custody. Existing no-merge/no-paid-compute holds are retained.

## Evidence status

The final component gates report **70 passing tests**: graph 7, structured
10 library plus 3 CLI, and RR5 42 library plus 8 public-example tests. The
graph suite also includes 128 seeded dense-oracle systems. The final graph
receipt is `graph/evidence/RECEIPT.json`, pointing to the corrected 06 gate;
earlier attempts remain intact. These observations remain producer evidence
until the repository's independent signed verification process supplies a
verdict. [pcc:unverified]

Read [the implementation report](reports/REPORT.md), its
[PDF](reports/m31-integrated-build.pdf), and the
[independent review](evidence/INDEPENDENT_REVIEW.md). They separate exact
source scope, terminal component results, the root-library gate and the
remaining admitted scientific work.

## Build and entry points

From the repository root:

```sh
bash research/m31_integrated_build_20261123/check.sh NEW_EVIDENCE_DIRECTORY
```

This bounded local wrapper runs the three component release gates, records
each exit and continues subsequent planned components after a failure.
Set `CARGO` and `RUSTC` to explicit installed executable paths when needed;
set `OFFLINE=1` to use a populated registry cache. The constituent commands
were executed directly; the wrapper itself is syntax-checked.

- [Graph CLI and operator API](graph/README.md): four-arm comparison on an
  admitted fixed-curve capture, plus forward and transpose tape products.
- [Structured solver CLI and library](structured/README.md): one JSON
  request on stdin; complete, incomplete and malformed cases have distinct
  exit codes.
- [RR5 visitor APIs and public-query adapter](rr5/README.md): immediate
  signed-row delivery and target-functional stopping in the existing native
  crate.
