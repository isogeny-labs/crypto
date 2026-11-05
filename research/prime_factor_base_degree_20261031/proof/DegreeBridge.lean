import RelationDegree
import Mathlib.RingTheory.MvPolynomial.Basic

/-! Canonical highest-total-degree forms: the component indices below are
    computed from the input polynomials, rather than supplied by the caller. -/
noncomputable section
open MvPolynomial
namespace DegreeBridge
variable {K : Type*} [Field K]

def top {σ : Type*} (p : MvPolynomial σ K) : MvPolynomial σ K :=
  homogeneousComponent p.totalDegree p

def topIdeal {σ : Type*} (generators : Set (MvPolynomial σ K)) :
    Ideal (MvPolynomial σ K) := Ideal.span (top '' generators)

lemma degree_le_of_above_zero {σ : Type*} (p : MvPolynomial σ K) (n : ℕ)
    (h : ∀ k, n < k → homogeneousComponent k p = 0) : p.totalDegree ≤ n := by
  classical
  change p.support.sup (fun e => e.degree) ≤ n
  apply Finset.sup_le
  intro e he
  by_contra! hn
  have hc := congrArg (coeff e) (h e.degree hn)
  simp only [coeff_homogeneousComponent, if_pos rfl, coeff_zero] at hc
  exact (MvPolynomial.mem_support_iff.mp he) hc

lemma degree_eq_of_components {σ : Type*} (p : MvPolynomial σ K) (n : ℕ)
    (h : ∀ k, n < k → homogeneousComponent k p = 0)
    (hne : homogeneousComponent n p ≠ 0) : p.totalDegree = n := by
  apply le_antisymm (degree_le_of_above_zero p n h)
  by_contra! hlt
  exact hne (homogeneousComponent_eq_zero n p hlt)

section Secant
local notation "P" => MvPolynomial (Fin 3) K
open SecantTopForms

def probe : P →+* K := eval₂Hom (RingHom.id K) (fun i => if i = 2 then 0 else 1)
@[simp] lemma probe_X (i : Fin 3) : probe (K := K) (X i) = if i = 2 then 0 else 1 :=
  eval₂Hom_X' _ _ _
@[simp] lemma probe_C (c : K) : probe (C c : P) = c := eval₂Hom_C _ _ _

theorem g1_degree (r : K) : (g1 r).totalDegree = 2 := by
  apply degree_eq_of_components
  · intro n hn
    have h0 : n ≠ 0 := by omega
    have h1 : n ≠ 1 := by omega
    have h2 : n ≠ 2 := by omega
    simp only [g1, (homogeneousComponent n : P →ₗ[K] P).map_sub,
      hc_pow, hc_X, hc_C]
    simp [h0, h1, h2]
  · rw [g1_component]
    simp [X_pow_eq_monomial]

theorem g2_degree (r s A : K) : (g2 r s A).totalDegree = 2 := by
  apply degree_eq_of_components
  · intro n hn
    have h0 : n ≠ 0 := by omega
    have h1 : n ≠ 1 := by omega
    have h2 : n ≠ 2 := by omega
    simp only [g2, (homogeneousComponent n : P →ₗ[K] P).map_sub,
      homogeneousComponent_C_mul, hc_product, hc_pow, hc_X, hc_C]
    simp [h0, h1, h2]
  · rw [g2_component]
    intro h
    have hp := congrArg (probe (K := K)) h
    simp [map_sub, map_mul, map_pow] at hp

theorem q_degree (i : Fin 3) (hi : i ≠ 2) (d : ℕ) (hd : 2 ≤ d)
    (a r s : K) (ha : a ≠ 0) (lower : P) (hl : lower.totalDegree < d) :
    (q i d a r s lower).totalDegree = d := by
  apply degree_eq_of_components
  · intro n hn
    have h0 : n ≠ 0 := by omega
    have h1 : n ≠ 1 := by omega
    have h2 : n ≠ 2 := by omega
    have hnd : n ≠ d := by omega
    rw [q, mul_sub]
    simp only [(homogeneousComponent n : P →ₗ[K] P).map_add,
      (homogeneousComponent n : P →ₗ[K] P).map_sub,
      mul_comm (X 2 : P) (C r), homogeneousComponent_C_mul, hc_pow,
      hc_product, hc_X, hc_C, homogeneousComponent_eq_zero n lower (by omega)]
    simp [h0, h1, h2, hnd]
  · have hv : probe (homogeneousComponent d (q i d a r s lower)) = a := by
      by_cases h : d = 2
      · subst d
        rw [q_component_two i a r s lower hl]
        simp [map_sub, map_mul, map_pow, hi]
      · rw [q_component i d (by omega) a r s lower hl]
        simp [map_mul, map_pow, hi]
    intro hzero
    have h := congrArg (probe (K := K)) hzero
    rw [hv, map_zero] at h
    exact ha h

def canonicalSecant (d : ℕ) (a b r s A : K) (lowerX lowerZ : P) : Ideal P :=
  topIdeal {g1 r, g2 r s A, q 0 d a r s lowerX, q 1 d b r s lowerZ}

