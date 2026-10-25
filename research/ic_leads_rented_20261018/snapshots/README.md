# Snapshot of the A3 and C1 RunPod hosts (2026-10-18 22:20Z), stopped for cost

The owner stopped both closure hosts as too expensive on RunPod and asked for
the runs to move to AWS spot instances. This directory keeps what each host had
produced when it was terminated. Nothing here is edited; files are copied from
each pod's `:8000` listing (`STATUS`, `host-quick.txt`, tails of `boot.log` and
`build.log`, `closure/runs/queue.log`, the host record, and for A3 the draw
shards under `closure/draws/`).

| pod | job | host | ran | spend (approx.) | state when stopped |
|---|---|---|---|---|---|
| `6sh5qyoyusv6ux` | A3 (PROTOCOL.md §11.2) | cpu3m 32 vCPU / 256 GB, $1.76/h | 21:50–22:20Z | $0.90 | `GENERATING_A3`; replay control passed (32/32 lines identical) |
| `9g6slrqws33cr8` | C1 (§11.3) | cpu3m 64 vCPU / 512 GB, $3.52/h | 21:50–22:20Z | $1.80 | `C1_DONE`: allocation failure |

## A3: partial draw stream (not a result under §11.4)

- 473 fresh draws in the frozen range 251–3250 had been written by the 32
  shards: every draw 251–705 plus scattered later ones (max 730).
- 6 are unsatisfiable (`outcome.kind = at_least`, degree 4, as draws 42 and
  214 were): draws 359, 531, 534, 559, 612, 682. 467 are satisfiable. That is
  6/473 = 1.3 %, in line with the 2/251 of A1.
- No closure of a fresh draw ran: §11.2 measures only after the whole range is
  generated and its sha256 recorded. These shards are therefore not the A3 draw
  file and are not committed under `draws/`.
- Use on resume: generation is deterministic (`--shard I/J`, fixed seed), so a
  resumed run regenerates the full range 251–3250 under §11.2 unchanged and
  must reproduce these 473 lines on every field but `secs`. That is an extra
  replay control, not a substitute for the range.

## C1: degree 5 does not fit in 471 GB (outcome `X` of §11.4's table, for C1)

- `dreg_closure --to 5 --max-cols 1100000 --draw 0` on `m3-19-7`, under
  `prlimit --as` A = 471,040,000,000 B (0.92 × the 512 GB cgroup).
- Exit 134 after 1,317 s: `memory allocation of 94976 bytes failed`, peak RSS
  424,510,832 kB (≈ 405 GiB), 637 % CPU (`runs-deg5-m3-19-7.d0.log`).
- The projection in §11.3 was 217 GB; the first host failed at 169 GB resident
  under 184 GB. Two hosts now show the cell's memory well above the projection.
  §11.3 reads an allocation failure as not evidence about the degree, so C1
  stays open. A further attempt needs a host of roughly 768 GB or more and its
  own amendment before it runs (not registered here).

## Moving to AWS spot

`../aws_spot/aws_spot.py` launches the same `pod/boot.sh` job on an EC2 spot
instance (Ubuntu 24.04, results on :8000, self-terminating at the job end or the
lifetime cap). It needs complete AWS credentials: this session's environment
has `AWS_ACCESS_KEY_ID` but no `AWS_SECRET_ACCESS_KEY`, so nothing was launched.

| run | instance (candidate) | memory | on-demand $/h (us-west-2, list) | spot $/h | expected hours |
|---|---|---|---|---|---|
| A3 rerun (§11.2 unchanged) | r7i.8xlarge (32 vCPU) | 256 GB | 2.12 | confirm with `aws_spot.py prices` (typically 25–40 % of on-demand) | 4–6 |
| C1 retry (needs an amendment) | r7i.24xlarge / r6i.32xlarge | 768 GB / 1 TB | 6.35 / 8.06 | confirm with `prices` | 3–12 |

Spot prices were not queried (no credentials); the on-demand figures are list
prices and the spot column is to be filled from `prices` before launch.
Spot interruption terminates the instance; a resumed A3 regenerates its range
from the fixed seed, so an interruption costs time, not correctness.
