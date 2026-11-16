import Init
set_option maxRecDepth 10000

/-! Checked mask arithmetic for four trace-minus-fourteen Frobenius orbits.
    Cover-search elimination and geometric normalization are not formalized. -/
namespace GHSTraceMinus14Orbits

def coeff (m i : Nat) : Nat := m / 3^i % 3

def basisCoeff (e t : Nat) : Nat :=
  if e < 5 then (if e = t then 1 else 0)
  else (if e - 4 = t then 1 else 0) + (if e - 5 = t then 2 else 0)

def productCoeff (u v t : Nat) : Nat :=
  ((List.range 5).foldl (fun acc i =>
    acc + (List.range 5).foldl (fun inner j =>
      inner + coeff u i * coeff v j * basisCoeff (i + j) t) 0) 0) % 3

def multiplyMasks (u v : Nat) : Nat :=
  (List.range 5).foldl (fun acc t => acc + productCoeff u v t * 3^t) 0

def cubeMask (u : Nat) : Nat := multiplyMasks (multiplyMasks u u) u

theorem orbit_11 :
    cubeMask 11 = 17 ∧ cubeMask 17 = 71 ∧
    cubeMask 71 = 147 ∧ cubeMask 147 = 238 ∧ cubeMask 238 = 11 := by decide

theorem orbit_202 :
    cubeMask 202 = 81 ∧ cubeMask 81 = 117 ∧
    cubeMask 117 = 47 ∧ cubeMask 47 = 187 ∧ cubeMask 187 = 202 := by decide

theorem orbit_38 :
    cubeMask 38 = 172 ∧ cubeMask 172 = 223 ∧
    cubeMask 223 = 53 ∧ cubeMask 53 = 241 ∧ cubeMask 241 = 38 := by decide

theorem orbit_80 :
    cubeMask 80 = 153 ∧ cubeMask 153 = 190 ∧
    cubeMask 190 = 156 ∧ cubeMask 156 = 217 ∧ cubeMask 217 = 80 := by decide

theorem source_and_cell_counts :
    (4 : Nat) * 5 = 20 ∧ 12 * 4 = 48 ∧
    48 * 59697 = 2865456 ∧ 12 * 20 = 240 := by decide

theorem selected_prime_divides_order : (258 : Nat) = 6 * 43 := by decide

end GHSTraceMinus14Orbits
