# Brief: RunPod runner for the m = 8 sparse PKM relation-finder panel (panel A)

Context: see BRIEF-instrument.md in this directory (another worker builds the native instrument
`examples/pkm_m8_panel.rs`; coordinate only through files in this directory). The owner authorized
the spend; the host route is RunPod CPU pods (the lab's isolab monthly budget is exhausted:
1,808.95 of 1,300 USD on 2026-11-14, so EKS takes no new launches). Reference kit:
`research/runpod_kit_20261112/` (rp.sh, podserver.py). The RunPod key is the environment variable
RUNPOD_API_KEY here (NOT proxy-injected as that kit assumes). NEVER print the key, never write it to
a file, never put it in a commit. Two pods belonging to other sessions are RUNNING right now
(`bdr-wallfix`, `pkm-odd-highmem-20261114`): never touch, stop or delete any pod you did not create.
Do NOT create any pod in this task: build and dry-run only. Everything is `[pcc:unverified]`.

## Deliverables (all under research/pkm_m8_sparse_panel_20261114/pod/)
1. `MSOLVE_FORMAT.md` (write this FIRST, within your first 10 minutes): msolve 0.6.5 is at
   /usr/bin/msolve (sha256 c2722288d22c3a1eab00c0b818d0204d4a98104d059b2fd4ecb7343e8ef759fa, Ubuntu
   24.04 package 0.6.5-1build2). Determine empirically, on a tiny zero-dimensional system over a prime
   field (e.g. x^2-2, y-x over F_10007, then a 3-variable one with several solutions), what
   `msolve -P 1 -f in.ms -o out` emits over F_p (rational parametrization: eliminating polynomial,
   denominator, parametrizations; or an error), what `-g 2` emits (reduced Groebner basis), and what
   `-v 2` prints per F4 round (the legend of the trace). Document the exact output grammar with examples
   so the instrument's `verify` can parse it. Also test `-P 1` on a system with NO solution and with a
   positive-dimensional one. Record msolve's exit codes.
2. `rp.sh`: a copy of the kit's client adapted to use `$RUNPOD_API_KEY` via a header built in the shell
   WITHOUT echoing it (e.g. `-H "Authorization: Bearer $RUNPOD_API_KEY"` inside the script; keep
   `set -euo pipefail`; no `set -x`). Add `rp.sh flavors` that GETs the CPU flavor/pricing endpoint(s) you
   can find in the RunPod REST v1 API (read-only; try /v1/cpu-types or the OpenAPI at
   https://rest.runpod.io/v1/openapi.json) and prints vcpu/memory/price per flavor, so the main session
   can pick a 32 vCPU / 256 GB (or the largest available cpu5m) pod and record its price. Do a read-only
   `rp.sh flavors` run and save its output to `flavors.json`. List existing pods (read-only) to confirm
   the client works; do not create anything.
3. `podserver.py`: copy from the kit (it is the pod-side HTTP server; keep it byte-identical unless a
   change is needed; say if so).
4. `job.sh`: the pod-side job runner. Inputs: a tarball `bundle.tgz` (the instrument binary built on
   the pod from source? NO: the pod is glibc 2.39 like this container, so ship the release binary built
   here plus the cell/target/system files). Steps: install msolve from the Ubuntu archive pinned to
   0.6.5-1build2 (`apt-get install -y msolve=0.6.5-1build2`), record `sha256sum /usr/bin/msolve`,
   `dpkg -s msolve`, `nproc`, `free -g`, `lscpu`, `/proc/cpuinfo` model; then for each cell in a frozen
   order given by a `plan.json` (the main session writes it; design the schema: list of {cell, target,
   solver: msolve|native, threads, timeout_s, mem_cap_kb, order}), run exactly one solve at a time:
   `taskset -c 0-(threads-1) /usr/bin/time -v timeout <timeout_s> prlimit --as=<mem_cap> msolve -t T
   -v 2 -g 2 -P 1 -f X.ms -o X.out` with stdout/stderr to files, then the instrument's `verify` and
   `native-solve` where planned; write a `runs.jsonl` row per solve with: cell, target, solver, argv,
   start/end UTC, wall, user+sys CPU (from time -v), max RSS kB, exit code, censored (timeout / rlimit /
   signal), msolve output sha256, verify.json path. Never retry; never skip a planned row; a failed row
   is recorded and the plan continues. At the end, `done.json` with totals and a tarball `results.tgz`
   of all logs (compress; msolve -v 2 logs can be large: keep them, they are the matrix-size evidence).
5. `parse_msolve_log.py` (tabulation only): from an msolve `-v 2` log produce per-round rows
   (degree, sel pairs, matrix rows x cols, density, new, zero reductions, time) and totals
   (max round degree, sum of rows*cols*density as a "nonzero-entries work proxy", wall). Validate on the
   committed log `research/prime_leads_chase_20261113/L1-pkm/decisive/p16801793_m8_t3_s1_random_A3.log`.
6. `launch.sh`: the main-session-side orchestration for ONE pod: create (name `pkm-m8-panelA-<stamp>`,
   flavor/vcpu/disk from args), wait, put bundle, exec job.sh, poll `done.json` every 10 minutes
   (sleep is fine inside this long-running script; it is not a hook), get results.tgz, delete pod; with
   a hard wall deadline argument after which it fetches whatever exists and deletes the pod. Print the
   pod id and hourly price at create time. Dry-run mode `--dry-run` that does everything except the
   RunPod API calls (uses a local directory as the "pod" and runs job.sh locally on a tiny plan).
7. Local dry run: with msolve installed locally, build a tiny plan (use
   `cargo run --release --example pkm_oracle_program -- export-msolve` on a small request if the
   instrument example is not ready yet; see examples/pkm_oracle_program.rs for the request schema and
   its tests for a sample request) and run `job.sh` locally under `--dry-run` with 2 threads, 2 GB cap,
   120 s timeout. Save the dry-run outputs under pod/dryrun/ (small files only).

Report back: the msolve output grammar findings (most important), flavor/price table, the plan.json
schema, the dry-run result, and anything the protocol must fix (thread counts, caps, what `time -v`
reports, msolve exit codes on timeout/rlimit).
