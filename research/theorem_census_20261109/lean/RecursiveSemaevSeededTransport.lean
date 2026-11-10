import RecursiveSemaevTransport

/-! Transport starting from an actual scalar-fixing S4 coefficient-ring
equivalence. The seed is a variable permutation, not a degree recurrence. -/
namespace RecursiveSemaevSeededTransportAudit
noncomputable section
open Polynomial RecursiveSemaevDegreeAudit RecursiveSemaevFocusAudit
open RecursiveSemaevTransportAudit ResultantDegreeStepAudit
variable {K : Type*} [Field K]

set_option backward.isDefEq.respectTransparency false in
def seededTransport (K : Type*) [Field K] (seed : InputRing K 1 ≃+* InputRing K 1) :
    (n : ℕ) → InputRing K (n+1) ≃+* InputRing K (n+1)
  | 0 => seed
  | n+1 => (Polynomial.mapEquiv (R:=InputRing K (n+1)) (S:=InputRing K (n+1))
      (seededTransport K seed n)).trans
      (Polynomial.Bivariate.swap (R:=InputRing K n)).toRingEquiv

set_option backward.isDefEq.respectTransparency false in
theorem seededTransport_succ (seed : InputRing K 1 ≃+* InputRing K 1)
    (n : ℕ) (p : InputRing K (n+2)) :
    seededTransport K seed (n+1) p=
      Polynomial.Bivariate.swap
        ((p : Polynomial (InputRing K (n+1))).map
          (seededTransport K seed n).toRingHom) := rfl

set_option backward.isDefEq.respectTransparency false in
theorem seededTransport_scalar (seed : InputRing K 1 ≃+* InputRing K 1)
    (hs : ∀ a : K,seed (coefficientEmbedding 1 a)=coefficientEmbedding 1 a)
    (n : ℕ) (a : K) :
    seededTransport K seed n (coefficientEmbedding (n+1) a)=
      coefficientEmbedding (n+1) a := by
  induction n with
  | zero => exact hs a
  | succ n ih =>
    rw [seededTransport_succ (K:=K) seed n]
    change Polynomial.Bivariate.swap
      ((C (coefficientEmbedding (n+1) a)).map (seededTransport K seed n).toRingHom)=
      C (C (coefficientEmbedding n a))
    rw [Polynomial.map_C]
    have ht : (seededTransport K seed n).toRingHom
        (coefficientEmbedding (n+1) a)=C (coefficientEmbedding n a) := ih
    exact (congrArg (fun x : InputRing K (n+1) =>
      Polynomial.Bivariate.swap (R:=InputRing K n) (C x)) ht).trans
      (Polynomial.Bivariate.swap_C_C _)

set_option backward.isDefEq.respectTransparency false in
def liftInitial (K : Type*) [Field K] (x : InputRing K 1) :
    (n : ℕ) → InputRing K (n+1)
  | 0 => x
  | n+1 => C (liftInitial K x n)

set_option backward.isDefEq.respectTransparency false in
theorem seededTransport_coordinate (seed : InputRing K 1 ≃+* InputRing K 1)
    (x : InputRing K 1) (hx : seed x=(X : Polynomial (InputRing K 0))) (n : ℕ) :
    seededTransport K seed n (liftInitial K x n)=(X : Polynomial (InputRing K n)) := by
  induction n with
  | zero => exact hx
  | succ n ih =>
    rw [seededTransport_succ (K:=K) seed n]
    change Polynomial.Bivariate.swap
      ((C (liftInitial K x n)).map (seededTransport K seed n).toRingHom)=_
    rw [Polynomial.map_C]
    have ht : (seededTransport K seed n).toRingHom (liftInitial K x n)=
        (X : Polynomial (InputRing K n)) := ih
    exact (congrArg (fun y : InputRing K (n+1) =>
      Polynomial.Bivariate.swap (R:=InputRing K n) (C y)) ht).trans
      Polynomial.Bivariate.swap_X

def seededPolynomial (W : WeierstrassCurve.Affine K)
    (seed : InputRing K 1 ≃+* InputRing K 1) (n : ℕ) : Polynomial (InputRing K (n+1)) :=
  (recursivePolynomial W (n+1)).map (seededTransport K seed n).toRingHom

theorem seeded_elimination_degree (W : WeierstrassCurve.Affine K)
    (seed : InputRing K 1 ≃+* InputRing K 1) (n : ℕ) :
    (seededPolynomial W seed n).natDegree=2^(n+2) := by
  unfold seededPolynomial
  rw [Polynomial.natDegree_map_eq_of_injective (seededTransport K seed n).injective]
  simpa only [Nat.add_assoc] using recursivePolynomial_natDegree W (n+1)

