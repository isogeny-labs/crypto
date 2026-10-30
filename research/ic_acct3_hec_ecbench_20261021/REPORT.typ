#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set par(justify: true)
#show heading.where(level: 1): set text(size: 12pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[The hyperelliptic head-to-head as ecbench records]
  #v(2pt)
  `research/ic_acct3_hec_ecbench_20261021` · session 2026-10-21 · report 2026-10-26 · class: accounting port
]

= Question

`research/hec_bitop_20261020` priced hyperelliptic index calculus and its
Pollard rho reference in `bitop/v1` through a standalone example, with no
sealed records and no replay. This item ports both arms into ecbench as the
methods `hec.ic_fb_walk` and `hec.rho_full_width` (commit `8c37d9a6e`) and asks
one question, frozen in `PROTOCOL.md` before the session ran:

*Run through ecbench on the study's genus-2, $p = 41$ row ($N = 1321$,
$k = 447$), with the study's instance, target and seeds, do the records'
`bitop/v1` totals equal the study's `bitops_per_run`, run for run, for both
arms?*

The decision is exact integer equality, computed by the Rust example
`examples/hec_ecbench_equivalence.rs`. This is an accounting port. It changes
no IC cost and is not an improvement of the index-calculus baseline.

= Session

- *Spec* `specs/g2-p41.json` (`ECS1hf41f08dfbb2f`) → session `sessions/g2-p41`
  (`ECBS1he77b325d1865`). Built at `1961d1878` with a dirty tree (the port
  under review; `git_dirty: true` in `session.json`), binary sha256 `055fa754…`.
- *Arms.* 5 IC arms (fixed seeds 20260916–20260920; `branches 64`,
  `d_step_in 4`, `extra_relations 8`) and 40 rho arms (fixed seeds
  `0xC0FFEE` + 0…39). One round, no warm-up, `--cpus none`,
  `RAYON_NUM_THREADS=1`. Isolation L0: no wall-clock figure is used.
- *Runs.* 45 requested, 45 records, 45 verified ($[k]D_1 = Q$ checked by the
  runner; neither method sees the planted scalar).
- *Audit.* `ecbench verify --replay 2`: ok, 0 problems; replays of seq 0 (IC)
  and seq 22 (rho) both `identical` (`sessions/g2-p41.audit.json`).
- *Pricing.* `ecbench work --counted`: all 45 records `complete`, nothing
  unpriced, nothing known-uncounted (`analysis/work.jsonl`).

= Result: equivalent

All 45 per-run totals and all 12 per-phase sums (6 phases × 2 arms) are equal
to the committed study counts (`analysis/equivalence.md`).

#table(columns: 6, stroke: 0.4pt, inset: 4pt, align: (left, right, right, right, right, right),
  [*arm*], [*runs*], [*equal*], [*ecbench Σ bitop/v1*], [*study Σ bitop/v1*], [*mean per run*],
  [`hec.ic_fb_walk`], [5], [5], [117,422,628], [117,422,628], [23,484,525.6],
  [`hec.rho_full_width`], [40], [40], [4,535,211,568], [4,535,211,568], [113,380,289.2],
)

#figure(image("figures/phases_ecbench_vs_study.svg", width: 100%),
  caption: [Per-phase `bitop/v1` summed over each arm's runs, session
  `ECBS1he77b325d1865` (filled) against `research/hec_bitop_20261020`
  (ring), from `analysis/equivalence.md`. Log scale. The `unattributed` phase
  is 0 in both, for both arms, and is not drawn. Counts are exact; there is no
  sampling interval to show.])

#figure(image("figures/runs_ecbench_vs_study.svg", width: 72%),
  caption: [Per-run `bitop/v1` totals, ecbench (y) against the study (x), all 45
  runs (5 IC, 40 rho). Every point lies on $y = x$ by exact integer equality.])

*Position against rho.* The IC/rho ratio carried over is the study's 0.207 at
this row (mean `bitop/v1` per run, 23.5 M against 113.4 M), so this row
already sits below its matched full-width rho, with the study's disclosed
coverage limits (`research/hec_bitop_20261020/PROTOCOL.md`). It is the study's
figure reproduced through ecbench, not a new measurement or an improvement. The rho arm's sum is
dominated by its `precompute` phase (3.97e9 of 4.54e9 bitops); the IC arm's by
`walk` (6.97e7 of 1.17e8). These are the study's recorded splits.

= Limitations

- *One row.* Genus 3 and 4 and the other primes are not run here; one size
  gives no slope, so no bound is fitted.
- *GAE figure.* The records' GAE count holds only the Jacobian operations the
  algorithms count, and they name
  `jacobian_field_ring_and_word_work_outside_group_operations` as unpriced, so
  `S` is a lower bound. The `bitop/v1` work block is the complete figure.
- *Floor.* `work`'s `floor_bitops` uses an elliptic affine addition's price; it
  is not a calibrated floor for a Jacobian.
- *Conventions* are the study's: one word operation per RNG draw, memory
  movement uncounted, validation inside the total.
- *Validation gate.* There is no `VALIDATION.json` (no replay certificate or
  validator report), so nothing here goes to the IC leaderboard or
  `benchmarks/README.md`; it is an accounting port, not a performance result.
- *No instruction audit* (`ecbench audit-ir`) was run for these records.

= Evidence

#set par(justify: false)
`PROTOCOL.md`, `specs/g2-p41.json`, `sessions/g2-p41/` (records, plan, host,
session), `sessions/g2-p41.audit.json`, `analysis/work.jsonl`,
`analysis/equivalence.md` (`examples/hec_ecbench_equivalence.rs`),
`figures/plots.py` (parses `equivalence.md`; plots recorded values only).
