# CARD-eigen_seta_speedups-20261017-1

Write-once. Author: experiment-director. Executor: executor-mechanical.
Status: NOT RELAYABLE until the specification_errors at the end are resolved by the main session.

```yaml
card_id: CARD-eigen_seta_speedups-20261017-1
author_role: experiment-director
executor_role: executor-mechanical
source:
  spec: none supplied in dispatch           # see specification_error E1
  protocol: none supplied in dispatch       # see specification_error E1
  dispatch: "Eigen-Seta KEM before/after decaps/keygen speed changes, run-20261017T2350Z-speedups"
git_commit: e82172bff8ce6cd7a0da59217b8a550447e2d5fa   # /home/user/crypto HEAD at compile time
eigenx_commits_from_dispatch:
  base:   fc02c6b  (eigenx main)
  new:    1eeb2d3  (eigenx branch claude/eigenx-scheme-assessment-0z5d0u, default flags)
  native: 1eeb2d3  (RUSTFLAGS='-C target-cpu=native')
binaries_sha256_at_compile_time:
  /tmp/eigenx-base/release/examples/bench_kem:          a13b96b7dececd05c1aa39fde5fbe833b3c4d49ded8b96c9a03e2c8a1eb7df28
  /tmp/eigenx-target/release/examples/bench_kem:        05049e54f9d881f02457390b06d66ef65408fae478d1aa2ef7c85cce7cf117fa
  /tmp/eigenx-target-native/release/examples/bench_kem: c95cb6722028dfc96df36bc6f5c135f6e9522c8149ab36e3c9b195cfce203bfc
run_id: run-20261017T2350Z-speedups
outdir: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/
write_scope:
  - /home/user/crypto/research/eigen_seta_speedups_20261017/records/
rules:
  - Run the steps below strictly in order, one step per Bash tool call, each in the foreground. Do not rebuild anything. Run no command not listed.
  - Where a step names stdout_file / stderr_file / stdout_append_file, redirect that stream to exactly that path ("> path", "2> path", ">> path"). Nothing else is shell syntax.
  - Every Bash call that runs a step uses timeout 3600000 ms (see specification_error E2).
  - Compute, round, interpret and summarise nothing.

steps:
  - id: 0
    purpose: precondition, no valgrind running
    argv: [pgrep, -c, valgrind]
    timeout_seconds: 60
    success_exit_codes: [1]          # pgrep exits 1 when count is 0
    expect_stdout_line: "0"
    on_failure: stop and report; do not wait, do not retry

  - id: 1
    purpose: output dir must not exist yet
    argv: [test, "!", -e, /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups]
    timeout_seconds: 60
    success_exit_codes: [0]

  - id: 2
    argv: [mkdir, -p, /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups]
    timeout_seconds: 60
    success_exit_codes: [0]
    expect_files: [/home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups]

  - id: 3
    purpose: write RUN_ID
    action: create file /home/user/crypto/research/eigen_seta_speedups_20261017/records/RUN_ID with exactly the content "run-20261017T2350Z-speedups" followed by one newline
    expect_files: [/home/user/crypto/research/eigen_seta_speedups_20261017/records/RUN_ID]

  - id: 4
    purpose: binary hashes, before any benchmark
    argv: [sha256sum, /tmp/eigenx-base/release/examples/bench_kem, /tmp/eigenx-target/release/examples/bench_kem, /tmp/eigenx-target-native/release/examples/bench_kem]
    stdout_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/binaries.sha256
    timeout_seconds: 120
    success_exit_codes: [0]
    expect_files: [/home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/binaries.sha256]

  # host.txt: six commands, appended in this order, one Bash call each
  - id: 5
    argv: [lscpu]
    stdout_append_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/host.txt
    timeout_seconds: 60
    success_exit_codes: [0]
  - id: 6
    argv: [nproc]
    stdout_append_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/host.txt
    timeout_seconds: 60
    success_exit_codes: [0]
  - id: 7
    argv: [uname, -a]
    stdout_append_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/host.txt
    timeout_seconds: 60
    success_exit_codes: [0]
  - id: 8
    argv: [rustc, --version]
    stdout_append_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/host.txt
    timeout_seconds: 60
    success_exit_codes: [0]
  - id: 9
    argv: [date, -u, "+%Y-%m-%dT%H:%MZ"]
    stdout_append_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/host.txt
    timeout_seconds: 60
    success_exit_codes: [0]
  - id: 10
    argv: [uptime]
    stdout_append_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/host.txt
    timeout_seconds: 60
    success_exit_codes: [0]

  # Benchmarks. Order is fixed: L1(base,new,native), L3(base,new,native), L5(base,new,native).
  # After EACH benchmark step, the executor appends one line "<ARM> <SET> <status>" to
  # exit_status.txt (status = the decimal exit status that step returned), via
  # argv [printf, "%s\n", "<ARM> <SET> <status>"] with stdout_append_file = exit_status.txt.
  - id: 11
    argv: [timeout, "3600", /tmp/eigenx-base/release/examples/bench_kem, L1, "3", "5", "1"]
    stdout_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/base_L1.tsv
    stderr_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/base_L1.err
    timeout_seconds: 3600
    record_exit_status_line_prefix: "base L1 "
  - id: 12
    argv: [timeout, "3600", /tmp/eigenx-target/release/examples/bench_kem, L1, "3", "5", "1"]
    stdout_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/new_L1.tsv
    stderr_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/new_L1.err
    timeout_seconds: 3600
    record_exit_status_line_prefix: "new L1 "
  - id: 13
    argv: [timeout, "3600", /tmp/eigenx-target-native/release/examples/bench_kem, L1, "3", "5", "1"]
    stdout_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/native_L1.tsv
    stderr_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/native_L1.err
    timeout_seconds: 3600
    record_exit_status_line_prefix: "native L1 "
  - id: 14
    argv: [timeout, "3600", /tmp/eigenx-base/release/examples/bench_kem, L3, "2", "3", "1"]
    stdout_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/base_L3.tsv
    stderr_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/base_L3.err
    timeout_seconds: 3600
    record_exit_status_line_prefix: "base L3 "
  - id: 15
    argv: [timeout, "3600", /tmp/eigenx-target/release/examples/bench_kem, L3, "2", "3", "1"]
    stdout_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/new_L3.tsv
    stderr_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/new_L3.err
    timeout_seconds: 3600
    record_exit_status_line_prefix: "new L3 "
  - id: 16
    argv: [timeout, "3600", /tmp/eigenx-target-native/release/examples/bench_kem, L3, "2", "3", "1"]
    stdout_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/native_L3.tsv
    stderr_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/native_L3.err
    timeout_seconds: 3600
    record_exit_status_line_prefix: "native L3 "
  - id: 17
    argv: [timeout, "3600", /tmp/eigenx-base/release/examples/bench_kem, L5, "1", "3", "1"]
    stdout_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/base_L5.tsv
    stderr_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/base_L5.err
    timeout_seconds: 3600
    record_exit_status_line_prefix: "base L5 "
  - id: 18
    argv: [timeout, "3600", /tmp/eigenx-target/release/examples/bench_kem, L5, "1", "3", "1"]
    stdout_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/new_L5.tsv
    stderr_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/new_L5.err
    timeout_seconds: 3600
    record_exit_status_line_prefix: "new L5 "
  - id: 19
    argv: [timeout, "3600", /tmp/eigenx-target-native/release/examples/bench_kem, L5, "1", "3", "1"]
    stdout_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/native_L5.tsv
    stderr_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/native_L5.err
    timeout_seconds: 3600
    record_exit_status_line_prefix: "native L5 "

  - id: 20
    argv: [uptime]
    stdout_append_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/host.txt
    timeout_seconds: 60
    success_exit_codes: [0]

exit_status_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/exit_status.txt
retries: none
stop_when:
  - Step 0 exits with a status other than 1, or prints a line other than 0.
  - Step 1, 2, 4, 5, 6, 7, 8, 9, 10 or 20 exits with a status other than 0.
  - The sha256 of any binary printed by step 4 differs from binaries_sha256_at_compile_time above.
  - "Benchmark steps 11-19: stop condition on non-zero status is unspecified (specification_error E3)."
relay:
  stdout_tail_lines: 20            # per step, copied verbatim (value chosen by director; no source value, see E4)
  files:
    - /home/user/crypto/research/eigen_seta_speedups_20261017/records/RUN_ID
    - /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/binaries.sha256
    - /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/host.txt
    - /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/exit_status.txt
```

