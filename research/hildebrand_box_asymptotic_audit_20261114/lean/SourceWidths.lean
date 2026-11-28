module

public import DenominatorLimits

@[expose] public section

namespace HildebrandBoxAsymptoticAudit

open Filter
open scoped Topology

/-- Unnormalized integer-coordinate width in the author's corrected lemma,
with d=k-1. Retaining the real powers makes the source specialization explicit. -/
noncomputable def sourceWidth (d : ℕ) (ε : ℝ) (n : ℕ) : ℝ :=
  (ε/2)^(1/(d : ℝ))*(n : ℝ)^(((d : ℝ)-1)/(d : ℝ))/2

noncomputable def sourceCoefficient (d : ℕ) (ε : ℝ) : ℝ := (ε/2)/(2 : ℝ)^d

theorem source_coefficient_positive (d : ℕ) {ε : ℝ} (hε : 0<ε) :
    0<sourceCoefficient d ε := by unfold sourceCoefficient; positivity

theorem source_width_positive (d : ℕ) {ε : ℝ} (hε : 0<ε) {n : ℕ} (hn : 0<n) :
    0<sourceWidth d ε n := by
  have hn' : (0 : ℝ)<n := by exact_mod_cast hn
  unfold sourceWidth
  positivity

theorem source_width_tendsto {d : ℕ} (hd : 2 ≤ d) {ε : ℝ} (hε : 0<ε) :
    Tendsto (sourceWidth d ε) atTop atTop := by
  have hd' : (2 : ℝ) ≤ d := by exact_mod_cast hd
  have ha : 0<(ε/2)^(1/(d : ℝ)) := Real.rpow_pos_of_pos (by linarith) _
  have hb : 0<((d : ℝ)-1)/(d : ℝ) := div_pos (by linarith) (by linarith)
  exact (((tendsto_rpow_atTop hb).comp tendsto_natCast_atTop_atTop).const_mul_atTop ha).atTop_div_const (by norm_num)

theorem source_width_power {d : ℕ} (hd : 1 ≤ d) {ε : ℝ} (hε : 0<ε) (n : ℕ) :
    (sourceWidth d ε n)^d=sourceCoefficient d ε*(n : ℝ)^(d-1) := by
  have hd0 : (d : ℝ) ≠ 0 := by exact_mod_cast (show d ≠ 0 by omega)
  have ha : ((ε/2)^(1/(d : ℝ)))^d=ε/2 := by
    rw [← Real.rpow_natCast,← Real.rpow_mul (by positivity),one_div_mul_cancel hd0,Real.rpow_one]
  have hexp : (((d : ℝ)-1)/(d : ℝ))*(d : ℝ)=((d-1 : ℕ) : ℝ) := by
    rw [Nat.cast_sub hd,Nat.cast_one]
    field_simp
  have hb : ((n : ℝ)^(((d : ℝ)-1)/(d : ℝ)))^d=(n : ℝ)^(d-1) := by
    rw [← Real.rpow_natCast,← Real.rpow_mul (by positivity),hexp,Real.rpow_natCast]
  unfold sourceWidth sourceCoefficient
  rw [div_pow,mul_pow,ha,hb]
  ring

theorem source_width_one (ε : ℝ) (n : ℕ) : sourceWidth 1 ε n=ε/4 := by
  norm_num [sourceWidth]
  ring

theorem one_width_not_tendsto_atTop (ε : ℝ) :
    ¬ Tendsto (sourceWidth 1 ε) atTop atTop := by
  intro h
  have he : ∀ᶠ n : ℕ in atTop, ε/4+1<sourceWidth 1 ε n :=
    h.eventually_gt_atTop (ε/4+1)
  obtain ⟨n,hn⟩ := he.exists
  rw [source_width_one] at hn
  linarith

end HildebrandBoxAsymptoticAudit
