import RootEquiv
import Mathlib.SetTheory.Cardinal.NatCard

namespace TwoTorsionRankAudit
open Polynomial
noncomputable section
variable {K : Type*} [Field K] [DecidableEq K] [NeZero (2 : K)]

theorem four_ne_zero : (4 : K)≠0 := by
  have he : (4 : K)=2^2 := by norm_num
  rw [he]
  exact pow_ne_zero _ (NeZero.ne _)

theorem cubic_ne_zero (W : WeierstrassCurve.Affine K) :
    W.twoTorsionPolynomial.toPoly≠0 :=
  Cubic.ne_zero_of_a_ne_zero (P:=W.twoTorsionPolynomial) four_ne_zero

theorem root_mem_finset (W : WeierstrassCurve.Affine K) (x : K) :
    x∈W.twoTorsionPolynomial.toPoly.roots.toFinset ↔ W.twoTorsionPolynomial.toPoly.IsRoot x := by
  rw [Multiset.mem_toFinset]
  exact Polynomial.mem_roots (cubic_ne_zero W)

noncomputable instance rootFintype (W : WeierstrassCurve.Affine K) : Fintype (Root W) :=
  Fintype.ofFinset W.twoTorsionPolynomial.toPoly.roots.toFinset (root_mem_finset W)

theorem root_card_finset (W : WeierstrassCurve.Affine K) :
    Nat.card (Root W)=W.twoTorsionPolynomial.toPoly.roots.toFinset.card := by
  rw [Nat.card_eq_fintype_card]
  exact Fintype.card_ofFinset _ _

theorem root_card_le_three (W : WeierstrassCurve.Affine K) : Nat.card (Root W)≤3 := by
  rw [root_card_finset]
  exact Cubic.card_roots_le

theorem torsion_finite (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Finite (TwoTorsion W) :=
  Finite.of_equiv (Option (Root W)) (pointRootEquiv W hΔ).symm

theorem torsion_card_eq (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (TwoTorsion W)=Nat.card (Root W)+1 := by
  rw [Nat.card_congr (pointRootEquiv W hΔ),Finite.card_option]

theorem torsion_card_le_four (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (TwoTorsion W)≤4 := by
  rw [torsion_card_eq W hΔ]
  have h := root_card_le_three W
  omega

theorem cubic_map_identity (P : Cubic K) : Cubic.map (RingHom.id K) P=P := by
  cases P
  rfl

theorem root_card_three [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (Root W)=3 := by
  have hd : W.twoTorsionPolynomial.discr≠0 :=
    W.twoTorsionPolynomial_discr_ne_zero (isUnit_iff_ne_zero.mpr (NeZero.ne _))
      (isUnit_iff_ne_zero.mpr hΔ)
  have hs : (W.twoTorsionPolynomial.toPoly.map (RingHom.id K)).Splits := by
    simpa only [Polynomial.map_id] using IsAlgClosed.splits W.twoTorsionPolynomial.toPoly
  have hc := Cubic.card_roots_of_discr_ne_zero (P:=W.twoTorsionPolynomial)
    (φ:=RingHom.id K) (show W.twoTorsionPolynomial.a≠0 from four_ne_zero) hs hd
  rw [root_card_finset]
  simpa only [cubic_map_identity,Cubic.roots] using hc

theorem geometric_two_torsion_card [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) : Nat.card (TwoTorsion W)=4 := by
  rw [torsion_card_eq W hΔ,root_card_three W hΔ]

theorem scalar_map_two_nonzero {L : Type*} [Field L] (f : K →+* L) : NeZero (2 : L) := by
  constructor
  have h := f.injective.ne (NeZero.ne (2 : K))
  simpa only [map_ofNat,map_zero] using h

theorem geometric_scalar_extension_card {L : Type*} [Field L] [DecidableEq L] [IsAlgClosed L]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (f : K →+* L) :
    Nat.card (TwoTorsion (W.map f))=4 := by
  letI := scalar_map_two_nonzero f
  apply geometric_two_torsion_card
  rw [WeierstrassCurve.map_Δ]
  simpa only [map_zero] using f.injective.ne hΔ

#print axioms geometric_two_torsion_card
#print axioms geometric_scalar_extension_card
end
end TwoTorsionRankAudit
