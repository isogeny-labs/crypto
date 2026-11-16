import RecursiveSemaevFocus

/-! Actual variable transport across successive canonical resultant stages.
This module constructs ring equivalences rather than assuming degree growth. -/
namespace RecursiveSemaevTransportAudit
noncomputable section
open Polynomial RecursiveSemaevDegreeAudit RecursiveSemaevFocusAudit
variable {K : Type*} [Field K]

set_option backward.isDefEq.respectTransparency false in
/-- At n=0 focus the current penultimate coordinate. At each future stage,
lift the old equivalence and exchange the last two coefficient variables. -/
def focusTransport (K : Type*) [Field K] (start : ℕ) :
    (n : ℕ) → InputRing K (start+n+1) ≃+* InputRing K (start+n+1)
  | 0 => RingEquiv.refl _
  | n+1 =>
    (Polynomial.mapEquiv (R:=InputRing K (start+n+1)) (S:=InputRing K (start+n+1))
      (focusTransport K start n)).trans
      (Polynomial.Bivariate.swap (R:=InputRing K (start+n))).toRingEquiv

set_option backward.isDefEq.respectTransparency false in
theorem focusTransport_succ (start n : ℕ) (p : InputRing K (start+n+2)) :
    focusTransport K start (n+1) p=
      Polynomial.Bivariate.swap
        ((p : Polynomial (InputRing K (start+n+1))).map
          (focusTransport K start n).toRingHom) := rfl

set_option backward.isDefEq.respectTransparency false in
theorem focusTransport_scalar (start n : ℕ) (a : K) :
    focusTransport K start n (coefficientEmbedding (start+n+1) a)=
      coefficientEmbedding (start+n+1) a := by
  induction n with
  | zero => rfl
  | succ n ih =>
    rw [focusTransport_succ (K:=K) start n]
    change Polynomial.Bivariate.swap
      ((C (coefficientEmbedding (start+n+1) a)).map
        (focusTransport K start n).toRingHom)=
      C (C (coefficientEmbedding (start+n) a))
    rw [Polynomial.map_C]
    have hs : (focusTransport K start n).toRingHom
        (coefficientEmbedding (start+n+1) a)=C (coefficientEmbedding (start+n) a) := ih
    exact (congrArg (fun x : InputRing K (start+n+1) =>
      Polynomial.Bivariate.swap (R:=InputRing K (start+n)) (C x)) hs).trans
      (Polynomial.Bivariate.swap_C_C _)

set_option backward.isDefEq.respectTransparency false in
def bornVariable (K : Type*) [Field K] (start : ℕ) :
    (n : ℕ) → InputRing K (start+n+1)
  | 0 => (X : Polynomial (InputRing K start))
  | n+1 => C (bornVariable K start n)

set_option backward.isDefEq.respectTransparency false in
theorem focusTransport_bornVariable (start n : ℕ) :
    focusTransport K start n (bornVariable K start n)=
      (X : Polynomial (InputRing K (start+n))) := by
  induction n with
  | zero => rfl
  | succ n ih =>
    rw [focusTransport_succ (K:=K) start n]
    change Polynomial.Bivariate.swap
      ((C (bornVariable K start n)).map (focusTransport K start n).toRingHom)=_
    rw [Polynomial.map_C]
    have hs : (focusTransport K start n).toRingHom (bornVariable K start n)=
        (X : Polynomial (InputRing K (start+n))) := ih
    exact (congrArg (fun x : InputRing K (start+n+1) =>
      Polynomial.Bivariate.swap (R:=InputRing K (start+n)) (C x)) hs).trans
      Polynomial.Bivariate.swap_X

def focusedPolynomial (W : WeierstrassCurve.Affine K) (start n : ℕ) :
    Polynomial (InputRing K (start+n+1)) :=
  (recursivePolynomial W (start+n+1)).map (focusTransport K start n).toRingHom

theorem focused_elimination_degree (W : WeierstrassCurve.Affine K) (start n : ℕ) :
    (focusedPolynomial W start n).natDegree=2^(start+n+2) := by
  unfold focusedPolynomial
  rw [Polynomial.natDegree_map_eq_of_injective (focusTransport K start n).injective]
  simpa only [Nat.add_assoc] using recursivePolynomial_natDegree W (start+n+1)

theorem focused_nonzero (W : WeierstrassCurve.Affine K) (start n : ℕ) :
    focusedPolynomial W start n≠0 := by
  exact (Polynomial.map_ne_zero_iff (focusTransport K start n).injective).mpr
    (recursivePolynomial_nonzero W (start+n+1))

theorem focused_zero (W : WeierstrassCurve.Affine K) (start : ℕ) :
    focusedPolynomial W start 0=recursivePolynomial W (start+1) := by
  simp [focusedPolynomial,focusTransport]

section Naturality
variable {A B : Type*} [CommRing A] [CommRing B]
open S4DegreeAudit ResultantDegreeStepAudit

def doubleMap (φ : A→+*B) : A[X][X]→+*B[X][X] :=
  Polynomial.mapRingHom (Polynomial.mapRingHom φ)

theorem firstLift_map (φ : A→+*B) (f : A[X]) :
    (firstLift f).map (doubleMap φ)=firstLift (f.map φ) := by
  ext k
  simp [doubleMap,firstLift_coeff]

