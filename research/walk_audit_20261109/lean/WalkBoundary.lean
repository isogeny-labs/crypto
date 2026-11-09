import Mathlib.Probability.ProbabilityMassFunction.Constructions
import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

/-! A known fixed-support obstruction, represented by an actual PMF of
uniform iid step histories. It does not refute Hildebrand1994 Theorem1,
which averages over uniformly chosen distinct supports. -/
namespace WalkBoundaryAudit
noncomputable section
open scoped ENNReal BigOperators

def uniformPMF (α : Type*) [Fintype α] [Nonempty α] : PMF α :=
  PMF.ofFintype (fun _ => (Fintype.card α : ℝ≥0∞)⁻¹) (by
    simp only [Finset.sum_const, Finset.card_univ, nsmul_eq_mul]
    exact ENNReal.mul_inv_cancel (by exact_mod_cast Fintype.card_ne_zero)
      (by simp))

@[simp] theorem uniformPMF_apply (α : Type*) [Fintype α] [Nonempty α] (a : α) :
    uniformPMF α a = (Fintype.card α : ℝ≥0∞)⁻¹ := rfl

def position (p t : ℕ) (history : Fin t → Fin 4) : ZMod p :=
  ∑ i, ((history i).val : ZMod p)

/-- Exactly t independent uniform choices among increments0,1,2,3. -/
def walkLaw (p t : ℕ) : PMF (ZMod p) :=
  (uniformPMF (Fin t → Fin 4)).map (position p t)

theorem history_product_probability (t : ℕ) (h : Fin t → Fin 4) :
    uniformPMF (Fin t → Fin 4) h = ∏ _ : Fin t, (4 : ℝ≥0∞)⁻¹ := by
  simp [uniformPMF_apply, ENNReal.inv_pow]

