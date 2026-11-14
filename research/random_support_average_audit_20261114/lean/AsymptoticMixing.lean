module

public import FiniteAverage
public import DenominatorLimits

@[expose] public section

namespace RandomSupportAverageAudit

open Real Filter SupportGaussianAudit InjectiveBoxAudit HildebrandCellAudit
open WalkCharacterAudit WeightedCycleCouplingAudit CycleCouplingAudit FiniteFourierUpperBoundAudit
open HildebrandBoxAsymptoticAudit
open scoped Topology

theorem eventual_denominator_half (d : ℕ) :
    ∃ N : ℕ, d+1 ≤ N ∧ ∀ n : ℕ, N ≤ n →
      (n : ℝ)^d/2 ≤ ((n-1).descFactorial d : ℝ) := by
  have he : ∀ᶠ n : ℕ in atTop, (1/2 : ℝ)<((n-1).descFactorial d : ℝ)/(n : ℝ)^d :=
    (actual_denominator_tendsto d).eventually (eventually_gt_nhds (by norm_num))
  obtain ⟨N,hN⟩ := eventually_atTop.mp he
  refine ⟨max N (d+1),le_max_right _ _,?_⟩
  intro n hn
  have hn0 : (0 : ℝ)<n := by exact_mod_cast (show 0<n by omega)
  have hb := hN n (le_trans (le_max_left _ _) hn)
  have hm := (lt_div_iff₀ (pow_pos hn0 d)).mp hb
  linarith

theorem gaussian_scale_square {ω : ℝ} (hω : 0<ω) {p t : ℕ} (hp : 0<p) (ht : 0<t) :
    (sqrt (Real.pi/((8*ω^2*(t : ℝ))/(p : ℝ)^2)))^2 =
      (Real.pi/(8*ω^2))*(p : ℝ)^2/(t : ℝ) := by
  have hp' : (0 : ℝ)<p := by exact_mod_cast hp
  have ht' : (0 : ℝ)<t := by exact_mod_cast ht
  rw [Real.sq_sqrt (by positivity)]
  field_simp

theorem finite_fourth_power_bound {p d : ℕ} [Fact p.Prime] (h : d ≤ p-1)
    (hD : (p : ℝ)^d/2 ≤ ((p-1).descFactorial d : ℝ))
    (q : PMF (Fin (d+1))) {ω : ℝ} (hω : 0<ω) (hw : ∀ i, ω ≤ weight q i)
    (t : ℕ) (ht : 0<t) (x : ZMod p) :
    (expectedDistance p d h q t x)^4 ≤
      (Real.pi/(8*ω^2))^d*(p : ℝ)^2/(4*(t : ℝ)^d) := by
  let G : ℝ := sqrt (Real.pi/((8*ω^2*(t : ℝ))/(p : ℝ)^2))
  let K : ℝ := Real.pi/(8*ω^2)
  let V : ℝ := (p : ℝ)*G^d/(2*(p : ℝ)^d)
  have hp' : (0 : ℝ)<p := by exact_mod_cast (Fact.out : p.Prime).pos
  have ht' : (0 : ℝ)<t := by exact_mod_cast ht
  have hP := pow_pos hp' d
  have hG : 0 ≤ G := Real.sqrt_nonneg _
  have hG2 : G^2=K*(p : ℝ)^2/(t : ℝ) :=
    gaussian_scale_square hω (Fact.out : p.Prime).pos ht
  have hv : V^2=K^d*(p : ℝ)^2/(4*(t : ℝ)^d) := by
    calc
      _ = (p : ℝ)^2*(G^2)^d/(4*((p : ℝ)^2)^d) := by
        unfold V
        rw [div_pow,mul_pow]
        ring
      _ = (p : ℝ)^2*(K*(p : ℝ)^2/(t : ℝ))^d/(4*((p : ℝ)^2)^d) := by rw [hG2]
      _ = _ := by
        rw [div_pow,mul_pow]
        field_simp
  have hpn : ((p-1 : ℕ) : ℝ) ≤ (p : ℝ) := by exact_mod_cast Nat.sub_le p 1
  have hb := finite_expected_distance_bound h q hω hw t ht x
  have hn : ((p-1 : ℕ) : ℝ)/4*G^d ≤ (p : ℝ)/4*G^d :=
    mul_le_mul_of_nonneg_right (div_le_div_of_nonneg_right hpn (by norm_num)) (pow_nonneg hG d)
  have hBV := div_le_div₀ (by positivity : 0 ≤ (p : ℝ)/4*G^d) hn (by positivity : 0<(p : ℝ)^d/2) hD
  have hm : (expectedDistance p d h q t x)^2 ≤ V := by
    calc
      _ ≤ ((p-1 : ℕ) : ℝ)/4*G^d/((p-1).descFactorial d : ℝ) := hb
      _ ≤ (p : ℝ)/4*G^d/((p : ℝ)^d/2) := hBV
      _ = _ := by unfold V; ring
  have hs := pow_le_pow_left₀ (sq_nonneg (expectedDistance p d h q t x)) hm 2
  have hm4 : (expectedDistance p d h q t x)^4 ≤ V^2 := by
    simpa only [← pow_mul] using hs
  exact hm4.trans_eq hv

noncomputable def sourceTime (d : ℕ) (γ : ℝ) (p : ℕ) : ℕ := ⌊γ*(p : ℝ)^((2 : ℝ)/(d : ℝ))⌋₊

