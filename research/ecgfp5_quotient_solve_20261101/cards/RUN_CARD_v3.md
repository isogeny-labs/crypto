# RUN CARD v3: ecgfp5_quotient_solve_20261101 (frozen command card, amendment A2)

Executor tier: mechanical. Do exactly what is written. Do not interpret, retry,
edit, repair or choose. If a step says stop, stop and report the fact.
This card replaces nothing: v1 and v2 stay as they are and are not to be run.

CHECKOUT = /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr
EXPDIR   = CHECKOUT/research/ecgfp5_quotient_solve_20261101

Approval: owner request 2026-11-01 ("try weighted ordering to lower the F4 cost");
protocol amendment A2 (PROTOCOL.md section 11) frozen by commit
5115f58ebed273023929b61820cdb0736b279709.

Working directory: every command in sections 1, 3 and 5, and P2, P3, P4 in section 2, runs
with the tool's working directory set to EXPDIR. Do not write `cd ... &&` or `cd ... ;` in
front of any command. Run each command in its own tool call. The `&` appears only at the end
of command a in section 3, exactly as written. P1 and P1b use absolute paths.

## 1. Bindings (sha256, files under EXPDIR)

| file | sha256 |
|---|---|
| PROTOCOL.md | baaab8a2b32182b86bfa218bf64e29bef0a735832adb83c033d43c051b2f7349 |
| run_weighted.sh | f20a0e413a1af43a7d13aa5f42ea96c271bea145f334d8d0d129702ab69dd09f |
| run_cells.sh | eedcc4ad5d3a522ae8ebad99aa7ed938f1de688a385e047610ffdc86ae18cd49 |
| src/qsolve.rs | dff49f203567e0a262692b53070843c59a3610b1e4f540899965127576e2b51c |
| bin/qsolve | 630f3a9da1272452a58bab9a46008738c292199f7a7c3a82d9fe979c016a9d35 |
| bin/qsolve-w | c69c885cf59ead5fdd2812a62df8da36efe92ebff806d9bc2a5621212c884a06 |

Check command: `sha256sum PROTOCOL.md run_weighted.sh run_cells.sh src/qsolve.rs bin/qsolve bin/qsolve-w`
Rule: stop with `specification_error` if any printed hash differs from the table.

## 2. Preconditions (each is one command)

P1. `git -C /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr rev-parse HEAD`
    expected: `5115f58ebed273023929b61820cdb0736b279709`
    If it prints anything else, run P1b; if P1b fails, stop with `specification_error`.
P1b. `git -C /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr diff --name-only 5115f58ebed273023929b61820cdb0736b279709 HEAD -- research/ecgfp5_quotient_solve_20261101/PROTOCOL.md research/ecgfp5_quotient_solve_20261101/run_weighted.sh research/ecgfp5_quotient_solve_20261101/run_cells.sh research/ecgfp5_quotient_solve_20261101/src/qsolve.rs`
    expected: exit status 0 and no output lines.
P2. `ps -o comm= -p 4562`
    expected: `qsolve` (the running baseline 4111 solve).
    If it prints nothing or anything else, stop with `precondition_failed` and run nothing in section 3.
P3 (must_not_exist). `ls -d results/m5_p4111_weighted results/m5_p262231_planted results/weighted_drive.log`
    expected: three lines `ls: cannot access 'results/...': No such file or directory`, one per path, and no path printed as existing.
    If any of the three paths exists, stop with `precondition_failed`.

Any precondition or hash mismatch: run nothing in section 3; go to section 5 and report the failing command and its output.

## 3. Commands (verbatim, each in its own tool call, in this order, tool working directory = EXPDIR)

a. `nohup ./run_weighted.sh drive 4562 > results/weighted_drive.log 2>&1 < /dev/null &`
   Detached. The tool call returns at once. Do NOT wait for it, do NOT poll it, do NOT kill it.
b. `sleep 2; pgrep -f 'run_weighted.sh drive' ; head -5 results/weighted_drive.log`
   timeout 30 s. Report the pgrep output and the head output as printed (empty output is reported as empty).
   Then return. The driver runs for hours; do not wait for it.

Stop rules (observable facts):
- Run a only if all hashes and P1 (or P1b), P2, P3 passed.
- Run b only after a has returned.
- No retries. No edits to any file. No reruns. No second launch. No commands other than those in sections 1, 2, 3 and 5.

## 4. Declared outputs (paths only)

- results/weighted_drive.log
(The driver itself later creates results/m5_p4111_weighted/ and results/m5_p262231_planted/;
the executor does not wait for them and does not list them.)

## 5. Report back (verbatim, no interpretation, no summary)

Return, in this order:
1. The output of the section 1 sha256 check and of P1, P2, P3 (and P1b if run).
2. The exit status of a and of b.
3. The pgrep output of b.
4. The head output of b.
Steps not run are reported as "not run" with the stop rule that applied.

## 6. Write scope

Only this directory may be created or changed:
- /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr/research/ecgfp5_quotient_solve_20261101/results/

No git operations (no add, commit, push, checkout, reset). Do not edit any other file.
