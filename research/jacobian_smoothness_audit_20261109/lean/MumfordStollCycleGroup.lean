import MumfordStollCantor
import Mathlib.Algebra.Group.TransferInstance
import Mathlib.GroupTheory.SpecificGroups.Cyclic

/-! The five actual pairs form a cyclic arithmetic model, matched to all
Cantor additions within that model. The full ten-pair ambient group and
geometric Picard identification are separate obligations. -/
namespace MumfordStollCycleGroupAudit
noncomputable section
open Polynomial MumfordStollAudit MumfordStollCantorAudit
set_option backward.isDefEq.respectTransparency false
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩

/-- Bezout plus common-divisor certificates fix the normalized actual gcd. -/
theorem normalized_full_gcd (P Q : ReducedPair) (d a b c A B C : K[X])
    (hd : d.Monic) (hA : d*A=P.u) (hB : d*B=Q.u) (hC : d*C=P.v+Q.v)
    (hbez : a*P.u+b*Q.u+c*(P.v+Q.v)=d) :
    gcd (gcd P.u Q.u) (P.v+Q.v)=d := by
  let g := gcd (gcd P.u Q.u) (P.v+Q.v)
  have hgP : g∣P.u := dvd_trans (gcd_dvd_left _ _) (gcd_dvd_left _ _)
  have hgQ : g∣Q.u := dvd_trans (gcd_dvd_left _ _) (gcd_dvd_right _ _)
  have hgV : g∣P.v+Q.v := gcd_dvd_right _ _
  have hgd : g∣d := by
    rw [←hbez]
    exact dvd_add (dvd_add (dvd_mul_of_dvd_right hgP a)
      (dvd_mul_of_dvd_right hgQ b)) (dvd_mul_of_dvd_right hgV c)
  have hdg : d∣g := dvd_gcd (dvd_gcd ⟨A,hA.symm⟩ ⟨B,hB.symm⟩) ⟨C,hC.symm⟩
  exact (gcd_eq_normalize hgd hdg).trans hd.normalize_eq_self

theorem step_has_actual_full_gcd (P Q R : ReducedPair) (h : ExplicitCantorStep P Q R) :
    ∃ d U : K[X], d=gcd (gcd P.u Q.u) (P.v+Q.v) ∧
      d.Monic ∧ U.Monic ∧ d^2*U=P.u*Q.u := by
  obtain ⟨d,a,b,c,U,V,q,A,B,C,F,hd,hU,hA,hB,hC,hbez,hcomp,_⟩ := h
  exact ⟨d,U,(normalized_full_gcd P Q d a b c A B C hd hA hB hC hbez).symm,
    hd,hU,hcomp⟩

theorem reduction_matches_remainder (R : ReducedPair) (V : K[X]) (h : R.v=-V) :
    R.v=(-V)%ₘR.u := by
  rw [←h]
  exact ((Polynomial.modByMonic_eq_self_iff R.monic_u).mpr R.degree_v).symm

theorem cyclePoint_injective : Function.Injective cyclePoint := by
  intro i j h
  have he := point_injective h
  exact Fin.ext (congrArg (fun k : Fin 10=>k.val) he)

