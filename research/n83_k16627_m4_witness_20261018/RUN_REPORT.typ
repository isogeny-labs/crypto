#set page(paper: "a4", margin: 15mm)
#set text(font: "New Computer Modern", size: 9.5pt)
= N83 K = 16,627: phase-observable complete witness

*Run:* 2026-10-18 UTC. *Curve:* `icv1-f2m83-tm6151469093347-debefd74`.
The protocol (`PROTOCOL.md`) preceded implementation. The exact retained
signed-Frobenius object has 16,627 columns and 2,760,082 points. Worker
source is bound to clean pre-stack commit `f68b9a94ada669360f88be4d8c8a5288cca55d73`;
`source-files.sha256`, `inputs-before-run.sha256`, `BUILD_RECEIPT.md` and
`COMMANDS.md` retain source, input, binary, build and test checks. The
complete source-linked account is `RUN_REPORT.md`.

#image("witness-phases.svg", width: 100%)

== Frozen positive arm

#table(
  columns: (auto, auto),
  inset: 5pt,
  [Measure], [Observed value],
  [Status], [`UNKNOWN_wall_cap`],
  [UTC interval / outer exit], [03:16:27–03:46:27 / 124],
  [CPU / hard memory / swap], [1 / 4 GiB / 0],
  [Cgroup peak / OOM], [2,661,351,424 bytes / false],
  [Base import / group preflight], [39.982 s / 39.999 s elapsed],
  [Full model / SAT entry], [40.252 s / 40.267 s elapsed],
  [Model size], [201,598 variables; 5,915,238 clauses; 3,901 XOR rows],
  [Group preflight], [rank 1; tampered R+G rejected],
  [SAT return / conflicts / models], [unknown / unknown / unknown],
  [Worker JSON / unpinned arm], [absent / not attempted],
)

`positive/worker.json.phases.jsonl` contains five appended, flushed records.
The full orbit model and seven x-block pinning completed before the solver
was entered. No solver-return record was written before the outer cap. The
delay lies within the solver call, but the receipt cannot distinguish its
initial propagation from later conflict search. No SAT-produced field model,
group lift or post-SAT rank was returned. The positive prerequisite for an
unpinned search was therefore unverified.

The outer outcome, container state, host configuration, timestamps, cgroup
peak and empty stdout/stderr are retained under `positive/` and beside this
report. These shared-host L0 walls are descriptive, not a complete IC cost
or fastest-tool comparison. A separately frozen root-propagation checkpoint
and completed conflict receipts are the next discriminator. Natural-query
yield and useful-rank comparisons across the K=16,627 and near-size-matched
bases remain open; the canonical IC scoreboard is unchanged.
