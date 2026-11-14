# Resume: crypto @ claude/f4-f5-algorithm-speed-bvp7qd

## Goal
Make the GHS-weak plane-quartic double-large-prime IC (research/quartic_dl1_cd_sqrt_20261113, c_D = 1.7 + sqrt chain)
faster in measured CPU time at identical counted work, and record the factor-base-growth null result.

## Lead / hypothesis
Pilot workflow wf_f57f915b-f9f (2026-11-14, L0, not a claim): (1) a compiled sqrt chain gave relations-stage CPU
0.72-0.92x; (2) fast open-addressing hashing + allocation removal gave 0.56-0.72x; both have identical counted fields.
(3) Incremental factor-base growth: no robust counted gain, CPU 1.16-1.52x (null). Confirm (1),(2) and a combined
build under frozen protocols; write up (3) as a null result.

## Spec
Three new study dirs, one per lane, each with PROTOCOL.md committed before data:
research/ic_gw_sqrt_compiled_20261114, research/ic_gw_fasthash_20261114, research/ic_gw_fbgrow_20261114.
Reference binary: v3 sha256 de500bc1... (/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/j2/layer/opt/study/bin/v3). Candidates built from /tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/wf/{chain,hash,grow}/src.
Sizes q = 262111 524287 1048507 2097091 4194271 (k = 18..22), seeds 1-3, c_D 1.7, args "ic q seed D 3.0 c".

## State
Pilots done (raw under /tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/wf/). Measured runs are being launched by a three-lane workflow (Sonnet agents);
timed units are serialised with flock on /tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/wf/timing.lock.

## Next steps
1. Lanes freeze protocol, run, analyze, report/graph/PDF, benchrec record (each in its own study dir).
2. Main session: review, validation gate, leaderboard component section, session log, PR (ready, auto-merge).

## Runs in flight
Workflow lanes in this container (local L0, owner chose local because the EKS budget is exceeded, launch/BUDGET-HOLD.md
in the quartic_dl1_cd_sqrt study). Outputs under each study dir's runs/.

## Stop and success conditions
Success: relations-stage CPU ratio < 1 at every (k, seed) with identical counted fields, outside the A/A floor.
No cloud spend.

## Open questions
Whether to repeat on EKS (L2) once the budget allows.
