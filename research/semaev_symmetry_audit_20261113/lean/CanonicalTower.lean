import ThreeActions
import RecursiveSemaevTransport

namespace SemaevSymmetryAudit
open Polynomial S4DegreeAudit ResultantDegreeStepAudit RecursiveSemaevDegreeAudit
open SemaevReassociationAudit RecursiveSemaevTransportAudit
noncomputable section
universe u

/-- A fixed, explicit tower with one actual independent variable per successor. -/
abbrev Tower (R : Type u) [CommRing R] : ℕ → CommRingCat.{u}
  | 0 => CommRingCat.of R
  | n+1 => CommRingCat.of (Polynomial (Tower R n))

variable {R : Type u} [CommRing R]

instance towerDomain [IsDomain R] (n : ℕ) : IsDomain (Tower R n) := by
  induction n with
  | zero => exact inferInstanceAs (IsDomain R)
  | succ n ih =>
    let := ih
    exact inferInstanceAs (IsDomain (Polynomial (Tower R n)))

def towerScalar : (n : ℕ) → R →+* Tower R n
  | 0 => RingHom.id R
  | n+1 => C.comp (towerScalar n)

def towerVariable : (n : ℕ) → Fin n → Tower R n
  | 0 => Fin.elim0
  | n+1 => Fin.cases X (fun i => C (towerVariable n i))

@[simp] theorem towerScalar_zero (a : R) : towerScalar 0 a = a := rfl
@[simp] theorem towerScalar_succ (n : ℕ) (a : R) :
    towerScalar (n+1) a = C (towerScalar n a) := rfl

@[simp] theorem towerVariable_zero (n : ℕ) : towerVariable (R := R) (n+1) 0 = X := rfl
@[simp] theorem towerVariable_succ (n : ℕ) (i : Fin n) :
    towerVariable (R := R) (n+1) i.succ = C (towerVariable n i) := rfl

theorem towerVariable_two_one : towerVariable (R := R) 2 1 = C X := by
  change towerVariable 2 (0 : Fin 1).succ = _
  rw [towerVariable_succ, towerVariable_zero]

theorem towerVariable_four_one : towerVariable (R := R) 4 1 = C X := by
  change towerVariable 4 (0 : Fin 3).succ = _
  rw [towerVariable_succ, towerVariable_zero]

theorem towerVariable_four_two : towerVariable (R := R) 4 2 = C (C X) := by
  change towerVariable 4 (0 : Fin 2).succ.succ = _
  rw [towerVariable_succ, towerVariable_succ, towerVariable_zero]

theorem towerVariable_four_three : towerVariable (R := R) 4 3 = C (C (C X)) := by
  change towerVariable 4 (0 : Fin 1).succ.succ.succ = _
  rw [towerVariable_succ, towerVariable_succ, towerVariable_succ, towerVariable_zero]

/-- Index0 swaps the outermost two inputs; each successor lifts an inner swap. -/
def adjacent : (n : ℕ) → Fin (n+1) → Tower R (n+2) ≃+* Tower R (n+2)
  | 0, _ => (Polynomial.Bivariate.swap (R := R)).toRingEquiv
  | n+1, i => Fin.cases
      (Polynomial.Bivariate.swap (R := Tower R (n+1))).toRingEquiv
      (fun j => Polynomial.mapEquiv (adjacent n j)) i

theorem adjacent_succ_apply (n : ℕ) (i : Fin (n+1)) (p : Polynomial (Tower R (n+2))) :
    adjacent (n+1) i.succ p = p.map (adjacent n i).toRingHom := rfl

theorem adjacent_two_one : adjacent (R := R) 2 (1 : Fin 3) =
    Polynomial.mapEquiv (Polynomial.Bivariate.swap (R := Tower R 1)).toRingEquiv := by
  change adjacent 2 (0 : Fin 2).succ = _
  rfl

