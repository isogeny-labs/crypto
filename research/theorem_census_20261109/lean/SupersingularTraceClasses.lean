module
public import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
public import Mathlib.Algebra.Field.ZMod
public import Mathlib.GroupTheory.Perm.Cycle.Type
public import Mathlib.Data.Fintype.Option
public import Mathlib.Tactic

public section

/-! Arithmetic consequences of the explicitly supplied supersingular
trace-square classification. The classification itself is not proved here.
Prime order is required in this point-count-only argument. -/
namespace SupersingularTraceClassesAudit
noncomputable section
open scoped BigOperators
open scoped Classical

/-- Integer trace-square cases quoted in FST2010 p237. -/
def TraceSquares (q : ℕ) (t : ℤ) : Prop :=
  t^2=0 ∨ t^2=(q:ℤ) ∨ t^2=2*(q:ℤ) ∨ t^2=3*(q:ℤ) ∨ t^2=4*(q:ℤ)

theorem trace_powers {L : Type*} [Field L] (a b : L) (hab : a+1=b)
    (hc : b^2=0 ∨ b^2=a ∨ b^2=2*a ∨ b^2=3*a ∨ b^2=4*a) :
    a^2=1 ∨ a^3=1 ∨ a^4=1 ∨ a^6=1 := by
  rcases hc with h0 | h1 | h2 | h3 | h4
  · have hs : (a+1)^2=0 := by rw [hab]; exact h0
    have hz : a+1=0 := sq_eq_zero_iff.mp hs
    left
    linear_combination (a-1)*hz
  · have hs : (a+1)^2=a := by rw [hab]; exact h1
    right; left
    linear_combination (a-1)*hs
  · have hs : (a+1)^2=2*a := by rw [hab]; exact h2
    right; right; left
    linear_combination (a^2-1)*hs
  · have hs : (a+1)^2=3*a := by rw [hab]; exact h3
    right; right; right
    linear_combination (a^3-1)*(a+1)*hs
  · have hs : (a+1)^2=4*a := by rw [hab]; exact h4
    have hz : (a-1)^2=0 := by linear_combination hs
    have ha : a-1=0 := sq_eq_zero_iff.mp hz
    left
    linear_combination (a+1)*ha

private theorem small_divisor_cases (d k : ℕ)
    (hk : k=2 ∨ k=3 ∨ k=4 ∨ k=6) (hd : d∣k) :
    d=1 ∨ d=2 ∨ d=3 ∨ d=4 ∨ d=6 := by
  have hk6 : k≤6 := by omega
  have hd6 : d≤6 := (Nat.le_of_dvd (by omega : 0<k) hd).trans hk6
  have hsmall : ∀ x y : Fin 7,
      (y.val=2 ∨ y.val=3 ∨ y.val=4 ∨ y.val=6) → x.val∣y.val →
      x.val=1 ∨ x.val=2 ∨ x.val=3 ∨ x.val=4 ∨ x.val=6 := by decide +kernel
  exact hsmall ⟨d,by omega⟩ ⟨k,by omega⟩ hk hd

theorem power_order_cases {L : Type*} [Monoid L] (a : L)
    (hp : a^2=1 ∨ a^3=1 ∨ a^4=1 ∨ a^6=1) :
    orderOf a=1 ∨ orderOf a=2 ∨ orderOf a=3 ∨ orderOf a=4 ∨ orderOf a=6 := by
  rcases hp with h2 | h3 | h4 | h6
  · exact small_divisor_cases _ 2 (by simp) (orderOf_dvd_of_pow_eq_one h2)
  · exact small_divisor_cases _ 3 (by simp) (orderOf_dvd_of_pow_eq_one h3)
  · exact small_divisor_cases _ 4 (by simp) (orderOf_dvd_of_pow_eq_one h4)
  · exact small_divisor_cases _ 6 (by simp) (orderOf_dvd_of_pow_eq_one h6)

theorem cast_trace_squares {L : Type*} [CommRing L] (q : ℕ) (t : ℤ)
    (hc : TraceSquares q t) :
    (t:L)^2=0 ∨ (t:L)^2=(q:L) ∨ (t:L)^2=2*(q:L) ∨
      (t:L)^2=3*(q:L) ∨ (t:L)^2=4*(q:L) := by
  rcases hc with h0 | h1 | h2 | h3 | h4
  · left
    simpa using congrArg (Int.castRingHom L) h0
  · right; left
    simpa using congrArg (Int.castRingHom L) h1
  · right; right; left
    simpa using congrArg (Int.castRingHom L) h2
  · right; right; right; left
    simpa using congrArg (Int.castRingHom L) h3
  · right; right; right; right
    simpa using congrArg (Int.castRingHom L) h4

theorem trace_count_order_cases (q N r : ℕ) (hr : Nat.Prime r) (hrN : r∣N)
    (hc : TraceSquares q ((q:ℤ)+1-(N:ℤ))) :
    orderOf (q:ZMod r)=1 ∨ orderOf (q:ZMod r)=2 ∨ orderOf (q:ZMod r)=3 ∨
      orderOf (q:ZMod r)=4 ∨ orderOf (q:ZMod r)=6 := by
  let : Fact (Nat.Prime r) := ⟨hr⟩
  let t : ℤ := (q:ℤ)+1-(N:ℤ)
  have hN : (N:ZMod r)=0 := (ZMod.natCast_eq_zero_iff N r).mpr hrN
  have ht : (q:ZMod r)+1=(t:ZMod r) := by
    dsimp [t]
    push_cast
    simp only [hN,sub_zero]
  apply power_order_cases
  apply trace_powers (q:ZMod r) (t:ZMod r) ht
  exact cast_trace_squares q t hc

