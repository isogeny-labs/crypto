import Std

def states (k : Nat) : Nat := 83 * (k * (k - 1) / 2) + 42 * k
def load80Request (s : Nat) : Nat := (5 * s + 3) / 4
def rootWord (relativeState selector fingerprint : Nat) : Nat :=
  4 * (relativeState + 1) + 2 * selector + fingerprint

theorem states_monotone {a b : Nat} (h : a ≤ b) : states a ≤ states b := by
  have hmul : a * (a - 1) ≤ b * (b - 1) :=
    Nat.mul_le_mul h (Nat.sub_le_sub_right h 1)
  have hdiv : a * (a - 1) / 2 ≤ b * (b - 1) / 2 :=
    Nat.div_le_div_right hmul
  have hscaled : 83 * (a * (a - 1) / 2) ≤ 83 * (b * (b - 1) / 2) :=
    Nat.mul_le_mul_left 83 hdiv
  have hlin : 42 * a ≤ 42 * b := Nat.mul_le_mul_left 42 h
  simpa [states] using Nat.add_le_add hscaled hlin

theorem request_monotone {a b : Nat} (h : a ≤ b) :
    load80Request a ≤ load80Request b := by
  have hm : 5 * a ≤ 5 * b := Nat.mul_le_mul_left 5 h
  have ha : 5 * a + 3 ≤ 5 * b + 3 := Nat.add_le_add_right hm 3
  simpa [load80Request] using (Nat.div_le_div_right ha (c := 4))

theorem core_states : states 4549 = 858778416 := by decide
theorem full_states : states 5500 = 1255377750 := by decide
theorem extension_states : states 5500 - states 4549 = 396599334 := by decide

theorem relative_ids_fit :
    states 4549 < 2^30 ∧ states 5500 - states 4549 < 2^30 := by decide

theorem split_slot_buckets :
    2^29 < load80Request (states 4549) ∧
    load80Request (states 4549) ≤ 2^30 ∧
    2^28 < load80Request (states 5500 - states 4549) ∧
    load80Request (states 5500 - states 4549) ≤ 2^29 := by decide

theorem k4550_primary_crosses :
    2^30 < load80Request (states 4550) := by decide

-- With the primary fixed at 2^31 slots, K4549 is the largest possible
-- first prefix. The split lookup always probes this prefix first.
theorem k4549_max_primary (k : Nat)
    (hcap : load80Request (states k) ≤ 2^30) : k ≤ 4549 := by
  by_cases h : k ≤ 4549
  · exact h
  · have hk : 4550 ≤ k := by omega
    have hs : states 4550 ≤ states k := states_monotone hk
    have hr := request_monotone hs
    have hx := Nat.lt_of_lt_of_le k4550_primary_crosses hr
    exact False.elim ((Nat.not_lt_of_ge hcap) hx)

theorem split_load80 :
    10 * states 4549 ≤ 4 * 2^31 ∧
    10 * (states 5500 - states 4549) ≤ 4 * 2^30 := by decide

-- State ordinals, not pair labels, choose the table. Each local ordinal
-- decodes to its original global ordinal without an auxiliary map.
theorem ordinal_split_roundtrip (cut total global : Nat)
    (hcut : cut ≤ total) (hglobal : global < total) :
    (if global < cut then global < cut else global - cut < total - cut) ∧
    (if global < cut then global else cut + (global - cut)) = global := by
  by_cases h : global < cut
  · simp [h]
  · simp [h]
    omega

theorem relative_witness_roundtrip (base relativeState selector fingerprint : Nat)
    (hstate : relativeState < 2^30 - 1)
    (hselector : selector < 2)
    (hfp : fingerprint < 2) :
    0 < rootWord relativeState selector fingerprint ∧
    rootWord relativeState selector fingerprint < 2^32 ∧
    base + (rootWord relativeState selector fingerprint / 4 - 1) =
      base + relativeState ∧
    (rootWord relativeState selector fingerprint / 2) % 2 = selector ∧
    rootWord relativeState selector fingerprint % 2 = fingerprint := by
  have h30 : 2^30 = 1073741824 := by decide
  have h32 : 2^32 = 4294967296 := by decide
  unfold rootWord
  omega

theorem relative_witness_injective (i j r s f g : Nat)
    (hr : r < 2) (hs : s < 2) (hf : f < 2) (hg : g < 2)
    (heq : rootWord i r f = rootWord j s g) :
    i = j ∧ r = s ∧ f = g := by
  unfold rootWord at heq
  omega

theorem k5500_split_principal_bytes :
    4 * (2^31 + 2^30) + 11 * states 5500 + 2^29 + 2^28 =
      27499363506 := by decide

theorem split_saves_425_gib :
    32062766258 - 27499363506 = 4563402752 := by decide

#print axioms core_states
#print axioms states_monotone
#print axioms request_monotone
#print axioms full_states
#print axioms extension_states
#print axioms relative_ids_fit
#print axioms split_slot_buckets
#print axioms k4549_max_primary
#print axioms split_load80
#print axioms ordinal_split_roundtrip
#print axioms relative_witness_roundtrip
#print axioms relative_witness_injective
#print axioms k5500_split_principal_bytes
#print axioms split_saves_425_gib
