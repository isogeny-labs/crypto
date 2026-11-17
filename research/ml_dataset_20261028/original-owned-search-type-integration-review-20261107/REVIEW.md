# Source integration review of the phase-accounting search adapter

The exact bound source assembly passed. Manual inspection found no concrete Rust module-path, visibility, trait-signature, worker-initializer, or enum-variant mismatch in the inspected additions. This is source review, not a compiler or borrow-checker result: `compiled=false`, `type_checked=false`. No substitute Budget, map, capture owner, or worker harness was constructed. Frozen inputs were not edited.

The inspected proposal is `original-owned-search-phase-accounting-proposal-20261107`, manifest SHA-256 `4f4d1fe5968c0f9da3ff800ace32e1c656b34d8f7936a314e7dfa26841a39150`. Its adapter is SHA-256 `e1159f56cc37800f592fb37939e6c489e94249fedf8e94a34f633b6a8931877c` (14,994 bytes). All paths below are relative to `assembly/staged/`, unless a proposal filename is explicitly named.

## Mechanical checks

`assemble_source.sh` copied and verified 53 input references at 49 unique input paths. These include every current proposal member, all 12 current binding references, all 12 original transport source bindings, and all original transport members. Four additional declarations needed for this review (`capture.rs`, `types.rs`, `sparse_equations.rs`, `binary_semaev_chain_sat.rs`) were snapshotted and hashed as **supplemental observed source**; their hashes are not silently promoted to pre-existing proposal bindings. Copied input bytes are retained by digest, and `assembly/INPUTS.jsonl` distinguishes their provenance.

The corrected registration patch and unchanged worker patch pass ordinary `git apply --check` and apply in the new scratch tree. The resulting worker equals the bound `solvers.proposed.rs` byte for byte. Separately applying `phase-accounting.patch` to its bound lift adapter reproduces the current adapter exactly. The staged file checksums pass. The staged tree is a scoped review tree, not a complete crate.

## Module resolution

The relevant path attributes are declarations in out-of-line source files, not declarations inside an inline module block. Rust resolves them from the containing source file's directory. This follows the official [Rust Reference module-source rules](https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute).

| Declaration | Resolved source file | Review |
|---|---|---|
| `src/ml_dataset/solvers.rs:20` | `src/ml_dataset/koblitz_m3.rs` | Existing source |
| `src/ml_dataset/koblitz_m3.rs:14854` | `src/ml_dataset/sat_original_owner.rs` | Existing source |
| `src/ml_dataset/sat_original_owner.rs:974` | `src/ml_dataset/original_boolean_search.rs` | Corrected registration |
| `src/ml_dataset/original_boolean_search.rs:10` | `src/ml_dataset/dynamic_boolean_search_scheduler.rs` | Exists at intended relative path |
| `src/ml_dataset/original_boolean_search.rs:11` | `src/cryptanalysis/boolean_identity_bqi.rs` | `../cryptanalysis` is correct |
| `src/ml_dataset/original_boolean_search.rs:12-13` | `src/ml_dataset/dynamic_boolean_search_{bqi,check}.rs` | Both exist |
| `src/ml_dataset/dynamic_boolean_search_scheduler.rs:160` | `src/ml_dataset/dynamic_boolean_search.rs` | Child module remains file-relative |
| `src/cryptanalysis/binary_semaev_chain_sat.rs:13` | `src/cryptanalysis/binary_semaev_chain_provenance.rs` | Existing source |
| `src/cryptanalysis/binary_semaev_chain_provenance.rs:1170` | `src/cryptanalysis/checked_binary_search_lift.rs` | Corrected registration |

The inline `sparse` module in the new adapter has no nested file declaration. It only re-exports the exact expansion polynomial/charge types and declares the adapter Owner trait. It therefore does not introduce an extra directory into the file paths above.

## Types and visibility

