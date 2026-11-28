module
public import PicardQuotient
public import Mathlib.LinearAlgebra.Finsupp.LinearCombination
public import Mathlib.Data.Finsupp.Ext

@[expose] public section
namespace EllipticPicardBridgeAudit
noncomputable section
open WeierstrassCurve.Affine GeometricPrincipalDivisorAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- Actual finite integer linear combination in the actual point group. -/
def divisorPointSum (W : WeierstrassCurve.Affine K) : (W.Point →₀ ℤ) →+ W.Point :=
  (Finsupp.linearCombination ℤ (id : W.Point → W.Point)).toAddMonoidHom

theorem point_sum_single (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℤ) :
    divisorPointSum W (Finsupp.single P n) = n • P :=
  Finsupp.linearCombination_single ℤ (v := id) n P

/-- Subtract the actual degree at the actual origin to obtain an actual degree-zero divisor. -/
def adjustDegree (W : WeierstrassCurve.Affine K) : (W.Point →₀ ℤ) →+ ZeroDivisors W where
  toFun D := ⟨D-Finsupp.single 0 (divisorDegree W D),by
    change divisorDegree W (D-Finsupp.single 0 (divisorDegree W D))=0
    rw [map_sub,degree_single,sub_self]⟩
  map_zero' := by apply Subtype.ext; simp
  map_add' D E := by
    apply Subtype.ext
    change D+E-Finsupp.single 0 (divisorDegree W (D+E)) =
      (D-Finsupp.single 0 (divisorDegree W D))+(E-Finsupp.single 0 (divisorDegree W E))
    rw [map_add,Finsupp.single_add]
    abel

omit [DecidableEq K] in
theorem adjust_single (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℤ) :
    adjustDegree W (Finsupp.single P n) = n • pointDivisor W P := by
  apply Subtype.ext
  change Finsupp.single P n-Finsupp.single 0 (divisorDegree W (Finsupp.single P n)) =
    n • (Finsupp.single P 1-Finsupp.single 0 1)
  rw [degree_single,smul_sub,Finsupp.smul_single_one,Finsupp.smul_single_one]

omit [DecidableEq K] in
theorem adjust_degree_zero (W : WeierstrassCurve.Affine K) (D : ZeroDivisors W) :
    adjustDegree W D.val=D := by
  apply Subtype.ext
  change D.val-Finsupp.single 0 (divisorDegree W D.val)=D.val
  rw [D.property,Finsupp.single_zero,sub_zero]

variable [IsAlgClosed K]

/-- Actual finite integer-divisor reduction, derived from actual chord relations through homomorphisms. -/
theorem divisor_class_reduction (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (D : W.Point →₀ ℤ) :
    pointClass W hΔ (divisorPointSum W D)=divisorClass W hΔ (adjustDegree W D) := by
  have he : (pointClass W hΔ).comp (divisorPointSum W) =
      (divisorClass W hΔ).comp (adjustDegree W) := by
    apply Finsupp.addHom_ext
    intro P n
    simp only [AddMonoidHom.comp_apply,point_sum_single,adjust_single,map_zsmul]
    rfl
  exact DFunLike.congr_fun he D

/-- Every class of actual geometric degree-zero point divisors is represented by an actual point. -/
theorem point_class_surjective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    Function.Surjective (pointClass W hΔ) := by
  intro C
  obtain ⟨D,rfl⟩ := QuotientAddGroup.mk'_surjective (principalZero W hΔ) C
  refine ⟨divisorPointSum W D.val,?_⟩
  rw [divisor_class_reduction,adjust_degree_zero]
  rfl

theorem point_class_bijective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    Function.Bijective (pointClass W hΔ) :=
  ⟨point_class_injective W hΔ,point_class_surjective W hΔ⟩

/-- Actual elliptic Abel-Jacobi group equivalence in the chosen geometric divisor model. -/
def ellipticPicardEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    W.Point ≃+ PicardZero W hΔ :=
  AddEquiv.ofBijective (pointClass W hΔ) (point_class_bijective W hΔ)

theorem elliptic_picard_equiv_apply (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) : ellipticPicardEquiv W hΔ P = divisorClass W hΔ (pointDivisor W P) := rfl

#print axioms divisor_class_reduction
#print axioms ellipticPicardEquiv
end
end EllipticPicardBridgeAudit
