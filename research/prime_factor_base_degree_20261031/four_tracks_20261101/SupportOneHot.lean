import Recoding
noncomputable section
open MvPolynomial Finsupp
namespace SupportOneHot
variable {K : Type*} [Field K]
set_option maxHeartbeats 300000

def OneHot (u : Fin 3 → K) : Prop := PencilSelector.Boolean u ∧
  u 0*u 1 = 0 ∧ u 0*u 2 = 0 ∧ u 1*u 2 = 0

theorem classification (u : Fin 3 → K) : OneHot u ↔
    u = ![0,0,0] ∨ u = ![1,0,0] ∨ u = ![0,1,0] ∨ u = ![0,0,1] := by
  constructor
  · rintro ⟨hb,h01,h02,h12⟩
    rcases hb 0 with h0|h0 <;> rcases hb 1 with h1|h1 <;> rcases hb 2 with h2|h2
    all_goals simp_all
    all_goals
      solve
      | left; funext i; fin_cases i <;> simp_all
      | right; left; funext i; fin_cases i <;> simp_all
      | right; right; left; funext i; fin_cases i <;> simp_all
      | right; right; right; funext i; fin_cases i <;> simp_all
  · rintro (rfl|rfl|rfl|rfl) <;>
      simp [OneHot, PencilSelector.Boolean, Fin.forall_fin_succ]

def offset (u : Fin 3 → K) := 2*u 0+10*u 1+12*u 2
theorem offset_image (t : K) : (∃ u, OneHot u ∧ t = offset u) ↔
    t = 0 ∨ t = 2 ∨ t = 10 ∨ t = 12 := by
  simp only [classification]
  simp [offset, exists_or, and_or_left, or_and_right]

local notation "P" => PencilTop.P K 6
def extra : Set P :=
  {X (Sum.inr 0)*X (Sum.inr 1), X (Sum.inr 0)*X (Sum.inr 2),
    X (Sum.inr 1)*X (Sum.inr 2), X (Sum.inr 3)*X (Sum.inr 4),
    X (Sum.inr 3)*X (Sum.inr 5), X (Sum.inr 4)*X (Sum.inr 5)}
def ideal (a b : K) : Ideal P := PencilTop.ideal 6 a b ⊔ Ideal.span extra

theorem extra_mem {p : P} (a b : K) (h : p ∈ extra) : p ∈ ideal a b :=
  (show Ideal.span extra ≤ ideal a b from le_sup_right) (Ideal.subset_span h)

theorem two_single_le {σ : Type*} {i j : σ} (hij : i ≠ j)
    {e : σ →₀ ℕ} {a b : ℕ} (hi : a ≤ e i) (hj : b ≤ e j) :
    single i a+single j b ≤ e := by
  classical
  intro k
  by_cases hki : i = k
  · subst k
    simpa only [Finsupp.add_apply, Finsupp.single_eq_same,
      Finsupp.single_eq_of_ne hij.symm, add_zero] using hi
  · by_cases hkj : j = k
    · subst k
      simpa only [Finsupp.add_apply, Finsupp.single_eq_same,
        Finsupp.single_eq_of_ne hij, zero_add] using hj
    · simp only [Finsupp.add_apply, Finsupp.single_eq_of_ne hki,
        Finsupp.single_eq_of_ne hkj, zero_add, zero_le]

