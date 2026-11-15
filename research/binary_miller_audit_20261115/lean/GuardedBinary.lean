module
public import GuardedStep

@[expose] public section
namespace BinaryMillerAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerRecursionAudit MillerBasechangeAudit FinitePointEvaluationAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- A carried binary evaluation trace. Constructors require only actual factor domains,
never an evaluation/correctness premise. This is a mathematical execution relation. -/
inductive GuardedRun (W : WeierstrassCurve.Affine K) (P : W.Point) (D : W.Point →₀ ℤ) :
    ℕ → W.Point → supportUnits W D.support → Kˣ → Prop
  | zero : GuardedRun W P D 0 0 1 1
  | one : GuardedRun W P D 1 P 1 1
  | even (n : ℕ) (hn : 2 ≤ n) (he : n % 2 = 0)
      (R : W.Point) (f : supportUnits W D.support) (v : Kˣ)
      (prior : GuardedRun W P D (n / 2) R f v) (hd : FactorsAdmissible W D R R) :
      GuardedRun W P D n (R + R) (f * f * liftedStep W D R R hd)
        (v * v * stepValue W D R R hd)
  | odd (n : ℕ) (hn : 2 ≤ n) (he : n % 2 ≠ 0)
      (R : W.Point) (f : supportUnits W D.support) (v : Kˣ)
      (prior : GuardedRun W P D (n / 2) R f v) (hd : FactorsAdmissible W D R R)
      (ha : FactorsAdmissible W D (R + R) P) :
      GuardedRun W P D n (R + R + P) ((f * f * liftedStep W D R R hd) * liftedStep W D (R + R) P ha)
        ((v * v * stepValue W D R R hd) * stepValue W D (R + R) P ha)

theorem guarded_run_value (W : WeierstrassCurve.Affine K) (P : W.Point) (D : W.Point →₀ ℤ)
    {n : ℕ} {R : W.Point} {f : supportUnits W D.support} {v : Kˣ}
    (h : GuardedRun W P D n R f v) : divisorEvaluation W D f = v := by
  induction h with
  | zero => exact map_one (divisorEvaluation W D)
  | one => exact map_one (divisorEvaluation W D)
  | even n hn he R f v prior hd ih =>
      rw [guarded_doubling_value, ih]
  | odd n hn he R f v prior hd ha ih =>
      rw [guarded_addition_value, guarded_doubling_value, ih]

theorem guarded_run_erasure (W : WeierstrassCurve.Affine K) (P : W.Point) (D : W.Point →₀ ℤ)
    {n : ℕ} {R : W.Point} {f : supportUnits W D.support} {v : Kˣ}
    (h : GuardedRun W P D n R f v) :
    binaryState W P n = (R, ((f : W.FunctionFieldˣ) : W.FunctionField)) := by
  induction h with
  | zero => simp [binary_state_small W P 0 (by omega)]
  | one => simp [binary_state_small W P 1 (by omega)]
  | even n hn he R f v prior hd ih =>
      rw [binary_state_large W P n hn, ih]
      dsimp only
      simp only [he, ite_true]
      apply congrArg (fun z : W.FunctionField => (R + R, z))
      change _ = ((f : W.FunctionFieldˣ) : W.FunctionField) * ((f : W.FunctionFieldˣ) : W.FunctionField) *
        ((liftedStep W D R R hd : W.FunctionFieldˣ) : W.FunctionField)
      rw [lifted_step_field]
  | odd n hn he R f v prior hd ha ih =>
      rw [binary_state_large W P n hn, ih]
      dsimp only
      simp only [he, ite_false]
      apply congrArg (fun z : W.FunctionField => (R + R + P, z))
      change _ = (((f : W.FunctionFieldˣ) : W.FunctionField) * ((f : W.FunctionFieldˣ) : W.FunctionField) *
        ((liftedStep W D R R hd : W.FunctionFieldˣ) : W.FunctionField)) *
        ((liftedStep W D (R + R) P ha : W.FunctionFieldˣ) : W.FunctionField)
      rw [lifted_step_field, lifted_step_field]

/-- All iterations: the numerical recurrence evaluates the actual chosen Miller function.
The only trace premises are the constructor's actual factor-unit guards. -/
theorem guarded_run_correct (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) (D : W.Point →₀ ℤ) {n : ℕ} {R : W.Point}
    {f : supportUnits W D.support} {v : Kˣ} (h : GuardedRun W P D n R f v) :
    R = n • P ∧ ((f : W.FunctionFieldˣ) : W.FunctionField) = millerFunction W P n ∧
      divisorEvaluation W D f = v := by
  have hs := guarded_run_erasure W P D h
  rw [binary_state_correct W hΔ P n] at hs
  exact ⟨(congrArg Prod.fst hs).symm, (congrArg Prod.snd hs).symm, guarded_run_value W P D h⟩


/-- All and only the actual factor guards encountered by the carried binary schedule. -/
def TraceAdmissible (W : WeierstrassCurve.Affine K) (P : W.Point) (D : W.Point →₀ ℤ) (n : ℕ) : Prop :=
  if _ : 2 ≤ n then
    let R := (binaryState W P (n / 2)).1
    TraceAdmissible W P D (n / 2) ∧ FactorsAdmissible W D R R ∧
      (n % 2 ≠ 0 → FactorsAdmissible W D (R + R) P)
  else True
termination_by n
decreasing_by exact Nat.div_lt_self (by omega) (by decide)

theorem admissible_run_exists (W : WeierstrassCurve.Affine K) (P : W.Point) (D : W.Point →₀ ℤ)
    (n : ℕ) (h : TraceAdmissible W P D n) :
    ∃ R f v, GuardedRun W P D n R f v := by
  induction n using Nat.strong_induction_on with
  | h n ih =>
    by_cases hn : 2 ≤ n
    · rw [TraceAdmissible] at h
      simp only [hn, dite_true] at h
      obtain ⟨R, f, v, hr⟩ := ih (n / 2) (Nat.div_lt_self (by omega) (by decide)) h.1
      have hp : (binaryState W P (n / 2)).1 = R := congrArg Prod.fst (guarded_run_erasure W P D hr)
      have hd : FactorsAdmissible W D R R := by simpa only [hp] using h.2.1
      by_cases he : n % 2 = 0
      · exact ⟨_, _, _, GuardedRun.even n hn he R f v hr hd⟩
      · have ha : FactorsAdmissible W D (R + R) P := by simpa only [hp] using h.2.2 he
        exact ⟨_, _, _, GuardedRun.odd n hn he R f v hr hd ha⟩
    · have hs : n = 0 ∨ n = 1 := by omega
      rcases hs with rfl | rfl
      · exact ⟨_, _, _, GuardedRun.zero⟩
      · exact ⟨_, _, _, GuardedRun.one⟩

#print axioms admissible_run_exists
#print axioms guarded_run_value
#print axioms guarded_run_erasure
#print axioms guarded_run_correct
end
end BinaryMillerAudit
