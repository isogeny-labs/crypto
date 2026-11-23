# `inherited-f4-next` and `fes-f2` end to end, 2026-11-12

Stage diagnostic (one decomposition call, and the algebraic oracle end to
end), not a leaderboard row: no table row in `benchmarks/README.md`, no
VALIDATION.json. Study, profile, protocol and the pre-stated expectations:
[`research/inherited_f4_next_20261112/`](../../research/inherited_f4_next_20261112/).

Run 1, record `benchmarks/records/20261112T0630Z-inherited-f4-next-ab.json`
(measured): `inherited-f4-next` does the same counted work as `inherited-f4`
with faster kernels. Paired per call, 13 cells (14 to 32 unknowns), 6
targets × 3 repeats, one thread, local container `[pcc:unverified]`:

- 0.86 to 0.94× of `inherited-f4` per call on every cell (median 0.89);
- per-target `ops` (word XORs) equal on every target; 0 disagreements with
  the reference `fes-f2`;
- `fes-f2` is 0.08 to 0.8× of `inherited-f4` on wall from 14 to 24 unknowns
  and 7.8 to 14× at 30 to 32; its word-XOR count is below `inherited-f4`'s
  only at 14 unknowns.

Run 2, record `benchmarks/records/20261112T0633Z-inherited-f4-next-e2e.json`
(measured): `ic bench --sweep` on the six instances of the e2e ladder, the
algebraic oracle with `inherited-f4`, `inherited-f4-next` and `fes-f2`, all
priced from word-XOR counts `[pcc:unverified]`:

- Degree-13 Koblitz (14 unknowns): `fes-f2` gives S 94.7 (83.7–105.7)
  against `inherited-f4`'s 123.1 (117.8–128.4), 0.77×; 34.3× the matched rho
  instead of 44.6×. Every configuration recovered its planted logarithm.
- From 16 unknowns up `fes-f2` has the higher S (1.2× to 10×);
  `inherited-f4` stays the best algebraic row there.
- `inherited-f4-next` has exactly `inherited-f4`'s S on every instance and
  0.74 to 0.99× its wall per call inside the pipeline.

Not a leaderboard row: no replay certificate or validator report yet.

Files: `REPORT.pdf`, `results_ab.svg/png` (run 1), `results_e2e.svg/png` (run 2).
