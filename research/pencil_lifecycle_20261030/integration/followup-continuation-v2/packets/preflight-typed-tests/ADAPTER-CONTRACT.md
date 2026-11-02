# Proposed root file adapter (not implemented by this packet)

Use a metadata-only binary entry point `ecbench diagnostic-preflight --request PATH --request-sha256 HEX`; dispatch must occur before any plan generation, runner initialization, host capture or lock acquisition. This invocation is a proposed command, not an existing runnable CLI.

Request schema `ecbench.diagnostic_preflight_request/v1` must deny unknown fields and use a tagged stage union:

* `static_inventory_only`: nine `{route, plan:{path,sha256}, spec:{path,sha256}}` and four `{name,file:{path,sha256}}` enrollment artifacts. No actual-session section is permitted. Call `check_inventory`.
* `actual_session_subset_checked`: same immutable inventory, and exactly one `{route, host, session, preparation, launch, contexts:[file], owner_observation}`; all files carry path/SHA256. Selected plan/spec come from the inventory route, preventing an unrelated second copy. Call `check_actual_session`. The context list is nonempty and ordered; it names only the actual prepared execution subset. Future execution identities are not required or invented.

Before parsing, read each declared file once through a bounded no-follow/nonblocking regular-file handle; hash the bytes actually parsed. Reject duplicate JSON keys, wrong request checksum, unknown keys, wrong stage shape, duplicate artifact/route/context paths, excessive bytes and empty context list. Retain the existing 1 MiB request / 32 MiB individual inventory / 64 MiB aggregate budgets for inventory; establish a separate explicit aggregate cap if actual-context evidence is added rather than silently lifting these caps. Serialize the native Report to stdout, with no mutation or launch. Error exit must not produce an admitting report. Every success remains scientific_execution_admitted=false.

Owner observations are comparison inputs, not authentication. The file adapter must label their external authentication status unresolved and must not convert self-asserted receipt JSON to custody verified. The admitting controller must separately authenticate the actual owner receipt, reconcile cumulative worker time and deadline, and check prospectively frozen protocol/review and qualified published artifact bindings. No new permissions or scientific execution are part of this adapter.

# Root test linkage

To test the real module, link typed_tests.rs as a cfg(test) child of the integrated `diagnostic_preflight` module and run only `typed_wrapper_tests` through root Cargo after the existing check is terminal. The sibling inventory-inputs.json is embedded at compile time; its path entries resolve the already-retained real artifacts. No copied root implementation, root dependency stubs or alternative wrapper are involved. This private packet does not claim compiled results until root supplies terminal receipts. Root integration may replace absolute linkage with a repository-owned fixture manifest without changing any pinned input hashes.
