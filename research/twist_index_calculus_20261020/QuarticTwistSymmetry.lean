/-!
Quartic twist structural lemmas. These prove abstract consequences of the
endomorphism identities used by the experiment. They do not construct Fp⁴,
the twist curve, or the transported Frobenius; those are the explicit
unformalized bridge stated in QUARTIC_TWIST24_REPORT.md.
-/
namespace QuarticTwistSymmetry

def four {α : Type} (f : α → α) (x : α) : α := f (f (f (f x)))

/-- If the fourth iterate of a map is negation, its eighth iterate is the
identity when negation is involutive. -/
theorem eighth_from_fourth {α : Type} (psi neg : α → α)
    (hfour : ∀ x, four psi x = neg x)
    (hneg : ∀ x, neg (neg x) = x) (P : α) :
    four psi (four psi P) = P := by
  rw [hfour P, hfour (neg P), hneg P]

/-- The two scalar endomorphisms compose with product eigenvalue on a
cyclic subgroup. The hypotheses expose the group-action bridge. -/
structure Action (α : Type) where
  smul : Int → α → α
  frob : α → α
  cubic : α → α
  scalar_mul : ∀ a b : Int, ∀ x : α,
    smul a (smul b x) = smul (a * b) x
  frob_scalar : ∀ a : Int, ∀ x : α,
    frob (smul a x) = smul a (frob x)

theorem composed_eigenvalue {α : Type} (M : Action α) (P : α)
    (lf lc : Int) (hf : M.frob P = M.smul lf P)
    (hc : M.cubic P = M.smul lc P) :
    M.frob (M.cubic P) = M.smul (lf * lc) P := by
  calc
    M.frob (M.cubic P) = M.frob (M.smul lc P) := by rw [hc]
    _ = M.smul lc (M.frob P) := M.frob_scalar lc P
    _ = M.smul lc (M.smul lf P) := by rw [hf]
    _ = M.smul (lc * lf) P := M.scalar_mul lc lf P
    _ = M.smul (lf * lc) P := by rw [Int.mul_comm lc lf]

/-- Multiplying a relation by a common eigenvalue only rescales its folded
row; the transformed relation does not create an independent row. -/
theorem row_covariance (a b lf lc : Int) :
    lf * (a + b * lc) = a * lf + (b * lf) * lc := by
  calc
    lf * (a + b * lc) = lf * a + lf * (b * lc) := Int.mul_add lf a (b * lc)
    _ = a * lf + (b * lf) * lc := by
      rw [Int.mul_comm lf a, ← Int.mul_assoc lf b lc,
          Int.mul_comm lf b, Int.mul_assoc b lf lc]

#print axioms eighth_from_fourth
#print axioms composed_eigenvalue
#print axioms row_covariance
end QuarticTwistSymmetry
