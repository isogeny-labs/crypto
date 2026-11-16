import BalancedPaths

namespace RhoTwoChoiceCollisionAudit
noncomputable section
open scoped BigOperators ENNReal

/-- Literal normalized finite sum; the accompanying PMF theorem identifies its law. -/
def finiteMean {A : Type*} [Fintype A] (F : A → ℝ) : ℝ :=
  (∑ a, F a) / Fintype.card A

def uniformLaw (A : Type*) [Fintype A] [Nonempty A] : PMF A :=
  PMF.ofFintype (fun _ => ENNReal.ofReal (1 / (Fintype.card A : ℝ))) (by
    rw [← ENNReal.ofReal_sum_of_nonneg (fun _ _ => by positivity)]
    simp [Fintype.card_ne_zero])

theorem finiteMean_actual_law {A : Type*} [Fintype A] [Nonempty A] (F : A → ℝ) :
    finiteMean F = ∑ a, (uniformLaw A a).toReal * F a := by
  simp only [uniformLaw, PMF.ofFintype_apply,
    ENNReal.toReal_ofReal (by positivity : (0 : ℝ) ≤ 1 / (Fintype.card A : ℝ))]
  simp [finiteMean, div_eq_mul_inv, Finset.sum_mul, mul_comm]

theorem finiteMean_const {A : Type*} [Fintype A] [Nonempty A] (c : ℝ) :
    finiteMean (fun _ : A => c) = c := by
  simp [finiteMean, Fintype.card_ne_zero]

theorem finiteMean_mono {A : Type*} [Fintype A] {F H : A → ℝ}
    (h : ∀ a, F a ≤ H a) : finiteMean F ≤ finiteMean H :=
  div_le_div_of_nonneg_right (Finset.sum_le_sum (fun a _ => h a)) (by positivity)

theorem finiteMean_nonneg {A : Type*} [Fintype A] {F : A → ℝ}
    (h : ∀ a, 0 ≤ F a) : 0 ≤ finiteMean F := by
  exact div_nonneg (Finset.sum_nonneg (fun a _ => h a)) (by positivity)

theorem finiteMean_add {A : Type*} [Fintype A] (F H : A → ℝ) :
    finiteMean (fun a => F a + H a) = finiteMean F + finiteMean H := by
  simp [finiteMean, Finset.sum_add_distrib, add_div]

theorem finiteMean_sub {A : Type*} [Fintype A] (F H : A → ℝ) :
    finiteMean (fun a => F a - H a) = finiteMean F - finiteMean H := by
  simp [finiteMean, Finset.sum_sub_distrib, sub_div]

theorem finiteMean_sum {A J : Type*} [Fintype A] [Fintype J] (F : J → A → ℝ) :
    finiteMean (fun a => ∑ j, F j a) = ∑ j, finiteMean (F j) := by
  simp only [finiteMean, Finset.sum_div]
  exact Finset.sum_comm

theorem finiteMean_prod {A B : Type*} [Fintype A] [Fintype B] (F : A × B → ℝ) :
    finiteMean F = finiteMean (fun b => finiteMean (fun a => F (a,b))) := by
  simp only [finiteMean, Fintype.card_prod, Nat.cast_mul, Fintype.sum_prod_type]
  rw [Finset.sum_comm]
  simp [Finset.sum_div, div_div]

theorem finiteMean_equiv {A B : Type*} [Fintype A] [Fintype B]
    (e : A ≃ B) (F : B → ℝ) : finiteMean (fun a => F (e a)) = finiteMean F := by
  rw [finiteMean, finiteMean, Equiv.sum_comp, Fintype.card_congr e]

