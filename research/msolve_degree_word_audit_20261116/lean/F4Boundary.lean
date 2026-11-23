import DegreeFamily

namespace MsolveDegreeWordAudit
noncomputable section
open MvPolynomial SolvingDegreeBound
variable {K : Type*} [Field K]

/-- A bound on the actual model cannot be replaced by the stored degree word. -/
theorem no_lower_run (n : ℕ) (G : ℕ → Set (Poly K))
    (hG : F4Run (n+2) (inputs n) G) (t : ℕ) : linearY ∉ G t := by
  intro hy
  have hsub := run_subset (input_span_closed (K:=K) n)
    (fun _ hp => Submodule.subset_span hp) hG t
  exact linearY_not_span n (hsub hy)

theorem first_ne_zero (n : ℕ) : (firstPoly n : Poly K) ≠ 0 := by
  intro h
  have hd := first_degree (K:=K) n
  rw [h, totalDegree_zero] at hd
  omega

theorem second_ne_zero (n : ℕ) : (secondPoly n : Poly K) ≠ 0 := by
  intro h
  have hd := second_degree (K:=K) n
  rw [h, totalDegree_zero] at hd
  omega

theorem first_row_degree (n : ℕ) :
    (monomial exponentY (1 : K)*firstPoly n).totalDegree = n+3 := by
  have hm : (monomial exponentY (1 : K) : Poly K) ≠ 0 := by simp
  calc
    _ = (monomial exponentY (1 : K) : Poly K).totalDegree + (firstPoly n : Poly K).totalDegree :=
      totalDegree_mul_of_isDomain hm (first_ne_zero n)
    _ = 1+(n+2) := by simp [totalDegree_monomial, exponentY, first_degree]
    _ = n+3 := by omega

theorem second_row_degree (n : ℕ) :
    (monomial exponentX (1 : K)*secondPoly n).totalDegree = n+3 := by
  have hm : (monomial exponentX (1 : K) : Poly K) ≠ 0 := by simp
  calc
    _ = (monomial exponentX (1 : K) : Poly K).totalDegree + (secondPoly n : Poly K).totalDegree :=
      totalDegree_mul_of_isDomain hm (second_ne_zero n)
    _ = 1+(n+2) := by simp [totalDegree_monomial, exponentX, second_degree]
    _ = n+3 := by omega

/-- Actual finite computation continued constantly after its first step. -/
def healthyRun (n : ℕ) : ℕ → Set (Poly K)
  | 0 => inputs n
  | _+1 => insert linearY (inputs n)

theorem healthy_run (n : ℕ) : F4Run (n+3) (inputs n) (healthyRun n : ℕ → Set (Poly K)) := by
  constructor
  · exact fun _ hp => Submodule.subset_span hp
  · intro t p hp
    cases t with
    | zero =>
      change p ∈ insert linearY (inputs n) at hp
      rcases Set.mem_insert_iff.mp hp with rfl | hp
      · have hr0 : monomial exponentY (1 : K)*firstPoly n ∈
            StepRows (n+3) (Past (healthyRun n) 0) := by
          refine ⟨exponentY, firstPoly n, ?_, rfl, ?_⟩
          · exact ⟨0, le_rfl, by simp [healthyRun, inputs]⟩
          · rw [first_row_degree]
        have hr1 : monomial exponentX (1 : K)*secondPoly n ∈
            StepRows (n+3) (Past (healthyRun n) 0) := by
          refine ⟨exponentX, secondPoly n, ?_, rfl, ?_⟩
          · exact ⟨0, le_rfl, by simp [healthyRun, inputs]⟩
          · rw [second_row_degree]
        rw [← reduction_identity n]
        exact Submodule.sub_mem _ (Submodule.subset_span (Or.inr hr0))
          (Submodule.subset_span (Or.inr hr1))
      · apply Submodule.subset_span
        exact Or.inl ⟨0, le_rfl, hp⟩
    | succ t =>
      apply Submodule.subset_span
      exact Or.inl ⟨t+1, le_rfl, by simpa only [healthyRun] using hp⟩

theorem healthy_run_contains_y (n : ℕ) : (linearY : Poly K) ∈ healthyRun n 1 := by
  simp [healthyRun]

theorem small_degree_fall_control :
    F4Run 3 (inputs 0) (healthyRun 0 : ℕ → Set (Poly K)) ∧
    (∀ G : ℕ → Set (Poly K), F4Run 2 (inputs 0) G → ∀ t, linearY ∉ G t) :=
  ⟨healthy_run 0, fun G hG => no_lower_run 0 G hG⟩

theorem boundary_stored_bound : max 65535 (lcmDegreeWord 65535 0 65534 1).toNat = 65535 := by
  rw [boundary_lcm_word]
  norm_num

theorem boundary_no_run :
    ¬ ∃ (G : ℕ → Set (Poly (ZMod 101))) (t : ℕ),
      F4Run 65535 (inputs 65533) G ∧ linearY ∈ G t := by
  rintro ⟨G, t, hG, hy⟩
  exact no_lower_run 65533 G hG t hy

#print axioms no_lower_run
#print axioms healthy_run
#print axioms boundary_no_run
end
end MsolveDegreeWordAudit
