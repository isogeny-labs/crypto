import Std
namespace SignedReciprocal
-- Conditional ring identity. Weighted projection and stream semantics remain separate obligations.
theorem norm_identity {R : Type} [Lean.Grind.CommRing R]
    (a b v vi w : R) (inverse : v * vi = 1) (trace : v + vi = w) :
    (a + v * b) * (a + vi * b) = a * a + w * a * b + b * b := by
  grind
#print axioms norm_identity
end SignedReciprocal
