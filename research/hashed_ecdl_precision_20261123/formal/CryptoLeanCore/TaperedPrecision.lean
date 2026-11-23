import Std

/-!
# Integer precision budget for a tapered contraction call

This is the scalar part of Appendix D.11 of the October25 contraction proof
(openai/math42907c9e, `build/quantitative-contraction.tex`). It models the
expression used by the native hashed-ECDLP `child_precision` function. A
consumer must separately bind a call history, its inherited precision and
source dimensions. No graph, counter, admission or running-time theorem is
asserted by this module.
-/

namespace CryptoLeanCore.TaperedPrecision

/-- The source precision expression. On an active native call `i < u ≤ a`,
`2^(a-i)` equals its packet length shifted by the node valuation. Division
here is natural-number floor division; positive `u` makes `(3*M+u-1)/u`
the source ceiling. This definition itself admits arbitrary natural inputs. -/
def childPrecision (p M u a i : Nat) : Nat :=
  p + 8 * ((3 * M + u - 1) / u) * 2 ^ (a - i) + 8 * M * (a + 1)

/-- The one defining expansion used by the resource laws below. -/
theorem childPrecision_formula (p M u a i : Nat) :
    childPrecision p M u a i =
      p + 8 * ((3 * M + u - 1) / u) * 2 ^ (a - i) + 8 * M * (a + 1) := rfl

/-- A single increment is bounded by `40*M*2^a` when the active scale is
positive and at most the edge count. This is a conservative source-step
bound, not a bound on an entire inherited path. -/
theorem increment_le (M u a i : Nat) (hu : 0 < u) (hum : u ≤ M) :
    childPrecision 0 M u a i ≤ 40 * M * 2 ^ a := by
  have hh : (3 * M + u - 1) / u ≤ 4 * M :=
    Nat.le_trans (Nat.le_mul_of_pos_right _ hu)
      (Nat.le_trans (Nat.div_mul_le_self _ _) (by omega))
  have hd : 2 ^ (a - i) ≤ 2 ^ a :=
    Nat.pow_le_pow_right (by decide) (Nat.sub_le _ _)
  have ha : a + 1 ≤ 2 ^ a := Nat.succ_le_of_lt Nat.lt_two_pow_self
  rw [childPrecision_formula, Nat.zero_add]
  calc
    _ ≤ 8 * (4 * M) * 2 ^ a + 8 * M * 2 ^ a :=
      Nat.add_le_add (Nat.mul_le_mul (Nat.mul_le_mul_left 8 hh) hd)
        (Nat.mul_le_mul_left (8 * M) ha)
    _ = 40 * M * 2 ^ a := by
      simp only [Nat.mul_assoc]
      omega

/-- Finite scalar refinement of D.11: an explicit inherited budget and two
dimension-only absorption conditions imply the sharp `34*M*D/u` envelope,
stated without division as `childPrecision* u ≤ 34*M*D`. The inherited
coefficient `c` is a parameter; this theorem does not establish its value
for a recursive path. No asymptotic or Rust-refinement claim is implicit. -/
theorem childPrecision_mul_le
    (p M u A i s q c : Nat)
    (hum : u ≤ M) (hi : i < u)
    (hroot : s ≤ M * q)
    (hinherited : p ≤ c * s * 2 ^ A)
    (habsorb : c * q * u ≤ 2 ^ (u + 1))
    (hevaluation : 8 * u * (A + 2 * u + 1) ≤ 2 ^ (A + (u + 1))) :
    childPrecision p M u (A + 2 * u) i * u ≤
      34 * M * 2 ^ (A + 2 * u - i) := by
  let D := 2 ^ (A + 2 * u - i)
  have hD : 2 ^ (A + (u + 1)) ≤ D :=
    Nat.pow_le_pow_right (by decide) (by omega)
  have hh : ((3 * M + u - 1) / u) * u ≤ 4 * M :=
    Nat.le_trans (Nat.div_mul_le_self _ _) (by omega)
  have hp : p * u ≤ M * D := by
    calc
      p * u ≤ (c * s * 2 ^ A) * u := Nat.mul_le_mul_right u hinherited
      _ = (2 ^ A * (c * u)) * s := by ac_rfl
      _ ≤ (2 ^ A * (c * u)) * (M * q) := Nat.mul_le_mul_left _ hroot
      _ = (M * 2 ^ A) * (c * q * u) := by ac_rfl
      _ ≤ (M * 2 ^ A) * 2 ^ (u + 1) := Nat.mul_le_mul_left _ habsorb
      _ = M * 2 ^ (A + (u + 1)) := by
        simp only [Nat.pow_add, Nat.mul_assoc]
      _ ≤ M * D := Nat.mul_le_mul_left M hD
  have hn : 8 * ((3 * M + u - 1) / u) * D * u ≤ 32 * M * D := by
    calc
      _ = (8 * D) * (((3 * M + u - 1) / u) * u) := by ac_rfl
      _ ≤ (8 * D) * (4 * M) := Nat.mul_le_mul_left _ hh
      _ = (8 * 4) * M * D := by ac_rfl
      _ = 32 * M * D := rfl
  have he : 8 * M * (A + 2 * u + 1) * u ≤ M * D := by
    calc
      _ = M * (8 * u * (A + 2 * u + 1)) := by ac_rfl
      _ ≤ M * 2 ^ (A + (u + 1)) := Nat.mul_le_mul_left _ hevaluation
      _ ≤ M * D := Nat.mul_le_mul_left M hD
  rw [childPrecision_formula]
  change (p + 8 * ((3 * M + u - 1) / u) * D + 8 * M * (A + 2 * u + 1)) * u ≤ _
  calc
    _ = p * u + 8 * ((3 * M + u - 1) / u) * D * u +
        8 * M * (A + 2 * u + 1) * u := by
      simp only [Nat.add_mul]
    _ ≤ M * D + 32 * M * D + M * D := Nat.add_le_add (Nat.add_le_add hp hn) he
    _ = 34 * M * D := by
      simp only [Nat.mul_assoc]
      omega

end CryptoLeanCore.TaperedPrecision
