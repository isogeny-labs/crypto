import GlobalWeights
import ResidueDegreeControl

namespace ResidueWeightedLengthAudit
noncomputable section

private instance primeTwo : Fact (Nat.Prime 2) := ⟨by decide⟩
abbrev Base := LocalAlgebraLengthAudit.F2
abbrev Quad := LocalAlgebraLengthAudit.F4
abbrev ProductAlgebra := Base × Quad

def firstProjection : ProductAlgebra →ₐ[Base] Base :=
  { RingHom.fst Base Quad with commutes' := fun _ => rfl }
def secondProjection : ProductAlgebra →ₐ[Base] Quad :=
  { RingHom.snd Base Quad with commutes' := fun _ => rfl }

theorem first_projection_surjective : Function.Surjective firstProjection :=
  fun c => ⟨(c, 0), rfl⟩
theorem second_projection_surjective : Function.Surjective secondProjection :=
  fun c => ⟨(0, c), rfl⟩

instance firstProjectionAlgebra : Algebra ProductAlgebra Base :=
  firstProjection.toRingHom.toAlgebra
instance secondProjectionAlgebra : Algebra ProductAlgebra Quad :=
  secondProjection.toRingHom.toAlgebra

theorem product_scalar_dimension : Module.finrank Base ProductAlgebra = 3 := by
  rw [Module.finrank_prod, Module.finrank_self,
    LocalAlgebraLengthAudit.residue_degree_control.2.2.1]

theorem product_ring_length : Module.length ProductAlgebra ProductAlgebra = 2 := by
  have hl : Module.length ProductAlgebra Base = 1 := by
    rw [Module.length_eq_of_surjective (S := ProductAlgebra) (R := Base) (M := Base)
      first_projection_surjective, Module.length_eq_finrank, Module.finrank_self]
    rfl
  have hr : Module.length ProductAlgebra Quad = 1 := by
    rw [Module.length_eq_of_surjective (S := ProductAlgebra) (R := Quad) (M := Quad)
      second_projection_surjective, Module.length_eq_finrank, Module.finrank_self]
    rfl
  let inc : Quad →ₗ[ProductAlgebra] ProductAlgebra :=
    { toFun := fun c => (0, c)
      map_add' := by intro a b; ext <;> simp
      map_smul' := by
        rintro ⟨a, b⟩ c
        change (0, b * c) = (a * 0, b * c)
        simp }
  let pr : ProductAlgebra →ₗ[ProductAlgebra] Base :=
    { toFun := Prod.fst
      map_add' := by intros; rfl
      map_smul' := by rintro ⟨a, b⟩ ⟨c, d⟩; rfl }
  have hi : Function.Injective inc := by
    intro a b hab
    exact congrArg Prod.snd hab
  have hs : Function.Surjective pr := fun c => ⟨(c, 0), rfl⟩
  have he : Function.Exact inc pr := by
    rintro ⟨a, b⟩
    change a = 0 ↔ ∃ c, (0, c) = (a, b)
    constructor
    · intro ha
      subst a
      exact ⟨b, rfl⟩
    · rintro ⟨c, hc⟩
      exact (congrArg Prod.fst hc).symm
  rw [Module.length_eq_add_of_exact inc pr hi hs he, hr, hl]
  rfl

def fieldPrime (K : Type*) [Field K] : PrimeSpectrum K := ⟨⊥, inferInstance⟩

theorem field_prime_unique (K : Type*) [Field K] (p : PrimeSpectrum K) : p = fieldPrime K := by
  apply PrimeSpectrum.ext
  exact p.asIdeal.eq_bot_of_prime

def fieldPrimeEquivUnit (K : Type*) [Field K] : PrimeSpectrum K ≃ PUnit.{1} where
  toFun _ := PUnit.unit
  invFun _ := fieldPrime K
  left_inv p := (field_prime_unique K p).symm
  right_inv p := by cases p; rfl

theorem field_prime_card (K : Type*) [Field K] : Nat.card (PrimeSpectrum K) = 1 := by
  calc
    Nat.card (PrimeSpectrum K) = Nat.card PUnit.{1} := Nat.card_congr (fieldPrimeEquivUnit K)
    _ = 1 := by simp

theorem product_prime_card : Nat.card (PrimeSpectrum ProductAlgebra) = 2 := by
  calc
    Nat.card (PrimeSpectrum ProductAlgebra) =
        Nat.card (PrimeSpectrum Base ⊕ PrimeSpectrum Quad) :=
      Nat.card_congr (PrimeSpectrum.primeSpectrumProd Base Quad)
    _ = Nat.card (PrimeSpectrum Base) + Nat.card (PrimeSpectrum Quad) := Nat.card_sum
    _ = 2 := by rw [field_prime_card, field_prime_card]

theorem product_weighted_degree : weightedDegree Base ProductAlgebra = 3 := by
  rw [← scalar_dimension_eq_weighted_degree Base ProductAlgebra]
  exact product_scalar_dimension

theorem product_unweighted_degree : unweightedDegree Base ProductAlgebra = 2 := by
  rw [reduced_unweighted_degree Base ProductAlgebra]
  exact product_prime_card

theorem deleting_residue_weights_fails :
    weightedDegree Base ProductAlgebra ≠ unweightedDegree Base ProductAlgebra := by
  rw [product_weighted_degree, product_unweighted_degree]
  decide

def firstIdeal : Ideal ProductAlgebra := RingHom.ker firstProjection.toRingHom
def secondIdeal : Ideal ProductAlgebra := RingHom.ker secondProjection.toRingHom

instance first_maximal : firstIdeal.IsMaximal :=
  RingHom.ker_isMaximal_of_surjective firstProjection.toRingHom first_projection_surjective
instance second_maximal : secondIdeal.IsMaximal :=
  RingHom.ker_isMaximal_of_surjective secondProjection.toRingHom second_projection_surjective

def firstPoint : PrimeSpectrum ProductAlgebra := ⟨firstIdeal, first_maximal.isPrime⟩
def secondPoint : PrimeSpectrum ProductAlgebra := ⟨secondIdeal, second_maximal.isPrime⟩

def firstResidueEquiv : firstIdeal.ResidueField ≃ₐ[Base] Base :=
  (maximalResidueQuotientEquiv Base ProductAlgebra firstIdeal).symm.trans
    (Ideal.quotientKerAlgEquivOfSurjective first_projection_surjective)

def secondResidueEquiv : secondIdeal.ResidueField ≃ₐ[Base] Quad :=
  (maximalResidueQuotientEquiv Base ProductAlgebra secondIdeal).symm.trans
    (Ideal.quotientKerAlgEquivOfSurjective second_projection_surjective)

theorem product_residue_degrees :
    Module.finrank Base firstIdeal.ResidueField = 1 ∧
    Module.finrank Base secondIdeal.ResidueField = 2 := by
  exact ⟨firstResidueEquiv.toLinearEquiv.finrank_eq.trans (Module.finrank_self Base),
    secondResidueEquiv.toLinearEquiv.finrank_eq.trans
      LocalAlgebraLengthAudit.residue_degree_control.2.2.1⟩

theorem first_point_from_product :
    (PrimeSpectrum.primeSpectrumProd Base Quad).symm (Sum.inl (fieldPrime Base)) = firstPoint := by
  apply PrimeSpectrum.ext
  rw [PrimeSpectrum.primeSpectrumProd_symm_inl_asIdeal]
  ext z
  change (z.1 = 0 ∧ z.2 ∈ (⊤ : Ideal Quad)) ↔ z.1 = 0
  simp

theorem second_point_from_product :
    (PrimeSpectrum.primeSpectrumProd Base Quad).symm (Sum.inr (fieldPrime Quad)) = secondPoint := by
  apply PrimeSpectrum.ext
  rw [PrimeSpectrum.primeSpectrumProd_symm_inr_asIdeal]
  ext z
  change (z.1 ∈ (⊤ : Ideal Base) ∧ z.2 = 0) ↔ z.2 = 0
  simp

theorem product_prime_classification (p : PrimeSpectrum ProductAlgebra) :
    p = firstPoint ∨ p = secondPoint := by
  obtain ⟨q, hq⟩ := (PrimeSpectrum.primeSpectrumProd Base Quad).symm.surjective p
  rcases q with q | q
  · rw [field_prime_unique Base q, first_point_from_product] at hq
    exact Or.inl hq.symm
  · rw [field_prime_unique Quad q, second_point_from_product] at hq
    exact Or.inr hq.symm

theorem product_points_distinct : firstPoint ≠ secondPoint := by
  intro he
  have hm : ((1 : Base), (0 : Quad)) ∈ secondPoint.asIdeal := rfl
  have hn : ((1 : Base), (0 : Quad)) ∉ firstPoint.asIdeal := by
    change (1 : Base) ≠ 0
    exact one_ne_zero
  rw [he] at hn
  exact hn hm

theorem product_local_length_one (p : PrimeSpectrum ProductAlgebra) :
    Module.length (Localization.AtPrime p.asIdeal) (Localization.AtPrime p.asIdeal) = 1 :=
  reduced_prime_local_length Base ProductAlgebra p

theorem product_not_local : ¬ IsLocalRing ProductAlgebra := by
  intro hl
  have he : ((1 : Base), (0 : Quad)) + ((0 : Base), (1 : Quad)) = 1 := by ext <;> simp
  rcases hl.isUnit_or_isUnit_of_add_one he with hu | hu
  · exact (not_isUnit_zero : ¬ IsUnit (0 : Quad)) (by
      simpa using hu.map (RingHom.snd Base Quad))
  · exact (not_isUnit_zero : ¬ IsUnit (0 : Base)) (by
      simpa using hu.map (RingHom.fst Base Quad))

theorem split_product_weighted : weightedDegree Base (Base × Base) = 2 := by
  rw [← scalar_dimension_eq_weighted_degree Base (Base × Base), Module.finrank_prod,
    Module.finrank_self]

theorem split_product_unweighted : unweightedDegree Base (Base × Base) = 2 := by
  rw [reduced_unweighted_degree Base (Base × Base)]
  calc
    Nat.card (PrimeSpectrum (Base × Base)) =
        Nat.card (PrimeSpectrum Base ⊕ PrimeSpectrum Base) :=
      Nat.card_congr (PrimeSpectrum.primeSpectrumProd Base Base)
    _ = Nat.card (PrimeSpectrum Base) + Nat.card (PrimeSpectrum Base) := Nat.card_sum
    _ = 2 := by rw [field_prime_card]

#print axioms product_ring_length
#print axioms product_prime_card
#print axioms deleting_residue_weights_fails
#print axioms product_residue_degrees
end
end ResidueWeightedLengthAudit
