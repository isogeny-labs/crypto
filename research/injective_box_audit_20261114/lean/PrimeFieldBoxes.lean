module

public import GeneralSupports
public import IntegerCounts
public import CenteredCell

@[expose] public section

namespace InjectiveBoxAudit

open HildebrandCellAudit CycleCouplingAudit

variable {p d : ℕ} [Fact p.Prime]

noncomputable def coordinateBox (j : ZMod p) (L U : ℝ) : Finset (Choice p) := by
  classical
  exact Finset.univ.filter (fun u => L ≤ (centered (-u.val*j) : ℝ)/p ∧
    (centered (-u.val*j) : ℝ)/p ≤ U)

theorem mem_coordinate_box (j : ZMod p) (L U : ℝ) (u : Choice p) :
    u ∈ coordinateBox j L U ↔ L ≤ (centered (-u.val*j) : ℝ)/p ∧
      (centered (-u.val*j) : ℝ)/p ≤ U := by
  classical
  simp [coordinateBox]

theorem transformed_center_injective (j : ZMod p) (hj : j ≠ 0) :
    Function.Injective (fun u : Choice p => centered (-u.val*j)) := by
  intro u v h
  have hc := congrArg (fun z : ℤ => (z : ZMod p)) h
  simp only [centered_cast] at hc
  have hz : ((-u.val)-(-v.val))*j=0 := by rw [sub_mul,hc,sub_self]
  have he : -u.val = -v.val := sub_eq_zero.mp ((mul_eq_zero.mp hz).resolve_right hj)
  exact Subtype.ext (neg_injective he)

theorem coordinate_card_bound (j : ZMod p) (hj : j ≠ 0) (L U : ℝ) (hLU : L ≤ U) :
    ((coordinateBox j L U).card : ℝ) ≤ (p : ℝ)*(U-L)+1 := by
  have hp : (0 : ℝ)<p := by exact_mod_cast (Fact.out : p.Prime).pos
  have hb := integer_image_card_bound (coordinateBox j L U)
    (fun u => centered (-u.val*j)) (transformed_center_injective j hj)
    (L*p) (U*p) (mul_le_mul_of_nonneg_right hLU hp.le) (by
      intro u hu
      have hm := (mem_coordinate_box j L U u).mp hu
      exact ⟨(le_div_iff₀ hp).mp hm.1,(div_le_iff₀ hp).mp hm.2⟩)
  nlinarith

noncomputable def sourceBoxEvent (a₀ j : ZMod p) (L U : Fin d → ℝ) :
    Finset (FullSupports p d a₀) := by
  classical
  exact Finset.univ.filter (fun f => ∀ i,
    L i ≤ (centered ((a₀-f.val i.succ)*j) : ℝ)/p ∧
      (centered ((a₀-f.val i.succ)*j) : ℝ)/p ≤ U i)

theorem source_box_pullback (a₀ j : ZMod p) (L U : Fin d → ℝ) (e : Sample (Choice p) d) :
    sourceEquiv a₀ e ∈ sourceBoxEvent a₀ j L U ↔
      e ∈ boxEvent (fun i => coordinateBox j (L i) (U i)) := by
  classical
  simp only [sourceBoxEvent,Finset.mem_filter,Finset.mem_univ,true_and]
  change (∀ i, L i ≤ (centered ((a₀-supportsFromSample a₀ e i.succ)*j) : ℝ)/p ∧
    (centered ((a₀-supportsFromSample a₀ e i.succ)*j) : ℝ)/p ≤ U i) ↔ _
  simp_rw [(full_source_argument a₀ j e _).2]
  simp only [boxEvent,Finset.mem_filter,Finset.mem_univ,true_and,mem_coordinate_box]

noncomputable def sourceProbability (p d : ℕ) [Fact p.Prime] (a₀ j : ZMod p)
    (h : d ≤ p-1) (L U : Fin d → ℝ) : ℝ :=
  expectation (sourceLaw p d a₀ h) (indicator (sourceBoxEvent a₀ j L U))