theorem high_monomials (a b : K) (e : PencilTop.Vars 6 →₀ ℕ)
    (he : 5 ≤ e.degree) (c : K) : monomial e c ∈ ideal a b := by
  classical
  have inc : PencilTop.ideal (K := K) 6 a b ≤ ideal a b := le_sup_left
  by_cases hbits : ∃ i : Fin 6, 2 ≤ e (Sum.inr i)
  · obtain ⟨i, hi⟩ := hbits
    apply PencilTop.monomial_upward _ (single_le_iff.mpr hi) _ c
    have hh : (X (Sum.inr i) : P)^2 ∈ PencilTop.ideal 6 a b :=
      Ideal.subset_span (by simp [PencilTop.ideal])
    simpa [← X_pow_eq_monomial] using inc hh
  have hb (i : Fin 6) : e (Sum.inr i) ≤ 1 := by
    have := not_exists.mp hbits i
    omega
  by_cases hp :
      (1 ≤ e (Sum.inr 0) ∧ 1 ≤ e (Sum.inr 1)) ∨
      (1 ≤ e (Sum.inr 0) ∧ 1 ≤ e (Sum.inr 2)) ∨
      (1 ≤ e (Sum.inr 1) ∧ 1 ≤ e (Sum.inr 2)) ∨
      (1 ≤ e (Sum.inr 3) ∧ 1 ≤ e (Sum.inr 4)) ∨
      (1 ≤ e (Sum.inr 3) ∧ 1 ≤ e (Sum.inr 5)) ∨
      (1 ≤ e (Sum.inr 4) ∧ 1 ≤ e (Sum.inr 5))
  · rcases hp with hp|hp|hp|hp|hp|hp
    · apply PencilTop.monomial_upward _ (show single (Sum.inr 0) 1+single (Sum.inr 1) 1 ≤ e from ?_) ?_ c
      · exact two_single_le (by decide) hp.1 hp.2
      · simpa [monomial_add_single, ← X_pow_eq_monomial] using extra_mem a b
          (by simp [extra] : (X (Sum.inr 0)*X (Sum.inr 1) : P) ∈ extra)
    · apply PencilTop.monomial_upward _ (show single (Sum.inr 0) 1+single (Sum.inr 2) 1 ≤ e from ?_) ?_ c
      · exact two_single_le (by decide) hp.1 hp.2
      · simpa [monomial_add_single, ← X_pow_eq_monomial] using extra_mem a b
          (by simp [extra] : (X (Sum.inr 0)*X (Sum.inr 2) : P) ∈ extra)
    · apply PencilTop.monomial_upward _ (show single (Sum.inr 1) 1+single (Sum.inr 2) 1 ≤ e from ?_) ?_ c
      · exact two_single_le (by decide) hp.1 hp.2
      · simpa [monomial_add_single, ← X_pow_eq_monomial] using extra_mem a b
          (by simp [extra] : (X (Sum.inr 1)*X (Sum.inr 2) : P) ∈ extra)
    · apply PencilTop.monomial_upward _ (show single (Sum.inr 3) 1+single (Sum.inr 4) 1 ≤ e from ?_) ?_ c
      · exact two_single_le (by decide) hp.1 hp.2
      · simpa [monomial_add_single, ← X_pow_eq_monomial] using extra_mem a b
          (by simp [extra] : (X (Sum.inr 3)*X (Sum.inr 4) : P) ∈ extra)
    · apply PencilTop.monomial_upward _ (show single (Sum.inr 3) 1+single (Sum.inr 5) 1 ≤ e from ?_) ?_ c
      · exact two_single_le (by decide) hp.1 hp.2
      · simpa [monomial_add_single, ← X_pow_eq_monomial] using extra_mem a b
          (by simp [extra] : (X (Sum.inr 3)*X (Sum.inr 5) : P) ∈ extra)
    · apply PencilTop.monomial_upward _ (show single (Sum.inr 4) 1+single (Sum.inr 5) 1 ≤ e from ?_) ?_ c
      · exact two_single_le (by decide) hp.1 hp.2
      · simpa [monomial_add_single, ← X_pow_eq_monomial] using extra_mem a b
          (by simp [extra] : (X (Sum.inr 4)*X (Sum.inr 5) : P) ∈ extra)
  have hs : (∑ i : Fin 6, e (Sum.inr i)) ≤ 2 := by
    have h0 := hb 0; have h1 := hb 1; have h2 := hb 2
    have h3 := hb 3; have h4 := hb 4; have h5 := hb 5
    simp [Fin.sum_univ_succ]
    omega
  rw [PencilTop.degree_vars] at he
  have hcore : 3 ≤ e (Sum.inl 0)+e (Sum.inl 1)+e (Sum.inl 2) := by omega
  obtain ⟨hm,hxz,hx3,hz3,hxm,hzm⟩ := PencilTop.core_members 6 a b
  by_cases hm' : 2 ≤ e (Sum.inl 2)
  · apply PencilTop.monomial_upward _ (single_le_iff.mpr hm') _ c
    simpa [← X_pow_eq_monomial] using inc hm
  by_cases hxz' : 1 ≤ e (Sum.inl 0) ∧ 1 ≤ e (Sum.inl 1)
  · apply PencilTop.monomial_upward _
      (show single (Sum.inl 0) 1+single (Sum.inl 1) 1 ≤ e from ?_) ?_ c
    · exact two_single_le (by decide) hxz'.1 hxz'.2
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using inc hxz
  by_cases hx3' : 3 ≤ e (Sum.inl 0)
  · apply PencilTop.monomial_upward _ (single_le_iff.mpr hx3') _ c
    simpa [← X_pow_eq_monomial] using inc hx3
  by_cases hz3' : 3 ≤ e (Sum.inl 1)
  · apply PencilTop.monomial_upward _ (single_le_iff.mpr hz3') _ c
    simpa [← X_pow_eq_monomial] using inc hz3
  have hh : (2 ≤ e (Sum.inl 0) ∧ 1 ≤ e (Sum.inl 2)) ∨
      (2 ≤ e (Sum.inl 1) ∧ 1 ≤ e (Sum.inl 2)) := by omega
  rcases hh with hh|hh
  · apply PencilTop.monomial_upward _
      (show single (Sum.inl 0) 2+single (Sum.inl 2) 1 ≤ e from ?_) ?_ c
    · exact two_single_le (by decide) hh.1 hh.2
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using inc hxm
  · apply PencilTop.monomial_upward _
      (show single (Sum.inl 1) 2+single (Sum.inl 2) 1 ≤ e from ?_) ?_ c
    · exact two_single_le (by decide) hh.1 hh.2
    · simpa [monomial_add_single, ← X_pow_eq_monomial] using inc hzm

theorem vanishes (a b : K) : SecantIdeal.Vanishes (ideal a b) 5 := by
  classical
  intro p hp
  rw [p.as_sum]
  apply Ideal.sum_mem
  intro e he
  apply high_monomials a b e _ _
  have := SecantIdeal.homogeneous_degree hp he
  omega

local notation "Q" => PencilTop.P K 2
def reduction : P →+* Q := eval₂Hom C (Sum.elim
  (fun i : Fin 3 => X (Sum.inl i))
  (fun i : Fin 6 => if i = 0 then X (Sum.inr 0) else
    if i = 3 then X (Sum.inr 1) else 0))
@[simp] theorem reduction_C (c : K) : reduction (C c) = (C c : Q) := eval₂Hom_C _ _ _
@[simp] theorem reduction_core (i : Fin 3) :
    reduction (X (Sum.inl i)) = (X (Sum.inl i) : Q) := eval₂Hom_X' _ _ _
@[simp] theorem reduction_bit (i : Fin 6) :
    reduction (X (Sum.inr i)) = (if i = 0 then X (Sum.inr 0) else
      if i = 3 then X (Sum.inr 1) else 0 : Q) := eval₂Hom_X' _ _ _

theorem reduction_ideal (a b : K) : ideal a b ≤
    Ideal.comap reduction (PencilTop.ideal 2 a b) := by
  apply sup_le
  · apply Ideal.span_le.mpr
    intro p hp
    change reduction p ∈ PencilTop.ideal 2 a b
    rcases hp with hp|hp
    · simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hp
      rcases hp with rfl|rfl|rfl|rfl
      all_goals
        simp only [_root_.map_pow, _root_.map_mul, _root_.map_sub,
          reduction_C, reduction_core]
      · exact Ideal.subset_span (Or.inl (Or.inl rfl))
      · exact Ideal.subset_span (Or.inl (Or.inr (Or.inl rfl)))
      · exact Ideal.subset_span (Or.inl (Or.inr (Or.inr (Or.inl rfl))))
      · exact Ideal.subset_span (Or.inl (Or.inr (Or.inr (Or.inr rfl))))
    · obtain ⟨i,rfl⟩ := hp
      by_cases hi : i = 0
      · subst i
        simp only [_root_.map_pow, reduction_bit, if_pos rfl]
        exact Ideal.subset_span (Or.inr ⟨0,rfl⟩)
      · by_cases hj : i = 3
        · subst i
          simp only [_root_.map_pow, reduction_bit, if_neg (show (3 : Fin 6) ≠ 0 by decide), if_pos rfl]
          exact Ideal.subset_span (Or.inr ⟨1,rfl⟩)
        · simp only [_root_.map_pow, reduction_bit, if_neg hi, if_neg hj,
            zero_pow (show (2 : ℕ) ≠ 0 by decide)]
          exact Ideal.zero_mem _
  · apply Ideal.span_le.mpr
    intro p hp
    change reduction p ∈ PencilTop.ideal 2 a b
    simp only [extra, Set.mem_insert_iff, Set.mem_singleton_iff] at hp
    rcases hp with rfl|rfl|rfl|rfl|rfl|rfl
    all_goals
      simp only [_root_.map_mul, reduction_bit, if_pos rfl, show (1 : Fin 6) ≠ 0 by decide, show (2 : Fin 6) ≠ 0 by decide, show (3 : Fin 6) ≠ 0 by decide, show (4 : Fin 6) ≠ 0 by decide, show (5 : Fin 6) ≠ 0 by decide, show (1 : Fin 6) ≠ 3 by decide, show (2 : Fin 6) ≠ 3 by decide, show (4 : Fin 6) ≠ 3 by decide, show (5 : Fin 6) ≠ 3 by decide, ↓reduceIte, zero_mul, mul_zero]
      exact Ideal.zero_mem _

def witness : P := X (Sum.inl 2)*X (Sum.inl 0)*X (Sum.inr 0)*X (Sum.inr 3)
theorem witness_homogeneous : (witness : P).IsHomogeneous 4 := by
  exact (((isHomogeneous_X K _).mul (isHomogeneous_X K _)).mul
    (isHomogeneous_X K _)).mul (isHomogeneous_X K _)
theorem witness_not_mem (a b : K) (h2 : (2 : K) ≠ 0) : witness ∉ ideal a b := by
  intro h
  have hh := reduction_ideal a b h
  change reduction witness ∈ PencilTop.ideal 2 a b at hh
  have heq : reduction witness = (PencilTop.highWitness 2 : Q) := by
    simp only [witness, _root_.map_mul, reduction_core, reduction_bit, if_pos rfl,
      if_neg (show (3 : Fin 6) ≠ 0 by decide), PencilTop.highWitness, Fin.prod_univ_two,
      ↓reduceIte]
    ring
  rw [heq] at hh
  exact PencilTop.highWitness_not_mem 2 a b h2 hh

theorem first_vanishing (a b : K) (h2 : (2 : K) ≠ 0) :
    SecantIdeal.FirstVanishing (ideal a b) 5 := by
  refine ⟨vanishes a b, ?_⟩
  intro d hd hv
  have hh := PencilTop.vanishes_mono (ideal a b) hv (show d ≤ 4 by omega)
  exact witness_not_mem a b h2 (hh _ witness_homogeneous)

theorem pair_top (i j : PencilTop.Vars 6) :
    DegreeBridge.top ((X i : P)*X j) = X i*X j := by
  have hh := (isHomogeneous_X K i).mul (isHomogeneous_X K j)
  unfold DegreeBridge.top
  rw [hh.totalDegree (mul_ne_zero (X_ne_zero i) (X_ne_zero j))]
  simpa using homogeneousComponent_of_mem hh

theorem extra_top {p : P} (h : p ∈ extra) : DegreeBridge.top p = p := by
  simp only [extra, Set.mem_insert_iff, Set.mem_singleton_iff] at h
  rcases h with rfl|rfl|rfl|rfl|rfl|rfl <;> apply pair_top

def generators (r s A a b cx cz dx dz : K) (vx vz : Fin 6 → K) : Set P :=
  PencilCanonical.generators 6 r s A a b cx cz dx dz vx vz ∪ extra

theorem canonical_ideal (r s A a b cx cz dx dz : K)
    (ha : a ≠ 0) (hb : b ≠ 0) (vx vz : Fin 6 → K) :
    DegreeBridge.topIdeal (generators r s A a b cx cz dx dz vx vz) = ideal a⁻¹ b⁻¹ := by
  have hex : DegreeBridge.top '' (extra : Set P) = extra := by
    ext p
    constructor
    · rintro ⟨q,hq,rfl⟩
      simpa only [extra_top hq] using hq
    · intro hp
      exact ⟨p,hp,extra_top hp⟩
  unfold generators DegreeBridge.topIdeal
  rw [Set.image_union, Ideal.span_union, hex]
  change DegreeBridge.topIdeal (PencilCanonical.generators 6 r s A a b cx cz dx dz vx vz) ⊔
    Ideal.span extra = ideal a⁻¹ b⁻¹
  rw [PencilCanonical.canonical_top_ideal 6 r s A a b cx cz dx dz ha hb vx vz]
  rfl

theorem canonical_first_vanishing (r s A a b cx cz dx dz : K)
    (ha : a ≠ 0) (hb : b ≠ 0) (h2 : (2 : K) ≠ 0) (vx vz : Fin 6 → K) :
    SecantIdeal.FirstVanishing
      (DegreeBridge.topIdeal (generators r s A a b cx cz dx dz vx vz)) 5 := by
  rw [canonical_ideal r s A a b cx cz dx dz ha hb vx vz]
  exact first_vanishing a⁻¹ b⁻¹ h2

end SupportOneHot

#print axioms SupportOneHot.classification
#print axioms SupportOneHot.offset_image
#print axioms SupportOneHot.vanishes
#print axioms SupportOneHot.first_vanishing
#print axioms SupportOneHot.canonical_first_vanishing
