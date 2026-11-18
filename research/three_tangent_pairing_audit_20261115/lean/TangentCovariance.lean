import TangentPerfect

namespace ThreeTangentPairingAudit
noncomputable section
open WeierstrassCurve.Affine BKThreeNormalizationAudit ThreeTorsionRankAudit.General
variable {K L : Type*} [Field K] [Field L]

theorem tangentValue_map (f : K →+* L) (W : WeierstrassCurve.Affine K)
    (x y u v : K) :
    tangentValue (W.map f) (f x) (f y) (f u) (f v)=f (tangentValue W x y u v) := by
  simp [tangentValue,tangentSlope,numerator,denominator,WeierstrassCurve.map,map_ofNat]

variable {F A B : Type*} [Field F] [Field A] [Field B] [Algebra F A] [Algebra F B]

theorem tangentValue_algmap (f : A →ₐ[F] B) (W : WeierstrassCurve.Affine F)
    (x y u v : A) :
    tangentValue (W.baseChange B) (f x) (f y) (f u) (f v)=
      f (tangentValue (W.baseChange A) x y u v) := by
  have hm := tangentValue_map f.toRingHom (W.baseChange A) x y u v
  have hw : (W.baseChange A).map f.toRingHom=W.baseChange B := by
    ext <;> exact f.commutes _
  rw [hw] at hm
  exact hm

variable [DecidableEq A] [DecidableEq B]

theorem tangentPair_algmap (f : A →ₐ[F] B) (W : WeierstrassCurve.Affine F)
    (P Q : (W.baseChange A).Point) :
    tangentPair (W.baseChange B) (Point.map (W':=W) f P) (Point.map (W':=W) f Q)=
      f (tangentPair (W.baseChange A) P Q) := by
  cases P with
  | zero => cases Q <;> simp only [←Point.zero_def,map_zero,Point.map_some,tangentPair,map_one]
  | some x y hN =>
    cases Q with
    | zero => simp only [←Point.zero_def,map_zero,Point.map_some,tangentPair,map_one]
    | some u v hM =>
      rw [Point.map_some,Point.map_some]
      by_cases hx : x=u
      · simp [tangentPair,hx]
      · have hfx : f x≠f u := f.injective.ne hx
        simp only [tangentPair,ite_eq_right hx,ite_eq_right hfx,map_div₀,map_neg,
          tangentValue_algmap f W x y u v,tangentValue_algmap f W u v x y]

theorem tangentPair_frobenius [Fintype F] [DecidableEq F]
    (W : WeierstrassCurve.Affine F) (P Q : (W.baseChange A).Point) :
    tangentPair (W.baseChange A) (FrobeniusPointDescentAudit.pointFrobenius W P)
      (FrobeniusPointDescentAudit.pointFrobenius W Q)=
      (tangentPair (W.baseChange A) P Q)^Fintype.card F :=
  tangentPair_algmap (FiniteField.frobeniusAlgHom F A) W P Q

#print axioms tangentPair_algmap
#print axioms tangentPair_frobenius
end
end ThreeTangentPairingAudit
