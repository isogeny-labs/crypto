# Genus-5 Humbert cover: three levers on dlpf50_l1 (2026-11-16)

Study: `research/humbert_genus5_cover_20261021/levers_20261116` (PROTOCOL.md frozen before data;
REPORT.typ / report.pdf). Bench record: `benchmarks/records/20261116T0712Z-humbert-genus5-levers.json`.

The three levers are REL_FAST (relation-search arithmetic), LA_THREADS=4 (threaded Krylov
iterations) and CLIQUE_SAFE (safe clique removal in dlpx). The campaign was 135 runs on toy
fields; all were solved, with the end-to-end check passing. Counted fields with the flags are
identical to the flag-free incumbent on all 75 flagged runs.

Paired median t_ic comb/base: 0.184, 0.182 and 0.170 at q = 16001, 24001 and 32003. The relation
phase is 0.042× and the LA phase 0.29×. Host: 4-vCPU shared VM, L0, unverified assertion
[pcc:unverified]. Not a leaderboard row: there is no VALIDATION.json (validation gate), and no
`benchmarks/README.md` row for the same reason.

Files: report.pdf, levers_tic.{svg,png}, levers_phases.{svg,png}, levers_gap_to_rho.{svg,png}.
