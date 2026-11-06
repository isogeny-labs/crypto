# Resume: crypto @ main e87fa18a73 (IC line-layout tournament v6, V6-AMENDMENT-1 merged in #3531)

## Goal
Run IC-LINE-LAYOUT-E2E-FP28-V6-20261106. It tests whether `layout=lines` lowers the fp28 IC wall time against the walk incumbent at equal counted cost.

## Lead / hypothesis
Candidate: `mitm:negation_folded=1,layout=lines` against the incumbent `prime-abscissa:size=407`, `mitm:negation_folded=1`, `targets=walk`, `max_trials=26479085`.
- Confirm: both wall sessions pass analyze-v6.sh.
- Kill: per the PROTOCOL-V6.md stopping rule.

## Spec
- Protocol: research/ic_line_layout_tournament_20261031/protocol-v6.json, frozen (sha256 12415faa…, commit 7c173e5a1d, merged in PR 3527 as f352040f06).
- Decision: DEC-20261106-662e33.
- Specs: spec-fp28-v6{,-qual,-counted}.json. Launch requests: launch-v6/.
- Pinned image: ea8e9ef7 / ecbench 73362eed.

## State
- Qualification passed (§5.1). Wall s1 and s2 exit 0, both on node ip-10-84-67-216, data SEALED (S3 runs/ic-line-v6-s{1,2}-20261106/).
- Counted companion ic-line-v6-counted-20261106: exit 42 (accepted by §7), audit OK, node ip-10-84-72-222.
- V6-AMENDMENT-1 frozen (protocol_sha256 2aedaaf8…) and merged in #3531 at 05:33Z. The owner approved launching s3 after the merge.
- A worker restart at about 05:50Z wiped /dev/shm: the old cards, clones and downloads are gone. Work now lives in the session scratchpad v6/ on the main disk.

## Next steps
1. Director rewrites card-s3 for the new paths; launch ic-line-v6-s3-20261106 via executor-mechanical.
2. After s3 has written job-exit.json: node check, then analyze-v6.sh on (s1,s2) and (s3,s1), validator, report and results PR.

## Runs in flight
- None at checkpoint time; s3 is about to launch.

## Stop and success conditions
- Budget: USD 5 ceiling, rolling hard-bound check (§14.1). Spend so far USD 0.155.
- The qualification requires all executions at L2 and a minimum solve of at least 5 ms.
- Never redispatch a consumed registration, and never auto-retry an ambiguous submission.

## Open questions
- Work is in the session scratchpad on the main disk (about 7 GB free); /dev/shm does not survive restarts.
- /home/user/crypto-autoresearcher is half-switched, pending the owner.
