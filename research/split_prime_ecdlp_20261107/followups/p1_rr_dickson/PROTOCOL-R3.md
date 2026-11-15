# P1 round 3: T24 m5 t4 to completion on a larger host (protocol, frozen 2026-11-08)

Follows round 2 (`PROTOCOL-R2.md` with its Addendum A; card B in `RESULTS.md`). In card B, RR
ran out of time at 10,800 s (D ≥ 6, working in degree 7, widest ≥ 146,277) and T reached the
13 GiB memory cap (D ≥ 9, widest ≥ 662,876). Under the frozen bound reading this cell is
undecided. Round 3 changes only the limits and the host. The instrument, msolve, instance,
seed, bound reading and decision rule stay as they were. User request (2026-11-08): "Yes run
it", meaning run the cell to completion on a larger host with about a day's time limit.
This is a stage diagnostic on the algebraic track. `S`, end-to-end cost and the rho
comparison are not set.

## What is fixed (unchanged from R2b)

- Instrument: `p1tool`, `src/main.rs` sha256 `7e50f8c8…84ed7` (PROTOCOL-R2 Addendum A).
- msolve: the Ubuntu noble package 0.6.5-1build2. Its `/usr/bin/msolve` sha256 is
  `c2722288…59fa`, the R2b bind, and the image build checks this.
- Invocation: `calibration/msolve_cell.sh`, i.e. `msolve -g 2 -t 1 -v 2` with `ulimit -v`
  and `timeout`.
- Cell: Dickson T24 (p 14,680,063, a 3,607,823, b 6,826,282), m = 5, t = 4, seed 2. Seed 2
  is R2b's first admissible seed (seed 1 was skipped because its base was too small).
- Instances: emitted again in the pod and required to be byte-identical to R2b's.
  - RR: `.ms` sha256 `9ba376d8…3cff`.
  - T: `.ms` sha256 `ec0af27e…590e`.
  - A mismatch is a HARD_ERROR and no msolve job runs. Driver: `calibration/run_cells_r3.sh`.
- Bound reading and rule 3 of PROTOCOL-R2, word for word:
  - RR below T on D and widest → "RR advantage holds at m = 5 with a tower".
  - RR below T on D only → "trade".
  - RR not below T on D → "fails at m = 5".
  - Otherwise → "undecided".
  - "Below" is decided when RR completed and its value is below T's value or T's lower bound.
- Bounds come from all of R2b and R3. A capped or timed-out job's D and widest are lower
  bounds, and R3's bounds replace R2b's only where they are larger. A completed job's values
  are exact. Nothing is imputed.

## What changes

| system | pod | VLIMIT (`ulimit -v`) | TLIMIT | Job deadline |
|---|---|---|---|---|
| RR | Iso Lab EKS `cpu-spot`, 2 vCPU / 64Gi | 62,914,560 KiB (60 GiB) | 86,400 s | 100,800 s |
| T | Iso Lab EKS `cpu-spot`, 2 vCPU / 60Gi | 58,720,256 KiB (56 GiB) | 86,400 s | 100,800 s |

- One command Job per system, in separate pods on separate nodes, with one msolve process per
  pod (`-t 1`). The two Jobs run at the same time. This round reads only D, widest and
  quotient dimension. Wall time is reported but decides nothing beyond the TLIMIT cutoff.
- Each VLIMIT sits below its pod's memory so that msolve stops at the ulimit (a recorded
  "cap") before a cgroup out-of-memory kill. An out-of-memory kill is recorded as
  "oom_kill", read as a cap at the pod memory, and is not a stop.
- Records sync to S3 every 60 s and again at exit.
- Spot loss: a lost pod is an infrastructure failure, not a result. It may be resubmitted
  twice per system, under a new Job name, from the start (msolve cannot checkpoint). A
  partial log from a lost pod is kept and reported, but it supplies no bound.
- Budget: a ceiling of USD 60 for all attempts together, failures included. When the budget
  is exhausted or the remaining balance is unknown, no new Job is launched.

## Predictions (written before data)

- RR at degree 7 grows its widest matrix about 5× per degree (round 2: 27,957 → 146,277), so
  it is expected to land near T's lower bound of 662,876. Whether "holds" or "trade" comes
  out is genuinely open. A widest above 662,876 needs T's completed value or a larger T
  bound to decide.
- T is expected to need more than 56 GiB, so it is likely to cap again. Its bound will then
  rise.

## Sources checked

The KB was unavailable in this session. Paths read:
`research/arity_completion_20261104/launch/{README.md, job-lib.sh, make-requests.sh}` (the
command-Job precedent),
`research/pkm_falldegree_unblock_20261107/{image/build_layer.sh, validate_command_request.sh}`,
`taskq/deploy/eks-karpenter/{remote/launch-remote-command, remote/run-command, ACCESS.md,
COST_CONTROLS.md}`, and `.claude/skills/isolab-eks/SKILL.md`.
