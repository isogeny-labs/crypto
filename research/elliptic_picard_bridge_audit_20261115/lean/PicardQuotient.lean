module
public import PicardSimplePole
public import Mathlib.GroupTheory.QuotientGroup.Basic

@[expose] public section
namespace EllipticPicardBridgeAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit MillerRecursionAudit FunctionDivisorFinitenessAudit
open GeometricPrincipalDivisorAudit
variable {K : Type*} [Field K] [IsAlgClosed K]

abbrev ZeroDivisors (W : WeierstrassCurve.Affine K) := (divisorDegree W).ker

/-- The actual full-function-field principal divisor homomorphism lands in degree zero. -/
def principalToZero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    Additive W.FunctionFieldˣ →+ ZeroDivisors W where
  toFun u := ⟨principalDivisorHom W u,function_divisor_degree_zero W hΔ u.toMul.ne_zero⟩
  map_zero' := Subtype.ext (principalDivisorHom W).map_zero
  map_add' u v := Subtype.ext ((principalDivisorHom W).map_add u v)

def principalZero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    AddSubgroup (ZeroDivisors W) := (principalToZero W hΔ).range

/-- The actual geometric degree-zero point-divisor quotient, not an abstract supplied group. -/
abbrev PicardZero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :=
  ZeroDivisors W ⧸ principalZero W hΔ

def divisorClass (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    ZeroDivisors W →+ PicardZero W hΔ := QuotientAddGroup.mk' (principalZero W hΔ)

def pointDivisor (W : WeierstrassCurve.Affine K) (P : W.Point) : ZeroDivisors W :=
  ⟨Finsupp.single P 1-Finsupp.single 0 1,by
    change divisorDegree W (Finsupp.single P 1-Finsupp.single 0 1)=0
    rw [map_sub,degree_single,degree_single,sub_self]⟩

omit [IsAlgClosed K] in
theorem point_divisor_zero (W : WeierstrassCurve.Affine K) : pointDivisor W 0=0 := by
  apply Subtype.ext
  exact sub_self _

variable [DecidableEq K]

/-- The actual chord/vertical step is a unit representing the addition relation in the quotient. -/
theorem point_addition_principal (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P Q : W.Point) :
    pointDivisor W P+pointDivisor W Q-pointDivisor W (P+Q) ∈ principalZero W hΔ := by
  refine ⟨Additive.ofMul (Units.mk0 (stepFunction W P Q) (step_function_nonzero W P Q)),?_⟩
  apply Subtype.ext
  ext R
  change pointOrder W (stepFunction W P Q) R =
    ((Finsupp.single P 1-Finsupp.single 0 1)+
    (Finsupp.single Q 1-Finsupp.single 0 1)-
    (Finsupp.single (P+Q) 1-Finsupp.single 0 1) : W.Point →₀ ℤ) R
  rw [(step_has_point_divisor W P Q).2 R]
  simp only [additionDivisor,Finsupp.add_apply,Finsupp.sub_apply]
  abel

/-- Actual point-to-divisor-class group homomorphism, derived from the actual addition function. -/
def pointClass (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    W.Point →+ PicardZero W hΔ where
  toFun P := divisorClass W hΔ (pointDivisor W P)
  map_zero' := by rw [point_divisor_zero,map_zero]
  map_add' P Q := by
    have hz := (QuotientAddGroup.eq_zero_iff
      (pointDivisor W P+pointDivisor W Q-pointDivisor W (P+Q))).mpr
      (point_addition_principal W hΔ P Q)
    have he : divisorClass W hΔ
      (pointDivisor W P+pointDivisor W Q-pointDivisor W (P+Q))=0 := hz
    rw [map_sub,map_add,sub_eq_zero] at he
    exact he.symm

/-- Injectivity uses the proved no-simple-pole bridge, not a supplied Picard or genus premise. -/
theorem point_class_injective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    Function.Injective (pointClass W hΔ) := by
  apply (injective_iff_map_eq_zero _).mpr
  intro P hP
  have hm : pointDivisor W P ∈ principalZero W hΔ :=
    (QuotientAddGroup.eq_zero_iff _).mp hP
  obtain ⟨u,hu⟩ := hm
  have hd := congrArg Subtype.val hu
  exact point_minus_origin_principal_only_zero W hΔ P u.toMul.ne_zero hd

#print axioms pointClass
#print axioms point_class_injective
end
end EllipticPicardBridgeAudit
