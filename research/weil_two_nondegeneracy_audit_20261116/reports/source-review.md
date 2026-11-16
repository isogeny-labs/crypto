# Sources checked before the level-two proof

The qualified modern Weil nondegeneracy theorem is the positive target.
Deleting its characteristic restriction is the falsification target.
This is a formalization of known mathematics, without a novelty claim.

Sources checked (KB): actual status, four searches and theorem search through
scripts/research-kb, plus Conductor; all exited127. Exact script/stdout/stderr/
exit receipts are retained in the attempt archive. KB unavailable is not an
absence result. The source review used bounded canonical and primary fallback.

Primary: [Galbraith, chapter26](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch26.pdf),
author version2.0,222784bytes,SHA256
c06cf2df9b8003e19694a4ebfe298a1557d61169f87ec01542258438ceac015e.
PDF1–5/printed573–577 text read before this protocol/setup/code. Earlier
individual page visual inspections credited; no fresh visual/fullchapter or
full Silverman/Washington proof read. Definition26.2.2 has coprimality;
26.2.3 lists nondegeneracy and cites external proof. Formula26.1 is g(D)/f(E),
with disjoint degree-zero representatives. Its footnote records inverse
conventions. Source26.3 defines Tate on a quotient, a different remaining task.

Canonical formal sources reviewed:
- weil_geometric_laws_audit_20261116/lean/LawsInputs.lean defines the actual
  geometricPairing and arbitrary proper representative agreement.
- LawsThreeBridge.lean shows how actual source agreement was done at3;
  it supplies no level2 theorem.
- weil_auxiliary_ratio_audit_20261115/lean/AuxiliaryDivisor.lean proves
  actual auxiliary scalar constancy by full function-divisor cancellation.
- AuxiliaryLeading/Scalar/Source/Representatives.lean supply actual infinity
  residues, translated regular functions and proper divisor evaluation.
- negation_pullback_audit_20261115/lean/PointDivisor.lean gives actual
  full vertical divisors including repeated2torsion; NegationField.lean fixes x.

Official Mathlib revision56a005855ea17e0b0958f73ec801ae140b0a1679:
- AlgebraicGeometry/EllipticCurve/Affine/Point.lean,
  isRoot_twoTorsionPolynomial_iff builds actual nonsingular points when2nonzero.
- Weierstrass.lean twoTorsionPolynomial has coefficients4,b2,2b4,b6;
  twoTorsionPolynomial_discr is16Delta.
- Algebra/CubicDiscriminant.lean card_roots_of_discr_ne_zero counts3distinct
  roots over a splitting field; Polynomial.mem_roots is the direct IsRoot bridge.

These inform a vertical-function source proof: normalize both verticals to
infinity residue1, evaluate the actual scalar identity there and obtain
c*(x-u)=u-x, hence c=-1. Proper auxiliary representatives connect that
identity to the already defined geometric quotient. Different actual roots
produce partners for any nonzero point. No assumed nondegeneracy or abstract
pairing substitutes for this. Ordinary characteristic2 gives the negative
control; characteristic0/3 and zero/self values give positive controls.

Reading-timing correction: fresh pre-code reread covered lines1–230/247,
including all of26.2 and26.3.3. Final17lines containing Miller's definition
and recursion were freshly read after state09; predecessor reading is
separate credit. The earlier “PDF1–5” description must not be read as all247
fresh lines before code. No source condition or target changed.
