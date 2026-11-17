import SecantIdeal
import Mathlib.Algebra.TrivSqZeroExt
import Mathlib.Tactic.Ring
import Mathlib.Tactic.FieldSimp
import Mathlib.RingTheory.Ideal.Maps

noncomputable section
open MvPolynomial Finsupp
namespace SecantQuadratic
variable {K : Type*} [Field K]
local notation "P" => MvPolynomial (Fin 3) K

/-- The genuine nonmonomial quadratic top ideal from the report. -/
def quadratic (a b : K) : Ideal P :=
  Ideal.span {(X 2 : P)^2, X 0*X 1,
    X 0^2-C a*X 2*X 0, X 1^2-C b*X 2*X 1}

lemma cubic_members (a b : K) :
    (X 2 : P)^2 ∈ quadratic a b ∧ X 0*X 1 ∈ quadratic a b ∧
    X 0^3 ∈ quadratic a b ∧ X 1^3 ∈ quadratic a b ∧
    X 0^2*X 2 ∈ quadratic a b ∧ X 1^2*X 2 ∈ quadratic a b := by
  let I := quadratic a b
  have hm : (X 2 : P)^2 ∈ I := Ideal.subset_span (by simp [quadratic])
  have hxz : (X 0 : P)*X 1 ∈ I := Ideal.subset_span (by simp [quadratic])
  have hx : (X 0 : P)^2-C a*X 2*X 0 ∈ I := Ideal.subset_span (by simp [quadratic])
  have hz : (X 1 : P)^2-C b*X 2*X 1 ∈ I := Ideal.subset_span (by simp [quadratic])
  have hxm : (X 0 : P)^2*X 2 ∈ I := by
    convert I.add_mem (I.mul_mem_left (X 2) hx)
      (I.mul_mem_left (C a*X 0) hm) using 1
    ring
  have hzm : (X 1 : P)^2*X 2 ∈ I := by
    convert I.add_mem (I.mul_mem_left (X 2) hz)
      (I.mul_mem_left (C b*X 1) hm) using 1
    ring
  have hx3 : (X 0 : P)^3 ∈ I := by
    convert I.add_mem (I.mul_mem_left (X 0) hx)
      (I.mul_mem_left (C a) hxm) using 1
    ring
  have hz3 : (X 1 : P)^3 ∈ I := by
    convert I.add_mem (I.mul_mem_left (X 1) hz)
      (I.mul_mem_left (C b) hzm) using 1
    ring
  exact ⟨hm, hxz, hx3, hz3, hxm, hzm⟩

lemma monomial_mem_upward (I : Ideal P) {g e : Fin 3 →₀ ℕ}
    (hge : g ≤ e) (h : monomial g (1 : K) ∈ I) (c : K) : monomial e c ∈ I := by
  have hm := I.mul_mem_left (monomial (e-g) c) h
  simpa only [monomial_mul, mul_one, tsub_add_cancel_of_le hge] using hm

