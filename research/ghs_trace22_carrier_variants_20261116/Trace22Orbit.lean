import Init
set_option maxRecDepth 10000

/-! Exact finite-field mask arithmetic and bare exponent screening for the
trace +22 carrier census. The cover search and attack algorithms are not
formalized here. -/
namespace GHSTrace22Orbit

def maskCoeff (m i : Nat) : Nat := m / 3^i % 3

def basisCoeff (e t : Nat) : Nat :=
  if e < 5 then (if e = t then 1 else 0)
  else (if e - 4 = t then 1 else 0) + (if e - 5 = t then 2 else 0)

def productCoeff (u v t : Nat) : Nat :=
  ((List.range 5).foldl (fun acc i =>
    acc + (List.range 5).foldl (fun inner j =>
      inner + maskCoeff u i * maskCoeff v j * basisCoeff (i + j) t) 0) 0) % 3

def multiplyMasks (u v : Nat) : Nat :=
  (List.range 5).foldl (fun acc t => acc + productCoeff u v t * 3^t) 0

def cubeMask (u : Nat) : Nat := multiplyMasks (multiplyMasks u u) u

theorem first_source_orbit :
    cubeMask 68 = 120 ∧ cubeMask 120 = 74 ∧
    cubeMask 74 = 99 ∧ cubeMask 99 = 111 ∧ cubeMask 111 = 68 := by decide

theorem second_source_orbit :
    cubeMask 84 = 144 ∧ cubeMask 144 = 211 ∧
    cubeMask 211 = 96 ∧ cubeMask 96 = 105 ∧ cubeMask 105 = 84 := by decide

theorem cell_count : (12 : Nat) * 2 = 24 ∧ 24 * 5 = 120 := by decide

theorem s11_bare_exponent_screen :
    (785807256797924665975331 : Nat)^5 > 11^5 * (3^11)^16 := by decide

end GHSTrace22Orbit