abbrev TwoSupport (p : ℕ) := ZMod p × {δ : ZMod p // δ ≠ 0}

def supportEmbedding {p : ℕ} (z : TwoSupport p) : Fin 2 ↪ ZMod p where
  toFun i := z.1 + (i.val : ZMod p) * z.2.val
  inj' := by
    intro i j h
    fin_cases i <;> fin_cases j
    · rfl
    · have hz : z.2.val = 0 := by simpa using h.symm
      exact (z.2.property hz).elim
    · have hz : z.2.val = 0 := by simpa using h
      exact (z.2.property hz).elim
    · rfl

def supportEquiv (p : ℕ) : TwoSupport p ≃ (Fin 2 ↪ ZMod p) where
  toFun := supportEmbedding
  invFun f := (f 0, ⟨f 1 - f 0, sub_ne_zero.mpr (by
    intro h
    have h10 := f.injective h
    norm_num at h10)⟩)
  left_inv z := by
    apply Prod.ext
    · change z.1 + ((0 : Fin 2).val : ZMod p) * z.2.val = z.1
      simp
    · apply Subtype.ext
      change (z.1 + ((1 : Fin 2).val : ZMod p) * z.2.val) -
        (z.1 + ((0 : Fin 2).val : ZMod p) * z.2.val) = z.2.val
      simp
  right_inv f := by
    apply Function.Embedding.ext
    intro i
    change f 0 + (i.val : ZMod p) * (f 1 - f 0) = f i
    fin_cases i <;> simp

variable {p : ℕ} [Fact p.Prime]

instance nonemptyNonzero : Nonempty {δ : ZMod p // δ ≠ 0} := ⟨⟨1, one_ne_zero⟩⟩

instance nonemptyTwoEmbeddings : Nonempty (Fin 2 ↪ ZMod p) :=
  ⟨supportEmbedding (0, ⟨1, one_ne_zero⟩)⟩

theorem support_mean_genuine (F : (Fin 2 ↪ ZMod p) → ℝ) :
    finiteMean (fun z : TwoSupport p => F (supportEmbedding z)) =
      ∑ f : Fin 2 ↪ ZMod p, (uniformLaw (Fin 2 ↪ ZMod p) f).toReal * F f := by
  rw [← finiteMean_actual_law]
  exact finiteMean_equiv (supportEquiv p) F

theorem relation_mass (d : ℕ) (hd : 0 < d) (hdp : d < p) (k : ZMod p) :
    finiteMean (fun z : TwoSupport p =>
      if (d : ZMod p) * z.1 + k * z.2.val = 0 then (1 : ℝ) else 0) = 1 / p := by
  classical
  have hdz : (d : ZMod p) ≠ 0 := by
    intro h
    have hdiv := (ZMod.natCast_eq_zero_iff d p).mp h
    exact (Nat.not_lt_of_ge (Nat.le_of_dvd hd hdiv)) hdp
  rw [finiteMean_prod]
  have hs (δ : {δ : ZMod p // δ ≠ 0}) :
      finiteMean (fun a : ZMod p =>
        if (d : ZMod p) * a + k * δ.val = 0 then (1 : ℝ) else 0) = 1 / p := by
    have he (a : ZMod p) : (d : ZMod p) * a + k * δ.val = 0 ↔
        a = -(k * δ.val) / (d : ZMod p) := by
      rw [eq_div_iff hdz]
      constructor <;> intro h <;> linear_combination h
    simp only [finiteMean, he, ZMod.card]
    simp
  simp only [hs]
  exact finiteMean_const _

theorem relation_zero_guard :
    finiteMean (fun z : TwoSupport p =>
      if (0 : ZMod p) * z.1 + 0 * z.2.val = 0 then (1 : ℝ) else 0) = 1 := by
  simp only [zero_mul, add_zero, ite_true]
  exact finiteMean_const _

theorem short_relation_mass_le (T W : ℕ) (hTp : T < p) :
    finiteMean (fun z : TwoSupport p =>
      if shortRelation T W z.1 z.2.val then (1 : ℝ) else 0) ≤
        (T : ℝ) * (2 * W + 1) / p := by
  classical
  calc
    _ ≤ finiteMean (fun z : TwoSupport p => ∑ d : Fin T, ∑ u : Fin (2 * W + 1),
        if ((d.val + 1 : ℕ) : ZMod p) * z.1 +
          ((((d.val + 1) / 2 : ℕ) : ZMod p) + (u.val : ZMod p) - (W : ZMod p)) *
            z.2.val = 0 then (1 : ℝ) else 0) := by
      apply finiteMean_mono
      intro z
      by_cases hz : shortRelation T W z.1 z.2.val
      · obtain ⟨d,u,hu⟩ := hz
        simp only [ite_true, show shortRelation T W z.1 z.2.val from ⟨d,u,hu⟩]
        apply le_trans (b := ∑ u : Fin (2 * W + 1),
          if ((d.val + 1 : ℕ) : ZMod p) * z.1 +
            ((((d.val + 1) / 2 : ℕ) : ZMod p) + (u.val : ZMod p) - (W : ZMod p)) *
              z.2.val = 0 then (1 : ℝ) else 0)
        · have hh := Finset.single_le_sum (f := fun u : Fin (2 * W + 1) =>
            if ((d.val + 1 : ℕ) : ZMod p) * z.1 +
              ((((d.val + 1) / 2 : ℕ) : ZMod p) + (u.val : ZMod p) - (W : ZMod p)) *
                z.2.val = 0 then (1 : ℝ) else 0)
            (fun _ _ => by split_ifs <;> norm_num) (Finset.mem_univ u)
          simpa only [hu, ite_true] using hh
        · exact Finset.single_le_sum (f := fun d : Fin T =>
            ∑ u : Fin (2 * W + 1), if ((d.val + 1 : ℕ) : ZMod p) * z.1 +
              ((((d.val + 1) / 2 : ℕ) : ZMod p) + (u.val : ZMod p) - (W : ZMod p)) *
                z.2.val = 0 then (1 : ℝ) else 0)
            (fun _ _ => Finset.sum_nonneg (fun _ _ => by split_ifs <;> norm_num))
            (Finset.mem_univ d)
      · simp only [hz, ite_false]
        exact Finset.sum_nonneg (fun _ _ =>
          Finset.sum_nonneg (fun _ _ => by split_ifs <;> norm_num))
    _ = _ := by
      simp only [finiteMean_sum]
      have he (d : Fin T) (u : Fin (2 * W + 1)) := relation_mass (d.val + 1)
        (by omega) (by have := d.isLt; omega)
        ((((d.val + 1) / 2 : ℕ) : ZMod p) + (u.val : ZMod p) - (W : ZMod p))
      simp only [he, Finset.sum_const, Finset.card_univ, Fintype.card_fin, nsmul_eq_mul]
      push_cast
      ring

#print axioms support_mean_genuine
#print axioms short_relation_mass_le
end
end RhoTwoChoiceCollisionAudit
