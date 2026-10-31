/-!
The abstract group-action part of the GLS twist relation fold. This file
does not formalize the finite-field formula for the particular curve:
that bridge is recorded separately in GLS_SYMMETRY_REPORT.md.
-/
namespace GLSSymmetry

/-- Only the standard integer scalar-action laws and equivariance needed by
the coefficient-folding theorem are assumed. -/
structure Action (α : Type) where
  add : α → α → α
  smul : Int → α → α
  psi : α → α
  scalar_add : ∀ a b : Int, ∀ x : α,
    add (smul a x) (smul b x) = smul (a + b) x
  scalar_mul : ∀ a b : Int, ∀ x : α,
    smul a (smul b x) = smul (a * b) x
  psi_scalar : ∀ a : Int, ∀ x : α,
    psi (smul a x) = smul a (psi x)

/-- An eigenvalue checked on a cyclic generator determines its action on
every point represented as an integer multiple of that generator. -/
theorem eigenvalue_on_cyclic_subgroup {α : Type} (M : Action α)
    (G P : α) (k lam : Int)
    (hP : P = M.smul k G)
    (hG : M.psi G = M.smul lam G) :
    M.psi P = M.smul lam P := by
  rw [hP]
  calc
    M.psi (M.smul k G) = M.smul k (M.psi G) := M.psi_scalar k G
    _ = M.smul k (M.smul lam G) := by rw [hG]
    _ = M.smul (k * lam) G := M.scalar_mul k lam G
    _ = M.smul (lam * k) G := by rw [Int.mul_comm k lam]
    _ = M.smul lam (M.smul k G) := (M.scalar_mul lam k G).symm

/-- Two physical columns P and psi(P) fold to one column with coefficient
a + b*lam. This is the algebraic relation-matrix identity used by the run. -/
theorem fold_relation_pair {α : Type} (M : Action α)
    (P : α) (a b lam : Int)
    (h : M.psi P = M.smul lam P) :
    M.add (M.smul a P) (M.smul b (M.psi P))
      = M.smul (a + b * lam) P := by
  calc
    M.add (M.smul a P) (M.smul b (M.psi P))
        = M.add (M.smul a P) (M.smul b (M.smul lam P)) := by rw [h]
    _ = M.add (M.smul a P) (M.smul (b * lam) P) := by
          rw [M.scalar_mul b lam P]
    _ = M.smul (a + b * lam) P := M.scalar_add a (b * lam) P

/-- The subgroup and row identities compose without assuming that each
factor point has a separately supplied eigenvalue. -/
theorem fold_from_generator {α : Type} (M : Action α)
    (G P : α) (k a b lam : Int)
    (hP : P = M.smul k G)
    (hG : M.psi G = M.smul lam G) :
    M.add (M.smul a P) (M.smul b (M.psi P))
      = M.smul (a + b * lam) P :=
  fold_relation_pair M P a b lam
    (eigenvalue_on_cyclic_subgroup M G P k lam hP hG)

#print axioms eigenvalue_on_cyclic_subgroup
#print axioms fold_relation_pair
#print axioms fold_from_generator
end GLSSymmetry
