# Exact next central wiring

The existing native source establishes the order in `sat_initial_central.rs::run`:

1. Around lines525–534: `storage::verify_manifest`, `execution::verify`, `verify_capture_context`, `execution::load`, `admit_capture_context`, and launch/capture equality.
2. Around lines630–642: reserve and execute `verify_factor_base_config_bounded`; preserve the same budget and check the result.
3. Around lines644–659: hash the retained `f.public`/`f.factor` bytes and mint `VerifiedWorkerOriginal` with `binding:Some(&binding)`.
4. Around lines662–672: existing code derives the owner binding and canonical typed launch hash. This is the precise identity derivation boundary, not the earlier schema read.

The proposed capsule helper belongs after step3 and before map/math replay. It can replace duplicate owner/launch digest logic in a dedicated owned-search verifier once that route exists. Do not merely widen the existing `config.solver != "sat"` gate: subsequent transcript handling is SAT-specific.

Before invoking it, extend the owning Frame with retained `identity_run_raw`, `identity_execution_raw`, `identity_manifest_raw` and `identity_capsule:Option<Capsule>`. Use the existing budgeted `read(...,&mut f.budget,&mut f.pending_read)` path with1MiB caps so failed reads retain custody. Require all loaded context files and manifest artifacts to be revalidated under the same central admission, and fingerprint the manifest before/after capsule construction. The raw manifest fingerprint identifies the exact loaded manifest, not an independently trusted external version.

The call shape is:

```rust
let capsule = owned_identity::derive(
    &token, &launch,
    &f.identity_run_raw, &f.identity_execution_raw, &f.identity_manifest_raw,
    &mut f.budget,
)?;
f.identity_capsule = Some(capsule);
```

All errors return through the existing central retained-frame failure path. Store the capsule before later fallible map/row checks. Revalidate source files and exact manifest/context fingerprints before projecting a result. The caller must retain the frame across projection; a successful capsule alone does not admit row results or solve status.

For independent Python integration, expose `capture_binding_sha256` as capture binding and exactly public_sha256/factor_sha256/launch_sha256/binding_sha256 as the four source hashes, with the bound capsule/manifest receipt. A future `cell` command should require this admission receipt and verify its exact source files; it should not synthesize a trusted receipt from arbitrary CLI hash strings. Original-map-to-equation replay remains a separate obligation.

The current patch only registers the helper. These retention fields, call, dedicated F4/owned-search replay admission, and exporter projection are proposals here, not implemented or typechecked claims.
