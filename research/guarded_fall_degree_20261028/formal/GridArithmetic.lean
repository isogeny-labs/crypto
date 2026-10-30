import CubicNormalization

namespace GridArithmetic
noncomputable section
open MvPolynomial CubicArithmetic

variable {K σ : Type*} [Field K]

/-- Outer index is the z exponent; inner index is the w exponent. -/
abbrev Grid (K : Type*) := Fin 3 → Fin 3 → K

def gridVal (G : Grid K) (a b : K) : K :=
  val (G 0) a+val (G 1) a*b+val (G 2) a*b^2
def gridPoly (w z : MvPolynomial σ K) (G : Grid K) : MvPolynomial σ K :=
  poly w (G 0)+poly w (G 1)*z+poly w (G 2)*z^2

@[simp] theorem gridVal_add (G H : Grid K) (a b : K) :
    gridVal (G+H) a b = gridVal G a b+gridVal H a b := by
  simp [gridVal, Pi.add_apply]; ring
@[simp] theorem gridVal_smul (d : K) (G : Grid K) (a b : K) :
    gridVal (d•G) a b = d*gridVal G a b := by
  simp [gridVal, Pi.smul_apply]; ring
@[simp] theorem gridVal_zero (a b : K) : gridVal (0 : Grid K) a b = 0 := by
  simp [gridVal]
@[simp] theorem gridPoly_add (G H : Grid K) (w z : MvPolynomial σ K) :
    gridPoly w z (G+H) = gridPoly w z G+gridPoly w z H := by
  simp [gridPoly, Pi.add_apply]; ring
@[simp] theorem gridPoly_smul (d : K) (G : Grid K) (w z : MvPolynomial σ K) :
    gridPoly w z (d•G) = d • gridPoly w z G := by
  simp [gridPoly, Pi.smul_apply, smul_eq_C_mul]; ring
@[simp] theorem gridPoly_zero (w z : MvPolynomial σ K) : gridPoly w z (0 : Grid K) = 0 := by
  simp [gridPoly]
@[simp] theorem eval_gridPoly (ρ : σ → K) (w z : MvPolynomial σ K) (G : Grid K) :
    MvPolynomial.eval ρ (gridPoly w z G) =
      gridVal G (MvPolynomial.eval ρ w) (MvPolynomial.eval ρ z) := by
  simp [gridPoly, gridVal]

def gridValMap (a b : K) : Grid K →ₗ[K] K where
  toFun := fun G => gridVal G a b
  map_add' := fun G H => gridVal_add G H a b
  map_smul' := by intros; simp

def gridPolyMap (w z : MvPolynomial σ K) : Grid K →ₗ[K] MvPolynomial σ K where
  toFun := gridPoly w z
  map_add' := fun G H => gridPoly_add G H w z
  map_smul' := by intros; simp

theorem gridPoly_degree (w z : MvPolynomial σ K) (G : Grid K)
    (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1) :
    (gridPoly w z G).totalDegree ≤ 4 := by
  have h0 := poly_degree w (G 0) hw
  have h1 := poly_degree w (G 1) hw
  have h2 := poly_degree w (G 2) hw
  have hz2 := totalDegree_pow z 2
  have hm1 := totalDegree_mul (poly w (G 1)) z
  have hm2 := totalDegree_mul (poly w (G 2)) (z^2)
  have hs := totalDegree_add (poly w (G 0)) (poly w (G 1)*z)
  have hs' := totalDegree_add (poly w (G 0)+poly w (G 1)*z) (poly w (G 2)*z^2)
  unfold gridPoly
  omega

def shiftW (s : Coeff K) (G : Grid K) : Grid K := fun j => rotate s (G j)
def shiftZ (s : Coeff K) (G : Grid K) : Grid K := fun j =>
  if j=0 then s 2 • G 2 else if j=1 then G 0-s 1 • G 2 else G 1+s 0 • G 2

theorem shiftW_identity (s : Coeff K) (G : Grid K) (w z : MvPolynomial σ K) :
    w*gridPoly w z G-poly z (fun j => G j 2)*cubic w s =
      gridPoly w z (shiftW s G) := by
  simp [gridPoly, shiftW, poly, rotate, cubic]
  ring

theorem shiftZ_identity (s : Coeff K) (G : Grid K) (w z : MvPolynomial σ K) :
    z*gridPoly w z G-poly w (G 2)*cubic z s =
      gridPoly w z (shiftZ s G) := by
  simp [gridPoly, shiftZ, poly, cubic, Pi.add_apply, Pi.sub_apply, Pi.smul_apply,
    smul_eq_mul, map_add, map_sub, map_mul]
  ring

