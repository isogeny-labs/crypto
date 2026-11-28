import GaloisBinaryControl

namespace WeilGaloisCovarianceAudit
noncomputable section
open WeierstrassCurve.Affine MillerBasechangeAudit WeilGeometricLawsAudit

/-- Transparent bundles contain actual point/function maps, not assumed pairing laws. -/
def ActualFieldCovariance : Prop :=
  ∀ (K L : Type) [Field K] [Field L] [Algebra K L] [DecidableEq K] [DecidableEq L]
    [IsAlgClosed K] [IsAlgClosed L] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℕ) (P Q : torsionPoints W (n : ℤ)),
    geometricPairing (extendedCurve (L := L) W) (extended_delta_nonzero W hΔ) (n : ℤ)
      (torsionBasechange W (n : ℤ) P) (torsionBasechange W (n : ℤ) Q)=
        Units.map (algebraMap K L).toMonoidHom (geometricPairing W hΔ (n : ℤ) P Q)

def ActualAutomorphismCovariance : Prop :=
  ∀ (K : Type) [Field K] [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (σ : K ≃+* K)
    (n : ℕ) (P Q : torsionPoints W (n : ℤ)),
    geometricPairing (twistedCurve W σ) (twisted_discriminant W hΔ σ) (n : ℤ)
      (automorphismTorsion W σ (n : ℤ) P) (automorphismTorsion W σ (n : ℤ) Q)=
        Units.map σ.toMonoidHom (geometricPairing W hΔ (n : ℤ) P Q)

def ActualFixedCurveCovariance : Prop :=
  ∀ (K : Type) [Field K] [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (σ : K ≃+* K) (h : twistedCurve W σ=W)
    (n : ℕ) (P Q : torsionPoints W (n : ℤ)),
    geometricPairing W hΔ (n : ℤ) (fixedCurveTorsion W σ h (n : ℤ) P)
      (fixedCurveTorsion W σ h (n : ℤ) Q)=
        Units.map σ.toMonoidHom (geometricPairing W hΔ (n : ℤ) P Q)

private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : DecidableEq ThreeTorsionRankAudit.General.BinaryClosure:=Classical.decEq _

def ActualBinaryControls : Prop :=
  (∀ P Q : torsionPoints ThreeTorsionRankAudit.General.binaryCurve 3,
    geometricPairing _ ThreeTorsionRankAudit.General.binary_discriminant 3 (binaryAction P) (binaryAction Q)=
      geometricPairing _ ThreeTorsionRankAudit.General.binary_discriminant 3 P Q^2) ∧
  (ThreeTorsionRankAudit.General.binaryCurve.Δ≠0 ∧
    ∃ P Q : torsionPoints ThreeTorsionRankAudit.General.binaryCurve 3,
      geometricPairing _ ThreeTorsionRankAudit.General.binary_discriminant 3 P Q≠1 ∧
      geometricPairing _ ThreeTorsionRankAudit.General.binary_discriminant 3 (binaryAction P) (binaryAction Q)≠
        geometricPairing _ ThreeTorsionRankAudit.General.binary_discriminant 3 P Q) ∧
  (∃ W : WeierstrassCurve.Affine ThreeTorsionRankAudit.General.BinaryClosure,
    W.Δ≠0 ∧ twistedCurve W binaryFrobenius≠W)

theorem actual_geometric_pairing_covariance_certificate :
    ActualFieldCovariance ∧ ActualAutomorphismCovariance ∧
      ActualFixedCurveCovariance ∧ ActualBinaryControls := by
  refine ⟨?_,?_,?_,binary_frobenius_value,binary_point_only_invariance_counter,
    binary_coefficient_invariance_guard_counter⟩
  · intro K L _ _ _ _ _ _ _ W hΔ n P Q
    exact geometric_pairing_basechange W hΔ n P Q
  · intro K _ _ _ W hΔ σ n P Q
    exact geometric_pairing_automorphism W hΔ σ n P Q
  · intro K _ _ _ W hΔ σ h n P Q
    exact geometric_pairing_fixed_curve W hΔ σ h n P Q

#print axioms actual_geometric_pairing_covariance_certificate
end
end WeilGaloisCovarianceAudit
