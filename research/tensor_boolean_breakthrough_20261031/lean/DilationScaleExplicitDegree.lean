import DilationScaleDegree
import FullSymmetry

/-!
# Explicit arity-only degree bound for simultaneous Semaev scaling

This module converts the all-input symmetry and exact individual-variable
degree of the canonical Semaev polynomial into a coarse, explicit total-degree
bound.  It then shows that simultaneous linear specialization cannot increase
that bound.
-/

namespace DilationSemaevBridge

noncomputable section

open Polynomial
open SemaevSymmetryAudit
open RecursiveSemaevDegreeAudit

universe u v

theorem towerEquiv_natDegree {R : Type u} [CommRing R]
    (n : ℕ) (p : MvPolynomial (Fin (n + 1)) R) :
    (towerEquiv R (n + 1) p : Polynomial (Tower R n)).natDegree =
      p.degreeOf 0 := by
  change ((MvPolynomial.finSuccEquiv R n p).map
    (towerEquiv R n).toRingHom).natDegree = p.degreeOf 0
  rw [Polynomial.natDegree_map_eq_of_injective (towerEquiv R n).injective]
  exact MvPolynomial.natDegree_finSuccEquiv p

theorem degreeOf_eq_of_isSymmetric {R : Type u} [CommRing R]
    {n : ℕ} {p : MvPolynomial (Fin (n + 1)) R}
    (hp : p.IsSymmetric) (i : Fin (n + 1)) :
    p.degreeOf i = p.degreeOf 0 := by
  let σ : Equiv.Perm (Fin (n + 1)) := Equiv.swap 0 i
  have hrename : MvPolynomial.rename σ p = p := hp σ
  have hdegree := MvPolynomial.degreeOf_rename_of_injective
    (p := p) σ.injective 0
  rw [hrename] at hdegree
  simpa [σ] using hdegree

theorem totalDegree_le_card_mul_of_degreeOf_le
    {R : Type u} [CommRing R] {n D : ℕ}
    (p : MvPolynomial (Fin n) R)
    (hdegree : ∀ i, p.degreeOf i ≤ D) :
    p.totalDegree ≤ n * D := by
  rw [MvPolynomial.totalDegree]
  apply Finset.sup_le
  intro d hd
  calc
    d.sum (fun _ e => e) = ∑ i, d i :=
      Finsupp.sum_fintype d (fun _ e => e) (by simp)
    _ ≤ ∑ _i : Fin n, D := by
      apply Finset.sum_le_sum
      intro i hi
      exact (MvPolynomial.degreeOf_le_iff.mp (hdegree i)) d hd
    _ = n * D := by simp

theorem canonicalMv_degreeOf (K : Type u) [Field K]
    (W : WeierstrassCurve.Affine K) (n : ℕ) (i : Fin (n + 3)) :
    (canonicalMv W n).degreeOf i = 2 ^ (n + 1) := by
  calc
    (canonicalMv W n).degreeOf i = (canonicalMv W n).degreeOf 0 :=
      degreeOf_eq_of_isSymmetric (canonicalMv_isSymmetric W n) i
    _ = (towerEquiv K (n + 3) (canonicalMv W n) :
        Polynomial (Tower K (n + 2))).natDegree := by
      rw [towerEquiv_natDegree]
    _ = (towerCanonical W n : Polynomial (Tower K (n + 2))).natDegree := by
      simp only [canonicalMv, RingEquiv.apply_symm_apply]
    _ = 2 ^ (n + 1) := towerCanonical_degree W n

theorem canonicalMv_totalDegree_le (K : Type u) [Field K]
    (W : WeierstrassCurve.Affine K) (n : ℕ) :
    (canonicalMv W n).totalDegree ≤ (n + 3) * 2 ^ (n + 1) := by
  apply totalDegree_le_card_mul_of_degreeOf_le
  intro i
  rw [canonicalMv_degreeOf K W n i]

