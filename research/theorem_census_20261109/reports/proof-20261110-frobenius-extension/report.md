---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Actual finite extensions and Frobenius iterates

For actual finite fields F⊆K, a field extension B/K and a general
Weierstrass model W over F,
Lean proves the following actual-point equivalence. Here $q=|F|$,
$k=[K:F]$ and the scalar-tower embeddings are explicit. [pcc:unverified]

$$\operatorname{Frob}_q^{\,k}(P)=P\iff
\exists Q\in W(K):\ \operatorname{map}_{K\to B}(Q)=P.$$

It proves the same criterion for the entire actual r-torsion set, every
natural r. It also checks $|K|=|F|^{[K:F]}$ and derives $[K:F]=k$ from
$|K|=|F|^k$, giving the explicit cardinality formulation used by F_(q^k).
No rationality predicate or torsion basis is assumed. [pcc:unverified]

![Actual field, point-map and descent dependencies. [pcc:unverified]](proof.pdf){width=94%}

\newpage

# What closes the bridge

Actual Point.map respects algebra-homomorphism powers, by its composition
law and induction. The actual scalar F-Frobenius to power $[K:F]$ equals
K-Frobenius restricted to F, since both act as $x\mapsto x^{|K|}$.
Generalized actual finite-base point descent then turns fixedness into
an actual W(K) point witness, using coordinate descent and checked
nonsingularity transport. Zero is handled separately. Applying this
point theorem to every point killed by r gives the torsion-set result.
[pcc:unverified]

The chosen embeddings and their compatibility are actual algebra/scalar
tower data. This does not construct a canonical finite subfield inside
a fixed algebraic closure. Mathlib points contain nonsingular affine
points and zero; the descent theorem thus also applies without requiring
the whole model's discriminant nonzero. In particular it applies to
nonsingular elliptic curves. Torsion is the actual geometric point set,
without a group-scheme interpretation. [pcc:unverified]

Main binding:
`FrobeniusFiniteExtensionAudit.finite_extension_audit_certificate`.
Proof map: `../../FROBENIUS_FINITE_EXTENSION.md`. Exact hypotheses,
source hashes, gate and actual packet standing are in
`../../frobenius-finite-extension-sources.json` and the separate verification
record. Whole census gate490passes, zero failures/zero trusted-native exceptions.
Local source commit `3c660a40` precedes packet `PCC-20261110-a496b9`;
actual local verdict and signed standing are recorded separately.
Lean4.35.0/Mathlib56a00585 are pinned. Only propext,
Classical.choice and Quot.sound; no hole, custom axiom, native-decision
exception or linter waiver. Failed attempts are preserved; fresh
lean4checker is unavailable. This module adds no elaboration option.
[pcc:unverified]

Read BK1998 Theorem1/proof/Remark1 pp142–143, DOI10.1007/s001459900040;
Mathlib actual Point.map composition, coordinate/nonsingularity descent,
AlgHom powers/restriction, finite-field Frobenius, actual cardinality/
finrank and power injectivity. KB/Conductor unavailable; bounded
primary/repository/Mathlib fallback, no novelty or broad source promotion.
[pcc:unverified]

The earlier nonsingular F7/nonfixed3-torsion disproof remains unchanged:
it attacks dropping BK's exclusion, which the qualified theorem and
corpus node retain. The new component proves a valid bridge. Actual
geometric torsion rank two, determinant q and Weil necessity remain open,
as does the all-p<500 census. No native experiment, timing or paid compute.
Signed PCC standing remains unverified pending a trusted signed verdict.
[pcc:unverified]
