import GaloisAutomorphism

namespace WeilGaloisCovarianceAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit
open ThreeTorsionRankAudit.General

private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : DecidableEq BinaryClosure := Classical.decEq _
private instance : CharP BinaryClosure 2 := charP_of_injective_algebraMap' (ZMod 2) 2

def binaryFrobenius : BinaryClosure ≃+* BinaryClosure := frobeniusEquiv BinaryClosure 2

theorem binary_curve_frobenius : twistedCurve binaryCurve binaryFrobenius=binaryCurve := by
  apply WeierstrassCurve.ext <;> simp [twistedCurve,binaryCurve,WeierstrassCurve.map]

def binaryAction (P : torsionPoints binaryCurve 3) : torsionPoints binaryCurve 3 :=
  fixedCurveTorsion binaryCurve binaryFrobenius binary_curve_frobenius 3 P

/-- Actual Frobenius acts on the actual source quotient by squaring. -/
theorem binary_frobenius_value (P Q : torsionPoints binaryCurve 3) :
    geometricPairing binaryCurve binary_discriminant 3 (binaryAction P) (binaryAction Q)=
      geometricPairing binaryCurve binary_discriminant 3 P Q^2 := by
  calc
    _=Units.map binaryFrobenius.toMonoidHom
        (geometricPairing binaryCurve binary_discriminant 3 P Q) := by
      exact geometric_pairing_fixed_curve binaryCurve binary_discriminant
        binaryFrobenius binary_curve_frobenius 3 P Q
    _=_ := Units.ext rfl

/-- Nontrivial units cannot equal their own square. -/
theorem unit_square_ne_self {F : Type*} [Field F] (u : Fˣ) (h : u≠1) : u^2≠u := by
  intro he
  apply h
  apply mul_left_cancel (a := u)
  simpa only [pow_two,mul_one] using he

/-- Dropping the codomain action is false on an actual nonsingular binary curve. -/
theorem binary_point_only_invariance_counter :
    binaryCurve.Δ≠0 ∧
    ∃ P Q : torsionPoints binaryCurve 3,
      geometricPairing binaryCurve binary_discriminant 3 P Q≠1 ∧
      geometricPairing binaryCurve binary_discriminant 3 (binaryAction P) (binaryAction Q)≠
        geometricPairing binaryCurve binary_discriminant 3 P Q := by
  obtain ⟨hd,P,Q,hne⟩:=actual_binary_three_positive
  refine ⟨hd,P,Q,hne,?_⟩
  rw [binary_frobenius_value]
  exact unit_square_ne_self _ hne

/-- The same actual witness also supplies a coefficient that Frobenius really moves. -/
theorem actual_moved_coefficient :
    ∃ c : BinaryClosure,c≠0 ∧ binaryFrobenius c≠c := by
  obtain ⟨_,P,Q,hne,_⟩:=binary_point_only_invariance_counter
  let u:=geometricPairing binaryCurve binary_discriminant 3 P Q
  refine ⟨u.val,u.ne_zero,?_⟩
  intro he
  apply unit_square_ne_self u hne
  apply Units.ext
  exact he

def movedBinaryCurve (c : BinaryClosure) : WeierstrassCurve.Affine BinaryClosure := ⟨0,0,1,0,c⟩

theorem moved_binary_discriminant (c : BinaryClosure) : (movedBinaryCurve c).Δ≠0 := by
  have h4 : (4 : BinaryClosure)=0 := (CharP.cast_eq_zero_iff BinaryClosure 2 4).mpr (by decide)
  have he : (movedBinaryCurve c).Δ=-(27 : BinaryClosure) := by
    norm_num [movedBinaryCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
      WeierstrassCurve.b₆,WeierstrassCurve.b₈]
    rw [h4,zero_mul,add_zero,one_pow,mul_one]
  rw [he,neg_ne_zero]
  intro h
  have hd : 2 ∣ 27 := (CharP.cast_eq_zero_iff BinaryClosure 2 27).mp h
  norm_num at hd

/-- Omitting coefficient invariance can really change a nonsingular curve. -/
theorem binary_coefficient_invariance_guard_counter :
    ∃ W : WeierstrassCurve.Affine BinaryClosure,W.Δ≠0 ∧ twistedCurve W binaryFrobenius≠W := by
  obtain ⟨c,_,hc⟩:=actual_moved_coefficient
  refine ⟨movedBinaryCurve c,moved_binary_discriminant c,?_⟩
  intro he
  apply hc
  exact congrArg WeierstrassCurve.a₆ he

#print axioms binary_curve_frobenius
#print axioms binary_frobenius_value
#print axioms binary_point_only_invariance_counter
#print axioms actual_moved_coefficient
#print axioms moved_binary_discriminant
#print axioms binary_coefficient_invariance_guard_counter
end
end WeilGaloisCovarianceAudit
