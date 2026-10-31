import KoblitzRound08

namespace KoblitzRound09

/-- The dual split base carries inverse-transpose, not the source action. -/
theorem dual_pairing_identity {R : Type*} [CommRing R]
    (p0 p1 p2 p3 q0 q1 q2 q3 : R) :
    (-(p0+p1+p2+p3))*(-q3)+p0*(q0-q3)+p1*(q1-q3)+p2*(q2-q3)=
      p0*q0+p1*q1+p2*q2+p3*q3 := by ring

def sourceMatrix : Matrix (Fin 4) (Fin 4) ℤ :=
  !![-1,-1,-1,-1;1,0,0,0;0,1,0,0;0,0,1,0]
def dualMatrix : Matrix (Fin 4) (Fin 4) ℤ :=
  !![0,0,0,-1;1,0,0,-1;0,1,0,-1;0,0,1,-1]

theorem dual_matrix_left : sourceMatrix.transpose*dualMatrix=1 := by decide +kernel
theorem dual_matrix_right : dualMatrix*sourceMatrix.transpose=1 := by decide +kernel
theorem dual_matrix_order : dualMatrix^5=1 := by decide +kernel

def graphMatrix (t : ZMod 19) : Matrix (Fin 2) (Fin 2) (ZMod 19) :=
  !![2*t,1;-2,0]
def rootMatrix (t : ZMod 19) : Matrix (Fin 2) (Fin 2) (ZMod 19) :=
  !![0,9;1,t]

theorem root_matrix_inverse : ∀ t : ZMod 19,
    graphMatrix t*rootMatrix t=1 ∧ rootMatrix t*graphMatrix t=1 := by decide +kernel

theorem rotation_reflection_commute : ∀ k j : ZMod 5,
    (∀ i : ZMod 5, j-i+k=j-(i+k)) ↔ k=0 := by decide +kernel

theorem stable_twist : ∀ u : ZMod 5, 2*u+1=0 ↔ u=2 := by decide +kernel

theorem weil_five_plus {R : Type*} [CommRing R] (x : R) :
    (x^8+x^7-x^6-3*x^5-x^4-6*x^3-4*x^2+8*x+16)*(x^2-x+2)=
      x^10-11*x^5+32 := by ring

theorem weil_five_minus {R : Type*} [CommRing R] (x : R) :
    (x^8-x^7-x^6+3*x^5-x^4+6*x^3-4*x^2-8*x+16)*(x^2+x+2)=
      x^10+11*x^5+32 := by ring

theorem pow_nine_nineteen {R : Type*} [CommRing R] (z : R) :
    (z^9)^19=(z^19)^9 := by ring

theorem root_lift_plus {K : Type*} [Field K] (z w h a b : K)
    (hz : z^19=a) (hw : w^19=b) :
    ((z^9*w)*h)^19=(a^9*b)*h^19 := by
  simp only [mul_pow,pow_nine_nineteen,hz,hw]

theorem root_lift_minus {K : Type*} [Field K] (z w h a b : K)
    (hz : z^19=a) (hw : w^19=b) :
    ((z^9/w)*h)^19=(a^9/b)*h^19 := by
  simp only [mul_pow,div_pow,pow_nine_nineteen,hz,hw]

theorem rooted_deck_phase {K : Type*} [Field K] (phase z : K)
    (hz : z≠0) (h : phase*z=z) : phase=1 := by
  apply mul_right_cancel₀ hz
  simpa using h

theorem chart_transport_cancel {K : Type*} [Field K]
    (a r h s : K) (n : ℕ) (hr : r≠0) :
    ((a*r)^n*h*s)/(r^n)=a^n*h*s := by
  field_simp
  ring

/-- Conditional Riemann-Hurwitz arithmetic, not an actual quotient genus. -/
theorem reflection_genus_bound (g : ℤ) (h : 4*g-4≤2*2841-2) : g≤1421 := by omega

end KoblitzRound09

#print axioms KoblitzRound09.dual_pairing_identity
#print axioms KoblitzRound09.dual_matrix_left
#print axioms KoblitzRound09.dual_matrix_right
#print axioms KoblitzRound09.dual_matrix_order
#print axioms KoblitzRound09.root_matrix_inverse
#print axioms KoblitzRound09.rotation_reflection_commute
#print axioms KoblitzRound09.stable_twist
#print axioms KoblitzRound09.weil_five_plus
#print axioms KoblitzRound09.weil_five_minus
#print axioms KoblitzRound09.root_lift_plus
#print axioms KoblitzRound09.root_lift_minus
#print axioms KoblitzRound09.rooted_deck_phase
#print axioms KoblitzRound09.chart_transport_cancel
#print axioms KoblitzRound09.reflection_genus_bound

namespace KoblitzRound09

/-- A finite orbit sum is fixed when the action permutes its summands. -/
theorem trace_orbit_invariant {I A : Type*} [Fintype I] [AddCommMonoid A]
    (T : A →+ A) (e : I ≃ I) (y : I → A)
    (h : ∀ i, T (y i) = y (e i)) : T (∑ i, y i) = ∑ i, y i := by
  rw [map_sum]
  simp_rw [h]
  exact e.sum_comp y

/-- The separability idempotent recovers a coordinate from trace-weighted sums. -/
theorem trace_dual_recovery {I J R : Type*} [Fintype I] [Fintype J] [DecidableEq J]
    [CommRing R] (d : I → R) (a : I → J → R) (v : J → R) (j0 : J)
    (h : ∀ j, ∑ i, d i * a i j = if j = j0 then 1 else 0) :
    (∑ i, d i * (∑ j, a i j * v j)) = v j0 := by
  classical
  simp_rw [Finset.mul_sum, ← mul_assoc]
  rw [Finset.sum_comm]
  simp_rw [← Finset.sum_mul, h]
  simp

end KoblitzRound09

#print axioms KoblitzRound09.trace_orbit_invariant
#print axioms KoblitzRound09.trace_dual_recovery
