import RecursiveSemaevInitialDegrees

/-! All canonical input families, with actual polynomial-ring permutations
and explicit arithmetic coverage of every input position. -/
namespace RecursiveSemaevAllDegreesAudit
noncomputable section
open Polynomial S4DegreeAudit ResultantDegreeStepAudit RecursiveSemaevDegreeAudit
open RecursiveSemaevFocusAudit RecursiveSemaevTransportAudit
open RecursiveSemaevSeededTransportAudit RecursiveSemaevInitialDegreesAudit
variable {K : Type*} [Field K]

theorem raw_base_ring (W : WeierstrassCurve.Affine K) :
    (quad (pairA (K:=K)) (pairB W) (pairC W) : K[X][X][X])=
      s3Ring (C (liftPair W.b₂)) (C (liftPair W.b₄))
        (C (liftPair W.b₆)) (C (liftPair W.b₈))
        (C (pairX (K:=K))) (C (pairY (K:=K))) (X:K[X][X][X]) := by
  simp [quad,s3Ring,qaRing,qbRing,qcRing,pairA,pairB,pairC,pairX,pairY,liftPair,
    Polynomial.C_ofNat]
  ring

#print axioms raw_base_ring

theorem raw_base_cycle (W : WeierstrassCurve.Affine K) :
    rotate (S:=K) (quad (pairA (K:=K)) (pairB W) (pairC W))=
      quad (pairA (K:=K)) (pairB W) (pairC W) := by
  calc
    _=rotate (S:=K) (s3Ring (C (liftPair W.b₂)) (C (liftPair W.b₄))
        (C (liftPair W.b₆)) (C (liftPair W.b₈))
        (C (pairX (K:=K))) (C (pairY (K:=K))) (X:K[X][X][X])) :=
      congrArg (rotate (S:=K)) (raw_base_ring W)
    _=s3Ring (C (liftPair W.b₂)) (C (liftPair W.b₄))
        (C (liftPair W.b₆)) (C (liftPair W.b₈))
        (X:K[X][X][X]) (C (pairX (K:=K))) (C (pairY (K:=K))) := by
      rw [s3Ring_map (R:=K[X][X][X]) (S:=K[X][X][X]) (rotate (S:=K))]
      simp only [liftPair,pairX,pairY,rotate_scalar (S:=K),
        rotate_scalar_polynomial (S:=K) (X:K[X]),Polynomial.map_X,
        rotate_Z (S:=K),rotate_T (S:=K)]
    _=s3Ring (C (liftPair W.b₂)) (C (liftPair W.b₄))
        (C (liftPair W.b₆)) (C (liftPair W.b₈))
        (C (pairX (K:=K))) (C (pairY (K:=K))) (X:K[X][X][X]) := s3Ring_cycle ..
    _=quad (pairA (K:=K)) (pairB W) (pairC W) := (raw_base_ring W).symm

#print axioms raw_base_cycle

set_option backward.isDefEq.respectTransparency false in
theorem base_cycle (W : WeierstrassCurve.Affine K) :
    seedZero K (recursivePolynomial W 0)=recursivePolynomial W 0 :=
  raw_base_cycle W

set_option backward.isDefEq.respectTransparency false in
theorem base_first_degree (W : WeierstrassCurve.Affine K) :
    (seedZero K (recursivePolynomial W 0)).natDegree=2 :=
  (congrArg (fun p : InputRing K 1 => (p : Polynomial (InputRing K 0)).natDegree)
    (base_cycle W)).trans (basePolynomial_natDegree W)

set_option backward.isDefEq.respectTransparency false in
theorem base_second_degree (W : WeierstrassCurve.Affine K) :
    ((seedZero K).trans (seedZero K) (recursivePolynomial W 0)).natDegree=2 :=
  (congrArg (fun p : InputRing K 1 =>
    (seedZero K p : Polynomial (InputRing K 0)).natDegree) (base_cycle W)).trans
    (base_first_degree W)

set_option backward.isDefEq.respectTransparency false in
theorem base_second_coordinate :
    (seedZero K).trans (seedZero K) (initialOne K)=(X : Polynomial (InputRing K 0)) :=
  (congrArg (seedZero K) (seedZero_middle (K:=K))).trans (seedZero_coordinate (K:=K))

theorem input_position_cases (stage : ℕ) (i : Fin (stage+3)) :
    i.val=0 ∨ i.val=1 ∨ i.val=stage+2 ∨
      ∃ start n : ℕ,stage=start+n+1 ∧ i.val=start+2 := by
  by_cases h0 : i.val=0
  · exact Or.inl h0
  right
  by_cases h1 : i.val=1
  · exact Or.inl h1
  right
  by_cases hl : i.val=stage+2
  · exact Or.inl hl
  right
  exact ⟨i.val-2,stage+1-i.val,by omega,by omega⟩

/-- Degree is read after the explicitly constructed variable permutations.
The family of introduced inputs plus the two initial inputs and last input
covers every position, with stage n having arity n+3. -/
theorem all_input_degree_audit_certificate :
    (∀ (F : Type*) [Field F] (W : WeierstrassCurve.Affine F),
      (seedZero F (initialZero F)=(X : Polynomial (InputRing F 0)) ∧
       (seedZero F).trans (seedZero F) (initialOne F)=(X : Polynomial (InputRing F 0))) ∧
      (seedZero F (recursivePolynomial W 0)).natDegree=2 ∧
      ((seedZero F).trans (seedZero F) (recursivePolynomial W 0)).natDegree=2 ∧
      (∀ n : ℕ,
        seededTransport F (seedZero F) n (liftInitial F (initialZero F) n)=
          (X : Polynomial (InputRing F n)) ∧
        (Polynomial.Bivariate.swap (R:=InputRing F n)
          (seededPolynomial W (seedZero F) n)).natDegree=2^(n+2) ∧
        seededTransport F (seedOne F) n (liftInitial F (initialOne F) n)=
          (X : Polynomial (InputRing F n)) ∧
        (Polynomial.Bivariate.swap (R:=InputRing F n)
          (seededPolynomial W (seedOne F) n)).natDegree=2^(n+2)) ∧
      (∀ start n : ℕ,
        focusTransport F start n (bornVariable F start n)=
          (X : Polynomial (InputRing F (start+n))) ∧
        (Polynomial.Bivariate.swap (R:=InputRing F (start+n))
          (focusedPolynomial W start n)).natDegree=2^(start+n+2)) ∧
      (∀ stage : ℕ,(recursivePolynomial W stage).natDegree=2^(stage+1))) ∧
    (∀ (stage : ℕ) (i : Fin (stage+3)),i.val=0 ∨ i.val=1 ∨ i.val=stage+2 ∨
      ∃ start n : ℕ,stage=start+n+1 ∧ i.val=start+2) := by
  constructor
  · intro F _ W
    refine ⟨⟨seedZero_coordinate,base_second_coordinate⟩,
      base_first_degree W,base_second_degree W,?_,?_,?_⟩
    · intro n
      exact ⟨seededTransport_coordinate (seedZero F) (initialZero F) seedZero_coordinate n,
        first_input_degree W n,
        seededTransport_coordinate (seedOne F) (initialOne F) seedOne_coordinate n,
        second_input_degree W n⟩
    · intro start n
      exact ⟨focusTransport_bornVariable start n,transported_degree W start n⟩
    · intro stage
      exact recursivePolynomial_natDegree W stage
  · exact input_position_cases

#print axioms all_input_degree_audit_certificate
#print axioms base_cycle
#print axioms base_first_degree
#print axioms base_second_degree
#print axioms input_position_cases
end
end RecursiveSemaevAllDegreesAudit
