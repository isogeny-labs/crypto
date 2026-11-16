import GaloisGeometric

namespace WeilGaloisCovarianceAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerBasechangeAudit WeilGeometricLawsAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

/-- Coefficients really move under a general field automorphism. -/
def twistedCurve (W : WeierstrassCurve.Affine K) (σ : K ≃+* K) : WeierstrassCurve.Affine K :=
  W.map σ.toRingHom

omit [DecidableEq K] [IsAlgClosed K] in
theorem twisted_discriminant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (σ : K ≃+* K) : (twistedCurve W σ).Δ≠0 := by
  rw [twistedCurve,WeierstrassCurve.map_Δ]
  exact fun h => hΔ (σ.injective (h.trans (map_zero σ).symm))

/-- Actual point map with the algebra induced by the specified automorphism. -/
def automorphismTorsion (W : WeierstrassCurve.Affine K) (σ : K ≃+* K) (n : ℤ) :
    torsionPoints W n →+ torsionPoints (twistedCurve W σ) n :=
  let _ := σ.toRingHom.toAlgebra
  torsionBasechange (L := K) W n

theorem geometric_pairing_automorphism (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (σ : K ≃+* K) (n : ℕ) (P Q : torsionPoints W (n : ℤ)) :
    geometricPairing (twistedCurve W σ) (twisted_discriminant W hΔ σ) (n : ℤ)
      (automorphismTorsion W σ (n : ℤ) P) (automorphismTorsion W σ (n : ℤ) Q)=
        Units.map σ.toMonoidHom (geometricPairing W hΔ (n : ℤ) P Q) := by
  let _ := σ.toRingHom.toAlgebra
  exact geometric_pairing_basechange (L := K) W hΔ n P Q

/-- Transport only across an actual equality of the coefficient models. -/
def curveTorsionTransport {W V : WeierstrassCurve.Affine K} (h : W=V) (n : ℤ)
    (P : torsionPoints W n) : torsionPoints V n := h ▸ P

theorem geometric_pairing_curve_transport {W V : WeierstrassCurve.Affine K} (h : W=V)
    (hW : W.Δ≠0) (hV : V.Δ≠0) (n : ℤ) (P Q : torsionPoints W n) :
    geometricPairing V hV n (curveTorsionTransport h n P) (curveTorsionTransport h n Q)=
      geometricPairing W hW n P Q := by
  cases h
  rfl

def fixedCurveTorsion (W : WeierstrassCurve.Affine K) (σ : K ≃+* K)
    (h : twistedCurve W σ=W) (n : ℤ) (P : torsionPoints W n) : torsionPoints W n :=
  curveTorsionTransport h n (automorphismTorsion W σ n P)

/-- Same-curve Galois covariance requires coefficient invariance and codomain action. -/
theorem geometric_pairing_fixed_curve (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (σ : K ≃+* K) (h : twistedCurve W σ=W) (n : ℕ) (P Q : torsionPoints W (n : ℤ)) :
    geometricPairing W hΔ (n : ℤ) (fixedCurveTorsion W σ h (n : ℤ) P)
      (fixedCurveTorsion W σ h (n : ℤ) Q)=
        Units.map σ.toMonoidHom (geometricPairing W hΔ (n : ℤ) P Q) := by
  unfold fixedCurveTorsion
  rw [geometric_pairing_curve_transport h (twisted_discriminant W hΔ σ) hΔ]
  exact geometric_pairing_automorphism W hΔ σ n P Q

#print axioms geometric_pairing_automorphism
#print axioms geometric_pairing_curve_transport
#print axioms geometric_pairing_fixed_curve
end
end WeilGaloisCovarianceAudit
