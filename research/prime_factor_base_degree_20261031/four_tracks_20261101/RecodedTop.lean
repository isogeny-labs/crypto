import PencilTop

noncomputable section
open MvPolynomial Finsupp
namespace RecodedTop
set_option maxHeartbeats 300000
set_option maxRecDepth 10000
variable {K : Type*} [Field K]
local notation "P" => MvPolynomial (Fin 7) K

def ideal (a b : K) : Ideal P :=
  Ideal.span {X 2^2, X 0*X 1,
    X 0^2-C a*X 2*X 0-X 3*X 4,
    X 1^2-C b*X 2*X 1-X 5*X 6,
    X 3^2, X 4^2, X 5^2, X 6^2}

theorem ring_reductions {R : Type*} [CommRing R] (a b x z m u v s t : R)
    (hm : m^2 = 0) (hxz : x*z = 0)
    (hx : x^2-a*m*x-u*v = 0) (hz : z^2-b*m*z-s*t = 0)
    (hu : u^2 = 0) (hv : v^2 = 0) (hs : s^2 = 0) (ht : t^2 = 0) :
    x^5 = 0 ∧ z^5 = 0 ∧ m*x^4 = 0 ∧ m*z^4 = 0 ∧
    x^3*u = 0 ∧ x^3*v = 0 ∧ z^3*s = 0 ∧ z^3*t = 0 ∧
    m*x^2*u = 0 ∧ m*x^2*v = 0 ∧ m*z^2*s = 0 ∧ m*z^2*t = 0 ∧
    x*s*t = 0 ∧ z*u*v = 0 := by
  have hxu : x^2*u-a*m*x*u = 0 := by linear_combination u*hx+v*hu
  have hxv : x^2*v-a*m*x*v = 0 := by linear_combination v*hx+u*hv
  have hzs : z^2*s-b*m*z*s = 0 := by linear_combination s*hz+t*hs
  have hzt : z^2*t-b*m*z*t = 0 := by linear_combination t*hz+s*ht
  have hmxu : m*x^2*u = 0 := by linear_combination m*hxu+a*x*u*hm
  have hmxv : m*x^2*v = 0 := by linear_combination m*hxv+a*x*v*hm
  have hmzs : m*z^2*s = 0 := by linear_combination m*hzs+b*z*s*hm
  have hmzt : m*z^2*t = 0 := by linear_combination m*hzt+b*z*t*hm
  have hxx : m*x^3-m*x*u*v = 0 := by linear_combination m*x*hx+a*x^2*hm
  have hzz : m*z^3-m*z*s*t = 0 := by linear_combination m*z*hz+b*z^2*hm
  have hx4 : x^4-2*a*m*x^3 = 0 := by
    linear_combination (x^2+a*m*x+u*v)*hx+a^2*x^2*hm+v^2*hu-2*a*hxx
  have hz4 : z^4-2*b*m*z^3 = 0 := by
    linear_combination (z^2+b*m*z+s*t)*hz+b^2*z^2*hm+t^2*hs-2*b*hzz
  have hmx4 : m*x^4 = 0 := by linear_combination m*hx4+2*a*x^3*hm
  have hmz4 : m*z^4 = 0 := by linear_combination m*hz4+2*b*z^3*hm
  refine ⟨?_, ?_, hmx4, hmz4, ?_, ?_, ?_, ?_, hmxu, hmxv, hmzs, hmzt, ?_, ?_⟩
  · linear_combination x*hx4+2*a*hmx4
  · linear_combination z*hz4+2*b*hmz4
  · linear_combination x*hxu+a*hmxu
  · linear_combination x*hxv+a*hmxv
  · linear_combination z*hzs+b*hmzs
  · linear_combination z*hzt+b*hmzt
  · linear_combination -x*hz+(z-b*m)*hxz
  · linear_combination -z*hx+(x-a*m)*hxz

