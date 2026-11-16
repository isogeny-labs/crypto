Integrate diagnostic_file_adapter.rs as src/cryptanalysis/ecbench/diagnostic_file_adapter.rs and add `pub mod diagnostic_file_adapter;` to mod.rs. Apply ecbench-dispatch.patch, which adds a Clap metadata-only command and dispatch directly to check_file; no runner calls. Root owns compilation and execution. Unix is required for filesystem validation; other platforms explicitly return an error.

All evidence, INCLUDING request and actual-session metadata, shares the unchanged 64 MiB total cap. Request max 1 MiB, inventory item max 32 MiB, host/session max 4 MiB, other actual files max 64 KiB. Paths and opened device/inode identities must be unique. Directory entries above the final filename are not protected against symlink traversal; file SHA256 still binds the exact opened bytes and file type must be regular. The final component is opened O_NOFOLLOW. No authenticity is inferred from the owner comparison file; every FileReport exposes owner_authentication_verified=false and external_owner_verification_required=true.

Manual retained-data coverage:
`PENCIL_RETAINED_INVENTORY=/var/tmp/pencil-preflight-typed-tests-20261031-v1/inventory-inputs.json cargo test --offline --locked --lib typed_wrapper_tests -- --ignored`
The original three tests are preserved in the earlier snapshot. Revised tests use the runtime inventory environment variable and are ignored in ordinary CI because their retained artifacts require explicit hydration. A default run filtering them is NOT a passed retained-data validation. Root must retain the explicitly selected run's terminal exit/output.

File invocation after actual integration/build:
`ecbench diagnostic-preflight --request /absolute/request.json --request-sha256 <exact-byte-sha256>`
Static request stage is static_inventory_only; actual stage is actual_session_subset_checked and includes actual:{route,host,session,preparation,launch,contexts,owner_observation}. Every file value is {path,sha256}. Owner file fields are exactly OwnerObservation fields, with no claimed verified flag accepted. All schema objects deny unknown fields; duplicate keys are rejected before parsing.

The adapter has three proposed always-on native tests for exact hashes/aliases, no-follow/type/caps, and strict tagged request rejection. This packet has not been compiled; root owns the cached build. No scientific work was executed.
