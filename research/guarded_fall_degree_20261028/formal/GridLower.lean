import FilteredClosure
import ThreePointRigidity
import AffineCoefficient
import PrimeBitLocal

namespace GridLower
noncomputable section
open MvPolynomial

variable {K : Type*} [Field K]

abbrev P (K : Type*) [Field K] := MvPolynomial (Fin 5) K

def base (x y z : K) (j : Fin 4) : K :=
  if j = 0 then x else if j = 1 then y else if j = 2 then z else (x-y)⁻¹

/-- Specialize the guarded base coordinates but leave u formal. -/
def specialize (x y z : K) : P K →+* Polynomial K :=
  (Polynomial.mapRingHom (eval (base x y z))).comp (finSuccEquiv K 4).toRingHom

/-- The ideal of polynomials vanishing on the entire guarded base grid,
with u left as an indeterminate. -/
def gridIdeal (S : Set K) : Ideal (P K) where
  carrier := {p | ∀ x ∈ S, ∀ y ∈ S, x ≠ y → ∀ z ∈ S, specialize x y z p = 0}
  zero_mem' := by
    intro x hx y hy hxy z hz
    exact map_zero (specialize x y z)
  add_mem' := by
    intro p q hp hq x hx y hy hxy z hz
    simp only [map_add, hp x hx y hy hxy z hz, hq x hx y hy hxy z hz, add_zero]
  smul_mem' := by
    intro q p hp x hx y hy hxy z hz
    change specialize x y z (q*p) = 0
    rw [map_mul, hp x hx y hy hxy z hz, mul_zero]

/-- The u² coefficient of a degree-at-most-three polynomial specializes to
one fixed affine expression in the guarded base coordinates. -/
theorem coefficient_affine (p : P K) (hd : p.totalDegree ≤ 3) :
    ∃ l₀ lₓ lᵧ lz lv : K, ∀ x y z,
      (specialize x y z p).coeff 2 = l₀+lₓ*x+lᵧ*y+lz*z+lv*(x-y)⁻¹ := by
  let q := (finSuccEquiv K 4 p).coeff 2
  refine ⟨q.coeff 0, q.coeff (Finsupp.single 0 1), q.coeff (Finsupp.single 1 1),
    q.coeff (Finsupp.single 2 1), q.coeff (Finsupp.single 3 1), ?_⟩
  intro x y z
  change (Polynomial.map (eval (base x y z)) (finSuccEquiv K 4 p)).coeff 2 = _
  rw [Polynomial.coeff_map]
  change eval (base x y z) q = _
  conv_lhs => rw [AffineCoefficient.affine_expansion q (AffineCoefficient.square_coefficient_degree p hd)]
  simp [Fin.sum_univ_succ, base]
  ring

/-- The low-degree part of I + K f1 + K f2 lies in I. The hypotheses here
are scalar coefficient formulas, not an assumed closure obstruction. -/
theorem low_part (a b c r : K) (hab : a ≠ b) (hac : a ≠ c)
    (hbc : b ≠ c) (h6 : (6 : K) ≠ 0)
    (f₁ f₂ p i : P K) (c₁ c₂ : K)
    (hi : i ∈ gridIdeal ({a,b,c} : Set K))
    (hp : p = i+c₁•f₁+c₂•f₂) (hd : p.totalDegree ≤ 3)
    (hf₁ : ∀ x y z, (specialize x y z f₁).coeff 2 = (x-y)^2)
    (hf₂ : ∀ x y z, (specialize x y z f₂).coeff 2 = (z-r)^2) :
    p ∈ gridIdeal ({a,b,c} : Set K) := by
  obtain ⟨l₀,lₓ,lᵧ,lz,lv,hl⟩ := coefficient_affine p hd
  have hw : c₁ = 0 ∧ c₂ = 0 := by
    apply ThreePointRigidity.quartic_weights_zero a b c r c₁ c₂ l₀ lₓ lᵧ lz lv hab hac hbc h6
    intro x hx y hy hxy z hz
    have hzpoly := hi x hx y hy hxy z hz
    have heq := hl x y z
    rw [hp] at heq
    simp only [smul_eq_C_mul, map_add, map_mul] at heq
    have hC (t : K) : specialize x y z (C t) = Polynomial.C t := by
      simp [specialize, finSuccEquiv_apply]
    simp only [hC, hzpoly, zero_add, Polynomial.coeff_add,
      Polynomial.coeff_C_mul, hf₁, hf₂] at heq
    exact heq
  rcases hw with ⟨rfl,rfl⟩
  simpa [hp] using hi

