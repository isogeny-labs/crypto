# plan.json schema `pkm.m8-panel.plan/v2` (protocol v2)  [pcc:unverified]

Top level of `bundle.tgz`. `job.sh` runs the rows sorted by `order` (unique integers), one process at a time,
never retrying and never skipping. The bundle must NEVER contain a file named `hidden.json` (any case, any
directory): `launch.sh` refuses to create a pod and `job.sh` refuses to start if one is present.

```json
{ "schema": "pkm.m8-panel.plan/v2", "plan_id": "panelA-20261114-v2",
  "instrument": "bin/pkm_m8_panel",
  "msolve": {"version": "0.6.5-1build2", "sha256": "c2722288d22c3a1eab00c0b818d0204d4a98104d059b2fd4ecb7343e8ef759fa"},
  "defaults": {"threads": 8, "timeout_s": 14400, "rss_cap_kb": "cgroup0.9"},
  "rows": [ {
    "order": 1,                       // frozen execution order (required, unique)
    "kind": "natural",                // required: aa | planted | natural | gb_check | native | canary | timer_floor | probe
    "cell": "t3r8", "target": "sat1", // required (names only)
    "solver": "msolve",               // msolve | native  (required)
    "msolve_mode": "param",           // param = -P 1 | gb = -g 2 ; NEVER combined (legacy key `mode` accepted)
    "threads": 8,                     // msolve -t and taskset -c <first threads allowed cpus>
    "timeout_s": 14400,               // wall timeout (timeout -k 30); censored 'timeout' on expiry
    "rss_cap_kb": "cgroup0.9",        // resident-size cap in kB, or "cgroupF" = F x container memory limit; t=2 rows: 62914560 (60 GB)
    "system": "systems/t3r8/sat1.ms", // default systems/<cell>/<target>.ms (canary: the fixed system file)
    "system_native": "systems/t3r8/sat1.native.json",
    "cell_dir": "cells/t3r8", "targets_dir": "cells/t3r8",   // for `verify`; these hold PUBLIC files only
    "verify": true,                   // default: true for msolve+param (kinds aa, planted, natural, probe), else false
    "native_max_degree": 12, "native_deadline_s": 3570, "flags": [],
    "on_replacement": false           // true: row runs only on a replacement pod (A/A repeat there)
  } ] }
```

Kind defaults (row > kind default > `defaults`): `canary` = msolve gb, 8 threads, 120 s, no verify;
`timer_floor` = msolve gb, 1 thread, 60 s, no verify (give it a one-line trivial system, e.g. `x\n10007\nx-1`).

## What job.sh does per row
`taskset -c CPUS /usr/bin/time -v -o time_v.txt timeout -k 30 TIMEOUT prlimit --as=1.25*CAP /usr/bin/msolve -t T -v 2 {-P 1|-g 2} -f SYSTEM -o solve.out`
(native rows: the same wrapper around `INSTRUMENT native-solve --system … --max-degree … --deadline … --out …`), then for verify rows with rc 0
`INSTRUMENT verify --cell … --targets … --msolve-out solve.out --target … --out verify.json` (not part of the solver wall).
* Before each row it waits up to `PRE_WAIT_S` (default 60) for any msolve already running, and records it as
  `concurrent_process_before_start` if still there (a lock file also stops a second job.sh in the same workdir).
* **Watchdog** (python, one per row): every 2 s reads `/proc/<pid>/status` of the whole process tree, sums VmRSS, records max VmHWM,
  max VmPeak and max summed RSS; if summed RSS exceeds `rss_cap_kb`: SIGTERM, 10 s, SIGKILL. `prlimit --as` = 1.25 x cap is only a backstop
  (msolve needs ~1.2 GB of address space to start, so a cap below ~1 GB resident gives `crash`, not a clean stop).
* Cap value: `env/mem.json` records `/sys/fs/cgroup/memory.max`, `/sys/fs/cgroup/memory/memory.limit_in_bytes` and `MemTotal`;
  `limit_kb` = the smaller finite one (meminfo only if no finite cgroup limit, then `source` says so). "cgroup0.9" = 0.9 x `limit_kb`.
* **Censoring** (`censored`, one of): `rss_cap` (watchdog fired), `timeout` (rc 124, time -v `Exit status: 124`), `crash` (rc 139 with max RSS < 50% of cap
  or wall < 60 s), `as_rlimit` (any other rc 139: backstop), `signal:N`, `error` (other nonzero), `ok`. Also recorded: exit_code, signal, wall_s, user_s,
  sys_s, max_rss_kb (time -v), vmpeak_kb/vmhwm_kb/watchdog (poll counts, hit time), pod_id, replacement, kind.
* msolve rows also get `log_table` from `tools/parse_msolve_log.py`: rounds, max degree, nonzero-entries proxy over all passes, and
  `up_to_degree_8` = cumulative real/cpu seconds of rounds with degree <= 8 (canary drift metric).

## Outputs (`WORKDIR/out`)
`env/{env.txt,mem.json,apt.log,plan.json}`, `rows/NNN_cell_target_solver_mode/{solve.out,stdout.log,stderr.log,time_v.txt,watchdog.json,log_table.json,verify.json}`,
`runs.jsonl`, `progress.json`, `job.log`, `results.tgz`, then `done.json` (`pkm.m8-panel.done/v2`: totals, by_status, sha256 of bundle/plan/job.sh/msolve/results.tgz, pod_id).
Launcher sync (every poll, into `--out DIR`): `runs.jsonl`, `verify/<row>.json`, `rows_meta/<row>/{log_table,watchdog}.json`, `env/env.txt`, `env/mem.json`.