theorem adjacent_scalar (n : ℕ) (i : Fin (n+1)) (a : R) :
    adjacent n i (towerScalar (n+2) a) = towerScalar (n+2) a := by
  induction n with
  | zero =>
    change Polynomial.Bivariate.swap (C (C a)) = C (C a)
    exact Polynomial.Bivariate.swap_C_C a
  | succ n ih =>
    refine Fin.cases ?_ (fun j => ?_) i
    · change Polynomial.Bivariate.swap (C (C (towerScalar (n+1) a))) = _
      exact Polynomial.Bivariate.swap_C_C _
    · change (C (towerScalar (n+2) a)).map (adjacent n j).toRingHom = C (towerScalar (n+2) a)
      rw [Polynomial.map_C]
      exact congrArg C (ih j)

variable {K : Type u} [Field K]

def inputEquiv (K : Type u) [Field K] :
    (n : ℕ) → InputRing K n ≃+* Tower K (n+2)
  | 0 => RingEquiv.refl _
  | n+1 => Polynomial.mapEquiv (inputEquiv K n)

theorem inputEquiv_scalar (n : ℕ) (a : K) :
    inputEquiv K n (coefficientEmbedding n a) = towerScalar (n+2) a := by
  induction n with
  | zero => rfl
  | succ n ih =>
    change (C (coefficientEmbedding n a)).map (inputEquiv K n).toRingHom = C (towerScalar (n+2) a)
    rw [Polynomial.map_C]
    exact congrArg C ih

/-- The object is explicitly the image of the existing canonical polynomial. -/
def towerCanonical (W : WeierstrassCurve.Affine K) (n : ℕ) : Tower K (n+3) :=
  (recursivePolynomial W n).map (inputEquiv K n).toRingHom

theorem towerCanonical_succ (W : WeierstrassCurve.Affine K) (n : ℕ) :
    towerCanonical W (n+1) = resultantStep (2^(n+1)) (towerCanonical W n)
      (towerScalar (n+2) W.b₂) (towerScalar (n+2) W.b₄)
      (towerScalar (n+2) W.b₆) (towerScalar (n+2) W.b₈) := by
  unfold towerCanonical
  rw [recursivePolynomial_succ]
  change (resultantStep (2^(n+1)) (recursivePolynomial W n)
      (coefficientEmbedding n W.b₂) (coefficientEmbedding n W.b₄)
      (coefficientEmbedding n W.b₆) (coefficientEmbedding n W.b₈)).map
      (mapRingHom (inputEquiv K n).toRingHom) = _
  rw [resultantStep_map]
  have hs (a : K) : (inputEquiv K n).toRingHom (coefficientEmbedding n a) =
      towerScalar (n+2) a := inputEquiv_scalar n a
  rw [hs,hs,hs,hs]

theorem towerCanonical_degree (W : WeierstrassCurve.Affine K) (n : ℕ) :
    (towerCanonical W n : Polynomial (Tower K (n+2))).natDegree = 2^(n+1) := by
  rw [towerCanonical, natDegree_map_eq_of_injective (inputEquiv K n).injective,
    recursivePolynomial_natDegree]

theorem towerCanonical_double (W : WeierstrassCurve.Affine K) (n : ℕ) :
    towerCanonical W (n+2) = doubleStep (2^(n+1)) (towerCanonical W n)
      (towerScalar (n+2) W.b₂) (towerScalar (n+2) W.b₄)
      (towerScalar (n+2) W.b₆) (towerScalar (n+2) W.b₈) := by
  change towerCanonical W ((n+1)+1) = _
  rw [towerCanonical_succ, towerCanonical_succ]
  change resultantStep (2^((n+1)+1)) (resultantStep (2^(n+1)) (towerCanonical W n)
      (towerScalar (n+2) W.b₂) (towerScalar (n+2) W.b₄)
      (towerScalar (n+2) W.b₆) (towerScalar (n+2) W.b₈))
      (C (towerScalar (n+2) W.b₂)) (C (towerScalar (n+2) W.b₄))
      (C (towerScalar (n+2) W.b₆)) (C (towerScalar (n+2) W.b₈)) = _
  rw [pow_succ]
  rfl

#print axioms towerCanonical_succ

end
end SemaevSymmetryAudit