theorem increments_distinct (p : ℕ) [NeZero p] (hp : 4 ≤ p) :
    Function.Injective (fun i : Fin 4 => (i.val : ZMod p)) := by
  intro i j hij
  apply Fin.ext
  have hi : i.val < p := lt_of_lt_of_le i.isLt hp
  have hj : j.val < p := lt_of_lt_of_le j.isLt hp
  have h := (ZMod.natCast_eq_natCast_iff' i.val j.val p).mp hij
  simpa [Nat.mod_eq_of_lt hi, Nat.mod_eq_of_lt hj] using h

def arcEvent (p t : ℕ) : Finset (ZMod p) :=
  (Finset.range (3*t+1)).image (fun n : ℕ => (n : ZMod p))

theorem history_sum_le (t : ℕ) (history : Fin t → Fin 4) :
    (∑ i, (history i).val) ≤ 3*t := by
  calc
    _ ≤ ∑ _ : Fin t, 3 := Finset.sum_le_sum (fun i _ => by
      have h := (history i).isLt
      omega)
    _ = 3*t := by simp [Nat.mul_comm]

theorem position_mem_arc (p t : ℕ) (history : Fin t → Fin 4) :
    position p t history ∈ arcEvent p t := by
  apply Finset.mem_image.mpr
  refine ⟨∑ i, (history i).val, Finset.mem_range.mpr ?_, ?_⟩
  · have h := history_sum_le t history
    omega
  · simp [position]

theorem walk_event_mass_one (p t : ℕ) :
    (walkLaw p t).toOuterMeasure (arcEvent p t : Set (ZMod p)) = 1 := by
  apply (PMF.toOuterMeasure_apply_eq_one_iff _ _).mpr
  intro z hz
  rcases (PMF.mem_support_map_iff _ _ _).mp hz with ⟨h, _, rfl⟩
  exact position_mem_arc p t h

theorem arc_card_le (p t : ℕ) : (arcEvent p t).card ≤ 3*t+1 := by
  exact (Finset.card_image_le).trans (by simp)

theorem uniform_arc_mass_bound (p t : ℕ) [NeZero p] :
    (uniformPMF (ZMod p)).toOuterMeasure (arcEvent p t : Set (ZMod p)) ≤
      (3*t+1 : ℝ≥0∞) / p := by
  rw [PMF.toOuterMeasure_apply_finset]
  simp only [uniformPMF_apply, Finset.sum_const, nsmul_eq_mul, ZMod.card]
  rw [div_eq_mul_inv]
  gcongr
  exact_mod_cast arc_card_le p t

theorem uniform_arc_mass_101 :
    (uniformPMF (ZMod 101)).toOuterMeasure (arcEvent 101 10 : Set (ZMod 101)) =
      (31:ℝ≥0∞)/101 := by
  rw [PMF.toOuterMeasure_apply_finset]
  simp only [uniformPMF_apply, Finset.sum_const, nsmul_eq_mul, ZMod.card]
  have hcard : (arcEvent 101 10).card = 31 := by decide
  rw [hcard]
  simp [div_eq_mul_inv]

theorem uniform_arc_mass_le_half (p t : ℕ) [NeZero p] (h : 6*t+2 ≤ p) :
    (uniformPMF (ZMod p)).toOuterMeasure (arcEvent p t : Set (ZMod p)) ≤ 1/2 := by
  apply (uniform_arc_mass_bound p t).trans
  apply (ENNReal.div_le_iff (by exact_mod_cast NeZero.ne p) (by simp)).mpr
  rw [show (1/2 : ℝ≥0∞) * p = (p:ℝ≥0∞)/2 by simp [div_eq_mul_inv, mul_comm]]
  apply (ENNReal.le_div_iff_mul_le (Or.inl (by norm_num)) (Or.inl (by norm_num))).mpr
  norm_cast
  omega

/-- The actual walk fails the event characterization of 1/4 total-variation
mixing, for a linear number of steps in p. No random-support average. -/
theorem fixed_support_not_mixed (p t : ℕ) [NeZero p] (h : 6*t+2 ≤ p) :
    ¬ ∀ s : Set (ZMod p), (walkLaw p t).toOuterMeasure s ≤
      (uniformPMF (ZMod p)).toOuterMeasure s + 1/4 := by
  intro hmixed
  have he := hmixed (arcEvent p t : Set (ZMod p))
  rw [walk_event_mass_one] at he
  have hu := uniform_arc_mass_le_half p t h
  have hconst : (1/2 : ℝ≥0∞) + 1/4 < 1 := by
    have hc : ((1/2+1/4:NNReal):ℝ≥0∞) < 1 := by
      exact_mod_cast (by norm_num : (1/2+1/4:NNReal) < 1)
    simpa only [ENNReal.coe_add, ENNReal.coe_div (by norm_num : (2:NNReal) ≠ 0),
      ENNReal.coe_div (by norm_num : (4:NNReal) ≠ 0), ENNReal.coe_one,
      ENNReal.coe_ofNat] using hc
  have hsmall : (uniformPMF (ZMod p)).toOuterMeasure
      (arcEvent p t : Set (ZMod p)) + 1/4 < 1 :=
    lt_of_le_of_lt (add_le_add hu le_rfl) hconst
  exact not_le_of_gt hsmall he

theorem concrete_prime_walk_not_mixed :
    ¬ ∀ s : Set (ZMod 101), (walkLaw 101 10).toOuterMeasure s ≤
      (uniformPMF (ZMod 101)).toOuterMeasure s + 1/4 := by
  exact fixed_support_not_mixed 101 10 (by decide)

/-- Any deterministic iteration on the actual group repeats among its first
p+1 states. This is a bound, not a collision-time distribution theorem. -/
theorem iterates_repeat (p : ℕ) [NeZero p] (f : ZMod p → ZMod p) (x : ZMod p) :
    ∃ i j : Fin (p+1), i ≠ j ∧ f^[i.val] x = f^[j.val] x := by
  classical
  by_contra hn
  have hi : Function.Injective (fun i : Fin (p+1) => f^[i.val] x) := by
    intro i j hij
    by_contra hne
    exact hn ⟨i,j,hne,hij⟩
  have hcard := Fintype.card_le_of_injective _ hi
  simp only [Fintype.card_fin, ZMod.card] at hcard
  omega

theorem concrete_prime_walk_counterexample :
    Nat.Prime 101 ∧ Function.Injective (fun i : Fin 4 => (i.val : ZMod 101)) ∧
    (walkLaw 101 10).toOuterMeasure (arcEvent 101 10 : Set (ZMod 101)) = 1 ∧
    (uniformPMF (ZMod 101)).toOuterMeasure (arcEvent 101 10 : Set (ZMod 101)) =
      (31:ℝ≥0∞)/101 ∧
    ¬ ∀ s : Set (ZMod 101), (walkLaw 101 10).toOuterMeasure s ≤
      (uniformPMF (ZMod 101)).toOuterMeasure s + 1/4 := by
  exact ⟨by norm_num, increments_distinct 101 (by decide), walk_event_mass_one 101 10,
    uniform_arc_mass_101, concrete_prime_walk_not_mixed⟩

/-- A full-support step table transports uniformly sampled hash functions
bijectively to uniformly sampled endofunctions. -/
def fullSupportMapEquiv (G : Type*) [AddCommGroup G] : (G → G) ≃ (G → G) where
  toFun h x := x + h x
  invFun f x := f x - x
  left_inv h := by ext x; simp
  right_inv f := by ext x; simp

theorem uniformPMF_map_equiv {α : Type*} [Fintype α] [Nonempty α] (e : α ≃ α) :
    (uniformPMF α).map e = uniformPMF α := by
  classical
  ext b
  rw [PMF.map_apply, tsum_eq_single (e.symm b)]
  · simp
  · intro a ha
    have hb : b ≠ e a := by
      intro hh
      apply ha
      apply e.injective
      simpa using hh.symm
    simp [hb]

theorem full_support_hash_law (p : ℕ) [NeZero p] :
    (uniformPMF (ZMod p → ZMod p)).map (fullSupportMapEquiv (ZMod p)) =
      uniformPMF (ZMod p → ZMod p) := uniformPMF_map_equiv _

/-- Actual rho transition on Z/3 with the two nonzero distinct increments. -/
def smallRhoStep (hash : ZMod 3 → Fin 2) (x : ZMod 3) : ZMod 3 :=
  x + ((hash x).val : ZMod 3) + 1

def smallInDegree (hash : ZMod 3 → Fin 2) : ℚ :=
  (Finset.univ.filter (fun x => smallRhoStep hash x = 0)).card

theorem small_hash_space_cardinality : Fintype.card (ZMod 3 → Fin 2) = 8 := by simp

theorem small_increments_distinct_nonzero :
    Function.Injective (fun i : Fin 2 => ((i.val:ZMod 3)+1)) ∧
    ∀ i : Fin 2, (i.val:ZMod 3)+1 ≠ 0 := by decide

/-- Closed actual-graph counterexample to the literal reciprocal variance
sentence in Galbraith ch14 p297. Not a collision-time heuristic refutation. -/
theorem actual_indegree_variance_counterexample :
    (∑ h : ZMod 3 → Fin 2, smallInDegree h) / (Fintype.card (ZMod 3 → Fin 2):ℚ) = 1 ∧
    (∑ h : ZMod 3 → Fin 2, (smallInDegree h - 1)^2) /
      (Fintype.card (ZMod 3 → Fin 2):ℚ) = 1/2 ∧
    (∑ h : ZMod 3 → Fin 2, (smallInDegree h - 1)^2) /
      (Fintype.card (ZMod 3 → Fin 2):ℚ) ≠ 2 := by
  rw [small_hash_space_cardinality]
  unfold smallInDegree smallRhoStep
  decide +kernel

end
end WalkBoundaryAudit
