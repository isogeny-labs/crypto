import MumfordStollFullCertificates

/-! The full arithmetic is defined by the unique polynomial-run relation.
All 100 existence certificates precede the finite group transport. -/
namespace MumfordStollFullGroupAudit
noncomputable section
open Polynomial MumfordStollAudit MumfordStollGeneralAudit MumfordStollFullCertificatesAudit
set_option backward.isDefEq.respectTransparency false
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩

theorem point_index (i : Fin 10) : pointEquiv.symm (point i)=i :=
  pointEquiv.symm_apply_apply i

def SelectedRun (P Q R : ReducedPair) : Prop :=
  GeneralCantorRun (bezoutA (pointEquiv.symm P) (pointEquiv.symm Q))
    (bezoutB (pointEquiv.symm P) (pointEquiv.symm Q))
    (bezoutC (pointEquiv.symm P) (pointEquiv.symm Q)) P Q R

theorem selected_run_exists (P Q : ReducedPair) : ∃ R : ReducedPair, SelectedRun P Q R := by
  obtain ⟨i,rfl⟩ := point_surjective P
  obtain ⟨j,rfl⟩ := point_surjective Q
  refine ⟨point (expectedIndex i j),?_⟩
  simpa only [SelectedRun,point_index] using all_pairs i j

/-- The run witnesses identify the actual normalized full polynomial gcd. -/
theorem selected_run_actual_gcd (P Q R : ReducedPair) (h : SelectedRun P Q R) :
    ∃ d U : K[X], d=gcd (gcd P.u Q.u) (P.v+Q.v) ∧
      d.Monic ∧ U.Monic ∧ d^2*U=P.u*Q.u := by
  obtain ⟨d,U,V,q,A,B,C,F,hd,hU,hA,hB,hC,hbez,hcomp,_⟩ := h
  exact ⟨d,U,(MumfordStollCycleGroupAudit.normalized_full_gcd P Q d _ _ _
    A B C hd hA hB hC hbez).symm,hd,hU,hcomp⟩

theorem selected_run_unique (P Q R S : ReducedPair)
    (hR : SelectedRun P Q R) (hS : SelectedRun P Q S) : R=S :=
  run_output_unique _ _ _ P Q R S hR hS

/-- Deterministic output of the actual polynomial algorithm relation,
defined before introducing the ambient group structure. -/
def cantorAdd (P Q : ReducedPair) : ReducedPair := Classical.choose (selected_run_exists P Q)

theorem cantorAdd_runs (P Q : ReducedPair) : SelectedRun P Q (cantorAdd P Q) :=
  Classical.choose_spec (selected_run_exists P Q)

theorem selected_run_iff_output (P Q R : ReducedPair) :
    SelectedRun P Q R ↔ R=cantorAdd P Q := by
  constructor
  · exact fun h=>selected_run_unique P Q R (cantorAdd P Q) h (cantorAdd_runs P Q)
  · rintro rfl
    exact cantorAdd_runs P Q

theorem repPerm_involutive : Function.Involutive repPerm := by
  change ∀ i : Fin 10,repPerm (repPerm i)=i
  decide

def permEquiv : Fin 10≃Fin 10 :=
  ⟨repPerm,repPerm,repPerm_involutive,repPerm_involutive⟩

def scalarEquiv : ReducedPair≃Fin 10 := pointEquiv.symm.trans permEquiv

instance : AddCommGroup ReducedPair := scalarEquiv.addCommGroup
instance : DecidableEq ReducedPair := Classical.decEq _

instance (H : AddSubgroup ReducedPair) : Fintype H := by
  letI : Finite H := Finite.of_injective (fun P : H=>P.val) Subtype.val_injective
  exact Fintype.ofFinite H

theorem scalarEquiv_point (i : Fin 10) : scalarEquiv (point i)=repPerm i := by
  change repPerm (pointEquiv.symm (point i))=repPerm i
  rw [point_index]

theorem repPerm_expectedIndex (i j : Fin 10) :
    repPerm (expectedIndex i j)=repPerm i+repPerm j := repPerm_involutive _

theorem point_add (i j : Fin 10) : point i+point j=point (expectedIndex i j) := by
  apply scalarEquiv.injective
  change scalarEquiv.addEquiv (point i+point j)=scalarEquiv (point (expectedIndex i j))
  rw [map_add]
  simp only [Equiv.addEquiv_apply,scalarEquiv_point,repPerm_expectedIndex]

