# G2 gaps rechecked after integration

2026-10-30. Source merge `a030c1630518564bdb3633815391858b424377ce`.
This is a read-only update to [the accounting audit](G2_ACCOUNTING_AUDIT.md),
not an implemented counter fix, a priced result or a scientific panel.

The native PKM example/source tree is byte-identical to its local parent after
integration. The following observations still hold on the new dependency state:

- `Request::validate` creates a temporary ordinary affine reference and drops
  its counters. The validation phase records constructor helper arithmetic,
  which does not recover that separate reference work.
- `GroupOps.field_ops` and `montgomery_reductions` remain `serde(skip)` in
  `ic_boundary.rs`; the existing serialized query-generation ledger does not
  emit those field details. A separate explicit block is needed.
- Helper, reference and scalar inversions still expand through their counted
  multiplication helpers. A future price must choose the expanded leaves or
  the whole-call calibration, rather than charge both for one execution.
- F4 still documents its field counter as row-reduction work and its native
  counter as incomplete. `VisitReport` emits root multiplications, not a
  complete inventory of all root/substitution/symbolic/logical work.
- No durable full-panel failed-call journal, complete independent-replay count
  interval or qualified native resume route has been added by this integration.

The next source change should retain dropped validation counts, serialize the
separate query field/reduction block and declare counter ownership, with native
conservation and refusal tests. It must retain the remaining incomplete classes
and `comparison_admissible=false`. Complete emission/pricing, matched external
and rho references, G4/G5 panels and Rust/polynomial/cost proofs remain open.

Native validation of this source stopped at the inherited unclosed delimiter before any tests ran. Its new Lean gate passes
18 declarations with zero trusted exceptions; that does not discharge the cost
obligations. Do not edit bound source while the source-pinned job is active.
