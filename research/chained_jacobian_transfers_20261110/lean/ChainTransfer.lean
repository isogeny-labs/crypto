import Init.Grind.Module.Basic
import Init.Grind.Ring.Basic

/-!
Structural obligations for chains. Geometry is deliberately explicit input:
these theorems neither construct a Jacobian nor prove a norm identity for it.
Core-only, standard algebra instances; no axioms encoding research conclusions.
The Bezout argument reuses the method of the existing GhsSubgroupTransfer proof.
-/
set_option autoImplicit false

namespace ChainedTransfer
open Lean.Grind

structure Transfer (A B : Type) [IntModule A] [IntModule B] where
  apply : A → B
  zero : apply 0 = 0
  add : ∀ x y, apply (x + y) = apply x + apply y
  scalar : ∀ (n : Int) x, apply (n • x) = n • apply x

variable {A B C : Type} [IntModule A] [IntModule B] [IntModule C]

def Transfer.comp (g : Transfer B C) (f : Transfer A B) : Transfer A C where
  apply := fun x => g.apply (f.apply x)
  zero := by rw [f.zero, g.zero]
  add := by intro x y; rw [f.add, g.add]
  scalar := by intro n x; rw [f.scalar, g.scalar]

/-- Dual norm identities multiply along two actual homomorphisms. -/
theorem two_hop_norm
    (f : Transfer A B) (fdual : Transfer B A)
    (g : Transfer B C) (gdual : Transfer C B)
    (d e : Int)
    (hf : ∀ x, fdual.apply (f.apply x) = d • x)
    (hg : ∀ y, gdual.apply (g.apply y) = e • y) (x : A) :
    fdual.apply (gdual.apply (g.apply (f.apply x))) = (e*d) • x := by
  rw [hg, fdual.scalar, hf, IntModule.mul_zsmul]

/-- A Bezout certificate gives a left recovery map, hence injectivity.
`A` is the actual source subgroup, so the exponent assumption is scoped to it.
-/
theorem injective_with_recovery
    (f : Transfer A B) (back : Transfer B A) (d r u v : Int)
    (hr : ∀ x : A, r • x = 0)
    (hbez : u*d + v*r = 1)
    (hnorm : ∀ x, back.apply (f.apply x) = d • x) :
    Function.Injective f.apply := by
  have recovery (x : A) : u • back.apply (f.apply x) = x := by
    calc
      u • back.apply (f.apply x) = (u*d) • x := by rw [hnorm, IntModule.mul_zsmul]
      _ = (u*d) • x + (v*r) • x := by rw [IntModule.mul_zsmul v r, hr, IntModule.zsmul_zero, AddCommMonoid.add_zero]
      _ = (u*d + v*r) • x := (IntModule.add_zsmul _ _ _).symm
      _ = x := by rw [hbez, IntModule.one_zsmul]
  intro x y h
  calc
    x = u • back.apply (f.apply x) := (recovery x).symm
    _ = u • back.apply (f.apply y) := by rw [h]
    _ = y := recovery y

theorem two_hop_injective
    (f : Transfer A B) (fdual : Transfer B A)
    (g : Transfer B C) (gdual : Transfer C B)
    (d e r u v : Int)
    (hf : ∀ x, fdual.apply (f.apply x) = d • x)
    (hg : ∀ y, gdual.apply (g.apply y) = e • y)
    (hr : ∀ x : A, r • x = 0)
    (hbez : u*(e*d) + v*r = 1) :
    Function.Injective (fun x => g.apply (f.apply x)) := by
  exact injective_with_recovery (g.comp f) (fdual.comp gdual)
    (e*d) r u v hr hbez (two_hop_norm f fdual g gdual d e hf hg)

/-- A DLP relation is preserved without its scalar being known to the map. -/
theorem two_hop_scalar_relation
    (f : Transfer A B) (g : Transfer B C) (p : A) (k : Int) :
    g.apply (f.apply (k • p)) = k • g.apply (f.apply p) := by
  rw [f.scalar, g.scalar]

/-- An injective group transfer preserves and reflects n-torsion. In
particular, an isomorphism identifies rational 2-torsion subgroups. Their
finite cardinalities must therefore agree; counting them is a separate input.
-/
theorem injective_torsion_iff
    (f : Transfer A B) (hf : Function.Injective f.apply) (n : Int) (x : A) :
    n • f.apply x = 0 ↔ n • x = 0 := by
  constructor
  · intro h
    apply hf
    rw [f.scalar, f.zero]
    exact h
  · intro h
    rw [← f.scalar, h, f.zero]

