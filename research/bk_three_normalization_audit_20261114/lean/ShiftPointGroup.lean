import ShiftCoordinates

namespace BKThreeNormalizationAudit
noncomputable section
open WeierstrassCurve.Affine
variable {K : Type*} [Field K] [DecidableEq K]

omit [DecidableEq K] in
theorem shifted_nonsingular (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (r s t x y : K) (h : (shifted W r s t).Nonsingular x y) :
    W.Nonsingular (x+r) (y+s*x+t) :=
  (W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp ((shifted_equation W r s t x y).mp h.1)

def pointShift (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (r s t : K) :
    (shifted W r s t).Point → W.Point
  | .zero => .zero
  | .some x y h => .some (x+r) (y+s*x+t) (shifted_nonsingular W hΔ r s t x y h)

omit [DecidableEq K] in
theorem unshifted_nonsingular (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (r s t x y : K) (h : W.Nonsingular x y) :
    (shifted W r s t).Nonsingular (x-r) (y-s*(x-r)-t) := by
  apply ((shifted W r s t).equation_iff_nonsingular_of_Δ_ne_zero (by rwa [shifted_discriminant])).mp
  rw [shifted_equation]
  have hx : x-r+r=x := by ring
  have hy : y-s*(x-r)-t+s*(x-r)+t=y := by ring
  rw [hx,hy]
  exact h.1

def pointUnshift (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (r s t : K) :
    W.Point → (shifted W r s t).Point
  | .zero => .zero
  | .some x y h => .some (x-r) (y-s*(x-r)-t) (unshifted_nonsingular W hΔ r s t x y h)

omit [DecidableEq K] in
theorem pointShift_left_inverse (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (r s t : K) :
    Function.LeftInverse (pointUnshift W hΔ r s t) (pointShift W hΔ r s t) := by
  intro P
  cases P with
  | zero => rfl
  | some x y h =>
    simp only [pointShift,pointUnshift,Point.some.injEq]
    constructor <;> ring

omit [DecidableEq K] in
theorem pointShift_right_inverse (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (r s t : K) :
    Function.RightInverse (pointUnshift W hΔ r s t) (pointShift W hΔ r s t) := by
  intro P
  cases P with
  | zero => rfl
  | some x y h =>
    simp only [pointShift,pointUnshift,Point.some.injEq]
    constructor <;> ring

omit [DecidableEq K] in
theorem inverse_branch_iff (W : WeierstrassCurve.Affine K) (r s t x₁ x₂ y₁ y₂ : K) :
    (x₁+r=x₂+r ∧ y₁+s*x₁+t=W.negY (x₂+r) (y₂+s*x₂+t)) ↔
      (x₁=x₂ ∧ y₁=(shifted W r s t).negY x₂ y₂) := by
  constructor
  · rintro ⟨hx,hy⟩
    have hx' : x₁=x₂ := add_right_cancel hx
    refine ⟨hx',?_⟩
    rw [negY_identity,hx'] at hy
    exact add_right_cancel (add_right_cancel hy)
  · rintro ⟨hx,hy⟩
    subst x₂
    refine ⟨rfl,?_⟩
    rw [negY_identity,hy]

theorem shifted_slope (W : WeierstrassCurve.Affine K) (r s t x₁ x₂ y₁ y₂ : K)
    (h₁ : (shifted W r s t).Nonsingular x₁ y₁)
    (h₂ : (shifted W r s t).Nonsingular x₂ y₂)
    (hxy : ¬(x₁=x₂ ∧ y₁=(shifted W r s t).negY x₂ y₂)) :
    W.slope (x₁+r) (x₂+r) (y₁+s*x₁+t) (y₂+s*x₂+t)=
      (shifted W r s t).slope x₁ x₂ y₁ y₂+s := by
  by_cases hx : x₁=x₂
  · have hy : y₁=y₂ := (Y_eq_of_X_eq h₁.1 h₂.1 hx).resolve_right (fun hh => hxy ⟨hx,hh⟩)
    subst x₂
    subst y₂
    have hny : y₁≠(shifted W r s t).negY x₁ y₁ := fun h => hxy ⟨rfl,h⟩
    have hny' : y₁+s*x₁+t≠W.negY (x₁+r) (y₁+s*x₁+t) := by
      rw [negY_identity]
      intro h
      exact hny (add_right_cancel (add_right_cancel h))
    rw [WeierstrassCurve.Affine.slope_of_Y_ne rfl hny',
      WeierstrassCurve.Affine.slope_of_Y_ne rfl hny]
    have hden : (y₁+s*x₁+t)-W.negY (x₁+r) (y₁+s*x₁+t)=
        y₁-(shifted W r s t).negY x₁ y₁ := by rw [negY_identity]; ring
    have hD : 2*y₁+(shifted W r s t).a₁*x₁+(shifted W r s t).a₃=
        y₁-(shifted W r s t).negY x₁ y₁ := by
      simp only [WeierstrassCurve.Affine.negY]
      ring
    rw [numerator_identity,hden,hD]
    field_simp [sub_ne_zero.mpr hny]
  · have hx' : x₁+r≠x₂+r := fun h => hx (add_right_cancel h)
    rw [WeierstrassCurve.Affine.slope_of_X_ne hx',WeierstrassCurve.Affine.slope_of_X_ne hx]
    have hden : x₁+r-(x₂+r)=x₁-x₂ := by ring
    rw [hden]
    field_simp [sub_ne_zero.mpr hx]
    ring

theorem pointShift_add (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (r s t : K)
    (P Q : (shifted W r s t).Point) :
    pointShift W hΔ r s t (P+Q)=pointShift W hΔ r s t P+pointShift W hΔ r s t Q := by
  cases P with
  | zero => simp only [←Point.zero_def,zero_add,pointShift]
  | some x₁ y₁ h₁ =>
    cases Q with
    | zero => simp only [←Point.zero_def,add_zero,pointShift]
    | some x₂ y₂ h₂ =>
      have hv := inverse_branch_iff W r s t x₁ x₂ y₁ y₂
      by_cases hxy : x₁=x₂ ∧ y₁=(shifted W r s t).negY x₂ y₂
      · have hs := hv.mpr hxy
        rw [Point.add_of_Y_eq hxy.1 hxy.2]
        change (0:W.Point)=Point.some _ _ _+Point.some _ _ _
        rw [Point.add_of_Y_eq hs.1 hs.2]
      · have hs : ¬(x₁+r=x₂+r ∧ y₁+s*x₁+t=W.negY (x₂+r) (y₂+s*x₂+t)) :=
          fun h => hxy (hv.mp h)
        rw [Point.add_some hxy]
        simp only [pointShift]
        rw [Point.add_some hs,Point.some.injEq]
        rw [shifted_slope W r s t x₁ x₂ y₁ y₂ h₁ h₂ hxy,addX_identity,addY_identity]
        exact ⟨rfl,rfl⟩

def shiftEquiv (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (r s t : K) :
    (shifted W r s t).Point ≃+ W.Point where
  toFun := pointShift W hΔ r s t
  invFun := pointUnshift W hΔ r s t
  left_inv := pointShift_left_inverse W hΔ r s t
  right_inv := pointShift_right_inverse W hΔ r s t
  map_add' := pointShift_add W hΔ r s t

#print axioms shifted_slope
#print axioms pointShift_add
#print axioms shiftEquiv
end
end BKThreeNormalizationAudit
