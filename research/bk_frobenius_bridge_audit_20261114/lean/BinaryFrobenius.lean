import ActualCriterion

namespace BKFrobeniusBridgeAudit.Binary
noncomputable section
open ThreeTorsionRankAudit.General FrobeniusPointDescentAudit Matrix Module
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
abbrev F := ZMod 2
abbrev L := AlgebraicClosure F
private instance : DecidableEq L := Classical.decEq _
private instance : CharP L 2 := charP_of_injective_algebraMap' F 2
private instance : NeZero (3:L) := by
  constructor
  intro h
  have hd : 2 ∣ 3 := (CharP.cast_eq_zero_iff L 2 3).mp h
  norm_num at hd
private instance : NeZero (3:F) := ⟨by decide +kernel⟩

def curve : WeierstrassCurve.Affine F := ⟨0,0,1,0,1⟩
theorem discriminant_ne_zero : curve.Δ ≠ 0 := by decide +kernel

theorem rational_cardinality : Nat.card curve.Point=3 := by
  rw [Nat.card_congr curve.nonsingularPointEquiv]
  change Nat.card (Option {xy : F × F // curve.Nonsingular xy.1 xy.2})=3
  let : DecidablePred (fun xy : F × F => curve.Nonsingular xy.1 xy.2) :=
    fun xy => decidable_of_iff (xy.2^2+xy.2=xy.1^3+1) (by
      change xy.2^2+xy.2=xy.1^3+1 ↔ curve.Nonsingular xy.1 xy.2
      erw [←WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
        discriminant_ne_zero,WeierstrassCurve.Affine.equation_iff]
      simp [curve])
  rw [Nat.card_eq_fintype_card]
  decide +kernel

abbrev basis : Basis (Fin 2) (ZMod 3) (Torsion (L:=L) curve) :=
  actualGeometricBasis curve discriminant_ne_zero
abbrev matrix := frobeniusMatrix curve basis

theorem geometric_cardinality : Nat.card (Torsion (L:=L) curve)=9 :=
  scalar_extension_card (algebraMap F L) curve discriminant_ne_zero

theorem quartic_coordinate {x y : L} (hN : (curve.baseChange L).Nonsingular x y)
    (ht : (3:ℕ) • WeierstrassCurve.Affine.Point.some x y hN=0) : x^4=x := by
  have hr := (three_point_iff (curve.baseChange L) hN).mp ht
  have he : (curve.baseChange L).Ψ₃.eval x=0 := hr
  have hv : (curve.baseChange L).Ψ₃.eval x=3*x^4+15*x := by
    norm_num only [WeierstrassCurve.Ψ₃,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
      WeierstrassCurve.b₆,WeierstrassCurve.b₈,curve,WeierstrassCurve.baseChange,
      WeierstrassCurve.map,Polynomial.eval_add,Polynomial.eval_mul,
      Polynomial.eval_pow,Polynomial.eval_X,Polynomial.eval_C,Polynomial.eval_ofNat,map_zero,map_one,
      zero_pow (by decide : (2:ℕ)≠0),zero_mul,mul_zero,zero_add,add_zero,
      one_mul,mul_one,one_pow,zero_sub,sub_zero]
  have h2 : (2:L)=0 := CharP.cast_eq_zero L 2
  have h3 : (3:L)=1 := by linear_combination h2
  have h15 : (15:L)=1 := by linear_combination 7*h2
  rw [hv,h3,h15,one_mul,one_mul] at he
  linear_combination he-x*h2

/-- Universal integer identity used in the binary coordinate calculation. -/
theorem fourth_power_identity {R : Type*} [CommRing R] (x y : R) :
    y^4-y-(x^6+x^3)=
      (y^2+y-x^3-1)*(y^2+y+x^3)-2*y^3 := by ring

theorem frobenius_square_fixed (P : (curve.baseChange L).Point) (ht : (3:ℕ) • P=0) :
    (pointFrobenius curve)^[2] P=P := by
  cases P with
  | zero => rfl
  | some x y hN =>
    have hx := quartic_coordinate hN ht
    have he : y^2+y=x^3+1 := by
      simpa [WeierstrassCurve.Affine.equation_iff,curve,
        WeierstrassCurve.baseChange,WeierstrassCurve.map] using hN.1
    have h2 : (2:L)=0 := CharP.cast_eq_zero L 2
    have hxc : x^6=x^3 := by
      have hz : x*(x^3-1)=0 := by linear_combination hx
      rcases mul_eq_zero.mp hz with hz | hz
      · simp [hz]
      · have hcube := sub_eq_zero.mp hz
        calc x^6=(x^3)^2 := by ring
             _=x^3 := by rw [hcube]; ring
    have hy : y^4=y := by
      have hid := fourth_power_identity x y
      have he0 : y^2+y-x^3-1=0 := by linear_combination he
      rw [he0,h2,hxc] at hid
      linear_combination hid+x^3*h2
    change WeierstrassCurve.Affine.Point.some
      (FiniteField.frobeniusAlgHom F L (FiniteField.frobeniusAlgHom F L x))
      (FiniteField.frobeniusAlgHom F L (FiniteField.frobeniusAlgHom F L y)) _=
      WeierstrassCurve.Affine.Point.some x y hN
    rw [WeierstrassCurve.Affine.Point.some.injEq]
    constructor
    · change (x^2)^2=x
      simpa only [←pow_mul] using hx
    · change (y^2)^2=y
      simpa only [←pow_mul] using hy

theorem matrix_square : matrix^2=1 :=
  (matrix_power_identity_iff curve basis 2).mpr frobenius_square_fixed

theorem matrix_nonidentity : matrix≠1 := by
  intro hm
  have hf := (matrix_power_identity_iff curve basis 1).mp (by simpa using hm)
  have hd (T : Torsion (L:=L) curve) : ∃ Q : curve.Point,
      WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId F L) Q=T.val :=
    (point_frobenius_fixed_iff curve T.val).mp (by simpa only [Function.iterate_one] using hf T.val T.property)
  let descend : Torsion (L:=L) curve → curve.Point := fun T => (hd T).choose
  have hinj : Function.Injective descend := by
    intro P Q hpq
    apply Subtype.ext
    calc P.val=WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId F L) (descend P) :=
           (hd P).choose_spec.symm
         _=WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId F L) (descend Q) := congrArg _ hpq
         _=Q.val := (hd Q).choose_spec
  let : Finite curve.Point := BKLinearAudit.actual_rational_point_finite curve
  have hc := Nat.card_le_card_of_injective descend hinj
  rw [geometric_cardinality,rational_cardinality] at hc
  omega