theorem shiftW_val (s : Coeff K) (G : Grid K) (a b : K)
    (ha : scalarCubic s a = 0) :
    gridVal (shiftW s G) a b = a*gridVal G a b := by
  simp only [gridVal, shiftW, rotate_val s _ a ha]
  ring

theorem shiftZ_val (s : Coeff K) (G : Grid K) (a b : K)
    (hb : scalarCubic s b = 0) :
    gridVal (shiftZ s G) a b = b*gridVal G a b := by
  have h := rotate_val s (fun j => val (G j) a) b hb
  simp [gridVal, shiftZ, val, rotate, Pi.add_apply, Pi.sub_apply,
    Pi.smul_apply, smul_eq_mul] at h ⊢
  linear_combination h

theorem eq_of_gridValues (G H : Grid K) (a b c : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (h : ∀ x ∈ ({a,b,c} : Set K), ∀ y ∈ ({a,b,c} : Set K),
      gridVal G x y = gridVal H x y) : G = H := by
  have hz (x : K) (hx : x ∈ ({a,b,c} : Set K)) :
      (fun j => val (G j) x) = (fun j => val (H j) x) := by
    apply eq_of_three_values _ _ a b c hab hac hbc
    · exact h x hx a (by simp)
    · exact h x hx b (by simp)
    · exact h x hx c (by simp)
  funext j
  apply eq_of_three_values _ _ a b c hab hac hbc
  · exact congrFun (hz a (by simp)) j
  · exact congrFun (hz b (by simp)) j
  · exact congrFun (hz c (by simp)) j

def oneGrid : Grid K := fun j => if j=0 then oneCoeff else 0
@[simp] theorem gridVal_one (a b : K) : gridVal (oneGrid : Grid K) a b = 1 := by
  simp [gridVal, oneGrid]
@[simp] theorem gridPoly_one (w z : MvPolynomial σ K) :
    gridPoly w z (oneGrid : Grid K) = 1 := by simp [gridPoly, oneGrid]

def tensor (A B : Coeff K) : Grid K := fun j => B j • A
theorem tensor_val (A B : Coeff K) (a b : K) :
    gridVal (tensor A B) a b = val A a*val B b := by
  simp [gridVal, tensor, val]; ring
theorem tensor_poly (A B : Coeff K) (w z : MvPolynomial σ K) :
    gridPoly w z (tensor A B) = poly w A*poly z B := by
  simp [gridPoly, tensor, poly_smul, poly, smul_eq_C_mul]; ring

def roots (a b c : K) : Fin 3 → K := fun i => if i=0 then a else if i=1 then b else c
def basis (a b c : K) (i : Fin 3) : Coeff K :=
  if i=0 then lagrange a b c else if i=1 then lagrange b a c else lagrange c a b

theorem basis_values (a b c : K) (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (i j : Fin 3) : val (basis a b c i) (roots a b c j) = if i=j then 1 else 0 := by
  fin_cases i <;> fin_cases j <;>
    simp [basis, roots, val_lagrange, sub_ne_zero.mpr hab, sub_ne_zero.mpr hac,
      sub_ne_zero.mpr hbc, sub_ne_zero.mpr hab.symm, sub_ne_zero.mpr hac.symm,
      sub_ne_zero.mpr hbc.symm]

def inverseGrid (G : Grid K) (a b c : K) : Grid K :=
  ∑ i : Fin 3, ∑ j : Fin 3,
    (gridVal G (roots a b c i) (roots a b c j))⁻¹ • tensor (basis a b c i) (basis a b c j)

theorem inverseGrid_value (G : Grid K) (a b c : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) (i j : Fin 3) :
    gridVal (inverseGrid G a b c) (roots a b c i) (roots a b c j) =
      (gridVal G (roots a b c i) (roots a b c j))⁻¹ := by
  change gridValMap (roots a b c i) (roots a b c j) _ = _
  simp only [inverseGrid, map_sum, map_smul]
  change (∑ k : Fin 3, ∑ l : Fin 3,
    (gridVal G (roots a b c k) (roots a b c l))⁻¹ *
      gridVal (tensor (basis a b c k) (basis a b c l)) (roots a b c i) (roots a b c j)) = _
  simp [tensor_val, basis_values a b c hab hac hbc]

end
end GridArithmetic
