import CryptoLeanCore.TaperedPrecision

namespace NativeLeafGuard

/-- Direct application of Std's existing positive-divisor equivalence. -/
theorem active_size_lt (s M q : Nat) (hq : 0 < q) (hactive : s / q < M) :
    s < M * q :=
  (Nat.div_lt_iff_lt_mul hq).mp hactive

/-- The source maximum makes the divisor positive; failing its early size
return supplies the root comparison used by the existing precision theorem. -/
theorem source_guard_comparison (s M k : Nat)
    (hguard : ¬ M ≤ s / max k 1) : s ≤ M * max k 1 := by
  have hq : 0 < max k 1 := Nat.lt_of_lt_of_le Nat.zero_lt_one (Nat.le_max_right k 1)
  exact Nat.le_of_lt (active_size_lt s M (max k 1) hq (Nat.lt_of_not_ge hguard))

/-- Only hroot is discharged here. Every inherited/dimension premise of the
existing conditional34 law remains explicit and unchanged. -/
theorem precision_from_source_guard (p M u A i s k c : Nat)
    (hum : u ≤ M) (hi : i < u)
    (hguard : ¬ M ≤ s / max k 1)
    (hinherited : p ≤ c * s * 2 ^ A)
    (habsorb : c * max k 1 * u ≤ 2 ^ (u + 1))
    (hevaluation : 8 * u * (A + 2 * u + 1) ≤ 2 ^ (A + (u + 1))) :
    CryptoLeanCore.TaperedPrecision.childPrecision p M u (A + 2 * u) i * u ≤
      34 * M * 2 ^ (A + 2 * u - i) :=
  CryptoLeanCore.TaperedPrecision.childPrecision_mul_le p M u A i s (max k 1) c
    hum hi (source_guard_comparison s M k hguard) hinherited habsorb hevaluation

/-- Removing activity can falsify even the weaker non-strict comparison. -/
theorem inactive_counterexample : ¬ 3 / 2 < 1 ∧ ¬ 3 ≤ 1 * 2 := by decide

/-- The positive-divisor premise cannot be silently omitted. -/
theorem zero_divisor_control : 1 / 0 < 1 ∧ ¬ 1 < 1 * 0 := by decide

/-- The weaker conclusion may hold at an inactive divisible boundary. -/
theorem inactive_boundary : 4 / 2 = 2 ∧ 4 ≤ 2 * 2 ∧ ¬ 4 < 2 * 2 := by decide

/-- Nonempty arithmetic application; this does not assert source Setup
admission for these deliberately small scalar parameters. -/
theorem nonempty_scalar_application :
    CryptoLeanCore.TaperedPrecision.childPrecision 1 16 12 24 0 * 12 ≤
      34 * 16 * 2 ^ 24 := by
  exact precision_from_source_guard 1 16 12 0 0 24 2 1
    (by decide) (by decide) (by decide) (by decide) (by decide) (by decide)

#print axioms active_size_lt
#print axioms source_guard_comparison
#print axioms precision_from_source_guard
#print axioms inactive_counterexample
#print axioms zero_divisor_control
#print axioms inactive_boundary
#print axioms nonempty_scalar_application
#print axioms CryptoLeanCore.TaperedPrecision.childPrecision_mul_le
end NativeLeafGuard
