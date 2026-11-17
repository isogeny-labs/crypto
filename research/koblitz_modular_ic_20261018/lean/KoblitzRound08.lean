import KoblitzRound07Cofactor

namespace KoblitzRound08

/-- The explicit level-basis change is invertible on its stated chart. -/
theorem deck_basis_inverse {R : Type*} [CommRing R] (a u p q : R)
    (h : a*u=1) :
    a*(u*p+q)+(-a*q)=p ∧ -u*(-a*q)=q := by
  constructor
  · calc
      a*(u*p+q)+(-a*q)=(a*u)*p := by ring
      _=p := by rw [h]; simp
  · calc
      -u*(-a*q)=(a*u)*q := by ring
      _=q := by rw [h]; simp

theorem fixed_line_swap_descent {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (f : A →+ B) (p q : A) :
    (-(-f q), f p)=-(f (-q), -f p) := by simp

/-- A checked inverse matrix excludes identification of distinct coordinates. -/
theorem coordinate_change_injective {A B : Type*} (f : A → B) (g : B → A)
    (h : ∀ x, g (f x)=x) : Function.Injective f := by
  exact Function.LeftInverse.injective h

theorem kummer_denominator_identity {R : Type*} [CommRing R] (z d f : R) :
    (z*d)^19-f*d^19=d^19*(z^19-f) := by ring

theorem kummer_root_change {R : Type*} [CommRing R] (z r f : R)
    (h : z^19=f) : (z*r)^19=f*r^19 := by rw [mul_pow,h]

theorem kummer_jacobian_identity {R : Type*} [CommRing R] (z d n e : R) :
    (z^18*d)*(z*e)-1=e*(z^19*d-n)+(n*e-1) := by ring

/-- An explicit inverse for the localized Jacobian entry. -/
theorem kummer_jacobian_inverse {R : Type*} [CommRing R] (z d n e : R)
    (h : z^19*d=n) (hn : n*e=1) : (z^18*d)*(z*e)=1 := by
  have r := kummer_jacobian_identity z d n e
  rw [h,hn] at r
  linear_combination r

theorem binary_function_product {R : Type*} [CommRing R] (a b c d y h f : R) :
    (a+b*y)*(c+d*y)-((a*c+b*d*f)+(a*d+b*c+b*d*h)*y)=
      b*d*(y^2-h*y-f) := by ring

def millerDiv {A : Type*} [AddCommGroup A] (n : ℤ) (p : A) : FreeAbelianGroup A :=
  n • FreeAbelianGroup.of p - FreeAbelianGroup.of (n • p) -
    (n-1) • FreeAbelianGroup.of 0

theorem miller_divisor_recurrence {A : Type*} [AddCommGroup A] (n m : ℤ) (p : A) :
    millerDiv n p+millerDiv m p+
      (FreeAbelianGroup.of (n • p)+FreeAbelianGroup.of (m • p)-
       FreeAbelianGroup.of ((n+m) • p)-FreeAbelianGroup.of 0)=millerDiv (n+m) p := by
  simp only [millerDiv,add_zsmul,sub_zsmul]
  abel

theorem miller_divisor_torsion {A : Type*} [AddCommGroup A] (p : A)
    (h : (19:ℤ) • p=0) :
    millerDiv 19 p=(19:ℤ) • (FreeAbelianGroup.of p-FreeAbelianGroup.of 0) := by
  simp only [millerDiv,h,zsmul_sub]
  norm_num
  abel

theorem kummer_root_nonzero {K : Type*} [Field K] (z f : K)
    (h : z^19=f) (hf : f≠0) : z≠0 := by
  intro hz
  subst z
  simp at h
  exact hf h.symm

/-- The derivative is a unit over the declared field chart. -/
theorem kummer_derivative_nonzero {K : Type*} [Field K] [CharP K 2]
    (z f : K) (h : z^19=f) (hf : f≠0) : (19:K)*z^18≠0 := by
  have h2 : (2:K)=0 := by simpa using (CharP.natCast_eq_natCast_mod K 2 2)
  have h19 : (19:K)=1 := by linear_combination 9*h2
  rw [h19,one_mul]
  exact pow_ne_zero _ (kummer_root_nonzero z f h hf)

theorem diagonal_jacobian {K : Type*} [CommRing K] (a b : K) :
    Matrix.det (!![a,0;0,b])=a*b := by simp [Matrix.det_fin_two]

theorem complete_new_norm_degrees : 5*13200+6600=72600 := by norm_num

theorem transition_cocycle {K : Type*} [Field K] (a b c : K)
    (ha : a≠0) (hb : b≠0) : (c/b)*(b/a)=c/a := by
  field_simp

/-- Five independently excluded charts cannot cover six chart indices. -/
theorem six_chart_cover (bad : Fin 5 → Fin 6 → Prop)
    (disjoint : ∀ i k l, bad i k → bad i l → k=l) :
    ∃ k, ∀ i, ¬bad i k := by
  classical
  by_contra h
  push_neg at h
  let f : Fin 6 → Fin 5 := fun k => Classical.choose (h k)
  have hi : Function.Injective f := by
    intro k l heq
    apply disjoint (f k) k l
    · exact Classical.choose_spec (h k)
    · rw [heq]
      exact Classical.choose_spec (h l)
  have hc := Fintype.card_le_of_injective f hi
  norm_num at hc

end KoblitzRound08

#print axioms KoblitzRound08.deck_basis_inverse
#print axioms KoblitzRound08.fixed_line_swap_descent
#print axioms KoblitzRound08.coordinate_change_injective
#print axioms KoblitzRound08.kummer_denominator_identity
#print axioms KoblitzRound08.kummer_root_change
#print axioms KoblitzRound08.kummer_jacobian_identity
#print axioms KoblitzRound08.kummer_jacobian_inverse
#print axioms KoblitzRound08.binary_function_product
#print axioms KoblitzRound08.miller_divisor_recurrence
#print axioms KoblitzRound08.miller_divisor_torsion
#print axioms KoblitzRound08.kummer_root_nonzero
#print axioms KoblitzRound08.kummer_derivative_nonzero
#print axioms KoblitzRound08.diagonal_jacobian
#print axioms KoblitzRound08.complete_new_norm_degrees
