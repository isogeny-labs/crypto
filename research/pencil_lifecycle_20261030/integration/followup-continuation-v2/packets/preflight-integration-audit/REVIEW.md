# Integrated preflight source audit

This is an integration audit, not an independent producer review: I authored the earlier binding core, inventory, typed-wrapper proposal and file adapter. The existing prospective root PLAN.md at research/pencil_lifecycle_20261030/integration/followup-continuation-v2/preflight-review governs the audit. Exact reviewed bytes are retained under snapshot/ and bound by source-hashes.json. No root build was started and no running tests were duplicated. This report does not assert results from root's still-running release tests.

## Dispatch and side effects

The new Clap DiagnosticPreflight branch calls diagnostic_file_adapter::check_file and print_json, then returns through the ordinary command path. It does not invoke run_session, host capture, plan/workload generation, child spawning, cloud APIs, file publication, or lock acquisition. The existing binary startup registers closures in OnceLock; those solver closures are not invoked by this path. Spec::from_json parses/schema-checks only. prepare_lifecycle_child constructs metadata only; its synthetic output_root is never opened. SessionResourceJournal::begin constructs a value, and resource_journal_capacity serializes an upper-bound model without starting work. The observable intended effects are input reads and report/error stdout/stderr (ordinary filesystem access-time updates may occur).

## Exact bytes and input policy

The request's supplied SHA256 is checked against the bounded bytes parsed; request JSON duplicate keys are rejected. The internally tagged stage enum rejects unexpected fields, preventing an actual-session section from being silently ignored by static mode. Each file is opened once with O_NOFOLLOW|O_NONBLOCK, checked as regular, bounded while reading, and hashed before typed parsing. The 64 MiB aggregate budget includes the request and all actual metadata. Per-file caps remain enforced. Duplicate paths and repeated opened device/inode identities are rejected; malformed pin hashes cannot match computed lowercase digests.

This is not a sandboxed path resolver: caller-provided absolute paths and parent-directory traversal are allowed, and O_NOFOLLOW protects only the final component. No path is executed or written. Intermediate symlink substitution can alter the opened inode, but cannot substitute different accepted bytes without matching the pinned SHA256. The implementation supports the stated local trusted-request exact-byte workflow; do not describe it as directory confinement or general hostile multiuser filesystem isolation.

## Typed joins and stage boundary

Inventory pins all nine retained plan/spec byte pairs and four enrollment artifacts, enforces unique route/artifact names, checks the certificate's eight cohort joins, excludes the upper gate from that enrollment, and preserves 1,260 workloads/22,680 slots. It returns software_bindings_complete=false.

Actual-session validation selects one pinned route, parses real PlanDoc/Spec/HostCapsule/Session/DiagnosticLaunch types and derives spec identity using Spec::identity_view. It checks host schema, host/session/launch/owner binary equality, host/session environment class equality and recomputation, launch exact spec/ordered arms, and every configured arm's resolved method digest. Core joins exact host/plan/session/preparation/launch bytes, ECBS1 derivation, runner PID and owner clock comparison fields. Required observations are numeric and ordered, and preparation/lock intervals are checked. For each supplied actual context, the wrapper compares the typed envelope to the actual metadata producer using session identity, enrolled execution, arm and workload, then enforces ordered unique sequence/leaf keys. The actual context list is a nonempty subset; nonexistent future sessions are not required. Serializer-aware journal capacity uses actual string identities and all plan slots.

The existing host API recomputes env_class_id, not the capsule's separate env_class_sha256 text. That extra full-digest field is not validated here; neither source nor review should claim otherwise. Exact host bytes remain bound. Core accepts running/complete/interrupted status with partial bounds; it is not a comprehensive post-run session auditor.

## Custody and admission

OwnerObservation is explicitly comparison data. Its file schema rejects a fabricated extra verified flag, but possession of matching JSON cannot establish authenticated ownership. FileReport unconditionally exposes owner_authentication_verified=false and external_owner_verification_required=true. All reports keep scientific_execution_admitted=false, with authenticated custody, budget/deadline, prospective protocol, qualified published image, timing and p739-reuse obligations external. Actual software_bindings_complete=true therefore describes the structural joins only. An admitting controller must not treat either successful CLI exit or that field alone as permission to launch.

## Findings and validation disposition

No blocking integration defect found in the scoped source inspection. No code changes are proposed by this audit. The file-adapter tests and manually hydrated retained-data tests remain the concrete dynamic validation required from root's current build; their source presence or default ignored status is not a passing result. Source snapshots/hashes make subsequent changes detectable. All compilation, runtime qualification and scientific admission decisions remain separately evidenced.
