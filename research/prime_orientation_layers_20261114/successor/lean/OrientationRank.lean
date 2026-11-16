module
public import OrientationLeading
public import OrientationRefinement
public import Mathlib.LinearAlgebra.Matrix.Rank
import all OrientationAnchor
import all OrientationValuation
import all OrientationContent
import all OrientationProduct
import all OrientationHierarchy
import all OrientationResidue
import all OrientationDiagonal
import all OrientationJet
import all OrientationLeading
public section
noncomputable section
namespace OrientationRank
open Polynomial Finset OrientationAnchor OrientationResidue OrientationDiagonal OrientationJet OrientationLeading

theorem integral_prod {ι : Type*} (S : Finset ι) (f : ι → ℚ)
    (hf : ∀ j ∈ S, VBound 0 (f j)) : VBound 0 (∏ j ∈ S,f j) := by
  classical
  induction S using Finset.induction_on with
  | empty => simp [VBound]
  | @insert j S hj ih =>
    rw [prod_insert hj]
    simpa only [add_zero] using (hf j (mem_insert_self _ _)).mul
      (ih (by intro z hz; exact hf z (mem_insert_of_mem hz)))

theorem prod_difference {ι : Type*} (S : Finset ι) (f g : ι → ℚ)
    (hf : ∀ j ∈ S, VBound 0 (f j)) (hg : ∀ j ∈ S, VBound 0 (g j))
    (hd : ∀ j ∈ S, VBound 1 (f j-g j)) :
    VBound 1 ((∏ j ∈ S,f j)-(∏ j ∈ S,g j)) := by
  classical
  induction S using Finset.induction_on with
  | empty => simp [VBound]
  | @insert j S hj ih =>
    have hs (z : ι) (hz : z ∈ S) : z ∈ insert j S := mem_insert_of_mem hz
    have hp := integral_prod S f (by intro z hz; exact hf z (hs z hz))
    have hi := ih (by intro z hz; exact hf z (hs z hz))
      (by intro z hz; exact hg z (hs z hz)) (by intro z hz; exact hd z (hs z hz))
    have h1 : VBound 1 ((f j-g j)*(∏ z ∈ S,f z)) := by
      simpa only [add_zero] using (hd j (mem_insert_self _ _)).mul hp
    have h2 : VBound 1 (g j*((∏ z ∈ S,f z)-(∏ z ∈ S,g z))) := by
      simpa only [zero_add] using (hg j (mem_insert_self _ _)).mul hi
    rw [prod_insert hj,prod_insert hj]
    convert h1.add h2 using 1
    ring

theorem integer_integral (z : ℤ) : VBound 0 (z:ℚ) := by
  right
  rw [padicValRat.of_int]
  exact Nat.cast_nonneg _

theorem determinant_difference {ι : Type*} [Fintype ι] [DecidableEq ι]
    (A B : Matrix ι ι ℚ) (hA : ∀ r s,VBound 0 (A r s))
    (hB : ∀ r s,VBound 0 (B r s)) (h : ∀ r s,VBound 1 (A r s-B r s)) :
    VBound 1 (A.det-B.det) := by
  rw [Matrix.det_apply',Matrix.det_apply',← sum_sub_distrib]
  apply VBound.sum
  intro σ _
  have hp := prod_difference univ (fun j => A (σ j) j) (fun j => B (σ j) j)
    (by intro j _; exact hA _ _) (by intro j _; exact hB _ _)
    (by intro j _; exact h _ _)
  have hh : VBound 1 (((Equiv.Perm.sign σ:ℤ):ℚ)*
      ((∏ j,A (σ j) j)-(∏ j,B (σ j) j))) := by
    simpa only [zero_add] using (integer_integral (Equiv.Perm.sign σ)).mul hp
  convert hh using 1
  ring

theorem identity_integral {ι : Type*} [DecidableEq ι] (r s : ι) :
    VBound 0 ((1:Matrix ι ι ℚ) r s) := by
  simp only [Matrix.one_apply]
  split_ifs <;> simp [VBound]

theorem near_identity_integral {ι : Type*} [DecidableEq ι]
    (A : Matrix ι ι ℚ) (h : ∀ r s,VBound 1 (A r s-(1:Matrix ι ι ℚ) r s)) :
    ∀ r s,VBound 0 (A r s) := by
  intro r s
  simpa only [sub_add_cancel] using ((h r s).mono (by decide)).add (identity_integral r s)

theorem near_identity_det_nonzero {ι : Type*} [Fintype ι] [DecidableEq ι]
    (A : Matrix ι ι ℚ) (h : ∀ r s,VBound 1 (A r s-(1:Matrix ι ι ℚ) r s)) : A.det ≠ 0 := by
  have hh := determinant_difference A 1 (near_identity_integral A h) identity_integral h
  rw [Matrix.det_one] at hh
  intro hz
  rw [hz] at hh
  norm_num [VBound,padicValRat.neg] at hh

/-- One literal source port per retained residue class. -/
def classPort (a u : ℕ) (hu : u ≤ a) (r : Labels u) : Ports a :=
  ⟨r.val,lt_of_lt_of_le r.isLt (Nat.pow_le_pow_right (by decide) hu)⟩

def classMatrix (a u i : ℕ) (hu : u ≤ a) (m : ℕ → ℕ) : Matrix (Labels u) (Labels u) ℚ :=
  fun r s => normalizedCrossing a u i m (classPort a u hu r) (classPort a u hu s)

