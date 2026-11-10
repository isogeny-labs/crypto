import Mathlib

/-! Layer 4c: the reduction step of the anomalous-curve attack.

For an anomalous curve `#E(F_p) = p`, the group has prime order, so every
nonzero point has order `p`.  The Smart--Satoh--Araki attack supplies an
explicit *computable* injective homomorphism from `E(F_p)` to the additive
group `F_p` (the p-adic elliptic logarithm).  Once that map exists, the DLP is
a linear equation in `F_p`.

This file proves the two reduction facts:

* a group of cardinality `p` has no elements of order `1` other than zero, so
  every nonzero point has order exactly `p`;
* an injective additive map `L : G →+ ZMod p` with `L P ≠ 0` turns
  `Q = x·P` into `x = L Q / L P`, and `x` is read off as the residue's value.

The construction and injectivity of the p-adic logarithm remain the open part
of Layer 4c. -/

namespace ECDLPAnomalous

/-- In a group of prime cardinality `p`, every nonzero element has order `p`. -/
theorem anomalous_order_of_card {G : Type*} [AddCommGroup G] [Fintype G]
    {p : ℕ} [Fact p.Prime] (hcard : Fintype.card G = p) {P : G} (hP : P ≠ 0) :
    addOrderOf P = p := by
  have hdvd : addOrderOf P ∣ p := by
    rw [← hcard]
    exact addOrderOf_dvd_card
  rcases (Fact.out : p.Prime).eq_one_or_self_of_dvd _ hdvd with h1 | hp
  · exact absurd (AddMonoid.addOrderOf_eq_one_iff.mp h1) hP
  · exact hp

/-- **Logarithm reduction.**  An additive map to `ZMod p` sends the DLP
`Q = x·P` to the linear equation `x·L(P) = L(Q)`. -/
theorem log_reduction {G : Type*} [AddCommGroup G] {p : ℕ} [Fact p.Prime]
    (L : G →+ ZMod p) {P Q : G} {x : ℕ} (hQ : Q = x • P) (hLP : L P ≠ 0) :
    (x : ZMod p) = L Q / L P := by
  have hx : L Q = (x : ZMod p) * L P := by
    rw [hQ, L.map_nsmul, nsmul_eq_mul]
  rw [eq_div_iff hLP]
  exact hx.symm

/-- **Anomalous-curve recovery.**  If `G` has cardinality `p`, `L` has trivial
kernel and `P ≠ 0`, then the exponent `x < p` in `Q = x·P` is the residue of
`L Q / L P`.  Only the existence of `L` (the p-adic elliptic logarithm) is
assumed. -/
theorem anomalous_recover {G : Type*} [AddCommGroup G] [Fintype G]
    {p : ℕ} [Fact p.Prime] (_hcard : Fintype.card G = p) (L : G →+ ZMod p)
    (hL : ∀ z : G, L z = 0 → z = 0)
    {P Q : G} {x : ℕ} (hP : P ≠ 0) (hQ : Q = x • P) (hx : x < p) :
    x = (L Q / L P).val := by
  have hLP : L P ≠ 0 := fun h => hP (hL P h)
  have h := log_reduction L hQ hLP
  calc x = (x : ZMod p).val := (ZMod.val_natCast_of_lt hx).symm
    _ = (L Q / L P).val := by rw [h]

/-- The anomalous condition forces every nonzero point to be a generator:
its order is the full group order. -/
theorem anomalous_generator {G : Type*} [AddCommGroup G] [Fintype G]
    {p : ℕ} [Fact p.Prime] (hcard : Fintype.card G = p) {P : G} (hP : P ≠ 0) :
    addOrderOf P = Fintype.card G := by
  rw [hcard]
  exact anomalous_order_of_card hcard hP

end ECDLPAnomalous
