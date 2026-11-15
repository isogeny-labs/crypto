# RESULTS: PKM `m = 4` cells on reserved RunPod pods (Amendments 19 and 30)

Every number is `[pcc:unverified]`. Values are measured from the rows under `results/RPM4/` (sha256 of each results
tarball beside it) unless marked derived.

**Setting.** Kummer tower, `m = 4`, chain presentation, `p = 2013265921`, two random draws per cell. Exact HKY closure
(`--engine hky`). Binaries admitted by Amendment 17.

## Rows

| cell | draws | closure result (both draws agree) | d_F |
|:--|:--|:--|:--|
| Kummer `t = 4`, `c = 7` | 0, 1 | `1` enters at `c = 7` (91,276 columns, 1.33·10⁹ entries) | **7** |
| Null `t = 4`, `c = 7` | 0, 1 | `1` enters at `c = 7` (1.68·10⁹ entries) | **7** |
| Kummer `t = 5`, `c = 6` | 0, 1 | complete to `c = 6` without `1` (129,040 cols, 182,700 entries) | ≥ 7 |
| Null `t = 5`, `c = 6` | 0, 1 | complete to `c = 6` without `1` | ≥ 7 |
| Kummer `t = 5`, `c = 7` (Amendment 30, `max_entries 30064771072`) | 0, 1 | complete to `c = 7` without `1` (356,960 cols, 3.41·10⁶ entries) | **≥ 8** |
| Kummer `t = 6`, `c = 6` (Amendment 30) | 0, 1 | complete to `c = 6` without `1` (350,776 cols, 249,162 entries) | ≥ 7 |

`ffd = 4` on every row.

## Amendment 8 reading

Lower medians: `d_F(16) = 7` and `d_F(20) ≥ 8`. That is **"rise"** (derived, applying the registered rule).

Amendment 8 closes the lane ("the PKM tower factor base, judged by the HKY last fall degree") if C-step holds at
`t = 12–14` **and** `m = 4` reads rise.
- C-step holds at `t = 12` and `t = 13`.
- `t = 14` is pending its cap-7 cell (Amendment 30). At `t = 14`, cap 6 completes without refuting, so `d_F ≥ 7`.

## Sizing note

Amendment 30 sized the `t = 5`, `c = 7` closure at 40–160 GB, extrapolated from the `t = 4`, `c = 7` level. That level is
the one where `1` enters and the elimination fills in.
- At `t = 5`, `c = 7` is not a refuting level. It needed 3.4·10⁶ entries and finished in under a minute on both draws.
- The 256 GB pods were not needed, and cost cents.

## What this does not show

Degrees only. No exponent or cost result (§8). Two draws, one prime.
