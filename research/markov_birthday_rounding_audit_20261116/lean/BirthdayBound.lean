import BirthdayLaw

namespace MarkovBirthdayRoundingAudit
noncomputable section
open StaticHashCollisionAudit
variable (p : ℕ) [NeZero p]

/-- A valid finite birthday bound for the actual complete uniform chain. -/
theorem uniform_survival_exp (x : ZMod p) (n : ℕ) :
    iidSurvival (uniformWeight p) (resetUpdate p) x n ≤
      Real.exp (-((n : ℝ)*(n+1)/(2*(p : ℝ)))) := by
  induction n with
  | zero =>
    rw [uniform_birthday_survival]
    norm_num
  | succ n ih =>
    by_cases hn : p≤n+1
    · rw [birthday_survival_zero p x (n+1) hn]
      exact (Real.exp_pos _).le
    have hn' : n+1≤p := by omega
    have hnum : (n+1 : ℝ)≤p := by exact_mod_cast hn'
    have hf0 : 0≤1-(n+1 : ℝ)/p := by
      apply sub_nonneg.mpr
      exact (div_le_one (modulus_positive p)).mpr hnum
    have hf : 1-(n+1 : ℝ)/p≤Real.exp (-((n+1 : ℝ)/p)) := by
      have he:=Real.add_one_le_exp (-((n+1 : ℝ)/p))
      linarith
    rw [birthday_survival_recurrence p x n hn']
    calc
      _≤(1-(n+1 : ℝ)/p)*Real.exp (-((n : ℝ)*(n+1)/(2*(p : ℝ)))) :=
        mul_le_mul_of_nonneg_left ih hf0
      _≤Real.exp (-((n+1 : ℝ)/p))*Real.exp (-((n : ℝ)*(n+1)/(2*(p : ℝ)))) :=
        mul_le_mul_of_nonneg_right hf (Real.exp_pos _).le
      _=_ := by
        rw [←Real.exp_add]
        congr 1
        push_cast
        field_simp
        ring

/-- The time is an integer and its sufficient quadratic condition is explicit. -/
theorem uniform_integer_time_bound (x : ZMod p) (n : ℕ) (c : ℝ)
    (hn : 2*(p : ℝ)*c≤(n : ℝ)*(n+1)) :
    1-Real.exp (-c)≤collisionCDF p x n := by
  have hd : 0<2*(p : ℝ) := mul_pos (by norm_num) (modulus_positive p)
  have hc : c≤(n : ℝ)*(n+1)/(2*(p : ℝ)) := by
    apply (le_div_iff₀ hd).mpr
    nlinarith [hn]
  have he : Real.exp (-((n : ℝ)*(n+1)/(2*(p : ℝ))))≤Real.exp (-c) :=
    Real.exp_le_exp.mpr (by linarith)
  have hs:=uniform_survival_exp p x n
  rw [collision_cdf_complement]
  linarith

theorem uniform_ceiling_time_bound (x : ZMod p) (c : ℝ) (hc : 0≤c) :
    1-Real.exp (-c)≤collisionCDF p x (Nat.ceil (Real.sqrt (2*(p : ℝ)*c))) := by
  apply uniform_integer_time_bound
  let n:=Nat.ceil (Real.sqrt (2*(p : ℝ)*c))
  have hr0 : 0≤2*(p : ℝ)*c := by positivity
  have hr:=Real.sq_sqrt hr0
  have hs:=Nat.le_ceil (Real.sqrt (2*(p : ℝ)*c))
  have hsq : 2*(p : ℝ)*c≤(n : ℝ)^2 := by
    have hm:=mul_nonneg (sub_nonneg.mpr hs)
      (add_nonneg (Nat.cast_nonneg n : (0 : ℝ)≤n) (Real.sqrt_nonneg (2*(p : ℝ)*c)))
    dsimp only [n] at *
    nlinarith
  change 2*(p : ℝ)*c≤(n : ℝ)*(n+1)
  nlinarith [(Nat.cast_nonneg n : (0 : ℝ)≤n)]

/-- The same actual p47/c1/44 instance has a valid two-step finite control. -/
theorem fortyseven_two_step_control :
    1-Real.exp (-(1/44))≤collisionCDF 47 0 2 :=
  uniform_integer_time_bound 47 0 2 (1/44) (by norm_num)

theorem zero_confidence_control (x : ZMod p) :
    collisionCDF p x 0=1-Real.exp (-(0 : ℝ)) := by
  rw [collision_cdf_zero]
  norm_num

theorem singleton_one_step_control (x : ZMod 1) : collisionCDF 1 x 1=1 := by
  rw [collision_cdf_one]
  norm_num

#print axioms uniform_survival_exp
#print axioms uniform_integer_time_bound
#print axioms uniform_ceiling_time_bound
#print axioms fortyseven_two_step_control
end
end MarkovBirthdayRoundingAudit
