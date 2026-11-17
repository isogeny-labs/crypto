import ScalingGroup

namespace BKBasefieldScalingAudit
noncomputable section
open ThreeTorsionRankAudit
variable {K : Type*} [Field K] [DecidableEq K]

def killedPoints (a b : K) (n : ℕ) := {P : (shortCurve a b).Point // n • P=0}

def scalingTorsionEquiv (a b u : K) (hu : u≠0) (n : ℕ) :
    killedPoints a b n ≃ killedPoints (u^4*a) (u^6*b) n where
  toFun P:=⟨scalingEquiv a b u hu P.val,by
    rw [← map_nsmul,P.property,map_zero]⟩
  invFun P:=⟨(scalingEquiv a b u hu).symm P.val,by
    rw [← map_nsmul,P.property,map_zero]⟩
  left_inv P:=by apply Subtype.ext;exact (scalingEquiv a b u hu).left_inv P.val
  right_inv P:=by apply Subtype.ext;exact (scalingEquiv a b u hu).right_inv P.val

theorem point_cardinality_invariant (a b u : K) (hu : u≠0) :
    Nat.card (shortCurve a b).Point=Nat.card (shortCurve (u^4*a) (u^6*b)).Point :=
  Nat.card_congr (scalingEquiv a b u hu).toEquiv

theorem torsion_cardinality_invariant (a b u : K) (hu : u≠0) (n : ℕ) :
    Nat.card (killedPoints a b n)=Nat.card (killedPoints (u^4*a) (u^6*b) n) :=
  Nat.card_congr (scalingTorsionEquiv a b u hu n)

/-- General rational3-torsion count; no algebraic-closure/rank premise. -/
theorem rational_three_torsion_root_count (a b : K)
    (hd : 4*a^3+27*b^2≠0) (h2 : (2:K)≠0) (h3 : (3:K)≠0) :
    Nat.card (geometricTorsion a b)=
      (∑ x : xRoots a b,Fintype.card (yRoots a b x))+1 := by
  classical
  rw [Nat.card_congr (geometricCoordinateEquiv a b hd h2 h3),Nat.card_eq_fintype_card,
    Fintype.card_option,Fintype.card_sigma]

theorem scaling_audit_certificate (a b u : K) (hu : u≠0) :
    (Nat.card (shortCurve a b).Point=Nat.card (shortCurve (u^4*a) (u^6*b)).Point) ∧
    (∀ n : ℕ,Nat.card (killedPoints a b n)=
      Nat.card (killedPoints (u^4*a) (u^6*b) n)) :=
  ⟨point_cardinality_invariant a b u hu,torsion_cardinality_invariant a b u hu⟩

#print axioms scaling_audit_certificate
#print axioms rational_three_torsion_root_count
end
end BKBasefieldScalingAudit
