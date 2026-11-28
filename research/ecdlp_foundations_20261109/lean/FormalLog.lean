import Mathlib

/-! Layer 4c': the formal logarithm of the multiplicative formal group.

The p-adic elliptic logarithm is the composition of two maps: the formal-group
logarithm of the curve's formal group, and the identification of that formal
group with `pZ_p`.  This file formalises the model case, the multiplicative
formal group `𝔾ₘ(X,Y) = X + Y + X*Y`:

* `derivative_logOf_mul`: `(log(1+X))' * (1+X) = 1` transported to `f`;
* `logOf_mul`: `log(f*g) = log f + log g` for `f,g` with constant term 1;
* `logOf_one`, `logOf_pow`: the logarithm kills 1 and turns powers into
  multiples;
* `logOf_gm_law`: `log(1 + (f+g+f*g)) = log(1+f) + log(1+g)`, i.e. the
  logarithm is additive for the multiplicative formal-group law.

The elliptic formal group and its logarithm are not in Mathlib; this file
supplies the analytic identity the construction needs, without assuming it. -/

open PowerSeries

set_option linter.style.haveILetI false

namespace ECDLPFormalLog

variable {A : Type*} [CommRing A] [Algebra ℚ A]

/-- `logOf f = log(1+X)` substituted at `f - 1`, i.e. the formal `log f`. -/
theorem derivative_logOf_mul {f : A⟦X⟧} (hf : constantCoeff f = 1) :
    d⁄dX (logOf f) * f = d⁄dX f := by
  have hsub : HasSubst (f - 1) := HasSubst.of_constantCoeff_zero' (by simp [hf])
  rw [logOf_eq, derivative_subst hsub]
  have hchain : subst (f - 1) (d⁄dX (log A)) * (1 + (f - 1)) = 1 := by
    have h := congrArg (subst (f - 1)) (derivative_log_mul_one_add_X (A := A))
    rw [subst_mul hsub, subst_add hsub, subst_X hsub] at h
    have h1 : subst (f - 1) (1 : A⟦X⟧) = 1 := by
      simpa using (subst_C (a := (f - 1 : A⟦X⟧)) (r := (1 : A)))
    rw [h1] at h
    have hf' : (1 : A⟦X⟧) + (f - 1) = f := by ring
    simpa [hf'] using h
  have hA : subst (f - 1) (d⁄dX (log A)) * f = 1 := by
    simpa using hchain
  have hd : d⁄dX (f - 1) = d⁄dX f := by simp
  rw [hd]
  calc (subst (f - 1) (d⁄dX (log A)) * d⁄dX f) * f
      = (subst (f - 1) (d⁄dX (log A)) * f) * d⁄dX f := by ring
    _ = 1 * d⁄dX f := by rw [hA]
    _ = d⁄dX f := one_mul _

/-- **Multiplicativity of the formal logarithm.**  For `f,g` with constant term
`1`, `log(f*g) = log f + log g`. -/
theorem logOf_mul {f g : A⟦X⟧} (hf : constantCoeff f = 1) (hg : constantCoeff g = 1) :
    logOf (f * g) = logOf f + logOf g := by
  haveI : IsAddTorsionFree A := IsAddTorsionFree.of_module_rat A
  have hfg : constantCoeff (f * g) = 1 := by simp [hf, hg]
  apply derivative.ext
  · rw [map_add]
    have h1 := derivative_logOf_mul (A := A) hfg
    have h2 := derivative_logOf_mul (A := A) hf
    have h3 := derivative_logOf_mul (A := A) hg
    have hunit : IsUnit (f * g) := (isUnit_iff_constantCoeff).mpr (by simp [hfg])
    obtain ⟨u, hu⟩ := hunit
    have hEq : (d⁄dX (logOf (f * g))) * (f * g) =
        (d⁄dX (logOf f) + d⁄dX (logOf g)) * (f * g) := by
      calc (d⁄dX (logOf (f * g))) * (f * g)
          = d⁄dX (f * g) := h1
        _ = d⁄dX f * g + f * d⁄dX g := by
            rw [Derivation.leibniz]
            ring
        _ = (d⁄dX (logOf f) * f) * g + f * (d⁄dX (logOf g) * g) := by
            rw [← h2, ← h3]
        _ = (d⁄dX (logOf f) + d⁄dX (logOf g)) * (f * g) := by ring
    have := congrArg (fun t => t * (↑u⁻¹ : A⟦X⟧)) hEq
    rw [← hu] at this ⊢
    simpa [mul_assoc] using this
  · simp [constantCoeff_logOf hf, constantCoeff_logOf hg, constantCoeff_logOf hfg]

/-- The formal logarithm of `1` is zero. -/
theorem logOf_one : logOf (1 : A⟦X⟧) = 0 := by
  have h1 : constantCoeff (1 : A⟦X⟧) = 1 := by simp
  have h := logOf_mul (f := 1) (g := 1) h1 h1
  have h' : logOf (1 : A⟦X⟧) = logOf 1 + logOf 1 := by simpa using h
  exact (add_left_cancel (by rw [add_zero]; exact h')).symm

/-- The formal logarithm turns powers into scalar multiples. -/
theorem logOf_pow (f : A⟦X⟧) (hf : constantCoeff f = 1) (n : ℕ) :
    logOf (f ^ n) = n • logOf f := by
  induction n with
  | zero => simpa using logOf_one
  | succ n ih =>
      rw [pow_succ, logOf_mul (by simp [hf]) hf, ih, succ_nsmul]

/-- **Additivity for the multiplicative formal group.**  With
`F(f,g) = f + g + f*g` (so `1 + F(f,g) = (1+f)(1+g)`), the formal logarithm
satisfies `log(1+F(f,g)) = log(1+f) + log(1+g)`. -/
theorem logOf_gm_law {f g : A⟦X⟧} (hf : constantCoeff f = 0) (hg : constantCoeff g = 0) :
    logOf (1 + (f + g + f * g)) = logOf (1 + f) + logOf (1 + g) := by
  have h1 : constantCoeff (1 + f) = 1 := by simp [hf]
  have h2 : constantCoeff (1 + g) = 1 := by simp [hg]
  have hmul : (1 + f) * (1 + g) = 1 + (f + g + f * g) := by ring
  rw [← hmul, logOf_mul h1 h2]

end ECDLPFormalLog
