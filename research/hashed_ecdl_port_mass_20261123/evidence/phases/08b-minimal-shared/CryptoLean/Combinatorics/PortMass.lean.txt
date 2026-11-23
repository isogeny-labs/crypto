import Mathlib.Algebra.BigOperators.Group.Finset.Basic
import Mathlib.Algebra.Order.Group.Multiset
import Mathlib.Data.Fintype.Basic
import Mathlib.Data.Fintype.Card

/-!
# Grouped endpoint-port accounting

This is a composition of existing multiset counting/cardinality laws.
Multiplicities retain parallel edges. A consumer must bind its actual
constructor, packet coverage and label extraction separately; the laws do
not bound coefficient work, tensor entries or complete algorithm cost.
-/

open scoped BigOperators
namespace CryptoLean.PortMass

/-- Partitioning both endpoint tags over the full label alphabet counts
exactly two ports per edge, with all multiplicities retained. -/
theorem grouped_port_mass {G : Type*} [Fintype G] [DecidableEq G]
    (edges : Multiset (G × G)) :
    (∑ g : G, ((edges.map Prod.fst).count g + (edges.map Prod.snd).count g)) =
      2 * edges.card := by
  calc
    _ = ∑ g : G, (edges.map Prod.fst + edges.map Prod.snd).count g := by
      simp only [Multiset.count_add]
    _ = (edges.map Prod.fst + edges.map Prod.snd).card :=
      Multiset.sum_count_eq_card (by intro g _; exact Finset.mem_univ g)
    _ = edges.card + edges.card := by
      rw [Multiset.card_add, Multiset.card_map, Multiset.card_map]
    _ = 2 * edges.card := (Nat.two_mul _).symm

/-- A common nonnegative integer weight factors through the endpoint mass.
A precision weight is an accounting quantity, not measured bit operations. -/
theorem grouped_port_weight {G : Type*} [Fintype G] [DecidableEq G]
    (edges : Multiset (G × G)) (p : Nat) :
    (∑ g : G, (((edges.map Prod.fst).count g + (edges.map Prod.snd).count g) * p)) =
      2 * edges.card * p := by
  calc
    _ = ∑ g : G, (p • (edges.map Prod.fst + edges.map Prod.snd)).count g := by
      apply Finset.sum_congr rfl
      intro g _
      rw [Multiset.count_nsmul, Multiset.count_add, Nat.mul_comm]
    _ = (p • (edges.map Prod.fst + edges.map Prod.snd)).card :=
      Multiset.sum_count_eq_card (by intro g _; exact Finset.mem_univ g)
    _ = p * (edges.card + edges.card) := by
      rw [Multiset.card_nsmul, Multiset.card_add, Multiset.card_map, Multiset.card_map]
    _ = (edges.card + edges.card) * p := Nat.mul_comm _ _
    _ = 2 * edges.card * p := by rw [← Nat.two_mul]

/-- Old vertices partition once by label; a constant packet description
is replicated in every group, including groups with no retained ports. -/
theorem grouped_vertex_mass {G : Type*} [Fintype G] [DecidableEq G]
    (oldLabels : Multiset G) (packets : Nat) :
    (∑ g : G, (oldLabels.count g + packets)) =
      oldLabels.card + Fintype.card G * packets := by
  rw [Finset.sum_add_distrib]
  rw [Multiset.sum_count_eq_card (by intro g _; exact Finset.mem_univ g)]
  simp

end CryptoLean.PortMass
