# Protocol: scale-up S4, lead 3 (m = 3) and lead E (t0-bits) at k = 24 and 28 (written before data)

Date 2026-11-05, branch `scale-S4-m3-t0`. Frozen before any cell below was run. Class: component
measurement of the `pkm-semaev` oracle (F4 over the tower/bit tower). It makes no ranking claim,
gives no distance to rho, and claims no leaderboard row. Every statement is scoped to the cells
listed here.

## Instrument (frozen, no code change, no build)

`/home/user/frozen-bin/curve25519_family_scale`, sha256
`d66b85b0dcb60c2d1b551a2e3ce19de4cc76e6897ce359eafce1bf579606ca90` (checked before data and again
before the report; `analysis/binary.sha256`). CLI:
`f4cost <k> <form> <t> <G> <targets> <seed> <cap_s> <m> <mode> <lifts> <max_nnz|-> <trace>`.
k = 32 is refused (tower engine needs p < 2^31); scope is k in {20 (reference), 24, 28}. One process
at a time, `ulimit -v 3500000` (3.5 GB address space), `timeout` above the cap. If a code change
seems needed the experiment stops and reports.

Unit: F_p multiply-adds in F4 elimination only. System construction, root-grid reading, lift and
group re-adds are native work that AGENTS.md 5a requires to be priced (0, 0.1, 1 GAE per operation)
before any cross-method ranking; this protocol compares F4 multiply-adds between arms of the same
instrument only, and the native counters are reported beside them.

## Census taken before the protocol (not data for any prediction)

`f4cost ... <targets = 0>` returns the base geometry and `coverage_exact` without solving. These
were read for the k = 20, 24, 28 cells to set the sample sizes (`analysis/coverage_census.jsonl`).
Seen before freezing: isogeny k = 24 t = 6, m = 3: 39 columns, coverage_exact 0.00474; k = 20
isogeny t = 6, m = 3: 0.0195; t = 8: 0.960 (m = 2: 0.0351). The census run for bits t = 6, m = 3
at k = 20 needed more than 2.8 GB with 0 targets (the system build, not F4); this is why the lead 3
"memory" failures at bits t = 6 may not have been F4 failures. Recorded as an observation, not used
in a prediction.

## Part 1: lead 3 (m = 3) at larger k, deep guessing

Cells: form in {isogeny, bits}, t in {6, 8}, G in {t-2, t-1} (t = 6: G = 4, 5; t = 8: G = 6, 7),
mode in {full, chain}: 16 m = 3 cells per k. Comparison cells: m = 2, mode full, same
(k, form, t, G): 8 per k. 50 random subgroup targets per cell, `cap_s` 900, `timeout` 1000,
`lifts` 0, no `max_nnz`, trace 0. Seed = 2026110600 + t + 100 * ((k - 20) / 4); the m = 2 and
m = 3 cells of one (k, t) use the same seed, hence the same target stream.

- k = 24: all 24 cells, in the order: m = 2 cell, then m = 3 chain, then m = 3 full, per
  (form, t, G), t = 6 first.
- k = 28: run only for (form, t, G, mode) whose k = 24 counterpart returned data (not `failed`);
  a cell whose k = 24 counterpart failed is recorded as `skipped` with that reason (the tower
  sizes are equal, so the same failure is the prior; this is a budget rule, not a result).
  The m = 2 comparison cell is run whenever any m = 3 cell of its (form, t, G) is run.
- k = 20 reproduction cell: exactly lead 3's cell `bits t = 4 G = 3 chain`,
  `f4cost 20 bits 4 3 100 2026111304 600 3 chain` (lead 3: 543067299 multiply-adds per target,
  solving degree 6, 0/100 hits, coverage_exact 0.00046).
- k = 20 deep-G cells (the k-scaling anchor): t = 6 only, G in {4, 5}, both forms, both modes,
  and the m = 2 partners (12 cells), seed as above with k = 20 (2026110606).

Failures are data: the driver keeps stderr and the return code per cell; code 124 is a timeout,
a Rust abort or `memory allocation` message or signal 6/9/11 is a memory failure. A capped cell is
analysed on the targets done. A cell that finishes fewer than 5 targets is "intractable", not
extrapolated.

