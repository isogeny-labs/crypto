import AffineGrid

namespace StagedResultant
noncomputable section
open MvPolynomial CubicArithmetic CubicNormalization GridArithmetic GridClosure AffineGrid FilteredClosure

variable {K σ : Type*} [Field K]

def monic (t u : MvPolynomial σ K) (A B : Coeff K) : MvPolynomial σ K :=
  u^2+poly t A*u+poly t B
def fLeft (s A B : Coeff K) : Coeff K := (2 : K) • B-mulReduced s A A
def fRight (s C D : Coeff K) : Coeff K := mulReduced s C C-(2 : K) • D
def nF (s A B C D : Coeff K) : Grid K := sep (fLeft s A B) (fRight s C D)
def nG (s A B C D : Coeff K) : Grid K :=
  -embedW (mulReduced s A B)+embedZ (mulReduced s C D)+tensor B C-tensor A D

def affineQuotient (t u : MvPolynomial σ K) (s A B C D : Coeff K) : MvPolynomial σ K :=
  productQuotient t s A B*u+productQuotient t s C D

theorem affineQuotient_degree (t u : MvPolynomial σ K) (s A B C D : Coeff K)
    (ht : t.totalDegree ≤ 1) (hu : u.totalDegree ≤ 1) :
    (affineQuotient t u s A B C D).totalDegree ≤ 2 := by
  have h1 := productQuotient_degree t s A B ht
  have h2 := productQuotient_degree t s C D ht
  have hm := totalDegree_mul (productQuotient t s A B) u
  have hs := totalDegree_add (productQuotient t s A B*u) (productQuotient t s C D)
  unfold affineQuotient
  omega

theorem correction_derivable (S : Set (MvPolynomial σ K)) (t u : MvPolynomial σ K)
    (s A B C D : Coeff K) (ht : t.totalDegree ≤ 1) (hu : u.totalDegree ≤ 1)
    (hg : Derivable S 5 (cubic t s)) :
    Derivable S 5 (cubic t s*affineQuotient t u s A B C D) := by
  have hq := affineQuotient_degree t u s A B C D ht hu
  have hc := cubic_degree t s ht
  rw [mul_comm]
  exact Derivable.mul _ hg ((totalDegree_mul _ _).trans (by omega))

theorem normalized_N_identity (s A B C D : Coeff K) (w z u : MvPolynomial σ K) :
    MvPolynomial.C 2*(u*(monic w u A B-monic z u C D))-
      (poly w A-poly z C)*(monic w u A B+monic z u C D)-
      affine w z u (nF s A B C D) (nG s A B C D) =
    cubic z s*affineQuotient z u s C C C D-
      cubic w s*affineQuotient w u s A A A B := by
  simp only [monic, affine, nF, nG, fLeft, fRight, gridPoly_add, gridPoly_neg,
    gridPoly_sub, sep_poly, embedW_poly, embedZ_poly, tensor_poly, poly_add,
    poly_sub, poly_smul, smul_eq_C_mul, affineQuotient, map_ofNat]
  linear_combination -u*(product_identity s A A w)-(product_identity s A B w)+
    u*(product_identity s C C z)+(product_identity s C D z)

