# H1: opt-in multiplication coordinate with dense fallback

23 November 2026. Implementation and finite software qualification only.
Producer observations remain **[pcc:unverified]**. The prospective
recorded-prefix comparison has **not run**; no coordinate is recommended as
faster by this packet.

[Report PDF](report.pdf) · [Flow diagram](action-flow.svg) ·
[Prospective Runpod component plan](PLAN.md) ·
[Test receipt](validation/native-tests.log)

## Implemented interface

- `zero_dim_la::fast_stage_with_action` takes an explicit zero-based action
  coordinate. It builds that multiplication matrix, puts `NF(1)` in the
  matching functional slot, recovers every other coordinate, and returns
  roots in the original input order. The Gröbner basis/order is unchanged.
- `zero_dim_la::fast_stage` continues to select the last coordinate.
- `jv_action_operator::linear_stage_from_basis_with_action` exposes the
  option through the IC library and retains the existing dense stage on
  fast-stage refusal. It adds both attempted fast and dense fallback
  multiplication counts to the existing partial linear counter.
- Invalid coordinates are configuration errors at the adapter boundary,
  before solver/RNG work. A legitimate empty root set remains distinct
  from a fast-stage refusal. The caller still owes a valid input basis;
  these interfaces add no certification of an incomplete F4 result.
- A source-built control example supplies all-coordinate checks and a
  prospective fixed-prefix selection driver. It does not change the
  existing graph collector, residual stream, production solver defaults,
  graph implementation ownership or benchmark records.

## Software controls actually run

The scoped native test harness reports **14 passed, zero failed, one
ignored**. The ignored test is the pre-existing explicit throughput test;
it was not executed. Eight passing tests are existing arithmetic/hash
controls and six are new action-coordinate controls. [pcc:unverified]

| New control | Evidence and scope |
|---|---|
| Original coordinate order | Every coordinate on explicit two-, three-, six- and eight-variable shape-position bases returns the known root set |
| Incumbent compatibility | Default fixture roots, all non-wall fast counters, and next RNG value equal the pinned original stage |
| Nonseparating coordinate | Refusal after the existing four projections; dense fallback returns the brute-force root set, and both attempt/fallback counts are included |
| Empty rational root set | Returns an actual empty result for the explicit nonsplit quadratic, rather than a refusal |
| Repeated root | The explicit nonradical fixture returns the exact point, matching the incumbent |
| Invalid coordinate | Refused before RNG advancement; adapter error is not counted as an empty answer |

These are finite software controls, not fresh ECDLP solves, an all-input
completeness theorem or independent signed validation. [pcc:unverified]

## Scoped build and inherited diagnostics

The example compiles the changed `zero_dim_la.rs` and action adapter directly
against the retained Root41 dependency library. Its private `PolyRing` and
`UPoly` cannot be imported across that library boundary: the initial build
failure is retained. `scoped_poly_ring.rs` therefore contains the original
Fp helpers and polynomial-ring items copied from `gaudry_cubic.rs`, with no
algorithm or counter edits. A nonblank-line byte comparison checked both
copied source sections against the current original. Default compatibility
is additionally checked against the pinned library on the exact fixture.

Initial strict Clippy on the included old linear module reports three
pre-existing diagnostics under Clippy1.100: two `chunks_exact_to_as_chunks`
suggestions and its BM `needless_range_loop`. The example documents those
two lint allowances on that included module only. The new adapter and driver
remain under `-D warnings`; the resulting scoped check exits0. This is not
a claim that full-library Clippy or CI passes. [pcc:unverified]

The root library's separately recorded unrelated syntax failure is not
rerun or rewritten. Exact compiler argv, toolchain, source/library and
LOCAL binary hashes are in `validation/`. Compiler output remains verbatim.
Native executables stay LOCAL at `/tmp/ic-action-h1-20261123/`, outside the
shared target directory. The worktree's Root41 artifacts are unchanged.

## Corrected test assumption and retained failures

The first new test run had13passes, one failure and the existing ignored
throughput test. Its assertion incorrectly required every repeated-root
ideal to force fallback. On `x-y, y^2` the incumbent and new operator can
recover `(0,0)` exactly. The corrected control requires that exact result
and incumbent agreement; it makes no general claim about nonradical ideals.
The initial assertion log and plan are retained. The nonseparating fixture
still exercises actual refusal and dense recovery. No recorded-prefix or
performance command had been executed. [pcc:unverified]

## Pending comparison and interpretation

The example's prospective driver can check the first32 saved six-coordinate
systems, choose a coordinate on indices0..15 by the explicitly limited
multiplication proxy, and retain results on indices16..31. It includes all
development candidate counts so selection cannot be presented as free.
It also retains generated bases for later reuse. This remains an exposed
same-fixture component test, not fresh-instance generalization.

That comparison stays on the user's selected **Runpod** backend. It was
not dispatched: existing funding/checkpoint prerequisites remain unresolved,
and `PLAN.md` is not a frozen or admitted execution protocol. Synthetic
unit tests do not replace that comparison. No field-operation ordering,
runtime gain or better action coordinate is established by this packet.

The next technical step is the source-bound Runpod coordinate comparison;
H2's elimination-template construction remains an independent possible
continuation. Full operation costs and cold selection/fallback overhead
must be included before interpreting any later component result.

## Source and literature receipt

- Previous hypothesis report: `../hypotheses-20261123/README.md`, H1.
- Faugère–Mou, *Sparse FGLM algorithms*,
  <https://arxiv.org/pdf/1304.1238>, especially the sparse multiplication
  operator and shape-position qualifications. Generic grevlex-last-variable
  advantages are retained as a reason this hypothesis may have no benefit.
- Actual source read: `zero_dim_la.rs` matrix/functionals/readout;
  `jv_cover.rs::linear_stage_from_basis`; original polynomial-ring items;
  pinned Root41 library used by earlier recorded-input controls.
- Sources checked (KB): no exposed KB MCP tools; wrapper unavailable127.
  Conductor unavailable127. Use the preceding bounded primary review and
  repository sources, with no novelty or repository-wide absence claim.

This is additive library integration and finite regression evidence.
Canonical scoreboards and measured graphs remain unchanged because no
new performance comparison was run. The diagram shows the implemented path
and fallback; it is not a measured speed plot.
