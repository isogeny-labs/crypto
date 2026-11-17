import Mathlib.Algebra.Group.Hom.Basic
import Mathlib.Tactic

/-!
# H113: binary/trinary section-curl structural obligations

The declarations below are generic algebraic and reconstruction statements.
They do not assert that a measured winding predicate is exact.
-/

namespace H113BinaryTrinaryWinding

section CarryDiamond

variable {Q A : Type*}
variable [AddCommGroup A]

/-- The section carry for multiplication by `n`. The section is deliberately
only a set-theoretic function. -/
def carry (n : ℕ) (liftSection : Q → A) (root target : Q) : A :=
  n • liftSection root - liftSection target

/-- The `[2] ∘ [3]` carry path telescopes to the direct `[6]` defect. -/
theorem carry_two_three_path
    (liftSection : Q → A) (h6 h2 q : Q) :
    2 • carry 3 liftSection h6 h2 + carry 2 liftSection h2 q =
      carry 6 liftSection h6 q := by
  simp only [carry, nsmul_sub]
  abel

/-- The `[3] ∘ [2]` carry path telescopes to the direct `[6]` defect. -/
theorem carry_three_two_path
    (liftSection : Q → A) (h6 h3 q : Q) :
    3 • carry 2 liftSection h6 h3 + carry 3 liftSection h3 q =
      carry 6 liftSection h6 q := by
  simp only [carry, nsmul_sub]
  abel

/-- The two binary/trinary section-carry paths commute in every additive
commutative group. -/
theorem commuting_carry_paths
    (liftSection : Q → A) (h6 h2 h3 q : Q) :
    2 • carry 3 liftSection h6 h2 + carry 2 liftSection h2 q =
      3 • carry 2 liftSection h6 h3 + carry 3 liftSection h3 q := by
  calc
    2 • carry 3 liftSection h6 h2 + carry 2 liftSection h2 q =
        carry 6 liftSection h6 q := carry_two_three_path liftSection h6 h2 q
    _ = 3 • carry 2 liftSection h6 h3 + carry 3 liftSection h3 q :=
      (carry_three_two_path liftSection h6 h3 q).symm

end CarryDiamond

section Representatives

/-- Four canonical representatives of one formal-layer equality yield an
integer winding in exactly the six-value interval frozen by H113. -/
theorem representative_winding_range
    (p a b c d numerator : ℤ)
    (hp : 0 < p)
    (ha0 : 0 ≤ a) (hap : a < p)
    (hb0 : 0 ≤ b) (hbp : b < p)
    (hc0 : 0 ≤ c) (hcp : c < p)
    (hd0 : 0 ≤ d) (hdp : d < p)
    (hnumerator : numerator = 2 * a + b - 3 * c - d)
    (hdiv : p ∣ numerator) :
    ∃ winding : ℤ,
      numerator = p * winding ∧ -3 ≤ winding ∧ winding ≤ 2 := by
  rcases hdiv with ⟨winding, hwinding⟩
  have hlower : -4 * p < numerator := by omega
  have hupper : numerator < 3 * p := by omega
  have hwlower : -4 < winding := by nlinarith [hwinding]
  have hwupper : winding < 3 := by nlinarith [hwinding]
  exact ⟨winding, hwinding, by omega, by omega⟩

end Representatives

section Oracle

variable {Point Value : Type*}

/-- Exactness of a public predicate supplies the canonical base-six digit.
Exactness is a hypothesis, not a conclusion of the formalization. -/
theorem exact_predicate_supplies_baseSix
    (observable : Point → Value) (predicate : Value → ℕ)
    (scalar : Point → ℕ)
    (hexact : ∀ q, predicate (observable q) = scalar q % 6) (q : Point) :
    predicate (observable q) = scalar q % 6 := by
  exact hexact q

