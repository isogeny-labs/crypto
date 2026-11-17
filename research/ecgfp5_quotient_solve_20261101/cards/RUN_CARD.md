# RUN CARD: ecgfp5_quotient_solve_20261101 (frozen command card)

Executor tier: mechanical. Do exactly what is written. Do not interpret, retry,
edit, repair or choose. If a step says stop, stop and report the fact.

CHECKOUT = /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr
EXPDIR   = CHECKOUT/research/ecgfp5_quotient_solve_20261101

Approval: owner request "yes, set it up and run it" (2026-10-31); protocol frozen by
commit 7144c8a61be08fa10e3e1a12e727b2790a83182b (PROTOCOL.md).

## 1. Bindings (sha256, files under EXPDIR)

| file | sha256 |
|---|---|
| PROTOCOL.md | 80ef375eb4cdbdd33c520100b75de4fd06a67c1c834a75f3715397fde09f8492 |
| run_cells.sh | f7edb156c5e872afad2b8944d05f1e0a0116f58de60657d79dfe639f8a1b93fe |
| build.sh | 22cd9be55e1cf3f56c755923b17969bad160807af47fef2dfda8ed43d6e0d171 |
| src/qsolve.rs | d0a0e900efebb7f1608a35bad597cc53b9e1c3e8222ee23f9ae7155c18478db6 |

Check command (run from EXPDIR): `sha256sum PROTOCOL.md run_cells.sh build.sh src/qsolve.rs`
Rule: stop with `specification_error` if any printed hash differs from the table.

## 2. Preconditions (run from anywhere; each is one command)

P1. `git -C /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr rev-parse HEAD`
    expected: `7144c8a61be08fa10e3e1a12e727b2790a83182b`
    If it prints anything else, run P1b; if P1b fails, stop with `specification_error`.
P1b. `git -C /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr diff --name-only 7144c8a61be08fa10e3e1a12e727b2790a83182b HEAD -- research/ecgfp5_quotient_solve_20261101/PROTOCOL.md research/ecgfp5_quotient_solve_20261101/run_cells.sh research/ecgfp5_quotient_solve_20261101/build.sh research/ecgfp5_quotient_solve_20261101/src/qsolve.rs`
    expected: exit status 0 and no output lines (HEAD is then a descendant that does not touch the bound files).
P2. `dpkg-query -W -f='${Version}' msolve`
    expected: `0.6.5-1build2`
P3. `ls /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr/research/ecgfp5_quotient_solve_20261101/results`
    expected (must_not_exist): `ls: cannot access ... No such file or directory`. If results/ exists, stop with `precondition_failed`.

Any precondition or hash mismatch: run nothing in section 3; go to section 5 and report the failing command and its output.

## 3. Commands (verbatim; working directory for every command is EXPDIR; run in this order)

a. `./build.sh`
   timeout 600 s. Capture: full stdout and stderr (rustc version and two sha256 lines).
b. `./run_cells.sh host`
   timeout 60 s. Capture: full stdout and stderr.
c. `./run_cells.sh m3 > results/m3.stdout 2>&1`
   timeout 3600 s. Capture: the exit status. The output is in results/m3.stdout.
d. `nohup ./run_cells.sh m5all > results/m5all.log 2>&1 & echo $! > results/m5all.pid`
   Run detached. Record the pid printed in results/m5all.pid. Do NOT wait for it and do
   NOT poll it; return as soon as the command returns (the cells take up to 3 x 4 h).

Stop rules (observable facts):
- If a, b or c exits nonzero (or hits its timeout), do not run any later command; go to section 5.
- Run d only if a, b and c all exited 0.
- No retries. No edits to any file. No reruns. No other commands except those in sections 2, 3 and 4.

## 4. Declared outputs (executor lists paths only, from EXPDIR)

- results/host-*.json
- results/m3_p4111_planted/, results/m3_p4111_random/
- results/m3_p65551_planted/, results/m3_p65551_random/
- results/m3_p33554371_planted/, results/m3_p33554371_random/
  (each: meta.json, *.check.json, *.run.json, orbits.json)
- results/m3.stdout
- results/m5all.pid
- results/m5all.log

Listing command: `ls -d results/host-*.json results/m3_p*/ results/m3.stdout results/m5all.pid results/m5all.log` (missing paths are reported as the ls error text, nothing else).

## 5. Report back (verbatim, no interpretation, no summary)

Return, in this order:
1. The output of the section 1 sha256 check command and of P1, P2, P3 (and P1b if run).
2. The exit status of a, b, c, d (d: exit status of the launching shell line).
3. The stdout and stderr of a and b.
4. The full content of results/m3.stdout (`cat results/m3.stdout`).
5. The pid: `cat results/m5all.pid`.
6. `ls results/`
7. The output of the section 4 listing command.
Any nonzero exit of a, b or c is reported as printed; nothing is retried.

## 6. Write scope

Only these directories may be created or changed:
- /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr/research/ecgfp5_quotient_solve_20261101/results/
- /tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/crypto-pr/research/ecgfp5_quotient_solve_20261101/bin/

Do not git add, commit, push, checkout or reset. Do not edit any other file.
