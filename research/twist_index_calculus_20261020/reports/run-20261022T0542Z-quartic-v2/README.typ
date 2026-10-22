#set page(paper: "a4", margin: 19mm)
#set text(font: "Liberation Sans", size: 9pt)
= Quartic twist24 full-cost v2 run
UTC start `2026-10-22 05:42:50.656`; source commit `4d6cb3168e`; host Intel Xeon 6975P-C, 32 logical CPUs; 16 Rayon threads; L0 shared host. Frozen protocol: `QUARTIC_TWIST_LINE_PROTOCOL.md`.

Seven curves, three one-target seeds each: 21/21 records, 63/63 IC solves, 1,008/1,008 matched rho solves. All scalars recovered $271$. Independent native replay checked 1,821 selected relations, orbit coefficients, matrix rows, rank spans and all answers. Zero failures or timeouts. Raw file: `quartic_twist24_full_v2.ndjson`, SHA-256 `2c050172a0136bf38185ca54ec1a558346548b10bb9b8463a8b77497c0e6aa34`.

The combined fold uses one fourth as many log columns as cubic-only on the same physical base. Median cubic/combined collection-query ratios run from 2.57 to 3.81 and row-operation ratios from 10.78 to 14.79. The recorded $F_p$-multiplication ratio ranges from 0.998 to 3.80. The full24 rho count remains below the twist IC count on every curve.

#image("quartic_twist24_cost_v2.png", width: 100%)

The plot is reconstructed from the raw 21 rows, with three-seed min/max bars. Field multiplications and modular row operations are separate units. Missing IC-stream group additions, hash/table work and calibrated GAE prevent a complete cost ranking. Wall milliseconds are L0 diagnostics only. Exact command, source hashes, raw links and limits are in `README.md`; the full research report is `QUARTIC_TWIST24_REPORT.md` in the study directory.
