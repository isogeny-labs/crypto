/-!
Exact arithmetic for the N83 symmetric-root state formula and the oriented
K5086/K4549 split request. The correspondence of these formulas to every
Rust execution path and actual K5086 base enumeration remain open checks;
see CAPACITY.md for the proof map and the source bridge.
-/
namespace N83SplitCapacity

def states (k : Nat) : Nat := 83 * k * (k - 1) / 2 + 42 * k

def request (n : Nat) : Nat := (5 * n + 3) / 4

def id31Slot (state selector fingerprint : Nat) : Nat :=
  fingerprint * 2 ^ 32 + (state + 1) * 2 + selector

theorem id31_slot_roundtrip (state selector fingerprint : Nat)
    (hstate : state + 1 < 2 ^ 31) (hselector : selector < 2) :
    ((id31Slot state selector fingerprint % 2 ^ 32) / 2 - 1 = state) ∧
    ((id31Slot state selector fingerprint % 2 ^ 32) % 2 = selector) ∧
    (id31Slot state selector fingerprint / 2 ^ 32 = fingerprint) := by
  unfold id31Slot
  omega

theorem id31_slot_nonzero_five_bytes (state selector fingerprint : Nat)
    (hstate : state + 1 < 2 ^ 31) (hselector : selector < 2)
    (hfingerprint : fingerprint < 2 ^ 8) :
    0 < id31Slot state selector fingerprint ∧
    id31Slot state selector fingerprint < 2 ^ 40 := by
  unfold id31Slot
  omega

theorem states_monotone {a b : Nat} (h : a ≤ b) : states a ≤ states b := by
  have hmul : a * (a - 1) ≤ b * (b - 1) :=
    Nat.mul_le_mul h (Nat.sub_le_sub_right h 1)
  have hscaled : 83 * (a * (a - 1)) ≤ 83 * (b * (b - 1)) :=
    Nat.mul_le_mul_left 83 hmul
  have hdiv : (83 * (a * (a - 1))) / 2 ≤ (83 * (b * (b - 1))) / 2 :=
    Nat.div_le_div_right hscaled
  have hlin : 42 * a ≤ 42 * b := Nat.mul_le_mul_left 42 h
  simpa [states, Nat.mul_assoc] using Nat.add_le_add hdiv hlin

theorem request_monotone {a b : Nat} (h : a ≤ b) : request a ≤ request b := by
  have hm : 5 * a ≤ 5 * b := Nat.mul_le_mul_left 5 h
  have ha : 5 * a + 3 ≤ 5 * b + 3 := Nat.add_le_add_right hm 3
  simpa [request] using (Nat.div_le_div_right ha (c := 4))

theorem k5086_state_fits : states 5086 < 2 ^ 30 := by decide
theorem k5087_state_crosses : 2 ^ 30 ≤ states 5087 := by decide
theorem k5086_max : ∀ k, 5087 ≤ k → 2 ^ 30 ≤ states k := by
  intro k hk
  exact Nat.le_trans k5087_state_crosses (states_monotone hk)

theorem k5086_core_fits : request (states 4549) ≤ 2 ^ 30 := by decide
theorem k5086_ext_fits : request (states 5086 - states 4549) ≤ 2 ^ 28 := by decide
theorem k5086_core_lower_crosses : 2 ^ 28 < request (states 4549) := by decide
theorem k5086_ext_lower_crosses : 2 ^ 27 < request (states 5086 - states 4549) := by decide
theorem k4550_core_crosses : 2 ^ 30 < request (states 4550) := by decide
theorem k4548_ext_crosses : 2 ^ 28 < request (states 5086 - states 4548) := by decide
theorem k5086_only_core_split : ∀ k, k < 5086 →
    request (states k) ≤ 2 ^ 30 →
    request (states 5086 - states k) ≤ 2 ^ 28 → k = 4549 := by
  intro k _ hcore hext
  by_cases hlow : k ≤ 4548
  · have hs : states k ≤ states 4548 := states_monotone hlow
    have he : states 5086 - states 4548 ≤ states 5086 - states k :=
      Nat.sub_le_sub_left hs (states 5086)
    have hr := request_monotone he
    have hx := Nat.lt_of_lt_of_le k4548_ext_crosses hr
    exact False.elim ((Nat.not_lt_of_ge hext) hx)
  · by_cases heq : k = 4549
    · exact heq
    · have hhi : 4550 ≤ k := by omega
      have hs : states 4550 ≤ states k := states_monotone hhi
      have hr := request_monotone hs
      have hx := Nat.lt_of_lt_of_le k4550_core_crosses hr
      exact False.elim ((Nat.not_lt_of_ge hcore) hx)
