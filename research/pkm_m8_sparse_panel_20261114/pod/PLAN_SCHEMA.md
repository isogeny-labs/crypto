# plan.json schema `pkm.m8-panel.plan/v1`  [pcc:unverified]

Written by the main session, placed at the top of `bundle.tgz`. `job.sh` runs the rows sorted by `order`
(unique integers), one at a time, never retrying and never skipping.

```json
{ "schema": "pkm.m8-panel.plan/v1",
  "plan_id": "panelA-20261114-v1",
  "instrument": "bin/pkm_m8_panel",                    // path inside the bundle (release binary built in the lab container, glibc 2.39)
  "msolve": {"version": "0.6.5-1build2", "sha256": "c2722288d22c3a1eab00c0b818d0204d4a98104d059b2fd4ecb7343e8ef759fa"},
  "defaults": {"threads": 2, "timeout_s": 120, "mem_cap_kb": 2097152},   // any row key below may be defaulted here
  "rows": [
    { "order": 1,                                      // frozen execution order (required, unique)
      "cell": "t3_r8", "target": "sat1",               // required; names only (used in paths and in runs.jsonl)
      "solver": "msolve",                              // "msolve" | "native"          (required)
      "mode": "param",                                 // msolve: "param" = -P 1 (default), "gb" = -g 2; NOT combinable
      "threads": 8,                                    // msolve -t and taskset -c <first threads allowed cpus>
      "timeout_s": 21600,                              // timeout -k 30 <s>; censored row on expiry
      "mem_cap_kb": 100663296,                         // prlimit --as (KiB); 0 = none. msolve needs >= ~1.3 GB just to start
      "system": "systems/t3_r8/sat1.ms",               // default systems/<cell>/<target>.ms
      "system_native": "systems/t3_r8/sat1.native.json",  // native rows; default systems/<cell>/<target>.native.json
      "cell_dir": "cells/t3_r8", "targets_dir": "cells/t3_r8",   // for `verify` (defaults cells/<cell>)
      "verify": true,                                  // default: true for msolve+param, false otherwise
      "native_max_degree": 12, "native_deadline_s": 3570,        // native-solve --max-degree/--deadline (deadline default timeout_s-30)
      "flags": []                                      // extra msolve flags, e.g. ["-c","0"]
    } ] }
```

Commands (exactly):
* msolve row: `taskset -c CPUS /usr/bin/time -v -o time_v.txt timeout -k 30 TIMEOUT prlimit --as=CAP*1024 /usr/bin/msolve -t T -v 2 {-P 1 | -g 2} [flags] -f SYSTEM -o solve.out > stdout.log 2> stderr.log`
* native row: same wrapper around `INSTRUMENT native-solve --system SYSTEM_NATIVE --max-degree D --deadline S --out solve.out`
* then, for verify rows with rc 0: `INSTRUMENT verify --cell CELL_DIR --targets TARGETS_DIR --msolve-out solve.out --target TARGET --out verify.json`
  (taskset to the same cpus, `timeout 900`, not part of the solver wall).

Outputs in `WORKDIR/out`: `env/{env.txt,apt.log,plan.json}`, `rows/NNN_cell_target_solver_mode/{solve.out,stdout.log,stderr.log,time_v.txt,log_table.json,verify.json}`,
`runs.jsonl` (one row per solve: cell, target, solver, mode, argv, threads, cpus, cpus_granted, timeout_s, mem_cap_kb, start_utc, end_utc, wall_s,
time_v_elapsed_s, user_s, sys_s, max_rss_kb, exit_code, signal, censored [null|timeout|rlimit_or_crash|signal:N|error], censor_note,
input/input_sha256, output/output_sha256/output_bytes, stdout(+sha/bytes), msolve_outcome, quotient_dim, elim_degree, verify{path,exit_code,wall_s,sha256}, log_table{...}),
`progress.json`, `job.log`, `results.tgz` (everything above), then `done.json` (totals, by_status, sha256 of bundle/plan/job.sh/msolve/results.tgz).
`tools/parse_msolve_log.py` in the bundle (optional) adds `log_table.json` per msolve row.