theorem seeded_nonzero (W : WeierstrassCurve.Affine K)
    (seed : InputRing K 1 ≃+* InputRing K 1) (n : ℕ) : seededPolynomial W seed n≠0 := by
  exact (Polynomial.map_ne_zero_iff (seededTransport K seed n).injective).mpr
    (recursivePolynomial_nonzero W (n+1))

set_option backward.isDefEq.respectTransparency false in
theorem seeded_succ (W : WeierstrassCurve.Affine K)
    (seed : InputRing K 1 ≃+* InputRing K 1)
    (hs : ∀ a : K,seed (coefficientEmbedding 1 a)=coefficientEmbedding 1 a) (n : ℕ) :
    seededPolynomial W seed (n+1)=
      (resultantStep (2^(n+2)) (seededPolynomial W seed n)
        (C (coefficientEmbedding n W.b₂)) (C (coefficientEmbedding n W.b₄))
        (C (coefficientEmbedding n W.b₆)) (C (coefficientEmbedding n W.b₈))).map
        (Polynomial.Bivariate.swap (R:=InputRing K n)).toRingHom := by
  unfold seededPolynomial
  change ((recursivePolynomial W ((n+1)+1)).map
    ((Polynomial.Bivariate.swap (R:=InputRing K n)).toRingHom.comp
      (Polynomial.mapRingHom (seededTransport K seed n).toRingHom)))=_
  rw [recursivePolynomial_succ,←Polynomial.map_map]
  rw [resultantStep_map (A:=InputRing K (n+1)) (B:=InputRing K (n+1))
    (seededTransport K seed n).toRingHom]
  have ht (a : K) : (seededTransport K seed n).toRingHom
      (coefficientEmbedding (n+1) a)=coefficientEmbedding (n+1) a :=
    seededTransport_scalar seed hs n a
  rw [ht,ht,ht,ht]
  rfl

set_option backward.isDefEq.respectTransparency false in
theorem seeded_degree_succ (W : WeierstrassCurve.Affine K)
    (seed : InputRing K 1 ≃+* InputRing K 1)
    (hs : ∀ a : K,seed (coefficientEmbedding 1 a)=coefficientEmbedding 1 a) (n : ℕ) :
    (Polynomial.Bivariate.swap (R:=InputRing K (n+1)) (seededPolynomial W seed (n+1))).natDegree=
      2*(Polynomial.Bivariate.swap (R:=InputRing K n) (seededPolynomial W seed n)).natDegree := by
  rw [seeded_succ (K:=K) W seed hs n]
  change (rotate (S:=InputRing K n)
    (resultantStep (2^(n+2)) (seededPolynomial W seed n)
      (C (coefficientEmbedding n W.b₂)) (C (coefficientEmbedding n W.b₄))
      (C (coefficientEmbedding n W.b₆)) (C (coefficientEmbedding n W.b₈)))).natDegree=_
  exact old_resultant_degree (S:=InputRing K n) _ _ _ _ _ _
    (le_of_eq (seeded_elimination_degree W seed n)) (seeded_nonzero W seed n)

set_option backward.isDefEq.respectTransparency false in
theorem seeded_degree (W : WeierstrassCurve.Affine K)
    (seed : InputRing K 1 ≃+* InputRing K 1)
    (hs : ∀ a : K,seed (coefficientEmbedding 1 a)=coefficientEmbedding 1 a) (n : ℕ) :
    (Polynomial.Bivariate.swap (R:=InputRing K n) (seededPolynomial W seed n)).natDegree=
      2^n*(Polynomial.Bivariate.swap (R:=InputRing K 0) (seededPolynomial W seed 0)).natDegree := by
  induction n with
  | zero => simp only [pow_zero,one_mul]
  | succ n ih =>
    calc
      _=2*(Polynomial.Bivariate.swap (R:=InputRing K n)
          (seededPolynomial W seed n)).natDegree := seeded_degree_succ W seed hs n
      _=2*(2^n*(Polynomial.Bivariate.swap (R:=InputRing K 0)
          (seededPolynomial W seed 0)).natDegree) := congrArg (fun d : ℕ=>2*d) ih
      _=_ := by rw [pow_succ]; ring

#print axioms seededTransport_coordinate
#print axioms seeded_succ
#print axioms seeded_degree
end
end RecursiveSemaevSeededTransportAudit
