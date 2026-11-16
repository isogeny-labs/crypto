import CatalogQ16

/- Named versions of exactly the original sixteen arithmetic examples.
   This does not establish Tate isogeny or verify catalog models. -/
namespace CatalogQ16ArithmeticAudit

theorem order_factorization : 3974 = 2 * 1987 := by decide

theorem trace_difference : 4097 - 3974 = 123 := by decide

theorem cubic_trace : (6 : Int) - 3 * 123 = -363 := by decide

theorem coprime_order_trace : Nat.gcd 1987 363 = 1 := by decide

theorem extension_one : 17 = 16 + 1 := by decide

theorem extension_two : 257 = 16^2 + 1 := by decide

theorem extension_three : 3728 = 16^3 + 1 - 3 * 123 := by decide

theorem census_total : 8142 + 48 = 8190 := by decide

theorem small_total : 126 + 2 = 128 := by decide

theorem first_pair : 2156 + 452 = 2608 := by decide

theorem second_pair : 5302 + 232 = 5534 := by decide

theorem second_pair_extended : 5302 + 232 + 48 = 5582 := by decide

theorem combined_pairs : 2156 + 452 + 5302 + 232 = 8142 := by decide

theorem first_orbit_total : 3852 + 180 = 4032 := by decide

theorem small_orbit_total : 12 + 4 = 16 := by decide

theorem all_orbit_total : 4032 + 16 = 4048 := by decide

end CatalogQ16ArithmeticAudit
