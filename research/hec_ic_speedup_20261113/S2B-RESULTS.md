# Slice S2b results: a dense step-column block in the native solve

**Protocol:** PLAN.md §8, frozen at `9e66fa70ce` before any S2b code.

**Code:** `Solver::Dense` in `crates/hec-native-ic/src/solve.rs`, commit `7ef8c38315`.

**Record:** `benchmarks/records/20261114T2233Z-hec-s2b-dense-solve.json`, made with `run_s2b.sh` on
2026-11-14.

**Raw output:** `s2b-20261114/`:

- `rep1.txt`–`rep5.txt`;
- `binaries.sha256`;
- `host.txt`, with the load average before the run and after each repeat;
- `summary.jsonl`, from `summarise_s2b.sh`;
- `pairs.jsonl`, from `pairs_s2b.sh`.

**Run report:** `reports/run-20261114T2233Z/`.

## Change

Each row is split in two:

- a **sparse part** over the factor-base columns that are not step places;
- a **dense array** over the ≈ 64 step columns and `k`.

Elimination then runs in two phases:

- **Phase 1** is Markowitz elimination over the sparse columns only. Each row operation merges the
  sparse parts, then runs a contiguous loop of the form `row −= c·pivot` over the dense array. Each
  product costs one REDC, against a factor prepared once per row operation (`Fp::pre`,
  `Fp::mul_pre`).
- **Phase 2** eliminates the residual dense block column by column. A remaining row with a non-zero
  `k` gives `k`.

`Solver::Fast` (S2a) stays in the crate as the comparison arm. `HEC_SOLVER=fast,dense` runs both
solvers in one process, alternating which goes first by seed parity.

## Verdicts

**E5 (exactness): holds.**

- G5: `solve::tests::dense_matches_fast` runs 90 random systems with a dense block. It asserts that
  `Dense` returns the same `Option<k>` as `Fast` and requires coverage of at least 20 systems that
  determine `k` and 10 that do not.
- G6, small rows: `ic::tests::dense_solver_agrees`.
- G6, full: in each of the 5 repeats, all 55 (row, seed) runs agreed on `k`, `trials`, `relations` and
  `solve_rounds`, which is 275/275. The bench asserts this. All 550 solves verify `k·D₁ = D₂`.
- All 11 crate tests pass in release and in debug. [pcc:unverified]

**E6 (genus 2, p ≥ 503: median paired `Fast/Dense` solve ratio ≥ 2×): holds** on every row:

| p | median ratio |
|--:|--:|
| 503 | 4.21× |
| 1009 | 4.04× |
| 2003 | 5.49× |
| 4001 | 5.75× |

[pcc:unverified]

**E7 (genus 2, p ≥ 503: median `Dense` solve share < 35%): holds.** The share is 17.7–26.3% across
those four rows; it was 45–68% with `Fast` in the same runs.

**Class:** engineering on the solve stage. Relation search and group operations are unchanged, as
G6 shows. Solve multiplications go *up*, not down (next section). The native engine computes no `S`.
No `S` claim, no advance.

## Table

Medians are over 25 (repeat, seed) pairs per row.

