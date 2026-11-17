import JVSchemeStalk

namespace JVSchemeStalkAudit
noncomputable section
open JVHilbertAudit JVLocalMultiplicityAudit AlgebraicGeometry

def firstChartScheme : Scheme := AlgebraicGeometry.Spec (CommRingCat.of FirstChart)
def firstOpenScheme : Scheme := AlgebraicGeometry.Spec (CommRingCat.of FirstOpen)
def secondChartScheme : Scheme := AlgebraicGeometry.Spec (CommRingCat.of SecondChart)
def secondOpenScheme : Scheme := AlgebraicGeometry.Spec (CommRingCat.of SecondOpen)
def secondConeScheme : Scheme := AlgebraicGeometry.Spec (CommRingCat.of SecondAlgebra)

theorem first_chart_scheme_empty : IsEmpty firstChartScheme :=
  PrimeSpectrum.isEmpty_iff_subsingleton.mpr first_chart_subsingleton

theorem first_open_scheme_empty : IsEmpty firstOpenScheme :=
  PrimeSpectrum.isEmpty_iff_subsingleton.mpr first_open_subsingleton

def fieldPoint : PrimeSpectrum F := ⟨⊥, inferInstance⟩

/-- Points induced by the actual source-root evaluation, without a supplied point. -/
def secondChartPoint : secondChartScheme :=
  PrimeSpectrum.comap secondChartEvaluation fieldPoint

def secondOpenPoint : secondOpenScheme :=
  PrimeSpectrum.comap secondOpenEvaluation fieldPoint

def secondConePoint : secondConeScheme :=
  PrimeSpectrum.comap secondEvaluation.toRingHom fieldPoint

theorem second_chart_scheme_nonempty : Nonempty secondChartScheme := ⟨secondChartPoint⟩
theorem second_open_scheme_nonempty : Nonempty secondOpenScheme := ⟨secondOpenPoint⟩

def firstBasicOpen : coneScheme.Opens := PrimeSpectrum.basicOpen h
def secondBasicOpen : secondConeScheme.Opens := PrimeSpectrum.basicOpen secondH

theorem first_basic_open_empty : (firstBasicOpen : Set coneScheme) = ∅ := by
  apply Set.eq_empty_of_forall_notMem
  intro p hp
  change h ∉ p.asIdeal at hp
  rw [prime_eq_origin p.asIdeal p.isPrime] at hp
  exact hp origin_h

theorem second_basic_open_contains : secondConePoint ∈ secondBasicOpen := by
  change secondEvaluation secondH ≠ 0
  rw [second_h_value]
  exact one_ne_zero

theorem second_basic_open_nonempty : (secondBasicOpen : Set secondConeScheme).Nonempty :=
  ⟨secondConePoint, second_basic_open_contains⟩

theorem cone_scheme_card : Nat.card coneScheme = 1 := spectrum_card

theorem stalk_length_not_point_count :
    Module.length OriginStalk OriginStalk ≠ Nat.card coneScheme := by
  rw [actual_stalk_length, cone_scheme_card]
  decide

#print axioms first_chart_scheme_empty
#print axioms secondChartPoint
#print axioms first_basic_open_empty
#print axioms second_basic_open_nonempty
end
end JVSchemeStalkAudit
