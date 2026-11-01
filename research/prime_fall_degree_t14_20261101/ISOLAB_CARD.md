# Iso Lab card for the cap-6 cells (owner decision 2026-11-01: `isolab_k8s_run` on the lab host, non-resumable)

Written for a session that has the `isolab-eks` stdio MCP (the lab host). The session that wrote this has no
`isolab_k8s_*` tool and no AWS CLI, so nothing here has been submitted. Frozen before any cap-6 row exists.

## What runs

Two independent jobs, one system each (Amendment 7 A). `D` is deterministic in the system; the host changes only
the odds of censoring. Both use the binary of `examples/pkm_tower_pilot.rs` at one pinned commit.

| field | job `T 6 0` | job `T 6 1` |
|:--|:--|:--|
| `job_name` | `pfd-t14-k0-c6-20261101` | `pfd-t14-k1-c6-20261101` |
| `image` | digest-pinned, UID 1000, holds `pkm_tower_pilot` (below) | same image |
| `memory` | `112Gi` | `112Gi` |
| `cpu` | `16` (even, ≤ 28; pool cap 32 vCPU / 128 GiB, so the two jobs run one after the other unless a second pool node fits) | `16` |
| `seconds` | `90000` (the run's `--budget 86400` plus startup and upload) | `90000` |
| `retries` | `0` (non-resumable: a Spot interruption ends the job; do not resubmit under a new name, report it) | `0` |

`command` (argv, no shell), per job, with `K` = 0 or 1:

```
pkm_tower_pilot --p 2013265921 --ladder-t none --planted 0 --random 2 --engine tower --kinds kummer --m 2 \
  --controls tower --budget 86400 --max-dense 27900000000 --trace --t-min 14 --max-t 14 --targets K --cap 6 \
  --out /work/T-t14-kK-c6.jsonl
```

with `RAYON_NUM_THREADS=16` if the tool takes an environment, else wrap in an image entry script that sets it. The
options are those of `run_cells.sh T 6 K` with the 112 GiB request of Amendment 7 A (`--max-dense` = the largest
multiple of 10⁸ with `4·C <= 112 GiB − 8 GiB` = 27,900,000,000) and `--budget 86400`. The `--out` file and the
stderr trace must be uploaded to `ISOLAB_RESULTS_URI` before the job ends (a `/work` path is temporary).

## Image (not built)

No existing workflow builds one. It needs `pkm_tower_pilot` from a pinned 40-character commit of `aburan28/crypto`
`main` (a static `x86_64-unknown-linux-musl` build, as `build-info.json` records for ecbench), `aws` for the upload,
`USER 1000:1000`, `HOME=/work`, in the lab's private ECR repository, referenced by digest. Record the source SHA,
binary SHA-256 and OCI digest as three distinct values (SKILL.md). Taking `Dockerfile.ecbench` as the template.

## Reading and what to record

- Each row follows Amendment 7 B: cap 6 refutes → `d_F ≤ 6`, C-step (7) refuted at `t = 14` for that draw; cap 6 does
  not refute → `d_F ≥ 7`, bracketed from below, then cap 7 is the next cell; stopped by `--budget` or `--max-dense`
  → censored, no reading. Both draws are non-decomposable (`runs/G-grid-zeros.jsonl`).
- **Expect censoring.** At 4 vCPU the `t = 12` cap-6 run exceeded 7,200 s; carrying the `t = 11 → 12` ratio two levels
  gives about 1.1·10⁶ s at 4 vCPU. At 16 vCPU with ideal scaling that is about 2.8·10⁵ s, above the 86,400 s budget.
  Whether the run fits is unknown; a censored row is a valid outcome and bounds nothing. Raising the budget is a
  change to Amendment 7 A and the owner's call, as is resumability for a multi-day job (`isolab.eks.resumable/v1`).
- Retain the job name, Job UID, Pod UID, image digest and the results URI. Download the objects and check their
  hashes; a succeeded Pod alone proves nothing. Land the rows in `runs/` here with a `REPORT.md` update, never
  re-run under a new job name to get a different outcome.
