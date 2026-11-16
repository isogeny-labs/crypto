import SecantIdeal
import SecantQuadratic
import Mathlib.Tactic.Ring

noncomputable section
open MvPolynomial
namespace SecantTopForms
variable {K : Type*} [Field K]
local notation "P" => MvPolynomial (Fin 3) K

def g1 (r : K) : P := X 2 ^ 2 - X 0 - X 1 - C r
def g2 (r s A : K) : P :=
  X 0 * X 1 - C (r^2) - C r * X 2 ^ 2 - C A - C (2*s) * X 2
def q (i : Fin 3) (d : ℕ) (a r s : K) (lower : P) : P :=
  C a * X i ^ d + lower - X 2 * (X i - C r) + C s

lemma hc_C (n : ℕ) (a : K) :
    homogeneousComponent n (C a : P) = if n = 0 then C a else 0 :=
  homogeneousComponent_of_mem (isHomogeneous_C (Fin 3) a)

lemma hc_X (n : ℕ) (i : Fin 3) :
    homogeneousComponent n (X i : P) = if n = 1 then X i else 0 :=
  homogeneousComponent_of_mem (isHomogeneous_X K i)

lemma hc_pow (n d : ℕ) (i : Fin 3) :
    homogeneousComponent n ((X i : P)^d) = if n = d then X i^d else 0 :=
  homogeneousComponent_of_mem (isHomogeneous_X_pow i d)

lemma hc_product (n : ℕ) (i j : Fin 3) :
    homogeneousComponent n ((X i : P)*X j) = if n = 2 then X i*X j else 0 :=
  homogeneousComponent_of_mem ((isHomogeneous_X K i).mul (isHomogeneous_X K j))

theorem g1_component (r : K) : homogeneousComponent 2 (g1 r) = (X 2 : P)^2 := by
  simp only [g1, (homogeneousComponent 2 : P →ₗ[K] P).map_sub, hc_pow, hc_X, hc_C]
  norm_num

theorem g2_component (r s A : K) :
    homogeneousComponent 2 (g2 r s A) = (X 0 : P)*X 1 - C r * X 2^2 := by
  simp only [g2, (homogeneousComponent 2 : P →ₗ[K] P).map_sub,
    homogeneousComponent_C_mul, hc_product, hc_pow, hc_X, hc_C]
  norm_num

theorem q_component (i : Fin 3) (d : ℕ) (hd : 3 ≤ d)
    (a r s : K) (lower : P) (hlower : lower.totalDegree < d) :
    homogeneousComponent d (q i d a r s lower) = C a * X i ^ d := by
  have h0 : d ≠ 0 := by omega
  have h1 : d ≠ 1 := by omega
  have h2 : d ≠ 2 := by omega
  rw [q, mul_sub]
  simp [map_add, map_sub, homogeneousComponent_eq_zero d lower hlower,
    hc_pow, hc_product, hc_X, hc_C, h0, h1, h2, mul_comm (X 2 : P) (C r)]

theorem q_component_two (i : Fin 3) (a r s : K) (lower : P)
    (hlower : lower.totalDegree < 2) :
    homogeneousComponent 2 (q i 2 a r s lower) = C a*X i^2-X 2*X i := by
  rw [q, mul_sub]
  simp [map_add, map_sub, homogeneousComponent_eq_zero 2 lower hlower,
    hc_pow, hc_product, hc_X, hc_C, mul_comm (X 2 : P) (C r)]

/-- The actual selected highest homogeneous components, not an exponent predicate. -/
def leadingIdeal (d : ℕ) (a b r s A : K) (lowerX lowerZ : P) : Ideal P :=
  Ideal.span {homogeneousComponent 2 (g1 r), homogeneousComponent 2 (g2 r s A),
    homogeneousComponent d (q 0 d a r s lowerX),
    homogeneousComponent d (q 1 d b r s lowerZ)}

theorem secant_eq_span (d : ℕ) :
    SecantIdeal.secant (K := K) d =
      Ideal.span {(X 0 : P)^d, X 1^d, X 0*X 1, X 2^2} := by
  simp [SecantIdeal.secant, SecantIdeal.secantExponents,
    Set.image_insert_eq, Set.image_singleton, ← X_pow_eq_monomial,
    monomial_add_single, ← X_pow_eq_monomial]

