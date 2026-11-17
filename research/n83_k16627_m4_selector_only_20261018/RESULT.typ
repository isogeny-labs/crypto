#set page(paper: "a4", margin: 14mm)
#set text(font: "New Computer Modern", size: 10pt)
= Result: N83 K = 16,627 selector-only control

*Run:* 2026-10-18 09:21:26--09:26:49 UTC. *Result:* `PASS_THREE_CHECKED_ARMS`.
The exact four-summand model had 201,598 variables, 5,915,238 clauses and
3,901 XOR rows before x and choice units. A1 and A2 pinned four
representative selectors and 28 phase-zero bits. B pinned the selectors
while leaving all phase bits free. All three returned the same checked
rank-one relation for public indices `[0, 166, 332, 498]`.

#image("selector-only-comparison.svg", width: 100%)

B's SAT call took 6,487.732878 ms with 517 completed decisions,
222 conflicts and 826 completed XOR passes. A1 and A2 took 10.045541 and
17.313917 ms, with zero decisions and three passes each. Worker walls,
including retained-base import, were 38,614.439768, 42,786.035270 and
36,311.395725 ms. All three one-CPU, hard-4-GiB, zero-swap containers
exited normally without OOM. The separate native verifier replayed each
relation and rejected changed-x and changed-row receipts. B returned before
its cap, so the frozen conditional phase-only C arm was omitted.

This is a constructed-witness solver-stage result. The worker did not
export SAT assignment bits, and the separate verifier shares the library's
field and curve arithmetic. Natural relation yield, useful rank, a selected
prebuilt base across m = 2, 3 and 4, complete cold IC cost and matched rho
remain open. See `RUN_REPORT.md`, `summary.json` and `raw-files.sha256` for
exact sources, commands, receipts and accounting.