def monomials : Set P :=
  {X 2^2, X 0*X 1, X 3^2, X 4^2, X 5^2, X 6^2,
    X 0^5, X 1^5, X 2*X 0^4, X 2*X 1^4,
    X 0^3*X 3, X 0^3*X 4, X 1^3*X 5, X 1^3*X 6,
    X 2*X 0^2*X 3, X 2*X 0^2*X 4, X 2*X 1^2*X 5, X 2*X 1^2*X 6,
    X 0*X 5*X 6, X 1*X 3*X 4}

theorem monomials_mem (a b : K) : ∀ {p : P}, p ∈ monomials → p ∈ ideal a b := by
  let I := ideal a b
  let φ := Ideal.Quotient.mk I
  have gen (p : P) (hp : p ∈
      ({X 2^2, X 0*X 1, X 0^2-C a*X 2*X 0-X 3*X 4,
        X 1^2-C b*X 2*X 1-X 5*X 6, X 3^2, X 4^2, X 5^2, X 6^2} : Set P)) :
      φ p = 0 := Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.subset_span hp)
  have hm : (φ (X 2))^2 = 0 := by
    simpa only [_root_.map_pow] using gen (X 2^2) (Or.inl rfl)
  have hxz : φ (X 0)*φ (X 1) = 0 := by
    simpa only [_root_.map_mul] using gen (X 0*X 1) (Or.inr (Or.inl rfl))
  have hx : (φ (X 0))^2-φ (C a)*φ (X 2)*φ (X 0)-φ (X 3)*φ (X 4) = 0 := by
    simpa only [_root_.map_sub, _root_.map_mul, _root_.map_pow] using
      gen (X 0^2-C a*X 2*X 0-X 3*X 4) (Or.inr (Or.inr (Or.inl rfl)))
  have hz : (φ (X 1))^2-φ (C b)*φ (X 2)*φ (X 1)-φ (X 5)*φ (X 6) = 0 := by
    simpa only [_root_.map_sub, _root_.map_mul, _root_.map_pow] using
      gen (X 1^2-C b*X 2*X 1-X 5*X 6) (Or.inr (Or.inr (Or.inr (Or.inl rfl))))
  have hu : (φ (X 3))^2 = 0 := by
    simpa only [_root_.map_pow] using gen (X 3^2)
      (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl)))))
  have hv : (φ (X 4))^2 = 0 := by
    simpa only [_root_.map_pow] using gen (X 4^2)
      (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl))))))
  have hs : (φ (X 5))^2 = 0 := by
    simpa only [_root_.map_pow] using gen (X 5^2)
      (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl)))))))
  have ht : (φ (X 6))^2 = 0 := by
    simpa only [_root_.map_pow] using gen (X 6^2)
      (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr rfl)))))))
  obtain ⟨h1,h2,h3,h4,h5,h6,h7,h8,h9,h10,h11,h12,h13,h14⟩ :=
    ring_reductions (φ (C a)) (φ (C b)) (φ (X 0)) (φ (X 1)) (φ (X 2))
      (φ (X 3)) (φ (X 4)) (φ (X 5)) (φ (X 6)) hm hxz hx hz hu hv hs ht
  intro p hp
  rw [← Ideal.Quotient.eq_zero_iff_mem]
  simp only [monomials, Set.mem_insert_iff, Set.mem_singleton_iff] at hp
  rcases hp with rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl|rfl
  all_goals simp only [_root_.map_pow, _root_.map_mul]; assumption

