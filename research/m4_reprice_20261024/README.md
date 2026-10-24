# ECC2K-130 m = 4 price table re-priced at the measured oracle exponent

2026-10-24. Class: **accounting** (derived arithmetic on committed measurements). It measures nothing
new, adds no algorithm and no speed claim, and moves no ECC2K-130 cost. Rho stays `2^60.81`
(`research/notes/ecc2k130/RESEARCH_ECC2K130_DESCENT_DEGREE.md` §2, citing the decomposition note §5.2).
Not a performance result, so there is no `benchmarks/README.md` row.

## Question

The descent-degree note prices the decomposition oracle as `C(N, <=D)^w` per target. Its
degree-5 row, `m = 4`, `w = 1`, `2^71.13`, `+10.32` bits over rho (§7, §8), is quoted downstream as
the optimistic case (`research/ic_leads_review_20261018/README.md` §1 item 4 and §3.1;
`backlog/XP-20261018-m4-solving-degree-equivariant/README.md`). The note itself calls `w = 1`
"one touch per column, which no elimination beats" (§2), i.e. a floor. What is the row at an oracle
exponent the repository has measured, and in rho-step units?

## Method

`examples/m4_reprice.rs` (native Rust, self-contained; reads `research/k_conversion_20261018/raw.jsonl`):

```text
targets        l + max(log2(m!) + n - m*l, 0)  = log2 max(2^l, m! 2^(n-(m-1)l)),  n = 131
oracle         w * log2 C(N, <=D) per target,  N = (m-2) n + m l            [A: column-touch units]
linear algebra log2(m) + 2 l                                                 [never converted]
total          log2( 2^(targets + oracle) + 2^LA ),  minimised over integer l in 4..69
B (rho steps)  oracle + log2(Ir per word-op / Ir per n=83 rho step), oracle only
```

- **Positive control (asserted before anything prints).** Rows of the existing table reproduce exactly:
  `+10.32` (m=4 D=5 w=1), `+15.50` (D=6 w=1), `+21.30` (D=7 w=1), `+45.71` (D=5 w=2, which the note
  already prints in §7/§8), `+57.78` (D=6 w=2), and the free-oracle, D=2, D=6, D=7 and fit rows of §2 for
  m = 3 and 4 (19 controls, listed at the top of `table.md`). The target-count floor
  `max(2^l, m! 2^(n-(m-1)l))` is what pins `l = 34` for m = 4, D >= 5.
- **Conversion (B).** Ir per word-op is `ir / word_ops` per closure row of
  `research/k_conversion_20261018/raw.jsonl` (median 19.20, range 4.48 to 32.50); a rho step is 6664.8 Ir
  at n = 83 (same file, 3 capped walks). Median conversion `-8.44` bits (range `-10.54` to `-7.68`). It
  is applied to the oracle only. The "K = Ir per column" figure of `k_conversion_20261018/README.md`
  is **not** stacked on `cols^w`: it is total closure Ir over columns, so it already contains the
  super-linear elimination cost and would count it twice.
