# RUN CARD v2: ecgfp5_quotient_solve_20261101 (frozen command card, amendment A1)

Executor tier: mechanical. Do exactly what is written. Do not interpret, retry,
edit, repair or choose. If a step says stop, stop and report the fact.
This card replaces cards/RUN_CARD.md (v1); do not use v1.

CHECKOUT = /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr
EXPDIR   = CHECKOUT/research/ecgfp5_quotient_solve_20261101

Approval: owner request "yes, set it up and run it" (2026-10-31); protocol amendment A1
(PROTOCOL.md section 10) frozen by commit c4d13025532ac399577652d2aeeca9c0ea4b6fc9.

Working directory: every command in sections 1, 3 and 5, and P2 and P3 in section 2, runs with the shell already in
EXPDIR. Set the tool's working directory to EXPDIR. Do not write `cd ... &&` or
`cd ... ;` in front of any command, and do not put `&` anywhere except where written.
P1 and P1b use absolute paths and run from anywhere.

## 1. Bindings (sha256, files under EXPDIR)

| file | sha256 |
|---|---|
| PROTOCOL.md | 8b3d9931df9b64c51003cf0ff0b74cb839d76d761a16f0be250a88eeaea51772 |
| run_cells.sh | eedcc4ad5d3a522ae8ebad99aa7ed938f1de688a385e047610ffdc86ae18cd49 |
| build.sh | 22cd9be55e1cf3f56c755923b17969bad160807af47fef2dfda8ed43d6e0d171 |
| src/qsolve.rs | d0a0e900efebb7f1608a35bad597cc53b9e1c3e8222ee23f9ae7155c18478db6 |

Check command: `sha256sum PROTOCOL.md run_cells.sh build.sh src/qsolve.rs`
Rule: stop with `specification_error` if any printed hash differs from the table.

## 2. Preconditions (each is one command)

P1. `git -C /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr rev-parse HEAD`
    expected: `c4d13025532ac399577652d2aeeca9c0ea4b6fc9`
    If it prints anything else, run P1b; if P1b fails, stop with `specification_error`.
P1b. `git -C /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr diff --name-only c4d13025532ac399577652d2aeeca9c0ea4b6fc9 HEAD -- research/ecgfp5_quotient_solve_20261101/PROTOCOL.md research/ecgfp5_quotient_solve_20261101/run_cells.sh research/ecgfp5_quotient_solve_20261101/build.sh research/ecgfp5_quotient_solve_20261101/src/qsolve.rs`
    expected: exit status 0 and no output lines.
P2. `dpkg-query -W -f='${Version}' msolve`
    expected: `0.6.5-1build2`
P3 (must_not_exist). Run this one command:
    `ls -d results/m3_p262231_planted results/m3_p33554371_planted results/m5all.pid results/m5_p4111_planted`
    expected: four lines `ls: cannot access 'results/...': No such file or directory`, one per path, and no path printed as existing.
    If any of the four paths exists, stop with `precondition_failed`.
    (The path results/_aborted_launch_20261101T0129Z/ and the files results/m3_p4111_*, results/m3_p65551_planted, results/m3.run1.stdout already exist; they are expected and are not touched.)

Any precondition or hash mismatch: run nothing in section 3; go to section 5 and report the failing command and its output.

## 3. Commands (verbatim, one command per tool call, in this order, shell in EXPDIR)

a. `./build.sh`
   timeout 600 s. Capture: full stdout and stderr; the exit status.
b. `./run_cells.sh m3 262231 33554371 > results/m3.run2.stdout 2>&1; echo "EXIT=$?" >> results/m3.run2.stdout`
   timeout 3600 s. The exit status to report is the EXIT= line: run `tail -n 1 results/m3.run2.stdout` and report it as printed.
c. `./run_cells.sh launch-m5all`
   timeout 60 s. It prints `launched m5all pid <N>` and writes results/m5all.pid itself.
   Do NOT wait for the chain, do NOT poll it, do NOT run anything else against it. Return as soon as the command returns.

Stop rules (observable facts):
- Run c only if a exited 0 AND the last line of results/m3.run2.stdout is exactly `EXIT=0`.
- If a exited nonzero, or the EXIT= line of b is anything other than `EXIT=0` (or is missing), or b hit its timeout: do not run c; go to section 5.
- If c exits nonzero or prints `refusing`, report it as printed.
- No retries. No edits to any file. No reruns. No commands other than those in sections 1, 2, 3 and 5.

## 4. Declared outputs (paths only)

- bin/qsolve
- results/m3_p262231_planted/, results/m3_p262231_random/
- results/m3_p33554371_planted/, results/m3_p33554371_random/
  (each: meta.json, *.check.json, *.run.json, orbits.json)
- results/m3.run2.stdout
- results/m5all.pid
- results/m5all.log

## 5. Report back (verbatim, no interpretation, no summary)

Return, in this order:
1. The output of the section 1 sha256 check and of P1, P2, P3 (and P1b if run).
2. The exit status of a; the EXIT= line of b (from the file); the exit status of c.
3. The stdout and stderr of a.
4. The full content of results/m3.run2.stdout (`cat results/m3.run2.stdout`).
5. The stdout of c.
6. `cat results/m5all.pid`
7. `ls results/`
Steps not run are reported as "not run" with the stop rule that applied. Nothing is retried.

## 6. Write scope

Only these directories may be created or changed:
- /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr/research/ecgfp5_quotient_solve_20261101/results/
- /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr/research/ecgfp5_quotient_solve_20261101/bin/

No git operations (no add, commit, push, checkout, reset). Do not edit any other file.
