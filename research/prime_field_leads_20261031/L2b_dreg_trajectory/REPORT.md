# L2b: d_reg trajectory of the constrained prime-field Semaev system (incomplete)

**Lead.** L2b of `research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md`;
criteria frozen in `PREREG.md` before the ladder (reopens: real-arm ratio of the
median maximal step degree < 1.5 while the control ratio ≥ 2; null_confirmed:
the two ratios agree within 25 %). **Status.** incomplete: the frozen ladder
(`run_ladder.sh`: k = 10, 12, 14, 16 with 2 curves × 4 targets × 2 arms, k = 18
and 20 reduced) produced one cell before the host stopped the workflow.
**Decision.** none; no trajectory ratio can be formed from one cell.

This report was written by the synthesis step (2026-11-01) from
`results/cells.csv`, `results/instances.jsonl`, `results/probe_cells.csv`,
`results/probe_instances.jsonl`, `PREREG.md`, `run_cell.sh`, `run_ladder.sh`
and the generator `gen/src/main.rs`. No number is new, and nothing was rerun.

## 1. Instrument

Generator `gen` (Rust) writes, for (p, curve seed, target seed, arm), the
system S₄(X₁, X₂, X₃, x(R)) = 0 plus three membership polynomials of degree
N = round(p^{1/3}) (real arm: the interval base; control arm: a random
polynomial f of the same number of terms and total degree with random
membership polynomials of the same degree). msolve (`-g 2 -v 2 -t 1`) under a
watchdog (time cap and 16 GiB RSS cap). Metrics parsed from the F4 round
table: maximal step degree, last degree, rounds, largest matrix, last matrix,
total rows, basis size, and `zdeg` as a stated proxy only (not a first fall
degree). Wall time is used only as the kill cap.

## 2. Every cell attempted

Ladder (`results/cells.csv`, seeds from the frozen ladder):

| p | log₂ p | N | arm | status | max deg | last deg | rounds | largest matrix | total rows |
|---|---|---|---|---|---|---|---|---|---|
| 65521 | 16.0 | 40 | ctrl | ok | 48 | 47 | 126 | 14235 × 14229 | 314140 |

Probe cells (`results/probe_cells.csv`, real arm, curve and target seed 9,
outside the frozen ladder's seeds; run to check the generator and caps before
the ladder, cap 600 s):

| p | log₂ p | N | arm | status | max deg | last deg | rounds | largest matrix | total rows |
|---|---|---|---|---|---|---|---|---|---|
| 4093 | 12.0 | 16 | real | ok | 24 | 23 | 273 | 2059 × 2033 | 39533 |
| 16381 | 14.0 | 25 | real | ok | 33 | 32 | 585 | 6093 × 6099 | 175479 |
| 65521 | 16.0 | 40 | real | ok | 48 | 47 | 1629 | 17516 × 17418 | 1444706 |
| 262139 | 18.0 | 64 | real | timeout 600 s | 72 (partial) | 48 | 2119 | 7279 × 31831 | 7462060 |
| 1048573 | 20.0 | 102 | real | timeout 600 s | 110 (partial) | 106 | 317 | 18822 × 81222 | 3895033 |

Observed, not decided: in every completed cell, real or control, the maximal
step degree is N + 8 (24, 33, 48, 48), which is the membership degree plus a
constant, as `PREREG.md` said to expect by construction ("both arms contain
three univariate membership polynomials of degree N, so max deg ≥ N in both
arms regardless of S₄"). The two timed-out cells reached degree N + 8 as well
before the cap. At p = 65521 the one control cell and the one (different-seed)
real probe cell agree in maximal degree (48) and in last degree (47) and differ
in rounds (126 against 1629) and total rows (3.1·10⁵ against 1.4·10⁶); with one
cell per arm and different seeds this is not a comparison.

Graph: `dreg_vs_log2p.svg` (maximal step degree and largest matrix width
against log₂ p for the six recorded cells, hollow markers for the two killed
cells; drawn by `plot.py` from the CSV files, with `results/summary.csv` from
`summarize.sh`).

## 3. Decision against the frozen rule

Neither clause can be evaluated: the trajectory ratio needs both arms at the
largest p where both completed every run and at p = 1021, and the ladder has
one arm at one p. Status incomplete; no decision; the lead card's question
stays open with the instrument built and the criteria frozen.

## 4. What a rerun needs

The ladder as written (`PAR=8`, cap 1500 s per cell) on a quiet host; the
k = 18 and k = 20 cells will exceed 600 s on the real arm (both probes timed
out) and are the ones the reopen clause depends on. Memory cap 16 GiB was not
reached in any cell.

## 5. Limits

- Design (the metric is pinned to N + O(1) by the membership polynomials) was
  acknowledged in `PREREG.md`; a flat real-arm trajectory in maximal step
  degree is not expected from the degree profile alone, so the matrix profile
  is the quantity that would carry a decision.
- No PDF renderer was available to the lead agent; `REPORT.pdf` was rendered
  by the synthesis step with Typst in a scratch virtualenv.

## Rerun 2026-11-01: the frozen ladder, complete (appended; the sections above are unchanged)

Plan `../rerun_20261101/RERUN_PLAN.md`, report `../rerun_20261101/REPORT.md`, data
`../rerun_20261101/L2b/`. msolve 0.6.5 with the exact sparse/dense kernel (`-l 1`),
which reproduces the three completed probe cells above and the one ladder cell in every
column. The ladder of `run_ladder.sh` ran as listed: 70 cells, 68 completed, the two
p = 1048573 cells stopped at the 1500 s cap.

Largest p where both arms completed every run: 262139. Median maximal step degree
18 at p = 1021 and 72 at p = 262139 in both arms: trajectory ratio 4.00 (real) and
4.00 (control). **Decision: null_confirmed.** Every completed cell has maximal degree
N + 8. Observed, not decided: the real arm does more F4 work than the control at every
rung (median total rows 1.26 to 21.6 times the control's from p = 1021 to 262139).
