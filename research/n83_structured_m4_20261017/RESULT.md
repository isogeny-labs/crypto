# N83 structured-base m=4 gate

The native worker imported the retained 2,001-column, 332,166-point
signed-Frobenius base and ran the frozen four-summand chain on the primary
`GF(2^83)` curve. Eight of eight queries reached 5,000 SAT conflicts each
and returned `Unknown`, for 40,000 completed conflicts and 784.331 seconds
of shared-host process wall. The worker exited 0 without OOM, reported no
accepted relation row within the cap and recorded rank 0 of 2,002 columns.

The unchanged five-summand source-path control completed four 5,000-conflict
`Unknown` calls. Its fifth call was interrupted at the 900-second worker
budget; the container exited 124 without OOM. It has no completed rank or
per-phase result. These are bounded observations on different arities and
do not select a factor base or establish a speed comparison.

The [run report](RUN_REPORT.md), [raw m=4 result](run4/result.json),
[m=5 cap](run5/cap.json), [event streams](run4/events.jsonl) and
[diagram](run-diagram.svg) provide the audit path. Source and object hashes,
caps, import integrity, environment, per-phase accounting and limitations
are in the report and [frozen protocol](PROTOCOL.md). No graphable verified
relation or rank increment was produced, so the canonical progress graphs
and index-calculus scoreboard remain unchanged.

**Next iteration:** compare this base with the near-size K=2,048 retained
explicit-orbit object using the same four-summand oracle and a degree-83
planted relation control. In parallel, construct and replay the K=16,627
signed-orbit base identified by the four-summand size bound, then gate its
native m=4 adapter and full resource cost.
