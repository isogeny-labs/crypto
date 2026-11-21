import ActualTorsion
import Mathlib.Data.Fintype.BigOperators

namespace ThreeTorsionRankAudit
noncomputable section
open Polynomial
variable {K : Type*} [Field K] [DecidableEq K]

def affineTorsionCoordinates (a b : K) :=
  {xy : K×K // ∃ h : (shortCurve a b).Nonsingular xy.1 xy.2,
    (3:ℕ) • WeierstrassCurve.Affine.Point.some xy.1 xy.2 h=0}

abbrev rootPairs (a b : K) := (x : xRoots a b) × ↥(yRoots a b x)

theorem quartic_ne_zero (a b : K) (h3 : (3:K)≠0) : quartic a b≠0 := by
  intro hh
  have hd:=quartic_natDegree a b h3
  rw [hh,natDegree_zero] at hd
  omega

def toRootCoordinates (a b : K) (h2 : (2:K)≠0) (h3 : (3:K)≠0)
    (xy : affineTorsionCoordinates a b) : rootPairs a b := by
  let h:=xy.property.choose
  have ht:=xy.property.choose_spec
  have hx := (affine_three_torsion_iff a b h2 h).mp ht
  let x : xRoots a b := ⟨xy.val.1,(mem_rootSet_of_ne (quartic_ne_zero a b h3)).mpr
    (by simpa using hx)⟩
  have he : xy.val.2^2=cubicValue a b xy.val.1 := by
    have hh:=h.1
    simpa [WeierstrassCurve.Affine.equation_iff,shortCurve,cubicValue] using hh
  exact ⟨x,⟨xy.val.2,(mem_rootSet_of_ne (square_polynomial_ne_zero
    (cubicValue a b (x:K)))).mpr (by simpa [squarePolynomial,x] using sub_eq_zero.mpr he)⟩⟩

def fromRootCoordinates (a b : K) (hd : 4*a^3+27*b^2≠0) (h2 : (2:K)≠0)
    (z : rootPairs a b) : affineTorsionCoordinates a b :=
  ⟨⟨z.1,z.2⟩,root_pair_nonsingular a b hd h2 z.1 z.2,
    (affine_three_torsion_iff a b h2 (root_pair_nonsingular a b hd h2 z.1 z.2)).mpr
      (x_root_eval a b z.1)⟩

def affineRootEquiv (a b : K) (hd : 4*a^3+27*b^2≠0)
    (h2 : (2:K)≠0) (h3 : (3:K)≠0) :
    affineTorsionCoordinates a b ≃ rootPairs a b where
  toFun:=toRootCoordinates a b h2 h3
  invFun:=fromRootCoordinates a b hd h2
  left_inv:=by rintro ⟨⟨x,y⟩,h⟩;rfl
  right_inv:=by rintro ⟨⟨x,hx⟩,⟨y,hy⟩⟩;rfl

def geometricTorsion (a b : K) := {P : (shortCurve a b).Point // (3:ℕ) • P=0}

def geometricCoordinateEquiv (a b : K) (hd : 4*a^3+27*b^2≠0)
    (h2 : (2:K)≠0) (h3 : (3:K)≠0) :
    geometricTorsion a b ≃ Option (rootPairs a b) :=
  ((shortCurve a b).nonsingularPointEquivSubtype
    (p:=fun P => (3:ℕ) • P=0) (by exact nsmul_zero 3)).trans
      (Equiv.optionCongr (affineRootEquiv a b hd h2 h3))

theorem geometric_three_torsion_cardinality [IsAlgClosed K] (a b : K)
    (hd : 4*a^3+27*b^2≠0) (h2 : (2:K)≠0) (h3 : (3:K)≠0) :
    Nat.card (geometricTorsion a b)=9 := by
  classical
  rw [Nat.card_congr (geometricCoordinateEquiv a b hd h2 h3),Nat.card_eq_fintype_card,
    Fintype.card_option,Fintype.card_sigma]
  have hy : ∀ x : xRoots a b,Fintype.card (yRoots a b x)=2 := by
    intro x
    exact square_root_card (cubicValue a b x) h2
      (quartic_cubic_no_common_root a b hd x (x_root_eval a b x))
  simp_rw [hy]
  simp [xRoots,quartic_root_card a b hd h2 h3]

#print axioms geometric_three_torsion_cardinality
end
end ThreeTorsionRankAudit
