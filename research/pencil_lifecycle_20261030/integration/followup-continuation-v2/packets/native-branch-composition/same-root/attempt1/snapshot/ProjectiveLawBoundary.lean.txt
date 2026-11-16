import Mathlib.Algebra.Field.Basic
import Mathlib.Tactic.Ring

/-!
# Mechanized boundary for the weighted genus-2 projective law

This file proves algebraic obligations used by the runtime case split and two
literal formula fragments.  It deliberately does not define a genus-2 curve,
Mumford reduction, divisors, or `Pic⁰`.  Consequently it does not claim that
the Rust result represents the sum of two divisor classes.  That geometric
bridge remains an explicit obligation in `README.md`.
-/

set_option autoImplicit false
set_option maxRecDepth 10000

namespace ProjectiveLawBoundary

section BranchSelection

variable {K : Type*} [Field K]

/-- On an odd-characteristic curve, equal squares give equal or opposite
ordinates.  This is the algebraic core excluding the runtime labels
`coincident_y_mismatch` and `shared_y_mismatch`. -/
theorem equal_or_opposite_of_sq_eq_sq
    (y v : K) (h : y ^ 2 = v ^ 2) : y = v ∨ y = -v := by
  have hprod : (y - v) * (y + v) = 0 := by
    calc
      (y - v) * (y + v) = y ^ 2 - v ^ 2 := by ring
      _ = 0 := sub_eq_zero.mpr h
  rcases mul_eq_zero.mp hprod with hzero | hzero
  · exact Or.inl (sub_eq_zero.mp hzero)
  · exact Or.inr (eq_neg_of_add_eq_zero_left hzero)

/-- With identical linear terms, two valid linear Mumford ordinates for the
same monic quadratic are equal or opposite.  The hypotheses `hx` and `hc`
are exactly the two coefficients of `v₂²-v₁² mod u`; `char_ne_two` records
the runtime's odd-characteristic precondition. -/
theorem same_u_zero_slope_delta_is_inverse
    (s t₁ t₂ : K) (hodd : (2 : K) ≠ 0) (hneq : t₂ ≠ t₁)
    (hx : (t₂ - t₁) * (2 * s) = 0)
    (hc : (t₂ - t₁) * (t₂ + t₁) = 0) :
    s = 0 ∧ t₂ = -t₁ := by
  have hdelta : t₂ - t₁ ≠ 0 := sub_ne_zero.mpr hneq
  have h2s : 2 * s = 0 := (mul_eq_zero.mp hx).resolve_left hdelta
  have hsum : t₂ + t₁ = 0 := (mul_eq_zero.mp hc).resolve_left hdelta
  constructor
  · exact (mul_eq_zero.mp h2s).resolve_left hodd
  · exact eq_neg_of_add_eq_zero_left hsum

/-- If the weighted quadratic coefficients differ and the runtime value
`B = dq*(q₂*r₁-q₁*r₂)-dr²` vanishes, then `dq` cannot also vanish.  This
excludes `shared_zero_q_delta` for distinct valid `u` inputs. -/
theorem shared_zero_q_delta_unreachable
    (dq dr q₁ q₂ r₁ r₂ : K)
    (hdistinct : dq ≠ 0 ∨ dr ≠ 0)
    (hB : dq * (q₂ * r₁ - q₁ * r₂) - dr ^ 2 = 0) :
    dq ≠ 0 := by
  intro hdq
  subst dq
  simp only [zero_mul, zero_sub] at hB
  have hdr : dr = 0 := sq_eq_zero_iff.mp (neg_eq_zero.mp hB)
  exact hdistinct.elim (fun h => h rfl) (fun h => h hdr)

end BranchSelection

section LiteralFormulaFragments

variable {R : Type*} [CommRing R]

/-- The cleared output-ordinate identity in the generic 1+2 branch.  The
definitions are a direct transcription of `add12_weighted_generic` and the
`projective-symbolic-1plus2/v1` certificate.  This theorem proves the literal
polynomial equality; it does not supply the divisor-of-`y-L` argument. -/
theorem generic12_output_v_clearing
    (X q r s t K A B D Q R₀ : R)
    (hB : B = s * K + A * q)
    (hD : D = t * K + A * r) :
    let S := A * Q - B * K ^ 2
    let T := A * R₀ * K ^ 2 - D * K ^ 4
    let uX := X ^ 2 + q * X + r
    let vX := s * X + t
    let N := K * vX + A * uX
    let U := K ^ 2 * X ^ 2 + Q * X + R₀
    K ^ 2 * (K ^ 2 * N - A * U) + K ^ 2 * S * X + T = 0 := by
  dsimp
  rw [hB, hD]
  ring

