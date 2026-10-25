# Execution plan and delegation (follow-up study)

The science is defined in `PLAN.md`; this file says who runs each part and in
what order. It follows the repository's model-tier rule (AGENTS.md, "Model
tiers"):

- **core** decides, reviews gates, and handles the PR and its merge;
- **director (Sonnet, `experiment-director`)** compiles decided measurements
  into `tools/task_card.py` cards and assesses each run;
- **mechanical (Haiku, `executor-mechanical`)** runs a card through
  `tools/task_card.py run` and relays its output verbatim, computing nothing;
- **Sonnet executors** do the work that needs judgment but makes no claim:
  search, spec design, the analysis code, the report text;
- an **independent validator** does validation.

Everything after Phase A runs as one workflow, a deterministic script of agent
calls, so the order, the gates and the retry rules are fixed in code rather
than chosen as it goes.

| # | step | tier / agent | input | output | gate (who decides) |
|:-:|:--|:--|:--|:--|:--|
| A | Seeded search, whole walks, identity checks, specs, estimate, frozen `ANALYSIS_PLAN.md` | Sonnet executor (running already, outside the workflow) | `PLAN.md` | `census/`, `specs/`, `runs/manifest.json`, `ANALYSIS_PLAN.md` | G-A (core) |
| 1 | **Gate A review**: check every G-A item against the files on disk | core | Phase A commits | pass, or a blocker list | the workflow stops on fail |
| 2 | **Batch compile**: split the manifest, in order, into batches that each run in under 90 min; one task card per batch | director (Sonnet) | manifest, census driver scripts | `runs/cards/batch-NN.json`, each passing `task_card.py lint` | lint must pass |
| 3 | **Run batch NN**: `task_card.py run`, then archive, `xz -t`, prune, progress line | mechanical (Haiku) | the card | sessions, archives, extracts, progress.jsonl | none (relays only) |
| 4 | **Assess batch NN**: audits OK, counts match the card, wall and disk guards | director (Sonnet) | the run's output on disk | ok, retry once, or stop | stop on a guard or after 2 failed tries |
| 5 | **Transfer-cost card**: `ca volcano --transport-bench` on the V5 and V6 paths | director, then mechanical, then director | census paths | `runs/transfer_cost.jsonl` | assess |
| 6 | **Drive uploads**: plan files, progress summary, report text | Haiku | listed text files | `DRIVE.md` rows | size read-back |
| 7 | **Gate B review**: every session audited, archived and pushed; counts reconcile | core | progress.jsonl, git | pass or blockers | the workflow stops on fail |
| 8 | **Analysis**: `analyze.py` to the frozen plan, results.json, figures, REPORT.md, PDF, benchmarks copy (no README row) | Sonnet executor | frozen `ANALYSIS_PLAN.md`, extracts | `analysis/`, `REPORT.md`, `report.pdf` | none (the validator judges) |
| 9 | **Independent validation**: replay sample, own-code recompute, report checks | `validator` agent | snapshot SHA | replay certificate, validator report, `VALIDATION.json` | pass or fail |
| 10 | **Erratum loop** (at most 2 rounds): on a fail, a Sonnet executor fixes only what the validator lists (AMENDMENTS A-n), then step 9 re-runs | Sonnet executor, then validator | validator findings | erratum commit | the validator |
| 11 | **Final review**: the headline, the README row text and the PR description | core | results, validator report | the README row committed, a PR description draft | merge waits for the owner |

## Rules every step follows

- Measurement steps run in sequence. ecbench sessions serialize on one lock,
  and disk is about 1.4 GB, so nothing measures in parallel.
- Guards: 20 h of session wall, under 1.5 GiB free disk, or 3 failed audits in
  a row stop the campaign. A stop is a result, reported with its denominators.
- No step builds with cargo. The frozen ecbench binary (`ac4bc5b2…`) is
  checked before each batch.
- Steps commit only their own files and push to
  `claude/volcano-census-followup-20261023`, never forcing.
- Merging PR #2435 needs the owner's go-ahead.
