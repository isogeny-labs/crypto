# Owner decisions for the open leads of PR #3651 (2026-11-08)

The owner's answer, verbatim: "Yes to all open leads" (session of 2026-11-08, after PR #3651
merged). This file maps that answer onto each open decision. Where an item offered several options,
the option taken is the one the study's README recommended, or the conservative one, and it is named
here. These are records of a decision, not results.

## Item 1: PKM fall-degree study (`research/pkm_falldegree_unblock_20261107/`)

| # | open decision | taken |
|---|---|---|
| 1 | Mac files versus a median check | A fresh stage A re-run from t = 1, with the t <= 10 repeat check at **median level** against 3,4,4,4,4,5,5,5,5,6. The Mac files are not reachable from a cloud session, so the row-level upgrade is not available. The preregistration already requires the fresh re-run. |
| 2 | ECR push | **Authorized**, by the launching identity, for one digest-pinned study image built from a committed revision with `image/build_layer.sh`. |
| 3 | Spot reclaim | A reclaimed non-resumable cell is **reported as lost**. It may be relaunched once under a new job name, and both attempts are kept. CRIU command mode is not qualified for these jobs. |
| 4 | Time and dollar ceilings | `seconds` as proposed in `requests/*.json`, which is at most 176,400 s per job. Dollar ceiling: at most US$40 of Spot compute per job, US$400 for the study. Jobs run serially, one 14-vCPU job at a time (pool limit). |
| 5 | Large host for m = 4 t = 5 c = 7 and m = 4 t = 6 (80–510 GB) | **Allowed in principle.** The cluster's cost controls (`cpu-spot` 16 vCPU / 128 GiB; quota 128 GiB) do not admit such a host today, so this needs a capacity change. That change is prepared and reported separately; it is not forced into the existing pool (Amendment 8). |
| 6 | Stage B m = 6 t = 6 rule; Amendment 8 null cells | Each is written as a dated protocol amendment **before** its cell runs. A cell with no frozen amendment does not run. |

### Later owner decisions on item 1

| # | date | question put to the owner | answer | what it authorizes |
|---|---|---|---|---|
| 7 | 2026-11-10, about 05:25Z | "Two decisions are still waiting for you: a third attempt for the lost t = 13 draw 0 cell (I recommend yes, at 56Gi), and stage B." | "Yes" | **One** third attempt of cell `pfd-x-t13-k0-c6` (t = 13, draw 0, cap 6), at 56Gi under Amendment 14, under a new job name. If it is lost again the cell goes back to the owner. Read by the main session as answering the question that had a yes/no recommendation attached. It is **not** read as choosing among the stage B options of `STAGE_B_LOST.md`; that question is put to the owner again, separately. |
| 8 | 2026-11-10, about 15:40Z | "Still waiting on you: stage B and the m = 4 jobs (split into short jobs, keep relaunching, accept the loss, or hold for a capacity change). I recommend splitting." The option had been described as: "I write a dated amendment that splits stage B, and the m = 4 work where possible, into shorter jobs by stage or cell. Readings and rules stay as registered, and I get it reviewed before freezing." | "Yes" | Stage B is split into short units under a dated amendment, reviewed before it is frozen; readings and rules stay as registered. Read by the main session as agreeing to the recommendation in the question's last sentence. For the `m = 4` jobs it authorizes a split only where one is possible; each `m = 4` job is a single closure, so nothing is split there and they are not launched under this decision. |

## Item 2 follow-ups (`research/prime_medium_tier_rho_20261107/`)

- **Independent validation.** A validator independent of the producer re-checks the six sessions: replay certificate, validator report, matched-rho reference. The verdict stays `inconclusive` whatever the validator finds. Validation does not turn the run into a leaderboard row.
- **P4 pooling.** The completed run is not re-analysed with a pooling rule chosen after seeing it. A new confirmatory protocol is frozen before any new data: a predeclared pooled estimator for the rho-plain / rho-neg ratio, new seeds, and a sample size from a stated power calculation. It is then run on EKS.

## Items 3a, 3b, 3c and 5: study images

ECR push is authorized for each study's digest-pinned image, built from a committed revision. Each
runs under its own frozen protocol through `isolab.eks.command-request/v1`, by director card and
executor.

## `main` defect: `evidence_audit_pr` on `research/curve25519_f1_walk_20261107`

Fix it on `main` through this branch's next PR with the least invasive of the three proposed fixes
that works with the artifact store. The study's evidence is not edited, and no session is relabelled.
