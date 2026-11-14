import ShiftPointGroup

namespace BKThreeNormalizationAudit
noncomputable section
open WeierstrassCurve.Affine

theorem shifted_map {R S : Type*} [CommRing R] [CommRing S]
    (W : WeierstrassCurve.Affine R) (r s t : R) (f : R →+* S) :
    (shifted W r s t).map f=shifted (W.map f) (f r) (f s) (f t) := by
  ext <;> simp [shifted,WeierstrassCurve.map,map_ofNat]

variable {K : Type*} [Field K] [DecidableEq K]

def curveEquiv {W Z : WeierstrassCurve.Affine K} (h : W=Z) : W.Point ≃+ Z.Point := by
  subst Z
  exact AddEquiv.refl _

theorem curveEquiv_some {W Z : WeierstrassCurve.Affine K} (h : W=Z)
    (x y : K) (hN : W.Nonsingular x y) :
    curveEquiv h (Point.some x y hN)=Point.some x y (h ▸ hN) := by
  subst Z
  rfl

variable {F A B : Type*} [Field F] [Field A] [Field B]
  [Algebra F A] [Algebra F B] [DecidableEq A] [DecidableEq B]

omit [DecidableEq A] in
theorem shifted_baseChange (W : WeierstrassCurve.Affine F) (r s t : F) :
    (shifted W r s t).baseChange A=
      shifted (W.baseChange A) (algebraMap F A r) (algebraMap F A s) (algebraMap F A t) :=
  shifted_map W r s t (algebraMap F A)

omit [DecidableEq A] in
theorem base_discriminant_nonzero (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0) :
    (W.baseChange A).Δ≠0 := by
  exact (WeierstrassCurve.map_Δ W (algebraMap F A)).symm ▸
    (map_ne_zero (algebraMap F A)).mpr hΔ

def baseShiftEquiv (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0) (r s t : F) :
    ((shifted W r s t).baseChange A).Point ≃+ (W.baseChange A).Point :=
  (curveEquiv (shifted_baseChange (A:=A) W r s t)).trans
    (shiftEquiv (W.baseChange A) (base_discriminant_nonzero W hΔ)
      (algebraMap F A r) (algebraMap F A s) (algebraMap F A t))

theorem baseShiftEquiv_some (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0) (r s t : F)
    (x y : A) (hN : ((shifted W r s t).baseChange A).Nonsingular x y) :
    ∃ h : (W.baseChange A).Nonsingular (x+algebraMap F A r)
        (y+algebraMap F A s*x+algebraMap F A t),
      baseShiftEquiv W hΔ r s t (Point.some x y hN)=Point.some _ _ h := by
  change ∃ h, shiftEquiv (W.baseChange A) (base_discriminant_nonzero W hΔ)
    (algebraMap F A r) (algebraMap F A s) (algebraMap F A t)
    (curveEquiv (shifted_baseChange (A:=A) W r s t) (Point.some x y hN))=Point.some _ _ h
  rw [curveEquiv_some (shifted_baseChange (A:=A) W r s t) x y hN]
  exact ⟨_,rfl⟩

