import KoblitzRound03

open scoped Matrix
namespace KoblitzRound04

/-- Integral change of coordinates; no inversion of 2 is needed to reduce it. -/
theorem modular49_integral_change {R : Type*} [CommRing R] (X Y : R) :
  let x := X-2
  let d := x^2+7*x+7
  let y := d*Y+4*x^3+22*x^2+35*x+7
  y^2-7*x*y*(x^2+5*x+7)-x*(x^6+7*x^5+21*x^4+49*x^3+147*x^2+343*x+343) =
    d^2*(Y^2+X*Y-X^3+X^2+2*X+1) := by dsimp; ring

/-- The actual characteristic-two specialization of the modular eta model. -/
theorem modular49_binary_change {K : Type*} [CommRing K] [CharP K 2] (X Y : K) :
  let y := (X^2+X+1)*Y+X+1
  y^2+X*y*(X^2+X+1)+X*(X^6+X^5+X^4+X^3+X^2+X+1) =
    (X^2+X+1)^2*(Y^2+X*Y+X^3+X^2+1) := by
  have h2 : (2:K)=0 := by simpa using (CharP.natCast_eq_natCast_mod K 2 2)
  dsimp
  linear_combination (-X^6-2*X^5-2*X^4+X^3*Y-X^3+2*X^2*Y+2*X*Y+X+Y) * h2

/-- All mixed terms cancel in the pullback of the fivefold product pairing. -/
theorem natural_polarization_pullback {R : Type*} [CommRing R]
    (a b c d e f g h q r : R) :
  (a-q)*(e-r)+(b-q)*(f-r)+(c-q)*(g-r)+(d-q)*(h-r)+
    (-a-b-c-d-q)*(-e-f-g-h-r) =
    a*e+b*f+c*g+d*h+(a+b+c+d)*(e+f+g+h)+5*q*r := by ring

/-- The kernel of the natural quotient is precisely diagonal five-torsion. -/
theorem natural_quotient_kernel {G : Type*} [AddCommGroup G]
    (a : Fin 5 → G) (q : G) (htrace : ∑ i, a i = 0) :
    (∀ i, a i-q=0) ↔ (a=(fun _ => q) ∧ 5 • q=0) := by
  constructor
  · intro h
    have ha : a=(fun _ => q) := funext (fun i => sub_eq_zero.mp (h i))
    refine ⟨ha, ?_⟩
    simpa [ha] using htrace
  · rintro ⟨ha,hq⟩ i
    simp [ha]

/-- The natural quotient has a rational inverse when the trace diagonal is invertible. -/
theorem natural_quotient_rational_inverse {A R : Type*}
    [AddCommGroup A] [AddCommGroup R] (trace : R →+ A) (diag : A →+ R)
    (hinverse : ∀ q, trace (diag q)=q) (p : R) :
    trace (p-diag (trace p))=0 ∧ (p-diag (trace p))+diag (trace p)=p := by
  constructor
  · simp [hinverse]
  · abel

/-- Integral augmentation removes the diagonal component without dividing by five. -/
theorem augmentation_sum_zero {G : Type*} [AddCommGroup G] (d : Fin 5 → G) :
    ∑ i, (5 • d i-∑ j, d j)=0 := by
  simp [Finset.sum_sub_distrib, Finset.sum_nsmul]

/-- Six-term Frobenius trace is fixed, conditional on the actual period-six input. -/
theorem trace6_fixed {G : Type*} [AddCommGroup G] (F : G →+ G) (x : G)
    (h : F (F (F (F (F (F x)))))=x) :
  F (x+F x+F (F x)+F (F (F x))+F (F (F (F x)))+F (F (F (F (F x))))) =
    x+F x+F (F x)+F (F (F x))+F (F (F (F x)))+F (F (F (F (F x)))) := by
  simp only [map_add]
  rw [h]
  abel

/-- Coordinate bounds follow from positive quadratic forms and their dual vectors. -/
theorem dual_coordinate_bound (q x d : ℝ) (hd : 0<d)
    (hpos : ∀ t : ℝ, 0 ≤ q-2*t*x+t^2*d) : x^2 ≤ q*d := by
  have hn : d ≠ 0 := ne_of_gt hd
  have h := hpos (x/d)
  have he : (q-2*(x/d)*x+(x/d)^2*d)*d = q*d-x^2 := by field_simp; ring
  have hp := mul_nonneg h (le_of_lt hd)
  rw [he] at hp
  linarith

/-- The characteristic-two quadratic twist isomorphism, including its residual. -/
theorem binary_twist_substitution {K : Type*} [CommRing K] [CharP K 2]
    (x y s a b : K) (hs : s^2+s=1) :
  (y+s*x)^2+x*(y+s*x)+x^3+a*x^2+b =
    y^2+x*y+x^3+(a+1)*x^2+b := by
  have h2 : (2:K)=0 := by simpa using (CharP.natCast_eq_natCast_mod K 2 2)
  linear_combination x^2 * hs + s*x*y*h2

/-- Cuspidal images killed by fourteen and eleven are zero. -/
theorem coprime_cusp_image_zero {G : Type*} [AddCommGroup G] (p : G)
    (h14 : 14 • p=0) (h11 : 11 • p=0) : p=0 := by
  have h : (56:ℕ) • p = (55:ℕ) • p + p := by
    rw [show (56:ℕ)=55+1 by decide, add_nsmul, one_nsmul]
  have hz56 : (56:ℕ) • p=0 := by simpa [smul_smul] using congrArg (fun x => (4:ℕ) • x) h14
  have hz55 : (55:ℕ) • p=0 := by simpa [smul_smul] using congrArg (fun x => (5:ℕ) • x) h11
  simpa [hz56,hz55] using h.symm

/-- Base-pullback relations supported on four base fibers have rank at most four. -/
theorem fiber_relation_rank_bound {K : Type*} [Field K] {m n : ℕ}
    (fiber : Fin n → Fin 4) (W : Matrix (Fin m) (Fin 4) K) :
    Matrix.rank (fun i j => W i (fiber j)) ≤ 4 := by
  let B : Matrix (Fin 4) (Fin n) K := fun k j => if k=fiber j then 1 else 0
  have h : (fun i j => W i (fiber j)) = W * B := by
    ext i j
    change W i (fiber j) = ∑ k, W i k * B k j
    simp [B]
  rw [h]
  exact (Matrix.rank_mul_le_left W B).trans (Matrix.rank_le_width W)

end KoblitzRound04

#print axioms KoblitzRound04.modular49_integral_change
#print axioms KoblitzRound04.modular49_binary_change
#print axioms KoblitzRound04.natural_polarization_pullback
#print axioms KoblitzRound04.natural_quotient_kernel
#print axioms KoblitzRound04.natural_quotient_rational_inverse
#print axioms KoblitzRound04.augmentation_sum_zero
#print axioms KoblitzRound04.trace6_fixed
#print axioms KoblitzRound04.dual_coordinate_bound
#print axioms KoblitzRound04.binary_twist_substitution
#print axioms KoblitzRound04.coprime_cusp_image_zero
#print axioms KoblitzRound04.fiber_relation_rank_bound
