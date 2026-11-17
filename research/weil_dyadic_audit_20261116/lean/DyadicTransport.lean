import TwoCertificate
import SurjectiveCertificate
import DyadicImports

namespace WeilDyadicAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

theorem geometric_pairing_transport {n m : ℤ} (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (he : n=m) (P Q : torsionPoints W n) (R S : torsionPoints W m)
    (hp : P.val=R.val) (hq : Q.val=S.val) :
    geometricPairing W hΔ n P Q=geometricPairing W hΔ m R S := by
  subst m
  have hP : P=R:=Subtype.ext hp
  have hQ : Q=S:=Subtype.ext hq
  rw [hP,hQ]

theorem actual_dyadic_zsmul_surjective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℕ) : Function.Surjective (fun P : W.Point=>(2 : ℤ)^n • P) := by
  intro Q
  obtain ⟨P,h⟩:=EllipticDoublingSurjectivityAudit.actual_dyadic_surjective W hΔ n Q
  refine ⟨P,?_⟩
  have he : (2 : ℤ)^n • P=(2^n : ℕ) • P := by
    simpa only [Nat.cast_pow,Nat.cast_ofNat] using natCast_zsmul P (2^n)
  exact he.trans h

def twoToDyadic (W : WeierstrassCurve.Affine K) (n : ℕ) (P : torsionPoints W 2) :
    torsionPoints W ((2 : ℤ)^(n+1)) :=
  ⟨P.val,by
    change (2 : ℤ)^(n+1) • P.val=0
    rw [pow_succ,mul_smul,P.property,smul_zero]⟩

omit [IsAlgClosed K] in
theorem two_to_dyadic_ne_zero (W : WeierstrassCurve.Affine K) (n : ℕ)
    (P : torsionPoints W 2) (hP : P≠0) : twoToDyadic W n P≠0 := by
  intro h
  have hv:=congrArg (fun X : torsionPoints W ((2 : ℤ)^(n+1))=>X.val) h
  exact hP (Subtype.ext hv)

#print axioms geometric_pairing_transport
#print axioms actual_dyadic_zsmul_surjective
#print axioms two_to_dyadic_ne_zero
end
end WeilDyadicAudit
