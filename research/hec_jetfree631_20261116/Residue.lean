import Mathlib
namespace JetFree
 theorem eval_residue {R : Type*} [CommRing R] (I : Ideal R)
    (p : Polynomial R) (y u t : R) (hu : u ∈ I) :
    (Ideal.Quotient.mk I) (p.eval (y + u * t)) =
      (Ideal.Quotient.mk I) (p.eval y) := by
  let f := Ideal.Quotient.mk I
  have hz : f u = 0 := Ideal.Quotient.eq_zero_iff_mem.mpr hu
  calc
    f (p.eval (y + u * t)) = p.eval₂ f (f (y + u * t)) :=
      (Polynomial.eval₂_at_apply f (y + u * t)).symm
    _ = p.eval₂ f (f y) := by simp only [map_add, map_mul, hz, zero_mul, add_zero]
    _ = f (p.eval y) := Polynomial.eval₂_at_apply f y
end JetFree
#print axioms JetFree.eval_residue
