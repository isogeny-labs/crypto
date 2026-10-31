import WeightedFirstFall
import Mathlib.Data.ZMod.Basic
import Mathlib.Algebra.Field.ZMod

/-!+# Exact ordinary first/last fall degrees for the guarded three-abscissa model

The theorem is about exactly the six generators in `GuardedModel.inputs`.
The first-fall convention is the explicit graded top-syzygy/Koszul convention
in `WeightedFirstFall`; last fall uses ordinary total degree and actual
bounded-degree closure. No numerical certificate, presumed refutation,
resultant hypothesis, or presumed unit ideal is an input to this theorem.
-/
namespace GuardedExactPair
noncomputable section
open MvPolynomial GridLower GuardedTops RationalFibers WeightedFirstFall GuardedLastFall

variable {K : Type*} [Field K]

theorem generator_range (α β a b c r : K) :
    Set.range (generator α β a b c r) = GuardedModel.inputs α β a b c r := by
  ext p
  constructor
  · rintro ⟨i,rfl⟩
    fin_cases i <;> simp [generator,GuardedModel.inputs]
  · intro hp
    rcases hp with rfl | rfl | rfl | rfl | rfl | rfl
    · exact ⟨0,by simp [generator]⟩
    · exact ⟨1,by simp [generator]⟩
    · exact ⟨2,by simp [generator]⟩
    · exact ⟨3,by simp [generator]⟩
    · exact ⟨4,by simp [generator]⟩
    · exact ⟨5,by simp [generator]⟩

/-- Complete exact pair under stronger polynomial-system generality:
every field with 6!=0, rational base lifts, and no field solution. -/
theorem exact_pair (α β a b c r : K) (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (hchar : (6 : K) ≠ 0)
    (hlift : ∀ x ∈ ({a,b,c} : Set K), ∃ Y : K, Y^2=x^3+α*x+β)
    (hns : NoSolution α β a b c r) :
    IsFirstFallDegree (generator α β a b c r) 4 ∧
      IsLastFallDegree (GuardedModel.inputs α β a b c r) 5 :=
  ⟨exact_first_four α β a b c r, exact_last_five α β a b c r hab hac hbc hchar hlift hns⟩

/-- The original nonsingular short-Weierstrass/rational-target setting.
The direct polynomial proof in fact needs fewer hypotheses than this wrapper. -/
theorem exact_pair_rational_curve (α β a b c r Y : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) (hchar : (6 : K) ≠ 0)
    (_hdisc : 4*α^3+27*β^2 ≠ 0) (_htarget : Y^2=r^3+α*r+β)
    (hlift : ∀ x ∈ ({a,b,c} : Set K), ∃ Z : K, Z^2=x^3+α*x+β)
    (hns : NoSolution α β a b c r) :
    IsFirstFallDegree (generator α β a b c r) 4 ∧
      IsLastFallDegree (GuardedModel.inputs α β a b c r) 5 :=
  exact_pair α β a b c r hab hac hbc hchar hlift hns

theorem prime_six_ne_zero (p : ℕ) [hp : Fact p.Prime] (hp3 : 3<p) : (6 : ZMod p) ≠ 0 := by
  intro h
  have hd : p ∣ 6 := (ZMod.natCast_eq_zero_iff 6 p).mp h
  have hm : p ∣ 2*3 := by simpa using hd
  rcases hp.out.dvd_mul.mp hm with h2 | h3
  · have hle : p≤2 := Nat.le_of_dvd (by omega) h2
    omega
  · have hle : p≤3 := Nat.le_of_dvd (by omega) h3
    omega

/-- Prime-field statement with p>3, not a characteristic assumption supplied
as an unproved premise. This covers the modulus sizes of the large-curve
applications without enumerating or trusting their numerical certificates. -/
theorem exact_pair_prime (p : ℕ) [Fact p.Prime] (hp3 : 3<p)
    (α β a b c r Y : ZMod p) (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (hdisc : 4*α^3+27*β^2 ≠ 0) (htarget : Y^2=r^3+α*r+β)
    (hlift : ∀ x ∈ ({a,b,c} : Set (ZMod p)), ∃ Z : ZMod p, Z^2=x^3+α*x+β)
    (hns : NoSolution α β a b c r) :
    IsFirstFallDegree (generator α β a b c r) 4 ∧
      IsLastFallDegree (GuardedModel.inputs α β a b c r) 5 :=
  exact_pair_rational_curve α β a b c r Y hab hac hbc (prime_six_ne_zero p hp3)
    hdisc htarget hlift hns

end
end GuardedExactPair

#print axioms GuardedExactPair.exact_pair
#print axioms GuardedExactPair.exact_pair_rational_curve
#print axioms GuardedExactPair.exact_pair_prime
