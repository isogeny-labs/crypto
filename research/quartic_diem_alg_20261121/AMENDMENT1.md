# Amendment 1 (2026-11-21, after the run, before any analysis)

The frozen command omitted `--rho-cap 1.2e8`; with the default cap the quartic rho arm is skipped
above `expected_steps > 3e5` and was skipped at q = 8209 (6 skip records in `quartic_alg.jsonl`),
so Q5 had no rho row there. The `Q_rho_neg` rows of the six (instance, seed) pairs at 8209 are taken
from the bound-r study (`bound_r.jsonl`, run with `--rho-cap 1.2e8`), as PROTOCOL section 2 already
allowed for the reusable rho rows. `analyze.py` does this and checks that the nine quartic instance
lines (curve seed, `f`, order, `h`, `r`, point count) are identical in both files; the rho rows at
2053 and 4099 of this run equal the bound-r rows on every counted field (see
`analysis.json: boundr_crosscheck`). Nothing else changed.

Protocol sentence Q1, read as the analysis implements it: (a) Scan vs Algebraic agree on every counted
field outside `rel_decomp`; (b) `+alg+sge` agrees with Scan on the whole relation phase (relations,
rows, trials, core shape, attempts, `rel_*` outside the decomposition share); its `group_ops`,
`cantor_mul` and `cantor_inv` also contain the certificate multiplications `[t']G` of the candidates
that each solve returns, which the merged solve may change, so they are reported, not required equal.
