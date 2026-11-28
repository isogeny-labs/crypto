# Genus-5 Humbert cover: slope test against a SIMD matched rho (2026-11-20)

Study: `research/humbert_genus5_cover_20261021/slope_20261120` (PROTOCOL.md frozen before data;
REPORT.typ / report.pdf). Bench record: `benchmarks/records/20261120T2333Z-humbert-genus5-slope.json`.

The campaign was 160 runs on toy fields, q = 16001–65521; all solved. Counted fields are
identical: 10/10 against v7 and 40/40 with LA_FASTMOD. LA_FASTMOD makes t_LA 0.21–0.24× and
t_ic 0.45–0.69×. The SIMD rho runs at 3.6–4.5× the v7 rho's steps per second. The best IC arm
takes 3.89, 1.97, 2.00 and 1.37 of the SIMD rho's expected time. The slopes are 1.21 for IC and
2.00 for rho, which puts the extrapolated crossover near q ≈ 92 000, beyond q < 2^16. L0,
unverified assertion [pcc:unverified]. Not a leaderboard or README row: there is no
VALIDATION.json (validation gate).