abbrev CyclePair := {P : ReducedPair // ∃ i : Fin 5, cyclePoint i=P}

def cycleMap (i : Fin 5) : CyclePair := ⟨cyclePoint i,i,rfl⟩

theorem cycleMap_bijective : Function.Bijective cycleMap := by
  constructor
  · intro i j h
    exact cyclePoint_injective (congrArg Subtype.val h)
  · intro P
    obtain ⟨i,hi⟩ := P.property
    exact ⟨i,Subtype.ext hi⟩

def cycleEquiv : CyclePair≃Fin 5 := (Equiv.ofBijective cycleMap cycleMap_bijective).symm

instance : AddCommGroup CyclePair := cycleEquiv.addCommGroup
instance : Fintype CyclePair := Fintype.ofEquiv _ cycleEquiv.symm

theorem cycleEquiv_map (i : Fin 5) : cycleEquiv (cycleMap i)=i :=
  Equiv.symm_apply_apply (Equiv.ofBijective cycleMap cycleMap_bijective) i

theorem cycleMap_add (i j : Fin 5) : cycleMap i+cycleMap j=cycleMap (i+j) := by
  apply cycleEquiv.injective
  change cycleEquiv.addEquiv (cycleMap i+cycleMap j)=cycleEquiv (cycleMap (i+j))
  rw [map_add]
  simp only [Equiv.addEquiv_apply,cycleEquiv_map]

theorem cycleMap_zero : cycleMap 0=0 := by
  apply cycleEquiv.injective
  change cycleEquiv (cycleMap 0)=cycleEquiv.addEquiv 0
  rw [map_zero,cycleEquiv_map]

theorem addition_is_Cantor (P Q : CyclePair) :
    ExplicitCantorStep P.val Q.val (P+Q).val := by
  obtain ⟨i,rfl⟩ := cycleMap_bijective.surjective P
  obtain ⟨j,rfl⟩ := cycleMap_bijective.surjective Q
  rw [cycleMap_add]
  exact cycle_all_pairs i j

theorem generator_order : addOrderOf (cycleMap 1)=5 := by
  apply addOrderOf_eq_prime
  · apply cycleEquiv.injective
    change cycleEquiv.addEquiv (5 • cycleMap 1)=cycleEquiv.addEquiv 0
    rw [map_nsmul,map_zero]
    simp only [Equiv.addEquiv_apply,cycleEquiv_map]
    decide
  · intro h
    have he := congrArg cycleEquiv (h.trans cycleMap_zero.symm)
    simp only [cycleEquiv_map] at he
    exact (by decide : (1:Fin 5)≠0) he

theorem all_natural_multiples (P : CyclePair) : ∃ n : ℕ, n • cycleMap 1=P := by
  obtain ⟨i,rfl⟩ := cycleMap_bijective.surjective P
  refine ⟨i.val,?_⟩
  apply cycleEquiv.injective
  change cycleEquiv.addEquiv (i.val • cycleMap 1)=cycleEquiv (cycleMap i)
  rw [map_nsmul]
  simp only [Equiv.addEquiv_apply,cycleEquiv_map]
  revert i
  decide

theorem generated_by_actual_point : AddSubgroup.zmultiples (cycleMap 1)=⊤ := by
  apply top_unique
  intro P _
  obtain ⟨n,rfl⟩ := all_natural_multiples P
  exact AddSubgroup.nsmul_mem _ (AddSubgroup.mem_zmultiples _) n

theorem generator_coordinates : (cycleMap 1).val.u=X ∧ (cycleMap 1).val.v=1 := by
  simp [cycleMap,cyclePoint,point,candidatePair,representative,uPolynomial,vPolynomial]

theorem cycle_cardinality : Fintype.card CyclePair=5 :=
  (Fintype.card_congr cycleEquiv).trans (by decide)

theorem every_cycle_pair_smooth (P : CyclePair) : P.val.u.Splits := by
  obtain ⟨i,rfl⟩ := cycleMap_bijective.surjective P
  exact (representative_splits ⟨i.val,by omega⟩).mpr (by change i.val<8; omega)

instance (P : CyclePair) : Decidable P.val.u.Splits := Classical.propDecidable _

theorem cycle_smooth_cardinality :
    Fintype.card {P : CyclePair // P.val.u.Splits}=5 := by
  rw [Fintype.card_congr (Equiv.subtypeUnivEquiv every_cycle_pair_smooth)]
  exact cycle_cardinality

theorem cycle_is_proper : ¬ ∀ P : ReducedPair, ∃ Q : CyclePair, Q.val=P := by
  intro h
  obtain ⟨Q,hQ⟩ := h (point 9)
  obtain ⟨i,hi⟩ := cycleMap_bijective.surjective Q
  have he := point_injective (congrArg Subtype.val hi |>.trans hQ)
  have hv := congrArg Fin.val he
  have hb := i.isLt
  change i.val=9 at hv
  omega

def ambientFraction : ℚ :=
  (Fintype.card {P : ReducedPair // P.u.Splits} : ℚ)/Fintype.card ReducedPair

def cycleFraction : ℚ :=
  (Fintype.card {P : CyclePair // P.val.u.Splits} : ℚ)/Fintype.card CyclePair

theorem exact_proportion_inequality : cycleFraction=1 ∧ ambientFraction=4/5 ∧
    cycleFraction≠ambientFraction := by
  have hc : cycleFraction=1 := by
    norm_num [cycleFraction,cycle_smooth_cardinality,cycle_cardinality]
  have ha : ambientFraction=4/5 := by
    norm_num [ambientFraction,smooth_reducedPair_cardinality,reducedPair_cardinality]
  exact ⟨hc,ha,by rw [hc,ha]; norm_num⟩

/-- A faithful finite arithmetic cycle with actual normalized gcd and
prime order different from the characteristic; no full ambient/Picard claim. -/
theorem stoll_cycle_audit_certificate :
    Nat.Prime 5 ∧ (5:ℕ)≠3 ∧ addOrderOf (cycleMap 1)=5 ∧
    (cycleMap 1).val.u=X ∧ (cycleMap 1).val.v=1 ∧
    Fintype.card ReducedPair=10 ∧ Fintype.card {P : ReducedPair // P.u.Splits}=8 ∧
    Fintype.card CyclePair=5 ∧ Fintype.card {P : CyclePair // P.val.u.Splits}=5 ∧
    (¬ ∀ P : ReducedPair, ∃ Q : CyclePair, Q.val=P) ∧
    (∀ P Q : CyclePair, ExplicitCantorStep P.val Q.val (P+Q).val) ∧
    (∀ P Q : CyclePair, ∃ d U : K[X],
      d=gcd (gcd P.val.u Q.val.u) (P.val.v+Q.val.v) ∧
      d.Monic ∧ U.Monic ∧ d^2*U=P.val.u*Q.val.u) ∧
    cycleFraction=1 ∧ ambientFraction=4/5 ∧ cycleFraction≠ambientFraction := by
  exact ⟨by decide,by decide,generator_order,generator_coordinates.1,generator_coordinates.2,
    reducedPair_cardinality,smooth_reducedPair_cardinality,cycle_cardinality,
    cycle_smooth_cardinality,cycle_is_proper,addition_is_Cantor,
    (fun P Q=>step_has_actual_full_gcd _ _ _ (addition_is_Cantor P Q)),
    exact_proportion_inequality⟩

#print axioms stoll_cycle_audit_certificate
#print axioms step_has_actual_full_gcd
end
end MumfordStollCycleGroupAudit
