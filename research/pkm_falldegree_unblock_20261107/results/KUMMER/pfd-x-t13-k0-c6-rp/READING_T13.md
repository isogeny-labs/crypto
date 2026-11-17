# Kummer `m = 2`, `P1`, `t = 13`: the bracket completed (attempt 4 of `pfd-x-t13-k0-c6`; Amendment 19)

Measured and derived. `[pcc:unverified]`. Applied by hand from the Amendment 7 B / D.5 rule text, as in `RESULTS_T13_BRACKET.md`.

- **New row** (`fetch-20261115T0720Z/runs/X-m2-t13-k0-c6.jsonl`, manifest-checked; pod `q7scp0zg6ic3oj`, frozen binaries):
  - `x_r` 1118191946. That is the draw-0 system of the admitted cap-5 row Y1 (`RESULTS_T13_BRACKET.md` line 51).
  - Cap 6: `degree_reached` 6, `inconsistent` true, `basis_len` 1, `max_cols` 177,063, `muladds` 288,586,444,503,310,
    `wall_s` 7,907.7, `timed_out` false, `oversize` false.
  - `pfd_grid_zeros`: `grid_zeros` 0, `decomposable` false.
- **Validity of this pod's rows.** The positive control on the same pod passed: X1 and X3 were reproduced exactly
  (`results/KUMMER/CONTROL_E37_20261115.md`).
- **Draw 0.** Cap 5 does not refute (Y1), so `d_F ≥ 6`. Cap 6 refutes (this row), so `d_F ≤ 6`. **`d_F = 6`.**
- **Draw 1.** `d_F = 6` (Y2 and Y4, admitted).
- **Median 6, `c*(13) = 6`.** Amendment 7 D.5 reads **"holds at t = 13"**. The earlier reading was "undecided", because
  draw 0 had no cap-6 row after three Spot losses.
- **What it shows.** The Kummer ladder's C-step prediction holds at `t = 12` and `t = 13`, both with `c* = 6`. Neither
  separates "C-step" from "flat at 6". Amendment 7 E names `t = 14` (predicting 7) as the decisive cell. Kummer `t = 14`
  cap 6 is not on this route yet (owner decision needed, R3 §2).
- **Cost.** Pod 04:37–07:20Z, about US$2.8 (derived).
