# Affine mul caller source review

The proposed relation follows the pinned UInt64 caller order and derives storage invariants at each reached loop boundary. No source correction was identified for its stated acceptance-only scope.

- Acc uses literal UInt64 lowbit; Run performs accumulator add then shift then conditional square, matching native1062–1073. Scalar0 done and scalar1 skip-last-square are explicit.
- Reach closure derives both-register WellFormed from initial invariant and imported every_execution_preserves; accepted_return uses test_accepts_wellformed, not output acceptance as premise.
- Shift strict decrease uses UInt64.toNat_shiftRight and natural division by2; actual pinned toolchain source contains these APIs. Proof elaboration remains pending.
- Steps counts loop iterations separately from addition events. bounded_execution/accepted_within_sixty_four assert existence of a witness with n<=64, not elapsed-time/cost bounds or a universal count theorem for arbitrary relations.
- Controls0/1/2/3 reflect exact event order; valid prime17 input and degree5 curve premises are constructed. Existing validator-only noncanonical-padding counterexample is retained.
- 17main declarations and9controls give26 prospective named readbacks; negative WellFormed assertion targets actual canonicality gap.
- All listed source bindings matched inspected bytes, including completed prerequisite review8bec7. No scalar/divisor-class equation, full Rust/compiler fidelity, extension-field behavior or cost claim added.

- Canonical initial stored slots remain premise; point/transfer provenance is still separate.
- Source-compatible mathematical relation and storage acceptance only; prospective elaboration/kernel/readback and bounded dependency protocol remain required.
