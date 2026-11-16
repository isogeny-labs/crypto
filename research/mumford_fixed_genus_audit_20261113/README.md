# Fixed genus and unbounded fields: exact arithmetic counterfamily

For every `t≥0`, Lean uses the actual field of size `3^(2t+1)` and the
same degree-five, monic, separable curve polynomial `X^5−X`, with reduced
pair genus bound 2. Field sizes are proved unbounded. A concrete cyclic
subgroup of order 2 has smooth fraction 1, while the full finite universe
of valid reduced polynomial pairs has smooth fraction strictly below 1.
The actual nonsmooth witness has `u=X^2+1, v=0`. [pcc:unverified]

The group is on the eight-element image of the actual complete F3
Cantor group under injective coefficient extension. Every image sum is
proved equivalent to the unique actual normalized Cantor output, even
when tested against an arbitrary ambient candidate. The actual normalized
gcd, Bézout numerator, exact quotient and reduction identities are
transported. The subgroup is proved to be the integer multiples of the
actual generator `(X,0)`, of exact order 2, and is proper. The ambient
universe contains **all** valid reduced polynomial pairs, with finiteness
proved by coefficient encoding. It is not defined by the image list.
[pcc:unverified]

The same witness becomes smooth over every even-degree field of size
`3^(2t+2)`: Lean proves that `−1` is square and explicitly factors
`X^2+1` into linear polynomials. A direct universal negation disproves
preservation of this witness's nonsmoothness under every finite field
extension. Thus the odd-extension restriction is tested by an actual
control. [pcc:unverified]

The closed main is
`MumfordFixedGenusAudit.fixed_genus_counterfamily_certificate` in
[FixedGenusCertificate.lean](lean/FixedGenusCertificate.lean).
It binds the actual fields, curve, arithmetic, subgroup, fractions,
counterexamples beyond every field-size bound and both direct negations.
See [proof map](proof-map.json), [source review](evidence/source-review.json)
and [report](reports/report.pdf). The whole-project gate passes
60 declarations, zero failures and zero trusted native exceptions.
Exact local **PCC-20261113-eee828** replay passed at 2026-11-13T22:57:14Z.
Signed standing remains unverified; see [receipt](verification.json).
[pcc:unverified]

The formal group law is on the coefficient image. A full group law on
all ambient pairs and its geometric Picard/Jacobian identification remain
open. The subgroup order stays 2. Strict finite disparities at arbitrarily
large fields do not prove a uniform limiting gap and do not refute a
typical or large-prime subgroup asymptotic. Gaudry2000 explicitly assumes
almost-prime Jacobian order and a large prime subgroup in the algorithm;
its whole-Jacobian `1/g!` statement is asymptotic with fixed genus. This
packet does not refute those qualified claims. [pcc:unverified]

Stoll2014 Lemmas4.16–4.17, Theorem4.18 and Lemma5.2 give the established
reduced-pair, arithmetic and branch-factor torsion construction. Their
exact read scope and PDF hashes are recorded. No novelty or full-book
read is claimed. Issued parent sources are unchanged. No holes, custom
axioms, native evaluator, raised limits or new global elaboration flags
were introduced. No native experiment, sampling, paid compute or agent
upload was run. [pcc:unverified]

Rebuild using `lake build` in `lean/`, or run
`tools/lean/proof-gate.sh research/mumford_fixed_genus_audit_20261113/lean`
from the repository root. Complete custom source/config and artifact
hashes are in [sources.json](sources.json); failed sources/logs/exits are
retained losslessly. The [all-fourteen map](../theorem_census_20261109/README.md#original-fourteen-requirements)
continues to track the remaining obligations. [pcc:unverified]
