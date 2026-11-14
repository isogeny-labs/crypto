import RecursiveSemaevDegree

/-! Symmetry of the two newest variables of the actual recursive resultant.
Older-variable symmetry and geometric zero-sum recursion remain separate. -/
namespace RecursiveSemaevSymmetryAudit
noncomputable section
open Polynomial S4DegreeAudit ResultantDegreeStepAudit RecursiveSemaevDegreeAudit
variable {R : Type*} [CommRing R]

def exchange : R[X][X] →+* R[X][X] := Polynomial.Bivariate.swap.toRingHom

theorem exchange_inner : exchange (C (X:R[X]))=(X:R[X][X]) :=
  Polynomial.Bivariate.swap_X

theorem exchange_outer : exchange (X:R[X][X])=C (X:R[X]) :=
  Polynomial.Bivariate.swap_Y

theorem exchange_scalar (r : R) : exchange (C (C r))=C (C r) :=
  Polynomial.Bivariate.swap_C_C r

theorem exchange_qa :
    exchange (qaRing (C (X:R[X])) X)=qaRing (C (X:R[X])) X := by
  simp only [qaRing,map_pow,map_sub,exchange_inner,exchange_outer]
  ring

theorem exchange_qb (b2 b4 b6 : R) :
    exchange (qbRing (C (C b2)) (C (C b4)) (C (C b6)) (C (X:R[X])) X)=
      qbRing (C (C b2)) (C (C b4)) (C (C b6)) (C (X:R[X])) X := by
  simp only [qbRing,map_neg,map_add,map_mul,map_ofNat,
    exchange_scalar,exchange_inner,exchange_outer]
  ring

theorem exchange_qc (b4 b6 b8 : R) :
    exchange (qcRing (C (C b4)) (C (C b6)) (C (C b8)) (C (X:R[X])) X)=
      qcRing (C (C b4)) (C (C b6)) (C (C b8)) (C (X:R[X])) X := by
  simp only [qcRing,map_sub,map_pow,map_mul,map_add,
    exchange_scalar,exchange_inner,exchange_outer]
  ring

theorem firstLift_exchange (f : R[X]) : (firstLift f).map exchange=firstLift f := by
  ext k
  simp only [Polynomial.coeff_map,firstLift_coeff,exchange_scalar]

theorem secondLift_exchange (b2 b4 b6 b8 : R) :
    (secondLift b2 b4 b6 b8).map exchange=secondLift b2 b4 b6 b8 := by
  unfold secondLift
  simp only [quad,Polynomial.map_add,Polynomial.map_mul,Polynomial.map_pow,
    Polynomial.map_C,Polynomial.map_X,exchange_qa,exchange_qb,exchange_qc]

theorem resultantStep_exchange (D : ℕ) (f : R[X]) (b2 b4 b6 b8 : R) :
    exchange (resultantStep D f b2 b4 b6 b8)=resultantStep D f b2 b4 b6 b8 := by
  unfold resultantStep
  rw [←Polynomial.resultant_map_map,firstLift_exchange,secondLift_exchange]

variable {K : Type*} [Field K]

theorem recursive_new_pair_exchange (W : WeierstrassCurve.Affine K) (n : ℕ) :
    exchange (R:=InputRing K n) (recursivePolynomial W (n+1))=
      recursivePolynomial W (n+1) := by
  rw [recursivePolynomial_succ]
  exact resultantStep_exchange ..

set_option backward.isDefEq.respectTransparency false in
theorem recursive_penultimate_degree (W : WeierstrassCurve.Affine K) (n : ℕ) :
    (exchange (R:=InputRing K n) (recursivePolynomial W (n+1))).natDegree=2^(n+2) := by
  rw [recursive_new_pair_exchange]
  simpa only [Nat.add_assoc] using recursivePolynomial_natDegree W (n+1)

/-- Stage n+1 has arity n+4; the two newest variables have equal sharp degree. -/
theorem newest_pair_audit_certificate :
    ∀ (F : Type*) [Field F] (W : WeierstrassCurve.Affine F) (n : ℕ),
    exchange (R:=InputRing F n) (recursivePolynomial W (n+1))=
      recursivePolynomial W (n+1) ∧
    (recursivePolynomial W (n+1)).natDegree=2^(n+2) ∧
    (exchange (R:=InputRing F n) (recursivePolynomial W (n+1))).natDegree=2^(n+2) := by
  intro F _ W n
  exact ⟨recursive_new_pair_exchange W n,
    by simpa only [Nat.add_assoc] using recursivePolynomial_natDegree W (n+1),
    recursive_penultimate_degree W n⟩

#print axioms newest_pair_audit_certificate
end
end RecursiveSemaevSymmetryAudit
