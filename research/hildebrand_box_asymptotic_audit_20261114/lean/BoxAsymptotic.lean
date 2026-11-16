module

public import SourceWidths

@[expose] public section

namespace HildebrandBoxAsymptoticAudit

open Filter InjectiveBoxAudit
open scoped Topology

noncomputable def correctionFactor (d : ℕ) (ε : ℝ) (n : ℕ) : ℝ :=
  (1+1/sourceWidth d ε n)^d/normalizedDenominator d n

theorem correction_factor_tendsto {d : ℕ} (hd : 2 ≤ d) {ε : ℝ} (hε : 0<ε) :
    Tendsto (correctionFactor d ε) atTop (𝓝 1) := by
  have hi : Tendsto (fun n : ℕ => (sourceWidth d ε n)⁻¹) atTop (𝓝 (0 : ℝ)) :=
    (source_width_tendsto hd hε).inv_tendsto_atTop
  have hs : Tendsto (fun n : ℕ => 1+(sourceWidth d ε n)⁻¹) atTop (𝓝 (1+(0 : ℝ))) :=
    (tendsto_const_nhds : Tendsto (fun _ : ℕ => (1 : ℝ)) atTop (𝓝 1)).add hi
  have ht : Tendsto (fun n : ℕ => (1+(sourceWidth d ε n)⁻¹)^d) atTop (𝓝 (1 : ℝ)) := by
    simpa only [add_zero,one_pow] using hs.pow d
  change Tendsto (fun n : ℕ => (1+1/sourceWidth d ε n)^d/normalizedDenominator d n) atTop (𝓝 1)
  have hv := ht.div (normalized_denominator_tendsto d) (by norm_num : (1 : ℝ) ≠ 0)
  have hc : Tendsto (fun n : ℕ => (1+1/sourceWidth d ε n)^d/normalizedDenominator d n)
      atTop (𝓝 ((1 : ℝ)/1)) := hv.congr' (Eventually.of_forall (fun n => by
        simp only [Pi.div_apply,one_div]))
  simpa only [div_one] using hc

theorem finite_ratio_identity {d : ℕ} (hd : 1 ≤ d) {ε : ℝ} (hε : 0<ε)
    (n : ℕ) (hn : d+1 ≤ n) :
    (sourceWidth d ε n+1)^d/((n-1).descFactorial d : ℝ) =
      (sourceCoefficient d ε/(n : ℝ))*correctionFactor d ε n := by
  have hn0 : (n : ℝ) ≠ 0 := by exact_mod_cast (show n ≠ 0 by omega)
  have hw := source_width_positive d hε (show 0<n by omega)
  have hc := source_coefficient_positive d hε
  have hD : (0 : ℝ)<(n-1).descFactorial d := by
    exact_mod_cast (Nat.descFactorial_pos.mpr (show d ≤ n-1 by omega))
  have hwone : 1+1/sourceWidth d ε n=(sourceWidth d ε n+1)/sourceWidth d ε n := by
    field_simp
  have hnpow : (n : ℝ)^d=(n : ℝ)^(d-1)*(n : ℝ) := by
    conv_lhs => rw [show d=(d-1)+1 by omega,pow_succ]
  rw [correctionFactor,hwone,div_pow,source_width_power hd hε n,
    ← denominator_normalization d n hn]
  field_simp
  rw [hnpow]
  ring

theorem finite_ratio_eventual_bound {d : ℕ} (hd : 2 ≤ d) {ε : ℝ} (hε : 0<ε) :
    ∃ N : ℕ, d+1 ≤ N ∧ ∀ n : ℕ, N ≤ n →
      (sourceWidth d ε n+1)^d/((n-1).descFactorial d : ℝ) ≤
        (sourceCoefficient d ε/(n : ℝ))*(11/10) := by
  have he : ∀ᶠ n : ℕ in atTop, correctionFactor d ε n<(11/10 : ℝ) :=
    (correction_factor_tendsto hd hε).eventually (eventually_lt_nhds (by norm_num))
  obtain ⟨N,hN⟩ := eventually_atTop.mp he
  refine ⟨max N (d+1),le_max_right _ _,?_⟩
  intro n hn
  have hdim : d+1 ≤ n := (le_max_right N (d+1)).trans hn
  rw [finite_ratio_identity (by omega : 1 ≤ d) hε n hdim]
  exact mul_le_mul_of_nonneg_left (hN n ((le_max_left _ _).trans hn)).le (by
    exact div_nonneg (source_coefficient_positive d hε).le (Nat.cast_nonneg n))

