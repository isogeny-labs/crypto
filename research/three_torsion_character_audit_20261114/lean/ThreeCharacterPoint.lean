import ThreeCharacterScalar

namespace ThreeTorsionCharacterAudit
noncomputable section
open WeierstrassCurve.Affine
variable {K : Type*} [Field K] [DecidableEq K]

omit [DecidableEq K] in
theorem origin_nonsingular (a b : K) (hb : b≠0) : (curve a b).Nonsingular 0 0 := by
  rw [WeierstrassCurve.Affine.nonsingular_iff']
  constructor
  · simp [WeierstrassCurve.Affine.equation_iff,curve]
  · right
    simpa only [curve,mul_zero,zero_add] using hb

def origin (a b : K) (hb : b≠0) : (curve a b).Point := .some 0 0 (origin_nonsingular a b hb)

omit [DecidableEq K] in
theorem origin_nonzero (a b : K) (hb : b≠0) : origin a b hb≠0 := Point.some_ne_zero _

theorem origin_three (a b : K) (hb : b≠0) : (3:ℕ) • origin a b hb=0 := by
  apply (ThreeTorsionRankAudit.General.three_point_iff (curve a b) (origin_nonsingular a b hb)).mpr
  change (curve a b).Ψ₃.eval 0=0
  rw [division_value,zero_mul]

def modelValue (W : WeierstrassCurve.Affine K) : W.Point → K
  | .zero => 1
  | .some x y _ => if x=0 then 1 else characterValue W.a₁ W.a₃ x y

abbrev pointValue (a b : K) : (curve a b).Point → K := modelValue (curve a b)

theorem pointValue_origin (a b : K) (hb : b≠0) : pointValue a b (origin a b hb)=1 := by
  simp [pointValue,modelValue,origin]

theorem outside_data (a b x y : K) (hN : (curve a b).Nonsingular x y)
    (ht : (3:ℕ) • Point.some x y hN=0) (hx : x≠0) :
    cubic a b x=0 ∧ equation a b x y=0 ∧ d a b x y≠0 ∧ y≠0 := by
  have hr := (ThreeTorsionRankAudit.General.three_point_iff (curve a b) hN).mp ht
  have he : equation a b x y=0 := by
    simpa [WeierstrassCurve.Affine.equation_iff',curve,equation] using hN.1
  have hc : cubic a b x=0 := by
    have hp : x*cubic a b x=0 := by rw [← division_value]; exact hr
    exact (mul_eq_zero.mp hp).resolve_left hx
  have hd : d a b x y≠0 :=
    ThreeTorsionRankAudit.General.psi_root_denominator_nonzero (curve a b) hN hr
  refine ⟨hc,he,hd,?_⟩
  intro hy
  have hh : x^3=0 := by
    rw [hy] at he
    simpa [equation] using congrArg Neg.neg he
  exact (pow_ne_zero 3 hx) hh

theorem pointValue_outside_nontrivial [NeZero (3:K)] (a b x y : K)
    (hN : (curve a b).Nonsingular x y) (ht : (3:ℕ) • Point.some x y hN=0) (hx : x≠0) :
    pointValue a b (Point.some x y hN)≠1 := by
  obtain ⟨hc,he,hd,_⟩ := outside_data a b x y hN ht hx
  simpa only [pointValue,modelValue,curve,ite_eq_right hx] using character_nontrivial a b x y hc he hd

theorem pointValue_cube (a b : K) (P : (curve a b).Point) (ht : (3:ℕ) • P=0) :
    (pointValue a b P)^3=1 := by
  cases P with
  | zero => norm_num [pointValue,modelValue]
  | some x y hN =>
    by_cases hx : x=0
    · simp [pointValue,modelValue,hx]
    · obtain ⟨hc,he,hd,_⟩ := outside_data a b x y hN ht hx
      simpa only [pointValue,modelValue,curve,ite_eq_right hx] using character_cube a b x y hc he hd

theorem pointValue_neg (a b : K) (P : (curve a b).Point) (ht : (3:ℕ) • P=0) :
    pointValue a b (-P)=(pointValue a b P)⁻¹ := by
  cases P with
  | zero => change (1:K)=1⁻¹; exact (inv_one).symm
  | some x y hN =>
    change pointValue a b (Point.some x ((curve a b).negY x y) _)=_
    by_cases hx : x=0
    · simp [pointValue,modelValue,hx]
    · obtain ⟨hc,he,hd,_⟩ := outside_data a b x y hN ht hx
      simpa only [pointValue,modelValue,ite_eq_right hx,WeierstrassCurve.Affine.negY,curve] using
        character_negation a b x y hc he hd

theorem addition_x (a b x y : K) (hx : x≠0) (he : equation a b x y=0) :
    (curve a b).addX x 0 ((curve a b).slope x 0 y 0)=-b*y/x^2 := by
  rw [WeierstrassCurve.Affine.slope_of_X_ne hx]
  simp only [sub_zero,WeierstrassCurve.Affine.addX,curve]
  field_simp
  unfold equation at he
  linear_combination he

theorem addition_y (a b x y : K) (hx : x≠0) (he : equation a b x y=0) :
    (curve a b).addY x 0 y ((curve a b).slope x 0 y 0)=-b^2*y/x^3 := by
  simp only [WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY]
  rw [addition_x a b x y hx he,WeierstrassCurve.Affine.slope_of_X_ne hx]
  simp only [WeierstrassCurve.Affine.negY,curve,sub_zero]
  field_simp
  unfold equation at he
  linear_combination b*he

theorem pointValue_add_origin_outside (a b : K) (hb : b≠0) (x y : K)
    (hN : (curve a b).Nonsingular x y) (ht : (3:ℕ) • Point.some x y hN=0) (hx : x≠0) :
    pointValue a b (Point.some x y hN+origin a b hb)=pointValue a b (Point.some x y hN) := by
  obtain ⟨hc,he,hd,hy⟩ := outside_data a b x y hN ht hx
  have hnew : -b*y/x^2≠0 := div_ne_zero (mul_ne_zero (neg_ne_zero.mpr hb) hy) (pow_ne_zero 2 hx)
  unfold origin
  rw [Point.add_of_X_ne hx]
  simp only [pointValue,modelValue,addition_x a b x y hx he,addition_y a b x y hx he,
    ite_eq_right hnew,ite_eq_right hx]
  exact character_translation a b x y hx hb hy hc he hd

theorem pointValue_add_origin (a b : K) (hb : b≠0) (Q : (curve a b).Point)
    (ht : (3:ℕ) • Q=0) (hn : pointValue a b Q≠1) :
    pointValue a b (Q+origin a b hb)=pointValue a b Q := by
  cases Q with
  | zero => exact (hn rfl).elim
  | some x y hN =>
    have hx : x≠0 := by intro h; exact hn (by simp [pointValue,modelValue,h])
    exact pointValue_add_origin_outside a b hb x y hN ht hx

theorem pointValue_kernel [NeZero (3:K)] (a b : K) (hb : b≠0)
    (Q : (curve a b).Point) (ht : (3:ℕ) • Q=0) :
    pointValue a b Q=1 ↔ Q=0 ∨ Q=origin a b hb ∨ Q=-(origin a b hb) := by
  constructor
  · intro hv
    cases Q with
    | zero => exact Or.inl rfl
    | some x y hN =>
      have hx : x=0 := by
        by_contra hx
        exact pointValue_outside_nontrivial a b x y hN ht hx hv
      have he : y*(y+b)=0 := by
        have h := hN.1
        rw [WeierstrassCurve.Affine.equation_iff] at h
        simp only [curve,hx,mul_zero,zero_mul,zero_pow (by decide : (3:ℕ)≠0),
          zero_pow (by decide : (2:ℕ)≠0),add_zero] at h
        linear_combination h
      rcases mul_eq_zero.mp he with hy | hy
      · right; left
        unfold origin
        rw [Point.some.injEq]
        exact ⟨hx,hy⟩
      · right; right
        change Point.some x y hN=Point.some 0 ((curve a b).negY 0 0) _
        rw [Point.some.injEq]
        refine ⟨hx,?_⟩
        simpa [curve,WeierstrassCurve.Affine.negY] using eq_neg_of_add_eq_zero_left hy
  · rintro (rfl | rfl | rfl)
    · rfl
    · exact pointValue_origin a b hb
    · rw [pointValue_neg a b _ (origin_three a b hb),pointValue_origin,inv_one]

theorem pointValue_add_origin_nsmul (a b : K) (hb : b≠0) (Q : (curve a b).Point)
    (ht : (3:ℕ) • Q=0) (hn : pointValue a b Q≠1) (n : ℕ) :
    pointValue a b (Q+n • origin a b hb)=pointValue a b Q := by
  induction n with
  | zero => simp
  | succ n ih =>
    have htn : (3:ℕ) • (Q+n • origin a b hb)=0 := by
      rw [nsmul_add,ht,zero_add,smul_comm,origin_three,smul_zero]
    have hnn : pointValue a b (Q+n • origin a b hb)≠1 := by rw [ih]; exact hn
    calc pointValue a b (Q+(n+1) • origin a b hb)=
           pointValue a b ((Q+n • origin a b hb)+origin a b hb) := by
             rw [add_nsmul,one_nsmul,add_assoc]
         _=pointValue a b (Q+n • origin a b hb) := pointValue_add_origin a b hb _ htn hnn
         _=pointValue a b Q := ih

theorem modelValue_frobenius {F L : Type*} [Field F] [Fintype F] [Field L]
    [Algebra F L] [DecidableEq L] (W : WeierstrassCurve.Affine F)
    (Q : (W.baseChange L).Point) :
    modelValue (W.baseChange L) (FrobeniusPointDescentAudit.pointFrobenius W Q)=
      (modelValue (W.baseChange L) Q)^Fintype.card F := by
  let f := FiniteField.frobeniusAlgHom F L
  cases Q with
  | zero => change (1:L)=1^Fintype.card F; exact (one_pow _).symm
  | some x y hN =>
    change (if f x=0 then 1 else characterValue (algebraMap F L W.a₁) (algebraMap F L W.a₃) (f x) (f y))=
      (if x=0 then 1 else characterValue (algebraMap F L W.a₁) (algebraMap F L W.a₃) x y)^Fintype.card F
    by_cases hx : x=0
    · simp [hx]
    · have hfx : f x≠0 := (map_ne_zero f).mpr hx
      simp only [ite_eq_right hx,ite_eq_right hfx]
      change characterValue (algebraMap F L W.a₁) (algebraMap F L W.a₃) (f x) (f y)=
        f (characterValue (algebraMap F L W.a₁) (algebraMap F L W.a₃) x y)
      have hm := characterValue_map f.toRingHom (algebraMap F L W.a₁) (algebraMap F L W.a₃) x y
      change characterValue (f (algebraMap F L W.a₁)) (f (algebraMap F L W.a₃)) (f x) (f y)=
        f (characterValue (algebraMap F L W.a₁) (algebraMap F L W.a₃) x y) at hm
      simpa only [f.commutes] using hm

#print axioms origin_three
#print axioms pointValue_cube
#print axioms pointValue_neg
#print axioms pointValue_add_origin
end
end ThreeTorsionCharacterAudit
