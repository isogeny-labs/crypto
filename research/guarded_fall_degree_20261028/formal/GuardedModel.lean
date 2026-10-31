import GridLower

namespace GuardedModel
noncomputable section
open MvPolynomial GridLower

variable {K : Type*} [Field K]

def membership (a b c : K) (t : P K) : P K := (t-C a)*(t-C b)*(t-C c)

def first (α β : K) : P K := PrimeBitLocal.oddS3 (C α) (C β) (X 1) (X 2) (X 0)
def second (α β r : K) : P K := PrimeBitLocal.oddS3 (C α) (C β) (X 0) (X 3) (C r)
def guard : P K := X 4*(X 1-X 2)-1

/-- Precisely the six ordinary-ring inputs; variable order is u,x,y,z,v. -/
def inputs (α β a b c r : K) : Set (P K) :=
  {membership a b c (X 1), membership a b c (X 2), membership a b c (X 3),
    first α β, second α β r, guard}

@[simp] theorem specialize_C (x y z t : K) :
    specialize x y z (C t) = Polynomial.C t := by
  simp [specialize, finSuccEquiv_apply]

@[simp] theorem specialize_u (x y z : K) :
    specialize x y z (X 0) = Polynomial.X := by
  simp [specialize, finSuccEquiv_X_zero]

@[simp] theorem specialize_x (x y z : K) :
    specialize x y z (X 1) = Polynomial.C x := by
  change Polynomial.map (eval (base x y z))
    (finSuccEquiv K 4 (X (Fin.succ (0 : Fin 4)))) = _
  rw [finSuccEquiv_X_succ]
  simp [base]

@[simp] theorem specialize_y (x y z : K) :
    specialize x y z (X 2) = Polynomial.C y := by
  change Polynomial.map (eval (base x y z))
    (finSuccEquiv K 4 (X (Fin.succ (1 : Fin 4)))) = _
  rw [finSuccEquiv_X_succ]
  simp [base]

@[simp] theorem specialize_z (x y z : K) :
    specialize x y z (X 3) = Polynomial.C z := by
  change Polynomial.map (eval (base x y z))
    (finSuccEquiv K 4 (X (Fin.succ (2 : Fin 4)))) = _
  rw [finSuccEquiv_X_succ]
  simp [base]

@[simp] theorem specialize_v (x y z : K) :
    specialize x y z (X 4) = Polynomial.C (x-y)⁻¹ := by
  change Polynomial.map (eval (base x y z))
    (finSuccEquiv K 4 (X (Fin.succ (3 : Fin 4)))) = _
  rw [finSuccEquiv_X_succ]
  simp [base]

theorem first_coefficient (α β x y z : K) :
    (specialize x y z (first α β)).coeff 2 = (x-y)^2 := by
  have heq : specialize x y z (first α β) =
      Polynomial.C ((x-y)^2)*Polynomial.X^2 +
      Polynomial.C (-2*((x+y)*(x*y+α)+2*β))*Polynomial.X +
      Polynomial.C ((x*y-α)^2-4*β*(x+y)) := by
    simp only [first, PrimeBitLocal.oddS3, map_add, map_sub, map_mul, map_pow,
      map_ofNat, specialize_C, specialize_x, specialize_y, specialize_u, map_neg]
    ring
  rw [heq]
  simp only [Polynomial.coeff_add, Polynomial.coeff_C_mul, Polynomial.coeff_X_pow,
    Polynomial.coeff_C, Polynomial.coeff_X]
  simp

theorem second_coefficient (α β r x y z : K) :
    (specialize x y z (second α β r)).coeff 2 = (z-r)^2 := by
  have heq : specialize x y z (second α β r) =
      Polynomial.C ((z-r)^2)*Polynomial.X^2 +
      Polynomial.C (-2*((z+r)*(z*r+α)+2*β))*Polynomial.X +
      Polynomial.C ((z*r-α)^2-4*β*(z+r)) := by
    simp only [second, PrimeBitLocal.oddS3, map_add, map_sub, map_mul, map_pow,
      map_ofNat, specialize_C, specialize_z, specialize_u, map_neg]
    ring
  rw [heq]
  simp only [Polynomial.coeff_add, Polynomial.coeff_C_mul, Polynomial.coeff_X_pow,
    Polynomial.coeff_C, Polynomial.coeff_X]
  simp

