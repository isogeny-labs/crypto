# Genus-5 Humbert cover: crossover test at q < 2^17 (2026-11-21)

Study: `research/humbert_genus5_cover_20261021/crossover_20261121` (PROTOCOL.md frozen before data;
REPORT.typ / report.pdf). Bench record: `benchmarks/records/20261121T0118Z-humbert-genus5-crossover.json`.

The campaign was 150 runs, all solved, over q = 65521–131071. The predicted crossover near
q ≈ 92 000 does not happen. The best IC arm takes 1.21, 1.48, 1.68, 1.79 and 2.00 of the SIMD
matched rho's expected time on the same instances. Two causes are measured. At fixed alpha, LA
grows with slope about 2 in q. And the r < 2^62 rule shrinks r/q^4 from 0.05 to 0.01, which flattens
rho. L0, unverified assertion [pcc:unverified]. Not a leaderboard or README row.