theorem source_probability_eq (a₀ j : ZMod p) (h : d ≤ p-1) (L U : Fin d → ℝ) :
    sourceProbability p d a₀ j h L U =
      boxProbability (Choice p) d (by rw [choice_card]; exact h)
        (fun i => coordinateBox j (L i) (U i)) := by
  rw [sourceProbability,sourceLaw,expectation_map]
  unfold boxProbability
  congr 1
  funext e
  unfold indicator
  by_cases hm : e ∈ boxEvent (fun i => coordinateBox j (L i) (U i))
  · have hs := (source_box_pullback a₀ j L U e).mpr hm
    simp only [hm,hs,↓reduceIte]
  · have hs : ¬ sourceEquiv a₀ e ∈ sourceBoxEvent a₀ j L U := by
      intro hs
      exact hm ((source_box_pullback a₀ j L U e).mp hs)
    simp only [hm,hs,↓reduceIte]

theorem source_probability_bound (a₀ j : ZMod p) (hj : j ≠ 0) (h : d ≤ p-1)
    (L U : Fin d → ℝ) (hLU : ∀ i, L i ≤ U i) :
    sourceProbability p d a₀ j h L U ≤
      (∏ i, ((p : ℝ)*(U i-L i)+1))/((p-1).descFactorial d : ℝ) := by
  rw [source_probability_eq]
  have hn : d ≤ Fintype.card (Choice p) := by rw [choice_card]; exact h
  have hb := box_probability_bound hn (fun i => coordinateBox j (L i) (U i))
  have hd : ((Fintype.card (Choice p)).descFactorial d : ℝ) =
      ((p-1).descFactorial d : ℝ) := by rw [choice_card]
  rw [hd] at hb
  have hc : ((∏ i, (coordinateBox j (L i) (U i)).card : ℕ) : ℝ) ≤
      ∏ i, ((p : ℝ)*(U i-L i)+1) := by
    rw [Nat.cast_prod]
    exact Finset.prod_le_prod₀ (fun i _ => Nat.cast_nonneg _)
      (fun i _ => coordinate_card_bound j hj (L i) (U i) (hLU i))
  exact hb.trans (div_le_div_of_nonneg_right hc (by positivity))

theorem zero_index_card : (coordinateBox (0 : ZMod p) 0 0).card=p-1 := by
  classical
  simp [coordinateBox,centered,choice_card]

theorem zero_index_bound_false (hp : 3 ≤ p) :
    ¬ ((coordinateBox (0 : ZMod p) 0 0).card : ℝ) ≤ (p : ℝ)*(0-0)+1 := by
  rw [zero_index_card,Nat.cast_sub (by omega : 1 ≤ p),Nat.cast_one]
  have hp' : (3 : ℝ) ≤ p := by exact_mod_cast hp
  norm_num
  linarith

theorem zero_index_probability (a₀ : ZMod p) (h : d ≤ p-1) :
    sourceProbability p d a₀ 0 h (fun _ => 0) (fun _ => 0) = 1 := by
  classical
  have he : sourceBoxEvent a₀ (0 : ZMod p) (fun _ : Fin d => 0) (fun _ => 0) = Finset.univ := by
    simp [sourceBoxEvent,centered]
  have hi : indicator (Finset.univ : Finset (FullSupports p d a₀)) = (fun _ => 1) := by
    funext f
    simp [indicator]
  rw [sourceProbability,he,hi]
  exact expectation_one _

omit [Fact p.Prime] in
theorem one_dimension_fits (hp : 3 ≤ p) : 1 ≤ p-1 := by omega

theorem zero_index_probability_bound_false (hp : 3 ≤ p) (a₀ : ZMod p) :
    ¬ sourceProbability p 1 a₀ 0 (one_dimension_fits hp) (fun _ => 0) (fun _ => 0) ≤
      1/((p-1 : ℕ) : ℝ) := by
  rw [zero_index_probability]
  have hn : (1 : ℝ)<(p-1 : ℕ) := by exact_mod_cast (show 1<p-1 by omega)
  have hp' : (0 : ℝ)<(p-1 : ℕ) := by linarith
  intro h
  have he := (le_div_iff₀ hp').mp h
  linarith

end InjectiveBoxAudit
