import GridArithmetic

namespace GridClosure
noncomputable section
open MvPolynomial CubicArithmetic GridArithmetic FilteredClosure

variable {K σ : Type*} [Field K]

theorem sum_derivable {ι : Type*} (F : Set (MvPolynomial σ K)) (d : ℕ)
    (s : Finset ι) (f : ι → MvPolynomial σ K)
    (h : ∀ i ∈ s, Derivable F d (f i)) : Derivable F d (∑ i ∈ s, f i) := by
  classical
  induction s using Finset.induction_on with
  | empty => simpa using (Derivable.zero : Derivable F d 0)
  | @insert i s hi ih =>
    rw [Finset.sum_insert hi]
    exact Derivable.add (h i (by simp)) (ih (by intros; apply h; simp_all))

theorem shiftW_derivable (F : Set (MvPolynomial σ K)) (s : Coeff K) (G : Grid K)
    (w z : MvPolynomial σ K) (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1)
    (hg : Derivable F 5 (cubic w s)) (hG : Derivable F 5 (gridPoly w z G)) :
    Derivable F 5 (gridPoly w z (shiftW s G)) := by
  have hd := gridPoly_degree w z G hw hz
  have hrow := poly_degree z (fun j => G j 2) hz
  have hc := cubic_degree w s hw
  rw [← shiftW_identity]
  exact FilteredClosure.sub
    (Derivable.mul w hG ((totalDegree_mul _ _).trans (by omega)))
    (Derivable.mul (poly z (fun j => G j 2)) hg ((totalDegree_mul _ _).trans (by omega)))

theorem shiftZ_derivable (F : Set (MvPolynomial σ K)) (s : Coeff K) (G : Grid K)
    (w z : MvPolynomial σ K) (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1)
    (hg : Derivable F 5 (cubic z s)) (hG : Derivable F 5 (gridPoly w z G)) :
    Derivable F 5 (gridPoly w z (shiftZ s G)) := by
  have hd := gridPoly_degree w z G hw hz
  have hrow := poly_degree w (G 2) hw
  have hc := cubic_degree z s hz
  rw [← shiftZ_identity]
  exact FilteredClosure.sub
    (Derivable.mul z hG ((totalDegree_mul _ _).trans (by omega)))
    (Derivable.mul (poly w (G 2)) hg ((totalDegree_mul _ _).trans (by omega)))

def shiftWPow (s : Coeff K) (G : Grid K) : ℕ → Grid K
  | 0 => G
  | n+1 => shiftW s (shiftWPow s G n)
def shiftZPow (s : Coeff K) (G : Grid K) : ℕ → Grid K
  | 0 => G
  | n+1 => shiftZ s (shiftZPow s G n)

theorem shiftWPow_value (s : Coeff K) (G : Grid K) (a b : K)
    (ha : scalarCubic s a = 0) (n : ℕ) :
    gridVal (shiftWPow s G n) a b = a^n*gridVal G a b := by
  induction n with
  | zero => simp [shiftWPow]
  | succ n ih => rw [shiftWPow, shiftW_val s _ a b ha, ih, pow_succ]; ring

theorem shiftZPow_value (s : Coeff K) (G : Grid K) (a b : K)
    (hb : scalarCubic s b = 0) (n : ℕ) :
    gridVal (shiftZPow s G n) a b = b^n*gridVal G a b := by
  induction n with
  | zero => simp [shiftZPow]
  | succ n ih => rw [shiftZPow, shiftZ_val s _ a b hb, ih, pow_succ]; ring

def gridProduct (s : Coeff K) (H G : Grid K) : Grid K :=
  ∑ i : Fin 3, ∑ j : Fin 3, H j i • shiftZPow s (shiftWPow s G i.val) j.val

theorem gridProduct_value (s : Coeff K) (H G : Grid K) (a b : K)
    (ha : scalarCubic s a = 0) (hb : scalarCubic s b = 0) :
    gridVal (gridProduct s H G) a b = gridVal H a b*gridVal G a b := by
  change gridValMap a b _ = _
  simp only [gridProduct, map_sum, map_smul]
  change (∑ i : Fin 3, ∑ j : Fin 3,
    H j i * gridVal (shiftZPow s (shiftWPow s G i.val) j.val) a b) = _
  simp only [shiftZPow_value s _ a b hb, shiftWPow_value s G a b ha]
  simp [Fin.sum_univ_succ, gridVal, val]
  ring

