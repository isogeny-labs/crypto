#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set par(justify: true)
#show heading.where(level: 1): set text(size: 12pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[Completing the archive's `bitop/v1` accounting]
  #v(2pt)
  `research/ic_acct6_archive_20261021` · study 2026-10-21 · tranche 1 run 2026-10-26 · report 2026-10-26 · class: accounting
]

= Question

Of the archive's verified, non-warm-up ecbench records (every
`research/**/records.jsonl`), how many price `complete` in `bitop/v1`, and how
can the rest be made complete? `PROTOCOL.md` (frozen before any re-run) keeps
two routes apart: *phase A*, new pricing rules for counters that committed
records already carry, with no primitive weight changed; and *phase B*,
re-running sessions on the instrumented binary to count work that was never
counted. This is accounting. It changes no measured cost and is not an
improvement of the index-calculus baseline.

Every number below is read from files written by the Rust example
`examples/ic_acct6_archive.rs`: `breakdown-before.json`,
`breakdown-after.json`, `plan.json`, `breakdown-tranche.json`, and the
generated `BEFORE_AFTER.md`, `PLAN.md` and `TRANCHE.md`. The figures only
plot them (`figures/plots.py`).

= Phase A: new rules, no runs

Rules: `docs/units/BITOPS.md`, "Extensions", 2026-10-21 (rules at `42f23b5e4`
against `4651c5d9f`); `committed_vectors_are_frozen` passes. Archive at main
`cde2881dd`: 705 session directories, 119 unreadable both before and after,
73,012 verified non-warm-up records.

#table(columns: 4, stroke: 0.4pt, inset: 4pt, align: (left, right, right, right),
  [], [*before*], [*after*], [*change*],
  [records], [73,012], [73,012], [0],
  [complete], [50,273], [50,306], [+33],
  [incomplete (total is a lower bound)], [22,682], [22,706], [+24],
  [unpriceable (error)], [57], [0], [−57],
)

#figure(image("figures/before_after_totals.svg", width: 100%),
  caption: [Pricing status of the 73,012 archive records before and after the
  phase-A rules (`breakdown-{before,after}.json`, `totals`). Exact counts.])

#figure(image("figures/before_after_categories.svg", width: 100%),
  caption: [The records the new rules touched, coloured by status after
  phase A. Extension-family rows from `by_method_family`; the CM-folded
  (`native_queries_uncharged`), projected-base
  (`projected_base_membership_checks_uncharged`) and isogeny-tower
  (`aux_supersingular`) rows from `by_reason` before (all three are 0 after).])

All 57 extension-family records now price: 33 (`rho.negation`) complete and 24
(`ic.pipeline`) incomplete. The 628 projected-base, 96 isogeny-tower and 477
CM-folded records no longer name an unpriced counter but stay incomplete: the
never-counted index-calculus items (`gauss_column_scans_and_pivot_inversions`,
`relation_row_construction`, `table_insert_attempts`, `walk_bookkeeping`,
`base_index_builds`) remain, and for CM-folded the native backend's unmetered
work is now named as `uncounted`. The rules made these records honest about
what is missing; they did not make them complete.

= Phase B: the re-run plan

`PLAN.md`: 396 sessions hold an incomplete record (22,706 records, 71.55 hours
of summed process wall time on the hosts that ran them). 337 are re-runnable
unchanged (22,013 records, 48.58 h), 17 of them leaderboard sources (1,383
records, 12.73 h). 59 are not: CM-folded sessions whose pinned native backends
are absent here, interrupted sessions with fewer records than their plan, and
two without a frozen spec.

#figure(image("figures/rerun_plan.svg", width: 100%),
  caption: [Incomplete records per re-run status group, from `plan.json` (396
  rows; the groups count rows by their recorded `rerun` field and sum their
  recorded `incomplete` and `est_hours`; the script asserts PLAN.md's totals
  396 / 22,706 / 71.55 h and 337 / 22,013). Hours are the original sessions'
  summed `process_wall_ns`, descriptive only.])

= Tranche 1: 2 of 34 sessions run

Card 1 never acquired the host's shared build lock and was stopped with no
session started. Card 2 ran 07:16–10:20 UTC; the lock was free long enough for
two sessions (22.8 s and 18.5 s of process wall time). The other 32 sessions of
the tranche were not launched.

Both re-runs (`p001`, `p002`, the n37 `ic.shared_rank` leaderboard sessions)
plan to the committed spec ids, and both audits pass (`ecbench verify --replay
2`: 384 records each, 2 replays reproduced, 0 problems). *Neither prices
complete.* The current binary's `ic.shared_rank` writes only the
`counted:base_index_builds` marker, so 520 of the 640 counted records still miss
the four other never-counted items; the 120 complete records are the rho arms.
The plan's `seen` column (0/6, 0/2) predicted this. No archive record is
superseded.

#figure(image("figures/tranche1.svg", width: 100%),
  caption: [Counted records of the two re-run sessions by pricing status
  (`breakdown-tranche.json`, `sessions`). L0 runs (`--cpus 1 --allow-busy`) on a
  shared host; counts only.])

= Limitations and next step

- *The tranche barely ran:* 2 of 34 planned sessions, and both stay
  incomplete because `ic.shared_rank` is under-instrumented. Phase B has not yet
  completed a single archive IC record.
- *Recommended amendment* (not applied to the frozen protocol): run first the
  24 re-runnable sessions whose incomplete configurations all have a complete
  record elsewhere (`seen` = total; 1,833 records, about 2.0 h), and instrument
  `ic.shared_rank`'s rank, row, walk and table paths before re-running its
  sessions, which include four of the leaderboard's first five.
- *119 unreadable session directories* (schema drift; listed in
  `BEFORE_AFTER.md`) are outside both breakdowns.
- *Validation gate.* No `VALIDATION.json`, no leaderboard or `benchmarks/`
  row: this is accounting, not a performance result.

= Evidence

#set par(justify: false)
`PROTOCOL.md`, `BEFORE_AFTER.md`, `PLAN.md`, `TRANCHE.md`,
`breakdown-{before,after,tranche}.json`, `plan.json`, `specs/`,
`sessions/p00{1,2}-mac_arm64_l0_01/` (with `verify.json`), `logs/`, `cards/`,
`run_tranche.sh`, `figures/plots.py` (plots recorded values only).
