# Amendment 2: second reclaim during n = 19; split the n = 19 session

Written 2026-10-18 19:05 UTC, before Parts B and C and before any further
n = 19 run.

The rerun `A-n19-02` (Amendment 1) was cut off by a second container
reclaim at 24 of 40 executions. Both reclaims came while the session was
idle with runs in the background. One n = 19 session lasts about 4.5 h:
the `clo-chain-g4` arm alone takes 19–48 min per run (eight runs). An
ecbench session cannot resume, and its workloads cannot be run as a
subset (they are derived from curve, target seed and index).

Changes:

- **The n = 19 arms other than `clo-chain-g4`** run in `A-n19-fast`, from
  `SPEC-A-n19-fast.json`. That is `SPEC-A-n19.json` with the `clo-chain-g4`
  arm removed, nothing else changed: the same eight workloads (checked with
  `ecbench plan`), method ids, seeds and binary.
- **`clo-chain-g4` at n = 19** is taken from the interrupted session
  `A-n19-02`, which recorded it, verified, on workloads 0–3 (1,139–2,887 s
  each). Workloads 4–7 are **censored by container lifetime**, reported as
  such, never imputed. `A-n19-02` is audited with `--allow-interrupted`
  and a 10-run replay rather than `--replay-all`, because re-executing its
  four chain runs costs about 2.2 h. Every other session keeps
  `--replay-all`.
- The interrupted `A-n19` (14 records) stays as recorded and is audited
  as an interrupted session.
- The operator keeps the session attached while runs proceed.

Nothing was read from `A-n19-02` before this amendment beyond its
per-record arm, workload index, status and solve time.