/-- Reconstruct a natural number from successive exact remainder queries in
an arbitrary radix at least two. -/
def radixReconstruct (base : ℕ) (hbase : 2 ≤ base)
    (oracle : ℕ → ℕ) : ℕ → ℕ
  | 0 => 0
  | k + 1 =>
      oracle (k + 1) +
        base * radixReconstruct base hbase oracle ((k + 1) / base)
termination_by k => k
decreasing_by
  exact Nat.div_lt_self (by omega) (by omega)

/-- An exact remainder oracle makes repeated radix division recover every
natural number. -/
theorem radixReconstruct_eq
    (base : ℕ) (hbase : 2 ≤ base) (oracle : ℕ → ℕ)
    (hexact : ∀ k, oracle k = k % base) :
    ∀ k, radixReconstruct base hbase oracle k = k := by
  intro k
  induction k using Nat.strong_induction_on with
  | h k ih =>
      cases k with
      | zero => simp [radixReconstruct]
      | succ n =>
          rw [radixReconstruct, hexact,
            ih ((n + 1) / base)
              (Nat.div_lt_self (by omega) (by omega))]
          exact Nat.mod_add_div (n + 1) base

/-- H113's integer decoder is the radix-six instance of the generic theorem. -/
theorem baseSixReconstruct_eq
    (oracle : ℕ → ℕ) (hexact : ∀ k, oracle k = k % 6) :
    ∀ k, radixReconstruct 6 (by omega) oracle k = k := by
  exact radixReconstruct_eq 6 (by omega) oracle hexact

end Oracle

section GroupDecoder

variable {A : Type*}
variable [AddCommGroup A]

/-- One conditional H113 decoder update. The inverse-six action is exposed as
an explicit subgroup obligation. -/
theorem baseSix_group_step
    (generator current : A) (scalar digit inverseSix : ℕ)
    (hcurrent : current = scalar • generator)
    (hdigit : digit = scalar % 6)
    (hinverse : ∀ point : A, inverseSix • (6 • point) = point) :
    inverseSix • (current - digit • generator) =
      (scalar / 6) • generator := by
  rw [hcurrent, hdigit]
  have hdecompose : scalar % 6 + 6 * (scalar / 6) = scalar := by
    exact Nat.mod_add_div scalar 6
  calc
    inverseSix • (scalar • generator - (scalar % 6) • generator) =
        inverseSix • ((6 * (scalar / 6)) • generator) := by
      have hsubtract :
          scalar • generator - (scalar % 6) • generator =
            (6 * (scalar / 6)) • generator := by
        have hscalar :
            scalar • generator =
              (scalar % 6) • generator +
                (6 * (scalar / 6)) • generator := by
          rw [← add_nsmul]
          exact congrArg (fun n : ℕ => n • generator) hdecompose.symm
        rw [hscalar]
        abel
      rw [hsubtract]
    _ = inverseSix • (6 • ((scalar / 6) • generator)) := by
      rw [Nat.mul_comm 6 (scalar / 6), mul_nsmul]
    _ = (scalar / 6) • generator := hinverse _

/-- Independent EC verification makes a reconstructed scalar unique whenever
the admitted scalar-action map is injective. -/
theorem verified_scalar_unique
    {Scalar Point : Type*} (mul : Scalar → Point) (target : Point)
    (candidate trueScalar : Scalar) (hcandidate : mul candidate = target)
    (htrue : mul trueScalar = target) (hinjective : Function.Injective mul) :
    candidate = trueScalar := by
  apply hinjective
  rw [hcandidate, htrue]

end GroupDecoder

#print axioms carry_two_three_path
#print axioms carry_three_two_path
#print axioms commuting_carry_paths
#print axioms representative_winding_range
#print axioms exact_predicate_supplies_baseSix
#print axioms radixReconstruct_eq
#print axioms baseSixReconstruct_eq
#print axioms baseSix_group_step
#print axioms verified_scalar_unique

end H113BinaryTrinaryWinding
