import RationalTwo

namespace BKBasefieldScalingAudit
noncomputable section
open ThreeTorsionRankAudit
variable {K : Type*} [Field K] [DecidableEq K]

theorem rational_low_torsion_certificate (a b : K)
    (hd : 4*a^3+27*b^2≠0) (h2 : (2:K)≠0) (h3 : (3:K)≠0) :
    (Nat.card (killedPoints a b 2)=Fintype.card (twoRoots a b)+1) ∧
    (Nat.card (killedPoints a b 3)=
      (∑ x : xRoots a b,Fintype.card (yRoots a b x))+1) := by
  constructor
  · exact rational_two_torsion_root_count a b hd h2
  · simpa only [killedPoints,geometricTorsion] using
      rational_three_torsion_root_count a b hd h2 h3

theorem geometric_low_torsion_certificate [IsAlgClosed K] (a b : K)
    (hd : 4*a^3+27*b^2≠0) (h2 : (2:K)≠0) (h3 : (3:K)≠0) :
    Nat.card (killedPoints a b 2)=4 ∧ Nat.card (killedPoints a b 3)=9 := by
  constructor
  · exact geometric_two_torsion_cardinality a b hd h2
  · simpa only [killedPoints,geometricTorsion] using
      geometric_three_torsion_cardinality a b hd h2 h3

#print axioms rational_low_torsion_certificate
#print axioms geometric_low_torsion_certificate
end
end BKBasefieldScalingAudit
