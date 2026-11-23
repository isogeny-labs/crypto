import SourceDescent

namespace JVSourceFieldAudit
noncomputable section

/-- The source factor base, with either sign: actual affine points whose
x-coordinate lies in the distinguished base field. -/
def InFactorBase (P : curve.Point) : Prop :=
  ∃ u : F, SemaevS3.pointX P=some (algebraMap F K u)

theorem factor_base_signed_sums_avoid_target_x (P Q : curve.Point)
    (hP : InFactorBase P) (hQ : InFactorBase Q) :
    SemaevS3.pointX (P+Q)≠some targetX ∧
      SemaevS3.pointX (P+-Q)≠some targetX := by
  obtain ⟨u,hu⟩ := hP
  obtain ⟨v,hv⟩ := hQ
  cases P with
  | zero => simp [SemaevS3.pointX] at hu
  | some x y hp =>
    change some x=some (algebraMap F K u) at hu
    have hx := Option.some.inj hu
    subst x
    cases Q with
    | zero => simp [SemaevS3.pointX] at hv
    | some z w hq =>
      change some z=some (algebraMap F K v) at hv
      have hz := Option.some.inj hv
      subst z
      have he := TheoremAudit.S3_pointX_iff curve rfl rfl rfl
        characteristic_two_nonzero hp hq targetX
      constructor
      · intro hh
        exact source_S3_has_no_base_solution u v (he.mpr (Or.inl hh))
      · intro hh
        exact source_S3_has_no_base_solution u v (he.mpr (Or.inr hh))

theorem actual_target_not_decomposable (P Q : curve.Point)
    (hP : InFactorBase P) (hQ : InFactorBase Q) :
    P+Q≠target ∧ P+-Q≠target := by
  have h := factor_base_signed_sums_avoid_target_x P Q hP hQ
  constructor
  · intro hh
    exact h.1 ((congrArg SemaevS3.pointX hh).trans target_x_coordinate)
  · intro hh
    exact h.2 ((congrArg SemaevS3.pointX hh).trans target_x_coordinate)

/-- Closed certificate connecting the source field, actual nonsingular curve,
actual target, exact coefficient descent and actual point group obstruction. -/
theorem literal_jv_source_field_certificate :
    Nat.card K=1030301 ∧ curve.Δ≠0 ∧ target≠0 ∧
    (∀ u : F, algebraMap F K u≠targetX) ∧
    descend 0 symmetric=f0 ∧ descend 1 symmetric=f1 ∧ descend 2 symmetric=f2 ∧
    (∀ u v : F, SemaevS3.S3 a b (algebraMap F K u) (algebraMap F K v) targetX≠0) ∧
    ∀ P Q : curve.Point, InFactorBase P → InFactorBase Q →
      P+Q≠target ∧ P+-Q≠target := by
  exact ⟨field_cardinality,curve_nonsingular,target_nonzero,target_x_not_base,
    actual_descent.1,actual_descent.2.1,actual_descent.2.2,
    source_S3_has_no_base_solution,actual_target_not_decomposable⟩

#print axioms literal_jv_source_field_certificate

end
end JVSourceFieldAudit
