#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 11pt)
= N83 planted four-summand control: bounded outcome

#image("planted-control.svg", width: 100%)

The exact K = 16,627 stored signed-Frobenius base and five compiled worker
sources passed the frozen hash checks. The native release worker ran in a
one-CPU, 4 GiB, zero-swap container. Its pinned positive arm reached the
1,800-second wall cap and exited 124 without OOM. Cgroup memory peaked at
2,660,335,616 bytes. No worker JSON or phase receipt was written.

*Status:* `UNKNOWN_wall_cap`. The stopped phase, SAT conflicts, candidate
models, group relation and rank are unknown. The unpinned arm was not
attempted because the positive-control prerequisite was unverified.
This is a censored experiment, not a natural relation-yield estimate or a
complete IC cost. The next experiment records phase boundaries and fixes the
complete planted x-coordinate chain before SAT search.

Reproduction pointers: `research/n83_k16627_m4_planted_20261017/PROTOCOL.md`,
`pinned/outcome.json`, `pinned/container-state.json`,
`BUILD_RECEIPT.md` and `RUN_REPORT.md`.
