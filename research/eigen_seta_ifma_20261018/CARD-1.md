card_id: CARD-eigen_seta_ifma-20261018-1
author_role: experiment-director
executor_role: executor-mechanical
source:
  spec: /home/user/crypto/research/eigen_seta_ifma_20261018/DISPATCH-1.txt
  spec_sha256: 1dcedd1af34799857ef383ce5fafe5490228944f1e7404ffd829796bdbd2d893
  protocol: /home/user/crypto/research/eigen_seta_ifma_20261018/DISPATCH-1.txt
  protocol_sha256: 1dcedd1af34799857ef383ce5fafe5490228944f1e7404ffd829796bdbd2d893
git_commit: 57e8f783525fb03fd9ea29c778b66db016bca229   # /home/user/crypto HEAD at compile time
binaries_compile_time_sha256:
  base:   05049e54f9d881f02457390b06d66ef65408fae478d1aa2ef7c85cce7cf117fa  /tmp/eigenx-base/release/examples/bench_kem
  ifma:   547298c7ff6030e4b203fe8f275c7711849465b1e892d61a21306880db4467db  /tmp/eigenx-target/release/examples/bench_kem
  native: 6951e9a189ce9091304f521781794a219a01f9f22d4d3b1b686aa069566efca6  /tmp/eigenx-target-native/release/examples/bench_kem
expected_git_heads_at_compile_time:
  /home/user/eigenx: 0a8da400a6090f6b561f320ca8f3280f5756e459
  /tmp/claude-0/-home-user/a00b6ebe-ce34-59ef-a640-978ac116ab30/scratchpad/eigenx-base: 580b633b92e96a4dbf69a666902b1afc4b3eba02
run_id: run-20261018T0320Z-ifma
outdir: /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma
write_scope:
  - /home/user/crypto/research/eigen_seta_ifma_20261018/records/
rules:
  - Do not rebuild anything. Do not run any command that is not listed in steps. Compute and interpret nothing.
  - Each step below is one Bash tool call, in order, unless the step says otherwise. Shell redirection (>, >>, 2>) is permitted (dispatch). No other shell syntax is permitted except where the step text itself contains it.
  - A step with "shell:" is run exactly as the string printed, as one Bash call. A step with "argv:" is run as that argument vector, one argument per list item, with the quoting needed to pass each item as a single argument.
  - Relay all printed output and exit statuses verbatim.
