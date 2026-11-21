import Mathlib

/-! Three-sum coverage is bounded by the number of unordered triples with repetition. -/

namespace H4TripleSumBound

variable {G : Type*} [AddCommMonoid G] [DecidableEq G]

/-- All three-summand values from a finite base, represented by its symmetric cube. -/
noncomputable def threeSums (B : Finset G) : Finset G :=
  (Finset.univ : Finset (Sym {x : G // x ∈ B} 3)).image
    (fun (t : Sym {x : G // x ∈ B} 3) => ((t : Multiset {x : G // x ∈ B}).map
      (fun x => (x : G))).sum)

/-- Every ordered triple from the base contributes to its symmetric-cube sum image. -/
theorem add_mem_threeSums (B : Finset G) (a b c : G)
    (ha : a ∈ B) (hb : b ∈ B) (hc : c ∈ B) :
    a + b + c ∈ threeSums B := by
  unfold threeSums
  refine Finset.mem_image.mpr ⟨
    Sym.cons ⟨a, ha⟩ (Sym.cons ⟨b, hb⟩ (Sym.cons ⟨c, hc⟩ Sym.nil)),
    Finset.mem_univ _, ?_⟩
  simp [Sym.coe_cons, Sym.coe_nil, Multiset.map_cons, Multiset.sum_cons, add_assoc]

/-- The directly enumerated ordered triple-sum image. -/
def orderedThreeSums (B : Finset G) : Finset G :=
  ((B.product B).product B).image (fun p => p.1.1 + p.1.2 + p.2)

theorem orderedThreeSums_subset (B : Finset G) :
    orderedThreeSums B ⊆ threeSums B := by
  intro x hx
  rcases Finset.mem_image.mp hx with ⟨⟨⟨a, b⟩, c⟩, hp, rfl⟩
  rcases Finset.mem_product.mp hp with ⟨hab, hc⟩
  rcases Finset.mem_product.mp hab with ⟨ha, hb⟩
  exact add_mem_threeSums B a b c ha hb hc

/-- The exact multiset-count bound, including repeated summands. -/
theorem card_threeSums_le (B : Finset G) :
    (threeSums B).card ≤ (B.card + 2).choose 3 := by
  unfold threeSums
  calc
    (Finset.univ.image (fun t : Sym {x : G // x ∈ B} 3 =>
      ((t : Multiset {x : G // x ∈ B}).map (fun x => (x : G))).sum)).card
        ≤ (Finset.univ : Finset (Sym {x : G // x ∈ B} 3)).card := Finset.card_image_le
    _ = (B.card + 2).choose 3 := by
      rw [Finset.card_univ, Sym.card_sym_eq_choose]
      simp only [Fintype.card_coe]
      congr 1

/-- A finite base with `B` points has at most `choose (B+2) 3` distinct triple sums. -/
theorem card_orderedThreeSums_le (B : Finset G) :
    (orderedThreeSums B).card ≤ (B.card + 2).choose 3 :=
  (Finset.card_le_card (orderedThreeSums_subset B)).trans (card_threeSums_le B)

/-- The instantiated bound for the audited 1072-point base size. -/
theorem card_orderedThreeSums_le_1072 (B : Finset G) (hB : B.card = 1072) :
    (orderedThreeSums B).card ≤ 205895824 := by
  have hchoose : (1074 : ℕ).choose 3 = 205895824 := by
    have htwo : (1074 : ℕ).choose 2 = 576201 := by
      rw [Nat.choose_two_right]
    have hthree := Nat.choose_succ_right_eq 1074 2
    rw [htwo] at hthree
    norm_num at hthree
    omega
  simpa [hB, hchoose] using card_orderedThreeSums_le B

#print axioms H4TripleSumBound.add_mem_threeSums
#print axioms H4TripleSumBound.orderedThreeSums_subset
#print axioms H4TripleSumBound.card_threeSums_le
#print axioms H4TripleSumBound.card_orderedThreeSums_le
#print axioms H4TripleSumBound.card_orderedThreeSums_le_1072

end H4TripleSumBound