theorem normalized_N_derivable (S : Set (MvPolynomial σ K)) (s A B C D : Coeff K)
    (w z u : MvPolynomial σ K) (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1)
    (hu : u.totalDegree ≤ 1) (hgw : Derivable S 5 (cubic w s))
    (hgz : Derivable S 5 (cubic z s)) (h1 : Derivable S 5 (monic w u A B))
    (h2 : Derivable S 5 (monic z u C D)) :
    Derivable S 5 (affine w z u (nF s A B C D) (nG s A B C D)) := by
  have hd1 := monic_degree w u A B hw hu
  have hd2 := monic_degree z u C D hz hu
  have hL := (totalDegree_sub (monic w u A B) (monic z u C D)).trans (max_le hd1 hd2)
  have hSum := (totalDegree_add (monic w u A B) (monic z u C D)).trans (max_le hd1 hd2)
  have hE := (totalDegree_sub (poly w A) (poly z C)).trans
    (max_le (poly_degree w A hw) (poly_degree z C hz))
  have hraw := FilteredClosure.sub
    (Derivable.smul (2 : K) (Derivable.mul u (FilteredClosure.sub h1 h2)
      ((totalDegree_mul _ _).trans (by omega))))
    (Derivable.mul (poly w A-poly z C) (Derivable.add h1 h2)
      ((totalDegree_mul _ _).trans (by omega)))
  have hcor := FilteredClosure.sub
    (correction_derivable S z u s C C C D hz hu hgz)
    (correction_derivable S w u s A A A B hw hu hgw)
  have hid := normalized_N_identity s A B C D w z u
  have heq : (2 : K) • (u*(monic w u A B-monic z u C D))-
      (poly w A-poly z C)*(monic w u A B+monic z u C D)-
      (cubic z s*affineQuotient z u s C C C D-
       cubic w s*affineQuotient w u s A A A B) =
      affine w z u (nF s A B C D) (nG s A B C D) := by
    simp only [smul_eq_C_mul]
    linear_combination hid
  rw [← heq]
  exact FilteredClosure.sub hraw hcor

theorem sepTimes_derivable (S : Set (MvPolynomial σ K)) (s A C Fw Fz : Coeff K)
    (G : Grid K) (w z u : MvPolynomial σ K) (hw : w.totalDegree ≤ 1)
    (hz : z.totalDegree ≤ 1) (hu : u.totalDegree ≤ 1)
    (hgw : Derivable S 5 (cubic w s)) (hgz : Derivable S 5 (cubic z s))
    (hN : Derivable S 5 (affine w z u (sep Fw Fz) G)) :
    Derivable S 5 (affine w z u (sepTimes s A C (sep Fw Fz)) (sepTimes s A C G)) := by
  have h0 : sep Fw Fz 2 2 = 0 := by simp [sep,embedW,embedZ]
  have hW : shiftW s (sep Fw Fz) 2 2 = 0 := by
    simp [shiftW,rotate,sep,embedW,embedZ]
  have hZ : shiftZ s (sep Fw Fz) 2 2 = 0 := by
    simp [shiftZ,sep,embedW,embedZ,Pi.add_apply,Pi.smul_apply]
  have hwN := affine_shiftW_derivable S s _ G w z u hw hz hu h0 hgw hN
  have hwwN := affine_shiftW_derivable S s _ _ w z u hw hz hu hW hgw hwN
  have hzN := affine_shiftZ_derivable S s _ G w z u hw hz hu h0 hgz hN
  have hzzN := affine_shiftZ_derivable S s _ _ w z u hw hz hu hZ hgz hzN
  have hlin := FilteredClosure.sub
    (Derivable.add (Derivable.add (Derivable.smul (A 0) hN) (Derivable.smul (A 1) hwN))
      (Derivable.smul (A 2) hwwN))
    (Derivable.add (Derivable.add (Derivable.smul (C 0) hN) (Derivable.smul (C 1) hzN))
      (Derivable.smul (C 2) hzzN))
  convert hlin using 1
  simp only [affine, sepTimes, gridPoly_add, gridPoly_sub, gridPoly_smul, smul_eq_C_mul]
  ring

theorem sep_multiplier_identity (s Fw Fz A C B D : Coeff K)
    (w z u : MvPolynomial σ K) :
    gridPoly w z (sep Fw Fz)*affine w z u (sep A C) (sep B D)-
      affine w z u (sepProduct s Fw Fz A C) (sepProduct s Fw Fz B D) =
      cubic w s*affineQuotient w u s Fw A Fw B+
      cubic z s*affineQuotient z u s Fz C Fz D := by
  simp only [affine, sep_poly, affineQuotient]
  linear_combination u*(sepProduct_identity s Fw Fz A C w z)+
    sepProduct_identity s Fw Fz B D w z