theorem baseShiftEquiv_map (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0) (r s t : F)
    (f : A →ₐ[F] B) (P : ((shifted W r s t).baseChange A).Point) :
    Point.map (W':=W) f (baseShiftEquiv W hΔ r s t P)=
      baseShiftEquiv W hΔ r s t (Point.map (W':=shifted W r s t) f P) := by
  cases P with
  | zero => simp only [←Point.zero_def,map_zero]
  | some x y h =>
    obtain ⟨hA,hEqA⟩ := baseShiftEquiv_some W hΔ r s t x y h
    rw [hEqA,Point.map_some,Point.map_some]
    obtain ⟨hB,hEqB⟩ := baseShiftEquiv_some W hΔ r s t (f x) (f y) _
    rw [hEqB,Point.some.injEq]
    simp only [map_add,map_mul,f.commutes,and_self]

theorem baseShiftEquiv_inclusion [Algebra A B] [IsScalarTower F A B]
    (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0) (r s t : F)
    (P : ((shifted W r s t).baseChange A).Point) :
    BKFrobeniusBridgeAudit.pointInclusion F A B W (baseShiftEquiv W hΔ r s t P)=
      baseShiftEquiv W hΔ r s t
        (BKFrobeniusBridgeAudit.pointInclusion F A B (shifted W r s t) P) := by
  cases P with
  | zero => simp only [←Point.zero_def,map_zero]
  | some x y h =>
    obtain ⟨hA,hEqA⟩ := baseShiftEquiv_some W hΔ r s t x y h
    change Point.map (W':=W) (Algebra.ofId A B) _=
      baseShiftEquiv W hΔ r s t (Point.map (W':=shifted W r s t) (Algebra.ofId A B) _)
    rw [hEqA,Point.map_some,Point.map_some]
    obtain ⟨hB,hEqB⟩ := baseShiftEquiv_some W hΔ r s t
      ((Algebra.ofId A B) x) ((Algebra.ofId A B) y) _
    rw [hEqB,Point.some.injEq]
    change algebraMap A B (x+algebraMap F A r)=algebraMap A B x+algebraMap F B r ∧
      algebraMap A B (y+algebraMap F A s*x+algebraMap F A t)=
        algebraMap A B y+algebraMap F B s*algebraMap A B x+algebraMap F B t
    simp only [map_add,map_mul,←IsScalarTower.algebraMap_apply,and_self]

theorem shift_frobenius [Fintype F] [DecidableEq F]
    (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0) (r s t : F)
    (P : ((shifted W r s t).baseChange A).Point) :
    FrobeniusPointDescentAudit.pointFrobenius W (baseShiftEquiv W hΔ r s t P)=
      baseShiftEquiv W hΔ r s t
        (FrobeniusPointDescentAudit.pointFrobenius (shifted W r s t) P) :=
  baseShiftEquiv_map W hΔ r s t (FiniteField.frobeniusAlgHom F A) P

def shiftTorsionEquiv (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0) (r s t : F) (n : ℕ) :
    {P : ((shifted W r s t).baseChange A).Point // n • P=0} ≃
      {P : (W.baseChange A).Point // n • P=0} where
  toFun P := ⟨baseShiftEquiv W hΔ r s t P.val,by rw [←map_nsmul,P.property,map_zero]⟩
  invFun P := ⟨(baseShiftEquiv W hΔ r s t).symm P.val,by rw [←map_nsmul,P.property,map_zero]⟩
  left_inv P := by apply Subtype.ext; exact (baseShiftEquiv W hΔ r s t).left_inv _
  right_inv P := by apply Subtype.ext; exact (baseShiftEquiv W hΔ r s t).right_inv _

theorem shift_torsion_cardinality (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0)
    (r s t : F) (n : ℕ) :
    Nat.card {P : ((shifted W r s t).baseChange A).Point // n • P=0}=
      Nat.card {P : (W.baseChange A).Point // n • P=0} :=
  Nat.card_congr (shiftTorsionEquiv W hΔ r s t n)

theorem shift_torsion_image_iff [Algebra A B] [IsScalarTower F A B]
    (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0) (r s t : F) (n : ℕ) :
    (∀ P : ((shifted W r s t).baseChange B).Point, n • P=0 →
      P∈Set.range (BKFrobeniusBridgeAudit.pointInclusion F A B (shifted W r s t))) ↔
    (∀ P : (W.baseChange B).Point, n • P=0 →
      P∈Set.range (BKFrobeniusBridgeAudit.pointInclusion F A B W)) := by
  constructor
  · intro h P hP
    let Q := (baseShiftEquiv W hΔ r s t).symm P
    have hQ : n • Q=0 := by
      change n • (baseShiftEquiv W hΔ r s t).symm P=0
      rw [←map_nsmul,hP,map_zero]
    obtain ⟨T,hT⟩ := h Q hQ
    refine ⟨baseShiftEquiv W hΔ r s t T,?_⟩
    rw [baseShiftEquiv_inclusion]
    rw [hT]
    exact (baseShiftEquiv W hΔ r s t).apply_symm_apply P
  · intro h P hP
    have hQ : n • baseShiftEquiv W hΔ r s t P=0 := by rw [←map_nsmul,hP,map_zero]
    obtain ⟨T,hT⟩ := h (baseShiftEquiv W hΔ r s t P) hQ
    refine ⟨(baseShiftEquiv W hΔ r s t).symm T,?_⟩
    apply (baseShiftEquiv W hΔ r s t).injective
    rw [←baseShiftEquiv_inclusion,
      (baseShiftEquiv W hΔ r s t).apply_symm_apply]
    exact hT

#print axioms baseShiftEquiv_map
#print axioms shift_torsion_cardinality
#print axioms shift_torsion_image_iff
end
end BKThreeNormalizationAudit