## specification_errors

- E1. No frozen spec JSON, PROTOCOL.md or preregistration path was named in the dispatch, so `source` has no spec_sha256 / protocol_sha256 and no value (keys, trials, jobs, set order) could be checked against a frozen file. Keys/trials/jobs and the set order were copied from the dispatch only. Supply the preregistration path (or confirm none exists).
- E2. The dispatch requires each step to run in the foreground with a Bash tool timeout of 3600000 ms, but the Bash tool's maximum foreground timeout is 600000 ms. `timeout 3600` inside the argv allows 3600 s per benchmark; a Bash call limited to 600000 ms may be cut off before the benchmark ends (L5 runs especially). Resolve explicitly: either confirm 3600000 ms is accepted, or authorize `run_in_background: true` for steps 11-19 (with the completion signal specified). I did not choose.
- E3. The dispatch says to record each exit status but not what to do when a benchmark exits non-zero (including 124 = timeout): stop, or continue with the next arm. Specify one.
- E4. `relay.stdout_tail_lines: 20` has no source value in the dispatch; give a number or "none" (the relayed files are on disk and benchmark stdout goes to files).
- E5. The argv-only rule cannot express the required `>`, `2>` and `>>` redirections, the RUN_ID file write, or the exit_status.txt lines with an observed status substituted; the card expresses them as named fields (`stdout_file`, `stdout_append_file`, `stderr_file`, `record_exit_status_line_prefix`, a file-create action). Confirm the executor may use those redirections and a printf-append of the observed status.
- E6. The dispatch's "nothing else running" is checked only for valgrind (step 0). No other host-load check was specified, so none was added.
- E7. The eigenx source commits (fc02c6b, 1eeb2d3) and the build flags are taken from the dispatch; the binaries were not rebuilt and /home/user/eigenx is not a git repository, so the binary-to-commit correspondence is unverified. Only the binaries' sha256 at compile time is recorded.
