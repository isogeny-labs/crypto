# RUN CARD v5: ecgfp5_quotient_solve_20261101 (frozen command card, amendment A4)

Executor tier: mechanical. Do exactly what is written. Do not interpret, retry,
edit, repair or choose. If a step says stop, stop and report the fact.
v1, v2, v3 and v4 stay as they are and are not to be run.

CHECKOUT = /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr
EXPDIR   = CHECKOUT/research/ecgfp5_quotient_solve_20261101

Approval: owner requests 2026-11-01 ("try weighted ordering to lower the F4 cost", "try F5
or a signature-based solver instead"); protocol amendment A4 (PROTOCOL.md section 13)
frozen by commit a4ea49137518f9d99eeca45fe735abb6b8ee581e.

Working directory: every command in sections 1, 3 and 5, and P2 to P4 in section 2, runs
with the tool's working directory set to EXPDIR. Do not write `cd ... &&` or `cd ... ;` in
front of any command. Run each command in its own tool call. The `&` appears only at the end
of command a in section 3, exactly as written. P1 and P1b use absolute paths.

## 1. Bindings (sha256, files under EXPDIR)

| file | sha256 |
|---|---|
| PROTOCOL.md | 27f7ed23170661172f5be124f9a978cccb2f7a605e223a8b8e765b0545ed2c42 |
| run_queue.sh | 7fccd2f11130e5ed7e91b78fffa7ece7336b1ad3fedda3741d8b6d9683f5d7c4 |
| run_probes.sh | 4ce34e77b9d4ebe98052a1f3fbd5b3dc489f6e19b6c9d80d7f63654297af9e17 |
| run_weighted.sh | f20a0e413a1af43a7d13aa5f42ea96c271bea145f334d8d0d129702ab69dd09f |
| run_cells.sh | eedcc4ad5d3a522ae8ebad99aa7ed938f1de688a385e047610ffdc86ae18cd49 |
| ms2sing.sh | eacb5a8c6f129092c26a00796002881998916af914b367a7a438b8d1abca077a |
| bin/qsolve | 630f3a9da1272452a58bab9a46008738c292199f7a7c3a82d9fe979c016a9d35 |
| bin/qsolve-w | c69c885cf59ead5fdd2812a62df8da36efe92ebff806d9bc2a5621212c884a06 |

Check command: `sha256sum PROTOCOL.md run_queue.sh run_probes.sh run_weighted.sh run_cells.sh ms2sing.sh bin/qsolve bin/qsolve-w`
Rule: stop with `specification_error` if any printed hash differs from the table.

## 2. Preconditions (each is one command)

P1. `git -C /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr rev-parse HEAD`
    expected: `a4ea49137518f9d99eeca45fe735abb6b8ee581e`
    If it prints anything else, run P1b; if P1b fails, stop with `specification_error`.
P1b. `git -C /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr diff --name-only a4ea49137518f9d99eeca45fe735abb6b8ee581e HEAD -- research/ecgfp5_quotient_solve_20261101/PROTOCOL.md research/ecgfp5_quotient_solve_20261101/run_queue.sh research/ecgfp5_quotient_solve_20261101/run_probes.sh research/ecgfp5_quotient_solve_20261101/run_weighted.sh research/ecgfp5_quotient_solve_20261101/run_cells.sh research/ecgfp5_quotient_solve_20261101/ms2sing.sh`
    expected: exit status 0 and no output lines.
P2a. `pgrep -f 'run_queue[.]sh'`
    expected: no output (exit status 1): no queue driver is running. Use exactly this pattern with the
    brackets; the bracket form does not match the executor's own wrapper shell. If it prints a pid, stop
    with `precondition_failed` and run nothing in section 3.
P2b. `pgrep -x msolve`
    expected: no output (exit status 1). If it prints a pid, stop with `precondition_failed`.
P2c. `pgrep -x Singular`
    expected: no output (exit status 1). If it prints a pid, stop with `precondition_failed`.
P3 (must_not_exist). `ls -d results/m5_p262231_planted results/m5_p33554371_planted results/m5_p4111_sba results/m5_p4111_weighted results/queue.log`
    expected: five lines `ls: cannot access 'results/...': No such file or directory`, one per path, and no path printed as existing.
    If any of the five paths exists, stop with `precondition_failed`.
P4a. `dpkg-query -W -f='${Version}' msolve`
    expected: `0.6.5-1build2`
P4b. `dpkg-query -W -f='${Version}' singular`
    expected: `1:4.3.2-p10+ds-1.1build1`

Any precondition or hash mismatch: run nothing in section 3; go to section 5 and report the failing command and its output.

## 3. Commands (verbatim, each in its own tool call, in this order, tool working directory = EXPDIR)

a. `nohup ./run_queue.sh > results/queue.log 2>&1 < /dev/null &`
   Detached. The tool call returns at once. Do NOT wait for it, do NOT poll it, do NOT kill it.
b. `sleep 3; pgrep -f 'run_queue[.]sh'; pgrep -x msolve; head -3 results/queue.log`
   timeout 30 s. Report the pgrep output and the head output as printed (empty output is reported as empty).
   Then return. The queue runs for hours; do not wait for it.

Stop rules (observable facts):
- Run a only if all hashes and P1 (or P1b), P2a, P2b, P2c, P3, P4a, P4b passed.
- Run b only after a has returned.
- No retries. No edits to any file. No reruns. No second launch. No commands other than those in sections 1, 2, 3 and 5.

## 4. Declared outputs (paths only)

- results/queue.log
(The queue itself later creates results/m5_p262231_planted/, results/m5_p33554371_planted/,
results/m5_p4111_sba/ and results/m5_p4111_weighted/; the executor does not wait for them and
does not list them.)

## 5. Report back (verbatim, no interpretation, no summary)

Return, in this order:
1. The output of the section 1 sha256 check and of P1, P2a, P2b, P2c, P3, P4a, P4b (and P1b if run).
2. The exit status of a and of b.
3. The pgrep output of b (the lines printed by its two pgrep commands, before the head lines).
4. The head output of b.
Steps not run are reported as "not run" with the stop rule that applied.

## 6. Write scope

Only this directory may be created or changed:
- /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr/research/ecgfp5_quotient_solve_20261101/results/

No git operations (no add, commit, push, checkout, reset). Do not edit any other file.
