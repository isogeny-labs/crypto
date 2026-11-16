module
public import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
public import Mathlib.GroupTheory.QuotientGroup.Basic
public import Mathlib.FieldTheory.Finite.Basic
public import Mathlib.Tactic

@[expose] public section
namespace CompositeTateDomainAudit
noncomputable section
open WeierstrassCurve.Affine
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev F := ZMod 5
local instance : CommRing F := (inferInstance : Field F).toCommRing

def curve : WeierstrassCurve.Affine F := ⟨0, 0, 0, 0, 1⟩
abbrev G := curve.Point

theorem curve_delta : curve.Δ ≠ 0 := by decide +kernel

def arithmeticEquiv : {xy : F × F // curve.Nonsingular xy.1 xy.2} ≃
    {xy : F × F // xy.2 ^ 2 = xy.1 ^ 3 + 1} :=
  Equiv.subtypeEquivProp (by
    funext xy
    apply propext
    rw [← WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_delta]
    simp [curve, WeierstrassCurve.Affine.equation_iff])

theorem curve_count : Nat.card G = 6 := by
  rw [Nat.card_congr curve.nonsingularPointEquiv]
  change Nat.card (Option {xy : F × F // curve.Nonsingular xy.1 xy.2}) = 6
  rw [Nat.card_congr arithmeticEquiv.optionCongr, Nat.card_eq_fintype_card]
  decide +kernel

instance pointFinite : Finite G :=
  Nat.finite_of_card_ne_zero (by rw [curve_count]; decide)

def threePoint : G := .some 0 1
  ((WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero curve_delta).mp (by
    rw [WeierstrassCurve.Affine.equation_iff]
    decide +kernel))

theorem three_point_nonzero : threePoint ≠ 0 := Point.some_ne_zero _
theorem three_point_killed : (3 : ℕ) • threePoint = 0 := by decide +kernel

theorem three_point_order : addOrderOf threePoint = 3 := by
  have hd : addOrderOf threePoint ∣ 3 := addOrderOf_dvd_iff_nsmul_eq_zero.mpr three_point_killed
  rcases (show Nat.Prime 3 by decide).eq_one_or_self_of_dvd _ hd with h | h
  · exact False.elim (three_point_nonzero (AddMonoid.addOrderOf_eq_one_iff.mp h))
  · exact h

theorem all_points_six (P : G) : (6 : ℕ) • P = 0 := by
  simpa only [curve_count] using (card_nsmul_eq_zero' (x := P))

def timesSix : G →+ G where
  toFun := fun P => (6 : ℕ) • P
  map_zero' := nsmul_zero _
  map_add' := fun P Q => nsmul_add P Q 6

def sixTorsion : AddSubgroup G := timesSix.ker
def sixMultiples : AddSubgroup G := timesSix.range
abbrev LeftDomain := Multiplicative sixTorsion
abbrev RightDomain := Multiplicative (G ⧸ sixMultiples)

theorem mem_six_torsion (P : G) : P ∈ sixTorsion ↔ (6 : ℕ) • P = 0 := by
  rfl

theorem mem_six_multiples (P : G) : P ∈ sixMultiples ↔ ∃ Q : G, (6 : ℕ) • Q = P := by
  rfl

def torsionEquiv : G ≃ sixTorsion where
  toFun P := ⟨P, (mem_six_torsion P).mpr (all_points_six P)⟩
  invFun P := P.val
  left_inv _ := rfl
  right_inv _ := rfl

theorem left_domain_card : Nat.card LeftDomain = 6 := by
  change Nat.card sixTorsion = 6
  rw [Nat.card_congr torsionEquiv.symm, curve_count]

theorem six_multiples_bot : sixMultiples = ⊥ := by
  ext P
  rw [mem_six_multiples, AddSubgroup.mem_bot]
  constructor
  · rintro ⟨Q, hQ⟩
    exact hQ.symm.trans (all_points_six Q)
  · intro h
    exact ⟨0, by simpa using h.symm⟩

def leftPoint : LeftDomain :=
  Multiplicative.ofAdd ⟨threePoint, (mem_six_torsion threePoint).mpr (all_points_six threePoint)⟩
def rightPoint : RightDomain := Multiplicative.ofAdd (QuotientAddGroup.mk' sixMultiples threePoint)

theorem right_domain_card : Nat.card RightDomain = 6 := by
  change Nat.card (G ⧸ sixMultiples) = 6
  rw [six_multiples_bot]
  exact (Nat.card_congr (QuotientAddGroup.quotientBot (G := G)).toEquiv).trans curve_count

theorem left_point_nontrivial : leftPoint ≠ 1 := by
  intro h
  exact three_point_nonzero (congrArg (fun P : LeftDomain => (P.toAdd : G)) h)

theorem left_point_cubed : leftPoint ^ 3 = 1 := by
  change (3 : ℕ) • (⟨threePoint, (mem_six_torsion threePoint).mpr
    (all_points_six threePoint)⟩ : sixTorsion) = 0
  apply Subtype.ext
  exact three_point_killed

theorem right_point_nontrivial : rightPoint ≠ 1 := by
  intro h
  have hm : threePoint ∈ sixMultiples :=
    (QuotientAddGroup.eq_zero_iff threePoint).mp (congrArg Multiplicative.toAdd h)
  rw [six_multiples_bot] at hm
  exact three_point_nonzero hm

theorem right_point_cubed : rightPoint ^ 3 = 1 := by
  change (3 : ℕ) • (QuotientAddGroup.mk' sixMultiples threePoint) = 0
  simpa only [map_nsmul, map_zero] using
    congrArg (QuotientAddGroup.mk' sixMultiples) three_point_killed

#print axioms curve_count
#print axioms three_point_order
#print axioms left_domain_card
#print axioms left_point_nontrivial
#print axioms right_point_nontrivial
end
end CompositeTateDomainAudit
