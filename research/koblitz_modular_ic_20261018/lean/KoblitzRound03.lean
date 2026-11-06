import KoblitzRelations
namespace KoblitzRound03

/-- The determinant character constraint strengthens the normalized square equation. -/
theorem determinant_normalized_square {F : Type*} [Field F]
    (epsilon eta chi p : F) (hchi : chi ≠ 0)
    (hdet : epsilon * eta = chi ^ 2) (hzero : epsilon + p * eta = 0) :
    (epsilon / chi) ^ 2 = -p := by
  rw [div_pow]
  apply (div_eq_iff (pow_ne_zero 2 hchi)).2
  linear_combination epsilon * hzero - p * hdet

/-- The zero-eigenvalue square equation halves the necessary exponent. -/
theorem determinant_half_exponent {R : Type*} [CommRing R]
    (u p : R) (m : ℕ) (horder : u ^ (2*m) = 1) (hsquare : u^2 = -p) :
    (-p)^m = 1 := by
  rw [← hsquare, ← pow_mul]
  exact horder

/-- Exact O_-7 norm identity used in the Hermitian arithmetic. -/
theorem cm_norm_identity {R : Type*} [CommRing R] (a b w : R)
    (hw : w^2+w+2=0) : (a+b*w)*(a-b-b*w) = a^2-a*b+2*b^2 := by
  linear_combination -b^2 * hw

/-- A two-dimensional alternating pairing transforms by the determinant. -/
theorem alternating_determinant {R : Type*} [CommRing R]
    (a b c d x₁ x₂ y₁ y₂ : R) :
    (a*x₁+b*x₂)*(c*y₁+d*y₂)-(c*x₁+d*x₂)*(a*y₁+b*y₂) =
      (a*d-b*c)*(x₁*y₂-x₂*y₁) := by ring

/-- An anti-isometry's graph is isotropic in the product alternating form. -/
theorem anti_isometry_graph_isotropic {R : Type*} [CommRing R]
    (a b c d pairing : R) (h : pairing*(a*d-b*c) = -1)
    (x₁ x₂ y₁ y₂ : R) :
    pairing*((a*x₁+b*x₂)*(c*y₁+d*y₂)-(c*x₁+d*x₂)*(a*y₁+b*y₂)) +
      (x₁*y₂-x₂*y₁) = 0 := by
  rw [alternating_determinant]
  linear_combination (x₁*y₂-x₂*y₁) * h

/-- A graph intersects the left factor trivially, regardless of the kernel prime. -/
theorem graph_transverse_left {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (psi : B →+ A) (a : A) (h : ∃ b, (a, (0:B)) = (psi b, b)) : a = 0 := by
  obtain ⟨b, hb⟩ := h
  have h0 : b = 0 := (congrArg Prod.snd hb).symm
  simpa [h0] using congrArg Prod.fst hb

/-- An injective graph map also intersects the right factor trivially. -/
theorem graph_transverse_right {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (psi : B →+ A) (hinj : Function.Injective psi) (b : B)
    (h : ∃ c, ((0:A), b) = (psi c, c)) : b = 0 := by
  obtain ⟨c,hc⟩ := h
  have hpc : psi c = 0 := (congrArg Prod.fst hc).symm
  have hc0 : c = 0 := hinj (by simpa using hpc : psi c = psi 0)
  simpa [hc0] using congrArg Prod.snd hc

/-- Quotienting by a graph preserves the left factor. The kernel bridge is explicit. -/
theorem graph_quotient_left_injective {A B Q : Type*}
    [AddCommGroup A] [AddCommGroup B] [AddCommGroup Q]
    (psi : B →+ A) (quotient : A × B →+ Q)
    (hkernel : ∀ a, quotient (a,0) = 0 → ∃ b, (a,(0:B)) = (psi b,b)) :
    Function.Injective (fun a : A => quotient (a,0)) := by
  intro a a' heq
  change quotient (a,0) = quotient (a',0) at heq
  have hz : quotient (a-a',0) = 0 := by
    have : (a-a',(0:B)) = (a,0)-(a',0) := by ext <;> simp
    rw [this, map_sub, heq, sub_self]
  exact sub_eq_zero.mp (graph_transverse_left psi (a-a') (hkernel _ hz))

/-- Exact substitution for the same-field cubic-cover control. -/
theorem binary_cubic_cover_substitution {K : Type*} [CommRing K] [CharP K 2]
    (u v a d : K) :
    (u*v+d)^2+u^3*(u*v+d)+(u^3)^3+a*(u^3)^2+d^2 =
      u^2*(v^2+u^2*v+u^7+a*u^4+d*u) := by
  have h2 : (2:K)=0 := by simpa using (CharP.natCast_eq_natCast_mod K 2 2)
  linear_combination (u*v*d+d^2) * h2

/-- Intersection of two equal-multiplicity five-point blowup classes. -/
def intersection (degree multiplicity degree' multiplicity' : ℤ) : ℤ :=
  degree*degree' - 5*multiplicity*multiplicity'

theorem conic_pullback_intersections :
    intersection 5 2 2 1 = 0 ∧ intersection 5 2 5 2 = 5 ∧
    intersection 10 4 2 1 = 0 ∧ intersection 10 4 10 4 = 20 ∧
    intersection 10 4 5 2 = 10 := by decide

/-- Adjunction gives genus six conditional on the geometric intersection bridge. -/
theorem decic_adjunction_genus (g : ℤ)
    (h : 2*g-2 = intersection 10 4 10 4 - intersection 10 4 5 2) : g = 6 := by
  norm_num [intersection] at h
  omega
end KoblitzRound03

#print axioms KoblitzRound03.determinant_normalized_square
#print axioms KoblitzRound03.determinant_half_exponent
#print axioms KoblitzRound03.cm_norm_identity
#print axioms KoblitzRound03.alternating_determinant
#print axioms KoblitzRound03.anti_isometry_graph_isotropic
#print axioms KoblitzRound03.graph_transverse_left
#print axioms KoblitzRound03.graph_transverse_right
#print axioms KoblitzRound03.graph_quotient_left_injective
#print axioms KoblitzRound03.binary_cubic_cover_substitution
#print axioms KoblitzRound03.conic_pullback_intersections
#print axioms KoblitzRound03.decic_adjunction_genus

namespace KoblitzRound03
/-- Rational-point injection contradicts the positive-sign genus-four count vector. -/
theorem genus4_positive_sign_count_obstruction {P Q : Type*} [Fintype P] [Fintype Q]
    (inclusion : P → Q) (hinj : Function.Injective inclusion)
    (hP : Fintype.card P = 4) (hQ : Fintype.card Q = 2) : False := by
  have h := Fintype.card_le_of_injective inclusion hinj
  omega

/-- The negative-sign genus-four count contradicts the supplied published maximum. -/
theorem genus4_negative_sign_count_obstruction (N : ℕ)
    (hPublishedMaximum : N ≤ 71) (hFrobeniusCount : N = 77) : False := by omega
end KoblitzRound03
#print axioms KoblitzRound03.genus4_positive_sign_count_obstruction
#print axioms KoblitzRound03.genus4_negative_sign_count_obstruction
