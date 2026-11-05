import GridClosure

namespace AffineGrid
noncomputable section
open MvPolynomial CubicArithmetic GridArithmetic GridClosure FilteredClosure

variable {K σ : Type*} [Field K]

def embedW (A : Coeff K) : Grid K := fun j => if j=0 then A else 0
def embedZ (A : Coeff K) : Grid K := fun j i => if i=0 then A j else 0
def sep (A B : Coeff K) : Grid K := embedW A+embedZ B

@[simp] theorem poly_neg (t : MvPolynomial σ K) (A : Coeff K) :
    poly t (-A) = -poly t A := by simp [poly, Pi.neg_apply]; ring
@[simp] theorem poly_sub (t : MvPolynomial σ K) (A B : Coeff K) :
    poly t (A-B) = poly t A-poly t B := by simp [sub_eq_add_neg]
@[simp] theorem val_neg (A : Coeff K) (t : K) : val (-A) t = -val A t := by
  simp [val, Pi.neg_apply]; ring
@[simp] theorem val_sub (A B : Coeff K) (t : K) : val (A-B) t = val A t-val B t := by
  simp [sub_eq_add_neg]
@[simp] theorem gridPoly_neg (w z : MvPolynomial σ K) (G : Grid K) :
    gridPoly w z (-G) = -gridPoly w z G := by simp [gridPoly, Pi.neg_apply]; ring
@[simp] theorem gridPoly_sub (w z : MvPolynomial σ K) (G H : Grid K) :
    gridPoly w z (G-H) = gridPoly w z G-gridPoly w z H := by simp [sub_eq_add_neg]
@[simp] theorem gridVal_neg (G : Grid K) (a b : K) :
    gridVal (-G) a b = -gridVal G a b := by simp [gridVal, Pi.neg_apply]; ring
@[simp] theorem gridVal_sub (G H : Grid K) (a b : K) :
    gridVal (G-H) a b = gridVal G a b-gridVal H a b := by simp [sub_eq_add_neg]

@[simp] theorem embedW_poly (w z : MvPolynomial σ K) (A : Coeff K) :
    gridPoly w z (embedW A) = poly w A := by simp [gridPoly, embedW]
@[simp] theorem embedZ_poly (w z : MvPolynomial σ K) (A : Coeff K) :
    gridPoly w z (embedZ A) = poly z A := by simp [gridPoly, embedZ, poly]
@[simp] theorem sep_poly (w z : MvPolynomial σ K) (A B : Coeff K) :
    gridPoly w z (sep A B) = poly w A+poly z B := by simp [sep]
@[simp] theorem embedW_val (a b : K) (A : Coeff K) : gridVal (embedW A) a b = val A a := by
  simp [gridVal, embedW]
@[simp] theorem embedZ_val (a b : K) (A : Coeff K) : gridVal (embedZ A) a b = val A b := by
  simp [gridVal, embedZ, val]
@[simp] theorem sep_val (a b : K) (A B : Coeff K) :
    gridVal (sep A B) a b = val A a+val B b := by simp [sep]

theorem sep_degree (w z : MvPolynomial σ K) (A B : Coeff K)
    (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1) :
    (gridPoly w z (sep A B)).totalDegree ≤ 2 := by
  rw [sep_poly]
  exact (totalDegree_add _ _).trans (max_le (poly_degree w A hw) (poly_degree z B hz))

/-- A reduced grid with no w²z² term has total degree at most three. -/
theorem grid_degree_three (w z : MvPolynomial σ K) (G : Grid K)
    (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1) (hG : G 2 2 = 0) :
    (gridPoly w z G).totalDegree ≤ 3 := by
  have h0 := poly_degree w (G 0) hw
  have h1 := poly_degree w (G 1) hw
  have h2 := poly_degree_one w (G 2) hw hG
  have hz2 := totalDegree_pow z 2
  have hm1 := totalDegree_mul (poly w (G 1)) z
  have hm2 := totalDegree_mul (poly w (G 2)) (z^2)
  have hs := totalDegree_add (poly w (G 0)) (poly w (G 1)*z)
  have hs' := totalDegree_add (poly w (G 0)+poly w (G 1)*z) (poly w (G 2)*z^2)
  unfold gridPoly
  omega

def affine (w z u : MvPolynomial σ K) (F G : Grid K) : MvPolynomial σ K :=
  gridPoly w z F*u+gridPoly w z G

theorem affine_degree (w z u : MvPolynomial σ K) (F G : Grid K)
    (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1) (hu : u.totalDegree ≤ 1)
    (hF : F 2 2 = 0) : (affine w z u F G).totalDegree ≤ 4 := by
  have hf := grid_degree_three w z F hw hz hF
  have hg := gridPoly_degree w z G hw hz
  exact (totalDegree_add _ _).trans
    (max_le ((totalDegree_mul _ _).trans (by omega)) hg)

