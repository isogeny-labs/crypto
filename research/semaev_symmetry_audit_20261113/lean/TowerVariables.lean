import CanonicalTower

namespace SemaevSymmetryAudit
open Polynomial
noncomputable section
variable {R : Type*} [CommRing R]

def towerEquiv (R : Type*) [CommRing R] :
    (n : ℕ) → MvPolynomial (Fin n) R ≃+* Tower R n
  | 0 => MvPolynomial.isEmptyRingEquiv R (Fin 0)
  | n+1 => (MvPolynomial.finSuccEquiv R n).toRingEquiv.trans
      (Polynomial.mapEquiv (towerEquiv R n))

theorem towerEquiv_C (n : ℕ) (a : R) :
    towerEquiv R n (MvPolynomial.C a) = towerScalar n a := by
  induction n with
  | zero =>
    change (MvPolynomial.isEmptyRingEquiv R (Fin 0)) (MvPolynomial.C a) = a
    simpa only [MvPolynomial.isEmptyRingEquiv_symm_apply]
      using (MvPolynomial.isEmptyRingEquiv R (Fin 0)).apply_symm_apply a
  | succ n ih =>
    change (Polynomial.mapEquiv (towerEquiv R n))
      ((MvPolynomial.finSuccEquiv R n) (MvPolynomial.C a)) = C (towerScalar n a)
    have hc : (MvPolynomial.finSuccEquiv R n) (MvPolynomial.C a) =
        C (MvPolynomial.C a) := by simp [MvPolynomial.finSuccEquiv_apply]
    rw [hc, Polynomial.mapEquiv_apply, Polynomial.map_C]
    exact congrArg C ih

theorem towerEquiv_X (n : ℕ) (i : Fin n) :
    towerEquiv R n (MvPolynomial.X i) = towerVariable n i := by
  induction n with
  | zero => exact Fin.elim0 i
  | succ n ih =>
    refine Fin.cases ?_ (fun j => ?_) i
    · change (Polynomial.mapEquiv (towerEquiv R n))
        ((MvPolynomial.finSuccEquiv R n) (MvPolynomial.X 0)) = X
      rw [MvPolynomial.finSuccEquiv_X_zero, Polynomial.mapEquiv_apply, Polynomial.map_X]
    · change (Polynomial.mapEquiv (towerEquiv R n))
        ((MvPolynomial.finSuccEquiv R n) (MvPolynomial.X j.succ)) = C (towerVariable n j)
      rw [MvPolynomial.finSuccEquiv_X_succ, Polynomial.mapEquiv_apply, Polynomial.map_C]
      exact congrArg C (ih j)

def adjacentPerm (n : ℕ) (i : Fin (n+1)) : Equiv.Perm (Fin (n+2)) :=
  Equiv.swap i.castSucc i.succ

theorem adjacentPerm_zero_zero (n : ℕ) :
    adjacentPerm n 0 0 = (0 : Fin (n+1)).succ := by simp [adjacentPerm]

theorem adjacentPerm_zero_one (n : ℕ) :
    adjacentPerm n 0 (0 : Fin (n+1)).succ = 0 := by simp [adjacentPerm]

theorem adjacentPerm_zero_far (n : ℕ) (j : Fin n) :
    adjacentPerm n 0 j.succ.succ = j.succ.succ := by
  change Equiv.swap (0 : Fin (n+2)) (0 : Fin (n+1)).succ j.succ.succ = _
  have h0 : j.succ.succ ≠ (0 : Fin (n+2)) := Fin.succ_ne_zero _
  have h1 : j.succ.succ ≠ (0 : Fin (n+1)).succ := by
    intro hh
    have hv := congrArg Fin.val hh
    simp only [Fin.val_succ, Fin.val_zero] at hv
    omega
  simp only [Equiv.swap_apply_def, ite_eq_right h0, ite_eq_right h1]

theorem adjacentPerm_succ_zero (n : ℕ) (i : Fin (n+1)) :
    adjacentPerm (n+1) i.succ 0 = 0 := by
  change Equiv.swap i.castSucc.succ i.succ.succ (0 : Fin (n+3)) = _
  have h0 : (0 : Fin (n+3)) ≠ i.castSucc.succ := (Fin.succ_ne_zero _).symm
  have h1 : (0 : Fin (n+3)) ≠ i.succ.succ := (Fin.succ_ne_zero _).symm
  simp only [Equiv.swap_apply_def, ite_eq_right h0, ite_eq_right h1]

