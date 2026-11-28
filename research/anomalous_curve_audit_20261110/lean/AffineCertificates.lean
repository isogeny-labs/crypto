import CurveData

namespace AnomalousAffineCertificateAudit
variable {K : Type*} [Field K] [DecidableEq K]
variable {W : WeierstrassCurve.Affine K}
variable {x₁ x₂ x₃ y₁ y₂ y₃ ℓ : K}

/-- A supplied secant slope is checked by multiplication, then tied to the
actual Mathlib point group. No inverse computation or point-table axiom. -/
theorem secant_add (h₁ : W.Nonsingular x₁ y₁) (h₂ : W.Nonsingular x₂ y₂)
    (h₃ : W.Nonsingular x₃ y₃) (hx : x₁≠x₂)
    (hℓ : ℓ*(x₁-x₂)=y₁-y₂)
    (hx₃ : W.addX x₁ x₂ ℓ=x₃) (hy₃ : W.addY x₁ x₂ y₁ ℓ=y₃) :
    WeierstrassCurve.Affine.Point.some x₁ y₁ h₁ + .some x₂ y₂ h₂ = .some x₃ y₃ h₃ := by
  have hs : W.slope x₁ x₂ y₁ y₂=ℓ := by
    rw [WeierstrassCurve.Affine.slope_of_X_ne hx]
    exact ((eq_div_iff (sub_ne_zero.mpr hx)).mpr hℓ).symm
  rw [WeierstrassCurve.Affine.Point.add_of_X_ne hx,
    WeierstrassCurve.Affine.Point.some.injEq]
  exact ⟨by rwa [hs],by rwa [hs]⟩

/-- The analogous actual-point doubling certificate with a checked tangent. -/
theorem tangent_double (h₁ : W.Nonsingular x₁ y₁) (h₃ : W.Nonsingular x₃ y₃)
    (hy : y₁≠W.negY x₁ y₁)
    (hℓ : ℓ*(y₁-W.negY x₁ y₁)=3*x₁^2+2*W.a₂*x₁+W.a₄-W.a₁*y₁)
    (hx₃ : W.addX x₁ x₁ ℓ=x₃) (hy₃ : W.addY x₁ x₁ y₁ ℓ=y₃) :
    WeierstrassCurve.Affine.Point.some x₁ y₁ h₁ + .some x₁ y₁ h₁ = .some x₃ y₃ h₃ := by
  have hd : y₁-W.negY x₁ y₁≠0 := sub_ne_zero.mpr hy
  have hs : W.slope x₁ x₁ y₁ y₁=ℓ := by
    rw [WeierstrassCurve.Affine.slope_of_Y_ne rfl hy]
    exact ((eq_div_iff hd).mpr hℓ).symm
  rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne hy,
    WeierstrassCurve.Affine.Point.some.injEq]
  exact ⟨by rwa [hs],by rwa [hs]⟩

#print axioms secant_add
#print axioms tangent_double
end AnomalousAffineCertificateAudit
