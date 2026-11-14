# Slice S2a results: the native solve (protocol: PLAN.md §7)

Run 2026-11-13 on the S1 host (Apple silicon, macOS).

- Binary: `crates/hec-native-ic`, `examples/hec_native_bench`, built with stable rustc 1.99.1 in
  release with `debug = 1`. Its SHA-256 is in `s2/binaries.sha256`.
- Both solvers are in the same binary. The run used `HEC_SOLVER=both` and was repeated twice
  (`s2/s2_both_run1.txt`, `s2/s2_both_run2.txt`).
- The table is `s2/s2_table.md`. Each cell is a mean over the five seeds 20260916..20260920.

## Change

`solve::solve_fast` changes three things relative to the S1 solver (`solve::solve_reference`, which
is kept unchanged apart from adding the pivot hash):

- It pushes a row id onto a column's holder list only when the column is new to the row. Before,
  the id went onto every column of every updated row.
- It keeps no holder list for the `k` column.
- It reuses one merge buffer instead of allocating one per row operation.

The Markowitz heap is re-keyed only for the columns whose counts changed. Pivot selection is the
same code: holders are deduped, filtered, and the shortest row is taken, ties by order.

## Verdicts

**E3 (exactness): holds.** In both runs all 55 (row, seed) pairs agreed exactly between the two
solvers, on:

- `k`;
- `trials`;
- the pivot-sequence hash;
- `pivots`;
- `row_ops`;
- `solve_mulmods`.

Each run prints a `GATE … identical` line for every pair (55 per run); the bench asserts it. All
110 solves per run verify `k·D1 = D2`. `solve_mulmods` also equals the S1 run (`s1/native_s1.txt`)
on every row.

The unit tests `solve::tests::recovers_planted_k` and `ic::tests::fast_solver_is_exact` assert the
same gate on small rows. All 9 crate tests pass in release and in debug.

**E4 (genus 2, p ≥ 503: mean solve time falls ≥ 2×): holds.**

| p | run 1 | run 2 |
|---|---|---|
| 503 | 3.13× | 3.30× |
| 1009 | 3.75× | 4.28× |
| 2003 | 3.36× | 3.97× |
| 4001 | 3.56× | 2.89× |

The lowest ratio is 2.89×. On every other row of both runs the solve falls 2.3–4.0×.

**Class:** engineering, on the native engine's solve stage. The counted work is identical by
construction and by the gate. Wall clock falls. This is not an `S` claim and not an advance.

## What it buys end to end

Against the S1 native binary's own solve times (`s1/native_s1.txt`, a different process on another
day), the fast solve is 2.5–3.4× faster at genus 2, p ≥ 503:

| p | S1 solve (ms) | S2 fast solve (ms) |
|---|---|---|
| 503 | 2.56 | 0.83–1.04 |
| 4001 | 20.5 | 7.2–7.6 |

These are not paired measurements, so the figures are context only.

The total-time ratios in the table (1.0–2.8×) are noisier than the solve ratios. The walk and
oracle do identical work under both solvers, yet their times vary between process invocations, for
example at genus 3, p = 61, run 1. Use the solve columns for the verdict.

## The trigger for S2b fires

PLAN §7 says S2b gets its own protocol if the solve still exceeds half of native wall time at
genus 2. It does on the larger rows. The fast solve's share of total time is:

- 59–64% at p = 503;
- 55–57% at p = 2003;
- 53–54% at p = 4001;
- 44% at p = 1009.

At genus 3 the share falls to 18–35%; at genus 4, to 3–10%.

**The trigger rests cleanly only on p = 503 (59–64% in both runs).** At p = 2003 and 4001 the
share is 53–57%, within run-to-run noise of 50%. The same binary's p = 2003 reference solve
took 22.8 ms in run 1 and 12.9 ms in run 2; the S1 binary took 11.0 ms.

Per-row medians next to the means are in `s2/s2_table_medians.md`. The median solve ratios at
genus 2, p ≥ 503 are 3.2–4.3×, so E4 does not depend on mean versus median.

S2b's gains will be smaller relative to this noise. Its protocol should pre-register median-based
metrics and more repeats than five seeds × two runs.

## Protocol deviations

1. **Alternation was by seed within each row, not by row.** Seeds 0, 2 and 4 run the reference
   first; seeds 1 and 3 run the fast solver first. This is a finer pairing than the protocol asked
   for. Both solvers see the same rows in the same process.
2. **The run was repeated twice.** The protocol named a single measurement. Both runs are reported,
   and both pass.

## Process steps (AGENTS.md), done in the PR

These cover both S1 and S2.

- **Recorded rerun.** `run_record.sh` and `summarise.sh` were run under
  `benchmarks/records/20261114T0356Z-hec-native-ic.json`. Raw output is in `rec-20261113/`; the
  small text files are in Git, and CI uploads them to the store.
  - E3 held a third time: 55/55.
  - E4 held at 3.19–3.44×.
- **Run report.** `reports/run-20261114T0356Z/` holds `report.typ`, `report.pdf`, and three
  graphs as SVG and PNG from `plots.py`. Copies are in `benchmarks/20261113-hec-native-ic/`.
- **`benchmarks/README.md` note.** It sits outside the admitted table, because the run is
  unvalidated.
- **ICV1.** Not applicable. ICV1 names elliptic curves only, so these Jacobians are named by
  equation and ℓ.
- **External comparison.** Not run. No open hyperelliptic-IC implementation is known. See the
  report.
- **S3 resume checkpoint.** Pushed under
  `checkpoints/crypto/research__hec-native-ic-20261113/`.
