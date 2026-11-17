# Backend report artifact pointer repair

Main commit `0302de6ba9fc414d76cb96f5bcf7c52a2c9cd6c6` failed
[backend-leaderboard job 122436099275](https://github.com/isogeny-labs/crypto/actions/runs/40574191858/job/122436099275)
because the hydrated `leaderboard.json` was stale. The checked-in reproduction
lock expects SHA-256
`16b0fd0e9a68483556b6d2e714de96e25004dfd5c423398bf74b3171d30ce6b5`,
while the artifact manifest still selects
`074eafcf56e144fae0668391f2115be4c5d40cd101c1fb1e9191521391cb06be`.

The existing producer was rebuilt with pinned Rust 1.99.1 and run **without
`--seal`**. It checks all 115 locked inputs and all expected output hashes before
writing. All five generated files exactly match the existing lock. Only the
ignored JSON report needs its artifact pointer repaired; the four tracked views,
reproduction lock, producer, raw sessions and historical receipts are unchanged.
The report contains 8,451,778 bytes.

The existing trusted-base artifact-autofix workflow uploads the exact generated
file to the content-addressed store, replays it and commits its pointer. This
repair uses that ordinary migration path, with no artifact-policy exception.
The initial PR temporarily carries the generated file for that migration;
successful pointer migration and fresh current-head checks remain necessary.

Validation: all 24 producer tests pass, `--check` passes, and `reproduce` succeeds
with the SHA-256 of the unchanged reviewed lock. The exact expected population
remains 17 sessions, 626 entries, 464 count-eligible entries, 170 planned cells
without measurements, 513 failed samples, 180 IC frontier memberships,
123 cross-method frontier memberships and zero timing winners. This is report
reproduction; no solver or timing experiment was executed.

Sources: the failed hosted job, the reproduction lock and artifact manifest,
`tools/backend-leaderboard/src/main.rs`, and the R0/R1/R2 distinction in
[`BACKEND_REPRODUCTION.md`](ecbench/BACKEND_REPRODUCTION.md). Raw validation
output and the report-only reproduction receipt are retained in
`ci-repair-backend-pointer-20261027/`.
