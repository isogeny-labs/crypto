---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Actual first-collision obstruction for fixed supports

For every deterministic labeling h:Z/p→Fin r, let f(x)=x+h(x)+1.
Lean defines T as the least index at which the actual iteration starting
at0 repeats an earlier state. It proves p≤rT and T≤p. Under the actual
independent uniform node-label product PMF, p≤rE[T] and E[T]≤p.
[pcc:unverified]

With r=20 and prime p>20, all increments are distinct and nonzero and
actual indegree has mean1, variance19/20. Yet no real C bounds E[T]²
by Cp uniformly over these primes. [pcc:unverified]

![Actual iteration, normalized probability and prime-infinitude proof. [pcc:unverified]](proof.pdf){width=89%}

\newpage

# Proof obligations, source qualifications and assurance

The proof lifts each actual modular path to a strictly increasing natural
path S. Its increments lie in1..r, so S(n)≤rn. Repeated residues at
i<T imply p divides S(T)−S(i)>0, giving p≤rT. A pigeonhole repeat
bound gives T≤p. No collision-time model is substituted for the actual
iteration. [pcc:unverified]

Monotonicity of the finite expectation transfers these pointwise bounds.
The weights are proved nonnegative and normalized, and the expectation
is proved equal to the genuine product PMF sum. The frozen parent proves
the actual edge-count variance for every distinct increment table.
At r20 this is19/20. Mathlib's proved Euclid theorem supplies prime
p>400C, contradicting E[T]²≤Cp and p≤20E[T]. An Archimedean bound
extends the disproof from natural to real C. [pcc:unverified]

| Declaration | Obligation |
|:--|:--|
| firstCollision_spec / firstCollision_le | Actual least repeated state and upper bound |
| repeat_lower_bound | Strict natural lift, positive displacement divisible by p |
| expectation_eq_pmf | Real expectation equals actual PMF sum |
| meanCollision_lower / upper | Linear lower bound and finite upper bound |
| twenty_partition_certificate | Actual support, iteration, PMF and moments |
| arbitrarily_large_collision_ratio | Prime-modulus witnesses for every natural coefficient |
| universal_variance_only_collision_bound_false | Direct negation for every real coefficient |

Galbraith's14.2.1 definition samples the increment table randomly. This
study fixes the special support1..20. Hildebrand1994 Theorem1 averages
over random distinct supports; its fixed-support distinction is explicit.
These source assumptions prevent promoting this counterfamily to a
refutation of their qualified random-support statements. Relevant primary
sections and exact hashes are in evidence/source-review.json. This is
formalization of a known fixed-support obstruction; no novelty claim.
[pcc:unverified]

Lean4.35.0 / Mathlib56a00585 are pinned. Only propext, Classical.choice
and Quot.sound; no holes, custom axioms, native_decide, waived linters or
increased elaboration settings. Issued parent proofs are unchanged.
Full gate, source pins, failed attempts and actual PCC replay/standing
are recorded separately. Signed standing remains unverified.
[pcc:unverified]

No native experiment, sampling, timing or paid compute. KB MCP absent,
wrapper127; bounded primary/repository/Mathlib fallback. Conductor and
lean4checker unavailable. Proof map: README.md. Exact dependency pins:
sources.json. Actual local verdict and signed standing: verification.json.
[pcc:unverified]
