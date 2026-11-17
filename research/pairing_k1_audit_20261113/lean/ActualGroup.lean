import TorsionBoundaries
import Mathlib.GroupTheory.QuotientGroup.Basic

namespace PairingKOneAudit
private instance primeSeven : Fact (Nat.Prime 7) := ⟨by decide⟩
private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩
abbrev F := ZMod 7
abbrev curve := TorsionBoundaryAudit.bkCurve

def G : curve.Point := .some 0 3 (by
  rw [WeierstrassCurve.Affine.nonsingular_iff]
  constructor
  · rw [WeierstrassCurve.Affine.equation_iff]; decide +kernel
  · decide +kernel)

theorem G_nine : (9:ℕ) • G=0 := by decide +kernel
theorem G_three_ne_zero : (3:ℕ) • G≠0 := by decide +kernel

theorem G_order : addOrderOf G=9 := by
  exact addOrderOf_eq_prime_pow (p:=3) (n:=1) G_three_ne_zero G_nine

def indexedPoint (i : ZMod 9) : curve.Point := i.val • G

theorem indexedPoint_add (i j : ZMod 9) :
    indexedPoint (i+j)=indexedPoint i+indexedPoint j := by
  have hp (n : ℕ) : (n%9) • G=n • G := by
    rw [←G_order]
    exact mod_addOrderOf_nsmul G n
  unfold indexedPoint
  rw [ZMod.val_add,hp,add_nsmul]

def indexHom : ZMod 9 →+ curve.Point where
  toFun := indexedPoint
  map_zero' := by simp [indexedPoint]
  map_add' := indexedPoint_add

theorem indexedPoint_zero_iff : ∀ i : ZMod 9,indexedPoint i=0 ↔ i=0 := by
  decide +kernel

theorem indexedPoint_injective : Function.Injective indexedPoint := by
  exact (injective_iff_map_eq_zero indexHom).mpr
    (fun i hi => (indexedPoint_zero_iff i).mp hi)

def coordinates : curve.Point → Option (F×F)
  | .zero => none
  | .some x y _ => some (x,y)

theorem coordinates_injective : Function.Injective coordinates := by
  intro U V h
  cases U <;> cases V <;> simp_all [coordinates]

theorem coordinate_coverage : ∀ x y : F,
    y^2=x^3+3*x+2 → ∃ i : ZMod 9,coordinates (indexedPoint i)=some (x,y) := by
  decide +kernel

theorem actual_equation {x y : F} (h : curve.Nonsingular x y) : y^2=x^3+3*x+2 := by
  simpa [curve,TorsionBoundaryAudit.bkCurve,WeierstrassCurve.Affine.equation_iff] using h.1

theorem indexHom_surjective : Function.Surjective indexHom := by
  intro R
  cases R with
  | zero => exact ⟨0,indexHom.map_zero⟩
  | some x y h =>
    obtain ⟨i,hi⟩ := coordinate_coverage x y (actual_equation h)
    exact ⟨i,coordinates_injective hi⟩

noncomputable def indexEquiv : ZMod 9 ≃+ curve.Point :=
  AddEquiv.ofBijective indexHom ⟨indexedPoint_injective,indexHom_surjective⟩

def threeTorsion : AddSubgroup curve.Point :=
  (nsmulAddMonoidHom 3 : curve.Point →+ curve.Point).ker

def torsionPoint (i : ZMod 3) : curve.Point := i.val • ((3:ℕ) • G)

theorem three_G_three : (3:ℕ) • ((3:ℕ) • G)=0 := by
  rw [smul_smul]
  exact G_nine

theorem torsionPoint_three (i : ZMod 3) : (3:ℕ) • torsionPoint i=0 := by
  unfold torsionPoint
  rw [smul_comm 3 i.val,three_G_three,nsmul_zero]

theorem torsionPoint_add (i j : ZMod 3) :
    torsionPoint (i+j)=torsionPoint i+torsionPoint j := by
  have ho : addOrderOf ((3:ℕ) • G)=3 := addOrderOf_eq_prime three_G_three G_three_ne_zero
  have hp (n : ℕ) : (n%3) • ((3:ℕ) • G)=n • ((3:ℕ) • G) := by
    simpa only [ho] using mod_addOrderOf_nsmul ((3:ℕ) • G) n
  unfold torsionPoint
  rw [ZMod.val_add,hp,add_nsmul]

def torsionIndex : ZMod 3 →+ curve.Point where
  toFun := torsionPoint
  map_zero' := by simp [torsionPoint]
  map_add' := torsionPoint_add

def torsionHom : ZMod 3 →+ threeTorsion := torsionIndex.codRestrict threeTorsion
  (fun i => by change (3:ℕ) • torsionPoint i=0; exact torsionPoint_three i)

theorem torsionPoint_zero_iff : ∀ i : ZMod 3,torsionPoint i=0 ↔ i=0 := by
  decide +kernel

theorem indexed_torsion_coverage : ∀ j : ZMod 9,
    (3:ℕ) • indexedPoint j=0 → ∃ i : ZMod 3,torsionPoint i=indexedPoint j := by
  decide +kernel

theorem torsionHom_bijective : Function.Bijective torsionHom := by
  constructor
  · apply (injective_iff_map_eq_zero torsionHom).mpr
    intro i hi
    exact (torsionPoint_zero_iff i).mp (congrArg Subtype.val hi)
  · intro U
    obtain ⟨j,hj⟩ := indexHom_surjective U.val
    obtain ⟨i,hi⟩ := indexed_torsion_coverage j (by rw [show indexedPoint j=U.val from hj]; exact U.property)
    exact ⟨i,Subtype.ext (hi.trans hj)⟩

noncomputable def torsionEquiv : ZMod 3 ≃+ threeTorsion :=
  AddEquiv.ofBijective torsionHom torsionHom_bijective

theorem actual_rational_torsion_cardinality : Nat.card threeTorsion=3 := by
  rw [Nat.card_congr torsionEquiv.toEquiv.symm,Nat.card_eq_fintype_card]
  exact ZMod.card 3

theorem torsionHom_one_ne_zero : torsionHom 1≠0 := by
  intro h
  have hi := torsionHom_bijective.injective (h.trans torsionHom.map_zero.symm)
  exact (show (1:ZMod 3)≠0 by decide) hi

#print axioms G_order
#print axioms indexHom_surjective
#print axioms actual_rational_torsion_cardinality
end PairingKOneAudit
