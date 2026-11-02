# MP-ml_empirical-3: annealed subspace selection. OUTCOME: refutes

Pre-registration: PROTOCOL.md. Code: anneal.py (uses ../MP-ml_empirical-3_skeptic/dstar_vs_V.py). Raw: raw_n17_l5.json, raw_n19_l5.json, log_*.txt. Summary: RESULTS.json.
Metric: certified D* (Boolean Macaulay refutation degree) of descended S_3 (m=2) on UNSAT targets. Top-5 per arm evaluated on 20 fresh targets (disjoint seed).

| cell | A random best-of-B | B anneal | C shuffled-objective anneal | D poly basis <1,t,..> | null 5/50/95% (40 random V) | perm B<A |
|---|---|---|---|---|---|---|
| n=19 l=5 B=200 (main) | 2.73 | 2.78 | 2.82 | 2.03 | 2.45 / 2.775 / 2.90 | p=0.83 |
| n=17 l=5 B=100 | 2.85 | 2.49 | 2.89 | 2.16 | 2.50 / 2.95 / 3.00 | p=0.021* |

- Main cell n=19: annealed winners sit at the random median (2.78 vs 2.775). This is prediction_if_false.
- n=17: arm B touches the null 5th percentile (2.49 vs 2.50). The permutation p-value is inflated: the 5 B winners are near-copies of one subspace that share basis vectors, so they are not independent. The gap does not carry over to n=19, so it shrinks with n. This matches the protocol rule for exhausted_at_scale.
- n=23 was started, then dropped to stay inside the 20 CPU-min budget. It has no data.
- The only subspaces clearly below the null are the explicit polynomial-basis family (D*≈2.0-2.2, dimVV=9). The annealer never found them (its winners have dimVV 14-15, which is generic). That effect belongs to the existing linearization-excess / polynomial-basis lead (LEAD-ICE-linearization-excess-fb-prior), not to annealing.
- Obstruction: on generic V, D* is an integer plateau {2,3} and depends mostly on the target. A 6-target training objective is winner's-curse noise. The annealer has no gradient toward the low-dimVV families.
- Caveats: toy n only. D* is a refutation degree (UNSAT targets), not d_last. l=5 only. Budgets are smaller than the B=600 in the skeptic plan. hky_last_fall_degree could not serve as the oracle because it takes no custom V (nprime is fixed at 9).
- Classification: engineering at best. Nothing here bears on n=131.
- Next step: anneal with a structural score (dimVV / linearization excess) as a continuous objective, at n in {23,29,31}. Test whether anything beats the poly-basis family. If nothing does, close the idea into the linearization-excess lead.
