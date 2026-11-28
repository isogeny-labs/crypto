import Std.Tactic

namespace DirectNorm103

-- The source-bound R101 check supplies the lambda exponents. In
-- characteristic two, only the odd exponent contributes to d/dlambda.
theorem lambdaExponentParity :
    16 % 2 = 0 ∧ 8 % 2 = 0 ∧ 4 % 2 = 0 ∧ 1 % 2 = 1 := by
  decide

theorem directDenominatorPower : 16 * 163 = 2608 := by omega
theorem directNumeratorDegreePower : 16 * 123 = 1968 := by omega

end DirectNorm103
