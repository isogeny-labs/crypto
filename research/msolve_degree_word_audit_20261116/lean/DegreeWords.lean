import DegreeImports

namespace MsolveDegreeWordAudit

/-- The unsigned16 accumulation used for a non-block exponent degree. -/
def degreeWord : List ℕ → UInt16
  | [] => 0
  | a::as => UInt16.ofNat a + degreeWord as

theorem degree_word_mod (as : List ℕ) :
    (degreeWord as).toNat = as.sum % 65536 := by
  induction as with
  | nil => simp [degreeWord]
  | cons a as ih =>
    simp [degreeWord, UInt16.toNat_add, UInt16.toNat_ofNat', ih, Nat.add_mod]

theorem degree_word_exact_iff (as : List ℕ) :
    (degreeWord as).toNat = as.sum ↔ as.sum < 65536 := by
  constructor
  · intro h
    have ht := UInt16.toNat_lt (degreeWord as)
    rw [h] at ht
    exact ht
  · intro h
    rw [degree_word_mod, Nat.mod_eq_of_lt h]

def lcmDegreeWord (a b c d : ℕ) : UInt16 := degreeWord [max a c, max b d]

theorem lcm_degree_guard (a b c d : ℕ)
    (h : max a c + max b d < 65536) :
    (lcmDegreeWord a b c d).toNat = max a c + max b d := by
  apply (degree_word_exact_iff _).mpr
  simpa only [List.sum_cons, List.sum_nil, add_zero] using h

theorem boundary_input_degrees :
    65535+0 < 65536 ∧ 65534+1 < 65536 := by decide +kernel

theorem boundary_lcm_word : (lcmDegreeWord 65535 0 65534 1).toNat = 0 := by
  norm_num [lcmDegreeWord, degree_word_mod, degreeWord]

theorem boundary_lcm_natural : max 65535 65534 + max 0 1 = 65536 := by decide +kernel

theorem unrestricted_lcm_degree_reading_false :
    ¬ ∀ a b c d : ℕ, a+b<65536 → c+d<65536 →
      (lcmDegreeWord a b c d).toNat = max a c + max b d := by
  intro h
  have hh := h 65535 0 65534 1 boundary_input_degrees.1 boundary_input_degrees.2
  rw [boundary_lcm_word, boundary_lcm_natural] at hh
  contradiction

theorem small_lcm_word : (lcmDegreeWord 2 0 1 1).toNat = 3 := by
  norm_num [lcmDegreeWord, degree_word_mod, degreeWord]

#print axioms degree_word_mod
#print axioms degree_word_exact_iff
#print axioms unrestricted_lcm_degree_reading_false
end MsolveDegreeWordAudit
