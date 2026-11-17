#set page(paper: "a4", margin: 15mm)
#set text(font: "New Computer Modern", size: 9.5pt)
= Result: N83 K = 16,627 search-progress control

*Status:* `UNKNOWN_wall_cap`, 2026-10-18 06:56:55–07:26:55 UTC.
The stored base and seven-x-block m=4 witness passed group and model
preflight. Root propagation returned `Fixpoint` in *199.977792 ms* with
zero decisions or conflicts. Ordinary SAT began at 39.623697 s.

#image("search-progress.svg", width: 100%)

The one-CPU worker reached its 1,800-second cap: exit 124, no OOM,
2,662,309,888-byte cgroup peak. All 175 periodic samples landed in XOR
propagation. The last complete sample, at 1,792.112144 s, recorded
*322,255 completed decisions, 360 conflicts and two restarts*.
The samples show recurrent search and XOR activity, not an exact CPU-time
fraction. Later work, SAT answer, group lift and returned-relation rank
remain unknown. The unpinned arm was not attempted.

This stage diagnostic selects no factor base and provides no full index-
calculus cost ratio. The next discriminating run should mirror completed
XOR-pass time, row scans, repivots and row operations at stable boundaries.
Natural relation yield and useful rank on the retained bases remain open.

*Audit:* `PROTOCOL.md`, `RUN_REPORT.md`, `positive/outcome.json`,
`positive/worker.json.progress.jsonl`, `positive-files.sha256`,
`source-files.sha256`, `inputs-before-run.sha256`, `VALIDATION.md`,
`COMMANDS.md`.