- `original_boolean_search.rs:19-23` passes the actual `koblitz_m3::Reservation` to `Budget::observe(current: Reservation, now: Instant)` (`koblitz_m3.rs:238`). `super::super` from `boolean_search` resolves to `koblitz_m3`. `phase_native`, the budget's private methods, and parent owner fields are accessed by descendants, which the [Rust privacy rules permit](https://doc.rust-lang.org/reference/visibility-and-privacy.html). All `pub(in crate::ml_dataset::solvers)` paths name an actual ancestor module.
- The `MatrixAdmission` implementations (`original_boolean_search.rs:26-32`) match `provenance::Admission::{reserve(Quote),check}` (`binary_semaev_chain_provenance.rs:29-38`) and the exact adapter `sparse::Owner` declaration (`original_boolean_search.rs:6-9`). The scheduler, bridge, checker, and search all consume this same Owner and the same re-exported Charge/Polynomial types.
- `LiftAdmission` implements the same provenance Admission plus `checked_model::VerificationAdmission::addition` (`checked_binary_sat_model.rs:15-17`). `signed_lift::verify_candidate` receives `&CheckedMap`, `&[bool]`, `&[(u64,u64)]`, `Option<(u64,u64)>`, retained `Progress`, `Instant`, and that original admission object. The declared `Math` fields (`sat_original_owner.rs:17-24`) match.
- `SearchLimits` and `AuditLimits` are re-exported at adapter lines 14-15. Their structs and constituent values have the Copy/Clone properties used by `Caps`. `Failure`'s payload types supply the requested Copy/Clone/Debug derives. Its match arms and `signed_lift::Outcome` variants agree with the inspected declarations.
- The full `OriginalOwner` move at adapter line 188 names every field in the bound owner declaration (`sat_original_owner.rs:77-92`). No lifetime-bearing original reference survives in `OriginalParts`; the original data/sink/budget are moved and copied digest fields are fixed arrays.
- The capture calls at adapter lines 184-185 match `CaptureEmitter::verify_directory(&Path)` (`capture.rs:911`) and `verify_attempt_limits(&AttemptBinding,&CaptureLimits)` (`capture.rs:1333`). `VerifiedWorkerOriginal.binding` is `Option<&AttemptBinding>` (`solvers.rs:264`).
- The new worker constructor (`solvers.rs:428-431`) and the modified existing explicit initializers (`solvers.rs:399,460-464,473-475`) supply the new custody fields. Other initializers use `..Default::default()`. `Option<OwnedAttempt>` does not require `OwnedAttempt: Default`. The new `SolverData` literal matches its actual two-field declaration (`types.rs:207-210`). The mod.rs guard prevents this unpublished custody from becoming a completed report.

## Concrete integration gaps

1. **No activated dispatch.** `into_boolean_search` and `from_original_boolean_search` have declarations only in this proposal; no call site connects them. The existing dispatcher (`solvers.rs:804-852`) selects existing affine, SAT-search and SAT-initial routes. The proposal remains retained transport. It must not be described as a compiled or executable new worker route.
2. **An admitted caps factory/entry point is still needed.** The consuming method is `pub(super)` inside the owner child (`original_boolean_search.rs:178`), usable from an owner-level sibling worker module, not directly from solvers. Re-exported `SearchLimits` and `AuditLimits` contain limits declared in private `scheduler`, `bridge`, and `bqi` modules (`scheduler:11`, `check:6`, `bridge:8`, `bqi:35`). There is no public default/factory here that constructs a complete Caps using the frozen worker policy. Add a narrow owner-level entry point and an internal caps constructor, or deliberate narrow re-exports; do not broaden budget authority or invent a new policy merely to satisfy visibility. This is an integration requirement, not a compile failure in currently unused declarations.
3. **Supplemental source must enter an exact build manifest.** The current proposal binds its budget and mathematical donors, but the capture/worker payload declarations needed to establish genuine whole-crate integration are only observed in this review. The next genuine build must bind these and the remaining full crate/dependencies; this scoped tree cannot establish that result.

No speculative corrective patch was produced because inspection did not establish a concrete compile mismatch. The next gate is an explicitly authorized genuine assembled-module compile and original-owner/capture boundary tests, followed by separately admitted dispatch/publication/replay integration. This review does not authorize the pending SAT rebuild.
