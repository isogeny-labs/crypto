import MumfordCantor

/-! The group structure below is matched to every actual polynomial Cantor
step. This is a finite arithmetic model, not a geometric Picard identification.
The order-two example refutes universal finite subgroup-proportion equality;
it makes no assertion about a typical large-prime or asymptotic regime. -/
namespace MumfordCantorGroupAudit
noncomputable section
open Polynomial MumfordSmoothnessAudit MumfordCantorAudit
set_option backward.isDefEq.respectTransparency false

abbrev Bits := Fin 3 → ZMod 2

def bits (i : Fin 8) : Bits :=
  ![(i.val:ZMod 2),((i.val/2:ℕ):ZMod 2),((i.val/4:ℕ):ZMod 2)]

theorem bits_bijective : Function.Bijective bits := by
  constructor <;> decide

def bitsEquiv : Fin 8 ≃ Bits := Equiv.ofBijective bits bits_bijective

theorem bits_sumIndex (i j : Fin 8) : bits (sumIndex i j)=bits i+bits j := by
  revert i j
  decide

def groupEquiv : ReducedPair ≃ Bits := pointEquiv.symm.trans bitsEquiv

instance : AddCommGroup ReducedPair := groupEquiv.addCommGroup
instance : DecidableEq ReducedPair := Classical.decEq _

theorem groupEquiv_point (i : Fin 8) : groupEquiv (point i)=bits i := by
  change bitsEquiv (pointEquiv.symm (pointEquiv i))=bits i
  rw [Equiv.symm_apply_apply]
  rfl

theorem point_add (i j : Fin 8) : point i+point j=point (sumIndex i j) := by
  apply groupEquiv.injective
  change groupEquiv.addEquiv (point i+point j)=groupEquiv (point (sumIndex i j))
  rw [map_add]
  simp only [Equiv.addEquiv_apply,groupEquiv_point]
  exact (bits_sumIndex i j).symm

theorem point_zero : point 0=0 := by
  apply groupEquiv.injective
  change groupEquiv (point 0)=groupEquiv.addEquiv 0
  rw [map_zero,groupEquiv_point]
  decide

theorem point_double (i : Fin 8) : point i+point i=0 := by
  rw [point_add,show sumIndex i i=0 by revert i; decide,point_zero]

theorem point_neg (i : Fin 8) : -point i=point i :=
  neg_eq_iff_add_eq_zero.mpr (point_double i)

theorem addition_is_Cantor (P Q : ReducedPair) : ZeroCantorStep P Q (P+Q) := by
  obtain ⟨i,rfl⟩ := point_surjective P
  obtain ⟨j,rfl⟩ := point_surjective Q
  rw [point_add]
  exact cantor_all_pairs i j

theorem cantor_iff_addition (P Q R : ReducedPair) :
    ZeroCantorStep P Q R ↔ R=P+Q := by
  constructor
  · intro h
    exact cantor_output_unique P Q R (P+Q) h (addition_is_Cantor P Q)
  · rintro rfl
    exact addition_is_Cantor P Q

def smallSubgroup : AddSubgroup ReducedPair where
  carrier := {P | P=point 0 ∨ P=point 1}
  zero_mem' := Or.inl point_zero.symm
  add_mem' := by
    intro P Q hP hQ
    rcases hP with rfl | rfl <;> rcases hQ with rfl | rfl
    · exact Or.inl (by rw [point_add]; rfl)
    · exact Or.inr (by rw [point_add]; rfl)
    · exact Or.inr (by rw [point_add]; rfl)
    · exact Or.inl (by rw [point_add]; rfl)
  neg_mem' := by
    intro P hP
    rcases hP with rfl | rfl
    · exact Or.inl (point_neg 0)
    · exact Or.inr (point_neg 1)

instance (P : ReducedPair) : Decidable (P∈smallSubgroup) := Classical.propDecidable _