def fieldTrace {K : Type*} [Field K] [Fintype K]
    (E : WeierstrassCurve.Affine K) : ℤ :=
  (Fintype.card K:ℤ)+1-(Nat.card E.Point:ℤ)

theorem actual_curve_classified_order_cases {K : Type*} [Field K] [Fintype K]
    (E : WeierstrassCurve.Affine K) (r : ℕ) (hr : Nat.Prime r)
    (hrN : r∣Nat.card E.Point) (hc : TraceSquares (Fintype.card K) (fieldTrace E)) :
    orderOf (Fintype.card K:ZMod r)=1 ∨ orderOf (Fintype.card K:ZMod r)=2 ∨
      orderOf (Fintype.card K:ZMod r)=3 ∨ orderOf (Fintype.card K:ZMod r)=4 ∨
      orderOf (Fintype.card K:ZMod r)=6 :=
  trace_count_order_cases _ _ r hr hrN hc

theorem actual_curve_classified_bound {K : Type*} [Field K] [Fintype K]
    (E : WeierstrassCurve.Affine K) (r : ℕ) (hr : Nat.Prime r)
    (hrN : r∣Nat.card E.Point) (hc : TraceSquares (Fintype.card K) (fieldTrace E)) :
    orderOf (Fintype.card K:ZMod r)≤6 := by
  have hh := actual_curve_classified_order_cases E r hr hrN hc
  rcases hh with h1 | h2 | h3 | h4 | h6 <;> omega

theorem actual_prime_order_point_exists {K : Type*} [Field K] [Fintype K]
    (E : WeierstrassCurve.Affine K) (r : ℕ) (hr : Nat.Prime r)
    (hrN : r∣Nat.card E.Point) : ∃ P : E.Point,addOrderOf P=r := by
  let : Fact (Nat.Prime r) := ⟨hr⟩
  let : Finite (WithZero {xy : K × K // E.Nonsingular xy.1 xy.2}) :=
    inferInstanceAs (Finite (Option {xy : K × K // E.Nonsingular xy.1 xy.2}))
  let : Finite E.Point := Finite.of_injective E.nonsingularPointEquiv
    E.nonsingularPointEquiv.injective
  exact exists_prime_addOrderOf_dvd_card' r hrN

theorem trace_count_coprime (q N r : ℕ) (hr : Nat.Prime r) (hrN : r∣N)
    (hc : TraceSquares q ((q:ℤ)+1-(N:ℤ))) : Nat.Coprime r q := by
  let : Fact (Nat.Prime r) := ⟨hr⟩
  apply hr.coprime_iff_not_dvd.mpr
  intro hd
  have hz : (q:ZMod r)=0 := (ZMod.natCast_eq_zero_iff q r).mpr hd
  have hh := trace_count_order_cases q N r hr hrN hc
  rw [hz,orderOf_zero] at hh
  rcases hh with h1 | h2 | h3 | h4 | h6 <;> omega

private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩

theorem composite_numeric_order : orderOf (289:ZMod 81)=9 := by
  exact orderOf_eq_prime_pow (p:=3) (n:=1) (by decide +kernel) (by decide +kernel)

/-- Numerical hypothesis control only; no actual curve or point of order81
is asserted by these integer/modular conditions. -/
theorem composite_count_only_counterexample :
    Nat.Coprime 81 289 ∧ 81∣(324:ℕ) ∧ TraceSquares 289 ((289:ℤ)+1-324) ∧
    ¬ orderOf (289:ZMod 81)≤6 := by
  refine ⟨by decide,by decide,?_,?_⟩
  · unfold TraceSquares
    norm_num
  · rw [composite_numeric_order]
    decide

/-- The classification is an explicit premise, and the composite control
is only a count/trace/modular-order statement, not an actual curve witness. -/
theorem trace_classification_audit_certificate :
    (∀ (K : Type*) [Field K] [Fintype K] (E : WeierstrassCurve.Affine K)
      (r : ℕ), Nat.Prime r → r∣Nat.card E.Point →
      TraceSquares (Fintype.card K) (fieldTrace E) →
      (orderOf (Fintype.card K:ZMod r)=1 ∨ orderOf (Fintype.card K:ZMod r)=2 ∨
       orderOf (Fintype.card K:ZMod r)=3 ∨ orderOf (Fintype.card K:ZMod r)=4 ∨
       orderOf (Fintype.card K:ZMod r)=6) ∧ orderOf (Fintype.card K:ZMod r)≤6 ∧
      Nat.Coprime r (Fintype.card K) ∧ (∃ P : E.Point,addOrderOf P=r)) ∧
    (Nat.Coprime 81 289 ∧ 81∣(324:ℕ) ∧ TraceSquares 289 ((289:ℤ)+1-324) ∧
      ¬ orderOf (289:ZMod 81)≤6) := by
  constructor
  · intro K _ _ E r hr hrN hc
    exact ⟨actual_curve_classified_order_cases E r hr hrN hc,
      actual_curve_classified_bound E r hr hrN hc,
      trace_count_coprime _ _ r hr hrN hc,actual_prime_order_point_exists E r hr hrN⟩
  · exact composite_count_only_counterexample

#print axioms trace_classification_audit_certificate
#print axioms actual_curve_classified_order_cases
#print axioms composite_count_only_counterexample
end
end SupersingularTraceClassesAudit