/-- The generic 2+2 interpolant differs from `B*v₁` by the explicit
multiple `-(C*X+A)*u₁` of the first input quadratic.  Instantiating `X` as
the polynomial indeterminate gives the literal congruence modulo `u₁`
checked by `projective-symbolic-22-common/v1`. -/
theorem generic22_interpolates_first
    (X q₁ r₁ s₁ t₁ B C A : R) :
    let u₁ := X ^ 2 + q₁ * X + r₁
    let v₁ := s₁ * X + t₁
    let N := B * v₁ - (C * X + A) * u₁
    N - B * v₁ = -(C * X + A) * u₁ := by
  dsimp
  ring

/-- The same cubic interpolant agrees with `B*v₂` on the second support.
The definitions of `A`, `B`, and `C` match the runtime `aa`, `bb`, and `cc`
after dehomogenization. -/
theorem generic22_interpolates_second
    (X q₁ r₁ s₁ t₁ q₂ r₂ s₂ t₂ : R) :
    let dq := q₁ - q₂
    let dr := r₁ - r₂
    let ds := s₁ - s₂
    let dt := t₁ - t₂
    let B := dq * (q₂ * r₁ - q₁ * r₂) - dr ^ 2
    let A := dq * (dt * q₂ - ds * r₂) - dr * dt
    let C := dq * dt - dr * ds
    let u₁ := X ^ 2 + q₁ * X + r₁
    let v₁ := s₁ * X + t₁
    let v₂ := s₂ * X + t₂
    B * v₁ - (C * X + A) * u₁ - B * v₂ =
      (-C * X - C * dq - A) * (X ^ 2 + q₂ * X + r₂) := by
  dsimp
  ring

end LiteralFormulaFragments

/-! ## Runtime/certificate identifier coverage

This finite mapping mirrors the reachable labels emitted by
`add12_weighted_complete` and `add22_weighted`.  It is intentionally a manual
source binding, guarded by source hashes in `source-bindings.json`; it is not a theorem
that the Rust and Lean definitions are definitionally equal.
-/

inductive RuntimeBranch where
  | add11Inverse | add11Doubling | add11Distinct
  | add12Common | add12Cancellation | add12Tripling
  | add12CoincidentDoublePlusOther
  | add22SameUInverse | add22SameUMixed
  | add22SharedCancellation | add22SharedPositive
  | add22CZeroDegreeOne | add22CommonDegreeTwo
  | double22Identity | double22PointDouble
  | double22CZeroDegreeOne | double22CommonDegreeTwo
  deriving DecidableEq, Repr

inductive Certificate where
  | definitionalInverse
  | generic12 | exceptional12 | tripling
  | mixed22 | shared22 | drop22 | common22 | double22
  deriving DecidableEq, Repr

def certificateFor : RuntimeBranch → Certificate
  | .add11Inverse => .definitionalInverse
  | .add11Doubling => .exceptional12
  | .add11Distinct => .shared22
  | .add12Common => .generic12
  | .add12Cancellation => .exceptional12
  | .add12Tripling => .tripling
  | .add12CoincidentDoublePlusOther => .exceptional12
  | .add22SameUInverse => .definitionalInverse
  | .add22SameUMixed => .mixed22
  | .add22SharedCancellation => .shared22
  | .add22SharedPositive => .shared22
  | .add22CZeroDegreeOne => .drop22
  | .add22CommonDegreeTwo => .common22
  | .double22Identity => .double22
  | .double22PointDouble => .double22
  | .double22CZeroDegreeOne => .double22
  | .double22CommonDegreeTwo => .double22

theorem runtime_certificate_mapping_total (b : RuntimeBranch) :
    ∃ c : Certificate, certificateFor b = c := ⟨certificateFor b, rfl⟩

/-! ## Explicit geometric bridge

The proposition below names the missing theorem rather than asserting it as
an axiom.  The implication is useful only after a caller provides the actual
curve/divisor-class result. -/

structure DivisorClassBridge (FormulaValid ClassSum : Prop) : Prop where
  formula_implies_class_sum : FormulaValid → ClassSum

theorem formula_to_divisor_class
    {FormulaValid ClassSum : Prop}
    (bridge : DivisorClassBridge FormulaValid ClassSum)
    (hformula : FormulaValid) : ClassSum :=
  bridge.formula_implies_class_sum hformula

end ProjectiveLawBoundary