lemma all_cubic_monomials (a b : K) (e : Fin 3 →₀ ℕ) (he : 3 ≤ e.degree) (c : K) :
    monomial e c ∈ quadratic a b := by
  obtain ⟨hm, hxz, hx3, hz3, hxm, hzm⟩ := cubic_members a b
  rw [SecantIdeal.degree_three] at he
  by_cases h : 2 ≤ e 2
  · apply monomial_mem_upward _ (Finsupp.single_le_iff.mpr h) _ c
    simpa [← X_pow_eq_monomial] using hm
  by_cases hxz' : 1 ≤ e 0 ∧ 1 ≤ e 1
  · apply monomial_mem_upward _ ((SecantIdeal.mixed_le_three e).mpr hxz') _ c
    simpa [monomial_add_single, ← X_pow_eq_monomial] using hxz
  by_cases hx3' : 3 ≤ e 0
  · apply monomial_mem_upward _ (Finsupp.single_le_iff.mpr hx3') _ c
    simpa [← X_pow_eq_monomial] using hx3
  by_cases hz3' : 3 ≤ e 1
  · apply monomial_mem_upward _ (Finsupp.single_le_iff.mpr hz3') _ c
    simpa [← X_pow_eq_monomial] using hz3
  have hlast : (2 ≤ e 0 ∧ 1 ≤ e 2) ∨ (2 ≤ e 1 ∧ 1 ≤ e 2) := by omega
  rcases hlast with hlast | hlast
  · apply monomial_mem_upward _ (show single 0 2 + single 2 1 ≤ e from ?_) _ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using hxm
  · apply monomial_mem_upward _ (show single 1 2 + single 2 1 ≤ e from ?_) _ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using hzm

theorem quadratic_vanishes (a b : K) : SecantIdeal.Vanishes (quadratic a b) 3 := by
  intro p hp
  rw [p.as_sum]
  apply Ideal.sum_mem
  intro e he
  exact all_cubic_monomials a b e (by rw [SecantIdeal.homogeneous_degree hp he]) _

abbrev D (K : Type*) := TrivSqZeroExt K K
abbrev DD (K : Type*) := TrivSqZeroExt (D K) (D K)

def coeffHom : K →+* DD K :=
  (TrivSqZeroExt.inlHom (D K) (D K)).comp (TrivSqZeroExt.inlHom K K)
def u : DD K := TrivSqZeroExt.inl (TrivSqZeroExt.inr (1 : K))
def v : DD K := TrivSqZeroExt.inr (1 : D K)
def wx (a : K) : DD K := coeffHom (a/2)*u + v

lemma u_sq : (u : DD K)^2 = 0 := by
  simp [u, pow_two, TrivSqZeroExt.inl_mul_inl]
lemma v_sq : (v : DD K)^2 = 0 := by simp [v, pow_two]

lemma half_add (a : K) (h2 : (2 : K) ≠ 0) :
    coeffHom (a/2) + coeffHom (a/2) = coeffHom a := by
  rw [← (coeffHom (K := K)).map_add]
  congr 1
  field_simp [h2]
  ring

lemma wx_sq (a : K) (h2 : (2 : K) ≠ 0) :
    wx a ^ 2 = coeffHom a * u * wx a := by
  unfold wx
  calc
    (coeffHom (a/2)*u + v)^2 =
        coeffHom (a/2)^2*u^2 +
          (coeffHom (a/2)+coeffHom (a/2))*u*v + v^2 := by ring
    _ = coeffHom a * u * v := by rw [u_sq, v_sq, half_add a h2]; ring
    _ = coeffHom a*u*(coeffHom (a/2)*u+v) := by
      calc
        _ = coeffHom a*coeffHom (a/2)*u^2 + coeffHom a*u*v := by rw [u_sq]; ring
        _ = _ := by ring

def witness (a : K) : P →+* DD K :=
  eval₂Hom coeffHom (fun i => if i = 0 then wx a else if i = 1 then 0 else u)

@[simp] lemma witness_X (a : K) (i : Fin 3) :
    witness a (X i) = if i = 0 then wx a else if i = 1 then 0 else u :=
  eval₂Hom_X' _ _ _

@[simp] lemma witness_C (a c : K) : witness a (C c) = coeffHom c :=
  eval₂Hom_C _ _ _

theorem quadratic_le_witness_ker (a b : K) (h2 : (2 : K) ≠ 0) :
    quadratic a b ≤ RingHom.ker (witness a) := by
  apply Ideal.span_le.mpr
  intro p hp
  simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hp
  change witness a p = 0
  rcases hp with rfl | rfl | rfl | rfl
  · simp [map_pow, u_sq]
  · simp [map_mul]
  · simp [map_sub, map_pow, map_mul, wx_sq a h2]
  · simp [map_sub, map_pow, map_mul]

theorem mx_not_mem (a b : K) (h2 : (2 : K) ≠ 0) :
    (X 2 : P)*X 0 ∉ quadratic a b := by
  intro h
  have hz := quadratic_le_witness_ker a b h2 h
  change witness a (X 2*X 0) = 0 at hz
  have hcoeff := congrArg (fun t : DD K => t.snd.snd) hz
  simp [map_mul, wx, u, v, coeffHom, TrivSqZeroExt.inlHom,
    TrivSqZeroExt.inl, TrivSqZeroExt.inr, smul_eq_mul] at hcoeff

theorem m_not_mem (a b : K) (h2 : (2 : K) ≠ 0) : (X 2 : P) ∉ quadratic a b := by
  intro h
  exact mx_not_mem a b h2 ((quadratic a b).mul_mem_right (X 0) h)

theorem one_not_mem (a b : K) (h2 : (2 : K) ≠ 0) : (1 : P) ∉ quadratic a b := by
  intro h
  apply m_not_mem a b h2
  simpa using (quadratic a b).mul_mem_left (X 2) h

/-- The actual nonmonomial quadratic ideal has first vanishing degree three. -/
theorem quadratic_first_vanishing (a b : K) (h2 : (2 : K) ≠ 0) :
    SecantIdeal.FirstVanishing (quadratic a b) 3 := by
  refine ⟨quadratic_vanishes a b, ?_⟩
  intro n hn hv
  have hn' : n = 0 ∨ n = 1 ∨ n = 2 := by omega
  rcases hn' with rfl | rfl | rfl
  · exact one_not_mem a b h2 (hv _ (isHomogeneous_one (Fin 3) K))
  · exact m_not_mem a b h2 (hv _ (isHomogeneous_X K 2))
  · exact mx_not_mem a b h2 (hv _ ((isHomogeneous_X K 2).mul (isHomogeneous_X K 0)))

end SecantQuadratic

#print axioms SecantQuadratic.quadratic_first_vanishing
#print axioms SecantQuadratic.mx_not_mem