- **w.** Not a constant in the data (`table.md`, "Measured inputs"). `log2 word_ops / log2 columns`
  is 1.83 to 1.95 on degree-3 closures (4765 to 6585 columns) and 2.19 to 2.20 on the one degree-4
  cell (17,902 columns); `research/dreg_closure_driver_20261016/RESULTS.md` reports 2.18 to 2.33
  at degree 4. An OLS slope of `log2 word_ops` on `log2 columns` over the 23 closure rows is 4.32
  (95% interval 3.85 to 4.80), but it is confounded: the largest cell is degree 4, the rest degree 3.
  Degree-3 rows alone give 2.80 with interval 1.55 to 4.05 over less than half a bit of `log2 columns`.
  So **no honest fitted interval for w exists in the recorded data**; the table uses 1 (floor),
  2 (note's `omega = 2`), the degree-4 ratios 2.19 and 2.33, 2.25, and the confounded 4.32 as a stress row.

## Table (m = 4, D = 5; the rest, with m = 3 and the l+4 fit, is in `table.md`)

| w | A: column-touch units, total | A: vs rho | B: rho-step units, total | B: vs rho |
|--:|--:|--:|--:|--:|
| 1.00 (floor; the existing +10.32) | `2^71.13` | `+10.32` | `2^67.76` | `+6.95` |
| 2.00 (§7/§8's own row) | `2^106.52` | `+45.71` | `2^98.08` | `+37.27` |
| 2.19 (degree-4 ratio, low) | `2^113.52` | `+52.71` | `2^105.08` | `+44.27` |
| 2.25 | `2^115.58` | `+54.77` | `2^107.14` | `+46.33` |
| 2.33 (degree-4 ratio, high) | `2^118.48` | `+57.67` | `2^110.04` | `+49.23` |
| 4.32 (confounded slope; stress) | `2^190.82` | `+130.01` | `2^182.38` | `+121.57` |

(`l = 34` throughout, except w = 1, where B moves to `l = 32`. At w = 1 the B row's conversion is
not meaningful as a price: `w = 1` is a floor on the touch count, and the conversion measures Ir per
word-op of an elimination that is not linear.)

Other degrees, m = 4, A / B vs rho: D=6 `+57.78 / +49.34` at w=2, `+68.35 / +59.92` at 2.25;
D=7 `+69.40 / +60.96` at w=2; D=4 `+33.10 / +24.66` at w=2, `+40.59 / +32.15` at 2.25;
D=2 `+8.83 / +5.44` at w=2.

## Reading

- The `+10.32` row is the **w = 1 floor**, not an estimate. At the exponents the repository has
  measured for its own closure (2.0 to 2.33), the degree-5 row is `+45.7` to `+57.7` bits in the note's
  column-touch units and `+37.3` to `+49.2` bits in rho steps. The review's "about +45" is the A column at
  w = 2; the measured-exponent optimistic case is **w = 2.25: +54.77 (A), +46.33 (B)**.
- The conversion to rho steps takes off about 8.4 bits; it does not close more than that.
- The dominant uncertainty is the extrapolation of w from matrices of 2^12 to 2^14 columns to
  `C(398, <=5)`, about 2^36 columns. The measured ratio rises with size and degree (1.83 to 1.95, then
  2.19), so w at 2^36 columns is more likely above 2.33 than below it, and each unit of w adds about 36 bits. Sparse or structured elimination could also go the other way, below 2; none is measured.
- The degree cell (does the closure stay at 5 up to N near 394) is still unmeasured and still the
  larger lever: D = 4 to 7 spans `+25` to `+61` bits in B at w = 2.

## What this does not establish

- No m = 4 solve, no degree beyond what the earlier studies measured, and no ECC2K-130 cost.
- Not a fitted w: see above. The 4.32 slope is shown only so that nobody quotes it as one.
- The unit prefactor `word_ops = cols^w` is exact at the degree-4 cell with w = its ratio, and
  overstates the measured count at degree 3.
- Ir ratios are Callgrind instruction counts from one binary (no wall-clock claims; host load
  was about 70). The n = 131 rho step is unmeasured (n = 83 is used).
- The linear-algebra term, the per-target independence of the oracle (no work shared across targets),
  and the `ell` range 4 to 69 are the old model's, unchanged.

## Files

- `examples/m4_reprice.rs` (run: `cargo run --release --example m4_reprice > research/m4_reprice_20261024/table.md`)
- `table.md` (generated; positive controls, measured inputs, full grid for m in {3, 4}, D in {2, 4, 5, 6, 7} plus the fit)

## Build status

`cargo check --example m4_reprice` with the shared `CARGO_TARGET_DIR` was started twice; the first
run stopped at `include_str!` paths outside the sparse checkout used for this worktree, the second
waited more than 15 minutes on another session's build-directory file lock and was not completed.
`table.md` was generated by compiling the same source stand-alone
(`CARGO_MANIFEST_DIR=<worktree> rustc --edition 2021 -O examples/m4_reprice.rs`); the example imports
nothing from the crate, and all 19 positive controls passed in that run. A `cargo check` in the full
workspace remains to be run by whoever reviews the branch.
