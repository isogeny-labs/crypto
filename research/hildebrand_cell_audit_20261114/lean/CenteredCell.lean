module

public import UniformChoices
public import Mathlib.Analysis.SpecialFunctions.Pow.Real

@[expose] public section

namespace HildebrandCellAudit

/-- Literal printed cell at k=2, ε=1, m₂=4; no supplied reduced endpoint. -/
noncomputable def printedLower (p : ℕ) : ℝ :=
  4 * Real.rpow ((1 : ℝ)/2) (1/((2 : ℝ)-1)) *
    Real.rpow (p : ℝ) (((2 : ℝ)-2)/((2 : ℝ)-1)) / (2*p)

noncomputable def printedUpper (p : ℕ) : ℝ :=
  (4+1) * Real.rpow ((1 : ℝ)/2) (1/((2 : ℝ)-1)) *
    Real.rpow (p : ℝ) (((2 : ℝ)-2)/((2 : ℝ)-1)) / (2*p)

noncomputable def printedBound (p : ℕ) : ℝ :=
  1.1 * ((1 : ℝ)/2) / ((2 : ℝ)^((2 : ℕ)-1) * p)

theorem printed_formulas (p : ℕ) (hp : 0 < p) :
    printedLower p = 1/(p : ℝ) ∧ printedUpper p = 5/(4*(p : ℝ)) ∧
      printedBound p = 11/(40*(p : ℝ)) := by
  have hp' : (p : ℝ) ≠ 0 := by exact_mod_cast hp.ne'
  refine ⟨?_,?_,?_⟩ <;>
    norm_num [printedLower,printedUpper,printedBound] <;> field_simp <;> ring

def centered {p : ℕ} (z : ZMod p) : ℤ :=
  if 2*z.val ≤ p then (z.val : ℤ) else (z.val : ℤ)-p

variable {p : ℕ} [Fact p.Prime]

theorem centered_cast (z : ZMod p) : (centered z : ZMod p) = z := by
  unfold centered
  split_ifs <;> simp

theorem centered_range (z : ZMod p) :
    -(p : ℤ) < 2*centered z ∧ 2*centered z ≤ p := by
  have hv := ZMod.val_lt z
  have hp := (Fact.out : p.Prime).pos
  unfold centered
  split_ifs <;> constructor <;> omega

theorem centered_unique (z : ZMod p) (v : ℤ) (hv : (v : ZMod p)=z)
    (hl : -(p : ℤ)<2*v) (hr : 2*v ≤ p) : v=centered z := by
  have hc := centered_range z
  have hcast : ((v-centered z : ℤ) : ZMod p)=0 := by
    rw [Int.cast_sub,hv,centered_cast,sub_self]
  have hd := (ZMod.intCast_zmod_eq_zero_iff_dvd (v-centered z) p).mp hcast
  obtain ⟨k,hk⟩ := hd
  have hp : (0 : ℤ)<p := by exact_mod_cast (Fact.out : p.Prime).pos
  by_cases hz : k=0
  · subst k; omega
  · have hh : k ≤ -1 ∨ 1 ≤ k := by omega
    rcases hh with hh|hh
    · have hm := mul_le_mul_of_nonneg_left hh hp.le
      nlinarith
    · have hm := mul_le_mul_of_nonneg_left hh hp.le
      nlinarith

theorem centered_eq_one (hp : 3 ≤ p) (z : ZMod p) : centered z=1 ↔ z=1 := by
  constructor
  · intro h
    have hc := centered_cast z
    rw [h] at hc
    simpa using hc.symm
  · intro h
    subst z
    unfold centered
    rw [ZMod.val_one'' (by omega)]
    simp [show 2 ≤ p by omega]

noncomputable def printedCell (u : Choice p) : Prop :=
  printedLower p ≤ (centered (-u.val) : ℝ)/p ∧
    (centered (-u.val) : ℝ)/p ≤ printedUpper p

theorem printed_cell_integer (hp : 3 ≤ p) (u : Choice p) :
    printedCell u ↔ centered (-u.val)=1 := by
  have hpr : (0 : ℝ)<p := by exact_mod_cast (Fact.out : p.Prime).pos
  have hupper : (5 : ℝ)/(4*p) = (5/4)/p := by field_simp
  unfold printedCell
  rw [(printed_formulas p (Fact.out : p.Prime).pos).1,
    (printed_formulas p (Fact.out : p.Prime).pos).2.1,hupper,
    div_le_div_iff_of_pos_right hpr,div_le_div_iff_of_pos_right hpr]
  constructor
  · rintro ⟨hl,hr⟩
    have hl' : (1 : ℤ) ≤ centered (-u.val) := by exact_mod_cast hl
    have hr₁ : (4 : ℝ)*(centered (-u.val) : ℝ) ≤ 5 := by linarith
    have hr' : (4 : ℤ)*centered (-u.val) ≤ 5 := by exact_mod_cast hr₁
    omega
  · intro h
    rw [h]
    norm_num

theorem printed_cell_singleton (hp : 3 ≤ p) (u : Choice p) :
    printedCell u ↔ u=minusOneChoice := by
  rw [printed_cell_integer hp,centered_eq_one hp]
  constructor
  · intro h
    apply Subtype.ext
    change u.val=(-1 : ZMod p)
    exact neg_eq_iff_eq_neg.mp h
  · intro h
    subst u
    simp [minusOneChoice]

end HildebrandCellAudit
