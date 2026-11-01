# T21 next card: wide producer arithmetic (n = 73, 83)

Follows T20 (`REPORT.typ`): the rank stage is 99% of the cold wall, 291 ns per probe landed.
Cuts, each a same-binary A/B with identical outputs, scan at K = 600 first (build contends with timed runs, so run under `tools/isolated_bench.py busy`):
1. Basis-aware root solve: drop the per-probe `to_normal`/`to_poly` byte-table maps (13.7% of the old scan).
2. Modulus-specialised reduction for the n = 73 and n = 83 pentanomials (as `wide_gf2m.rs` did for ECC2K-130).
3. Closed-form Koblitz order in `KoblitzCurve::new` (about 9 s of every process start; 0.2% of this row, only matters for short rows).
Kill: cuts 1 and 2 under 1.2x on the scan at K = 600; any change in relations or recovered scalar.
Expected: at most the remaining probe-loop share; end-to-end re-measurement only if the scan clears 1.2x, under a fresh frozen protocol with the T20 harness (`run_t20.sh`, `t20_paired_analysis`).
