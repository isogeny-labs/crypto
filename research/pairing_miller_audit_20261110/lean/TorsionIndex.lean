import MillerCurve

namespace PairingMillerTorsionAudit
open PairingMillerCurveAudit

theorem P_order : addOrderOf P=3 := addOrderOf_eq_prime P_three P_ne_zero
theorem Q_order : addOrderOf Q=3 := addOrderOf_eq_prime Q_three Q_ne_zero

theorem indexedPoint_add (u v : Index) :
    indexedPoint (u+v)=indexedPoint u+indexedPoint v := by
  have hp (n : ℕ) : (n%3) • P=n • P := by
    rw [←P_order]
    exact mod_addOrderOf_nsmul P n
  have hq (n : ℕ) : (n%3) • Q=n • Q := by
    rw [←Q_order]
    exact mod_addOrderOf_nsmul Q n
  unfold indexedPoint
  simp only [Prod.fst_add,Prod.snd_add,ZMod.val_add,hp,hq,add_nsmul]
  abel

def indexHom : Index→+curve.Point where
  toFun := indexedPoint
  map_zero' := by simp [indexedPoint]
  map_add' := indexedPoint_add

theorem indexedPoint_zero_iff : ∀ v : Index,indexedPoint v=0 ↔ v=0 := by decide +kernel

theorem indexedPoint_injective : Function.Injective indexedPoint := by
  exact (injective_iff_map_eq_zero indexHom).mpr
    (fun v hv=>(indexedPoint_zero_iff v).mp hv)

def coordinates : curve.Point→Option (F25×F25)
  | .zero => none
  | .some x y _ => some (x,y)

theorem coordinates_injective : Function.Injective coordinates := by
  intro U V h
  cases U <;> cases V <;> simp_all [coordinates]

theorem coordinate_coverage : ∀ x y : F25,
    y^2=x^3+1 → (x=0 ∨ x^3=1) →
    ∃ v : Index,coordinates (indexedPoint v)=some (x,y) := by decide +kernel

theorem all_three_torsion_indexed (R : curve.Point) (hR : (3:ℕ) • R=0) :
    ∃ v : Index,indexedPoint v=R := by
  cases R with
  | zero => exact ⟨0,by change indexedPoint 0=0;simp [indexedPoint]⟩
  | some x y h =>
    have he:=BKActualThreeAudit.actual_equation h
    have hx:=BKActualThreeAudit.three_coordinate_classification h hR
    obtain ⟨v,hv⟩:=coordinate_coverage x y he hx
    exact ⟨v,coordinates_injective hv⟩

def threeTorsion : AddSubgroup curve.Point :=
  (nsmulAddMonoidHom 3 : curve.Point→+curve.Point).ker

def torsionHom : Index→+threeTorsion := indexHom.codRestrict threeTorsion
  (fun v=>by change (3:ℕ) • indexedPoint v=0;exact indexedPoint_three v)

theorem torsionHom_bijective : Function.Bijective torsionHom := by
  constructor
  · intro u v h
    exact indexedPoint_injective (congrArg Subtype.val h)
  · intro R
    obtain ⟨v,hv⟩:=all_three_torsion_indexed R.val R.property
    exact ⟨v,Subtype.ext hv⟩

noncomputable def torsionEquiv : Index≃+threeTorsion :=
  AddEquiv.ofBijective torsionHom torsionHom_bijective

theorem actual_torsion_cardinality : Nat.card threeTorsion=9 := by
  rw [Nat.card_congr torsionEquiv.toEquiv.symm,Nat.card_eq_fintype_card]
  decide +kernel

#print axioms all_three_torsion_indexed
#print axioms actual_torsion_cardinality
end PairingMillerTorsionAudit
