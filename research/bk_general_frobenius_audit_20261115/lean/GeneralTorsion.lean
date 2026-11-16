import FrobeniusCertificate

namespace BKGeneralFrobeniusAudit
noncomputable section
variable {K : Type*} [Field K] [DecidableEq K]

/-- The actual subgroup of points killed by r; no rank hypothesis. -/
def torsion (W : WeierstrassCurve.Affine K) (r : ℕ) : AddSubgroup W.Point where
  carrier := {P | r • P = 0}
  zero_mem' := by simp
  add_mem' := by
    intro P Q hP hQ
    change r • P = 0 at hP
    change r • Q = 0 at hQ
    change r • (P + Q) = 0
    rw [nsmul_add, hP, hQ, zero_add]
  neg_mem' := by
    intro P hP
    change r • P = 0 at hP
    change r • (-P) = 0
    rw [smul_neg, hP, neg_zero]

abbrev Torsion (W : WeierstrassCurve.Affine K) (r : ℕ) := ↥(torsion W r)

theorem torsion_killed (W : WeierstrassCurve.Affine K) (r : ℕ) (P : Torsion W r) :
    r • P = 0 := by
  apply Subtype.ext
  exact P.property

instance torsionModule (W : WeierstrassCurve.Affine K) (r : ℕ) : Module (ZMod r) (Torsion W r) :=
  AddCommGroup.zmodModule (torsion_killed W r)

theorem torsion_one_subsingleton (W : WeierstrassCurve.Affine K) :
    Subsingleton (Torsion W 1) := by
  constructor
  intro P Q
  apply Subtype.ext
  have hp : (1 : ℕ) • P.val = 0 := P.property
  have hq : (1 : ℕ) • Q.val = 0 := Q.property
  have hP : P.val = 0 := by simpa only [one_nsmul] using hp
  have hQ : Q.val = 0 := by simpa only [one_nsmul] using hq
  exact hP.trans hQ.symm

def zeroTorsionEquiv (W : WeierstrassCurve.Affine K) : Torsion W 0 ≃+ W.Point where
  toFun := Subtype.val
  invFun P := ⟨P, by simp [torsion]⟩
  left_inv _ := rfl
  right_inv _ := rfl
  map_add' _ _ := rfl

section Maps
variable {F L B : Type*} [Field F] [Field L] [Field B]
  [Algebra F L] [Algebra F B] [DecidableEq L] [DecidableEq B]

def torsionMap (W : WeierstrassCurve.Affine F) (r : ℕ) (f : L →ₐ[F] B) :
    Torsion (W.baseChange L) r →+ Torsion (W.baseChange B) r where
  toFun P := ⟨WeierstrassCurve.Affine.Point.map (W' := W) f P.val, by
    change r • WeierstrassCurve.Affine.Point.map (W' := W) f P.val = 0
    have hp : r • P.val = 0 := P.property
    have h := congrArg (WeierstrassCurve.Affine.Point.map (W' := W) f) hp
    simpa only [map_nsmul, map_zero] using h⟩
  map_zero' := by apply Subtype.ext; exact map_zero _
  map_add' P Q := by apply Subtype.ext; exact map_add _ P.val Q.val

def linearTorsionMap (W : WeierstrassCurve.Affine F) (r : ℕ) (f : L →ₐ[F] B) :
    Torsion (W.baseChange L) r →ₗ[ZMod r] Torsion (W.baseChange B) r :=
  (torsionMap W r f).toZModLinearMap r

theorem torsionMap_injective (W : WeierstrassCurve.Affine F) (r : ℕ) (f : L →ₐ[F] B) :
    Function.Injective (linearTorsionMap W r f) := by
  intro P Q h
  apply Subtype.ext
  exact WeierstrassCurve.Affine.Point.map_injective (f := f) (congrArg Subtype.val h)

end Maps

#print axioms torsion_one_subsingleton
#print axioms zeroTorsionEquiv
#print axioms torsionMap_injective
end
end BKGeneralFrobeniusAudit
