# Protocol: why do isogeny seeds differ in hits per mul-add? (frozen before analysis)

2026-11-15. Follows `curve25519_isogeny_seeds_20261114` (PR #4213): per-seed rates 0.85 to 4.80 hits per 1e9 mul-adds, seeds 0-20,
cost per target identical across seeds, so the spread is in hit counts. All results [pcc:unverified]. No new engine runs: this
re-reads the existing 42 engine JSONs, `tower_cost.jsonl` and the V dumps `curve25519_miniature_descent_class_20261111/tower_dumps/v_t5_seed*.json`
(p = 1048573, A = 926).

## Outcome
Per seed s in 0..20: pooled rate r_s = (blind hits + lifts hits) / (blind + lifts counted mul-adds). Rank-based, n = 21.

## Candidate causes (features fixed now, before any correlation is computed)
F1 `|V|`: size of the dumped V set. F2 on-curve fraction: points of V whose x_M gives a curve point. F3 descent minority fraction
(min of delta=+1, delta=-1 over on-curve points, as in `classify_isogeny_base.py`). F4 engine `base_points` (from the arm JSON).
F5 engine `covered_points_all_cosets` / `columns` (blind arm). F6 `aux_order`: v2(aux_order) and |aux_order - (p+1)|.
F7 `curves_tried` (search effort; a control that should not matter).
Each feature is correlated with r_s by Spearman rho; exact two-sided permutation p-values (20,000 shuffles, seeded 2026111501).
Six primary tests (F1..F6, v2 and |trace| counted separately = 7); decision threshold p < 0.05/7 = 0.0071 (Bonferroni). F7 is a control
reported without threshold.

## Hypotheses
H10: at least one of F1-F6 is significantly correlated with r_s (Bonferroni). H11: the engine counters (F4, F5) explain more of
the rate rank than the number-theoretic features (F6): |rho(F4 or F5)| exceeds |rho| of v2 and |trace| both. H12 (control): F7 not
significant at 0.05.
Decision: supported only if the stated bound holds. If none of F1-F6 clears the threshold, the report says the cause is not
identified by these features; no post-hoc feature is promoted, any later-found one is labelled exploratory.

## Out of scope
New engine runs, other t, causal intervention, a claim that seed choice can be exploited. No Conductor task, isolab run or S3 checkpoint
(deterministic re-analysis of committed data).
