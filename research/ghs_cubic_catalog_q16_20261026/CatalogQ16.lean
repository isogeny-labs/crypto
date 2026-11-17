import Init

/- Exact arithmetic obligations for the frozen finite census. The geometric
   Tate isogeny theorem and catalog models are outside this Lean file. -/

example : 3974 = 2 * 1987 := by decide
example : 4097 - 3974 = 123 := by decide
example : (6 : Int) - 3 * 123 = -363 := by decide
example : Nat.gcd 1987 363 = 1 := by decide
example : 17 = 16 + 1 := by decide
example : 257 = 16^2 + 1 := by decide
example : 3728 = 16^3 + 1 - 3 * 123 := by decide
example : 8142 + 48 = 8190 := by decide
example : 126 + 2 = 128 := by decide
example : 2156 + 452 = 2608 := by decide
example : 5302 + 232 = 5534 := by decide
example : 5302 + 232 + 48 = 5582 := by decide
example : 2156 + 452 + 5302 + 232 = 8142 := by decide
example : 3852 + 180 = 4032 := by decide
example : 12 + 4 = 16 := by decide
example : 4032 + 16 = 4048 := by decide
