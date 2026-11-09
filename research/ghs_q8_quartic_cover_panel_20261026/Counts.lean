import Init

/-!
The finite arithmetic and abstract pullback argument for the order-554 panel.
The geometric curve identities and quotient equations are certified by the
Sage and Magma receipts, not by this Lean file.
-/

namespace GhsQ8QuarticCoverPanel

theorem sourceOrder : 2 * 277 = 554 := by decide
theorem oddPrime : Nat.Coprime 2 277 := by decide
theorem simpleRootDerivative : 6 + 3 * 41 = 129 := by decide
theorem simpleRoot : Nat.Coprime 277 129 := by decide
theorem fourTrivialOneSplit : 4 + 1 = 5 := by decide
theorem order554OutsideDirect : 324 + 9 = 333 := by decide
theorem outsideTwoKnownRoutes : 333 + 689 = 1022 := by decide
theorem longestTwoIsogenyPathDegree : 2 ^ 8 = 256 := by decide

theorem oddOrderDouble (x : Fin 277) (hx : x.val ≠ 0) :
    (x.val + x.val) % 277 ≠ 0 := by
  intro h
  have hb := x.isLt
  omega

theorem pullbackNonzero
    {A B : Type} [Zero A] [Zero B] [Add A]
    (pull : A → B) (push : B → A)
    (hpushzero : push 0 = 0)
    (hcomp : ∀ x, push (pull x) = x + x)
    (x : A) (hx : x + x ≠ 0) : pull x ≠ 0 := by
  intro hzero
  have hc := hcomp x
  rw [hzero, hpushzero] at hc
  exact hx hc.symm

theorem cyclicTraceTripleNonzero
    {A B C : Type} [Zero A] [Zero B] [Zero C]
    (x : A) (f : A → B) (g : A → C) (hx : x ≠ 0) :
    (x, f x, g x) ≠ (0, 0, 0) := by
  intro hzero
  exact hx (congrArg Prod.fst hzero)

theorem cyclicTraceThroughInjectiveProduct
    {A B C J : Type} [Zero A] [Zero B] [Zero C] [Zero J]
    (x : A) (f : A → B) (g : A → C)
    (combine : A × B × C → J)
    (hinj : Function.Injective combine)
    (hzero : combine (0, 0, 0) = 0)
    (hx : x ≠ 0) : combine (x, f x, g x) ≠ 0 := by
  intro h
  have ht : (x, f x, g x) = (0, 0, 0) := hinj (h.trans hzero.symm)
  exact hx (congrArg Prod.fst ht)

end GhsQ8QuarticCoverPanel
