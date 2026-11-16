import CubicCoordinates
import GeometricCardinality

namespace BKUnipotentAudit
noncomputable section
open FrobeniusPointDescentAudit
variable {L : Type*} [Field L] [Algebra F L] [DecidableEq L]

theorem frobenius_cube_fixed (P : (geomCurve (L:=L)).Point)
    (ht : (3:ℕ) • P=0) :
    pointFrobenius curve (pointFrobenius curve (pointFrobenius curve P))=P := by
  cases P with
  | zero => rfl
  | some x y h =>
    have hc := three_coordinate_classification h ht
    have hx : scalarFrob (scalarFrob (scalarFrob x))=x := by
      rcases hc with ⟨rfl,_⟩ | ⟨hc,_⟩
      · simp only [map_ofNat]
      · exact scalar_frob_cube_cubic hc
    have hy : scalarFrob (scalarFrob (scalarFrob y))=y := by
      rcases hc with ⟨_,rfl | rfl⟩ | ⟨hc,rfl | rfl⟩
      · simp only [map_ofNat]
      · simp only [map_neg,map_ofNat]
      · simp only [map_mul,map_ofNat]
        rw [scalar_frob_cube_cubic hc]
      · simp only [map_neg,map_mul,map_ofNat]
        rw [scalar_frob_cube_cubic hc]
    change WeierstrassCurve.Affine.Point.some
      (scalarFrob (scalarFrob (scalarFrob x)))
      (scalarFrob (scalarFrob (scalarFrob y))) _=
        WeierstrassCurve.Affine.Point.some x y h
    rw [WeierstrassCurve.Affine.Point.some.injEq]
    exact ⟨hx,hy⟩

abbrev K := TorsionBoundaryAudit.K
local instance closureDecidableEq : DecidableEq K := Classical.decEq _
abbrev geometricCurve := curve.baseChange K
abbrev sigma : geometricCurve.Point →+ geometricCurve.Point := pointFrobenius curve

theorem three_multiple_fixed (P : geometricCurve.Point) (ht : (3:ℕ) • P=0)
    (n : ℕ) : sigma^[3*n] P=P := by
  have h3 : sigma^[3] P=P := by
    simpa only [Function.iterate_succ_apply,Function.iterate_zero_apply] using
      frobenius_cube_fixed P ht
  induction n with
  | zero => rfl
  | succ n ih =>
    rw [Nat.mul_succ,Function.iterate_add_apply,h3]
    exact ih

theorem nonfixed_witness : ∃ P : geometricCurve.Point, (3:ℕ) • P=0 ∧ sigma P≠P := by
  obtain ⟨_,_,_,_,_,P,ht,hn⟩ := bk_nonfixed_control
  exact ⟨P,ht,hn⟩

theorem all_torsion_fixed_iff_three_dvd (k : ℕ) :
    (∀ P : geometricCurve.Point, (3:ℕ) • P=0 → sigma^[k] P=P) ↔ 3 ∣ k := by
  constructor
  · intro h
    obtain ⟨P,ht,hn⟩ := nonfixed_witness
    have hp := h P ht
    have hk : k=k%3+3*(k/3) := by omega
    rw [hk,Function.iterate_add_apply,three_multiple_fixed P ht] at hp
    have hb : k%3<3 := Nat.mod_lt _ (by decide)
    have hm : k%3=0 := by
      rcases (show k%3=0 ∨ k%3=1 ∨ k%3=2 by omega) with h0 | h1 | h2
      · exact h0
      · have hh : sigma P=P := by
          simpa only [h1,Function.iterate_one] using hp
        exact (hn hh).elim
      · have hh : sigma (sigma P)=P := by
          simpa only [h2,Function.iterate_succ_apply,Function.iterate_zero_apply] using hp
        have h3 := frobenius_cube_fixed P ht
        change sigma (sigma (sigma P))=P at h3
        rw [hh] at h3
        exact (hn h3).elim
    exact Nat.dvd_of_mod_eq_zero hm
  · rintro ⟨n,rfl⟩ P ht
    exact three_multiple_fixed P ht n

theorem cyclotomic_divisibility (k : ℕ) : 3 ∣ 7^k-1 := by
  have h : Nat.ModEq 3 1 (7^k) := by
    simpa using (Nat.ModEq.pow k (by decide : Nat.ModEq 3 1 7))
  exact (Nat.modEq_iff_dvd' (Nat.one_le_pow k 7 (by decide))).mp h

theorem embedding_degree_one : TorsionBoundaryAudit.embeddingDegree 7 3=1 := by
  change orderOf (7:ZMod 3)=1
  rw [show (7:ZMod 3)=1 by decide +kernel,orderOf_one]

theorem actual_geometric_cardinality : Nat.card {P : geometricCurve.Point // (3:ℕ) • P=0}=9 := by
  have he : geometricCurve=ThreeTorsionRankAudit.shortCurve (3:K) 2 := by
    ext <;> simp [geometricCurve,curve,TorsionBoundaryAudit.bkCurve,
      ThreeTorsionRankAudit.shortCurve,WeierstrassCurve.baseChange,
      WeierstrassCurve.map,map_ofNat]
  rw [he]
  change Nat.card (ThreeTorsionRankAudit.geometricTorsion (3:K) 2)=9
  apply ThreeTorsionRankAudit.geometric_three_torsion_cardinality
  · have hd : (4*(3:F)^3+27*2^2)≠0 := by decide +kernel
    simpa only [map_add,map_mul,map_pow,map_ofNat] using
      (map_ne_zero (algebraMap F K)).mpr hd
  · exact two_ne_zero
  · exact three_ne_zero

#print axioms frobenius_cube_fixed
#print axioms all_torsion_fixed_iff_three_dvd
#print axioms actual_geometric_cardinality

end
end BKUnipotentAudit
