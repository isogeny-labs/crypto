import Mathlib

namespace KoblitzRelations

/-- Homogeneous orbit recurrences remain true under independent scalar rescaling. -/
theorem recurrence_scale {F : Type*} [Field F] {I : Type*}
    (next : I → I) (t c : F) (v : I → F)
    (h : ∀ i, v i - t * v (next i) + 2 * v (next (next i)) = 0) :
    ∀ i, c * v i - t * (c * v (next i)) + 2 * (c * v (next (next i))) = 0 := by
  intro i
  linear_combination c * h i

/-- Anchoring a different block does not remove the target-block scaling freedom. -/
theorem separate_blocks_leave_target_scaling
    {F V W : Type*} [Field F] [AddCommGroup V] [Module F V]
    [AddCommGroup W] [Module F W] (L : V →ₗ[F] W)
    (base target : V) (hbase : L base = 0) (htarget : L target = 0)
    (c : F) : L base = 0 ∧ L (c • target) = 0 := by
  exact ⟨hbase, by simp [htarget]⟩

/-- Distinct scale parameters really give distinct nonzero target vectors. -/
theorem nonzero_target_scales_distinct
    {F V : Type*} [Field F] [AddCommGroup V] [Module F V]
    (v : V) (hv : v ≠ 0) (c d : F) (hcd : c ≠ d) : c • v ≠ d • v := by
  intro h
  have : c = d := (smul_left_injective F hv) h
  exact hcd this

