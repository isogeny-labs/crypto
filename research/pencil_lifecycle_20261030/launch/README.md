# Diagnostic launch binding slice (T948)

This is software integration preparation, not a trial, benchmark, execution admission, source attestation, or completed instrument. No scientific outputs are produced. Existing scientific scope and stop limits are unchanged.

The source is `src/cryptanalysis/ecbench/diagnostic_launch.rs`. Parent integration owns module registration, RunOptions, CLI, child fields and validation builds. This task does not edit those surfaces. It preserves the existing v1 prefix Context and adds a separately named v2 envelope.

## API and trust boundary

`DiagnosticLaunch::load(&Path)` reads at most 65,537 bytes and refuses documents exceeding 65,536 bytes. `from_bytes` has the same parse boundary. Recursive duplicate keys and unknown fields are rejected. Private raw JSON and its SHA-256 survive serde RunOptions round trips; every `document`, `validate_spec`, `validate_plan` or envelope validation rechecks raw integrity and schema.

`validate_spec(&Spec, spec_sha256, actual_binary_sha256)` must run before plan expansion. The caller must compute the spec digest with existing `short_id("ECS1", &spec.identity_view()).1`; the argument is not a raw-file digest. A missing actual binary digest rejects. It checks exact ordered arm names, roles and registry identifiers. `validate_plan(&Plan)` subsequently binds the SHA-256 of the full ResolvedMethod serialized JSON Value, matching the existing prefix reader. This is deliberately different from the internal `ResolvedMethod.method_sha256` identity. `resolved_method_binding` implements the shared convention. It does not replace the earlier actual binary check.

Only the existing frozen pencil/candidate, rho/reference and rho_AA/control arms are supported. All three must use identical profile version `ecbench.all_arm_lifecycle/v1` and profile digest. This promises the same lifecycle interface, not equal IO cost or completed all-arm activation. Method IDs follow the frozen continuation protocol; both rho arms intentionally share a registry identifier.

The document carries protocol, source manifest, external source receipt, profile and binary digests, explicit scientific run ID and attempt. Source scope is exactly `external_manifest_binding_only`. These are externally supplied bindings. This parser neither reads the original receipt nor authenticates its author nor attests that a binary was built from those sources. The separately trusted parent must select the reviewed receipt; no arbitrary input document becomes scientific authority merely by parsing.

`ChildEnvelope::validate(&DiagnosticLaunch)` binds unchanged prefix identity metadata to the exact parent launch, profile and arm. Parent must additionally associate session, execution, workload, method and seed with the actual scheduled child. Directory root ownership and lifecycle sink activation remain outside this parser; no path from the document controls output directories.

## Verification boundary

Seven synthetic test functions are included; none calls a solver, rho, workload generation, plan expansion, or child algorithm. Tests cover raw-hash custody, schema bounds, duplicate keys, frozen arms, profiles, source scope, pre-plan identity matching, resolved configuration matching using empty vectors, and v1/v2 envelope compatibility. They are intentionally not run by T948; parent owns registration and the next build. No passing result is asserted here.

## Sources checked

- Existing `canonical.rs`, `diagnostics.rs`, `spec.rs`, `methods.rs` MethodSpec/ResolvedMethod, and host BuildFacts interfaces.
- Frozen continuation protocol in `research/pencil_large_prime_20261029/protocol_draft/full_cohort_continuation_20261030` for exact three-arm method names and roles.
- Private all-arm lifecycle/cost patch specification, used as implementation design, not evidence.
- Serde official container attributes: https://serde.rs/container-attrs.html (deny_unknown_fields; not combined with flatten).

No novelty or performance claim follows from this software slice.

## Child transport follow-through (delegated root T954)

`record::DiagnosticChild` carries the exact launch, separate v2 envelope and runner-owned absolute output root. `ChildInput.diagnostic_lifecycle` is an optional new field; old inputs omit it. V1 context rejection remains unchanged; mixed v1/v2 inputs reject. The child boundary rejects duplicate keys and overlarge diagnostic input and revalidates the launch, envelope, scheduled algorithm seed and resolved method/full-record-method digest before any workload construction. Output custody checks canonical absolute paths, no symlink components, owner-only0700 root owned by the child uid, and no unprotected writable ancestors. A same-UID adversary is outside this metadata check; the same uid can replace the binary/input pipe.

`with_diagnostic_lifecycle` is a generic integration seam called only by synthetic callbacks in this slice. It installs the same common lifecycle scope on every arm and the separate prefix scope only on pencil. It attempts both finishes on ordinary callback return/error; panic drops clear scoped ownership without inventing completion. Writer totals remain separate from algorithm counts/cost. An external caller still owns the complete enclosing accounting and publication-tail receipt. Child_main remains disabled for v2 after validation, so this seam alone cannot admit an experiment.

Five child synthetic tests cover optional serde compatibility, explicit pre-workload rejection, seed/method/mixed-context/path failures, symlink rejection, all-arm fake callback with candidate-only prefix ownership, no overwrite, error status and panic cleanup. Curve fields in those fixtures are deliberately invalid and never built. No tests or builds were executed by this task.

Changes in audit.rs and examples/ecbench_child_overhead.rs add only None to existing ChildInput literals. Root owns matching runner/CLI literals, module registration, compilation and eventual reviewed activation. T951 was voluntarily released by root and succeeded by T954 with the same software scope; this does not reset any scientific allowance.

### Workload binding follow-up

`DiagnosticChild::validate_workload(&Workload)` hashes the full serialized Workload JSON Value, matching the existing parent reader instead of the internal workload identity digest. The ordinary child path invokes this after its existing rebuild/expected-ID check, before method resolution and hooks. The earlier v2 disabled gate remains. A sixth child fixture constructs invalid serialization-only Workload fields directly (no build), accepts matching full-value bytes and rejects a changed target seed. The generic fake-callback seam remains metadata-only; real child activation must preserve this workload check.
