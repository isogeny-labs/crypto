import F4Containment

/-! A domain boundary for the actual imported F4Run model. Ordinary model
runs stay in the input ideal; saturation may add polynomials outside it.
This is not a refinement theorem for msolve's C implementation. -/
namespace F4SaturationBoundaryAudit
noncomputable section
open MvPolynomial SolvingDegreeBound
variable {σ K : Type*} [Field K]

theorem every_run_in_input_ideal (D : ℕ) (F : Set (MvPolynomial σ K))
    (G : ℕ→Set (MvPolynomial σ K)) (hrun : F4Run D F G) :
    ∀ t,G t⊆Ideal.span F := by
  have hclosed : DClosed D ((Ideal.span F).restrictScalars K) := by
    intro f hf _ i
    exact (Ideal.span F).mul_mem_left _ hf
  exact run_subset hclosed (fun f hf=>Ideal.subset_span hf) hrun

/-- Standard saturation membership, with a concrete power witness. -/
def SaturationMember {R : Type*} [CommRing R] (I : Ideal R) (h p : R) : Prop :=
  ∃ n : ℕ,h^n*p∈I

abbrev Poly (K : Type*) [Field K] := MvPolynomial (Fin 2) K
def x : Poly K := X 0
def y : Poly K := X 1
def generators : Set (Poly K) := {x*y}

def atZeroOne : Poly K→+*K := MvPolynomial.eval ![0,1]

theorem eval_x : atZeroOne (x (K:=K))=0 := by simp [atZeroOne,x]
theorem eval_y : atZeroOne (y (K:=K))=1 := by simp [atZeroOne,y]

theorem inputIdeal_le_kernel :
    Ideal.span (generators (K:=K))≤RingHom.ker (atZeroOne (K:=K)) := by
  apply Ideal.span_le.mpr
  intro p hp
  change p=x*y at hp
  subst p
  change atZeroOne (x*y)=0
  rw [map_mul,eval_x,zero_mul]

theorem y_not_in_input_ideal : y (K:=K)∉Ideal.span generators := by
  intro h
  have hz : atZeroOne (y (K:=K))=0 := inputIdeal_le_kernel h
  rw [eval_y] at hz
  exact one_ne_zero hz

theorem y_in_saturation :
    SaturationMember (Ideal.span (generators (K:=K))) x y := by
  refine ⟨1,?_⟩
  rw [pow_one]
  apply Ideal.subset_span
  rfl

theorem saturation_output_cannot_be_F4Run (D t : ℕ)
    (G : ℕ→Set (Poly K)) (hy : y∈G t) : ¬F4Run D generators G := by
  intro hrun
  exact y_not_in_input_ideal (every_run_in_input_ideal D generators G hrun t hy)

theorem unrestricted_saturation_model_claim_false :
    ¬ ∀ (F : Set (Poly K)) (h p : Poly K),
      SaturationMember (Ideal.span F) h p →
        ∃ (D t : ℕ) (G : ℕ→Set (Poly K)),F4Run D F G ∧ p∈G t := by
  intro h
  obtain ⟨D,t,G,hrun,hy⟩ := h generators x y y_in_saturation
  exact saturation_output_cannot_be_F4Run D t G hy hrun

def booleanGenerators : Set (Poly K) := {x*y,x^2-x,y^2-y}

theorem booleanIdeal_le_kernel :
    Ideal.span (booleanGenerators (K:=K))≤RingHom.ker (atZeroOne (K:=K)) := by
  apply Ideal.span_le.mpr
  intro p hp
  simp only [booleanGenerators,Set.mem_insert_iff,Set.mem_singleton_iff] at hp
  rcases hp with rfl | rfl | rfl <;> change atZeroOne _=0
  all_goals simp [atZeroOne,x,y]

theorem y_not_in_boolean_ideal : y (K:=K)∉Ideal.span booleanGenerators := by
  intro h
  have hz : atZeroOne (y (K:=K))=0 := booleanIdeal_le_kernel h
  rw [eval_y] at hz
  exact one_ne_zero hz

theorem y_in_boolean_saturation :
    SaturationMember (Ideal.span (booleanGenerators (K:=K))) x y := by
  refine ⟨1,?_⟩
  rw [pow_one]
  apply Ideal.subset_span
  simp [booleanGenerators]

theorem x_minus_one_in_boolean_saturation :
    SaturationMember (Ideal.span (booleanGenerators (K:=K))) x (x-1) := by
  refine ⟨1,?_⟩
  rw [pow_one]
  have heq : x (K:=K)*(x-1)=x^2-x := by ring
  rw [heq]
  apply Ideal.subset_span
  simp [booleanGenerators]

theorem boolean_saturation_output_cannot_be_F4Run (D t : ℕ)
    (G : ℕ→Set (Poly K)) (hy : y∈G t) : ¬F4Run D booleanGenerators G := by
  intro hrun
  exact y_not_in_boolean_ideal
    (every_run_in_input_ideal D booleanGenerators G hrun t hy)

theorem f4_saturation_audit_certificate :
    (∀ (V A : Type*) [Field A] (D : ℕ) (F : Set (MvPolynomial V A))
      (G : ℕ→Set (MvPolynomial V A)),F4Run D F G → ∀ t,G t⊆Ideal.span F) ∧
    (∀ (A : Type*) [Field A],
      SaturationMember (Ideal.span (generators (K:=A))) x y ∧
      y (K:=A)∉Ideal.span generators ∧
      ¬ ∀ (F : Set (Poly A)) (h p : Poly A),
        SaturationMember (Ideal.span F) h p →
          ∃ (D t : ℕ) (G : ℕ→Set (Poly A)),F4Run D F G ∧ p∈G t) ∧
    (∀ (A : Type*) [Field A],
      SaturationMember (Ideal.span (booleanGenerators (K:=A))) x y ∧
      y (K:=A)∉Ideal.span booleanGenerators ∧
      ∀ (D t : ℕ) (G : ℕ→Set (Poly A)),y∈G t → ¬F4Run D booleanGenerators G) := by
  constructor
  · intro V A _ D F G hrun
    exact every_run_in_input_ideal D F G hrun
  · constructor
    · intro A _
      exact ⟨y_in_saturation,y_not_in_input_ideal,unrestricted_saturation_model_claim_false⟩
    · intro A _
      exact ⟨y_in_boolean_saturation,y_not_in_boolean_ideal,
        boolean_saturation_output_cannot_be_F4Run⟩

#print axioms SolvingDegreeBound.run_subset
#print axioms every_run_in_input_ideal
#print axioms y_not_in_input_ideal
#print axioms y_in_saturation
#print axioms unrestricted_saturation_model_claim_false
#print axioms boolean_saturation_output_cannot_be_F4Run
#print axioms f4_saturation_audit_certificate
end
end F4SaturationBoundaryAudit
