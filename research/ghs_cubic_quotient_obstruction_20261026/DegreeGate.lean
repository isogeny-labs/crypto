import Init

/-!
The natural-number degree inequalities used in the quotient-lift proof.
This file does not formalize the hyperelliptic curve, its automorphisms,
or the projective branch-set stabilizer; Sage and the written proof cover
those separate obligations.
-/

namespace GhsCubicDegreeGate

theorem low_degree (d : Nat) (hd : d < 4) : 2*d < d+4 := by omega
theorem high_degree (d : Nat) (hd : 4 < d) : d+4 < 2*d := by omega
theorem equal_degrees_only_at_four (d : Nat)
    (h : 2*d = d+4) : d = 4 := by omega

theorem branch_set_stabilizer_count : 4 * 3 = 12 := by decide
theorem nonidentity_partition : 3 + 8 = 11 := by decide

end GhsCubicDegreeGate
