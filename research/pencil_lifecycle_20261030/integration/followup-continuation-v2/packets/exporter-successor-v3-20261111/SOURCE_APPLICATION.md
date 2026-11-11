# Source application boundary

Apply the original pencil-managed-stream-integration-20261110 patch first, then this integration.patch to an isolated reviewed base. This patch adds v3 beside original v1 and does not require applying the drifted v2 patch. Cargo/mod patch-base hashes are in source-bindings.json. The exact donor native-schema requirements are separate; verify_base.py must pass on the actual proposed integration tree. In particular the advertised Git-head runner does not match the pinned donor runner, so a plain Git-head tree is not an admitted base. Any reconciliation requires explicit review; no compatibility is claimed here.

No source application, helper invocation, compiler or test was performed in producing this packet. The new binary is ecbench_retained_attempt_export_v3.
