# Actual first-collision obstruction for bounded increment tables

This dependent Lean project follows item10 of the fourteen-item audit.
It uses the actual map f(x)=x+(h(x)+1) on Z/p, with h:Z/p→Fin r,
and defines T as the least index whose state repeats an earlier state.
It proves p≤rT and T≤p for every labeling. The actual independent
uniform node-label PMF therefore has p≤rE[T] and E[T]≤p. [pcc:unverified]

For r=20 and p>20, increments1..20 are distinct and nonzero. Actual
indegree has mean1 and variance19/20. Nevertheless, for every real C,
some prime p>20 has E[T]²>Cp. This directly disproves a uniform
variance-only square-root bound over these fixed supports. [pcc:unverified]

Galbraith14.2.1 samples the increment table uniformly. Here it is fixed.
This known obstruction does not refute the qualified random-table
collision heuristic or Hildebrand's average-over-supports theorem.
No exact limiting collision constant or new native population is claimed.
No novelty claim. [pcc:unverified]

| Module / declaration | Obligation |
|:--|:--|
| FirstCollision.firstCollision | Actual least repeat, bounded by p |
| BoundedSteps.repeat_lower_bound | Strict natural lift and positive divisible displacement imply p≤rT |
| UniformCollision.expectation_eq_pmf | Finite sum equals actual normalized product PMF expectation |
| UniformCollision.meanCollision_lower | Pointwise bound gives p≤rE[T] |
| UniformCollision.mean_and_variance | Actual edge counts, distinct supports, uniform weights |
| Counterfamily.twenty_partition_certificate | All support, actual-iterate, PMF and variance conditions |
| Counterfamily.arbitrarily_large_collision_ratio | Arbitrarily large primes violate each natural coefficient |
| Counterfamily.universal_variance_only_collision_bound_false | Direct negation with every real coefficient |

Namespaces: RhoFirstCollisionAudit, RhoBoundedStepsAudit,
RhoUniformCollisionAudit and RhoCollisionCounterfamilyAudit.
The frozen parent WalkWeightedGeneralAudit is unchanged. [pcc:unverified]

Source review: Galbraith chapter14, adding-walk definition14.2.1 and
14.2.5 pp296–297; Hildebrand1994 Theorems1–2 pp191–193;
actual Mathlib Euclid proof Nat.exists_infinite_primes and relevant
iterate/ZMod APIs. Read scopes and primary hashes are in
[evidence/source-review.json](evidence/source-review.json).
KB unavailable (MCP absent/wrapper127); bounded primary/repository/Mathlib
fallback. Conductor and lean4checker unavailable. [pcc:unverified]

Lean4.35.0 / Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679.
Only propext, Classical.choice and Quot.sound; no holes, custom axioms,
native_decide, waivers or increased elaboration limits. No experiment,
new sampling, timing claim or paid compute. Full gate and failed builds
are retained losslessly. Sources.json freezes the preissue dependency
pins. Local PASS and independent signed standing are distinct.
[pcc:unverified]

```sh
cd research/rho_collision_audit_20261110/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/rho_collision_audit_20261110/lean
```

See sources.json, verification.json and the inspected
[report](reports/proof-20261110/report.pdf), with editable proof.dot.
[pcc:unverified]

Full project gate:35 declarations, zero failures and zero trusted-native
exceptions. All32 attempt/source files, current5 gate files and three
prior preissue gates are archived with byte-equal roundtrip checks.
Source f79068d3 precedes PCC-20261110-d4007f; both actual support/PMF
and direct-negation proof bindings pass local replay at17:27:36Z.
All16 formal/config dependency pins match. Signed standing remains
unverified. The final two-page44041byte report was inspected on both
pages. [pcc:unverified]
