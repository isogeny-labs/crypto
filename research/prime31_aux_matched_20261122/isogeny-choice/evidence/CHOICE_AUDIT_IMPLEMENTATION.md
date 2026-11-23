# Independent replay for the isogeny-choice driver

Implementation status: complete; compilation and the combined focused test run are owned by the root session. No new research campaign or performance run was launched by this agent.

Files changed:
- research/prime31_aux_matched_20261122/native/src/audit.rs
- research/prime31_aux_matched_20261122/native/tests/choice_audit_contract.rs

## Verification boundary

The new path imports serialization types from kernel_fibers and target_transport. It never calls their builders or map evaluators while auditing. It uses the existing independent affine group implementation in bases.rs, an independent modular-power implementation, and independently recomposed rational-map polynomials. The existing BaseDefinition verification remains unchanged on legacy events; the new prepared base is not disguised as a plain BaseDefinition.

The four auxiliary choices are independently grounded in fixed E_plus/E_minus and fixed S coordinates. The auditor verifies the exact 2-power source order, Hasse uniqueness, derived K, H0/H1/H2 generator metadata, complete 1024-point kernel inventory, the requested odd coset scalar, every degree-two transition, every kernel and coset point's map image, and the nonidentity/non-two-torsion endpoint. It reconstructs numerator, denominator and full fiber polynomial, checks the exact product over all1024 x-roots, and verifies denominator poles at all nonidentity kernel x-coordinates without numerator cancellation.

The target degree-three map coefficients are derived independently from alpha and the source curve. The auditor checks alpha's division-polynomial identity and nonsquare ordinate, reconstructs the codomain, and re-evaluates both x and y coordinates. Public G/Q images, full membership images including duplicate x-values and dropped poles, all canonical target lifts, and every sign/permutation entry must agree.

For matched plain controls, the verified structured preparation is retained as the reference. Collection is not allowed to start until choice_plain supplies a valid original BaseDefinition with the same usable column count and the proper target curve/order. Fixed seed1 and trial cap1,000,000 are bound, and the auditor independently reconstructs the first required lifting coordinates from x=1 instead of trusting a seed label.

Existing natural-query stream replay, signed row checks, independent batch RREF, duplicate/useful dependency handling, and partial/truncated stream logic are reused. Scalars on an isogenous arm are checked on both the destination and the original source.

## Event contract

IC:
1. choice_start {cell}
2. choice_kernel {fiber}
3. choice_prepared {base}
4. choice_plain {base,seed:1,max_x_trials:1000000,reference_columns,...}, only when plain_control=true
5. Existing collection_start/query_start/query_result/relation/collection_summary events
6. cell_summary with exact nested collection summary and original_scalar_replay

Rho:
1. choice_start with method rho_negation/rho_v6/rho_fast
2. rho_summary with result.recovered, independent_replay and public_target

A rho receipt cannot carry an IC kernel/base. An IC cell cannot terminate as rho. Old/new selected-base event families cannot be mixed.

A preparation-only stream remains incomplete. Audit complete still means a finished recovered cell. Negative search completeness, operation counters, timing, admission, source/input hash custody and signed PCC standing remain outside this mathematical witness audit.

The driver pins the campaign Q=(984837385,1142774582). The mathematical auditor binds and checks the source Q declared in the cell, allowing constructed public-Q correctness fixtures; external input provenance is separately required.

## Seven new development contracts

- All four independently replayed kernel inventories.
- Source and transported signed rows through the existing query and rank audit, plus native-codomain preparation.
- Kernel/map/coset/polynomial tampering.
- Destination/member/sign-permutation and row tampering.
- Event ordering, missing preparations, partial streams and trailing JSON fragments.
- Rho destination/source scalar checks with a synthetic known scalar, without running rho.
- Matched plain reference/cardinality/seed/cap validation.

Test transcripts are explicitly constructed unit fixtures. They are not new natural-query discoveries, Prime31 relation-collection results, scalar-recovery experiments, or timing samples.

## Driver failure and counter review

Read-only review found an oracle-error classification mismatch: an IC `oracle_error` summary initially exited with code 2, which the launcher classified as capped without recovery. At the root agent's direction, `choice_main.rs` now returns code 1 for that status. Other unrecovered bounded outcomes retain code 2. The auditor now accepts `driver_error` as a terminal failure only after a valid frozen cell; it never marks that receipt as a completed recovery. Existing ordering tests cover errors before preparation, after preparation, during a pending query, and after the collection summary, and reject a subsequent success record.

The remaining accounting boundary is explicit: native ledgers are partial, failed fast-rho operation counters are null rather than zero, and destination rho receipts include target transfer/validation plus independent replay on both curves. A target-setup failure can lose an internal ledger that was not returned; its process and cold cost remain required. New current-main domain/batch diagnostics remain available in each chart's raw visit report even when the collection's compact aggregate does not repeat every field.

Formatting and diff-whitespace checks passed after these fixes. The root session owns compilation and combined correctness execution; no separate build or experiment was started by this agent.