theorem natDegree_eval₂_linear_le_totalDegree
    {K : Type u} [Field K] {n : ℕ}
    (p : MvPolynomial (Fin n) K) (t : Fin n → K) :
    (p.eval₂ Polynomial.C
      (fun i => Polynomial.C (t i) * Polynomial.X)).natDegree ≤
        p.totalDegree := by
  rw [MvPolynomial.eval₂_eq']
  apply Polynomial.natDegree_sum_le_of_forall_le
  intro d hd
  calc
    (Polynomial.C (p.coeff d) *
        ∏ i, (Polynomial.C (t i) * Polynomial.X) ^ d i).natDegree ≤
        (Polynomial.C (p.coeff d)).natDegree +
          (∏ i, (Polynomial.C (t i) * Polynomial.X) ^ d i).natDegree :=
      Polynomial.natDegree_mul_le
    _ ≤ 0 + ∑ i, d i := by
      gcongr
      · simp
      · calc
          (∏ i, (Polynomial.C (t i) * Polynomial.X) ^ d i).natDegree ≤
              ∑ i, ((Polynomial.C (t i) * Polynomial.X) ^ d i).natDegree :=
            Polynomial.natDegree_prod_le (Finset.univ : Finset (Fin n))
              (fun i => (Polynomial.C (t i) * Polynomial.X) ^ d i)
          _ ≤ ∑ i, d i := by
            apply Finset.sum_le_sum
            intro i hi
            calc
              ((Polynomial.C (t i) * Polynomial.X) ^ d i).natDegree ≤
                  d i * (Polynomial.C (t i) * Polynomial.X).natDegree :=
                Polynomial.natDegree_pow_le
              _ ≤ d i * 1 := Nat.mul_le_mul_left _
                (linearPolynomial_natDegree_le_one (t i))
              _ = d i := Nat.mul_one _
    _ = ∑ i, d i := Nat.zero_add _
    _ = d.sum (fun _ e => e) :=
      (Finsupp.sum_fintype d (fun _ e => e) (by simp)).symm
    _ ≤ p.totalDegree := MvPolynomial.le_totalDegree hd

/-- Evaluate the explicit polynomial tower, with index `0` denoting the
outermost variable. -/
def towerEvalTo {K : Type u} [Field K] {R : Type v} [CommRing R]
    (phi : K →+* R) : (n : ℕ) → (Fin n → R) → Tower K n →+* R
  | 0, _ => phi
  | n + 1, xs => Polynomial.eval₂RingHom
      (towerEvalTo phi n (fun i => xs i.succ)) (xs 0)

theorem towerEvalTo_succ {K : Type u} [Field K]
    {R : Type v} [CommRing R] (phi : K →+* R)
    (n : ℕ) (xs : Fin (n + 1) → R) :
    towerEvalTo phi (n + 1) xs = Polynomial.eval₂RingHom
      (towerEvalTo phi n (fun i => xs i.succ)) (xs 0) := rfl

theorem towerEvalTo_scalar {K : Type u} [Field K]
    {R : Type v} [CommRing R] (phi : K →+* R) :
    ∀ (n : ℕ) (xs : Fin n → R) (a : K),
      towerEvalTo phi n xs (towerScalar n a) = phi a := by
  intro n
  induction n with
  | zero => intro xs a; rfl
  | succ n ih =>
      intro xs a
      rw [towerEvalTo_succ, towerScalar_succ, Polynomial.coe_eval₂RingHom,
        Polynomial.eval₂_C]
      exact ih _ a

theorem towerEvalTo_variable {K : Type u} [Field K]
    {R : Type v} [CommRing R] (phi : K →+* R) :
    ∀ (n : ℕ) (xs : Fin n → R) (i : Fin n),
      towerEvalTo phi n xs (towerVariable n i) = xs i := by
  intro n
  induction n with
  | zero => intro xs i; exact Fin.elim0 i
  | succ n ih =>
      intro xs i
      refine Fin.cases ?_ (fun j => ?_) i
      · rw [towerEvalTo_succ, towerVariable_zero,
          Polynomial.coe_eval₂RingHom, Polynomial.eval₂_X]
      · rw [towerEvalTo_succ, towerVariable_succ,
          Polynomial.coe_eval₂RingHom, Polynomial.eval₂_C]
        exact ih _ j

theorem towerEvalTo_comp_towerEquiv {K : Type u} [Field K]
    {R : Type v} [CommRing R] (phi : K →+* R)
    (n : ℕ) (xs : Fin n → R) :
    (towerEvalTo phi n xs).comp (towerEquiv K n).toRingHom =
      MvPolynomial.eval₂Hom phi xs := by
  apply MvPolynomial.ringHom_ext
  · intro a
    simp [RingHom.comp_apply, towerEquiv_C, towerEvalTo_scalar]
  · intro i
    simp [RingHom.comp_apply, towerEquiv_X, towerEvalTo_variable]

set_option backward.isDefEq.respectTransparency false in
theorem towerEvalTo_comp_inputEquiv {K : Type u} [Field K]
    {R : Type v} [CommRing R] (phi : K →+* R) :
    ∀ (n : ℕ) (xs : Fin (n + 2) → R),
      (towerEvalTo phi (n + 2) (fun i => xs i.rev)).comp
          (inputEquiv K n).toRingHom = inputEvalTo phi n xs := by
  intro n
  induction n with
  | zero =>
      intro xs
      apply RingHom.ext
      intro p
      rw [RingHom.comp_apply, towerEvalTo_succ, towerEvalTo_succ]
      change Polynomial.eval₂ (Polynomial.eval₂RingHom phi (xs 0))
        (xs 1) p = Polynomial.eval₂ (Polynomial.eval₂RingHom phi (xs 0))
          (xs 1) p
      rfl
  | succ n ih =>
      intro xs
      apply RingHom.ext
      intro p
      rw [RingHom.comp_apply, towerEvalTo_succ]
      change Polynomial.eval₂
          (towerEvalTo phi (n + 2) (fun i => xs i.succ.rev))
          (xs (Fin.rev 0)) ((Polynomial.mapEquiv (inputEquiv K n)) p) =
        Polynomial.eval₂ (inputEvalTo phi n (Fin.init xs))
          (xs (Fin.last (n + 2))) p
      rw [Polynomial.mapEquiv_apply, Polynomial.eval₂_map]
      have htail : (fun i : Fin (n + 2) => xs i.succ.rev) =
          fun i => (Fin.init xs) i.rev := by
        funext i
        simp only [Fin.rev_succ, Fin.init]
      rw [htail]
      have h := congrArg
        (fun f : InputRing K n →+* R => Polynomial.eval₂ f
          (xs (Fin.last (n + 2)))
          (show Polynomial (InputRing K n) from p))
        (ih (Fin.init xs))
      simpa only [Fin.rev_zero, RingEquiv.toRingHom_eq_coe] using h

theorem simultaneousScalePolynomial_eq_mvEval
    {K : Type u} [Field K] (W : WeierstrassCurve.Affine K)
    (n : ℕ) (t : Fin (n + 3) → K) :
    simultaneousScalePolynomial W n t =
      (canonicalMv W n).eval₂ Polynomial.C
        (fun i => Polynomial.C (t i.rev) * Polynomial.X) := by
  let xs : Fin (n + 3) → Polynomial K :=
    fun i => Polynomial.C (t i) * Polynomial.X
  calc
    simultaneousScalePolynomial W n t =
        towerEvalTo Polynomial.C (n + 3) (fun i => xs i.rev)
          (towerCanonical W n) := by
      change evalCanonicalTo Polynomial.C W n xs = _
      unfold evalCanonicalTo towerCanonical
      change Polynomial.eval₂ (inputEvalTo Polynomial.C n (Fin.init xs))
          (xs (Fin.last (n + 2))) (recursivePolynomial W n) = _
      rw [towerEvalTo_succ]
      change _ = Polynomial.eval₂
        (towerEvalTo Polynomial.C (n + 2) (fun i => xs i.succ.rev))
        (xs (Fin.rev 0))
        ((recursivePolynomial W n).map (inputEquiv K n).toRingHom)
      rw [Polynomial.eval₂_map]
      have htail : (fun i : Fin (n + 2) => xs i.succ.rev) =
          fun i => (Fin.init xs) i.rev := by
        funext i
        simp only [Fin.rev_succ, Fin.init]
      rw [htail, towerEvalTo_comp_inputEquiv Polynomial.C n (Fin.init xs),
        Fin.rev_zero]
    _ = towerEvalTo Polynomial.C (n + 3) (fun i => xs i.rev)
        (towerEquiv K (n + 3) (canonicalMv W n)) := by
      simp only [canonicalMv, RingEquiv.apply_symm_apply]
    _ = (canonicalMv W n).eval₂ Polynomial.C
        (fun i => xs i.rev) := by
      have h := congrArg
        (fun f : MvPolynomial (Fin (n + 3)) K →+* Polynomial K =>
          f (canonicalMv W n))
        (towerEvalTo_comp_towerEquiv Polynomial.C (n + 3)
          (fun i => xs i.rev))
      simpa [MvPolynomial.coe_eval₂Hom] using h
    _ = (canonicalMv W n).eval₂ Polynomial.C
        (fun i => Polynomial.C (t i.rev) * Polynomial.X) := rfl

theorem simultaneousScalePolynomial_natDegree_le_explicit
    {K : Type u} [Field K] (W : WeierstrassCurve.Affine K)
    (n : ℕ) (t : Fin (n + 3) → K) :
    (simultaneousScalePolynomial W n t).natDegree ≤
      (n + 3) * 2 ^ (n + 1) := by
  rw [simultaneousScalePolynomial_eq_mvEval]
  exact (natDegree_eval₂_linear_le_totalDegree
    (canonicalMv W n) (fun i => t i.rev)).trans
      (canonicalMv_totalDegree_le K W n)

end

end DilationSemaevBridge
