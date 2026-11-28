import Charts

namespace DiemChartAudit

variable {K : Type*} [Field K]

/-- A second genuine torus coordinate system, with distinct removed points. -/
def tilted (a : K) (ha : a ≠ 1) (u : Kˣ) : Line K :=
  Projectivization.mk K ((u : K)-1,(u : K)-a) (by
    intro h
    have hx : (u : K) = 1 := sub_eq_zero.mp (congrArg Prod.fst h)
    have hy : (u : K) = a := sub_eq_zero.mp (congrArg Prod.snd h)
    exact ha (hy.symm.trans hx))

def twoChart (a : K) (ha : a ≠ 1) (i : Fin 2) (u : Kˣ) : Line K :=
  if i = 0 then horizontal (u : K) else tilted a ha u

theorem tilted_hits_horizontal_zero (a : K) (ha0 : a ≠ 0) (ha1 : a ≠ 1) :
    tilted a ha1 (Units.mk0 a ha0) = horizontal 0 := by
  apply (Projectivization.mk_eq_mk_iff' K _ _ _ _).mpr
  refine ⟨a-1, ?_⟩
  apply Prod.ext <;> simp

theorem tilted_hits_vertical (a : K) (ha1 : a ≠ 1) :
    tilted a ha1 1 = vertical := by
  apply (Projectivization.mk_eq_mk_iff' K _ _ _ _).mpr
  refine ⟨1-a, ?_⟩
  apply Prod.ext <;> simp

/-- Exact repair to two charts when a third field element is available. -/
theorem two_chart_cover (a : K) (ha0 : a ≠ 0) (ha1 : a ≠ 1) (p : Line K) :
    ∃ i : Fin 2, ∃ u : Kˣ, twoChart a ha1 i u = p := by
  rcases line_normal_form p with ⟨z,hz⟩ | hv
  · by_cases h : z = 0
    · refine ⟨1,Units.mk0 a ha0,?_⟩
      rw [h] at hz
      simpa [twoChart] using (tilted_hits_horizontal_zero a ha0 ha1).trans hz
    · exact ⟨0,Units.mk0 z h,by simpa [twoChart] using hz⟩
  · exact ⟨1,1,by simpa [twoChart] using (tilted_hits_vertical a ha1).trans hv⟩

theorem two_chart_necessary [Finite K]
    (f : Fin 2 × Kˣ → Line K) (hf : Function.Surjective f) : 3 ≤ Nat.card K := by
  have hc := Nat.card_le_card_of_surjective f hf
  rw [line_card, Nat.card_prod, Nat.card_fin, Nat.card_units] at hc
  omega

theorem exists_third_element [Finite K] (hK : 3 ≤ Nat.card K) :
    ∃ a : K, a ≠ 0 ∧ a ≠ 1 := by
  classical
  let : Fintype K := Fintype.ofFinite K
  by_contra h
  push Not at h
  have hs : (Finset.univ : Finset K) ⊆ {0,1} := by
    intro a _
    simp only [Finset.mem_insert, Finset.mem_singleton]
    by_cases h0 : a=0
    · exact Or.inl h0
    · exact Or.inr (h a h0)
  have hc := Finset.card_le_card hs
  simp only [Finset.card_univ, Finset.card_pair (zero_ne_one : (0 : K) ≠ 1),
    Fintype.card_eq_nat_card] at hc
  omega

/-- The missing source hypothesis is exactly q≥3 for rational-point coverage. -/
theorem exists_two_chart_cover_iff [Finite K] :
    (∃ f : Fin 2 × Kˣ → Line K, Function.Surjective f) ↔ 3 ≤ Nat.card K := by
  constructor
  · rintro ⟨f,hf⟩
    exact two_chart_necessary f hf
  · intro h
    obtain ⟨a,ha0,ha1⟩ := exists_third_element h
    refine ⟨fun z => twoChart a ha1 z.1 z.2, ?_⟩
    intro p
    obtain ⟨i,u,hu⟩ := two_chart_cover a ha0 ha1 p
    exact ⟨(i,u),hu⟩

theorem tilted_injective (a : K) (ha : a ≠ 1) :
    Function.Injective (tilted a ha) := by
  intro u v h
  obtain ⟨c,hc⟩ := (Projectivization.mk_eq_mk_iff' K _ _ _ _).mp h
  have h0 : c*((v : K)-1) = (u : K)-1 := congrArg Prod.fst hc
  have h1 : c*((v : K)-a) = (u : K)-a := congrArg Prod.snd hc
  have he : c*(a-1) = a-1 := by linear_combination h0-h1
  have hc1 : c=1 := by
    apply (mul_right_cancel₀ (sub_ne_zero.mpr ha))
    simpa using he
  apply Units.ext
  rw [hc1] at h0
  simpa using h0.symm

theorem two_chart_injective (a : K) (ha : a ≠ 1) (i : Fin 2) :
    Function.Injective (twoChart a ha i) := by
  intro u v h
  by_cases hi : i=0
  · apply Units.ext
    exact horizontal_injective (by simpa [twoChart,hi] using h)
  · exact tilted_injective a ha (by simpa [twoChart,hi] using h)

theorem product_chart_count (n : ℕ) : Nat.card (Fin n → Fin 3) = 3^n := by
  simp

theorem product_three_chart_cover (n : ℕ) (p : Fin n → Line K) :
    ∃ i : Fin n → Fin 3, ∃ u : Fin n → Kˣ, ∀ j, threeChart (i j) (u j) = p j := by
  classical
  choose i u h using fun j => three_chart_cover (p j)
  exact ⟨i,u,h⟩

theorem product_two_chart_cover (a : K) (ha0 : a ≠ 0) (ha1 : a ≠ 1)
    (n : ℕ) (p : Fin n → Line K) :
    ∃ i : Fin n → Fin 2, ∃ u : Fin n → Kˣ,
      ∀ j, twoChart a ha1 (i j) (u j) = p j := by
  classical
  choose i u h using fun j => two_chart_cover a ha0 ha1 (p j)
  exact ⟨i,u,h⟩

/-- Over F2, even arbitrary maps from m actual n-tori require 3^n charts. -/
theorem binary_product_chart_lower_bound (n m : ℕ)
    (f : Fin m × (Fin n → (ZMod 2)ˣ) → (Fin n → Line (ZMod 2)))
    (hf : Function.Surjective f) : 3^n ≤ m := by
  have hc := Nat.card_le_card_of_surjective f hf
  simpa [Nat.card_fun, Nat.card_prod, binary_line_card, binary_torus_card] using hc

/-- The repaired number of calls is still bounded by a multiple of n!. -/
theorem chart_factorial_bound (n : ℕ) : 3^n ≤ 5*n.factorial := by
  induction n with
  | zero => norm_num
  | succ n ih =>
    by_cases hn : n < 2
    · interval_cases n <;> norm_num
    · calc
        3^(n+1) = 3*3^n := by ring
        _ ≤ 3*(5*n.factorial) := Nat.mul_le_mul_left _ ih
        _ ≤ 5*((n+1)*n.factorial) := by nlinarith
        _ = 5*(n+1).factorial := by rw [Nat.factorial_succ]

theorem chart_calls_times_bezout (n d : ℕ) (hd : 0 < d) :
    3^n * (n.factorial*d^n) ≤ 5*(n.factorial*d^n)^2 := by
  have hp : 1 ≤ d^n := Nat.one_le_pow n d hd
  have h := chart_factorial_bound n
  nlinarith [Nat.mul_le_mul_left (5*n.factorial) hp]

end DiemChartAudit