theorem secondLift_map (φ : A→+*B) (b2 b4 b6 b8 : A) :
    (secondLift b2 b4 b6 b8).map (doubleMap φ)=
      secondLift (φ b2) (φ b4) (φ b6) (φ b8) := by
  simp [secondLift,quad,qaRing,qbRing,qcRing,doubleMap,map_ofNat]

theorem resultantStep_map (φ : A→+*B) (D : ℕ) (f : A[X]) (b2 b4 b6 b8 : A) :
    (resultantStep D f b2 b4 b6 b8).map (Polynomial.mapRingHom φ)=
      resultantStep D (f.map φ) (φ b2) (φ b4) (φ b6) (φ b8) := by
  change doubleMap φ (resultantStep D f b2 b4 b6 b8)=_
  unfold resultantStep
  rw [←Polynomial.resultant_map_map (R:=A[X][X]) (S:=B[X][X])
    (firstLift f) (secondLift b2 b4 b6 b8) D 2 (doubleMap φ)]
  rw [firstLift_map,secondLift_map]

end Naturality

open ResultantDegreeStepAudit RecursiveSemaevSymmetryAudit

set_option backward.isDefEq.respectTransparency false in
theorem focused_succ (W : WeierstrassCurve.Affine K) (start n : ℕ) :
    focusedPolynomial W start (n+1)=
      (resultantStep (2^(start+n+2)) (focusedPolynomial W start n)
        (C (coefficientEmbedding (start+n) W.b₂))
        (C (coefficientEmbedding (start+n) W.b₄))
        (C (coefficientEmbedding (start+n) W.b₆))
        (C (coefficientEmbedding (start+n) W.b₈))).map
        (Polynomial.Bivariate.swap (R:=InputRing K (start+n))).toRingHom := by
  unfold focusedPolynomial
  change ((recursivePolynomial W ((start+n+1)+1)).map
    ((Polynomial.Bivariate.swap (R:=InputRing K (start+n))).toRingHom.comp
      (Polynomial.mapRingHom (focusTransport K start n).toRingHom)))=_
  rw [recursivePolynomial_succ,←Polynomial.map_map]
  rw [resultantStep_map (A:=InputRing K (start+n+1)) (B:=InputRing K (start+n+1))
    (focusTransport K start n).toRingHom]
  have hs (a : K) : (focusTransport K start n).toRingHom
      (coefficientEmbedding (start+n+1) a)=coefficientEmbedding (start+n+1) a :=
    focusTransport_scalar start n a
  rw [hs,hs,hs,hs]
  rfl

set_option backward.isDefEq.respectTransparency false in
theorem focused_degree_succ (W : WeierstrassCurve.Affine K) (start n : ℕ) :
    (Polynomial.Bivariate.swap (R:=InputRing K (start+n+1))
      (focusedPolynomial W start (n+1))).natDegree=
      2*(Polynomial.Bivariate.swap (R:=InputRing K (start+n))
        (focusedPolynomial W start n)).natDegree := by
  rw [focused_succ (K:=K) W start n]
  change (rotate (S:=InputRing K (start+n))
    (resultantStep (2^(start+n+2)) (focusedPolynomial W start n)
      (C (coefficientEmbedding (start+n) W.b₂))
      (C (coefficientEmbedding (start+n) W.b₄))
      (C (coefficientEmbedding (start+n) W.b₆))
      (C (coefficientEmbedding (start+n) W.b₈)))).natDegree=_
  exact old_resultant_degree (S:=InputRing K (start+n)) _ _ _ _ _ _
    (le_of_eq (focused_elimination_degree W start n)) (focused_nonzero W start n)

set_option backward.isDefEq.respectTransparency false in
theorem transported_degree (W : WeierstrassCurve.Affine K) (start n : ℕ) :
    (Polynomial.Bivariate.swap (R:=InputRing K (start+n))
      (focusedPolynomial W start n)).natDegree=2^(start+n+2) := by
  induction n with
  | zero =>
    rw [focused_zero]
    exact recursive_penultimate_degree W start
  | succ n ih =>
    calc
      _=2*(Polynomial.Bivariate.swap (R:=InputRing K (start+n))
          (focusedPolynomial W start n)).natDegree := focused_degree_succ W start n
      _=2*2^(start+n+2) := congrArg (fun d : ℕ=>2*d) ih
      _=2^(start+(n+1)+2) := by
        rw [show start+(n+1)+2=(start+n+2)+1 by omega,pow_succ]
        ring

theorem transport_degree_audit_certificate :
    ∀ (F : Type*) [Field F] (W : WeierstrassCurve.Affine F) (start n : ℕ),
      focusTransport F start n (bornVariable F start n)=
        (X : Polynomial (InputRing F (start+n))) ∧
      (∀ a : F,focusTransport F start n (coefficientEmbedding (start+n+1) a)=
        coefficientEmbedding (start+n+1) a) ∧
      (Polynomial.Bivariate.swap (R:=InputRing F (start+n))
        (focusedPolynomial W start n)).natDegree=2^(start+n+2) := by
  intro F _ W start n
  exact ⟨focusTransport_bornVariable start n,focusTransport_scalar start n,
    transported_degree W start n⟩

#print axioms transport_degree_audit_certificate
#print axioms transported_degree
#print axioms focused_succ
#print axioms focusTransport_scalar
#print axioms focusTransport_bornVariable
#print axioms focused_elimination_degree
#print axioms focused_nonzero
#print axioms resultantStep_map
end
end RecursiveSemaevTransportAudit
