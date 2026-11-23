import LawsCertificate
import TwoImports

namespace WeilTwoNondegeneracyAudit
noncomputable section
open WeierstrassCurve.Affine
open WeilGeometricLawsAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

theorem two_cubic_card [NeZero (2 : K)] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) : W.twoTorsionPolynomial.roots.toFinset.card=3 := by
  have h4 : (4 : K)≠0 := by
    have he : (4 : K)=(2 : K)*2 := by ring
    rw [he];exact mul_ne_zero (NeZero.ne _) (NeZero.ne _)
  have h16 : (16 : K)≠0 := by
    have he : (16 : K)=(4 : K)*4 := by ring
    rw [he];exact mul_ne_zero h4 h4
  have ha : W.twoTorsionPolynomial.a≠0 := h4
  have hs : (W.twoTorsionPolynomial.toPoly.map (RingHom.id K)).Splits := by
    simpa only [Polynomial.map_id] using IsAlgClosed.splits W.twoTorsionPolynomial.toPoly
  have hd : W.twoTorsionPolynomial.discr≠0 := by
    rw [WeierstrassCurve.twoTorsionPolynomial_discr];exact mul_ne_zero h16 hΔ
  have hm : Cubic.map (RingHom.id K) W.twoTorsionPolynomial=W.twoTorsionPolynomial := rfl
  simpa only [hm] using Cubic.card_roots_of_discr_ne_zero (φ:=RingHom.id K) ha hs hd

theorem exists_other_two_point [NeZero (2 : K)] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (x : K) :
    ∃ u v : K,∃ h : W.Nonsingular u v,u≠x ∧ Point.some u v h+Point.some u v h=0 := by
  have hc : 1<W.twoTorsionPolynomial.roots.toFinset.card := by
    rw [two_cubic_card W hΔ];decide
  obtain ⟨u,hu,hux⟩:=Finset.exists_mem_ne hc x
  have hp : W.twoTorsionPolynomial.toPoly≠0 := by
    apply Cubic.ne_zero_of_a_ne_zero
    change (4 : K)≠0
    have he : (4 : K)=(2 : K)*2 := by ring
    rw [he];exact mul_ne_zero (NeZero.ne _) (NeZero.ne _)
  have hr : W.twoTorsionPolynomial.toPoly.IsRoot u :=
    (Polynomial.mem_roots hp).mp (Multiset.mem_toFinset.mp hu)
  obtain ⟨v,h,hv⟩:=(Point.isRoot_twoTorsionPolynomial_iff (inferInstance : NeZero (2 : K)) hΔ u).mp hr
  exact ⟨u,v,h,hux,hv⟩

#print axioms two_cubic_card
#print axioms exists_other_two_point
end
end WeilTwoNondegeneracyAudit