theorem actual_class_identity (a u i : ℕ) (hu : u ≤ a) (m : ℕ → ℕ)
    (hi : i < u) (hm : ∀ j < u,0 < m j) (r s : Labels u) :
    VBound 1 (classMatrix a u i hu m r s-(1:Matrix (Labels u) (Labels u) ℚ) r s) := by
  have h := actual_leading_factorization a u i m hi hu hm (classPort a u hu r) (classPort a u hu s)
  rw [factor_entry] at h
  simpa only [classMatrix,classPort,Nat.mod_eq_of_lt r.isLt,Nat.mod_eq_of_lt s.isLt,
    Matrix.one_apply,Fin.val_inj] using h

theorem actual_class_det_nonzero (a u i : ℕ) (hu : u ≤ a) (m : ℕ → ℕ)
    (hi : i < u) (hm : ∀ j < u,0 < m j) : (classMatrix a u i hu m).det ≠ 0 :=
  near_identity_det_nonzero _ (actual_class_identity a u i hu m hi hm)


theorem actual_class_rank (a u i : ℕ) (hu : u ≤ a) (m : ℕ → ℕ)
    (hi : i < u) (hm : ∀ j < u,0 < m j) : (classMatrix a u i hu m).rank = 2^u := by
  simpa only [Fintype.card_fin] using Matrix.rank_of_det_ne_zero
    (actual_class_det_nonzero a u i hu m hi hm)

/-- Any rational factorization to this exact precision needs at least 2^u columns.
No integrality premise on either supplied factor is needed. -/
theorem actual_approximation_label_lower_bound (a u i k : ℕ) (hu : u ≤ a) (m : ℕ → ℕ)
    (hi : i < u) (hm : ∀ j < u,0 < m j)
    (A : Matrix (Ports a) (Fin k) ℚ) (B : Matrix (Fin k) (Ports a) ℚ)
    (happrox : ∀ x y,VBound 1 (normalizedCrossing a u i m x y-(A*B) x y)) :
    2^u ≤ k := by
  let Ar : Matrix (Labels u) (Fin k) ℚ := A.submatrix (classPort a u hu) id
  let Br : Matrix (Fin k) (Labels u) ℚ := B.submatrix id (classPort a u hu)
  let N : Matrix (Labels u) (Labels u) ℚ := Ar*Br
  have he (r s : Labels u) : N r s = (A*B) (classPort a u hu r) (classPort a u hu s) := by
    simp only [N,Ar,Br,Matrix.mul_apply,Matrix.submatrix_apply,id_eq]
  have hn (r s : Labels u) : VBound 1 (N r s-(1:Matrix (Labels u) (Labels u) ℚ) r s) := by
    have h1 : VBound 1 (N r s-classMatrix a u i hu m r s) := by
      rw [he]
      simpa only [classMatrix,neg_sub] using
        (happrox (classPort a u hu r) (classPort a u hu s)).neg
    have h2 := actual_class_identity a u i hu m hi hm r s
    convert h1.add h2 using 1
    ring
  have hd := near_identity_det_nonzero N hn
  have hr : N.rank = 2^u := by
    simpa only [Fintype.card_fin] using Matrix.rank_of_det_ne_zero hd
  have hk : N.rank ≤ k :=
    (Matrix.rank_mul_le_left Ar Br).trans (Matrix.rank_le_width Ar)
  omega


def scaledLeft (a u i k : ℕ) (m : ℕ → ℕ) (A : Matrix (Ports a) (Fin k) ℚ) :
    Matrix (Ports a) (Fin k) ℚ := fun x r => A x r/diagonalJet a u i m

theorem scaled_product (a u i k : ℕ) (m : ℕ → ℕ)
    (A : Matrix (Ports a) (Fin k) ℚ) (B : Matrix (Fin k) (Ports a) ℚ)
    (x y : Ports a) :
    (scaledLeft a u i k m A*B) x y = (A*B) x y/diagonalJet a u i m := by
  simp only [Matrix.mul_apply,scaledLeft,div_mul_eq_mul_div,Finset.sum_div]

/-- The same lower bound with the actual absolute first-jet precision charge. -/
theorem actual_weighted_label_lower_bound (a u i k : ℕ) (hu : u ≤ a) (m : ℕ → ℕ)
    (hi : i < u) (hm : ∀ j < u,0 < m j)
    (A : Matrix (Ports a) (Fin k) ℚ) (B : Matrix (Fin k) (Ports a) ℚ)
    (happrox : ∀ x y,VBound (jetWeight a u i m+1)
      (crossingFirst a u i m x y-(A*B) x y)) : 2^u ≤ k := by
  apply actual_approximation_label_lower_bound a u i k hu m hi hm (scaledLeft a u i k m A) B
  intro x y
  have hh := vbound_div (happrox x y) (r:=diagonalJet a u i m)
  have hv : padicValRat 2 (diagonalJet a u i m) = jetWeight a u i m :=
    diagonalJet_valuation a u i m hi hu
  rw [hv,show jetWeight a u i m+1-jetWeight a u i m=1 by ring] at hh
  have he : (crossingFirst a u i m x y-(A*B) x y)/diagonalJet a u i m =
      normalizedCrossing a u i m x y-(scaledLeft a u i k m A*B) x y := by
    rw [scaled_product,crossing_first_scale a u i m (by omega)]
    field_simp [diagonalJet_ne_zero a u i m (by omega)]
  rwa [he] at hh

end OrientationRank
