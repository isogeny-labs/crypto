import DoublingSeparableImports
import DoublingCertificate

namespace EllipticDoublingSeparabilityAudit
noncomputable section
open Polynomial EllipticDoublingDegreeAudit

/-- The finite-extension argument uses actual minpoly degree divisibility, not a separability premise. -/
theorem finite_separable_of_characteristic_not_dvd {F L : Type*} [Field F] [Field L] [Algebra F L]
    [Module.Finite F L] (p : Nat) [CharP F p] (hp : ¬ p ∣ Module.finrank F L) :
    Algebra.IsSeparable F L := by
  constructor
  intro x
  have hi : IsIntegral F x := Algebra.IsIntegral.isIntegral x
  change (minpoly F x).Separable
  rcases Polynomial.separable_or p (minpoly.irreducible hi) with hs | ⟨_,g,_,hg⟩
  · exact hs
  · exfalso
    apply hp
    apply dvd_trans ?_ (minpoly.degree_dvd hi)
    rw [←hg,Polynomial.natDegree_expand]
    exact dvd_mul_left p g.natDegree

theorem actual_odd_doubling_separable {K : Type*} [Field K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (h2 : (2 : K)≠0) :
    Algebra.IsSeparable (doublingImage W hΔ) W.FunctionField := by
  have : Module.Finite (doublingImage W hΔ) W.FunctionField := actual_doubling_finite W hΔ
  let p := ringChar K
  have : CharP K p := inferInstance
  have : CharP (doublingImage W hΔ) p := charP_of_injective_algebraMap' K p
  apply finite_separable_of_characteristic_not_dvd p
  rw [actual_doubling_degree]
  intro hd
  have hz : (4 : K)=0 := (CharP.cast_eq_zero_iff K p 4).mpr hd
  have he : (4 : K)=(2 : K)*(2 : K) := by norm_num
  exact mul_ne_zero h2 h2 (he ▸ hz)

theorem actual_identity_image_separable {K : Type*} [Field K]
    (W : WeierstrassCurve.Affine K) :
    Algebra.IsSeparable (AlgHom.id K W.FunctionField).fieldRange W.FunctionField := by
  have hf : Module.finrank (AlgHom.id K W.FunctionField).fieldRange W.FunctionField=1 :=
    EllipticFrobeniusDegreeAudit.identity_field_degree W
  have : Module.Finite (AlgHom.id K W.FunctionField).fieldRange W.FunctionField :=
    Module.finite_of_finrank_pos (by rw [hf]; decide)
  let p := ringChar K
  have : CharP K p := inferInstance
  have : CharP (AlgHom.id K W.FunctionField).fieldRange p := charP_of_injective_algebraMap' K p
  apply finite_separable_of_characteristic_not_dvd p
  rw [hf]
  intro hd
  have hz : (1 : K)=0 := by
    simpa only [Nat.cast_one] using (CharP.cast_eq_zero_iff K p 1).mpr hd
  exact one_ne_zero hz

#print axioms finite_separable_of_characteristic_not_dvd
#print axioms actual_odd_doubling_separable
end
end EllipticDoublingSeparabilityAudit