/-- Nonzero leading coefficients suffice; no genericity or target selection. -/
theorem leadingIdeal_eq_secant (d : ℕ) (hd : 3 ≤ d)
    (a b r s A : K) (ha : a ≠ 0) (hb : b ≠ 0)
    (lowerX lowerZ : P) (hX : lowerX.totalDegree < d) (hZ : lowerZ.totalDegree < d) :
    leadingIdeal d a b r s A lowerX lowerZ = SecantIdeal.secant d := by
  rw [leadingIdeal, g1_component, g2_component,
    q_component 0 d hd a r s lowerX hX, q_component 1 d hd b r s lowerZ hZ,
    secant_eq_span]
  let I : Ideal P := Ideal.span {X 2^2, X 0*X 1-C r*X 2^2,
    C a*X 0^d, C b*X 1^d}
  let J : Ideal P := Ideal.span {X 0^d, X 1^d, X 0*X 1, X 2^2}
  change I = J
  have hi1 : X 2^2 ∈ I := Ideal.subset_span (by simp)
  have hi2 : X 0*X 1-C r*X 2^2 ∈ I := Ideal.subset_span (by simp)
  have hi3 : C a*X 0^d ∈ I := Ideal.subset_span (by simp)
  have hi4 : C b*X 1^d ∈ I := Ideal.subset_span (by simp)
  have hx : X 0^d ∈ I := by
    have h := I.mul_mem_left (C a⁻¹) hi3
    simpa [← mul_assoc, ← map_mul, inv_mul_cancel₀ ha] using h
  have hz : X 1^d ∈ I := by
    have h := I.mul_mem_left (C b⁻¹) hi4
    simpa [← mul_assoc, ← map_mul, inv_mul_cancel₀ hb] using h
  have hxz : X 0*X 1 ∈ I := by
    simpa using I.add_mem hi2 (I.mul_mem_left (C r) hi1)
  apply le_antisymm
  · apply Ideal.span_le.mpr
    intro p hp
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hp
    have hj1 : X 2^2 ∈ J := Ideal.subset_span (by simp)
    have hj2 : X 0*X 1 ∈ J := Ideal.subset_span (by simp)
    have hj3 : X 0^d ∈ J := Ideal.subset_span (by simp)
    have hj4 : X 1^d ∈ J := Ideal.subset_span (by simp)
    rcases hp with rfl | rfl | rfl | rfl
    · exact hj1
    · exact J.sub_mem hj2 (J.mul_mem_left (C r) hj1)
    · exact J.mul_mem_left (C a) hj3
    · exact J.mul_mem_left (C b) hj4
  · apply Ideal.span_le.mpr
    intro p hp
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hp
    rcases hp with rfl | rfl | rfl | rfl
    · exact hx
    · exact hz
    · exact hxz
    · exact hi1

/-- First vanishing homogeneous degree of the displayed secant equations. -/
theorem secant_equations_first_vanishing (d : ℕ) (hd : 3 ≤ d)
    (a b r s A : K) (ha : a ≠ 0) (hb : b ≠ 0)
    (lowerX lowerZ : P) (hX : lowerX.totalDegree < d) (hZ : lowerZ.totalDegree < d) :
    SecantIdeal.FirstVanishing (leadingIdeal d a b r s A lowerX lowerZ) (d+1) := by
  rw [leadingIdeal_eq_secant d hd a b r s A ha hb lowerX lowerZ hX hZ]
  exact SecantIdeal.secant_first_vanishing d (by omega)

lemma normalize_relation (a : K) (ha : a ≠ 0) (x m : P) :
    C a⁻¹*(C a*x^2-m*x) = x^2-C a⁻¹*m*x := by
  calc
    _ = (C a⁻¹*C a)*x^2-C a⁻¹*m*x := by ring
    _ = _ := by rw [← map_mul, inv_mul_cancel₀ ha, map_one, one_mul]

lemma denormalize_relation (a : K) (ha : a ≠ 0) (x m : P) :
    C a*(x^2-C a⁻¹*m*x) = C a*x^2-m*x := by
  calc
    _ = C a*x^2-(C a*C a⁻¹)*m*x := by ring
    _ = _ := by rw [← map_mul, mul_inv_cancel₀ ha, map_one, one_mul]

