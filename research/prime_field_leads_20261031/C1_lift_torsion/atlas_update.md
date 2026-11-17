# Proposed atlas update: LEAD-TRF-padic-canonical-section (not applied)

Target: `/Volumes/SSD990-2/crypto-autoresearcher/knowledge/leads/ecdlp/transfer/LEAD-TRF-padic-canonical-section.yaml`
(this session does not edit the atlas repository).

Current `obstruction.statement` says, in part: "For p not dividing #E(F_p) the only homomorphic section is
the prime-to-p torsion section, which the formal logarithm kills ... Scope: non-anomalous prime-field curves,
all lifts of E."

Proposed `obstruction.statement`, adding the lift quantifier and the p | N boundary:

    For p > 2 and p not dividing #E(F_p), every smooth lift E~/Z_p of E has a unique homomorphic section
    E(F_p) -> E~(Z_p), namely its prime-to-p torsion section; it holds for every lift, not only the
    canonical one. Uniqueness holds for every lift (Hom(finite, Z_p) = 0). Every homomorphism to (Q_p,+)
    vanishes on it, and the Smart-type functional X -> log(N X~)/N vanishes on it since N s(Q) = O.
    When p divides #E(F_p) the splitting depends on the lift and is not settled by this card; N = p
    (anomalous) is one instance.

Proposed `forecloses` addition: "homomorphic section of E~(Z_p) -> E(F_p) on any lift when p does not
divide #E(F_p)".

Proposed `last_reviewed`: leave to the atlas maintainers; cite this report:
`research/prime_field_leads_20261031/C1_lift_torsion/REPORT.md` in the crypto worktree
`prime-field-leads-20261031` (branch research/prime-field-leads-20261031, PR isogeny-labs/crypto#3103).

Other cards: no edit proposed. LEAD-TRF-cm-canonical-lift-heights and
LEAD-EXO-padic-jet-canonical-lift are cited, not changed. Note for the P256 falsification note (not an atlas
card): line 302 and the "any lift" unit claim in RESEARCH_P256.md overstate what is shown.