steps:
  - id: 0
    argv: [pgrep, -c, -f, "[c]allgrind|[m]emcheck|[v]algrind|[d]udect|[c]argo"]
    timeout_seconds: 120
    success_exit_codes: [1]
    expect_stdout_exact: "0"
    on_failure: stop and report; no wait, no retry
  - id: 1.1
    argv: [test, "!", -e, /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma]
    timeout_seconds: 120
    success_exit_codes: [0]
  - id: 1.2
    argv: [mkdir, -p, /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma]
    timeout_seconds: 120
    success_exit_codes: [0]
    expect_files: [/home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma]
  - id: 1.3
    shell: printf '%s\n' run-20261018T0320Z-ifma > /home/user/crypto/research/eigen_seta_ifma_20261018/records/RUN_ID
    timeout_seconds: 120
    success_exit_codes: [0]
    expect_files: [/home/user/crypto/research/eigen_seta_ifma_20261018/records/RUN_ID]
  - id: 2
    shell: sha256sum /tmp/eigenx-base/release/examples/bench_kem /tmp/eigenx-target/release/examples/bench_kem /tmp/eigenx-target-native/release/examples/bench_kem > /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/binaries.sha256
    timeout_seconds: 120
    success_exit_codes: [0]
    expect_files: [/home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/binaries.sha256]
    expected_file_content_exact: |
      05049e54f9d881f02457390b06d66ef65408fae478d1aa2ef7c85cce7cf117fa  /tmp/eigenx-base/release/examples/bench_kem
      547298c7ff6030e4b203fe8f275c7711849465b1e892d61a21306880db4467db  /tmp/eigenx-target/release/examples/bench_kem
      6951e9a189ce9091304f521781794a219a01f9f22d4d3b1b686aa069566efca6  /tmp/eigenx-target-native/release/examples/bench_kem
    on_mismatch: stop and report (textual comparison of the three lines only; no computation)
  - id: 3.1
    shell: lscpu >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/host.txt
    timeout_seconds: 120
    success_exit_codes: [0]
  - id: 3.2
    shell: nproc >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/host.txt
    timeout_seconds: 120
    success_exit_codes: [0]
  - id: 3.3
    shell: uname -a >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/host.txt
    timeout_seconds: 120
    success_exit_codes: [0]
  - id: 3.4
    shell: rustc --version >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/host.txt
    timeout_seconds: 120
    success_exit_codes: [0]
  - id: 3.5
    shell: date -u +%Y-%m-%dT%H:%MZ >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/host.txt
    timeout_seconds: 120
    success_exit_codes: [0]
  - id: 3.6
    shell: uptime >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/host.txt
    timeout_seconds: 120
    success_exit_codes: [0]
  - id: 3.7
    shell: git -C /home/user/eigenx rev-parse HEAD >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/host.txt
    timeout_seconds: 120
    success_exit_codes: [0]
    note: output differing from expected_git_heads_at_compile_time is recorded, not a stop
  - id: 3.8
    shell: git -C /tmp/claude-0/-home-user/a00b6ebe-ce34-59ef-a640-978ac116ab30/scratchpad/eigenx-base rev-parse HEAD >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/host.txt
    timeout_seconds: 120
    success_exit_codes: [0]
    note: output differing from expected_git_heads_at_compile_time is recorded, not a stop
  - id: 4
    tool_settings: {run_in_background: true, timeout_ms: 7200000}
    shell: |
      timeout 3600 /tmp/eigenx-base/release/examples/bench_kem L1 3 5 1 > /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/base_L1.tsv 2> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/base_L1.err; printf '%s\n' "base L1 $?" >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/exit_status.txt; timeout 3600 /tmp/eigenx-target/release/examples/bench_kem L1 3 5 1 > /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/ifma_L1.tsv 2> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/ifma_L1.err; printf '%s\n' "ifma L1 $?" >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/exit_status.txt; timeout 3600 /tmp/eigenx-target-native/release/examples/bench_kem L1 3 5 1 > /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/native_L1.tsv 2> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/native_L1.err; printf '%s\n' "native L1 $?" >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/exit_status.txt; timeout 3600 /tmp/eigenx-base/release/examples/bench_kem L3 2 3 1 > /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/base_L3.tsv 2> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/base_L3.err; printf '%s\n' "base L3 $?" >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/exit_status.txt; timeout 3600 /tmp/eigenx-target/release/examples/bench_kem L3 2 3 1 > /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/ifma_L3.tsv 2> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/ifma_L3.err; printf '%s\n' "ifma L3 $?" >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/exit_status.txt; timeout 3600 /tmp/eigenx-target-native/release/examples/bench_kem L3 2 3 1 > /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/native_L3.tsv 2> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/native_L3.err; printf '%s\n' "native L3 $?" >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/exit_status.txt; timeout 3600 /tmp/eigenx-base/release/examples/bench_kem L5 1 3 1 > /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/base_L5.tsv 2> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/base_L5.err; printf '%s\n' "base L5 $?" >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/exit_status.txt; timeout 3600 /tmp/eigenx-target/release/examples/bench_kem L5 1 3 1 > /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/ifma_L5.tsv 2> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/ifma_L5.err; printf '%s\n' "ifma L5 $?" >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/exit_status.txt; timeout 3600 /tmp/eigenx-target-native/release/examples/bench_kem L5 1 3 1 > /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/native_L5.tsv 2> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/native_L5.err; printf '%s\n' "native L5 $?" >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/exit_status.txt; touch /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/DONE
    timeout_seconds: 7200
    success_exit_codes: [0]
    note: >
      Run this as ONE Bash call with run_in_background true and timeout 7200000 ms. The command is the single
      line above (all 9 runs plus the final touch, separated by "; "). A non-zero exit of any run (124 included)
      is recorded in exit_status.txt and the next run proceeds; no retries. Do not poll, sleep or read the output
      files while it runs. The completion signal is the harness notification for this background command.
  - id: 4.1
    argv: [test, -f, /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/DONE]
    timeout_seconds: 120
    success_exit_codes: [0]
    run_after: the harness notification for step 4 has arrived
    on_failure: stop and report that DONE is missing
  - id: 5
    shell: uptime >> /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/host.txt
    timeout_seconds: 120
    success_exit_codes: [0]
retries: none
stop_when:
  - Step 0 exits with a status other than 1 or prints anything other than 0.
  - Step 1.1 exits with a status other than 0 (the output dir already exists).
  - Any of steps 1.2, 1.3, 2, 3.1 to 3.6 exits with a status other than 0.
  - The content of binaries.sha256 written by step 2 differs textually from expected_file_content_exact in step 2.
  - Step 4.1 exits with a status other than 0 (DONE missing).
  - Any step, other than step 4, exceeds its timeout_seconds.
relay:
  stdout_tail_lines: none
  per_step: the printed output (if any) and the exit status of every step actually run, verbatim
  files:
    - /home/user/crypto/research/eigen_seta_ifma_20261018/records/RUN_ID
    - /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/binaries.sha256
    - /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/host.txt
    - /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/exit_status.txt
  also: whether /home/user/crypto/research/eigen_seta_ifma_20261018/records/run-20261018T0320Z-ifma/DONE exists (the exit status of step 4.1)
  not_relayed: the .tsv and .err files are not read or relayed by the executor