theorem leadingIdeal_two (a b r s A : K) (ha : a ≠ 0) (hb : b ≠ 0)
    (lowerX lowerZ : P) (hX : lowerX.totalDegree < 2) (hZ : lowerZ.totalDegree < 2) :
    leadingIdeal 2 a b r s A lowerX lowerZ = SecantQuadratic.quadratic a⁻¹ b⁻¹ := by
  rw [leadingIdeal, g1_component, g2_component,
    q_component_two 0 a r s lowerX hX, q_component_two 1 b r s lowerZ hZ]
  let I : Ideal P := Ideal.span {X 2^2, X 0*X 1-C r*X 2^2,
    C a*X 0^2-X 2*X 0, C b*X 1^2-X 2*X 1}
  let J := SecantQuadratic.quadratic a⁻¹ b⁻¹
  change I = J
  have hi1 : X 2^2 ∈ I := Ideal.subset_span (by simp)
  have hi2 : X 0*X 1-C r*X 2^2 ∈ I := Ideal.subset_span (by simp)
  have hi3 : C a*X 0^2-X 2*X 0 ∈ I := Ideal.subset_span (by simp)
  have hi4 : C b*X 1^2-X 2*X 1 ∈ I := Ideal.subset_span (by simp)
  have hx : X 0^2-C a⁻¹*X 2*X 0 ∈ I := by
    rw [← normalize_relation a ha]
    exact I.mul_mem_left (C a⁻¹) hi3
  have hz : X 1^2-C b⁻¹*X 2*X 1 ∈ I := by
    rw [← normalize_relation b hb]
    exact I.mul_mem_left (C b⁻¹) hi4
  have hxz : X 0*X 1 ∈ I := by
    simpa using I.add_mem hi2 (I.mul_mem_left (C r) hi1)
  have hj1 : X 2^2 ∈ J := Ideal.subset_span (by simp [J, SecantQuadratic.quadratic])
  have hj2 : X 0*X 1 ∈ J := Ideal.subset_span (by simp [J, SecantQuadratic.quadratic])
  have hj3 : X 0^2-C a⁻¹*X 2*X 0 ∈ J :=
    Ideal.subset_span (by simp [J, SecantQuadratic.quadratic])
  have hj4 : X 1^2-C b⁻¹*X 2*X 1 ∈ J :=
    Ideal.subset_span (by simp [J, SecantQuadratic.quadratic])
  apply le_antisymm
  · apply Ideal.span_le.mpr
    intro p hp
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hp
    rcases hp with rfl | rfl | rfl | rfl
    · exact hj1
    · exact J.sub_mem hj2 (J.mul_mem_left (C r) hj1)
    · rw [← denormalize_relation a ha]
      exact J.mul_mem_left (C a) hj3
    · rw [← denormalize_relation b hb]
      exact J.mul_mem_left (C b) hj4
  · apply Ideal.span_le.mpr
    intro p hp
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hp
    rcases hp with rfl | rfl | rfl | rfl
    · exact hi1
    · exact hxz
    · exact hx
    · exact hz

/-- Includes the actual quadratic branch, with no monomial substitution for it. -/
theorem all_secant_equations_first_vanishing (d : ℕ) (hd : 2 ≤ d)
    (a b r s A : K) (ha : a ≠ 0) (hb : b ≠ 0) (h2 : (2 : K) ≠ 0)
    (lowerX lowerZ : P) (hX : lowerX.totalDegree < d) (hZ : lowerZ.totalDegree < d) :
    SecantIdeal.FirstVanishing (leadingIdeal d a b r s A lowerX lowerZ) (d+1) := by
  by_cases h : d = 2
  · subst d
    rw [leadingIdeal_two a b r s A ha hb lowerX lowerZ hX hZ]
    exact SecantQuadratic.quadratic_first_vanishing a⁻¹ b⁻¹ h2
  · exact secant_equations_first_vanishing d (by omega) a b r s A ha hb
      lowerX lowerZ hX hZ

end SecantTopForms

#print axioms SecantTopForms.leadingIdeal_eq_secant
#print axioms SecantTopForms.secant_equations_first_vanishing
#print axioms SecantTopForms.leadingIdeal_two
#print axioms SecantTopForms.all_secant_equations_first_vanishing
