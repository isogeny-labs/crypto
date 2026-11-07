# Original-owned Boolean search transport source review

Reviewed original_boolean_search.rs SHA256 1a73820a6efdffcd60b7da1542e30fe554bf77544cf3b915a54684dd45c1c19a, the registered scheduler/search/checker/bridge imports, worker-transport.patch and the exact staged source APIs named in SOURCE_BINDINGS. This is a source/API audit, not a successful assembled compilation. Independent hash and patch-application preflight belongs to dataset_inventory.

The consuming boundary preserves the actual original Budget, Math, checked/unchecked map, publication acknowledgement, source identity hashes, capture limits and sink. It moves CheckedMap directly into the staged existing expand_owned API; no raw-polynomial source constructor or reissued owner appears. OwnedExpansion exposes the exact methods used. Child-module access to private parent imports/fields is valid Rust descendant visibility; the exported types are restricted to the enclosing solvers module. Caps and constituent limits are Copy as required. No concrete compile/API mismatch was found by this source inspection.

The staged build_map path does not begin a native phase, so Setup -> Construction -> Constructed -> Matrix -> MatrixComplete is consistent. Every Budget admission checks the construction deadline while in Setup/Construction and original deadline afterward. A local search cap cannot silently become a successful audited candidate: candidate access requires successful search and audit plus candidate_valid. Native source retention survives expansion failure; search/audit results move into ownership before error handling. Prior stop reasons remain sticky through Budget.fail. Normalization changes term order only, retains generator order and zeros, and pays for copied storage and comparisons.

Worker output moves the attempt into residual custody. The parent adopts that custody before returning its report-unavailable error. It emits no solved result or durable publication claim. No dispatch was added, so this is a transport integration proposal awaiting genuine compile and parent-return tests.

## Open accounting policy question before activation

Budget.begin_native documents a native phase ceiling; Budget.observe enforces it and accounts native versus checking reservations separately. The proposal instead routes expansion, normalization, native search and audit into reserve_checking. Expansion and search receive separately clamped local caps, but normalization and audit are outside those local caps. Therefore total processing during one phase can exceed that phase's advertised byte ceiling while staying within the global Budget memory/work ceilings. The native ledger remains zero for these operations.

This is not a demonstrated global budget bypass: every allocation/work quote is still charged to the same global owner, and deadlines remain enforced. Whether normalization/audit belong to adapter overhead outside the native ceiling is a policy decision. Before activation, explicitly document this accounting classification or use a cumulative phase admission wrapper that retains phase charges and enforces the intended ceiling across the intended operations. Do not silently reinterpret or raise original caps. No speculative patch is supplied because changing the classification without that decision could double-charge or alter established limits.

## Required next integration checks

Compile the exact assembled modules and test genuine owner/map/sink creation, lexical token scope ending, move through WorkerSolveOutput and parent adoption, failure after each reservation boundary, same ownership_id/deadlines, and construction/matrix boundary accounting. Standalone donor tests do not substitute for these checks. Retention is diagnostic ownership rather than a complete restartable checkpoint; local scratch limitations remain documented.

No production/frozen source edits, full build, cloud operation, dataset execution or Conductor operation occurred in this review. No concrete source correction beyond the accounting decision is requested.
