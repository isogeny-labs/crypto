/-!
Finite arithmetic gates for the q16 prime-to-subgroup-order isogeny argument.
The discriminant identity and Tate-module theorem are mathematical inputs,
not formalized here. The file checks the concrete modular consequences.
-/

namespace GHSQ16IsogenyGate

theorem frontier_count : 12 + 30 + 24 = 66 := by decide

theorem order_3974_prime_factor : 3974 = 2 * 1987 := by decide
theorem order_3978_prime_factor : 3978 = 234 * 17 := by decide
theorem order_4178_prime_factor : 4178 = 2 * 2089 := by decide

theorem discriminant_nonzero_3974 :
    1987 - ((3^6 * 16^6 * (4*16^3 - 123^2)^3) % 1987) = 587 := by decide

theorem discriminant_nonzero_3978 :
    17 - ((3^6 * 16^6 * (4*16^3 - 119^2)^3) % 17) = 8 := by decide

theorem discriminant_nonzero_4178 :
    2089 - ((3^6 * 16^6 * (4*16^3 - 81^2)^3) % 2089) = 1702 := by decide

end GHSQ16IsogenyGate
