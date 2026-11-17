import PowerTargets
import RationalTorsion

namespace CharPPairingDomainAudit
noncomputable section

/-- Equality of actual cokernel and kernel sizes for a finite group endomorphism. -/
theorem multiple_quotient_card {G : Type*} [AddCommGroup G] [Finite G] (n : ℕ) :
    Nat.card (G ⧸ (nsmulAddMonoidHom n : G →+ G).range)=
      Nat.card (nsmulAddMonoidHom n : G →+ G).ker := by
  exact AddSubgroup.index_range

variable {K : Type*} [Field K] [DecidableEq K]

def actualLeft (W : WeierstrassCurve.Affine K) (n : ℕ) : AddSubgroup W.Point :=
  (nsmulAddMonoidHom n : W.Point →+ W.Point).ker

abbrev ActualRight (W : WeierstrassCurve.Affine K) (n : ℕ) :=
  W.Point ⧸ (nsmulAddMonoidHom n : W.Point →+ W.Point).range

theorem actual_quotient_card [Finite K] (W : WeierstrassCurve.Affine K) (n : ℕ) :
    Nat.card (ActualRight W n)=Nat.card (actualLeft W n) := by
  have : Finite W.Point := BKLinearAudit.actual_rational_point_finite W
  exact multiple_quotient_card n

theorem actual_left_killed (W : WeierstrassCurve.Affine K) (n : ℕ)
    (P : actualLeft W n) : n • P=0 := by
  apply Subtype.ext
  exact P.property

def UnitPairingExists (W : WeierstrassCurve.Affine K) (n : ℕ)
    (L : Type*) [Field L] : Prop :=
  ∃ e : actualLeft W n → actualLeft W n → Lˣ,
    LeftBilinear e ∧ LeftNondegenerate e

def TatePairingExists (W : WeierstrassCurve.Affine K) (n : ℕ) : Prop :=
  ∃ e : actualLeft W n → ActualRight W n → PowerTarget K n,
    LeftBilinear e ∧ LeftNondegenerate e

theorem nonzero_actual_left (W : WeierstrassCurve.Affine K) (n : ℕ)
    (P : W.Point) (hP : P≠0) (hk : n • P=0) :
    (⟨P,hk⟩ : actualLeft W n)≠0 := by
  intro h
  exact hP (congrArg Subtype.val h)

theorem no_unit_pairing_of_actual_point (W : WeierstrassCurve.Affine K)
    (p k : ℕ) [Fact p.Prime] (P : W.Point) (hP : P≠0) (hk : (p^k) • P=0)
    (L : Type*) [Field L] [CharP L p] : ¬UnitPairingExists W (p^k) L := by
  exact characteristic_left_nondegenerate_impossible p k
    (actual_left_killed W (p^k)) ⟨P,hk⟩ (nonzero_actual_left W (p^k) P hP hk)

theorem no_tate_pairing_of_actual_point [Finite K] (W : WeierstrassCurve.Affine K)
    (p k : ℕ) [Fact p.Prime] [CharP K p]
    (P : W.Point) (hP : P≠0) (hk : (p^k) • P=0) :
    ¬TatePairingExists W (p^k) := by
  rintro ⟨e,_,hn⟩
  apply nonzero_actual_left W (p^k) P hP hk
  exact hn ⟨P,hk⟩ (fun Q => finite_power_target_eq_one p k (e ⟨P,hk⟩ Q))

#print axioms multiple_quotient_card
#print axioms actual_quotient_card
#print axioms no_unit_pairing_of_actual_point
#print axioms no_tate_pairing_of_actual_point
end
end CharPPairingDomainAudit
