import SecantIdeal
import Mathlib.Tactic.Ring

noncomputable section
open MvPolynomial
namespace DenseTopForms
variable {K : Type*} [Field K]
local notation "P" => MvPolynomial (Fin 2) K

/-- The ordinary short-Weierstrass third summation polynomial at fixed r. -/
def s3 (A B r : K) : P :=
  (X 0-X 1)^2*C (r^2) -
    C (2*r)*((X 0+X 1)*(X 0*X 1+C A)+C (2*B)) +
    (X 0*X 1-C A)^2 - C (4*B)*(X 0+X 1)

theorem s3_expansion (A B r : K) :
    s3 A B r =
      X 0^2*X 1^2 - C (2*r)*(X 0^2*X 1) - C (2*r)*(X 0*X 1^2) +
      C (r^2)*X 0^2 + C (r^2)*X 1^2 - C (2*r^2+2*A)*(X 0*X 1) -
      C (2*A*r+4*B)*X 0 - C (2*A*r+4*B)*X 1 + C (A^2-4*B*r) := by
  simp only [s3, map_add, map_sub, map_mul, map_pow, map_ofNat]
  ring

lemma hc_C (n : ℕ) (a : K) :
    homogeneousComponent n (C a : P) = if n = 0 then C a else 0 :=
  homogeneousComponent_of_mem (isHomogeneous_C (Fin 2) a)

lemma hc_pow (n d : ℕ) (i : Fin 2) :
    homogeneousComponent n ((X i : P)^d) = if n = d then X i^d else 0 :=
  homogeneousComponent_of_mem (isHomogeneous_X_pow i d)

lemma hc_mixed (n a b : ℕ) (i j : Fin 2) :
    homogeneousComponent n ((X i : P)^a*X j^b) =
      if n = a+b then X i^a*X j^b else 0 :=
  homogeneousComponent_of_mem ((isHomogeneous_X_pow i a).mul (isHomogeneous_X_pow j b))

theorem s3_component (A B r : K) :
    homogeneousComponent 4 (s3 A B r) = (X 0 : P)^2*X 1^2 := by
  rw [s3_expansion]
  have h31 : homogeneousComponent 4 ((X 0 : P)^2*X 1) = 0 := by
    simpa using hc_mixed (K := K) 4 2 1 0 1
  have h32 : homogeneousComponent 4 ((X 0 : P)*X 1^2) = 0 := by
    simpa using hc_mixed (K := K) 4 1 2 0 1
  have h2 : homogeneousComponent 4 ((X 0 : P)*X 1) = 0 := by
    simpa using hc_mixed (K := K) 4 1 1 0 1
  have hX (i : Fin 2) : homogeneousComponent 4 ((X i : P)) = 0 := by
    simpa using hc_pow (K := K) 4 1 i
  simp only [(homogeneousComponent 4 : P →ₗ[K] P).map_add,
    (homogeneousComponent 4 : P →ₗ[K] P).map_sub,
    homogeneousComponent_C_mul, hc_C, hc_pow, hc_mixed, h31, h32, h2, hX]
  norm_num

def leadingIdeal (b : ℕ) (A B r : K) (lowerX lowerZ : P) : Ideal P :=
  Ideal.span {homogeneousComponent b (X 0^b+lowerX),
    homogeneousComponent b (X 1^b+lowerZ), homogeneousComponent 4 (s3 A B r)}

theorem dense_eq_span (b : ℕ) :
    SecantIdeal.dense (K := K) b =
      Ideal.span {(X 0 : P)^b, X 1^b, X 0^2*X 1^2} := by
  simp [SecantIdeal.dense, SecantIdeal.denseExponents,
    Set.image_insert_eq, Set.image_singleton, ← X_pow_eq_monomial,
    monomial_add_single, ← X_pow_eq_monomial]

theorem leadingIdeal_eq_dense (b : ℕ) (A B r : K) (lowerX lowerZ : P)
    (hX : lowerX.totalDegree < b) (hZ : lowerZ.totalDegree < b) :
    leadingIdeal b A B r lowerX lowerZ = SecantIdeal.dense b := by
  simp [leadingIdeal, map_add, homogeneousComponent_eq_zero b lowerX hX,
    homogeneousComponent_eq_zero b lowerZ hZ, hc_pow, s3_component, dense_eq_span]

/-- Actual dense equations: monic membership polynomials plus ordinary S3. -/
theorem dense_equations_first_vanishing (b : ℕ) (hb : 2 ≤ b) (A B r : K)
    (lowerX lowerZ : P) (hX : lowerX.totalDegree < b) (hZ : lowerZ.totalDegree < b) :
    SecantIdeal.FirstVanishing (leadingIdeal b A B r lowerX lowerZ) (b+1) := by
  rw [leadingIdeal_eq_dense b A B r lowerX lowerZ hX hZ]
  exact SecantIdeal.dense_first_vanishing b hb

end DenseTopForms

#print axioms DenseTopForms.s3_component
#print axioms DenseTopForms.leadingIdeal_eq_dense
#print axioms DenseTopForms.dense_equations_first_vanishing