noncomputable def cellLower (d : ℕ) (ε : ℝ) (p : ℕ) (m : Fin d → ℤ) (i : Fin d) : ℝ :=
  (m i : ℝ)*sourceWidth d ε p/(p : ℝ)

noncomputable def cellUpper (d : ℕ) (ε : ℝ) (p : ℕ) (m : Fin d → ℤ) (i : Fin d) : ℝ :=
  ((m i : ℝ)+1)*sourceWidth d ε p/(p : ℝ)

theorem literal_cell_endpoints (d : ℕ) (ε : ℝ) (p : ℕ) (m : Fin d → ℤ) (i : Fin d) :
    cellLower d ε p m i=(m i : ℝ)*(ε/2)^(1/(d : ℝ))*
      (p : ℝ)^(((d : ℝ)-1)/(d : ℝ))/(2*(p : ℝ)) ∧
    cellUpper d ε p m i=((m i : ℝ)+1)*(ε/2)^(1/(d : ℝ))*
      (p : ℝ)^(((d : ℝ)-1)/(d : ℝ))/(2*(p : ℝ)) := by
  unfold cellLower cellUpper sourceWidth
  constructor <;> ring

theorem cell_width {d p : ℕ} (ε : ℝ) (hp : 0<p) (m : Fin d → ℤ) (i : Fin d) :
    (p : ℝ)*(cellUpper d ε p m i-cellLower d ε p m i)=sourceWidth d ε p := by
  have hp0 : (p : ℝ) ≠ 0 := by exact_mod_cast hp.ne'
  unfold cellUpper cellLower
  field_simp
  ring

theorem cell_order {d p : ℕ} {ε : ℝ} (hε : 0<ε) (hp : 0<p) (m : Fin d → ℤ) (i : Fin d) :
    cellLower d ε p m i ≤ cellUpper d ε p m i := by
  have hp' : (0 : ℝ)<p := by exact_mod_cast hp
  have hw := source_width_positive d hε hp
  unfold cellLower cellUpper
  apply div_le_div_of_nonneg_right _ hp'.le
  nlinarith

theorem dimension_fits {d N p : ℕ} (hN : d+1 ≤ N) (hp : N ≤ p) : d ≤ p-1 := by omega

/-- Corrected author Lemma2: k=d+1>=3. The threshold is independent of
integer cell shifts, first support point and nonzero Fourier index. -/
theorem corrected_source_box_bound {d : ℕ} (hd : 2 ≤ d) {ε : ℝ} (hε : 0<ε) :
    ∃ N : ℕ, ∃ hN : d+1 ≤ N, ∀ (p : ℕ) [Fact p.Prime] (hp : N ≤ p)
      (a₀ j : ZMod p), j ≠ 0 → ∀ m : Fin d → ℤ,
      sourceProbability p d a₀ j (dimension_fits hN hp) (cellLower d ε p m) (cellUpper d ε p m) ≤
        1.1*(ε/2)/((2 : ℝ)^d*(p : ℝ)) := by
  obtain ⟨N,hN,hbound⟩ := finite_ratio_eventual_bound hd hε
  refine ⟨N,hN,?_⟩
  intro p _ hp a₀ j hj m
  have hp' : 0<p := (Fact.out : p.Prime).pos
  calc
    _ ≤ (∏ i, ((p : ℝ)*(cellUpper d ε p m i-cellLower d ε p m i)+1))/
        ((p-1).descFactorial d : ℝ) :=
      source_probability_bound a₀ j hj (dimension_fits hN hp) _ _ (cell_order hε hp' m)
    _ = (sourceWidth d ε p+1)^d/((p-1).descFactorial d : ℝ) := by
      simp only [cell_width ε hp' m,Finset.prod_const,Fintype.card_fin,Finset.card_univ]
    _ ≤ (sourceCoefficient d ε/(p : ℝ))*(11/10) := hbound p hp
    _ = _ := by unfold sourceCoefficient; ring

end HildebrandBoxAsymptoticAudit
