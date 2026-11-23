import DegreeBridge
import SecantFixture

noncomputable section
namespace CanonicalFixture
open SecantFixture
local instance : Fact (Nat.Prime 17) := ⟨by decide⟩

/-- Exact F17 factor-base fixture, all four sign branches, canonical top forms
computed from the actual total degree of every original generator. -/
theorem degree_five_to_three (r s e f : F)
    (he : e = 1 ∨ e = -1) (hf : f = 1 ∨ f = -1) :
    SecantIdeal.FirstVanishing
      (DegreeBridge.canonicalSecant 2 (e*4) (f*4) r s 2
        (graphLower 0 e) (graphLower 1 f)) 3 ∧
    SecantIdeal.FirstVanishing
      (DegreeBridge.canonicalDense 4 2 2 r (denseLower 0) (denseLower 1)) 5 := by
  have h := DegreeBridge.canonical_strict_reduction (K := F) 2 (by decide)
    (e*4) (f*4) r s 2 2 (sign_nonzero e he) (sign_nonzero f hf) (by decide)
    (graphLower 0 e) (graphLower 1 f) (graphLower_degree 0 e) (graphLower_degree 1 f)
    (denseLower 0) (denseLower 1) (denseLower_degree 0) (denseLower_degree 1)
  exact ⟨h.1, h.2.1⟩

end CanonicalFixture

#print axioms CanonicalFixture.degree_five_to_three