theorem gridProduct_derivable (F : Set (MvPolynomial σ K)) (s : Coeff K) (H G : Grid K)
    (w z : MvPolynomial σ K) (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1)
    (hgw : Derivable F 5 (cubic w s)) (hgz : Derivable F 5 (cubic z s))
    (hG : Derivable F 5 (gridPoly w z G)) :
    Derivable F 5 (gridPoly w z (gridProduct s H G)) := by
  have hW (n : ℕ) : Derivable F 5 (gridPoly w z (shiftWPow s G n)) := by
    induction n with
    | zero => exact hG
    | succ n ih => exact shiftW_derivable F s _ w z hw hz hgw ih
  have hZ (n m : ℕ) : Derivable F 5 (gridPoly w z (shiftZPow s (shiftWPow s G n) m)) := by
    induction m with
    | zero => exact hW n
    | succ m ih => exact shiftZ_derivable F s _ w z hw hz hgz ih
  change Derivable F 5 (gridPolyMap w z (∑ i : Fin 3, ∑ j : Fin 3,
    H j i • shiftZPow s (shiftWPow s G i.val) j.val))
  simp only [map_sum, map_smul]
  apply sum_derivable
  intro i _
  apply sum_derivable
  intro j _
  exact Derivable.smul (H j i) (hZ i.val j.val)

theorem roots_mem (a b c : K) (i : Fin 3) : roots a b c i ∈ ({a,b,c} : Set K) := by
  fin_cases i <;> simp [roots]

theorem root_index (a b c x : K) (hx : x ∈ ({a,b,c} : Set K)) :
    ∃ i : Fin 3, roots a b c i = x := by
  rcases hx with rfl | rfl | rfl
  · exact ⟨0, by simp [roots]⟩
  · exact ⟨1, by simp [roots]⟩
  · exact ⟨2, by simp [roots]⟩

/-- Constructive inversion on the scalar 3-by-3 grid. Every multiplication
is a scalar-grid shift with transient total degree at most five. -/
theorem grid_refutation (F : Set (MvPolynomial σ K)) (G : Grid K) (a b c : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (w z : MvPolynomial σ K) (hw : w.totalDegree ≤ 1) (hz : z.totalDegree ≤ 1)
    (hgw : Derivable F 5 (cubic w (rootsCoefficients a b c)))
    (hgz : Derivable F 5 (cubic z (rootsCoefficients a b c)))
    (hG : Derivable F 5 (gridPoly w z G))
    (hne : ∀ x ∈ ({a,b,c} : Set K), ∀ y ∈ ({a,b,c} : Set K), gridVal G x y ≠ 0) :
    Derivable F 5 1 := by
  let s := rootsCoefficients a b c
  let I := inverseGrid G a b c
  have heq : gridProduct s I G = oneGrid := by
    have hv (i j : Fin 3) :
        gridVal (gridProduct s I G) (roots a b c i) (roots a b c j) = 1 := by
      have hri : scalarCubic s (roots a b c i) = 0 := by
        fin_cases i <;> simp [s, roots, scalarCubic_roots]
      have hrj : scalarCubic s (roots a b c j) = 0 := by
        fin_cases j <;> simp [s, roots, scalarCubic_roots]
      rw [gridProduct_value s I G _ _ hri hrj,
        inverseGrid_value G a b c hab hac hbc i j]
      exact inv_mul_cancel₀ (hne _ (roots_mem a b c i) _ (roots_mem a b c j))
    apply eq_of_gridValues _ _ a b c hab hac hbc
    intro x hx y hy
    obtain ⟨i,rfl⟩ := root_index a b c x hx
    obtain ⟨j,rfl⟩ := root_index a b c y hy
    simpa using hv i j
  have h := gridProduct_derivable F s I G w z hw hz hgw hgz hG
  simpa only [heq, gridPoly_one] using h

end
end GridClosure

#print axioms GridClosure.grid_refutation
