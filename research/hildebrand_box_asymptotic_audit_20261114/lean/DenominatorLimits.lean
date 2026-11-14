module

public import PrimeFieldBoxes
public import Mathlib.Data.Nat.Factorial.BigOperators
public import Mathlib.Analysis.SpecialFunctions.Pow.Asymptotics

@[expose] public section

namespace HildebrandBoxAsymptoticAudit

open Filter
open scoped Topology

noncomputable def normalizedDenominator (d n : ℕ) : ℝ :=
  ∏ i ∈ Finset.range d, (1-((i : ℝ)+1)/(n : ℝ))

theorem denominator_cast (d n : ℕ) (hn : d+1 ≤ n) :
    ((n-1).descFactorial d : ℝ) = ∏ i ∈ Finset.range d, ((n : ℝ)-((i : ℝ)+1)) := by
  rw [Nat.descFactorial_eq_prod_range,Nat.cast_prod]
  apply Finset.prod_congr rfl
  intro i hi
  have hid : i<d := Finset.mem_range.mp hi
  rw [Nat.cast_sub (by omega : i ≤ n-1),Nat.cast_sub (by omega : 1 ≤ n),Nat.cast_one]
  ring

theorem denominator_normalization (d n : ℕ) (hn : d+1 ≤ n) :
    ((n-1).descFactorial d : ℝ)/(n : ℝ)^d = normalizedDenominator d n := by
  have hn0 : (n : ℝ) ≠ 0 := by exact_mod_cast (show n ≠ 0 by omega)
  rw [denominator_cast d n hn]
  unfold normalizedDenominator
  calc
    _ = (∏ i ∈ Finset.range d, ((n : ℝ)-((i : ℝ)+1)))/
        (∏ _i ∈ Finset.range d, (n : ℝ)) := by simp
    _ = ∏ i ∈ Finset.range d, (((n : ℝ)-((i : ℝ)+1))/(n : ℝ)) :=
      (Finset.prod_div_distrib (fun i : ℕ => ((n : ℝ)-((i : ℝ)+1))) (fun _ => (n : ℝ))).symm
    _ = _ := by
      apply Finset.prod_congr rfl
      intro i _
      field_simp

theorem normalized_denominator_tendsto (d : ℕ) :
    Tendsto (normalizedDenominator d) atTop (𝓝 1) := by
  have hi : Tendsto (fun n : ℕ => (n : ℝ)⁻¹) atTop (𝓝 0) :=
    tendsto_natCast_atTop_atTop.inv_tendsto_atTop
  have hf : ∀ i ∈ Finset.range d,
      Tendsto (fun n : ℕ => 1-((i : ℝ)+1)/(n : ℝ)) atTop (𝓝 (1 : ℝ)) := by
    intro i _
    simpa only [mul_zero,sub_zero,div_eq_mul_inv] using
      (tendsto_const_nhds.sub (tendsto_const_nhds.mul hi) :
        Tendsto (fun n : ℕ => 1-((i : ℝ)+1)*(n : ℝ)⁻¹) atTop (𝓝 (1-((i : ℝ)+1)*0)))
  change Tendsto (fun n : ℕ => ∏ i ∈ Finset.range d, (1-((i : ℝ)+1)/(n : ℝ))) atTop (𝓝 1)
  simpa only [Finset.prod_const_one] using
    tendsto_finsetProd (Finset.range d) hf

theorem actual_denominator_tendsto (d : ℕ) :
    Tendsto (fun n : ℕ => ((n-1).descFactorial d : ℝ)/(n : ℝ)^d) atTop (𝓝 1) := by
  apply (normalized_denominator_tendsto d).congr'
  filter_upwards [eventually_ge_atTop (d+1)] with n hn
  exact (denominator_normalization d n hn).symm

end HildebrandBoxAsymptoticAudit