private theorem finite_cover : ∀ (x z : Fin 5) (m u v s t : Fin 2),
    6 ≤ x.val+z.val+m.val+u.val+v.val+s.val+t.val →
      2 ≤ m.val ∨ (1 ≤ x.val ∧ 1 ≤ z.val) ∨ 2 ≤ u.val ∨ 2 ≤ v.val ∨ 2 ≤ s.val ∨ 2 ≤ t.val ∨
      5 ≤ x.val ∨ 5 ≤ z.val ∨ (1 ≤ m.val ∧ 4 ≤ x.val) ∨ (1 ≤ m.val ∧ 4 ≤ z.val) ∨
      (3 ≤ x.val ∧ 1 ≤ u.val) ∨ (3 ≤ x.val ∧ 1 ≤ v.val) ∨
      (3 ≤ z.val ∧ 1 ≤ s.val) ∨ (3 ≤ z.val ∧ 1 ≤ t.val) ∨
      (1 ≤ m.val ∧ 2 ≤ x.val ∧ 1 ≤ u.val) ∨ (1 ≤ m.val ∧ 2 ≤ x.val ∧ 1 ≤ v.val) ∨
      (1 ≤ m.val ∧ 2 ≤ z.val ∧ 1 ≤ s.val) ∨ (1 ≤ m.val ∧ 2 ≤ z.val ∧ 1 ≤ t.val) ∨
      (1 ≤ x.val ∧ 1 ≤ s.val ∧ 1 ≤ t.val) ∨ (1 ≤ z.val ∧ 1 ≤ u.val ∧ 1 ≤ v.val) := by
  intro x z m u v s t
  fin_cases x <;> fin_cases z <;> fin_cases m <;> fin_cases u <;>
    fin_cases v <;> fin_cases s <;> fin_cases t <;> decide