theorem membership_x_grid (a b c : K) :
    membership a b c (X 1) ∈ gridIdeal ({a,b,c} : Set K) := by
  intro x hx y hy hxy z hz
  rcases hx with rfl | rfl | rfl <;>
    simp [membership, map_sub, map_mul]

theorem membership_y_grid (a b c : K) :
    membership a b c (X 2) ∈ gridIdeal ({a,b,c} : Set K) := by
  intro x hx y hy hxy z hz
  rcases hy with rfl | rfl | rfl <;>
    simp [membership, map_sub, map_mul]

theorem membership_z_grid (a b c : K) :
    membership a b c (X 3) ∈ gridIdeal ({a,b,c} : Set K) := by
  intro x hx y hy hxy z hz
  rcases hz with rfl | rfl | rfl <;>
    simp [membership, map_sub, map_mul]

theorem guard_grid (S : Set K) : (guard : P K) ∈ gridIdeal S := by
  intro x hx y hy hxy z hz
  calc
    specialize x y z guard = Polynomial.C ((x-y)⁻¹*(x-y)-1) := by
      simp only [guard, map_sub, map_mul, map_one, specialize_x, specialize_y, specialize_v]
    _ = 0 := by simp [inv_mul_cancel₀ (sub_ne_zero.mpr hxy)]

/-- Uniform degree-four obstruction for the actual six-input guarded
short-Weierstrass presentation. Curve nonsingularity and rational point
lifts are unnecessary for this lower bound. -/
theorem no_degree_four_refutation (α β a b c r : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) (h6 : (6 : K) ≠ 0) :
    ¬ FilteredClosure.Derivable (inputs α β a b c r) 4 1 := by
  apply GridLower.not_refutable_four a b c r hab hac hbc h6
    (first α β) (second α β r) (inputs α β a b c r)
    (first_coefficient α β) (second_coefficient α β r)
  intro p hp
  rcases hp with rfl | rfl | rfl | rfl | rfl | rfl
  · exact Or.inl (membership_x_grid a b c)
  · exact Or.inl (membership_y_grid a b c)
  · exact Or.inl (membership_z_grid a b c)
  · exact Or.inr (Or.inl rfl)
  · exact Or.inr (Or.inr rfl)
  · exact Or.inl (guard_grid _)

/-- The same obstruction holds at every smaller degree. -/
theorem no_refutation_below_five (α β a b c r : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) (h6 : (6 : K) ≠ 0)
    (d : ℕ) (hd : d < 5) :
    ¬ FilteredClosure.Derivable (inputs α β a b c r) d 1 := by
  intro h
  exact no_degree_four_refutation α β a b c r hab hac hbc h6
    (FilteredClosure.monotone (by omega) h)

/-- Lower bound for last fall under the exact unit-ideal hypothesis. -/
theorem lastFall_lower (α β a b c r : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) (h6 : (6 : K) ≠ 0)
    (hunit : (1 : P K) ∈ Ideal.span (inputs α β a b c r))
    (d : ℕ) (hd : d < 5) :
    ¬ FilteredClosure.LastFallBound (inputs α β a b c r) d := by
  intro h
  exact no_refutation_below_five α β a b c r hab hac hbc h6 d hd
    (FilteredClosure.refutation_of_lastFallBound hunit h)

def pairDifference (a b c : K) : P K :=
  X 1^2+X 1*X 2+X 2^2-C (a+b+c)*(X 1+X 2)+C (a*b+a*c+b*c)

theorem membership_degree (a b c : K) (j : Fin 5) :
    (membership a b c (X j)).totalDegree ≤ 3 := by
  have h (t : K) : (X j-C t : P K).totalDegree ≤ 1 := by
    exact (totalDegree_sub_C_le _ _).trans (by simp)
  exact (totalDegree_mul _ _).trans (by
    have hh := (totalDegree_mul (X j-C a : P K) (X j-C b)).trans
      (Nat.add_le_add (h a) (h b))
    have hc := h c
    omega)

