# Actual geometric 2-torsion and its characteristic boundary

For every nonsingular general Weierstrass model over a field with `2≠0`,
Lean proves its actual subgroup of Mathlib points killed by 2 is finite
and equivalent to the identity plus the distinct roots of the actual
2-torsion cubic. Its cardinality is `1 + number of roots`, at most 4.
Over every algebraically closed field, and after every algebraically
closed scalar extension, it has exactly 4 points. [pcc:unverified]

The proof constructs an explicit binary basis: two distinct nonzero
actual subgroup points `A,B`, every point equal to `0,A,B,A+B`, and
independence for the actual values of the two F2 coefficients under
the group's scalar action. This is the checked `BinaryBasis` predicate;
the rank/basis is derived from the actual geometry, not supplied as a
premise. The cubic discriminant is the existing checked `16*Δ` identity,
and Mathlib's actual coordinate/root construction supplies existence.
[pcc:unverified]

In characteristic 2, Lean proves directly from actual point negation
that a nonzero affine point is killed by 2 exactly when `a1*x+a3=0`.
For `Δ≠0,a1=0`, only the identity is killed by 2, over any field. For
`a1≠0`, there is at most one nonzero killed point; an actual curve-equation
construction gives one over every algebraically closed field. Thus the
geometric cardinality is 1 or 2, with no assumed rank. [pcc:unverified]

The concrete characteristic-2 curves `y²+xy=x³+1` and `y²+y=x³` have
discriminant 1 and actual 2-torsion cardinalities 2 and 1, respectively,
over every characteristic-2 field. On the first curve the nonzero point
is `(0,1)`, of exact order 2. Actual coordinatewise field extension is
defined and every target 2-torsion point is an image of a source point.
[pcc:unverified]

For the first curve over the actual field F2, the full point count is
proved to be 4, and its actual AlgebraicClosure(F2) has exactly two
geometric points killed by 2, both already rational over F2. Prime 2
divides the point count and its square divides it, while `2∤2−1` and
`gcd(2,2)≠1`. Actual binary fields have size `2^k` and `2∤2^k−1` for
every positive `k`. A direct Lean negation disproves the point-inclusion/
cyclotomic equivalence when coprimality is deleted. [pcc:unverified]

The **full corpus node** `THM-XT-BK-WEIL-SUFFICES` explicitly retains
`gcd(r,q)=1` as well as `r∤q−1`. The original BK1998 theorem states a
full `l²`-point condition. The characteristic-2 control refutes neither
qualified statement. The source/node hypotheses and exact corpus hash
are recorded in [source review](evidence/source-review.json). No corpus
status is rewritten and no qualified-source refutation is claimed.
[pcc:unverified]

The closed main is `TwoTorsionRankAudit.actual_two_torsion_rank_certificate`
in [RankCertificate.lean](lean/RankCertificate.lean). See the
[proof map](proof-map.json) and [report](reports/report.pdf). Whole-project
gate: 67 declarations, zero failures and zero trusted native exceptions.
Exact local **PCC-20261113-3aa5ac** replay passed at 2026-11-13T23:40:16Z.
Signed standing remains unverified; see [receipt](verification.json).
[pcc:unverified]

This closes the actual geometric rank/basis at `r=2`. Generic odd-prime
torsion rank, Frobenius determinant, actual matrix representation and
descent remain separate obligations, as do the full supersingular
geometric/Hasse classification and canonical pairings. `ordinaryMap`
is the actual coordinatewise point map used for field inclusion; no new
general Frobenius representation or group-scheme formalization is
claimed. The external Silverman book was unavailable locally and is not
marked read; exact existing Mathlib proofs and original BK text were
read. No novelty claim, native experiment, paid compute or agent upload.
[pcc:unverified]

Reproduce using `lake build` in `lean/`, or from the repository root
`tools/lean/proof-gate.sh research/two_torsion_rank_audit_20261113/lean`.
Lean/Mathlib versions, full custom sources/configs and artifact hashes
are pinned in [sources.json](sources.json). All failed sources/logs/exits
are retained losslessly. No holes, custom axioms, native evaluator,
raised limits, new global elaboration flags or waivers were introduced.
The [all-fourteen map](../theorem_census_20261109/README.md#original-fourteen-requirements)
retains the remaining work. [pcc:unverified]
