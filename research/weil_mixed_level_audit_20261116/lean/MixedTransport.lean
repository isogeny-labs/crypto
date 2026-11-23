import DyadicCertificate
import MixedImports

namespace WeilMixedLevelAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

def mixedLevel (n : ℕ) : ℤ := (2 : ℤ)^n*3

def threeToMixed (W : WeierstrassCurve.Affine K) (n : ℕ) (P : torsionPoints W 3) :
    torsionPoints W (mixedLevel n) :=
  ⟨P.val,by
    change ((2 : ℤ)^n*3) • P.val=0
    rw [mul_smul,P.property,smul_zero]⟩

def twoToMixed (W : WeierstrassCurve.Affine K) (n : ℕ) (P : torsionPoints W 2) :
    torsionPoints W (mixedLevel (n+1)) :=
  ⟨P.val,by
    change ((2 : ℤ)^(n+1)*3) • P.val=0
    have he : (2 : ℤ)^(n+1)*3=((2 : ℤ)^n*3)*2 := by rw [pow_succ];ring
    rw [he,mul_smul,P.property,smul_zero]⟩

omit [IsAlgClosed K] in
theorem three_to_mixed_ne_zero (W : WeierstrassCurve.Affine K) (n : ℕ)
    (P : torsionPoints W 3) (hp : P≠0) : threeToMixed W n P≠0 := by
  intro h
  exact hp (Subtype.ext (congrArg (fun X : torsionPoints W (mixedLevel n)=>X.val) h))

omit [IsAlgClosed K] in
theorem two_to_mixed_ne_zero (W : WeierstrassCurve.Affine K) (n : ℕ)
    (P : torsionPoints W 2) (hp : P≠0) : twoToMixed W n P≠0 := by
  intro h
  exact hp (Subtype.ext (congrArg (fun X : torsionPoints W (mixedLevel (n+1))=>X.val) h))

#print axioms three_to_mixed_ne_zero
#print axioms two_to_mixed_ne_zero
end
end WeilMixedLevelAudit