theorem guard_degree : (guard : P K).totalDegree ≤ 2 := by
  have hh : (X 1-X 2 : P K).totalDegree ≤ 1 := by
    exact (totalDegree_sub _ _).trans (by simp)
  have hm := totalDegree_mul (X 4 : P K) (X 1-X 2)
  have hg := totalDegree_sub (X 4*(X 1-X 2) : P K) 1
  simp only [totalDegree_X, totalDegree_one] at hm hg
  unfold guard
  omega

theorem pairDifference_degree (a b c : K) : (pairDifference a b c).totalDegree ≤ 2 := by
  have hxy : (X 1+X 2 : P K).totalDegree ≤ 1 := by
    exact (totalDegree_add _ _).trans (by simp)
  have hpow (j : Fin 5) : (X j^2 : P K).totalDegree ≤ 2 := by simp
  have hmul : (X 1*X 2 : P K).totalDegree ≤ 2 := by
    exact (totalDegree_mul _ _).trans (by simp)
  have hsum := totalDegree_add (X 1^2 : P K) (X 1*X 2)
  have hsum' := totalDegree_add (X 1^2+X 1*X 2 : P K) (X 2^2)
  have hlin := totalDegree_mul (C (a+b+c) : P K) (X 1+X 2)
  have hsub := totalDegree_sub (X 1^2+X 1*X 2+X 2^2 : P K)
    (C (a+b+c)*(X 1+X 2))
  have hfinal := totalDegree_add (X 1^2+X 1*X 2+X 2^2-C (a+b+c)*(X 1+X 2) : P K)
    (C (a*b+a*c+b*c))
  simp only [totalDegree_C] at hlin hfinal
  have h1 := hpow 1
  have h2 := hpow 2
  unfold pairDifference
  omega

/-- This is an actual filtered-closure derivation of the quadratic
consequence at degree four, rather than just its scalar identity. -/
theorem pairDifference_derivable (α β a b c r : K) :
    FilteredClosure.Derivable (inputs α β a b c r) 4 (pairDifference a b c) := by
  have hx : FilteredClosure.Derivable (inputs α β a b c r) 4 (membership a b c (X 1)) :=
    .input (by simp [inputs]) ((membership_degree a b c 1).trans (by omega))
  have hy : FilteredClosure.Derivable (inputs α β a b c r) 4 (membership a b c (X 2)) :=
    .input (by simp [inputs]) ((membership_degree a b c 2).trans (by omega))
  have hg : FilteredClosure.Derivable (inputs α β a b c r) 4 guard :=
    .input (by simp [inputs]) (guard_degree.trans (by omega))
  have hdiff : (membership a b c (X 1)-membership a b c (X 2)).totalDegree ≤ 3 :=
    (totalDegree_sub _ _).trans (max_le (membership_degree a b c 1) (membership_degree a b c 2))
  have hv := FilteredClosure.Derivable.mul (X 4) (FilteredClosure.sub hx hy)
    ((totalDegree_mul _ _).trans (by simpa using Nat.add_le_add_left hdiff 1))
  have hq := FilteredClosure.Derivable.mul (pairDifference a b c) hg
    ((totalDegree_mul _ _).trans (by
      have hh := pairDifference_degree a b c
      have hg' : (guard : P K).totalDegree ≤ 2 := guard_degree
      omega))
  have hid : (X 4 : P K)*(membership a b c (X 1)-membership a b c (X 2)) -
      pairDifference a b c*guard = pairDifference a b c := by
    unfold membership pairDifference guard
    simp only [map_add, map_mul]
    ring
  rw [← hid]
  exact FilteredClosure.sub hv hq

end
end GuardedModel

#print axioms GuardedModel.first_coefficient
#print axioms GuardedModel.second_coefficient
#print axioms GuardedModel.no_degree_four_refutation
#print axioms GuardedModel.no_refutation_below_five
#print axioms GuardedModel.lastFall_lower
#print axioms GuardedModel.pairDifference_derivable