theorem source_time_power {d p : ℕ} (hd : 1 ≤ d) (hp : 1 ≤ p) {R : ℝ} (hR : 1 ≤ R) :
    0<sourceTime d (2*R) p ∧ R*(p : ℝ)^2 ≤ (sourceTime d (2*R) p : ℝ)^d := by
  have hp' : (1 : ℝ) ≤ p := by exact_mod_cast hp
  have hd' : (0 : ℝ)<d := by exact_mod_cast (show 0<d by omega)
  have hs : 1 ≤ (p : ℝ)^((2 : ℝ)/(d : ℝ)) := Real.one_le_rpow hp' (by positivity)
  have hU : 1 ≤ R*(p : ℝ)^((2 : ℝ)/(d : ℝ)) := by
    simpa only [one_mul] using mul_le_mul hR hs (by norm_num : (0 : ℝ) ≤ 1) (by linarith : 0 ≤ R)
  have hf := Nat.lt_floor_add_one (2*R*(p : ℝ)^((2 : ℝ)/(d : ℝ)))
  have hl : R*(p : ℝ)^((2 : ℝ)/(d : ℝ)) ≤ (sourceTime d (2*R) p : ℝ) := by
    unfold sourceTime
    linarith
  have ht : 0<sourceTime d (2*R) p := by
    have hh : (0 : ℝ)<sourceTime d (2*R) p := by linarith
    exact_mod_cast hh
  have hrpow : R ≤ R^d := by
    obtain ⟨e,rfl⟩ := Nat.exists_eq_succ_of_ne_zero (show d ≠ 0 by omega)
    rw [pow_succ]
    have hh : 1 ≤ R^e := one_le_pow₀ hR
    nlinarith
  have hscale : ((p : ℝ)^((2 : ℝ)/(d : ℝ)))^d=(p : ℝ)^2 := by
    rw [← Real.rpow_natCast,← Real.rpow_mul (by positivity)]
    have he : (2/(d : ℝ))*(d : ℝ)=2 := by field_simp
    rw [he,Real.rpow_two]
  refine ⟨ht,?_⟩
  calc
    _ ≤ R^d*(p : ℝ)^2 := mul_le_mul_of_nonneg_right hrpow (sq_nonneg _)
    _ = (R*(p : ℝ)^((2 : ℝ)/(d : ℝ)))^d := by rw [mul_pow,hscale]
    _ ≤ _ := pow_le_pow_left₀ (by positivity) hl d

theorem qualified_expected_support_mixing {d : ℕ} (hd : 1 ≤ d)
    (q : PMF (Fin (d+1))) (hq : ∀ i, 0<weight q i) {ε : ℝ} (hε : 0<ε) :
    ∃ γ : ℝ, 0<γ ∧ ∃ N : ℕ, d+1 ≤ N ∧
      ∀ (p : ℕ) [Fact p.Prime] (h : d ≤ p-1), N ≤ p → ∀ x : ZMod p,
      expectedDistance p d h q (sourceTime d γ p) x<ε := by
  obtain ⟨ω,hω,hw⟩ := positive_weight_lower_bound q hq
  obtain ⟨N,hN,hD⟩ := eventual_denominator_half d
  let A : ℝ := (Real.pi/(8*ω^2))^d
  let R : ℝ := A/ε^4+1
  have hA : 0<A := by unfold A; positivity
  have hR : 1 ≤ R := by
    have hh : 0 ≤ A/ε^4 := by positivity
    change 1 ≤ A/ε^4+1
    linarith
  have hRpos : 0<R := by linarith
  refine ⟨2*R,by positivity,N,hN,?_⟩
  intro p _ h hp x
  have hp' : (0 : ℝ)<p := by exact_mod_cast (Fact.out : p.Prime).pos
  obtain ⟨ht,hT⟩ := source_time_power hd (Fact.out : p.Prime).one_le hR
  have hb := finite_fourth_power_bound h (hD p hp) q hω hw (sourceTime d (2*R) p) ht x
  have ht' : (0 : ℝ)<sourceTime d (2*R) p := by exact_mod_cast ht
  have hpow : 0<(sourceTime d (2*R) p : ℝ)^d := pow_pos ht' d
  have hbound : A*(p : ℝ)^2/(4*(sourceTime d (2*R) p : ℝ)^d) ≤ A/(4*R) := by
    apply (div_le_div_iff₀ (by positivity : 0<4*(sourceTime d (2*R) p : ℝ)^d)
      (by positivity : 0<4*R)).mpr
    have hh := mul_le_mul_of_nonneg_left hT hA.le
    nlinarith
  have hsmall : A/(4*R)<ε^4 := by
    apply (div_lt_iff₀ (by positivity : 0<4*R)).mpr
    have he : ε^4*R=A+ε^4 := by unfold R; field_simp
    nlinarith [pow_pos hε 4]
  have hfourth : (expectedDistance p d h q (sourceTime d (2*R) p) x)^4<ε^4 :=
    (hb.trans hbound).trans_lt hsmall
  by_contra hn
  have hn' : ε ≤ expectedDistance p d h q (sourceTime d (2*R) p) x := le_of_not_gt hn
  exact (not_le_of_gt hfourth) (pow_le_pow_left₀ hε.le hn' 4)

end RandomSupportAverageAudit
