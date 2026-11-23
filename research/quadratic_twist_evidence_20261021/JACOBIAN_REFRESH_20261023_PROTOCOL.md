# PR-caused Jacobian metadata refresh

Frozen before executing the regeneration, 2026-10-23.
Starting head: `3c51d745f9b885690162d03253db69a860062e11`.

CI run 39743812835, job 119707234495 builds and passes module tests, then
rejects `docs/curves/jacobian-transfers.json` as stale. This PR added 1,921
models to `docs/curves/registry.json`; the report still covers only 2,572.
This is a generated-metadata omission caused by this PR, not an inherited
failure to patch around. The registry itself will remain unchanged.

Use the existing Rust `jacobian_transfers` producer, its unchanged cost model,
and the committed 200,000 factor budget. Reproduce the old report's check
failure; regenerate the active JSON and marked Markdown table; replay the
exact workflow checks, query/chart commands and altered-file rejection.
Verify that every one of the 2,572 prior per-curve objects is unchanged and
that the new report covers the registry exactly, with no duplicate or missing
model. Preserve before/after hashes and actual commands, errors and exits in
`validation/group-map-20261023/jacobian/`. Old dated reports remain frozen.
Render the new catalog's chart in the dated continuation report, distinguishing
formula leads from verified constructions and measurements. No new GHS,
speed, hardness, subgroup-recovery or practical-security claim is authorized
by generated formula rows.

Scope expands to `docs/curves/jacobian-transfers.json` and the generated table
in `docs/curves/JACOBIAN_TRANSFERS.md`. Neither is one of the three conflicted
paths found by a read-only merge-tree check. Do not edit the conflicted
browser, traits or leaderboard files while Copilot's request is pending.