/-- A push-pull degree prime to r implies injectivity on r-torsion.
The Bezout witness is explicit; no claim of a geometric map is assumed. -/
theorem push_pull_injective_on_torsion
    {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (pull : A →+ B) (push : B →+ A)
    (degree r u v : ℤ) (hBezout : u * degree + v * r = 1)
    (hComp : ∀ x, push (pull x) = degree • x)
    (x y : A) (hx : r • x = 0) (hy : r • y = 0)
    (hxy : pull x = pull y) : x = y := by
  have hd : degree • (x - y) = 0 := by
    rw [smul_sub, ← hComp x, ← hComp y, hxy, sub_self]
  have hr : r • (x - y) = 0 := by simp [smul_sub, hx, hy]
  apply sub_eq_zero.mp
  calc
    x-y = (1 : ℤ) • (x-y) := by simp
    _ = (u*degree + v*r) • (x-y) := by rw [hBezout]
    _ = u • (degree • (x-y)) + v • (r • (x-y)) := by
      simp only [add_zsmul, mul_smul]
    _ = 0 := by simp [hd, hr]

/-- Necessary condition for a zero Hecke eigenvalue to match a character pair. -/
theorem eisenstein_zero_necessary {F : Type*} [CommRing F]
    (epsilon eta p : F) (D : ℕ)
    (hEpsilon : epsilon ^ D = 1) (hEta : eta ^ D = 1)
    (hHecke : epsilon + p * eta = 0) : (-p) ^ D = 1 := by
  have h : epsilon = -p * eta := by linear_combination hHecke
  rw [h, mul_pow, hEta, mul_one] at hEpsilon
  exact hEpsilon

/-- The extension-field point-order factorization, at each Frobenius pair. -/
theorem quadratic_extension_factorization {R : Type*} [CommRing R] (a b : R) :
    (1-a*a)*(1-b*b) = ((1-a)*(1-b))*((1+a)*(1+b)) := by ring

/-- The rational flex-point divisor relations on the Klein quartic force 7-torsion. -/
theorem klein_flex_relations {A : Type*} [AddCommGroup A] (D E : A)
    (h1 : E = (3 : ℤ) • D) (h2 : (3 : ℤ) • E = (2 : ℤ) • D) :
    (7 : ℤ) • D = 0 := by
  rw [h1, ← mul_smul] at h2
  have h9 : (9 : ℤ) • D = (2 : ℤ) • D := by simpa using h2
  calc
    (7 : ℤ) • D = (9 : ℤ) • D - (2 : ℤ) • D := by module
    _ = 0 := sub_eq_zero.mpr h9

/-- Polynomial identity underlying the explicit Klein cyclic quotient. -/
theorem klein_invariant_identity {K : Type*} [CommRing K] [CharP K 2]
    (x y z : K) :
    let a := x+y+z
    let b := x*y+y*z+z*x
    let c := x*y*z
    a^2*c^2 + a*b^2*c + a^3*b*c + a^5*c + b^4 =
      (x^3*y+y^3*z+z^3*x)*(x*y^3+y*z^3+z*x^3) := by
  dsimp
  have h6 : (6 : K) = 0 := by simpa using (CharP.natCast_eq_natCast_mod K 2 6)
  have h18 : (18 : K) = 0 := by simpa using (CharP.natCast_eq_natCast_mod K 2 18)
  have h36 : (36 : K) = 0 := by simpa using (CharP.natCast_eq_natCast_mod K 2 36)
  have h61 : (61 : K) = 1 := by simpa using (CharP.natCast_eq_natCast_mod K 2 61)
  ring_nf
  simp [h6, h18, h36, h61]

/-- Character orthogonality used by the Walsh point-count identity. -/
theorem additive_character_orthogonality
    {A : Type*} [AddGroup A] [Fintype A] (psi : AddChar A ℂ) (h : psi ≠ 0) :
    ∑ a, psi a = 0 := AddChar.sum_eq_zero_iff_ne_zero.mpr h

/-- Three-term Frobenius trace is fixed whenever the input is fixed by its cube. -/
theorem frobenius_trace_fixed {A : Type*} [AddCommGroup A]
    (F : A →+ A) (x : A) (h : F (F (F x)) = x) :
    F (x + F x + F (F x)) = x + F x + F (F x) := by
  simp only [map_add, h]
  abel

/-- A Frobenius intertwiner also intertwines every iterate. -/
theorem intertwiner_iterates {A B : Type*}
    (FA : A → A) (FB : B → B) (M : A → B)
    (h : ∀ x, FB (M x) = M (FA x)) :
    ∀ n x, (FB^[n]) (M x) = M ((FA^[n]) x) := by
  intro n
  induction n with
  | zero => simp
  | succ n ih =>
    intro x
    rw [Function.iterate_succ_apply', ih, h, Function.iterate_succ_apply']

/-- A linear transfer with nonzero anchored logarithm recovers the scalar.
The geometric transfer and the correctness of the logarithm remain inputs. -/
theorem anchored_transfer_recovery {F V : Type*} [Field F]
    [AddCommGroup V] [Module F V] (transfer : F →ₗ[F] V)
    (logarithm : V →ₗ[F] F) (h : logarithm (transfer 1) ≠ 0) (d : F) :
    logarithm (transfer d) / logarithm (transfer 1) = d := by
  have hd : transfer d = d • transfer 1 := by
    simpa using transfer.map_smul d (1 : F)
  rw [hd, map_smul, smul_eq_mul, mul_div_cancel_right₀ _ h]

/-- The characteristic-two Klein gradient has no projective zero on the curve.
This proves the polynomial criterion; the projective Jacobian criterion is external. -/
theorem klein_gradient_common_zero {K : Type*} [Field K] (x y z : K)
    (hc : x^3*y+y^3*z+z^3*x = 0)
    (hx : x^2*y+z^3 = 0) (hy : x^3+y^2*z = 0)
    (hz : y^3+z^2*x = 0) : x = 0 ∧ y = 0 ∧ z = 0 := by
  have hb : y^3*z = 0 := by linear_combination hc - x*hx
  have hy0 : y = 0 := by
    rcases mul_eq_zero.mp hb with h | h
    · exact eq_zero_of_pow_eq_zero h
    · exact eq_zero_of_pow_eq_zero (by simpa [h] using hz : y^3 = 0)
  have hz0 : z = 0 := eq_zero_of_pow_eq_zero (by simpa [hy0] using hx : z^3 = 0)
  have hx0 : x = 0 := eq_zero_of_pow_eq_zero (by simpa [hy0, hz0] using hy : x^3 = 0)
  exact ⟨hx0, hy0, hz0⟩

end KoblitzRelations

#print axioms KoblitzRelations.recurrence_scale
#print axioms KoblitzRelations.separate_blocks_leave_target_scaling
#print axioms KoblitzRelations.nonzero_target_scales_distinct
#print axioms KoblitzRelations.push_pull_injective_on_torsion
#print axioms KoblitzRelations.eisenstein_zero_necessary
#print axioms KoblitzRelations.quadratic_extension_factorization
#print axioms KoblitzRelations.klein_flex_relations
#print axioms KoblitzRelations.klein_invariant_identity
#print axioms KoblitzRelations.additive_character_orthogonality
#print axioms KoblitzRelations.frobenius_trace_fixed
#print axioms KoblitzRelations.intertwiner_iterates
#print axioms KoblitzRelations.anchored_transfer_recovery
#print axioms KoblitzRelations.klein_gradient_common_zero