theorem canonicalSecant_eq (d : ℕ) (hd : 2 ≤ d)
    (a b r s A : K) (ha : a ≠ 0) (hb : b ≠ 0)
    (lowerX lowerZ : P) (hX : lowerX.totalDegree < d) (hZ : lowerZ.totalDegree < d) :
    canonicalSecant d a b r s A lowerX lowerZ = leadingIdeal d a b r s A lowerX lowerZ := by
  simp only [canonicalSecant, topIdeal, Set.image_insert_eq, Set.image_singleton,
    top, g1_degree, g2_degree,
    q_degree 0 (by decide) d hd a r s ha lowerX hX,
    q_degree 1 (by decide) d hd b r s hb lowerZ hZ, leadingIdeal]

end Secant

section Dense
local notation "P" => MvPolynomial (Fin 2) K
open DenseTopForms

theorem s3_degree (A B r : K) : (s3 A B r).totalDegree = 4 := by
  apply degree_eq_of_components
  · intro n hn
    have h0 : n ≠ 0 := by omega
    have h1 : n ≠ 1 := by omega
    have h2 : n ≠ 2 := by omega
    have h3 : n ≠ 3 := by omega
    have h4 : n ≠ 4 := by omega
    have h31 : homogeneousComponent n ((X 0 : P)^2*X 1) = 0 := by
      simpa [h3] using hc_mixed (K := K) n 2 1 0 1
    have h32 : homogeneousComponent n ((X 0 : P)*X 1^2) = 0 := by
      simpa [h3] using hc_mixed (K := K) n 1 2 0 1
    have htwo : homogeneousComponent n ((X 0 : P)*X 1) = 0 := by
      simpa [h2] using hc_mixed (K := K) n 1 1 0 1
    have hX (i : Fin 2) : homogeneousComponent n ((X i : P)) = 0 := by
      simpa [h1] using hc_pow (K := K) n 1 i
    rw [s3_expansion]
    simp only [(homogeneousComponent n : P →ₗ[K] P).map_add,
      (homogeneousComponent n : P →ₗ[K] P).map_sub,
      homogeneousComponent_C_mul, hc_C, hc_pow, hc_mixed, h31, h32, htwo, hX]
    simp [h0, h1, h2, h4]
  · rw [s3_component]
    simp [X_pow_eq_monomial, monomial_mul]

theorem membership_degree (i : Fin 2) (b : ℕ) (lower : P) (hl : lower.totalDegree < b) :
    ((X i : P)^b+lower).totalDegree = b := by
  apply degree_eq_of_components
  · intro n hn
    have hnb : n ≠ b := by omega
    simp only [(homogeneousComponent n : P →ₗ[K] P).map_add, hc_pow,
      homogeneousComponent_eq_zero n lower (by omega)]
    simp [hnb]
  · rw [(homogeneousComponent b : P →ₗ[K] P).map_add,
      homogeneousComponent_eq_zero b lower hl, hc_pow]
    simp [X_pow_eq_monomial]

def canonicalDense (b : ℕ) (A B r : K) (lowerX lowerZ : P) : Ideal P :=
  topIdeal {X 0^b+lowerX, X 1^b+lowerZ, s3 A B r}

theorem canonicalDense_eq (b : ℕ) (A B r : K) (lowerX lowerZ : P)
    (hX : lowerX.totalDegree < b) (hZ : lowerZ.totalDegree < b) :
    canonicalDense b A B r lowerX lowerZ = leadingIdeal b A B r lowerX lowerZ := by
  simp only [canonicalDense, topIdeal, Set.image_insert_eq, Set.image_singleton, top,
    membership_degree 0 b lowerX hX, membership_degree 1 b lowerZ hZ,
    s3_degree, leadingIdeal]

end Dense

/-- The headline comparison uses each original polynomial's own total degree. -/
theorem canonical_strict_reduction (d : ℕ) (hd : 2 ≤ d)
    (a b r s A B : K) (ha : a ≠ 0) (hb : b ≠ 0) (h2 : (2 : K) ≠ 0)
    (graphX graphZ : MvPolynomial (Fin 3) K)
    (hgX : graphX.totalDegree < d) (hgZ : graphZ.totalDegree < d)
    (memberX memberZ : MvPolynomial (Fin 2) K)
    (hmX : memberX.totalDegree < 2*d) (hmZ : memberZ.totalDegree < 2*d) :
    SecantIdeal.FirstVanishing
      (canonicalSecant d a b r s A graphX graphZ) (d+1) ∧
    SecantIdeal.FirstVanishing
      (canonicalDense (2*d) A B r memberX memberZ) (2*d+1) ∧
    d+1 < 2*d+1 := by
  rw [canonicalSecant_eq d hd a b r s A ha hb graphX graphZ hgX hgZ,
    canonicalDense_eq (2*d) A B r memberX memberZ hmX hmZ]
  exact RelationDegree.strict_reduction d hd a b r s A B ha hb h2
    graphX graphZ hgX hgZ memberX memberZ hmX hmZ

end DegreeBridge

#print axioms DegreeBridge.g1_degree
#print axioms DegreeBridge.g2_degree
#print axioms DegreeBridge.q_degree
#print axioms DegreeBridge.s3_degree
#print axioms DegreeBridge.membership_degree
#print axioms DegreeBridge.canonical_strict_reduction