theorem involution_fixed_vector_det : ∀ A : Matrix (Fin 2) (Fin 2) (ZMod 3),
    A^2=1 → A≠1 → (∃ v : Fin 2 → ZMod 3, v≠0 ∧ A *ᵥ v=v) → A.det=2 := by
  decide +kernel

theorem actual_determinant : matrix.det=(Fintype.card F : ZMod 3) := by
  have hv := actual_rational_fixed_vector curve basis (by rw [rational_cardinality])
  have hd := involution_fixed_vector_det matrix matrix_square matrix_nonidentity hv
  simpa only [ZMod.card,Nat.cast_ofNat] using hd

theorem actual_matrix_power_iff (k : ℕ) : matrix^k=1 ↔ 3 ∣ 2^k-1 := by
  rw [actual_power_criterion_of_det curve basis (by rw [rational_cardinality])
    (by decide +kernel) actual_determinant]
  exact cyclotomic_cast_iff 2 k (by decide)

theorem actual_extension_criterion {K : Type*} [Field K] [Fintype K]
    [Algebra F K] [Algebra K L] [IsScalarTower F K L] [DecidableEq K]
    (k : ℕ) (hc : Fintype.card K=2^k) :
    (∀ P : (curve.baseChange L).Point, (3:ℕ) • P=0 →
      P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId K L))) ↔
        3 ∣ 2^k-1 := by
  rw [actual_extension_iff_matrix_power curve basis k (by simpa only [ZMod.card] using hc)]
  exact actual_matrix_power_iff k

#print axioms rational_cardinality
#print axioms frobenius_square_fixed
#print axioms matrix_nonidentity
#print axioms actual_determinant
#print axioms actual_matrix_power_iff
#print axioms actual_extension_criterion
end
end BKFrobeniusBridgeAudit.Binary
