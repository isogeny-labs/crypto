# Actual geometric Weil pairing covariance

[Report](reports/report.pdf), [protocol](PROTOCOL.md), [proof map](proof-map.json),
[source plan](sources.json), [source review](evidence/source-review.json),
[verification](verification.json).

Main: `WeilGaloisCovarianceAudit.actual_geometric_pairing_covariance_certificate`.

The [actual geometric pairing covariance proof](README.md)
now proves field-extension covariance of the source26.1 geometric divisor
quotient for every natural level, including zero, over algebraically closed
fields and general Weierstrass models with nonzero discriminant. The point,
function, local-ring and value maps are actual maps. Actual normalized
orders and residues preserve proper unit domains, actual recursive Miller
functions commute with basechange, and all four auxiliary admissibility
conditions survive. Actual principal divisors, disjoint supports and point
sums identify the chosen auxiliary representatives with the selected
geometric pairing. At zero the actual quotient equals1; root membership
alone would be vacuous. [pcc:unverified]

An actual field automorphism acts on the coefficient-twisted curve and on
the field-unit value. Same-curve covariance requires an explicit equality
of the twisted and original models. On the actual nonsingular binary curve
y²+y=x³+1 over the algebraic closure of F2, Frobenius fixes the model and
squares the actual level3 pairing value. A credited actual torsion pair
has value u unequal to1, so u²≠u; Lean disproves point-only invariance
with the codomain action omitted. The same witness supplies a coefficient
moved by Frobenius, and Lean constructs a nonsingular y²+y=x³+c whose
coefficient model changes. These refute deleted actions/model hypotheses.
Galbraith26.2.3(4) includes the value action; its qualified covariance is
proved, not refuted. [pcc:unverified]

Local **PCC-20261116-edc634 PASS at 2026-11-16T08:13:03Z**, signed status unverified. Wholegate27,
seven modules(six concrete, one official bridge), zero failures/trusted
native proof evaluations and zero new-source warnings. Eleven source
states retain failures01–07,09,10 and passes08,11. All368 predecessor
pins match before/after every state/gate and around replay. Thirty-four
packages, no copies, hydration or update. Full162-character main header
SHA`ff44e0ff24ef06b93d2666bc49f788f7335f482ead464508c5c0352ee8412648`
names four transparent proposition bundles defined in the pinned source;
separate binding after completed gate gives1complete/0prefix/0unresolved/
empty suffix. Full normalized and parsed headers read separately.
Supporting full written headers retain nested named arguments and inline
proof terms as supplemental source metadata; the frozen main exporter is
unchanged. The proof map pins282 custom modules,278 imported(271 credited
and7 new).285 direct formal/17 artifact pins match. Full predecessor
manifest is pinned and its368 entries expanded, including older
configurations/artifacts; metadata dedup removes no checked dependency.
Sourceplan61185B/full inline main. Lossless233 files/88 objects/60876-byte
XZ/one part/round-trip and actual restore equal. First-valid one-page
47303-byte report/all4900 text chars read/fullpage visual/inbounds.
Same-host prover/header checking is not independent compiler/parser/host
assurance. No holes/custom axioms/native proof evaluation/raised limits/
new flags/waivers/frozen edits/mathPython/spend. [pcc:unverified]

Sources: Galbraith chapter26,222784B/SHA
`c06cf2df9b8003e19694a4ebfe298a1557d61169f87ec01542258438ceac015e`;
fresh complete bounded PDF2–4 text and separately recovered PDF3 visual,
with predecessor PDF1–5 text/all individual visuals credited.26.2.2 has
the coprime guard;26.1 fixes quotient orientation. External Silverman/
Washington/Hess proofs not read in full. Six KB wrapper calls and initial/
expanded Conductor returned127; canonical actual/official/primary fallback,
known methods credited, no novelty or absence claim. General
multiplication-isogeny comparison, all-level nondegeneracy, descent,
canonical Tate/general MOV/Frey–Rück/nonclosed residues/backend runtime
and the other fourteen-item obligations remain open. [pcc:unverified]

Publication: ready [PR #4389](https://github.com/isogeny-labs/crypto/pull/4389). Remote proof `53e8bb9dd8c4dd29a507112fb6f0c59c6293019f` and local receipt `0f1d38263b0dc33b2aeff6035a55c7c9823260c8` share tree `870bc68d7a09c14dacca5ce54c2e205346f45614`. Anonymous fetch completed before exact fresh head/title/body/ready confirmation. All46 pinsets/6074 entries match. New-head CI, review and signing remain pending; next scheduled owner08:30UTC. [pcc:unverified]
