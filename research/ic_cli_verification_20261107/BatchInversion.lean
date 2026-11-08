import Lean.Elab.Tactic.Omega

/-!
The algebraic step behind `Curve.addBatch`'s Montgomery inversion.

`batchInv` builds prior products, inverts the total once, then walks back:

    inverse_i = (x_0 * ... * x_i)^{-1}
    out_i     = inverse_i * prior_i

For the recovered value to be `x_i^{-1}`, the step must satisfy

    x_i * ((prior_i * x_i)^{-1} * prior_i) = 1.

This file proves that identity in any commutative group. The Rust/Python
field is tested against the scalar inversion path; this proof covers the
inversion algorithm's algebra, not the field representation.
-/

namespace BatchInversion

structure CommGroup where
  G : Type
  mul : G → G → G
  one : G
  inv : G → G
  assoc : ∀ a b c, mul (mul a b) c = mul a (mul b c)
  comm : ∀ a b, mul a b = mul b a
  one_mul : ∀ a, mul one a = a
  inv_mul_cancel : ∀ a, mul a (inv a) = one
  inv_mul : ∀ a b, inv (mul a b) = mul (inv b) (inv a)

namespace CommGroup

variable (g : CommGroup)

theorem mul_one (a : g.G) : g.mul a g.one = a := by
  rw [g.comm, g.one_mul]

/-- Inverting `prior * x` and multiplying back by `prior` recovers
`x^{-1}`. -/
theorem batch_step (prior x : g.G) :
    g.mul (g.inv (g.mul prior x)) prior = g.inv x := by
  rw [g.inv_mul]
  rw [g.assoc]
  rw [g.comm (g.inv prior) prior]
  rw [g.inv_mul_cancel]
  rw [g.mul_one]

/-- The recovered value really is an inverse: multiplying by `x` gives one. -/
theorem batch_step_invariant (prior x : g.G) :
    g.mul x (g.mul (g.inv (g.mul prior x)) prior) = g.one := by
  rw [batch_step, g.inv_mul_cancel]

end CommGroup

end BatchInversion

#print axioms BatchInversion.CommGroup.batch_step
#print axioms BatchInversion.CommGroup.batch_step_invariant
