import BKCertificate

/- Explicit-parameter names for the actual affine point maps in the frozen
   F₅ example. These definitions supply no mathematical assumptions. -/
namespace BKFullBindingAudit
noncomputable section
open BKActualThreeAudit

private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance galoisDecidableEq (k : ℕ) : DecidableEq (GaloisField 5 k) :=
  Classical.decEq _

noncomputable def rationalPointMap
    (L : Type*) [Field L] [Algebra F L] [DecidableEq L] :
    curve.Point →+ (curve.baseChange L).Point :=
  WeierstrassCurve.Affine.Point.map (W' := curve) (Algebra.ofId F L)

noncomputable def extensionPointMap
    (L : Type*) [Field L] [Algebra F L] [DecidableEq L]
    (K : Type*) [Field K] [Algebra F K] [Algebra K L]
    [IsScalarTower F K L] [DecidableEq K] :
    (curve.baseChange K).Point →+ (curve.baseChange L).Point :=
  WeierstrassCurve.Affine.Point.map (W' := curve) (Algebra.ofId K L)

noncomputable def galoisPointMap
    (L : Type*) [Field L] [Algebra F L] [DecidableEq L]
    (k : ℕ) (ι : GaloisField 5 k →ₐ[F] L) :
    (curve.baseChange (GaloisField 5 k)).Point →+ (curve.baseChange L).Point :=
  WeierstrassCurve.Affine.Point.map (W' := curve) ι

end
end BKFullBindingAudit
