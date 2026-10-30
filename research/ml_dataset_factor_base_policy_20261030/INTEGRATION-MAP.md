# Required v2 integration after standalone implementation

This is a source-read map, not completed v2 CLI support. Preserve the full
contract while shared mod.rs/solvers.rs remain reserved by T-887.

| Surface | Existing source boundary | Required v2 behavior |
| --- | --- | --- |
| plan/preflight | mod.rs plan construction and direct DatasetPlan reads | Optional policy flags/file; omitted flags retain v1 bytes; explicit selection emits strict v2; report policy blocks and caps. |
| generate and manifests | mod.rs generate, CellManifest/config.json, worker args | Bind origin and exact policy in versioned occurrence/manifest; preserve old config bytes and old run/cell hashes; policy is outer index block. |
| worker construction | mod.rs worker curve-construction/replay phases | Dispatch generate_with_policy; retain restricted construction artifact and full expected-policy/request replay before solver; charge setup/replay separately. |
| solver domains | solvers.rs domain preparation and every native adapter | Consume exact verified points/x lift maps. A policy cannot silently select a different native domain or widen supported family/arity. |
| run binding and resume | mod.rs bound_run, reconcile, cell verification | Strict original-byte plan parser; v2 plan hash and case/policy binding; reject substituted policies/configs and preserve retry identity. |
| protocol/budget | mod.rs verify_protocol and launch grant construction | Carry complete normalized axes/policies and actual cell count; never hash only nested v1 axes for a v2 run. |
| start | startup_gate.rs decode of frozen plan/preflight binding | Verify entire v2 plan/hash and exact policy-bearing launch; existing v1 commands unchanged. |
| status/reports | report.rs survey and CaseConfig axis aggregation | Distinguish policy occurrences and actual population sizes/caps; retain unsupported/failed cells; no success inferred from construction. |
| export | export.rs verified-cell projection and strict input allowlist | Exact population/action semantics only; new policy-context projection version; restricted candidate/log/cost/provenance remains excluded; preserve old v1 exported bytes. |
| replay/archive | immutable artifact registry, storage and S3 receipts | Hash/role-bind policy definition and restricted construction; semantic rehash controls; same conditional publication/readback path. |

Acceptance needs both legacy no-flag and explicit-v2 legacy plan runs, signed
orbit runs on curve and twist, key/generator/m/backend construction-independence
checks, caps/unsupported outcomes, wrong policy/ordering/action/stabilizer and
leakage controls, immutable export retry, and source-bound full required checks.
The constructor's mathematical verifier alone cannot authenticate the expected
plan policy; worker/manifest admission must call the stronger transcript verifier.