theorem sep_multiplier_derivable (S : Set (MvPolynomial σ K)) (s Fw Fz A C B D : Coeff K)
    (w z u : MvPolynomial σ K) (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1)
    (hu : u.totalDegree ≤ 1) (hgw : Derivable S 5 (cubic w s))
    (hgz : Derivable S 5 (cubic z s))
    (hL : Derivable S 5 (affine w z u (sep A C) (sep B D))) :
    Derivable S 5 (affine w z u (sepProduct s Fw Fz A C) (sepProduct s Fw Fz B D)) := by
  have hf := sep_degree w z Fw Fz hw hz
  have he := sep_degree w z A C hw hz
  have hj := sep_degree w z B D hw hz
  have hm := totalDegree_mul (gridPoly w z (sep A C)) u
  have hl := totalDegree_add (gridPoly w z (sep A C)*u) (gridPoly w z (sep B D))
  have hraw := Derivable.mul (gridPoly w z (sep Fw Fz)) hL
    ((totalDegree_mul _ _).trans (by change _ + (affine w z u (sep A C) (sep B D)).totalDegree ≤ 5; unfold affine; omega))
  have hcor := Derivable.add
    (correction_derivable S w u s Fw A Fw B hw hu hgw)
    (correction_derivable S z u s Fz C Fz D hz hu hgz)
  have hid := sep_multiplier_identity s Fw Fz A C B D w z u
  have heq : gridPoly w z (sep Fw Fz)*affine w z u (sep A C) (sep B D)-
      (cubic w s*affineQuotient w u s Fw A Fw B+
       cubic z s*affineQuotient z u s Fz C Fz D) =
      affine w z u (sepProduct s Fw Fz A C) (sepProduct s Fw Fz B D) := by
    linear_combination hid
  rw [← heq]
  exact FilteredClosure.sub hraw hcor

def quadRes (A B C D : K) : K :=
  (B-D)^2-A*(A-C)*(B-D)+B*(A-C)^2

theorem quadRes_split (ρ τ C D : K) :
    quadRes (-(ρ+τ)) (ρ*τ) C D = (ρ^2+C*ρ+D)*(τ^2+C*τ+D) := by
  unfold quadRes
  ring

theorem nG_value (s A B C D : Coeff K) (a b : K)
    (ha : scalarCubic s a = 0) (hb : scalarCubic s b = 0) :
    gridVal (nG s A B C D) a b = -(val A a-val C b)*(val B a+val D b) := by
  simp only [nG, gridVal_sub, gridVal_add, gridVal_neg, embedW_val, embedZ_val,
    tensor_val, mulReduced_val s A B a ha, mulReduced_val s C D b hb]
  ring

def resultGrid (s A B C D : Coeff K) : Grid K :=
  sepTimes s A C (nG s A B C D)-
    sepProduct s (fLeft s A B) (fRight s C D) B (-D)

theorem resultGrid_value (s A B C D : Coeff K) (a b : K)
    (ha : scalarCubic s a = 0) (hb : scalarCubic s b = 0) :
    gridVal (resultGrid s A B C D) a b = -2*quadRes (val A a) (val B a) (val C b) (val D b) := by
  rw [resultGrid, gridVal_sub, sepTimes_value s A C _ a b ha hb,
    sepProduct_value s (fLeft s A B) (fRight s C D) B (-D) a b ha hb,
    nG_value s A B C D a b ha hb]
  simp only [fLeft, fRight, val_sub, val_smul, val_neg,
    mulReduced_val s A A a ha, mulReduced_val s C C b hb, quadRes]
  ring

theorem root_in_set (a b c x : K) (hx : x ∈ ({a,b,c} : Set K)) :
    scalarCubic (rootsCoefficients a b c) x = 0 := by
  rcases hx with rfl | rfl | rfl <;> simp [scalarCubic_roots]

