# Fresh panel input qualification, 2026-11-27

This receipt covers input preparation and parser controls only. No solver, benchmark, image, S3 upload or scientific workflow ran. The source PR and final paired protocol are still unresolved. [pcc:unverified]

- Rust 1.78, offline locked release `hashed-ecdl-panel` tests: 15 passed, 0 failed. The new controls check 84 unique fresh seeds, independent tiny-curve target points, v1 and v2 loading, seed-sidecar tampering, no overwrite, and rejection of a complete v2 run while source pins are pending. Raw `panel-tests.log` is inside the lossless `development.tar.xz`.
- Rust 1.78 strict panel Clippy encountered the pre-existing `format_collect` diagnostic in `panel_driver.rs:242`. Scoped Clippy with only that inherited class allowed passed. Its raw `panel-clippy-scoped.log` is in the archive; the strict diagnostic is not represented as a strict pass.
- Changed-file rustfmt check passed. Archived `panel-build.log` and `replay.log` retain the final-source generator build and byte-equal reproduction of all 86 pinned panel files plus the input hash list. `source.sha256` pins the final native generator/loader and seed policy. `raw-logs.sha256` was verified after extraction of the five archive members.
- The first new generator test exposed an omitted v2 seed sidecar in `hash_list`; the implementation was corrected before preparation and final qualification. Existing v1 panel inputs and manifests were not edited. The `pending-final-source-freeze` markers prevent this receipt from becoming timing admission.

The seed manifest, panel manifest and input-list SHA-256 digests are `119f95bec637ec55675dec63e5e20f5fd379c80eec92140968d6bbc3bd864b36`, `b65f0397ea80c9e9ec5d6a95efb5a835dee10ef9b9a5f3519780c656a6e0db95`, and `be7ceff3ac847d71f9ebb7ea464db687a47c4a40c6607c9fa2c9d008d553cc01`. [pcc:unverified]