theorem k5086_root_bound : 2 * states 5086 < 2 ^ 31 := by decide
theorem k5086_point_count_formula : 166 * 5086 = 844276 := by decide
theorem k5086_table_bytes : 5 * (2 ^ 31 + 2 ^ 29) = 13421772800 := by decide
theorem k5086_wide3_bloom_bucket :
    2 ^ 34 < 16 * request (states 5086) ∧
    16 * request (states 5086) ≤ 2 ^ 35 := by decide
theorem k5086_primary_wide3_bloom_bucket :
    2 ^ 33 < 16 * request (states 4549) ∧
    16 * request (states 4549) ≤ 2 ^ 34 := by decide
theorem k5086_extension_wide3_bloom_bucket :
    2 ^ 31 < 16 * request (states 5086 - states 4549) ∧
    16 * request (states 5086 - states 4549) ≤ 2 ^ 32 := by decide
theorem k5086_state_bytes : 11 * states 5086 = 11808494247 := by decide
theorem k5086_three_array_bytes :
    11 * states 5086 + 5 * (2 ^ 31 + 2 ^ 29) + 2 ^ 32 = 29525234343 := by decide
theorem k5086_dual_bloom_bytes : 2 ^ 31 + 2 ^ 29 = 2684354560 := by decide
theorem k5086_dual_three_array_bytes :
    11 * states 5086 + 6 * (2 ^ 31 + 2 ^ 29) = 27914621607 := by decide

-- The opt-in ID31 slot keeps exact-key verification while assigning 31 bits
-- to a state ID, one to the root selector, and eight to the fingerprint.
theorem k7193_state_fits_id31 : states 7193 < 2 ^ 31 := by decide
theorem k7194_state_crosses_id31 : 2 ^ 31 ≤ states 7194 := by decide
theorem k7193_id31_max : ∀ k, 7194 ≤ k → 2 ^ 31 ≤ states k := by
  intro k hk
  exact Nat.le_trans k7194_state_crosses_id31 (states_monotone hk)
theorem k5500_state_fits_id31 : states 5500 < 2 ^ 31 := by decide
theorem k5500_root_bound_id31 : 2 * states 5500 < 2 ^ 32 := by decide
theorem k5500_core_request :
    2 ^ 29 < request (states 4549) ∧
    request (states 4549) ≤ 2 ^ 30 := by decide
theorem k5500_extension_request :
    2 ^ 28 < request (states 5500 - states 4549) ∧
    request (states 5500 - states 4549) ≤ 2 ^ 29 := by decide
theorem k5500_state_count : states 5500 = 1255377750 := by decide
theorem k5500_point_count_formula : 166 * 5500 = 913000 := by decide
theorem k5500_dual_three_array_bytes :
    11 * states 5500 + 6 * (2 ^ 31 + 2 ^ 30) = 33136508082 := by decide

#print axioms states_monotone
#print axioms request_monotone
#print axioms id31_slot_roundtrip
#print axioms id31_slot_nonzero_five_bytes
#print axioms k5086_state_fits
#print axioms k5087_state_crosses
#print axioms k5086_max
#print axioms k5086_core_fits
#print axioms k5086_ext_fits
#print axioms k5086_core_lower_crosses
#print axioms k5086_ext_lower_crosses
#print axioms k4550_core_crosses
#print axioms k4548_ext_crosses
#print axioms k5086_only_core_split
#print axioms k5086_root_bound
#print axioms k5086_point_count_formula
#print axioms k5086_table_bytes
#print axioms k5086_wide3_bloom_bucket
#print axioms k5086_primary_wide3_bloom_bucket
#print axioms k5086_extension_wide3_bloom_bucket
#print axioms k5086_state_bytes
#print axioms k5086_three_array_bytes
#print axioms k5086_dual_bloom_bytes
#print axioms k5086_dual_three_array_bytes
#print axioms k7193_state_fits_id31
#print axioms k7194_state_crosses_id31
#print axioms k7193_id31_max
#print axioms k5500_state_fits_id31
#print axioms k5500_root_bound_id31
#print axioms k5500_core_request
#print axioms k5500_extension_request
#print axioms k5500_state_count
#print axioms k5500_point_count_formula
#print axioms k5500_dual_three_array_bytes
end N83SplitCapacity
