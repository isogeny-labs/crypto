import NatClassifier
set_option autoImplicit false
namespace PencilWordRefinement
open ProjectiveLawRuntime

def wordNeg (p b : UInt64) : UInt64 := (p - b%p)%p
def wordSub (p a b : UInt64) : UInt64 := (a + wordNeg p b)%p

theorem wordNeg_value (p b : UInt64) (hp : 0 < p.toNat) :
    (wordNeg p b).toNat = residueNeg p.toNat b.toNat := by
  have h : b%p ≤ p := UInt64.le_iff_toNat_le.mpr (by
    rw [UInt64.toNat_mod]
    exact Nat.le_of_lt (Nat.mod_lt _ hp))
  simp only [wordNeg, residueNeg, UInt64.toNat_mod, UInt64.toNat_sub_of_le p (b%p) h]

theorem wordSub_value (p a b : UInt64) (hp : 0 < p.toNat)
    (hbound : p.toNat < 2^31) (ha : a.toNat < p.toNat) :
    (wordSub p a b).toNat = residueSub p.toNat a.toNat b.toNat := by
  have hn : (wordNeg p b).toNat < p.toNat := by
    rw [wordNeg_value p b hp]
    exact Nat.mod_lt _ hp
  have hsum : a.toNat + (wordNeg p b).toNat < 2^64 := by omega
  have hsum2 : a.toNat + residueNeg p.toNat b.toNat < 2^64 := by
    simpa only [wordNeg_value p b hp] using hsum
  simp only [wordSub, UInt64.toNat_mod, UInt64.toNat_add, Nat.mod_eq_of_lt hsum2,
    wordNeg_value p b hp, residueSub]

def classifyWord (p s₁ t₁ s₂ t₂ : UInt64) : Branch22Prelude :=
  if s₁ = wordNeg p s₂ ∧ t₁ = wordNeg p t₂ then .sameUInverse
  else if s₁ = s₂ ∧ t₁ = t₂ then .sameUDouble
  else if wordSub p s₁ s₂ = 0 then .sameUMixedZeroSlope
  else .sameUMixed

theorem word_classifier_to_nat (p s₁ t₁ s₂ t₂ : UInt64)
    (hp : 0 < p.toNat) (hbound : p.toNat < 2^31)
    (hs₁ : s₁.toNat < p.toNat) :
    classifyWord p s₁ t₁ s₂ t₂ =
      classifyNat p.toNat s₁.toNat t₁.toNat s₂.toNat t₂.toNat := by
  unfold classifyWord classifyNat
  simp only [← UInt64.toNat_inj, wordNeg_value p s₂ hp, wordNeg_value p t₂ hp,
    wordSub_value p s₁ s₂ hp hbound hs₁, UInt64.toNat_ofNat]

theorem word_classifier_decode (p s₁ t₁ s₂ t₂ : UInt64) [Fact p.toNat.Prime]
    (hfive : 5 ≤ p.toNat) (hbound : p.toNat < 2^31)
    (hs₁ : s₁.toNat < p.toNat) (ht₁ : t₁.toNat < p.toNat)
    (hs₂ : s₂.toNat < p.toNat) (ht₂ : t₂.toNat < p.toNat) :
    classifyWord p s₁ t₁ s₂ t₂ =
    classifySameU22 (s₁.toNat : ZMod p.toNat) (t₁.toNat : ZMod p.toNat)
      (s₂.toNat : ZMod p.toNat) (t₂.toNat : ZMod p.toNat) := by
  have hp : 0 < p.toNat := by omega
  rw [word_classifier_to_nat p s₁ t₁ s₂ t₂ hp hbound hs₁]
  exact classifier_decode p.toNat s₁.toNat t₁.toNat s₂.toNat t₂.toNat hp hs₁ ht₁ hs₂ ht₂

-- Overlapping inverse/equal predicates must take the inverse branch first.
example : classifyWord 5 0 0 0 0 = .sameUInverse := by decide
end PencilWordRefinement
