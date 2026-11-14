import ConcreteCounterpoint

namespace VeluFrobeniusAudit
noncomputable section
open WeierstrassCurve.Affine FrobeniusPointDescentAudit

theorem irrational_scalar_without_stability_false :
    ¬∀ P : (baseCurve.baseChange F25).Point, baseCurve.Δ ≠ 0 →
      addOrderOf P = 3 → ∃ k : ℤ, k • P = pointFrobenius baseCurve P := by
  intro h
  obtain ⟨_, _, _, P, hP, _, hk, _⟩ := finite_curve_counter
  exact hk (h P baseCurve_delta_ne hP)

/-- Actual prime3 geometric counter, subgroup qualification and stable irrational control. -/
theorem velu_frobenius_certificate :
    (Nat.Prime 3 ∧ Nat.Coprime 3 5 ∧ baseCurve.Δ ≠ 0) ∧
    (∀ (G : Type) [AddCommGroup G] (F : G →+ G) (P : G),
      (∀ Q ∈ AddSubgroup.zmultiples P, F Q ∈ AddSubgroup.zmultiples P) ↔
        ∃ k : ℤ, k • P = F P) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [Algebra (ZMod 5) K] [CharP K 5]
      (a : K) (ha : a ^ 2 = 2),
      (3 : ℕ) • (rationalPoint : (baseCurve.baseChange K).Point) = 0 ∧
      (3 : ℕ) • antiPoint a ha = 0 ∧
      pointFrobenius baseCurve (rationalPoint : (baseCurve.baseChange K).Point) = rationalPoint ∧
      pointFrobenius baseCurve (antiPoint a ha) = -antiPoint a ha ∧
      addOrderOf (mixedPoint a ha) = 3 ∧
      pointFrobenius baseCurve (mixedPoint a ha) ∉ AddSubgroup.zmultiples (mixedPoint a ha)) ∧
    (Fintype.card F25 = 25 ∧ Module.finrank (ZMod 5) F25 = 2 ∧
      (baseCurve.baseChange F25).Δ ≠ 0 ∧
      ∃ P : (baseCurve.baseChange F25).Point, addOrderOf P = 3 ∧
        pointFrobenius baseCurve P ∉ AddSubgroup.zmultiples P ∧
        (¬∃ k : ℤ, k • P = pointFrobenius baseCurve P) ∧
        (¬∃ Q : baseCurve.Point, rationalInclusion F25 Q = P)) ∧
    (¬∀ P : (baseCurve.baseChange F25).Point, baseCurve.Δ ≠ 0 →
      addOrderOf P = 3 → ∃ k : ℤ, k • P = pointFrobenius baseCurve P) ∧
    (∃ B : (baseCurve.baseChange F25).Point, addOrderOf B = 3 ∧
      pointFrobenius baseCurve B = -B ∧
      (∀ Q ∈ AddSubgroup.zmultiples B, pointFrobenius baseCurve Q ∈ AddSubgroup.zmultiples B) ∧
      (¬∃ Q : baseCurve.Point, rationalInclusion F25 Q = B)) := by
  refine ⟨⟨by decide +kernel, by decide +kernel, baseCurve_delta_ne⟩, ?_, ?_,
    finite_curve_counter, irrational_scalar_without_stability_false,
    finite_stable_irrational_control⟩
  · intro G _ F P
    exact cyclic_stable_iff_scalar F P
  · intro K _ _ _ _ a ha
    exact ⟨rationalPoint_three, antiPoint_three a ha, rationalPoint_frobenius,
      antiPoint_frobenius a ha, (mixedPoint_counter a ha).1, (mixedPoint_counter a ha).2⟩

#print axioms irrational_scalar_without_stability_false
#print axioms velu_frobenius_certificate
end
end VeluFrobeniusAudit
