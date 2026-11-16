import LawsCharacteristic

namespace WeilGeometricLawsAudit
noncomputable section
open WeierstrassCurve.Affine
open WeilDivisorChoiceAudit MillerBasechangeAudit
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev FiveBase:=ZMod 5
noncomputable abbrev fiveBaseCommRing : CommRing FiveBase := (inferInstance : Field FiveBase).toCommRing
attribute [local instance] fiveBaseCommRing

/-- Credited ordinary F5 model, constructed here without conflicting S3 module imports. -/
def fiveBaseCurve : WeierstrassCurve.Affine FiveBase := ⟨0,0,0,3,2⟩
theorem five_base_delta : fiveBaseCurve.Δ≠0 := by decide +kernel
theorem five_base_nonsingular : fiveBaseCurve.Nonsingular 1 1 := by
  apply (fiveBaseCurve.equation_iff_nonsingular_of_Δ_ne_zero five_base_delta).mp
  rw [equation_iff]
  decide +kernel

def fiveBasePoint : fiveBaseCurve.Point := Point.some 1 1 five_base_nonsingular
theorem five_base_point_nonzero : fiveBasePoint≠0 := Point.some_ne_zero _
theorem five_base_point_killed : (5 : ℕ) • fiveBasePoint=0 := by decide +kernel

abbrev FiveClosure:=AlgebraicClosure FiveBase
private instance : DecidableEq FiveClosure:=Classical.decEq _
private instance : CharP FiveClosure 5:=charP_of_injective_algebraMap' FiveBase 5
abbrev fiveGeometricCurve : WeierstrassCurve.Affine FiveClosure := fiveBaseCurve.baseChange FiveClosure

def fiveGeometricMap : fiveBaseCurve.Point →+ fiveGeometricCurve.Point :=
  Point.map (W':=fiveBaseCurve) (Algebra.ofId FiveBase FiveClosure)
def fiveGeometricPoint : fiveGeometricCurve.Point := fiveGeometricMap fiveBasePoint

theorem five_geometric_delta : fiveGeometricCurve.Δ≠0 :=
  extended_delta_nonzero fiveBaseCurve five_base_delta

theorem five_geometric_point_nonzero : fiveGeometricPoint≠0 := by
  intro he
  apply five_base_point_nonzero
  apply Point.map_injective (W':=fiveBaseCurve) (Algebra.ofId FiveBase FiveClosure)
  exact he.trans (map_zero fiveGeometricMap).symm

theorem five_geometric_point_killed : (5 : ℕ) • fiveGeometricPoint=0 := by
  simpa only [fiveGeometricPoint,map_nsmul,map_zero] using congrArg fiveGeometricMap five_base_point_killed

theorem five_geometric_point_order : addOrderOf fiveGeometricPoint=5 := by
  have hd : addOrderOf fiveGeometricPoint ∣ 5 := addOrderOf_dvd_iff_nsmul_eq_zero.mpr five_geometric_point_killed
  rcases (show Nat.Prime 5 by decide).eq_one_or_self_of_dvd _ hd with he|he
  · exact False.elim (five_geometric_point_nonzero (AddMonoid.addOrderOf_eq_one_iff.mp he))
  · exact he

def actualFiveTorsionPoint : torsionPoints fiveGeometricCurve 5 :=
  ⟨fiveGeometricPoint,by
    change (5 : ℤ) • fiveGeometricPoint=0
    exact (natCast_zsmul fiveGeometricPoint 5).trans five_geometric_point_killed⟩

/-- The actual source quotient collapses on this actual geometric curve despite nonzero five-torsion. -/
theorem actual_five_pairing_trivial (P Q : torsionPoints fiveGeometricCurve 5) :
    geometricPairing fiveGeometricCurve five_geometric_delta 5 P Q=1 :=
  geometric_pairing_characteristic_trivial 5 fiveGeometricCurve five_geometric_delta P Q

/-- Literal unqualified root/nondegeneracy clauses of Miller1986 Definition13 cannot coexist here. -/
theorem miller_1986_unqualified_five_counter :
    fiveGeometricCurve.Δ≠0 ∧ fiveGeometricPoint≠0 ∧ addOrderOf fiveGeometricPoint=5 ∧
    ¬∃ e : fiveGeometricCurve.Point→fiveGeometricCurve.Point→FiveClosureˣ,
      (∀ P Q, (5 : ℕ) • P=0→(5 : ℕ) • Q=0→e P Q^5=1) ∧
      (∀ P,addOrderOf P=5→∃ Q,addOrderOf Q=5 ∧ e P Q≠1) := by
  refine ⟨five_geometric_delta,five_geometric_point_nonzero,five_geometric_point_order,?_⟩
  rintro ⟨e,hr,hn⟩
  obtain ⟨Q,hQ,hne⟩:=hn fiveGeometricPoint five_geometric_point_order
  have hQt : (5 : ℕ) • Q=0 := addOrderOf_dvd_iff_nsmul_eq_zero.mp (by rw [hQ])
  exact hne (characteristic_root_one 5 _ (hr _ _ five_geometric_point_killed hQt))

/-- The exact-order wording also needs an N>1 exception at level1. -/
theorem miller_1986_level_one_counter :
    ¬∃ e : fiveGeometricCurve.Point→fiveGeometricCurve.Point→FiveClosureˣ,
      (∀ P Q,(1 : ℕ) • P=0→(1 : ℕ) • Q=0→e P Q^1=1) ∧
      (∀ P,addOrderOf P=1→∃ Q,addOrderOf Q=1 ∧ e P Q≠1) := by
  rintro ⟨e,hr,hn⟩
  obtain ⟨Q,hQ,hne⟩:=hn 0 (AddMonoid.addOrderOf_eq_one_iff.mpr rfl)
  have hQt : (1 : ℕ) • Q=0 := addOrderOf_dvd_iff_nsmul_eq_zero.mp (by rw [hQ])
  have he:=hr 0 Q (by simp) hQt
  exact hne (by simpa only [pow_one] using he)

#print axioms five_base_point_killed
#print axioms five_geometric_point_order
#print axioms actual_five_pairing_trivial
#print axioms miller_1986_unqualified_five_counter
#print axioms miller_1986_level_one_counter
end
end WeilGeometricLawsAudit
