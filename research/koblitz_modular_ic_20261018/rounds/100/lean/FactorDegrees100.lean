import Std.Tactic

namespace FactorDegrees100

-- Degree arithmetic for the three irreducible factors checked independently
-- over F2[x] by verify100.py. This file does not prove irreducibility or
-- the polynomial factor product.
theorem rootDegree : 11 + 3 * 22 + 46 = 123 := by omega
theorem numeratorDegree : 16 * (11 + 3 * 22 + 46) = 1968 := by omega
theorem supportDegrees : (List.eraseDups [11, 22, 1]).length = 3 := by decide

end FactorDegrees100