theorem monomial_high (a b : K) (e : Fin 7 →₀ ℕ) (he : 6 ≤ e.degree) (c : K) :
    monomial e c ∈ ideal a b := by
  have hdeg : e.degree = e 0+e 1+e 2+e 3+e 4+e 5+e 6 := by
    change e.sum (fun _ j => j) = _
    rw [Finsupp.sum_fintype]
    · simp [Fin.sum_univ_succ]
      omega
    · intro i
      rfl
  rw [hdeg] at he
  have hh :
      2 ≤ e 2 ∨ (1 ≤ e 0 ∧ 1 ≤ e 1) ∨ 2 ≤ e 3 ∨ 2 ≤ e 4 ∨ 2 ≤ e 5 ∨ 2 ≤ e 6 ∨
      5 ≤ e 0 ∨ 5 ≤ e 1 ∨ (1 ≤ e 2 ∧ 4 ≤ e 0) ∨ (1 ≤ e 2 ∧ 4 ≤ e 1) ∨
      (3 ≤ e 0 ∧ 1 ≤ e 3) ∨ (3 ≤ e 0 ∧ 1 ≤ e 4) ∨
      (3 ≤ e 1 ∧ 1 ≤ e 5) ∨ (3 ≤ e 1 ∧ 1 ≤ e 6) ∨
      (1 ≤ e 2 ∧ 2 ≤ e 0 ∧ 1 ≤ e 3) ∨ (1 ≤ e 2 ∧ 2 ≤ e 0 ∧ 1 ≤ e 4) ∨
      (1 ≤ e 2 ∧ 2 ≤ e 1 ∧ 1 ≤ e 5) ∨ (1 ≤ e 2 ∧ 2 ≤ e 1 ∧ 1 ≤ e 6) ∨
      (1 ≤ e 0 ∧ 1 ≤ e 5 ∧ 1 ≤ e 6) ∨ (1 ≤ e 1 ∧ 1 ≤ e 3 ∧ 1 ≤ e 4) := by
    by_cases hm : 2 ≤ e 2
    · exact Or.inl hm
    by_cases hxz : 1 ≤ e 0 ∧ 1 ≤ e 1
    · exact Or.inr (Or.inl hxz)
    by_cases hu : 2 ≤ e 3
    · exact Or.inr (Or.inr (Or.inl hu))
    by_cases hv : 2 ≤ e 4
    · exact Or.inr (Or.inr (Or.inr (Or.inl hv)))
    by_cases hs : 2 ≤ e 5
    · exact Or.inr (Or.inr (Or.inr (Or.inr (Or.inl hs))))
    by_cases ht : 2 ≤ e 6
    · exact Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl ht)))))
    by_cases hx5 : 5 ≤ e 0
    · exact Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl hx5))))))
    by_cases hz5 : 5 ≤ e 1
    · exact Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl hz5)))))))
    exact finite_cover ⟨e 0, by omega⟩ ⟨e 1, by omega⟩ ⟨e 2, by omega⟩
      ⟨e 3, by omega⟩ ⟨e 4, by omega⟩ ⟨e 5, by omega⟩ ⟨e 6, by omega⟩ he
  rcases hh with h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h|h
  · apply PencilTop.monomial_upward _ (show single 2 2 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 2^2 : P) ∈ monomials from Or.inl rfl)
  · apply PencilTop.monomial_upward _ (show single 0 1+single 1 1 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 0*X 1 : P) ∈ monomials from Or.inr (Or.inl rfl))
  · apply PencilTop.monomial_upward _ (show single 3 2 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 3^2 : P) ∈ monomials from Or.inr (Or.inr (Or.inl rfl)))
  · apply PencilTop.monomial_upward _ (show single 4 2 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 4^2 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inl rfl))))
  · apply PencilTop.monomial_upward _ (show single 5 2 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 5^2 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl)))))
  · apply PencilTop.monomial_upward _ (show single 6 2 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 6^2 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl))))))
  · apply PencilTop.monomial_upward _ (show single 0 5 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 0^5 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl)))))))
  · apply PencilTop.monomial_upward _ (show single 1 5 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 1^5 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl))))))))
  · apply PencilTop.monomial_upward _ (show single 2 1+single 0 4 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 2*X 0^4 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl)))))))))
  · apply PencilTop.monomial_upward _ (show single 2 1+single 1 4 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 2*X 1^4 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl))))))))))
  · apply PencilTop.monomial_upward _ (show single 0 3+single 3 1 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 0^3*X 3 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl)))))))))))
  · apply PencilTop.monomial_upward _ (show single 0 3+single 4 1 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 0^3*X 4 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl))))))))))))
  · apply PencilTop.monomial_upward _ (show single 1 3+single 5 1 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 1^3*X 5 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl)))))))))))))
  · apply PencilTop.monomial_upward _ (show single 1 3+single 6 1 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 1^3*X 6 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl))))))))))))))
  · apply PencilTop.monomial_upward _ (show single 2 1+single 0 2+single 3 1 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 2*X 0^2*X 3 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl)))))))))))))))
  · apply PencilTop.monomial_upward _ (show single 2 1+single 0 2+single 4 1 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 2*X 0^2*X 4 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl))))))))))))))))
  · apply PencilTop.monomial_upward _ (show single 2 1+single 1 2+single 5 1 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 2*X 1^2*X 5 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl)))))))))))))))))
  · apply PencilTop.monomial_upward _ (show single 2 1+single 1 2+single 6 1 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 2*X 1^2*X 6 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl))))))))))))))))))
  · apply PencilTop.monomial_upward _ (show single 0 1+single 5 1+single 6 1 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 0*X 5*X 6 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inl rfl)))))))))))))))))))
  · apply PencilTop.monomial_upward _ (show single 1 1+single 3 1+single 4 1 ≤ e from ?_) ?_ c
    · intro i
      fin_cases i <;> simp_all
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using
        monomials_mem a b (show (X 1*X 3*X 4 : P) ∈ monomials from Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (Or.inr (rfl))))))))))))))))))))

theorem vanishes (a b : K) : SecantIdeal.Vanishes (ideal a b) 6 := by
  classical
  intro p hp
  rw [p.as_sum]
  apply Ideal.sum_mem
  intro e he
  exact monomial_high a b e (by
    have := SecantIdeal.homogeneous_degree hp he
    omega) _

end RecodedTop
