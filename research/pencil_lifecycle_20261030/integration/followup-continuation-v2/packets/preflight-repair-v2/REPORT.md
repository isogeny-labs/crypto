# Staged preflight repair v2

The standalone structural-binding core passes29 native tests: eight session/context regressions, one real nine-plan inventory/corruption test, and20 unchanged canonical codec/SHA tests copied from the repository. The bounded inventory CLI passes eight process checks. The typed root integration proposal is separate and has NOT been compiled by this agent. No runtime admission, actual owner observation, scientific session, build publication or cloud mutation is established.

## Stages and repair scope

Static inventory checks all nine original plan byte pins, corresponding prepared-spec byte pins, and exact reviewed enrollment certificate/request/receipt/review hashes. It joins the eight cohort plans by name/hash/spec_id to the reviewed certificate, keeps the upper gate outside enrollment, and reports1260workloads/22680slots. Success is explicitly stage=static_inventory_only, software_bindings_complete=false, scientific_execution_admitted=false. It reuses the already reviewed enrollment, rather than regenerating the22680slot proof.

Actual-session API requires nonoptional plan/spec/host/session/preparation/launch and owner-observation inputs plus a nonempty actual ordered context subset. It does not demand nonexistent future sessions or contexts. The core checks exact preparation schema/field inventory and omitted clock_id, Linux clock type/rule, clock metadata, three numeric ordered observations, null unknown resource metrics, joined plan/spec/host/launch bytes, derived ECBS1 identity, runner PID, owner clock/byte observations, session status/counts and exact context leaves/order. Unknown/missing preparation fields and self-consistently altered session labels are rejected.

The root wrapper additionally parses actual Spec, Session, HostCapsule and DiagnosticLaunch; derives Spec::identity_view; invokes launch.validate_spec; crosschecks host build binary and environment class through host::recompute_class; constructs each expected envelope through unchanged prepare_lifecycle_child; compares the full typed envelope; and computes journal capacity with the existing serializer helper using actual identities. Only resource_journal_capacity visibility changes to pub(crate); no existing validation is weakened.

OwnerObservation is an input from an independently authenticated custody verifier, not authentication by possession of self-declared JSON. The structural core cannot establish that provenance. Both API stages retain scientific_execution_admitted=false and explicit external custody/budget/protocol/image/timing/reuse conditions. env_class_id is recomputed through the existing public API; this API does not expose the full environment-class digest for separate recomputation.

## Review-driven additive changes

V1 private proposal and review remain unchanged. Root review identified missing typed-host joins, absent validate_spec and incoherent/null preparation observations. V2 adds host schema/build/environment joins and class recomputation, existing spec-policy validation, and all-three numeric ordered observations as required by actual runner publication. Positive synthetic fixtures now use numeric start/acquired/execute values; new mutations reject missing/null/string/partial or reversed observation states and host/session identity drift. Synthetic owner/session fixtures establish rejection behavior only, not actual custody.

## Executable checks and adapter boundary

Private Cargo package uses exact repository canonical.rs and hash/sha256.rs, existing offline dependencies and isolated target directory. Core tests never invoke a solver. Full-nine static inventory tests read and hash real retained artifacts without rebuilding plans. CLI opens each file once with O_NOFOLLOW|O_NONBLOCK, rejects nonregular files, bounds request1MiB/input32MiB/total64MiB, hashes the bytes it parses, validates typed deny-unknown request schema, and invokes the static checker. Eight process checks: full inventory success; wrong request hash, all missing, wrong stage, unknown field, duplicate route, bad certificate pin and symlink rejection. There is intentionally no standalone actual-session CLI claiming the root typed checks; that adapter still needs integration with the reviewed root API.

## Delivery and remaining qualification

proposal/ contains the three exact root module files. V1 integration.patch adds modules and changes capacity helper visibility; v2-additive.patch contains review-driven revisions. Root integration paths are src/cryptanalysis/ecbench/{diagnostic_preflight,diagnostic_binding_core,diagnostic_inventory}.rs, mod.rs and runner.rs. Root's existing target/release/deps/libcrypto_lib-daf9fdac4ff20898.rlib and target/debug metadata can be reused; root library recompilation is still required. No ready release/ecbench executable was present at inspection.

Remaining: independent source review of v2; actual typed-root compilation/focused tests and full source qualification as required; root actual-session adapter; authenticated owner-custody verification and actual runtime evidence. The scientific envelope remains25200 active-worker seconds through2026-10-31T06:00Z subject to its original execution conditions. This package fills software prerequisites; it does not count as any of the required1260 scientific targets.
