# Canonical observation packaging

`package.py` packages existing native mathematical-study outputs into `experiment.export/v1`. It does not run a solver, recompute prices, freeze a protocol, create a review, or certify a clock. It uses the canonical protocol module/schema and retains its validation findings.

The request is `pencil-canonical-package-input/v1` with `export_id` and `{path, sha256}` bindings for `canonical_tool`, `protocol`, `native_analysis`, `analysis_script`, and `execution_history`. Paths are explicit filesystem paths. `native_analysis` is the unchanged `pencil-price-cohort-output/v1` produced by `analyze_cohort`; its embedded input manifest binds the prepared receipt, plans, raw records and optional resource sidecar. `analysis_script` is the source pinned by protocol `analysis_plan.script`, currently `native_price/src/bin/analyze_cohort.rs`. Optional `additional_sidecars` is a list of hashed bindings for session, host, controller/checkpoint, replay, isolation and other retained receipts. This adapter and the canonical helper must be pinned in protocol `extensions.canonical_packaging` by `adapter_sha256` and `canonical_tool_sha256`.

```
uv run --no-project --with 'jsonschema>=4' python research/pencil_large_prime_20261029/canonical_export/package.py request.json --out export.json
```

The adapter refuses an unfrozen or invalid protocol, changed hashes, source/mapping/binary/protocol identity disagreement, changed record-to-plan identities, duplicate records, and incomplete plan sets. Output creation is exclusive. Missing record slots are represented explicitly in `backend_record.missing_slots`, never fabricated as attempted runs. A schema-valid partial export is written with canonical shortfall findings and nonzero exit status; those findings are retained rather than suppressed.

Common cold observations come only from `time.method_call_wall_ns`; `solve_wall_ns` remains a separately named raw observation. Integer nanoseconds are retained in counters as well as the canonical seconds projection. The five online phase names are checked exactly, with integer/overflow-safe sum and explicit null diagnostics. Raw online structure, zero-stage reasons, scalar/base/native counts, failures and details remain in the hashed raw records. Analysis prices, exact rational cells, calibration components, A/A results and missing-pair nulls remain in the hashed unchanged native analysis output. No API/detail diagnostic totals are repriced or summed here. Child RSS and parent/process clocks remain separate observations; absent RSS stays null.

`wall_valid` is always false in this packaging adapter. Even a well-formed clock and clean producer flag do not establish independent isolation/admission or control review. `backend_record.diagnostics` exposes online structural defects and explicitly states that qualification is not established. Passing the canonical schema/file checks therefore certifies packaging consistency only, not comparative eligibility. Qualified ratios remain the responsibility of a separately bound independent review; this adapter does not add qualified ratios.

## Schema limits and resource history

Canonical v1 rows require `attempts >= 1` and an integer `resumed`; neither is inferred. The required hash-bound `execution_history` file has schema `pencil-execution-history/v1`, exact `protocol_sha256` and `binary_sha256`, and a `records` array. Each entry binds `record_id`, `session_id`, `seq`, explicit integer `attempts` and `resumed`, optional canonical `checkpoint`, and a nonempty `evidence` list of `{path, sha256}` source receipts. Restored records require a checkpoint policy and explicit restored generation. Every observed record must have exactly one history entry; unknown/null counts, missing records, duplicate history and unmatched history are rejected before output creation. Source receipts are hashed and attached, not promoted to independent review.

If reliable counts are unavailable, canonical export is refused; raw records, native analysis and resource sidecars remain available unchanged. Unmatched infrastructure attempts belong in the separate resource sidecar, not a fabricated canonical execution row. No default zero restore count or assumed single-attempt count is emitted. All clocks remain unqualified even with explicit history.
Canonical `flow.planned` counts represented records; the full expected slot denominator and missing slots are explicitly in `backend_record` and the native output. Incomplete cohorts retain canonical shortfall findings. The schema cannot encode an unattempted row as such without a schema amendment; this implementation does not silently change canonical semantics. Warmup and A/A arm identities remain on their observed rows. Repeated record identities are rejected; retry histories belong in sidecars rather than duplicate execution cells.

## Software tests

```
uv run --no-project --with 'jsonschema>=4' python -m unittest discover -s research/pencil_large_prime_20261029/canonical_export -p 'test_*.py' -v
```

Tests use temporary, explicitly named software fixtures with synthetic records; they do not run the mathematical cohort. The positive fixture exercises the actual canonical schema and `check-export --verify-files` subprocess. Other tests retain incomplete/failed observations and reject bad hashes, identities and mappings, and detect wrong online phase names and sums. Fixture-only frozen documents exist solely inside temporary test directories and are removed afterward; they are not research approvals.
