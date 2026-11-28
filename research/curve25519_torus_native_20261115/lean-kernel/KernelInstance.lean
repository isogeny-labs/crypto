import CryptoLean.Group.SingleRelation
import Mathlib.Data.Matrix.Mul
import Mathlib.Algebra.Field.ZMod
import Mathlib.LinearAlgebra.Matrix.ToLin
import Mathlib.GroupTheory.Index

set_option autoImplicit false
open scoped BigOperators
open Matrix

example {K I : Type*} [Field K] (s : Finset I) (a b v : I → K) (x : K)
    (h : ∑ i ∈ s, v i * (a i + x * b i) = 0)
    (hb : (∑ i ∈ s, v i * b i) ≠ 0) :
    -(∑ i ∈ s, v i * a i) / (∑ i ∈ s, v i * b i) = x :=
  CryptoLean.SingleRelation.recover_from_relation s a b v x h hb

namespace Curve25519KernelInstance
def scalarOrder : ℕ := 2^252 + 27742317777372353535851937790883648493

variable {r b : ℕ} [Fact (Nat.Prime scalarOrder)]

theorem extract_scalar
    (C : Matrix (Fin r) (Fin b) (ZMod scalarOrder))
    (z : Fin b → ZMod scalarOrder)
    (a tag v : Fin r → ZMod scalarOrder) (x : ZMod scalarOrder)
    (hrows : C *ᵥ z = fun i => a i + x * tag i)
    (hker : v ᵥ* C = 0)
    (hb : (∑ i, v i * tag i) ≠ 0) :
    -(∑ i, v i * a i) / (∑ i, v i * tag i) = x := by
  have hz : v ⬝ᵥ (C *ᵥ z) = 0 := by
    rw [Matrix.dotProduct_mulVec, hker, zero_dotProduct]
  have h : (∑ i, v i * (a i + x * tag i)) = 0 := by
    simpa only [hrows, dotProduct] using hz
  exact CryptoLean.SingleRelation.recover_from_relation Finset.univ a tag v x h hb

theorem mixed_label_fiber (x : ZMod scalarOrder) (u : Fin r → ZMod scalarOrder) :
    Nat.card {ab : (Fin r → ZMod scalarOrder) × (Fin r → ZMod scalarOrder) //
      ∀ i, ab.1 i + x * ab.2 i = u i} = scalarOrder^r := by
  simpa only [Nat.card_eq_fintype_card, ZMod.card, Fintype.card_fin] using
    (CryptoLean.SingleRelation.mixed_label_fiber_card x u)

theorem annihilator_card (v : Fin r → ZMod scalarOrder) (hv : v ≠ 0) :
    Nat.card {w : Fin r → ZMod scalarOrder // v ⬝ᵥ w = 0} * scalarOrder =
      scalarOrder^r := by
  let f := (dotProductBilin (ZMod scalarOrder) (ZMod scalarOrder) v).toAddMonoidHom
  have hsurj : Function.Surjective f := by
    obtain ⟨i, hi⟩ := Function.ne_iff.mp hv
    have hi : v i ≠ 0 := hi
    intro y
    refine ⟨Pi.single i (y / v i), ?_⟩
    change v ⬝ᵥ Pi.single i (y / v i) = y
    rw [dotProduct_single, mul_comm, div_mul_cancel₀ y hi]
  have hc := AddSubgroup.card_ker_mul_card_of_surjective hsurj
  change Nat.card {w : Fin r → ZMod scalarOrder // v ⬝ᵥ w = 0} *
    Nat.card (ZMod scalarOrder) = Nat.card (Fin r → ZMod scalarOrder) at hc
  simpa only [Nat.card_fun, Nat.card_eq_fintype_card, Fintype.card_fun,
    ZMod.card, Fintype.card_fin] using hc

#print axioms extract_scalar
#print axioms mixed_label_fiber
#print axioms annihilator_card
end Curve25519KernelInstance