theorem affine_shiftW_identity (s : Coeff K) (F G : Grid K) (w z u : MvPolynomial σ K) :
    w*affine w z u F G-
      (poly z (fun j => F j 2)*u+poly z (fun j => G j 2))*cubic w s =
      affine w z u (shiftW s F) (shiftW s G) := by
  unfold affine
  linear_combination u*(shiftW_identity s F w z)+shiftW_identity s G w z

theorem affine_shiftZ_identity (s : Coeff K) (F G : Grid K) (w z u : MvPolynomial σ K) :
    z*affine w z u F G-
      (poly w (F 2)*u+poly w (G 2))*cubic z s =
      affine w z u (shiftZ s F) (shiftZ s G) := by
  unfold affine
  linear_combination u*(shiftZ_identity s F w z)+shiftZ_identity s G w z

theorem affine_shiftW_derivable (S : Set (MvPolynomial σ K)) (s : Coeff K) (F G : Grid K)
    (w z u : MvPolynomial σ K) (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1)
    (hu : u.totalDegree ≤ 1) (hF : F 2 2 = 0)
    (hc : Derivable S 5 (cubic w s)) (h : Derivable S 5 (affine w z u F G)) :
    Derivable S 5 (affine w z u (shiftW s F) (shiftW s G)) := by
  have hh := affine_degree w z u F G hw hz hu hF
  have hrF := poly_degree_one z (fun j => F j 2) hz hF
  have hrG := poly_degree z (fun j => G j 2) hz
  have hm := totalDegree_mul (poly z (fun j => F j 2)) u
  have hs := totalDegree_add (poly z (fun j => F j 2)*u) (poly z (fun j => G j 2))
  have hcor := cubic_degree w s hw
  rw [← affine_shiftW_identity]
  exact FilteredClosure.sub
    (Derivable.mul w h ((totalDegree_mul _ _).trans (by omega)))
    (Derivable.mul _ hc ((totalDegree_mul _ _).trans (by omega)))

theorem affine_shiftZ_derivable (S : Set (MvPolynomial σ K)) (s : Coeff K) (F G : Grid K)
    (w z u : MvPolynomial σ K) (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1)
    (hu : u.totalDegree ≤ 1) (hF : F 2 2 = 0)
    (hc : Derivable S 5 (cubic z s)) (h : Derivable S 5 (affine w z u F G)) :
    Derivable S 5 (affine w z u (shiftZ s F) (shiftZ s G)) := by
  have hh := affine_degree w z u F G hw hz hu hF
  have hrF := poly_degree_one w (F 2) hw hF
  have hrG := poly_degree w (G 2) hw
  have hm := totalDegree_mul (poly w (F 2)) u
  have hs := totalDegree_add (poly w (F 2)*u) (poly w (G 2))
  have hcor := cubic_degree z s hz
  rw [← affine_shiftZ_identity]
  exact FilteredClosure.sub
    (Derivable.mul z h ((totalDegree_mul _ _).trans (by omega)))
    (Derivable.mul _ hc ((totalDegree_mul _ _).trans (by omega)))

def sepProduct (s A C B D : Coeff K) : Grid K :=
  embedW (mulReduced s A B)+tensor A D+tensor B C+embedZ (mulReduced s C D)

theorem sepProduct_identity (s A C B D : Coeff K) (w z : MvPolynomial σ K) :
    (poly w A+poly z C)*(poly w B+poly z D)-gridPoly w z (sepProduct s A C B D) =
      cubic w s*productQuotient w s A B+cubic z s*productQuotient z s C D := by
  simp only [sepProduct, gridPoly_add, embedW_poly, embedZ_poly, tensor_poly]
  linear_combination product_identity s A B w+product_identity s C D z

theorem sepProduct_value (s A C B D : Coeff K) (a b : K)
    (ha : scalarCubic s a = 0) (hb : scalarCubic s b = 0) :
    gridVal (sepProduct s A C B D) a b =
      (val A a+val C b)*(val B a+val D b) := by
  simp only [sepProduct, gridVal_add, embedW_val, embedZ_val, tensor_val,
    mulReduced_val s A B a ha, mulReduced_val s C D b hb]
  ring

def sepTimes (s A C : Coeff K) (G : Grid K) : Grid K :=
  A 0 • G+A 1 • shiftW s G+A 2 • shiftW s (shiftW s G)-
    (C 0 • G+C 1 • shiftZ s G+C 2 • shiftZ s (shiftZ s G))

theorem sepTimes_value (s A C : Coeff K) (G : Grid K) (a b : K)
    (ha : scalarCubic s a = 0) (hb : scalarCubic s b = 0) :
    gridVal (sepTimes s A C G) a b = (val A a-val C b)*gridVal G a b := by
  simp only [sepTimes, gridVal_sub, gridVal_add, gridVal_smul,
    shiftW_val s _ a b ha, shiftZ_val s _ a b hb]
  unfold val
  ring

end
end AffineGrid