theorem point_zero : point 0=0 := by
  apply scalarEquiv.injective
  change scalarEquiv (point 0)=scalarEquiv.addEquiv 0
  rw [map_zero,scalarEquiv_point]
  rfl

def negIndex (i : Fin 10) : Fin 10 := repPerm (-repPerm i)

theorem point_neg (i : Fin 10) : -point i=point (negIndex i) := by
  apply scalarEquiv.injective
  change scalarEquiv.addEquiv (-point i)=scalarEquiv (point (negIndex i))
  rw [map_neg]
  simp only [Equiv.addEquiv_apply,scalarEquiv_point,negIndex]
  exact (repPerm_involutive _).symm

theorem addition_is_Cantor (P Q : ReducedPair) : SelectedRun P Q (P+Q) := by
  obtain ⟨i,rfl⟩ := point_surjective P
  obtain ⟨j,rfl⟩ := point_surjective Q
  rw [point_add]
  simpa only [SelectedRun,point_index] using all_pairs i j

theorem addition_eq_cantorAdd (P Q : ReducedPair) : P+Q=cantorAdd P Q :=
  (selected_run_iff_output _ _ _).mp (addition_is_Cantor P Q)

def properSubgroup : AddSubgroup ReducedPair where
  carrier := {P | (pointEquiv.symm P).val<5}
  zero_mem' := by
    rw [←point_zero]
    change (pointEquiv.symm (point 0)).val<5
    rw [point_index]
    decide
  add_mem' := by
    intro P Q hP hQ
    obtain ⟨i,rfl⟩ := point_surjective P
    obtain ⟨j,rfl⟩ := point_surjective Q
    change (pointEquiv.symm (point i)).val<5 at hP
    change (pointEquiv.symm (point j)).val<5 at hQ
    rw [point_index] at hP hQ
    rw [point_add]
    change (pointEquiv.symm (point (expectedIndex i j))).val<5
    rw [point_index]
    exact (by decide : ∀ i j : Fin 10, i.val<5 → j.val<5 → (expectedIndex i j).val<5) i j hP hQ
  neg_mem' := by
    intro P hP
    obtain ⟨i,rfl⟩ := point_surjective P
    change (pointEquiv.symm (point i)).val<5 at hP
    rw [point_index] at hP
    rw [point_neg]
    change (pointEquiv.symm (point (negIndex i))).val<5
    rw [point_index]
    exact (by decide : ∀ i : Fin 10, i.val<5 → (negIndex i).val<5) i hP

theorem subgroup_membership (i : Fin 10) : point i∈properSubgroup ↔ i.val<5 := by
  change (pointEquiv.symm (point i)).val<5 ↔ i.val<5
  rw [point_index]

theorem generator_multiples (i : Fin 10) (h : i.val<5) : i.val • point 1=point i := by
  apply scalarEquiv.injective
  change scalarEquiv.addEquiv (i.val • point 1)=scalarEquiv (point i)
  rw [map_nsmul]
  simp only [Equiv.addEquiv_apply,scalarEquiv_point]
  exact (by decide : ∀ i : Fin 10, i.val<5 → i.val • repPerm 1=repPerm i) i h

theorem subgroup_cyclic : properSubgroup=AddSubgroup.zmultiples (point 1) := by
  apply le_antisymm
  · intro P hP
    obtain ⟨i,rfl⟩ := point_surjective P
    have hi := (subgroup_membership i).mp hP
    rw [←generator_multiples i hi]
    exact AddSubgroup.nsmul_mem _ (AddSubgroup.mem_zmultiples _) i.val
  · exact AddSubgroup.zmultiples_le.mpr ((subgroup_membership 1).mpr (by decide))

theorem generator_order : addOrderOf (point 1)=5 := by
  apply addOrderOf_eq_prime
  · apply scalarEquiv.injective
    change scalarEquiv.addEquiv (5 • point 1)=scalarEquiv.addEquiv 0
    rw [map_nsmul,map_zero]
    simp only [Equiv.addEquiv_apply,scalarEquiv_point]
    decide
  · intro h
    have he := point_injective (h.trans point_zero.symm)
    exact (by decide : (1:Fin 10)≠0) he

theorem generator_coordinates : (point 1).u=X ∧ (point 1).v=1 := by
  simp [point,candidatePair,representative,uPolynomial,vPolynomial]

