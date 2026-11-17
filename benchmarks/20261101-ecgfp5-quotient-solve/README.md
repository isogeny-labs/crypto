# EcGFp5 (Z/2)^4 ⋊ S5 quotient solve, m = 5 (2026-11-01): documentary copy

Browsing copies of the report for
[`research/ecgfp5_quotient_solve_20261101/`](../../research/ecgfp5_quotient_solve_20261101/):
[`report.pdf`](report.pdf), [`quotient_solve.svg`](quotient_solve.svg) and a PNG render.

The study measures the per-system cost of solving one point-decomposition system
(msolve 0.6.5, 4 threads, shared cloud host, tier L). That is a stage cost, not an
end-to-end ECDLP solve, so it is **not** an admitted benchmark row. It has no
`VALIDATION.json`, no replay certificate and no matched rho reference.

- Three prime cells: D = 2^16 at each, and the planted point was recovered at each.
- Whole-solve c_cyc = 2^46.30 / 2^46.56 / 2^46.63. Under the frozen G1 rule this is a
  CLOSE verdict (threshold 2^46).
- Probe arms at p = 4111: weighted F4 (W1, W3, W2) and Singular sba (F5a, F5b). None
  improved on grevlex F4. W1 completed 1.16× slower; the other arms stopped at a cap
  or were stopped by the owner.

No index-calculus pipeline improvement is claimed. The IC leaderboard is unchanged.
