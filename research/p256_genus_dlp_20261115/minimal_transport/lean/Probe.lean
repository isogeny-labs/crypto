import Mathlib.Data.Int.Basic

-- Assembly probe: inherit the integer positive-multiple bound as stated.
#check Int.le_of_dvd
example (q n : ℤ) (hq : 0 < q) (h : n ∣ q) : n ≤ q :=
  Int.le_of_dvd hq h