/-- The separating space I + K f1 + K f2. Its ideal component is not
artificially degree-truncated; the closure rule tests the actual product. -/
def envelope (I : Ideal (P K)) (f₁ f₂ : P K) : Submodule K (P K) where
  carrier := {p | ∃ i ∈ I, ∃ c₁ c₂ : K, p = i+c₁•f₁+c₂•f₂}
  zero_mem' := by exact ⟨0, I.zero_mem, 0, 0, by simp⟩
  add_mem' := by
    rintro p q ⟨i,hi,c₁,c₂,rfl⟩ ⟨j,hj,d₁,d₂,rfl⟩
    refine ⟨i+j, I.add_mem hi hj, c₁+d₁, c₂+d₂, ?_⟩
    simp only [smul_eq_C_mul, map_add]
    ring
  smul_mem' := by
    rintro a p ⟨i,hi,c₁,c₂,rfl⟩
    refine ⟨C a*i, I.mul_mem_left (C a) hi, a*c₁, a*c₂, ?_⟩
    simp only [smul_eq_C_mul, map_mul]
    ring

theorem ideal_mem_envelope (I : Ideal (P K)) (f₁ f₂ p : P K) (hp : p ∈ I) :
    p ∈ envelope I f₁ f₂ := ⟨p,hp,0,0,by simp⟩

theorem quartics_mem_envelope (I : Ideal (P K)) (f₁ f₂ : P K) :
    f₁ ∈ envelope I f₁ f₂ ∧ f₂ ∈ envelope I f₁ f₂ := by
  exact ⟨⟨0,I.zero_mem,1,0,by simp⟩, ⟨0,I.zero_mem,0,1,by simp⟩⟩

/-- The full degree-four separating-space theorem for any two quartics
with these u² coefficients, and any other inputs vanishing on the guarded
grid. There is no hypothesis that already asserts a degree obstruction. -/
theorem not_refutable_four (a b c r : K) (hab : a ≠ b) (hac : a ≠ c)
    (hbc : b ≠ c) (h6 : (6 : K) ≠ 0) (f₁ f₂ : P K) (F : Set (P K))
    (hf₁ : ∀ x y z, (specialize x y z f₁).coeff 2 = (x-y)^2)
    (hf₂ : ∀ x y z, (specialize x y z f₂).coeff 2 = (z-r)^2)
    (hF : ∀ p ∈ F, p ∈ gridIdeal ({a,b,c} : Set K) ∨ p = f₁ ∨ p = f₂) :
    ¬ FilteredClosure.Derivable F 4 1 := by
  let I := gridIdeal ({a,b,c} : Set K)
  let W := envelope I f₁ f₂
  have hlow (p : P K) (hp : p ∈ W) (hd : p.totalDegree ≤ 3) : p ∈ I := by
    rcases hp with ⟨i,hi,c₁,c₂,hp⟩
    exact low_part a b c r hab hac hbc h6 f₁ f₂ p i c₁ c₂ hi hp hd hf₁ hf₂
  apply FilteredClosure.not_refutable_of_separator W
  · intro p hp _
    rcases hF p hp with h | h | h
    · exact ideal_mem_envelope I f₁ f₂ p h
    · subst p; exact (quartics_mem_envelope I f₁ f₂).1
    · subst p; exact (quartics_mem_envelope I f₁ f₂).2
  · exact FilteredClosure.closed_at_four_of_low_part I W hlow
      (fun p hp _ => ideal_mem_envelope I f₁ f₂ p hp)
  · intro h1
    have hi := hlow 1 h1 (by simp)
    have hh := hi a (by simp) b (by simp) hab c (by simp)
    simpa using hh

end
end GridLower

#print axioms GridLower.coefficient_affine
#print axioms GridLower.low_part
#print axioms GridLower.not_refutable_four