theorem subgroup_proper : properSubgroup≠⊤ := by
  have hn : point 9∉properSubgroup := by rw [subgroup_membership]; decide
  intro he
  apply hn
  rw [he]
  exact AddSubgroup.mem_top _

theorem subgroup_cardinality : Fintype.card properSubgroup=5 := by
  let e : {i : Fin 10 // i.val<5} ≃ properSubgroup :=
    pointEquiv.subtypeEquiv (fun i=>(subgroup_membership i).symm)
  exact (Fintype.card_congr e).symm.trans (by decide)

theorem subgroup_all_smooth (P : properSubgroup) : P.val.u.Splits := by
  rcases P with ⟨P,hP⟩
  obtain ⟨i,rfl⟩ := point_surjective P
  have hi := (subgroup_membership i).mp hP
  exact (representative_splits i).mpr (by omega)

instance (H : AddSubgroup ReducedPair) (P : H) : Decidable P.val.u.Splits :=
  Classical.propDecidable _

theorem subgroup_smooth_cardinality :
    Fintype.card {P : properSubgroup // P.val.u.Splits}=5 := by
  rw [Fintype.card_congr (Equiv.subtypeUnivEquiv subgroup_all_smooth)]
  exact subgroup_cardinality

def ambientFraction : ℚ :=
  (Fintype.card {P : ReducedPair // P.u.Splits} : ℚ)/Fintype.card ReducedPair

def subgroupFraction (H : AddSubgroup ReducedPair) : ℚ :=
  (Fintype.card {P : H // P.val.u.Splits} : ℚ)/Fintype.card H

theorem exact_fractions : subgroupFraction properSubgroup=1 ∧ ambientFraction=4/5 ∧
    subgroupFraction properSubgroup≠ambientFraction := by
  have hH : subgroupFraction properSubgroup=1 := by
    norm_num [subgroupFraction,subgroup_smooth_cardinality,subgroup_cardinality]
  have hA : ambientFraction=4/5 := by
    norm_num [ambientFraction,smooth_reducedPair_cardinality,reducedPair_cardinality]
  exact ⟨hH,hA,by rw [hH,hA]; norm_num⟩

theorem universal_cyclic_fraction_equality_false :
    ¬ ∀ P : ReducedPair, subgroupFraction (AddSubgroup.zmultiples P)=ambientFraction := by
  intro h
  have hh := h (point 1)
  rw [←subgroup_cyclic] at hh
  exact exact_fractions.2.2 hh

theorem point_nonzero (i : Fin 10) : point i≠0 ↔ i≠0 := by
  rw [←point_zero,point_injective.ne_iff]

theorem point_splits (i : Fin 10) : (point i).u.Splits ↔ i.val<8 :=
  representative_splits i

theorem ambient_nonzero_smooth_cardinality :
    Nat.card {P : ReducedPair // P≠0 ∧ P.u.Splits}=7 := by
  let e : {i : Fin 10 // i≠0 ∧ i.val<8} ≃
      {P : ReducedPair // P≠0 ∧ P.u.Splits} :=
    pointEquiv.subtypeEquiv (fun i=>by
      change (i≠0 ∧ i.val<8) ↔ point i≠0 ∧ (point i).u.Splits
      rw [point_nonzero,point_splits])
  exact (Nat.card_congr e).symm.trans (by rw [Nat.card_eq_fintype_card]; decide)

theorem ambient_nonzero_cardinality : Nat.card {P : ReducedPair // P≠0}=9 := by
  let e : {i : Fin 10 // i≠0} ≃ {P : ReducedPair // P≠0} :=
    pointEquiv.subtypeEquiv (fun i=>(point_nonzero i).symm)
  exact (Nat.card_congr e).symm.trans (by rw [Nat.card_eq_fintype_card]; decide)

theorem subgroup_nonzero_smooth_cardinality :
    Nat.card {P : ReducedPair // P∈properSubgroup ∧ P≠0 ∧ P.u.Splits}=4 := by
  let e : {i : Fin 10 // i.val<5 ∧ i≠0 ∧ i.val<8} ≃
      {P : ReducedPair // P∈properSubgroup ∧ P≠0 ∧ P.u.Splits} :=
    pointEquiv.subtypeEquiv (fun i=>by
      change (i.val<5 ∧ i≠0 ∧ i.val<8) ↔
        point i∈properSubgroup ∧ point i≠0 ∧ (point i).u.Splits
      rw [subgroup_membership,point_nonzero,point_splits])
  exact (Nat.card_congr e).symm.trans (by rw [Nat.card_eq_fintype_card]; decide)

theorem subgroup_nonzero_cardinality :
    Nat.card {P : ReducedPair // P∈properSubgroup ∧ P≠0}=4 := by
  let e : {i : Fin 10 // i.val<5 ∧ i≠0} ≃
      {P : ReducedPair // P∈properSubgroup ∧ P≠0} :=
    pointEquiv.subtypeEquiv (fun i=>by
      change (i.val<5 ∧ i≠0) ↔ point i∈properSubgroup ∧ point i≠0
      rw [subgroup_membership,point_nonzero])
  exact (Nat.card_congr e).symm.trans (by rw [Nat.card_eq_fintype_card]; decide)

def ambientNonzeroFraction : ℚ :=
  (Nat.card {P : ReducedPair // P≠0 ∧ P.u.Splits} : ℚ)/
    Nat.card {P : ReducedPair // P≠0}

def subgroupNonzeroFraction (H : AddSubgroup ReducedPair) : ℚ :=
  (Nat.card {P : ReducedPair // P∈H ∧ P≠0 ∧ P.u.Splits} : ℚ)/
    Nat.card {P : ReducedPair // P∈H ∧ P≠0}

theorem exact_nonzero_fractions : subgroupNonzeroFraction properSubgroup=1 ∧
    ambientNonzeroFraction=7/9 ∧
    subgroupNonzeroFraction properSubgroup≠ambientNonzeroFraction := by
  simp only [subgroupNonzeroFraction,ambientNonzeroFraction,
    ambient_nonzero_smooth_cardinality,ambient_nonzero_cardinality,
    subgroup_nonzero_smooth_cardinality,subgroup_nonzero_cardinality]
  norm_num

theorem universal_nonzero_cyclic_fraction_equality_false :
    ¬ ∀ P : ReducedPair,
      subgroupNonzeroFraction (AddSubgroup.zmultiples P)=ambientNonzeroFraction := by
  intro h
  have hh := h (point 1)
  rw [←subgroup_cyclic] at hh
  exact exact_nonzero_fractions.2.2 hh

/-- Complete actual finite arithmetic counterexample. The geometric
Picard/Jacobian identification and qualified asymptotics remain separate. -/
theorem full_stoll_counterexample_certificate :
    curvePolynomial.Monic ∧ curvePolynomial.natDegree=5 ∧ curvePolynomial.Separable ∧
    Fintype.card ReducedPair=10 ∧ Fintype.card {P : ReducedPair // P.u.Splits}=8 ∧
    (point 1).u=X ∧ (point 1).v=1 ∧ Nat.Prime 5 ∧ (5:ℕ)≠3 ∧
    addOrderOf (point 1)=5 ∧ properSubgroup=AddSubgroup.zmultiples (point 1) ∧
    properSubgroup≠⊤ ∧ Fintype.card properSubgroup=5 ∧
    Fintype.card {P : properSubgroup // P.val.u.Splits}=5 ∧
    (∀ P Q : ReducedPair, SelectedRun P Q (P+Q)) ∧
    (∀ P Q : ReducedPair, P+Q=cantorAdd P Q) ∧
    subgroupFraction properSubgroup=1 ∧ ambientFraction=4/5 ∧
    subgroupFraction properSubgroup≠ambientFraction ∧
    (¬ ∀ P : ReducedPair, subgroupFraction (AddSubgroup.zmultiples P)=ambientFraction) ∧
    subgroupNonzeroFraction properSubgroup=1 ∧ ambientNonzeroFraction=7/9 ∧
    (¬ ∀ P : ReducedPair,
      subgroupNonzeroFraction (AddSubgroup.zmultiples P)=ambientNonzeroFraction) := by
  exact ⟨curve_monic,curve_degree,curve_separable,reducedPair_cardinality,
    smooth_reducedPair_cardinality,generator_coordinates.1,generator_coordinates.2,
    by decide,by decide,generator_order,subgroup_cyclic,subgroup_proper,subgroup_cardinality,
    subgroup_smooth_cardinality,addition_is_Cantor,addition_eq_cantorAdd,
    exact_fractions.1,exact_fractions.2.1,exact_fractions.2.2,
    universal_cyclic_fraction_equality_false,exact_nonzero_fractions.1,
    exact_nonzero_fractions.2.1,universal_nonzero_cyclic_fraction_equality_false⟩

#print axioms full_stoll_counterexample_certificate
end
end MumfordStollFullGroupAudit
