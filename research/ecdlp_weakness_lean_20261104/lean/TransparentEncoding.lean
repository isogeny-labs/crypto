import Mathlib

/-!
# Shoup's bound is a statement about encodings, not about groups

Corpus targets: `GAP-PA-GGM-TRANSPARENT-ENCODING`, `CLAIM-BK-GG-BSGS-OPTIMAL`,
`LB-SHOUP-DLP`. Shoup's theorem lower-bounds the discrete logarithm for
algorithms that see group elements only through a random injective encoding.
The overbroad reading "every algorithm needs Ω(√r) group operations in a group
of prime order r" is false: the cyclic group `ZMod r` with the identity
encoding has a discrete-logarithm algorithm that performs **no** group
operation at all, because the encoding already is the logarithm.

* `dlog_identity_encoding`: in `ZMod r` with generator `1`, the logarithm of
  `h` is `h.val`, read off the encoding.
* `dlog_cyclic_transparent`: for any additive cyclic group with a generator `g`
  and the "transparent" encoding `enc (x • g) = x`, the logarithm is the
  encoding.

So any optimality claim for BSGS or rho that cites Shoup is relative to the
generic (random-encoding) model; it says nothing about algorithms that use a
specific representation, which is where index calculus lives.
-/

namespace TransparentEncoding

theorem dlog_identity_encoding (r : ℕ) [NeZero r] (h : ZMod r) :
    (h.val : ℤ) • (1 : ZMod r) = h := by
  simp

/-- A transparent encoding: a left inverse of `x ↦ x • g` on the residues. -/
theorem dlog_cyclic_transparent {G : Type*} [AddCommGroup G] (g : G) (r : ℕ) [NeZero r]
    (enc : G → ZMod r) (henc : ∀ x : ZMod r, enc (x.val • g) = x) (x : ZMod r) :
    (enc (x.val • g)).val • g = x.val • g := by
  rw [henc]

/-- The zero-operation algorithm: output `enc h`. Its correctness needs only the
encoding, never a group operation; the "generic algorithm" hypothesis of
Shoup's bound is what excludes it. -/
theorem zero_operation_dlog {G : Type*} [AddCommGroup G] (g : G) (r : ℕ) [NeZero r]
    (enc : G → ZMod r) (henc : ∀ x : ZMod r, enc (x.val • g) = x) :
    ∀ x : ZMod r, ∃ y : ZMod r, y = enc (x.val • g) ∧ y.val • g = x.val • g :=
  fun x => ⟨enc (x.val • g), rfl, by rw [henc]⟩

end TransparentEncoding
