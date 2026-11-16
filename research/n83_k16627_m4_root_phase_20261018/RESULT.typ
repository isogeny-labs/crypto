#set page(paper: "a4", margin: 15mm)
#set text(font: "New Computer Modern", size: 9.5pt)
= Result: N83 K = 16,627 root-phase control

*Status:* `UNKNOWN_wall_cap`, 2026-10-18 05:22:09–05:52:09 UTC.
The exact retained signed-Frobenius object and the frozen m=4 seven-block
witness passed group and model preflight. The root pass returned a
`Fixpoint` in *185.023 ms* with zero decisions and conflicts. Ordinary
SAT then entered at 35.437 s and did not return before the 1,800-second
outer wall cap. Exit was 124, `OOMKilled=false`, and cgroup peak was
2,661,863,424 bytes. No worker result or post-SAT group lift was produced;
the dependent unpinned arm was not attempted.

#image("root-phase.svg", width: 100%)

This is a phase diagnostic for one pinned positive control. The pre-SAT
rank-one group check does not measure natural useful rank. Post-root
decisions, conflicts, models and rank remain unknown. The next frozen
control must retain completed search progress before trying a policy
change or comparing factor-base quality. This result selects no m=2,
m=3 or m=4 factor base and supplies no full index-calculus cost ratio.

*Audit:* `PROTOCOL.md`, `RUN_REPORT.md`, `positive/outcome.json`,
`positive/worker.json.phases.jsonl`, `positive-files.sha256`, `source-files.sha256`,
`inputs-before-run.sha256`, `VALIDATION.md` and `COMMANDS.md`.
