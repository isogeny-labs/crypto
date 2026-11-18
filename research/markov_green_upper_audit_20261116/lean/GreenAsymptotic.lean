import GreenEnvelope

namespace MarkovGreenUpperAudit
noncomputable section
open scoped BigOperators
open MarkovGreenBoundAudit
open Filter Topology Asymptotics

def remainder (e : ℕ → ℝ) (t : ℕ) : ℝ := |e t| *((t : ℝ)+1)*((t : ℝ)+2)

theorem remainder_nonnegative (e : ℕ → ℝ) (t : ℕ) : 0≤remainder e t := by
  unfold remainder
  positivity

theorem remainder_limit (e : ℕ → ℝ) (he : e =o[atTop] (fun t : ℕ => 1/(t : ℝ)^2)) :
    Tendsto (remainder e) atTop (𝓝 0) := by
  have hz : Tendsto (fun t : ℕ => e t*(t : ℝ)^2) atTop (𝓝 0) := by
    simpa [div_eq_mul_inv] using he.tendsto_div_nhds_zero
  have ha : Tendsto (fun t : ℕ => |e t| *(t : ℝ)^2) atTop (𝓝 0) := by
    have hc (t : ℕ) : |(t : ℝ)| = (t : ℝ) := abs_of_nonneg (Nat.cast_nonneg t)
    simpa only [Real.norm_eq_abs, abs_zero, abs_mul, abs_pow, hc] using hz.norm
  apply squeeze_zero' (g := fun t : ℕ => 6*(|e t| *(t : ℝ)^2))
    (Eventually.of_forall (remainder_nonnegative e))
  · filter_upwards [eventually_ge_atTop 1] with t ht
    have ht' : (1 : ℝ)≤t := by exact_mod_cast ht
    have hp : ((t : ℝ)+1)*((t : ℝ)+2) ≤ 6*(t : ℝ)^2 := by nlinarith
    have hb := mul_le_mul_of_nonneg_left hp (abs_nonneg (e t))
    unfold remainder
    nlinarith
  · simpa using ha.const_mul 6

variable {S : ℕ → Type*} [∀ t, Fintype (S t)] [∀ t, DecidableEq (S t)] [∀ t, Nonempty (S t)]

/-- Varying actual finite chains: pointwise envelope gives an explicit vanishing upper error. -/
theorem asymptotic_green_upper (P : ∀ t, Matrix (S t) (S t) ℝ)
    (hP : ∀ t, P t ∈ doublyStochastic ℝ (S t)) (e : ℕ → ℝ) (d : ℝ)
    (hd : 0≤d) (hd1 : d<1)
    (he : e =o[atTop] (fun t : ℕ => 1/(t : ℝ)^2))
    (henv : ∀ t j : ℕ, j≤t → ∀ x y : S t, (P t^j) x y ≤ e t+d^j) :
    Tendsto (remainder e) atTop (𝓝 0) ∧ ∀ t : ℕ,
      rowMaximum (P t) t ≤ 2/(1-d)^2+remainder e t ∧
      columnMaximum (P t) t ≤ 2/(1-d)^2+remainder e t := by
  refine ⟨remainder_limit e he, ?_⟩
  intro t
  have hb := finite_green_envelope (P t) (hP t) t |e t| d hd hd1 (by
    intro j hj x y
    exact (henv t j hj x y).trans (add_le_add (le_abs_self (e t)) le_rfl))
  simpa only [remainder, add_comm] using hb

theorem eventual_epsilon_green_upper (P : ∀ t, Matrix (S t) (S t) ℝ)
    (hP : ∀ t, P t ∈ doublyStochastic ℝ (S t)) (e : ℕ → ℝ) (d : ℝ)
    (hd : 0≤d) (hd1 : d<1)
    (he : e =o[atTop] (fun t : ℕ => 1/(t : ℝ)^2))
    (henv : ∀ t j : ℕ, j≤t → ∀ x y : S t, (P t^j) x y ≤ e t+d^j) :
    ∀ ε : ℝ, 0<ε → ∀ᶠ t in atTop,
      rowMaximum (P t) t ≤ 2/(1-d)^2+ε ∧
      columnMaximum (P t) t ≤ 2/(1-d)^2+ε := by
  intro ε hε
  have hh := asymptotic_green_upper P hP e d hd hd1 he henv
  filter_upwards [hh.1.eventually (gt_mem_nhds hε)] with t ht
  exact ⟨(hh.2 t).1.trans (by linarith), (hh.2 t).2.trans (by linarith)⟩

#print axioms remainder_limit
#print axioms asymptotic_green_upper
#print axioms eventual_epsilon_green_upper
end
end MarkovGreenUpperAudit
