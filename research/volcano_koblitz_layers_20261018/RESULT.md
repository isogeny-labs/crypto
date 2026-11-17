# Koblitz crater vs level-1 neighbours, ecbench: RESULT

Binary Koblitz curves n ∈ {19, 23, 37, 41, 61}, ℓ-isogeny level-1 neighbours of the crater built by `crypto volcano ecbench-spec`; solvers rho.plain, rho.negation, rho.signed_frobenius_strong (crater only); unit ecbench.gae, counts deterministic, isolation L0. All sessions audited (see `sessions/*.verify.log`). Numbers: TABLES.md, tables.json, fig_layers.svg, `reports/run-20261018T1625Z/report.pdf`.

## Measured
- Level-1 / crater, rho.negation: all five intervals contain 1 (n=19 1.089 [0.993, 1.195]; n=23 0.944 [0.854, 1.046]; n=37 0.959 [0.825, 1.107]; n=41 1.018 [0.897, 1.148]; n=61 0.837 [0.625, 1.073]).
- Level-1 / crater, rho.plain: contains 1 at n=19, 23, 37; n=41 is 1.152 [1.021, 1.304], which excludes 1 (1 of 10 intervals); see TABLES.md for n=61.
- Frobenius solver / negation solver on the crater (paired targets): 2.153 at n=19 and 0.864 at n=23 (fixed overhead dominates), 0.264 at n=37, 0.150 [0.132, 0.170] at n=41 against the ideal 1/√n = 0.156, so about √n gain at n ≥ 41.

## Reading
Generic rho cost is the same on the crater and its level-1 neighbours within the intervals (about ±10–35%, wider at n=61 with 6 neighbours, 4 targets each); one of ten intervals excludes 1, which is not more than chance at 95%. The intervals are not tight enough to rule out a difference below roughly 10–20%. The earlier claim of "±8%" was wrong and is retracted. The result concerns generic solvers only: no explicit isogeny map was computed, so nothing is claimed about transfer attacks along the volcano. Neighbour curves are not registered in the curve registry. n=43 was excluded (no prime-order subgroup with cofactor < 2^24). The rho-neg-aa control is bit-identical to rho-neg by construction (deterministic counts).
