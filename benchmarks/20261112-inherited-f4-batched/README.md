# `inherited-f4-batched`: fewer counted rewrites, same decisions, 2026-11-12

Stage diagnostic (one decomposition call, and the algebraic oracle end to
end), not a leaderboard row: no table row in `benchmarks/README.md`, no
VALIDATION.json. Study, decision and counting arguments, protocol and the
pre-stated expectations:
[`research/inherited_f4_batched_20261112/`](../../research/inherited_f4_batched_20261112/).
Binary built at 4f0eca9eb4 (arena fix), one thread, local container
`[pcc:unverified]`.

Run 1, record `benchmarks/records/20261112T1829Z-inherited-f4-batched-ab.json`:
paired `ic descent`, 13 cells (14 to 32 unknowns), 6 targets × 3 repeats.

- Word XORs 0.70 to 0.91× of `inherited-f4` per target (median 0.84), never
  more; 0 disagreements with `fes-f2`.
- Wall 0.85 to 0.95× from 24 unknowns up, but 1.02 to 1.14× at 14 to 22
  unknowns; the pre-stated falsifier (wall above 1.05 at 20 unknowns and up)
  fired on random 19:10:2 (1.08).

Run 2, record `benchmarks/records/20261112T1831Z-inherited-f4-batched-e2e.json`:
`ic bench --sweep` on the six e2e-ladder instances, 3 repeats.

- S of the batched row 0.82 to 0.89× of `inherited-f4`'s on every instance
  and below it on every repeat; all configurations recovered their logarithm.
- Below 24 unknowns that S drop rests on counted work that disappears while
  uncounted per-row overhead keeps the wall at 0.99 to 1.12× of eager.

Files: `REPORT.pdf`, `results_ab.svg/png` (run 1), `results_e2e.svg/png` (run 2).
