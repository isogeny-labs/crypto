#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set par(justify: true)
#show heading.where(level: 1): set text(size: 12pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[Counting the five never-counted index-calculus items]
  #v(2pt)
  `research/ic_acct7_complete_counters_20261107` · 2026-11-07 · branch `claude/ic-complete-counters` · class: accounting
]

= Question

Item 6 (`research/ic_acct6_archive_20261021`) found 22,706 archive records
pricing incomplete in `bitop/v1` because the code never counted five
index-calculus items: the elimination's scans and pivot inversions, relation
row construction, table insert attempts, walk bookkeeping and base index
builds. Its two re-runs of `ic.shared_rank` sessions stayed incomplete because
that method wrote only one of the five markers. This change counts the items
where the current code still does not, and checks which archive
configurations now write every marker. It changes no measured cost.

= What the code now counts

`ic.shared_rank` writes every marker, each counter at the site that does the
work: the folded table's insert attempts (`oracle_setup`), the matrix's entry
scans and pivot-inversion steps after the full-rank check's reads
(`linear_algebra`), every 64-bit word its `StdRng` hands out through a
forwarding wrapper that leaves the stream unchanged (`relations`,
`target_query`), the rows' words, sums and `h·a` products
(`linear_algebra`, `target_relation_check`), the target's curve-equation
check, and the descent's column-log combination as two new counters,
`combination_modr_muls` and `combination_modr_adds` (BITOPS.md
"Extensions", 2026-11-07; no primitive weight changed; the frozen vectors and
the work-v1 fixture tests pass). The `gaudry-d4` and `gaudry-d4-bin`
oracles build no table in their set-up, so they now say so with the
`table_insert_attempts` marker; with large primes on, the binary oracle's
pivot store is uncounted and the item stays listed.

= Which archive configurations write every marker

The 106 distinct IC configurations in item 6's 396 session specs were run on
toy curves of the same kind by an `#[ignore]` diagnostic test (a check of code
paths, not a measurement). Figure 1 counts, over the 90 configurations both
runs covered, those whose record still lists each item.

#figure(image("figures/items_by_config.svg", width: 100%),
  caption: [Configurations 1–90 of `coverage/configs.jsonl` whose record lists each item. "main" is the intermediate run with the `ic.shared_rank` row as item 6's tranche recorded it on main (four items). Source: `coverage/items.tsv`, `coverage-intermediate.tsv`, `coverage-final.tsv`.])

Of the 97 configurations run on this branch, 68 price complete and 10 do not:
five deferred sparse solvers (`sparse-cado`, `wiedemann`, `lanczos`,
`filter+*`, whose `SparseRows::solve` has no per-site counters, so the
elimination item stays), `ic.k4_gaudry` and `ic.large_prime` (their own
pipelines count none of the five), `gaudry-d4-bin` (all five counted; solver
gaps remain) and two `cyclotomic` bases (an unpriceable size counter). 19
could not run inside the library test (ecbench-binary plug-ins, absent
`ic.cm_folded_*` native backends, one curve without an invariant subspace);
nine slow `gaudry-d4` configurations over $F_(p^4)$ were not run.

= The committed archive, before and after

#figure(image("figures/archive_totals.svg", width: 100%),
  caption: [Every `research/**/records.jsonl` at main `a2be66091` (876 session directories, 123 unreadable, 107,921 verified non-warm-up records) priced by `examples/ic_acct6_archive.rs breakdown` with main's rules and this branch's. Source: `breakdown-before.json`, `breakdown-after.json`, `BEFORE_AFTER.md`.])

#table(columns: 3, stroke: 0.4pt, inset: 4pt, align: (left, right, right),
  [], [*before*], [*after*],
  [complete], [78,245], [78,245],
  [incomplete], [27,676], [27,676],
  [unpriceable], [2,000], [2,000],
)

The breakdowns are identical apart from their label, as they must be: a
committed record carries the markers its binary wrote, and the only pricing
change is two rules for counter names no committed record carries. Records
become complete only by re-running on this binary; no leaderboard row's
pricing changed.

= Re-run tranche

The 24 re-runnable sessions whose incomplete configurations all have a
complete record somewhere (item 6's PLAN.md, about 2.0 h) were not run: this
host has no AWS identity and the resume-checkpoint gate refuses without an
upload token, the stated stop conditions. No archive record is superseded.

= Limits

Coverage was checked on toy curves, not on the sessions' curves. The sparse
solvers, `ic.k4_gaudry`, `ic.large_prime` and `ic.cm_folded_*` keep their
items, and `check_full_rank_base_columns`'s representative scan (one word a
base point) is not counted in `ic.shared_rank`'s verification phase.
