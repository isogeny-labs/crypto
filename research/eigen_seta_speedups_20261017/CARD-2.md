# CARD-eigen_seta_speedups-20261017-2

Write-once. Supersedes CARD-eigen_seta_speedups-20261017-1 (CARD.md, not relayed). Author: experiment-director. Executor: executor-mechanical.
All former specification_errors E1-E7 were resolved by the coordinator; resolutions are recorded in the card. Open specification_errors: none.

```yaml
card_id: CARD-eigen_seta_speedups-20261017-2
author_role: experiment-director
executor_role: executor-mechanical
source:
  spec: none (coordinator: no external spec; the dispatch text is the preregistration)
  protocol: /home/user/crypto/research/eigen_seta_speedups_20261017/DISPATCH-1.txt
  protocol_sha256: ec31218f5e50ea2e907561a025825f7368fec38ae3d02e72aae87ef822eeb73b   # sha256 of the dispatch text as received, saved verbatim to that file
git_commit: e82172bff8ce6cd7a0da59217b8a550447e2d5fa   # /home/user/crypto HEAD at compile time
eigenx_commits_expected:                              # recorded in host.txt; a mismatch is recorded, not a stop
  /home/user/eigenx: 1eeb2d3 (observed at compile time 1eeb2d3b1a373b1d5a0caa95261bb493b6b3ad06)
  /tmp/claude-0/-home-user/a00b6ebe-ce34-59ef-a640-978ac116ab30/scratchpad/eigenx-base: fc02c6b (observed at compile time fc02c6b497c5ed4f584888d7c72a265e431b989b)
binaries_sha256_at_compile_time:
  /tmp/eigenx-base/release/examples/bench_kem:          a13b96b7dececd05c1aa39fde5fbe833b3c4d49ded8b96c9a03e2c8a1eb7df28
  /tmp/eigenx-target/release/examples/bench_kem:        05049e54f9d881f02457390b06d66ef65408fae478d1aa2ef7c85cce7cf117fa
  /tmp/eigenx-target-native/release/examples/bench_kem: c95cb6722028dfc96df36bc6f5c135f6e9522c8149ab36e3c9b195cfce203bfc
run_id: run-20261017T2350Z-speedups
outdir: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/
write_scope:
  - /home/user/crypto/research/eigen_seta_speedups_20261017/records/
rules:
  - Run the steps in order, one step per Bash tool call. Steps 0-12 and 14-16 run in the foreground. Step 13 runs in the background. Do not rebuild anything. Run no command not listed.
  - Where a step names stdout_append_file / stdout_file, redirect stdout to exactly that path (">>" for append, ">" otherwise). Shell redirection is permitted for this card (coordinator-confirmed).
  - Compute, round, interpret and summarise nothing.
  - Never retry any step.

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

  # host.txt: eight commands, appended in this order, one Bash call each
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
  - id: 11
    argv: [git, -C, /home/user/eigenx, rev-parse, HEAD]
    stdout_append_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/host.txt
    timeout_seconds: 60
    success_exit_codes: [0]
    note: expected output begins 1eeb2d3; a different output is recorded as is and does not stop the run
  - id: 12
    argv: [git, -C, /tmp/claude-0/-home-user/a00b6ebe-ce34-59ef-a640-978ac116ab30/scratchpad/eigenx-base, rev-parse, HEAD]
    stdout_append_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/host.txt
    timeout_seconds: 60
    success_exit_codes: [0]
    note: expected output begins fc02c6b; a different output is recorded as is and does not stop the run

  - id: 13
    purpose: the nine benchmark runs, as ONE background Bash command
    tool_call: Bash with run_in_background true and timeout 7200000 (ms)
    order: L1(base,new,native), L3(base,new,native), L5(base,new,native)
    on_nonzero_exit_of_a_run: the printf in the command records "<ARM> <SET> <status>" (124 included) and the command continues with the next run; nothing is retried
    last_action: touch /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/DONE
    completion_signal: the harness notification for that background command; do not poll, sleep or re-read files before it arrives
    command_text_verbatim: |-
      timeout 3600 /tmp/eigenx-base/release/examples/bench_kem L1 3 5 1 > /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/base_L1.tsv 2> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/base_L1.err; printf '%s\n' "base L1 $?" >> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/exit_status.txt; timeout 3600 /tmp/eigenx-target/release/examples/bench_kem L1 3 5 1 > /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/new_L1.tsv 2> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/new_L1.err; printf '%s\n' "new L1 $?" >> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/exit_status.txt; timeout 3600 /tmp/eigenx-target-native/release/examples/bench_kem L1 3 5 1 > /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/native_L1.tsv 2> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/native_L1.err; printf '%s\n' "native L1 $?" >> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/exit_status.txt; timeout 3600 /tmp/eigenx-base/release/examples/bench_kem L3 2 3 1 > /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/base_L3.tsv 2> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/base_L3.err; printf '%s\n' "base L3 $?" >> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/exit_status.txt; timeout 3600 /tmp/eigenx-target/release/examples/bench_kem L3 2 3 1 > /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/new_L3.tsv 2> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/new_L3.err; printf '%s\n' "new L3 $?" >> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/exit_status.txt; timeout 3600 /tmp/eigenx-target-native/release/examples/bench_kem L3 2 3 1 > /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/native_L3.tsv 2> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/native_L3.err; printf '%s\n' "native L3 $?" >> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/exit_status.txt; timeout 3600 /tmp/eigenx-base/release/examples/bench_kem L5 1 3 1 > /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/base_L5.tsv 2> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/base_L5.err; printf '%s\n' "base L5 $?" >> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/exit_status.txt; timeout 3600 /tmp/eigenx-target/release/examples/bench_kem L5 1 3 1 > /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/new_L5.tsv 2> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/new_L5.err; printf '%s\n' "new L5 $?" >> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/exit_status.txt; timeout 3600 /tmp/eigenx-target-native/release/examples/bench_kem L5 1 3 1 > /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/native_L5.tsv 2> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/native_L5.err; printf '%s\n' "native L5 $?" >> /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/exit_status.txt; touch /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/DONE
    after_notification: run step 14

  - id: 14
    purpose: confirm the background command reached its last action
    argv: [test, -f, /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/DONE]
    timeout_seconds: 60
    success_exit_codes: [0]
    on_failure: stop and report

  - id: 15
    argv: [uptime]
    stdout_append_file: /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/host.txt
    timeout_seconds: 60
    success_exit_codes: [0]

retries: none
stop_when:
  - Step 0 exits with a status other than 1, or prints a line other than 0.
  - Step 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14 or 15 exits with a status other than 0.
  - The sha256 of any binary printed by step 4 differs from binaries_sha256_at_compile_time above.
  - The harness notification for step 13 reports a failure to launch the command (a non-zero status of an individual benchmark run is NOT a stop; it is recorded and the next run proceeds).
relay:
  stdout_tail_lines: none
  files:
    - /home/user/crypto/research/eigen_seta_speedups_20261017/records/RUN_ID
    - /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/binaries.sha256
    - /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/host.txt
    - /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/exit_status.txt
    - /home/user/crypto/research/eigen_seta_speedups_20261017/records/run-20261017T2350Z-speedups/DONE   # report only whether it exists
```
