#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9pt)
#set par(justify: true)

= Run report: the 12 jobs run after the stop, 2026-10-19T16:12Z

*Descriptive.* What ran and what it recorded. Interpretation is in the study's
`README.md` and `REPORT.pdf`. Written at the end of the queue; the audit status
is in the study README.

- *Study:* `research/gaudry_quartic_d4_exponent_20261015`, `PROTOCOL.md`
  amendment 4. These are 12 of the 72 jobs of the original plan; they were not
  started before the 16 h stop of 2026-10-15 (they are not new jobs) and were run
  at the owner's request.
- *Specs:* `specs/SPEC-<job>.json`, unchanged since `08722a2b7`. *Binary:* the
  same `ecbench` (sha256 `8627cd29…0aa3`), not rebuilt. Queue
  `sessions/jobs4.txt`, run by `run_queue4.sh`, one worker per allowed CPU
  (0–3), no start deadline; log `sessions/queue.log`.
- *Host:* Claude Code cloud container, 4 vCPU, rustc 1.97.0. Every session ran
  with `--allow-busy`: isolation earned L0 (7 records) or L1 (5). `S` is a count
  (`priced_by = counted`) and does not depend on wall time, which is descriptive
  only.
- *Workloads and arms:* as in the first run report: `fp4_torsion(p, 1, 16)`,
  `ic-d4-w` (plain, `linalg=wiedemann`) and `ic-d4-lp2` (`large_primes=2`),
  target seed `2026101500000 + 10p + t`, measurement seed `202610150`.

== Runs requested and completed

12 jobs requested, *12 completed, all verified* (`[d]G = Q`). Each session also
holds its paired `rho-neg` record (same target, same seed): 12 more verified
records, 5 of them on workloads with no earlier rho run (`main-p29-t2`,
`main-p37-t2`, `main-p53-t2`, `main-p61-t2`, `main-p197-t2`). Together with the
first run (60 sessions) this completes the 72 planned jobs.

#figure(image("s_new_runs.svg", width: 100%),
  caption: [`S` of the 12 index-calculus runs of this queue (coloured) among the
  48 of the first 60 jobs (grey). Sessions under `sessions/`.])

== Index-calculus runs (measured; `S` in GAE/√r, trials counted)

#let rows = csv("runs.tsv", delimiter: "\t")
#table(
  columns: (auto, auto, auto, auto, auto, auto, auto, auto),
  stroke: 0.3pt,
  inset: 3pt,
  [*job*], [*session*], [*arm*], [*p*], [*status*], [*S*], [*trials*], [*wall h*],
  ..rows.map(r => (r.at(0), r.at(1), r.at(2), r.at(3), r.at(5),
    str(calc.round(float(r.at(6)), digits: 0)), r.at(7), r.at(8))).flatten()
)

== Raw records

`sessions/<job>/records.jsonl` (plan, host and session files beside them), logs
`sessions/<job>.log`; audit receipts `audits/<job>.audit.json`, list
`audits/AUDITS.txt`.
