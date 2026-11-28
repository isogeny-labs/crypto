# `t14_bigmem`: design of a bigger-memory attempt for `t = 14` cap 7 (2026-11-21, not run)

Owner instruction of 2026-11-21: design, do not run, a larger-memory attempt for the `t = 14` cap-7 cell of the PKM
fall-degree study. No pod was started, no paid call was made, nothing outside this folder was changed. Every number in
these files is `[pcc:unverified]`.

| file | what it holds |
|:--|:--|
| `MEMORY.md` | per-run sizes, peaks, wall times and degrees for `t = 12, 13, 14`, caps 5-7 (`file:line`); what `--max-dense` bounds in the frozen code; the memory model; the low/central/high projection for degree 7 and the degree-8 requirement; the chosen `--max-dense` |
| `HOSTS.md` | the RunPod v2 CPU catalog (pods with 256 GB), which band fits which host, `RAYON_NUM_THREADS`, what `timed_out` meant, wall-time projection, cost per job |
| `DRAFT_AMENDMENT.md` | proposed Amendment 34 (unnumbered draft): job, stage `cap:14:0:7:54000000000`, host, memory floor, limits, reading, the deviation from Amendments 7 B, 14 and 31, route changes, study budget arithmetic, owner decisions, and the recommendation (section 8) |
| `project.py` | re-derives the derived numbers from committed files (`python3 -I project.py`) |
| `catalog/` | the read-only catalog responses of 2026-11-21 20:45Z |

In one line (`DRAFT_AMENDMENT.md` section 8): the attempt fits one `cpu5m` 32-vCPU 256 GB pod at
`--max-dense 54000000000` for the low and central bands (about 144-168 GiB), at a worst case of US$62.40 per draw, and
it needs owner decisions on the per-job cap and the study guard. It can return `d_F = 7` or a censored row, not
`d_F ≥ 8`, and the 86,400 s wall budget is the main risk after memory. `[pcc:unverified]`
