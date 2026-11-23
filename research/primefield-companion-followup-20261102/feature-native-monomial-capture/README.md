# Bounded capture and protocol binding

The revised comparison-protocol.draft.json validates against the preserved research-package tool/schema. Instrument commit b0b8a151ca4df04ee0404fb5152c50fce20724a6 was read back from GitHub; binary and source-lock hashes are bound in source-binding.json. The earlier draft remains immutable. This draft is not frozen: required preflight is absent, and no budget/capacity admission exists.

`capture.py --runner PATH --protocol FROZEN --admission RECEIPT --out NEW_DIRECTORY` checks the frozen protocol hash, expected binary, admission status/hash/deadline and one-attempt envelope. It executes only the fixed --run-admitted argument. The receipt is operator-supplied evidence, not a signature or an authorization mechanism. EKS resource, budget and single-job enforcement remain external requirements.

Before launch it exclusively creates the attempt directory and writes attempt.json. It captures stdout/stderr to files, kills the process group on timeout, and writes a separate receipt with exit code, elapsed duration and hashes. The directory cannot be reused. Empty output on timeout is valid custody, not a completed measurement. If the wrapper itself is killed, the prelaunch marker and partial files remain; reconcile Job/Pod state before any retry. No retry is built in.

`export.py DIRECTORY --protocol FROZEN` handles successful complete captures only. It requires matching raw hash, successful non-timeout receipt, protocol/binary bindings and the strict65-record parser. It emits schema-valid experiment.export/v1 with60 target/arm rows and17 separate numeric dimensions, adding full fixed costs to each query. No wall-time claim or host isolation is invented. Failed/incomplete attempts remain raw custody records and are refused by this success adapter; an explicit failure-flow adapter is still required before panel launch.

Tests use synthetic rows and Python subprocess stubs only. Ten methods pass, including imported strict-parser controls. Initial export-schema and timeout-fixture failures are preserved. Run with the project venv; the system jsonschema module lacks Draft202012Validator. Canonical hashing matches the bundled tool on finite protocol data. The full20-target native mode was not invoked. Original4563-case cohort unchanged.

Read-only EKS overview listed no monomial job; listed workloads were terminal except the existing suspended SAT Job. Compiled-in pool limits do not establish live owner-policy compliance. No resources or jobs were changed. Remaining: preflight, failure-flow export, protocol freeze, capture/export publication bindings and authoritative budget/capacity admission. Proposed USD1 ceiling is not reserved money.

Sources checked (KB): tools absent and wrapper unavailable in predecessor round; read matched instrument/analyzer and preserved protocol tool/schema. No mathematical or performance claim.