theorem adjacentPerm_succ_apply (n : ℕ) (i : Fin (n+1)) (j : Fin (n+2)) :
    adjacentPerm (n+1) i.succ j.succ = (adjacentPerm n i j).succ := by
  change Equiv.swap i.castSucc.succ i.succ.succ j.succ = _
  simp only [adjacentPerm, Equiv.swap_apply_def, Fin.succ_inj]
  split_ifs <;> rfl

theorem adjacent_zero_apply (n : ℕ) (p : Tower R (n+2)) :
    adjacent n 0 p = Polynomial.Bivariate.swap (R := Tower R n) p := by
  cases n <;> rfl

theorem adjacent_zero_variable (n : ℕ) (j : Fin (n+2)) :
    adjacent (R := R) n 0 (towerVariable (n+2) j) =
      towerVariable (n+2) (adjacentPerm n 0 j) := by
  refine Fin.cases ?_ (fun k => Fin.cases ?_ (fun l => ?_) k) j
  · rw [adjacentPerm_zero_zero]
    change adjacent n 0 X = C X
    rw [adjacent_zero_apply]
    exact Polynomial.Bivariate.swap_Y
  · rw [adjacentPerm_zero_one]
    change adjacent n 0 (C X) = X
    rw [adjacent_zero_apply]
    exact Polynomial.Bivariate.swap_X
  · rw [adjacentPerm_zero_far]
    change adjacent n 0 (C (C (towerVariable n l))) = C (C (towerVariable n l))
    rw [adjacent_zero_apply]
    exact Polynomial.Bivariate.swap_C_C _

theorem adjacent_variable (n : ℕ) (i : Fin (n+1)) (j : Fin (n+2)) :
    adjacent (R := R) n i (towerVariable (n+2) j) =
      towerVariable (n+2) (adjacentPerm n i j) := by
  induction n with
  | zero =>
    have hi : i = 0 := by
      apply Fin.ext
      have hil := i.isLt
      simp only [Fin.val_zero]
      omega
    subst i
    exact adjacent_zero_variable 0 j
  | succ n ih =>
    refine Fin.cases (adjacent_zero_variable (n+1) j) (fun k => ?_) i
    refine Fin.cases ?_ (fun l => ?_) j
    · rw [adjacentPerm_succ_zero]
      change (X : Polynomial (Tower R (n+2))).map (adjacent n k).toRingHom = X
      exact Polynomial.map_X _
    · rw [adjacentPerm_succ_apply]
      change (C (towerVariable (n+2) l)).map (adjacent n k).toRingHom =
        C (towerVariable (n+2) (adjacentPerm n k l))
      rw [Polynomial.map_C]
      exact congrArg C (ih k l)

theorem towerEquiv_adjacent (n : ℕ) (i : Fin (n+1)) (p : MvPolynomial (Fin (n+2)) R) :
    towerEquiv R (n+2) (MvPolynomial.rename (adjacentPerm n i) p) =
      adjacent n i (towerEquiv R (n+2) p) := by
  have hh : (towerEquiv R (n+2)).toRingHom.comp
      (MvPolynomial.rename (adjacentPerm n i)).toRingHom =
      (adjacent n i).toRingHom.comp (towerEquiv R (n+2)).toRingHom := by
    apply MvPolynomial.ringHom_ext
    · intro a
      change towerEquiv R (n+2) (MvPolynomial.rename _ (MvPolynomial.C a)) =
        adjacent n i (towerEquiv R (n+2) (MvPolynomial.C a))
      rw [MvPolynomial.rename_C, towerEquiv_C, adjacent_scalar]
    · intro j
      change towerEquiv R (n+2) (MvPolynomial.rename _ (MvPolynomial.X j)) =
        adjacent n i (towerEquiv R (n+2) (MvPolynomial.X j))
      rw [MvPolynomial.rename_X, towerEquiv_X, towerEquiv_X, adjacent_variable]
  exact congrArg (fun f => f p) hh

#print axioms towerEquiv_adjacent

end
end SemaevSymmetryAudit
