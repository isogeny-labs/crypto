#set page(paper: "a4", margin: 15mm)
#set text(font: "New Computer Modern", size: 9.5pt)
= N83 K = 16,627: root-propagation discriminator

*Run:* 2026-10-18 UTC. *Curve:* `icv1-f2m83-tm6151469093347-debefd74`.
The frozen protocol is `PROTOCOL.md`. The exact retained signed-Frobenius
base has 16,627 columns and 2,760,082 points. The accepted native worker
source is clean pre-stack commit
`43c9850a73f1d960f65ed470fdb8adac981a5ffe`;
`source-files.sha256`, `inputs-before-run.sha256`, `BUILD_RECEIPT.md`,
`VALIDATION.md` and `COMMANDS.md` retain source, object, binary, build and
test checks. The complete source-linked account is `RUN_REPORT.md`.

#image("root-phase.svg", width: 100%)

== Frozen positive arm

#table(
  columns: (auto, auto),
  inset: 5pt,
  [Measure], [Observed value],
  [Status], [`UNKNOWN_wall_cap`; exit 124; no OOM],
  [UTC interval], [05:22:09–05:52:09],
  [CPU / hard memory / swap], [1 / 4 GiB / 0],
  [Cgroup peak], [2,661,863,424 bytes],
  [Retained object], [83,701,045 compressed bytes; 16,627 orbit columns],
  [Pre-SAT group check], [rank 1; tampered target rejected],
  [Full m=4 model], [201,598 variables; 5,915,238 clauses; 3,901 XOR rows],
  [Root propagation], [`Fixpoint` in 185.023 ms; zero decisions and conflicts],
  [Ordinary SAT entry], [35.437 s elapsed; no solver return by outer cap],
)

== Interpretation

The initial level-zero root pass returned on this exact seven-block
witness in the instrumented run. That makes it an unlikely explanation
for the preceding arm's cap, but the earlier arm had no root marker.
A later decision, propagation, conflict or restart path still needs
completed progress receipts. Post-root decisions, conflicts and models
remain unknown. The pre-SAT rank-one check is
a planted group preflight, not a returned SAT relation or natural-query
useful rank. A cap is an unresolved bounded outcome; this run supplies no
factor-base winner, complete index-calculus cost or matched-rho ratio.

*Raw source:* `positive/worker.json.phases.jsonl`,
`positive/container-state.json`, `positive/outcome.json` and
`positive-files.sha256`.
Stdout/stderr, cgroup peak and the exact UTC interval are retained
beside them. Failed worker builds remain linked from `BUILD_RECEIPT.md`.