/-- Invariant fibers projected anti-invariantly yield two-torsion. This
alone does not imply zero: the odd-order certificate is used below. -/
theorem anti_invariant_twice_zero
    (f : Transfer A B) (sigma : Transfer B B) (g : Transfer B C)
    (hinv : ∀ x, sigma.apply (f.apply x) = f.apply x)
    (hanti : ∀ y, g.apply (sigma.apply y) = -g.apply y) (x : A) :
    (2 : Int) • g.apply (f.apply x) = 0 := by
  have h : g.apply (f.apply x) = -g.apply (f.apply x) := by
    calc
      g.apply (f.apply x) = g.apply (sigma.apply (f.apply x)) := congrArg g.apply (hinv x).symm
      _ = -g.apply (f.apply x) := hanti _
  have htwo (y : C) : (2 : Int) • y = y+y := by
    have hh := IntModule.add_zsmul (1 : Int) (1 : Int) y
    have ht : (2 : Int) = 1+1 := by decide
    rw [ht]
    simpa only [IntModule.one_zsmul] using hh
  rw [htwo]
  calc
    g.apply (f.apply x) + g.apply (f.apply x) = -g.apply (f.apply x) + g.apply (f.apply x) :=
      congrArg (fun z => z + g.apply (f.apply x)) h
    _ = 0 := AddCommGroup.neg_add_cancel _

theorem anti_invariant_odd_source_zero
    (f : Transfer A B) (sigma : Transfer B B) (g : Transfer B C)
    (hinv : ∀ x, sigma.apply (f.apply x) = f.apply x)
    (hanti : ∀ y, g.apply (sigma.apply y) = -g.apply y)
    (r u v : Int) (hr : ∀ x : A, r • x = 0)
    (hbez : u*2 + v*r = 1) (x : A) : g.apply (f.apply x) = 0 := by
  have htwo := anti_invariant_twice_zero f sigma g hinv hanti x
  have horder : r • g.apply (f.apply x) = 0 := by
    rw [← g.scalar, ← f.scalar, hr, f.zero, g.zero]
  calc
    g.apply (f.apply x) = (1 : Int) • g.apply (f.apply x) := (IntModule.one_zsmul _).symm
    _ = (u*2 + v*r) • g.apply (f.apply x) := by rw [hbez]
    _ = 0 := by
      rw [IntModule.add_zsmul, IntModule.mul_zsmul, IntModule.mul_zsmul,
          htwo, horder, IntModule.zsmul_zero, IntModule.zsmul_zero, AddCommMonoid.zero_add]

end ChainedTransfer

namespace CharacteristicBoundary
open Lean.Grind
attribute [local instance] Semiring.natCast

/-- Only additive and unital properties are needed for the obstruction. -/
structure UnitalAdditive (K L : Type) [Semiring K] [Semiring L] where
  apply : K → L
  zero : apply 0 = 0
  one : apply 1 = 1
  add : ∀ x y, apply (x+y) = apply x + apply y

variable {K L : Type} [Semiring K] [Semiring L]

theorem map_nat (f : UnitalAdditive K L) (n : Nat) : f.apply (n : K) = (n : L) := by
  induction n with
  | zero => simpa only [Semiring.natCast_zero] using f.zero
  | succ n ih => rw [Semiring.natCast_succ, f.add, ih, f.one, Semiring.natCast_succ]

/-- A unital field/ring map cannot change characteristic. The residue
condition follows for distinct prime characteristics, but says nothing about
arbitrary encodings of cyclic ECDLP subgroups. No such algorithm is ruled out.
-/
theorem no_unital_map_when_characteristic_not_divides
    (p q : Nat) [IsCharP K p] [IsCharP L q] (hres : p % q ≠ 0) :
    ¬ Nonempty (UnitalAdditive K L) := by
  intro ⟨f⟩
  have hp : (p : K) = 0 := (IsCharP.natCast_eq_zero_iff p p).mpr (Nat.mod_self p)
  have hl : (p : L) = 0 := by rw [← map_nat f p, hp, f.zero]
  exact hres ((IsCharP.natCast_eq_zero_iff q p).mp hl)

end CharacteristicBoundary

#print axioms ChainedTransfer.two_hop_norm
#print axioms ChainedTransfer.injective_with_recovery
#print axioms ChainedTransfer.two_hop_injective
#print axioms ChainedTransfer.two_hop_scalar_relation
#print axioms ChainedTransfer.injective_torsion_iff
#print axioms ChainedTransfer.anti_invariant_twice_zero
#print axioms ChainedTransfer.anti_invariant_odd_source_zero
#print axioms CharacteristicBoundary.map_nat
#print axioms CharacteristicBoundary.no_unital_map_when_characteristic_not_divides
