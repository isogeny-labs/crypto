#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set par(justify: true)
#show heading.where(level: 1): set text(size: 12pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[`bitop/v1` completeness of the symmetrised oracle]
  #v(2pt)
  `research/ic_acct7_symmetrised_20261021` · session 2026-10-21 · report 2026-10-26, repriced 2026-10-27 · class: accounting
]

= Question

`PROTOCOL.md` section A (frozen 2026-10-21, before any data): after commit
`fd93ddfe2` adds counters for the `koblitz-symmetrised` base build, the
symmetrised oracle's `prepare`, the symmetrised system build and the degree
scan, *does every `ic.pipeline_full` record with `oracle=symmetrised` price
`complete: true` (nothing unpriced, nothing known-uncounted) under
`ecbench work --counted`?* Pass requires every counted record of the IC arm
complete and a verify receipt with zero problems.

This is an accounting change: it makes records honest about their full cost.
It is not an improvement of the index-calculus baseline and makes no security
or speed claim. Section B (the Koblitz exponent ladder,
`specs/koblitz-exponent-n{13,19,23}.json`) is frozen and *not run*.

= Session

- *Spec* `specs/koblitz-symmetrised-n17.json` → `sessions/n17-symmetrised`
  (`ECBS1h6fd757ff9d9f`). Curve $K_1 \/ FF_(2^17)$
  (`icv1-f2m17-tm101-00378d4e`), 4 public targets, 2 measured rounds plus 1
  warm-up, alternate order, `ecbench run --cpus 1` (L0), binary sha256
  `bf8196c2…`.
- *Arms.* `ic-symmetrised-d9` (`ic.pipeline_full`,
  `koblitz-symmetrised:divisor=0;1`, `symmetrised:m=2,divisor=0;1`,
  `targets=walk`) and the reference `rho-frob`
  (`rho.signed_frobenius_full`).
- *Runs.* 24 records (8 warm-up), 24 verified; 16 counted (8 per arm).
- *Audit.* `ecbench verify --replay 2`: `audit OK: 24 records, 24 verified,
  2 replays reproduced, 0 problems` (`audit/n17-verify-replay2.json`).

= Result (section A): pass

Every counted record of both arms has `complete: true`, `unpriced: []` and
`known_uncounted: []` (`audit/n17-work-counted.jsonl`).

*Correction (review of PR \#2638).* As first priced, the `koblitz-symmetrised`
base's Artin–Schreier lifts and orbit walks were charged twice: in the
builder's `tally_*` counters and again through the natives `as_solves` (511)
and `frobenius_maps` (409) that `koblitz_factor_base` derives from the base's
size. The natives stay in the record for replay and are now priced once (a
`tallied:<native>` marker; a `koblitz-symmetrised` base phase carrying a tally
is read as marked, which covers this session's records). The figures below
are re-derived from the committed session with the fixed binary (sha256
`4ff2214e…`): each IC record falls by exactly 2,470,915 gates, all in
`factor_base`; every rho record is unchanged. Medians are taken by
`audit/medians.jq` (`audit/n17-work-medians.json`).

#table(columns: 5, stroke: 0.4pt, inset: 4pt, align: (left, right, right, left, right),
  [*arm*], [*counted*], [*complete*], [*median bitop/v1 (min–max)*], [*median GAE*],
  [`ic-symmetrised-d9`], [8], [8 / 8], [1.956e9 (0.980–3.664e9)], [897.1],
  [`rho-frob` (reference)], [8], [8 / 8], [1.868e6 (1.269–6.582e6)], [254.6],
)

#figure(image("figures/bitops_per_record.svg", width: 100%),
  caption: [`bitop/v1` per counted record, both arms, by round and recovered
  target scalar; dashed lines are each arm's median of 8 (the README's recorded
  medians, asserted by `figures/plots.py`). Session `ECBS1h6fd757ff9d9f`; exact
  counts, no interval stated (4 targets at 17 bits support none).])

*Distance remaining.* As the protocol requires, no speed claim is drawn from
four 17-bit targets. Stated as the distance remaining, the median IC record
costs about 1047× the matched rho in `bitop/v1` (1.956e9 against 1.868e6;
first priced 1048×). One IC record (seq 8) splits by phase as `relations`
1.897e9, `factor_base` 1.19e7 (first priced 1.44e7), `oracle_setup` 6.86e6,
`linear_algebra` 4.9e5 and `verify` 7.3e4.

= Instructions per counted gate (`ecbench audit-ir`)

`ecbench audit-ir --spec specs/koblitz-symmetrised-n17.json --seq 8 9 10 11`
re-executed four plan entries under Callgrind (`--cache-sim=no
--branch-sim=no`) with the session's binary; each reproduced its record's
bitop count exactly (`audit/n17-audit-ir.jsonl`, with the first pricing's
gate counts). The Ir is unchanged by the repricing; `audit/n17-audit-ir-repriced.jsonl`
divides each recorded `solve_ir` by the repriced gate count (jq,
`audit/audit-ir-reprice.jq`), without re-running Callgrind.

#figure(image("figures/ir_per_gate.svg", width: 100%),
  caption: [Callgrind solve instructions per counted `bitop/v1` gate (the
  repriced `ir_per_kilo_bitop_repriced` / 1000), two records per arm.])

The symmetrised arm executes 0.323 and 0.305 instructions per counted gate
(first priced 0.323 and 0.304);
the rho reference 1.395 and 0.442. The rho solves are short (about 2–7e6
gates), and per-solve work that does not grow with the walk would produce that
spread, but this has not been checked against the Callgrind parts. Two records
per arm describe this session and support no general ratio.

= Limitations

- *Section B not run.* The exponent ladder (`n = 13, 19, 23` at a fixed base
  size) is frozen in `PROTOCOL.md` and its specs, with no sessions.
- *One small size:* $K_1 \/ FF_(2^17)$, 4 targets, 2 rounds. No interval, no fit,
  no comparison claim.
- *L0 isolation* on a shared host; only counts are used, no wall time.
- *audit-ir* covers 4 of the 16 counted records.
- *Validation gate.* No `VALIDATION.json` (replay certificate, validator
  report), so no IC leaderboard or `benchmarks/` row. The IC/rho multiple is
  a diagnostic point (class accounting) on the scoreboard's progress chart.

= Evidence

#set par(justify: false)
`PROTOCOL.md`, `specs/`, `sessions/n17-symmetrised/`,
`audit/n17-verify-replay2.json`, `audit/n17-work-counted.jsonl`,
`audit/n17-work-summary.jsonl`, `audit/n17-work-medians.json`
(`audit/medians.jq`), `audit/n17-audit-ir.jsonl`,
`audit/n17-audit-ir-repriced.jsonl` (`audit/audit-ir-reprice.jq`),
`figures/plots.py` (plots recorded values only).
