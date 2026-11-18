#set page(paper: "a4", margin: 18mm)
#set text(font: "Linux Libertine", size: 9.5pt)
#set heading(numbering: "1.")

= Branch-controls successor machine review

#text(size: 8pt, fill: rgb("555555"))[
Frozen source `d20691095b0e...` · release `bcbe1b0b3a1d...` · published revision `d6365d5265bb...`
]

== Outcome

The sealed successor completed its three bounded phases in *287.195459435 worker wall seconds*. Four Lean modules exited zero. The deliberate wrong-branch module exited one with the expected `False` goal unsolved. All five child process groups were reaped.

The exact *13 requested declaration readbacks* were present. Each reported only `propext`, `Classical.choice`, and `Quot.sound`. This report audits the frozen native receipts without rerunning Lean.

#image("flow.svg", width: 100%)

== Recorded checks

#table(
  columns: (2.2fr, 1.3fr, 1.3fr),
  inset: 5pt,
  stroke: 0.4pt + rgb("cccccc"),
  table.header([*Check*], [*Observed*], [*Expected*]),
  [Positive modules], [4 exit 0], [4 exit 0],
  [Deliberate negative], [exit 1; unsolved `False`], [exit 1],
  [Named axiom readbacks], [13 exact names], [13],
  [Dependency custody], [16,736 files], [same pre/post],
  [Dependency bytes], [3,111,892,318], [same pre/post],
  [Attempt wall time], [287.195459435 s], [at most 600 s],
)

Every pre/compiler/post phase receipt recorded `passed`, no wait timeout, and terminal owner confirmation. The systemd commands retained 100% CPU quota, 8 GiB memory, zero swap, 128 tasks, and control-group kill mode.

== Exact theorem scope

The principal theorem is conditional on a field, a simple branch root, a separable quintic, a nonzero coefficient determinant, two support divisibilities, and vanishing of the support product at the branch. Under those hypotheses, the two source support polynomials and the residual are nonzero, and their three root multiplicities sum to one.

The rational control establishes those hypotheses for one fixed source instance. Its negative controls establish that the equal-support coefficient choice has zero determinant and that the selected wrong branch is not a root.

The machine evidence establishes elaboration of these exact frozen statements and their named axiom readback. It does not independently derive a bridge to broader collector claims, and no separate binary kernel rechecker was used.

== Custody boundary

The compiler-phase receipt reports zero dependency files checked. Custody is therefore not attributed to that receipt. The distinct pre and post phases each read and hashed all 16,736 declared files; their complete maps, retained-import maps, and runtime maps are equal.

Those are observations of pinned shared caches. They do not exclude a transient concurrent change that occurs and is restored between the two observations.

== Accounting and standardized artifacts

The conservative charge is 600 worker seconds. With the preceding 600-second attempt, the derived total is 1,200 charged seconds and 128,400 seconds remaining under the 129,600-second campaign ceiling. The pre-launch ledger still labels this attempt reserved; ledger transition remains coordinator-owned.

The machine-readable record is `review.json`; exact rows are in `review.csv`; the deterministic audit is `audit.py`. No compiler, solver, or experiment was executed by this reviewer.