/-- Staged extraction of a scalar resultant. This theorem constructs an
actual degree-five closure derivation, without a resultant-nonzero premise. -/
theorem resultGrid_derivable (S : Set (MvPolynomial σ K)) (A B C D : Coeff K)
    (a b c : K) (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (w z u : MvPolynomial σ K) (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1)
    (hu : u.totalDegree ≤ 1)
    (hgw : Derivable S 5 (cubic w (rootsCoefficients a b c)))
    (hgz : Derivable S 5 (cubic z (rootsCoefficients a b c)))
    (h1 : Derivable S 5 (monic w u A B)) (h2 : Derivable S 5 (monic z u C D)) :
    Derivable S 5 (gridPoly w z (resultGrid (rootsCoefficients a b c) A B C D)) := by
  let s := rootsCoefficients a b c
  have hn := normalized_N_derivable S s A B C D w z u hw hz hu hgw hgz h1 h2
  have he := sepTimes_derivable S s A C (fLeft s A B) (fRight s C D)
    (nG s A B C D) w z u hw hz hu hgw hgz hn
  have hL : Derivable S 5 (affine w z u (sep A (-C)) (sep B (-D))) := by
    convert FilteredClosure.sub h1 h2 using 1
    simp only [affine, sep_poly, poly_neg, monic]
    ring
  have hf := sep_multiplier_derivable S s (fLeft s A B) (fRight s C D)
    A (-C) B (-D) w z u hw hz hu hgw hgz hL
  have hcoef : sepTimes s A C (nF s A B C D) =
      sepProduct s (fLeft s A B) (fRight s C D) A (-C) := by
    apply eq_of_gridValues _ _ a b c hab hac hbc
    intro x hx y hy
    have hx0 := root_in_set a b c x hx
    have hy0 := root_in_set a b c y hy
    rw [sepTimes_value s A C _ x y hx0 hy0,
      sepProduct_value s (fLeft s A B) (fRight s C D) A (-C) x y hx0 hy0]
    simp only [nF, sep_val, val_neg]
    ring
  have heq : affine w z u (sepTimes s A C (nF s A B C D)) (sepTimes s A C (nG s A B C D))-
      affine w z u (sepProduct s (fLeft s A B) (fRight s C D) A (-C))
        (sepProduct s (fLeft s A B) (fRight s C D) B (-D)) =
      gridPoly w z (resultGrid s A B C D) := by
    unfold affine
    rw [hcoef]
    simp only [resultGrid, gridPoly_sub]
    ring
  rw [← heq]
  exact FilteredClosure.sub he hf

/-- The complete algebraic degree-five upper construction for normalized
quadratic families with a nowhere-zero scalar resultant on the split grid.
The elliptic-curve application must prove the displayed scalar condition. -/
theorem monic_refutation (S : Set (MvPolynomial σ K)) (A B C D : Coeff K)
    (a b c : K) (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) (hchar : (2 : K) ≠ 0)
    (w z u : MvPolynomial σ K) (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1)
    (hu : u.totalDegree ≤ 1)
    (hgw : Derivable S 5 (cubic w (rootsCoefficients a b c)))
    (hgz : Derivable S 5 (cubic z (rootsCoefficients a b c)))
    (h1 : Derivable S 5 (monic w u A B)) (h2 : Derivable S 5 (monic z u C D))
    (hne : ∀ x ∈ ({a,b,c} : Set K), ∀ y ∈ ({a,b,c} : Set K),
      quadRes (val A x) (val B x) (val C y) (val D y) ≠ 0) : Derivable S 5 1 := by
  apply grid_refutation S (resultGrid (rootsCoefficients a b c) A B C D)
    a b c hab hac hbc w z hw hz hgw hgz
    (resultGrid_derivable S A B C D a b c hab hac hbc w z u hw hz hu hgw hgz h1 h2)
  intro x hx y hy
  rw [resultGrid_value _ A B C D x y (root_in_set a b c x hx) (root_in_set a b c y hy)]
  exact mul_ne_zero (neg_ne_zero.mpr hchar) (hne x hx y hy)

end
end StagedResultant

#print axioms StagedResultant.resultGrid_derivable
#print axioms StagedResultant.monic_refutation
