# Drive uploads (folder 11MPowjLRt25fg1RypJnbDDvWrNc_7PZ3)

git is the copy of record (census A1.5). Size read-back = the `fileSize` the connector returned equals the local byte count.

| file | Drive id | bytes (Drive = local) | uploaded |
|:--|:--|--:|:--|
| volcano_followup_PLAN.md (PLAN.md) | 1q_HtMSo6vaGppcjDL2N2DL52-gRHBPm0 | 9075 = 9075 | 2026-10-23 Phase A |
| volcano_followup_ANALYSIS_PLAN.md (ANALYSIS_PLAN.md) | 1BN0paBc5tJGgOC2yJBFXgy9lEQls1ToY | 16686 = 16686 | 2026-10-23 Phase A |
| volcano_followup_AMENDMENTS.md (AMENDMENTS.md) | 1py2J6GBRPR6M8q-Peeb_yCiSSMgmO6lL | 4202 = 4202 | 2026-11-06 Phase B close (sha256 2e5330e233dbdc29e335dda3090d9991040db3b48056ebdfab6b88b00bdbb147, read back) |

Not uploaded on 2026-11-06: `volcano_followup_progress_summary.tsv` and
`volcano_followup_EXECUTION.md`. The Drive connector refused both creates
(auto-mode denial, "Unrequested Commit in a Connected App"). They need the
owner's go-ahead before a retry. The TSV is built locally from
`runs/progress.jsonl` (147 done rows, 9 `failed` refusal records excluded).