Metrics per cell: columns, base points, F4 multiply-adds per target, max solving degree, F4 calls
per target, hits (and verified), undecided, `coverage_exact` (the binary's figure for that m),
per-relation = multiply-adds per target / coverage_exact
(`muladds_per_relation_at_exact_coverage`), observed per-relation (total multiply-adds / hits)
only where hits >= 1, uncharged native work as a share, `uptime` at start. Comparison: ratio
(m = 3 per-relation) / (m = 2 per-relation) at equal (k, form, t, G), and mode full against chain.
Cells in which `undecided` equals the targets done (F4 skipped, system over the size limit, the
grid reading the answer) carry no F4 cost and no per-relation figure.

### Frozen predictions, Part 1 (with falsifiers)

- **A1 (tractability of deep G).** At k = 24 every m = 3 cell with t = 6 completes at least 25 of
  its 50 targets inside the cap, in both forms and both modes. Falsified by any t = 6 cell that is
  `failed` or finishes fewer than 25 targets.
- **A2 (t = 8 is out of reach).** At k = 24 no m = 3 cell with t = 8 completes 25 targets. Falsified
  by any t = 8, m = 3 cell with at least 25 targets done.
- **A3 (regression persists).** Wherever both the m = 3 cell and its m = 2 partner have F4 cost,
  the m = 3 per-relation cost at coverage_exact exceeds the m = 2 one by a factor of at least 10.
  Falsified by any such pair with ratio below 10; an "improvement" reading additionally needs a
  ratio below 1 (the success criterion counts a verified measured improvement; a ratio below 1
  would be reported with its distance to the matched rho left open, none is computed here).
- **A4 (cost is flat in k).** For a (form, t, G, mode) cell run at k = 20, 24 and 28 the F4
  multiply-adds per target agree within a factor 2 (the m = 2 pattern of lead B: 1.15e8, 1.42e8,
  1.43e8). Falsified by a ratio outside [0.5, 2] between any two k at the same cell.
- **A5 (degree falls with G).** For each (k, form, t, mode) with both G values run, the max solving
  degree at G = t-1 is not above that at G = t-2, and is strictly below it in at least one series.
  Falsified by any increase, or by no strict fall in any series.
- **A6 (full needs more than chain).** Where both modes ran with F4 cost, mode full costs at least
  as many multiply-adds per target as chain. Falsified by any pair with full below chain.
- **A7 (reproduction).** The k = 20 cell reproduces lead 3 exactly (543067299 multiply-adds per
  target, degree 6, hits 0). Falsified by any difference above 0.5 percent.

## Part 2: lead E (t0-bits against bits) at larger k

Pairing rule R1 of lead E: plain arm `bits` with lo = 0; the `t0-bits@lo` arm with the same t and
the lo in [0, 400) that minimises |columns(t0) - columns(plain)|, ties to the smallest lo, from
`curve25519_family t0size <k> <t> 0 399` (`analysis/pairing.tsv`, taken before data):

| k | t | plain columns / points | lo | t0 columns / points |
|--:|--:|:--|--:|:--|
| 20 | 4 | 7 / 14 | 0 | 6 / 12 |
| 20 | 5 | 11 / 22 | 1 | 12 / 24 |
| 20 | 6 | 25 / 50 | 34 | 26 / 52 |
| 24 | 4 | 7 / 14 | 16 | 6 / 12 |
| 24 | 5 | 14 / 28 | 12 | 14 / 28 |
| 24 | 6 | 33 / 66 | 1 | 32 / 64 |
| 28 | 4 | 7 / 14 | 17 | 8 / 16 |
| 28 | 5 | 16 / 32 | 27 | 16 / 32 |
| 28 | 6 | 33 / 66 | 0 | 34 / 68 |

(The k = 20 rows equal lead E's choice; there is no X arm here.) Cells: k in {24, 28}, t in
{4, 5, 6}, G in {0, 3}, arm in {bits, t0-bits}: 24 cells, m = 2, mode full, 100 random subgroup
targets plus `F4COST_PLANTED=20` planted targets, `cap_s` 600, `timeout` 700, `lifts` 0,
`max_nnz` 30000000, trace 0 (the command of lead E). Seed = 2026110540 + t for k = 24, 2026110580 + t
for k = 28, the same for both arms of a pair (same target stream), interleaved arm order.
k = 20 reproduction: lead E's cells `plain_t5_g3` and `t0_t5_g3` with seed 2026110205 (lead E:
3036910 and 3036908 multiply-adds per target, degree 5, planted 17/17 and 18/18, coverage_exact
0.00032 and 0.00026).

Metrics as lead E: degree, polynomial terms per specialisation, F4 multiply-adds per target, matrix
size, hits, planted found / processed / exact, coverage_exact, per-relation, ratios t0 / bits per
(k, t, G).

### Frozen predictions, Part 2 (with falsifiers)

- **B1 (degree equal).** The max solving degree is equal in all 12 pairs (k, t, G). Falsified by
  any difference. The degree-lowering expectation of lead E ("strictly lower at G = 0") is falsified
  if no pair has t0-bits below bits, and survives for a pair only if t0-bits is below.
- **B2 (terms equal).** Polynomial terms per specialisation are equal in all 12 pairs.
- **B3 (cost equal).** F4 multiply-adds per target, t0-bits / bits, lie in [0.5, 2] in all 12 pairs.
  Falsified by any ratio outside.
- **B4 (coverage penalty persists).** The per-relation ratio t0-bits / bits (each arm at its own
  coverage_exact) is above 1 in all 12 pairs. Falsified by any ratio at or below 1. In the two
  equal-column pairs (k = 24 t = 5; k = 28 t = 5) the coverage ratio lies in [0.35, 0.75].
- **B5 (completeness).** In every cell all processed planted targets are found and every returned
  relation re-sums to its target (`planted_hits = planted_done`, `planted_exact = planted_hits`,
  `hits_verified = hits`). Falsified by any miss or inexact relation. Random hits are expected near
  zero at these coverages, so the planted targets carry the check.
- **B6 (reproduction).** The k = 20 cells reproduce lead E: multiply-adds per target within 0.5
  percent, equal degree, planted all found. Falsified otherwise.

## Scope of conclusions

k in {20, 24, 28} at the cells above, one curve per k (`build_miniature(k)`), one seed per
(k, t), 50 targets per Part 1 cell and 100 per Part 2 cell. At coverage near 1e-3 or less random
hits do not discriminate, so per-relation figures use `coverage_exact` as the hit rate (modelled,
labelled, as in leads 3 and E) and observed per-relation is given only where hits >= 1. Nothing about
other forms, other t or G, m >= 4, the (w, D) presentation, the 255-bit setting or a complete DLP.
No rho reference is run here: gaps to the matched rho are not computed and no row is proposed
for the leaderboard or `benchmarks/README.md`.

## Shared-machine and bookkeeping rules

Three other agents share the box. One process at a time; the driver is resumable (a cell whose
JSON exists is skipped) and keeps `uptime`, return code and stderr per cell. Files over 64 KiB go
to the artifact store.
