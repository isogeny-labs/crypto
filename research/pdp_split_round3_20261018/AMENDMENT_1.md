# Amendment 1: Part A/C sessions move to isolab (2026-10-31)

Recorded before any isolab run of this round. The owner asked for it on
2026-10-31 ("Relaunch on the isolab").

**Why.** In the cloud container, the A-n19 session was interrupted seven
times between 2026-10-24 and 2026-10-25, at 4 or 5 of 56 executions each
time. The container is reclaimed whenever the session goes idle (see
`progress.txt`). The interrupted copies are kept as
`sessions/A-n19-interrupted-*`. None of them is read as a result.

**What changes.**

- The sessions A-n19, C-n15 and A-n17 run as isolab jobs. The job files are
  `isolab/job-*.json`, from `ecbench isolab-job --policy strict --cpus 2`.
  `strict` means bare metal, perf counters, isolation tier A, one core and
  its SMT sibling.
- The binary is the pinned `target/release/ecbench`, sha256 `226a31f0…`
  (`BINARIES.sha256`).
- Specs, seeds, arms and timeouts are unchanged.
- A-n13 (complete, 56/56 verified, cloud container, L0) stands as recorded.

**What does not change.** The counted cost unit is host-independent: it
depends only on code, workload and seed. So S and every ratio are read
exactly as registered. Wall time stays descriptive. The level each record
earned is reported from the records, not from the job's policy.

**Not moved.** The Part B ladder stages (`pdp_split_dreg`) are not ecbench
specs. They need a generic isolab run (`isolab_run`, binary
`pdp_split_dreg` sha256 `58bfbbd5…`), and are submitted the same way once
the hub is reachable from the session.
