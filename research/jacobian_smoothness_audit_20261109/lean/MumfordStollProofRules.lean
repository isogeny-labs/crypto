import MumfordStollGeneral

/-! A restricted coefficient simplification set. Lean requires registering
new simp attributes in a module imported before they are used. -/
register_simp_attr mumford_stoll_coeff

namespace MumfordStollProofRulesAudit
noncomputable section
open Polynomial MumfordStollAudit

theorem coeff_constant_product (a b : K) (p : K[X]) (n : ℕ) :
    (Polynomial.C a*Polynomial.C b*p).coeff n=a*(b*p.coeff n) := by
  simp only [mul_assoc,Polynomial.coeff_C_mul]

theorem coeff_right_constant_power (p : K[X]) (a : K) (m n : ℕ) :
    (p*(Polynomial.C a)^m).coeff n=p.coeff n*a^m := by
  rw [←map_pow,Polynomial.coeff_mul_C]

theorem coeff_left_constant_power (p : K[X]) (a : K) (m n : ℕ) :
    ((Polynomial.C a)^m*p).coeff n=a^m*p.coeff n := by
  rw [←map_pow,Polynomial.coeff_C_mul]

end
end MumfordStollProofRulesAudit
