# Canonical contract coverage

Reference: vendored `tools/experiment_protocol.reference.txt`, functions
`lint_protocol`, `lint_export`, `run_row`, `export_skeleton`, `compute_flow`,
`protocol_hash`, and the two byte-identical schemas. No reference code executes.

| Canonical obligation | Native disposition |
|---|---|
| Complete protocol/export JSON schema | Real Draft 2020-12 validator; additional properties and nested constraints checked |
| Duplicate arms/claims/comparisons/metrics | Rejected by protocol_lint |
| Comparison arm/factor identity; one-factor, AA, confounded and absolute designs | Checked |
| AA/positive/negative control identities, roles and explicit applicability reasons | Checked |
| Claim/decision comparisons and decisive designs | Checked |
| Prediction metric/provenance and in-sample restrictions | Checked |
| Multiple-comparison correction | Checked |
| Rank-test resolution | Unsupported: rejected explicitly; not silently accepted |
| Seed rule, fixed arm-order reason | Harness-drawn rule with explicit note only; every record compared to exact plan seed |
| Explicit-list/derived/external-nonce seed tables | Unsupported: rejected explicitly |
| Frozen preflight estimate and timeout per sample | Checked |
| Timed primary justification and isolation threshold | Checked; all exported clocks nevertheless invalid |
| Derived metric formulas | Checked |
| Checkpoint interval/grace/restore validation and measurement/CRIU conflict | Checked |
| Frozen hash, timestamp, analysis hash and frozen-before-data | Checked |
| Amendments filed before affected data | Checked |
| Same-instrument commit/binary/thread equality | Checked |
| Export arm roles/binary/commit and protocol id/version/hash | Checked |
| Unique run IDs and execution cells | Checked |
| Outcome class/status, interrupted/restored clocks, warmups, contention | Every clock must remain false; status/outcome class checked |
| Exclusion rules or refuted certificates | Unsupported by this raw-outcome adapter: fail closed; not silently qualified |
| Checkpoint generations for restored rows | Required, together with explicit known history and evidence |
| Flow totals | Recomputed from observed rows; missing slots kept separately |
| Notes observation-only | Enforced by actual schema |
| Planned measured rows, per-arm instances and replicates, cross-arm instance sets | Semantic findings retained, nonzero CLI exit |
| Blinded analysis | Unsupported: rejected explicitly |
| Analysis script and all attached files | Hash and size checked; outputs, artifacts, host references also checked |
| Helper/schema/adapter provenance | Hash bindings plus retained compiled source equality; schema bytes must match embedded versions |
| Native analysis manifest/source/mapping/protocol/binary pins | Checked before export |
| Native missing count vs actual plans and observations | Checked; exact missing identities rebuilt from plans |
| History attempts/restores and resources | Known integer fields/evidence required, exact resource sidecar equality checked |
| Counters | Arbitrary-width nonnegative integer tokens retained exactly, large values as decimal strings |
| Output overwrite | Exclusive create; tested |

The first 17 contract tests correspond to the historical packager's 17 software
contracts. Additional tests cover real schema rejection, native CLI positive and
negative file checking, exclusive output, u64/u128 preservation, flow/wall
semantic rejection, protocol comparison rejection and restore-evidence gaps.
The checker is deliberately study-specific, not a general replacement for every
backend supported by the canonical helper.
