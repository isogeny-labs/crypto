# Publication dry-run plan

The dry run operates on a copy of the result branch after the evidence bundle
passes `check`. It never edits generated HTML/Markdown/JSON by hand and does
not turn a plan or a blocked run into a row.

## Prospective sequence

1. Re-run the native publication gate against the final locally hash-bound
   bundle. Stop on exit 2 or 3.
2. Run the session audit/replay-all and the independent receipt checker again
   from the final revision.
3. Build/check the bound record or challenge verdict with native ECBench. A
   comparison alone is not a frontier move.
4. Regenerate the bounds frontier from committed records, then run its checker.
5. Add the frozen source to the canonical IC leaderboard generator, regenerate
   all twins, and run its `--check` mode.
6. Regenerate the lab browser from committed inputs and run its `--check` mode.
7. If and only if the bundle contains a new numeric result, append one
   source-backed point to the canonical progress timeline; regenerate the
   scoreboard embedding/table from that file.
8. Run `node scripts/check_ic_dashboard_coverage.mjs` over the complete inputs.
9. Run the benchmark-record gate, experiment-export verification, large-file
   policy, PCC prose lint, and repository CI checks.
10. Run `authorize` with a path that does not exist. Commit the resulting
    content-bound authorization beside the evidence; never use it across a
    changed manifest or source revision.

## Required views

| Result kind | Generated views owed |
|:--|:--|
| cold restricted-family IC/rho row | bounds/frontier, scoreboard progress, best-of row if changed, IC leaderboard, browser |
| rho/component improvement without E2E row | performance/component section, progress point when numeric, browser if identity/session changed |
| solver census only | live coverage report and browser; no priced frontier row |
| null, timeout, regression, or blocked campaign | run/result report and visual; dashboard exclusion reason when applicable |

## Dry-run acceptance

- every generator reads only committed frozen sources;
- prior figures remain as `was` rather than disappearing;
- ratios use cold one-target `S`/M-eq and strongest matched rho;
- pricing at `0,0.1,1` is present on both arms;
- restricted-family wording travels with every GHS row;
- no generic-curve or ECC2K-130 inference is added;
- all generated check modes and dashboard coverage pass; and
- the authorization manifest hash matches the exact committed bundle bytes.

Until these conditions are observed, the corresponding publication fields
remain false and the authorizer refuses output.
