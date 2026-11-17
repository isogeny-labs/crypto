# Source review of corrected preflight proposal v1

Reviewed frozen diagnostic_preflight.rs SHA7581712048ae1e7ccfda21a78035b9cd3d8818edd1757f674dbb2de05cf6058f and companion core/inventory under the prospective review plan. The staged API, exact nine-plan enrollment pins, actual producer context recomputation and duplicate/leaf checks address the previous proposal's main structural gaps. Standalone tests do not compile the typed wrapper.

Additional concrete bindings required before integration qualification:

1. Parsed HostCapsule and Session were discarded after type validation. Join actual host.build.binary_sha256 with session/launch/owner; check host schema, session environment class and recomputed host class identity.
2. Invoke the existing pure DiagnosticLaunch::validate_spec rather than only duplicating an ordered-plan arm subset of its checks.
3. Actual-session preparation must contain three numeric ordered observation offsets. runner.rs sets lock_wait_started at970, lock_acquired974 and execute_entry1095 before publishing preparation at1150+. All-null or partial observations in the standalone positive fixture are not the actual prepared-session state. Require start<=acquired<=entry and retain wall/difference identities; mutate missing, partial and reversed observations separately.

Sent these findings to producer for an additive revision; original frozen source remains retained. This review does not authenticate external OwnerObservation or admit scientific execution. Root typed integration and file-adapter checks remain separate.
