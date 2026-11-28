# One-solve Lanczos on the genus-5 cover (2026-11-16)

Copies for browsing. The source of truth is
[`research/humbert_genus5_cover_20261021/la_solver_20261116`](../../research/humbert_genus5_cover_20261021/la_solver_20261116/)
(frozen `PROTOCOL.md` with its addendum, raw `campaign/q*/results/v5.jsonl`, `results/*.csv`, `REPORT.typ`).
Bench record: [`records/20261116T0059Z-humbert-genus5-la-solver.json`](../records/20261116T0059Z-humbert-genus5-la-solver.json).

Not a row in `benchmarks/README.md`: there is no replay certificate, validator report or
`VALIDATION.json`, so the validation gate does not admit it. All wall-time numbers come from an
unisolated (L0) 4-vCPU VM and are unverified assertions [pcc:unverified]. Counted LA mulmods
(Lanczos / two Wiedemann solves, same matrix: 0.203) are host-independent.