| g | p | m | Fast solve ms | Dense solve ms | median ratio | pair min–max | Fast share | Dense share | solve mulmods F / D | row ops F / D |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 2 | 101 | 46 | 0.0897 | 0.0227 | 3.90× | 0.50–4.77 | 40% | 15% | 3238 / 7524 | 327 / 712 |
| 2 | 251 | 123 | 0.3679 | 0.0911 | 4.03× | 1.90–97.4 | 56% | 24% | 21008 / 36143 | 1233 / 2100 |
| 2 | 503 | 259 | 0.9436 | 0.2127 | 4.21× | 0.89–68.8 | 59% | 26% | 76422 / 116320 | 3370 / 3884 |
| 2 | 1009 | 478 | 1.7638 | 0.4245 | 4.04× | 0.12–48.0 | 45% | 18% | 216935 / 254084 | 6654 / 6877 |
| 2 | 2003 | 1000 | 6.7664 | 0.8842 | 5.49× | 0.51–33.5 | 68% | 25% | 448339 / 483199 | 12587 / 12737 |
| 2 | 4001 | 2005 | 13.7137 | 1.7612 | 5.75× | 0.50–49.9 | 61% | 20% | 892957 / 915581 | 24174 / 24251 |
| 3 | 61 | 33 | 0.0820 | 0.0203 | 4.10× | 1.33–9.40 | 18% | 5% | 4704 / 7445 | 413 / 631 |
| 3 | 211 | 104 | 0.5078 | 0.1407 | 3.82× | 0.34–44.9 | 27% | 9% | 57978 / 90518 | 1966 / 2535 |
| 3 | 401 | 196 | 0.8738 | 0.2307 | 3.64× | 0.05–152 | 23% | 8% | 128227 / 136469 | 3345 / 3213 |
| 4 | 31 | 16 | 0.0335 | 0.0104 | 3.23× | 0.44–244 | 3% | 1% | 1953 / 2444 | 209 / 231 |
| 4 | 61 | 36 | 0.1264 | 0.0328 | 4.55× | 0.32–44.9 | 8% | 2% | 12819 / 14107 | 708 / 813 |

## What improved, and what did not

**The wall clock improved; the counted work did not.** [pcc:unverified]

- `Dense` does more multiplications mod ℓ than `Fast`:
  - +52% at p = 503;
  - +2.5% at p = 4001;
  - up to +132% at p = 101.
- It does about as many row operations or more.
- The gain is in layout. The ≈ 65 hot columns stop being sparse entries that are merged, counted and
  tracked in holder lists; they become a contiguous array updated with one REDC per product.
- Under accounting that charges linear-algebra multiplications, as the BigUint engine's `S` does,
  S2b would be a small *increase*.

**Scaling.** At genus 2 the median `Dense` solve grows about linearly in the factor-base size `m`:

| m | median Dense solve |
|--:|--:|
| 259 | 0.21 ms |
| 478 | 0.42 ms |
| 1000 | 0.88 ms |
| 2005 | 1.76 ms |

The `Fast` solve grows faster: from 0.94 ms at m = 259 to 13.7 ms at m = 2005. The median ratio
rises from 4.2× to 5.75× over that range. This holds on round eight's sizes only: m ≤ 2005 at
genus 2. Nothing here says how either solver behaves beyond them. [pcc:unverified]

**Where the time goes now.**

- Genus 2: the solve is a fifth to a quarter of native time.
- Genus 3 and 4: it is 1–9%. The oracle and the walk dominate there, so plan track T2 (per-trial
  cost) is the next lever.

## Noise and protocol deviations

1. **The host was heavily shared.**
   - The load average was 81–95 before and during the run, 38 at the earlier smoke check (`host.txt`).
   - That breaks the L0 condition ("nothing else scheduled") that S1 and S2 had.
   - Single pairs swing from 0.05× to 244×, and each gated row (genus 2, p ≥ 503) has 4–5 of its 25 pairs
     below 2×. Across all genus-2 rows, 20–24 of 25 pairs are at or above 2×.
   - The pre-registered statistic is the median, which sits at 4.0–5.75×, at least twice the bar.
     But the spread is preemption, and the medians are not a quiet-host measurement.
   - A rerun on a quiet host would be a labelled robustness check. It has not been done.
2. **A smoke run came first.** One `HEC_SOLVER=fast,dense` process, run after the protocol froze and
   before the recorded run, checked G6 (55/55) and gave genus-2 p ≥ 503 ratios of 2.7–7.7×. It is
   not part of the verdict and its output is not committed.
3. The protocol said "row ops and solve_mulmods for both". The table reports their medians; they
   barely vary across seeds. [pcc:unverified]

## Not done

- **External comparison (§15):** not run. Planned under T3, against FLINT and CADO-NFS `bwc` on
  exported matrices.
- **Validation:** no `VALIDATION.json` exists, so the `benchmarks/` note sits outside the admitted
  table.