theorem smallSubgroup_cyclic : smallSubgroup=AddSubgroup.zmultiples (point 1) := by
  apply le_antisymm
  · intro P hP
    rcases hP with hP | hP
    · rw [hP,point_zero]
      exact AddSubgroup.zero_mem _
    · rw [hP]
      exact AddSubgroup.mem_zmultiples _
  · exact AddSubgroup.zmultiples_le.mpr (Or.inr rfl)

theorem smallSubgroup_membership (i : Fin 8) :
    point i∈smallSubgroup ↔ i=0 ∨ i=1 := by
  change (point i=point 0 ∨ point i=point 1) ↔ _
  rw [point_injective.eq_iff,point_injective.eq_iff]

theorem smallSubgroup_cardinality : Fintype.card smallSubgroup=2 := by
  let e : {i : Fin 8 // i=0 ∨ i=1} ≃ smallSubgroup :=
    pointEquiv.subtypeEquiv (fun i=>(smallSubgroup_membership i).symm)
  exact (Fintype.card_congr e).symm.trans (by decide)

theorem point_one_order : addOrderOf (point 1)=2 := by
  apply addOrderOf_eq_prime
  · simpa only [two_nsmul] using point_double 1
  · intro h
    have hh : (1:Fin 8)=0 := point_injective (h.trans point_zero.symm)
    exact (by decide : (1:Fin 8)≠0) hh

theorem generator_coordinates : (point 1).u=X ∧ (point 1).v=0 := by
  simp [point,candidatePair,representative,uPolynomial,vPolynomial]

theorem subgroup_proper : smallSubgroup≠⊤ := by
  have hn : point 7∉smallSubgroup := by rw [smallSubgroup_membership]; decide
  intro he
  apply hn
  rw [he]
  exact AddSubgroup.mem_top _

theorem subgroup_all_smooth (P : smallSubgroup) : P.val.u.Splits := by
  rcases P.property with hP | hP
  · rw [hP]
    exact (representative_splits 0).mpr (by decide)
  · rw [hP]
    exact (representative_splits 1).mpr (by decide)

theorem subgroup_smooth_cardinality :
    Fintype.card {P : smallSubgroup // P.val.u.Splits}=2 := by
  rw [Fintype.card_congr (Equiv.subtypeUnivEquiv subgroup_all_smooth)]
  exact smallSubgroup_cardinality

def subgroupSmoothFraction : ℚ :=
  (Fintype.card {P : smallSubgroup // P.val.u.Splits}:ℚ)/Fintype.card smallSubgroup

theorem subgroupSmoothFraction_value : subgroupSmoothFraction=1 := by
  unfold subgroupSmoothFraction
  rw [subgroup_smooth_cardinality,smallSubgroup_cardinality]
  norm_num

theorem subgroup_proportion_unequal : subgroupSmoothFraction≠smoothFraction := by
  rw [subgroupSmoothFraction_value,smoothFraction_value]
  norm_num

theorem subgroup_proportion_difference : subgroupSmoothFraction-smoothFraction=1/8 := by
  rw [subgroupSmoothFraction_value,smoothFraction_value]
  norm_num

def cyclicSmoothFraction (P : ReducedPair) : ℚ :=
  (Nat.card {Q : AddSubgroup.zmultiples P // Q.val.u.Splits}:ℚ)/
    Nat.card (AddSubgroup.zmultiples P)

theorem actual_cyclic_fraction : cyclicSmoothFraction (point 1)=1 := by
  unfold cyclicSmoothFraction
  rw [← smallSubgroup_cyclic,Nat.card_eq_fintype_card,Nat.card_eq_fintype_card,
    subgroup_smooth_cardinality,smallSubgroup_cardinality]
  norm_num

theorem universal_finite_cyclic_equality_false :
    ¬ ∀ P : ReducedPair, cyclicSmoothFraction P=smoothFraction := by
  intro h
  have hh := h (point 1)
  rw [actual_cyclic_fraction,smoothFraction_value] at hh
  norm_num at hh

theorem point_nonzero (i : Fin 8) : point i≠0 ↔ i≠0 := by
  rw [← point_zero,point_injective.ne_iff]

theorem point_splits (i : Fin 8) : (point i).u.Splits ↔ i≠7 :=
  representative_splits i

theorem ambient_nonzero_smooth_cardinality :
    Fintype.card {P : ReducedPair // P≠0 ∧ P.u.Splits}=6 := by
  let e : {i : Fin 8 // i≠0 ∧ i≠7} ≃ {P : ReducedPair // P≠0 ∧ P.u.Splits} :=
    pointEquiv.subtypeEquiv (fun i=>by
      change (i≠0 ∧ i≠7) ↔ point i≠0 ∧ (point i).u.Splits
      rw [point_nonzero,point_splits])
  exact (Fintype.card_congr e).symm.trans (by decide)

theorem subgroup_nonzero_smooth_cardinality :
    Fintype.card {P : ReducedPair // P∈smallSubgroup ∧ P≠0 ∧ P.u.Splits}=1 := by
  let e : {i : Fin 8 // (i=0 ∨ i=1) ∧ i≠0 ∧ i≠7} ≃
      {P : ReducedPair // P∈smallSubgroup ∧ P≠0 ∧ P.u.Splits} :=
    pointEquiv.subtypeEquiv (fun i=>by
      change ((i=0 ∨ i=1) ∧ i≠0 ∧ i≠7) ↔
        point i∈smallSubgroup ∧ point i≠0 ∧ (point i).u.Splits
      rw [smallSubgroup_membership,point_nonzero,point_splits])
  exact (Fintype.card_congr e).symm.trans (by decide)

/-- Nonzero smooth numerators, with uniform sampling from the whole group,
including zero. Conditioning the sampling space on nonzero is a different ratio. -/
theorem useful_smooth_fractions :
    (Fintype.card {P : ReducedPair // P≠0 ∧ P.u.Splits}:ℚ)/
        Fintype.card ReducedPair=3/4 ∧
    (Fintype.card {P : ReducedPair // P∈smallSubgroup ∧ P≠0 ∧ P.u.Splits}:ℚ)/
        Fintype.card smallSubgroup=1/2 := by
  rw [ambient_nonzero_smooth_cardinality,reducedPair_cardinality,
    subgroup_nonzero_smooth_cardinality,smallSubgroup_cardinality]
  norm_num

/-- Complete finite arithmetic model; no geometric or asymptotic premise is hidden. -/
theorem cantor_subgroup_audit_certificate :
    Squarefree curvePolynomial ∧
    Fintype.card ReducedPair=8 ∧
    (∀ P Q R : ReducedPair, ZeroCantorStep P Q R ↔ R=P+Q) ∧
    addOrderOf (point 1)=2 ∧
    ((point 1).u=X ∧ (point 1).v=0) ∧ smallSubgroup≠⊤ ∧
    smallSubgroup=AddSubgroup.zmultiples (point 1) ∧
    Fintype.card smallSubgroup=2 ∧
    subgroupSmoothFraction=1 ∧ smoothFraction=7/8 ∧
    (¬ ∀ P : ReducedPair, cyclicSmoothFraction P=smoothFraction) ∧
    Fintype.card {P : ReducedPair // P≠0 ∧ P.u.Splits}=6 ∧
    Fintype.card {P : ReducedPair // P∈smallSubgroup ∧ P≠0 ∧ P.u.Splits}=1 := by
  exact ⟨curve_squarefree,reducedPair_cardinality,cantor_iff_addition,
    point_one_order,generator_coordinates,subgroup_proper,smallSubgroup_cyclic,smallSubgroup_cardinality,
    subgroupSmoothFraction_value,smoothFraction_value,universal_finite_cyclic_equality_false,
    ambient_nonzero_smooth_cardinality,subgroup_nonzero_smooth_cardinality⟩

#print axioms cantor_subgroup_audit_certificate
#print axioms useful_smooth_fractions

end
end MumfordCantorGroupAudit
