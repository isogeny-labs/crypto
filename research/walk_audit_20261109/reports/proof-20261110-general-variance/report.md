---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Exact variance on arbitrary finite step supports

Lean proves that every finite abelian group, every distinct finite increment
table and every nonnegative real label law summing to 1 has actual
in-degree mean 1 and variance $1-\sum_i p_i^2$ at every target, under the
genuine independent-vertex-label PMF. The theorem covers all finite group
sizes and all such probability laws. [pcc:unverified]

The whole expanded walk project gate passes **70 declarations**, zero
failures and zero trusted-native exceptions. Only propext,
Classical.choice and Quot.sound are accepted. Lean 4.35.0 and Mathlib
revision prefix `56a00585` are pinned. No custom axiom or proof hole.
[pcc:unverified]

![Exact kernel controls on Z/3. The duplicate-step control deliberately violates distinctness. No sampling, uncertainty interval, runtime or collision-time inference.](moments.pdf){width=98%}

Lean directly disproves replacing distinctness by the weaker requirement
that every increment is nonzero. Two labels both selecting +1 on Z/3
produce a translation for every hash: the actual in-degree is always 1,
so variance is 0. For equal label probabilities, the unmerged-label
expression $1-\sum_i p_i^2$ is 1/2. The group, probability law and nonzero
increments are preserved; the failed hypothesis is explicit.
[pcc:unverified]

This is a negative control for an omitted assumption. It preserves the
qualified distinct-support theorem and does not refute the collision-time
heuristic. [pcc:unverified]

\newpage

# Actual graph, product probability law and proof

For $h:G\to I$, the graph is $x\mapsto x+g_{h(x)}$ and the PMF mass of $h$
is $\prod_{x\in G}p_{h(x)}$. A finite product-of-sums identity proves
normalization; nonnegativity gives an actual PMF. Coordinate-product
identities establish exact independence at different vertices.
[pcc:unverified]

The incoming-edge cardinality at target $z$ is proved equal to the sum of
indicators $h(z-g_i)=i$. Its mean is 1. Distinct increments give distinct
predecessors for different labels, allowing the second moment to split
into equal-label and independent-label terms. Finite ring algebra yields
$2-\sum_i p_i^2$ for the second moment and $1-\sum_i p_i^2$ for variance.
No graph table, empirical fit or collision-time law is assumed.
[pcc:unverified]

The main binding explicitly quantifies types, group and finite structures,
label weights, normalization, step injectivity and target. It combines
the universal moment theorem with the direct nonzero-only counterexample:
`general_variance_audit_certificate` in WalkWeightedGeneralAudit.
The source and proof map are linked from `../../WEIGHTED_GENERAL.md`.
[pcc:unverified]

Source commit `6f963444` precedes theorem packet `PCC-20261110-4e62d0`.
Its full local check passes at 01:29:41 UTC on producer host
`762f5fc120c9`; signed standing remains unverified. Exact statement/source
hashes, the whole gate, failed sources and diagnostic gzip archives are
in `../../general-variance-sources.json` and evidence/.
[pcc:unverified]

The collision-time factor $1/\sqrt{1-\sum_i p_i^2}$, limiting constants,
Hildebrand's random-support averaging and practical hash-family behavior
remain separate. No new native experiment, timing or paid compute was run.
Earlier issued proof/native/protocol sources remain frozen.
[pcc:unverified]

Primary Galbraith chapter 14, version 2, section 14.2.5, pages 296–297,
was re-read at the recorded scope in weighted-sources.json, including its
ideal random-function assumption and separate conjectural collision
estimates. No novelty claim is made. [pcc:unverified]
