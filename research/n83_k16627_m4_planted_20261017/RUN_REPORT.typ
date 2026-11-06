#set page(paper: "a4", margin: 15mm)
#set text(font: "New Computer Modern", size: 9.5pt)
= N83 K = 16,627: planted four-summand control

*Run:* 2026-10-18 UTC. *Curve:* `icv1-f2m83-tm6151469093347-debefd74`.
The protocol was committed before worker edits (`PROTOCOL.md`), and the exact
object digests were clarified before execution (`INPUT_ADDENDUM.md`). The
source snapshot binds clean commit `1322cb5681d31849a2f33a6b47e16f9e3d598e0a`.
The full account and links to raw records are in `RUN_REPORT.md`.

#image("planted-control.svg", width: 100%)

== Observed bounded result

#table(
  columns: (auto, auto),
  inset: 5pt,
  [Measure], [Pinned positive arm],
  [Status], [`UNKNOWN_wall_cap`],
  [Outer cap / exit], [1,800 s / 124],
  [Cgroup memory peak], [2,660,335,616 bytes],
  [OOM], [false],
  [Worker receipt], [absent],
  [Phase / conflicts / SAT models], [unknown / unknown / unknown],
  [Group relation / rank], [unknown / unknown],
  [Unpinned arm], [not attempted; positive prerequisite unverified],
)

The retained base has 16,627 signed-Frobenius columns and 2,760,082 points;
its exact compressed object and point-set digests were checked before the
run. The native arm64 release worker built offline; the touched exporter
passed 19/19 tests. The fresh worker container had one CPU, 4 GiB hard
memory, zero swap, no network and a read-only root. Its UTC interval was
01:11:04 to 01:41:04. `pinned/` contains the outer outcome, container state,
host config, cgroup peak and empty stdout/stderr.

The pre-stack release library suite passed 4,216 tests with 117 ignored.
After stacking on the capacity-study branch, its release build stopped at
26 inherited ecbench/Gaudry compilation errors before tests ran. The two
touched Rust files pass formatting; repository-wide formatting fails on
four unchanged files. `VALIDATION.md` links the complete logs and commands.

The worker wrote no phase or result JSON before the outer timeout. The
stopped phase cannot be identified from this receipt, and a SAT answer or
group relation cannot be inferred. The earlier full-model capacity pass is
a separate completed run; its cost cannot be combined with this censored
run into an end-to-end index-calculus measurement. The IC scoreboard remains
unchanged. The next frozen experiment should flush phase markers and pin
the planted intermediate x values so the complete chain witness can be
checked under a bounded SAT search.
