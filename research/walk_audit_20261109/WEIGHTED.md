# Exact unequal-weight in-degree moments

Lean proves a genuine finite probability law on all 8actual hash functions
Z/3→Fin2. Each vertex independently chooses the distinct nonzero increment
+1with probability a and+2with probability 1−a, for every 0≤a≤1. At vertex 0,
actual in-degree has mean 1 and variance 1−(a²+(1−a)²). For probabilities
3/4 and 1/4 the exact variance is 3/8. [pcc:unverified]

The complete expanded walk project gate passes43 declarations,0 failures
and0 trusted-native exceptions. Only propext, Classical.choice and
Quot.sound are accepted. No custom axiom, hole or native decision shortcut.
[Gate summary](evidence/weighted-gate-02.log) and
[full axiom receipt](evidence/weighted-gate-02/research_walk_audit_20261109_lean/audit.log).
[pcc:unverified]

The proof constructs a bijection between all 8three-bit tables and all
actual hash functions. Kernel finite decision checks each actual graph
in-degree. A product of the3 independent vertex-label weights is proved
nonnegative and normalized; PMF.ofFintype constructs the actual probability
law. Ordinary real polynomial algebra proves the moments for arbitrary a;
it is not a fit to sampled values. [pcc:unverified]

| Obligation | Declaration in `WalkWeightedAudit` |
|:--|:--|
| Every actual hash function is included | `table_bijective`, `sum_hashes` |
| Actual finite graph/count identity | `smallInDegree_table` |
| Nonnegative normalized product weights | `hashWeight_nonnegative`, `weight_sum` |
| Genuine PMF and exact real mass | `hashLaw`, `hashLaw_real_mass` |
| Mean and variance | `actual_weighted_moments` |
| Unequal known-answer control | `actual_unequal_weight_control` |
| Combined binding | `weighted_variance_audit_certificate` |

This validates the variance term 1−Σpᵢ² component for this exact finite ensemble.
It does not prove the 1/√(1−Σpᵢ²)collision-time penalty, a limiting constant,
Hildebrand's averaged mixing theorem, practical hash-family behavior or
general support-size formulas. No new native experiment, timing or paid
compute was run. Prior frozen source/protocol/data remain unchanged;
only additive proof-target registration changes build configuration.
[pcc:unverified]

Primary Galbraith chapter 14, version 2, §14.2.5, pages 296–297 was re-read, including its ideal
random-function model and separate conjectural collision estimates; PDF
SHAab59d8013db88ca145a101ddf9a9e28876fc4d10c76aac115d11da7bbbe1fada.
The earlier actual uniform variance counterexample remains its own result;
a true weighted moment is not presented as a disproof of the collision
heuristic. [pcc:unverified]

See [source](lean/WalkWeighted.lean), [source plan](weighted-sources.json),
and retained failed-source/diagnostic gzip artifacts in evidence/.
Scientific PCC standing is unsigned until an independent signed verdict.
[pcc:unverified]

Source commit `a88c6fa0` precedes packet `PCC-20261110-785828`. Its full local
check passes at 2026-11-10T00:55:32Z on producer host `762f5fc120c9`;
signed standing remains unverified. See [actual bindings](weighted-verification.json),
[inspected two-page report](reports/proof-20261110-weighted-rho/report.pdf), and
[editable report](reports/proof-20261110-weighted-rho/report.md). [pcc:unverified]
