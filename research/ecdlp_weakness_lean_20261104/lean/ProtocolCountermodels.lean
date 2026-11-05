import Mathlib

/-!
# Countermodels for the protocol-layer Priority 0 queue

Corpus targets: `GAP-PA-FS-ZERO-HASH`, `GAP-PA-PEDERSEN-RELATION`,
`GAP-PA-COFACTOR`, `GAP-PA-DLP-CDH-DDH-DIRECTION`, `GAP-PA-RSR-EXACT-SUBGROUP`,
`GAP-PA-ZK-SEPARATION` (ported from crypto-autoresearcher's assumption attack
corpus). Each section refutes the overbroad claim with a model satisfying its
premises and, where one exists, proves the repair.

These are algebraic statements about groups and modules. They make no claim about
any implementation; a security consequence for a named protocol needs a separate
refinement argument.
-/

namespace ProtocolCountermodels

/-! ## FS-ZERO-HASH: special soundness does not give Fiat–Shamir security -/

section FS

variable {A : Type*} [AddCommGroup A]

/-- Schnorr verification: `s • P = R + c • X`. -/
def schnorrAccepts (P X R : A) (c s : ℤ) : Prop := s • P = R + c • X

/-- With a challenge oracle that always returns `0`, the transcript
`(R, c, s) = (s • P, 0, s)` is accepted for **every** public key `X`: the forger
never uses a witness. Special soundness (two transcripts, distinct challenges)
cannot rule this out, because the oracle never produces a second challenge. -/
theorem zero_hash_forgery (P X : A) (s : ℤ) :
    schnorrAccepts P X (s • P) ((fun _ => (0 : ℤ)) (s • P)) s := by
  simp [schnorrAccepts]

end FS

/-! ## PEDERSEN-RELATION: binding is computational, not linear independence -/

section Pedersen

variable {R M : Type*} [CommRing R] [AddCommGroup M] [Module R M]

/-- Pedersen commitment `m • G + r • H`. -/
def commit (G H : M) (m r : R) : M := m • G + r • H

/-- If a relation `H = t • G` is known, every commitment has a second opening
`(m + t, r - 1)`; it differs from `(m, r)` whenever `t ≠ 0`. -/
theorem pedersen_alternate_opening (G : M) (t m r : R) :
    commit G (t • G) m r = commit G (t • G) (m + t) (r - 1) := by
  simp only [commit, smul_smul, ← add_smul]
  congr 1
  ring

end Pedersen

/-! ## COFACTOR: clearing by `h` is a bijection on the prime subgroup only when `r ∤ h` -/

/-- In `ZMod 9` the order-3 subgroup is `{0, 3, 6}`. Multiplication by the
cofactor `3` sends all of it to `0`: when `r ∣ h` (here `r² ∣ #G`), cofactor
clearing destroys the prime-order subgroup instead of preserving it. -/
theorem cofactor_kills_subgroup : ∀ x ∈ ({0, 3, 6} : Finset (ZMod 9)), (3 : ZMod 9) * x = 0 := by
  decide

/-- Repair: on a group of prime order `r`, multiplication by `h` with `r ∤ h` is
a bijection. -/
theorem cofactor_bijective {r : ℕ} [Fact r.Prime] (h : ZMod r) (hh : h ≠ 0) :
    Function.Bijective (fun x : ZMod r => h * x) := by
  refine ⟨fun a b hab => ?_, fun y => ⟨h⁻¹ * y, ?_⟩⟩
  · simpa [hh] using hab
  · simp [← mul_assoc, mul_inv_cancel₀ hh]

/-! ## DLP-CDH-DDH: a nondegenerate pairing decides DDH -/

/-- A symmetric bilinear pairing on a one-dimensional `K`-space with
`e(P, P) = u ≠ 0`. With `P = 1`, the DDH tuple `(aP, bP, cP)` satisfies
`e(aP, bP) = e(P, cP)` exactly when `c = ab`: one pairing evaluation decides DDH,
whatever the cost of CDH or DLP in the same group. -/
theorem pairing_decides_ddh {K : Type*} [Field K] (u : K) (hu : u ≠ 0) (a b c : K) :
    (a * b) * u = (1 * c) * u ↔ c = a * b := by
  constructor
  · intro h
    have := mul_right_cancel₀ hu h
    simpa [eq_comm] using this
  · intro h; simp [h]

/-! ## RSR-EXACT-SUBGROUP: blinding must stay in the prime subgroup -/

/-- Designated order-3 subgroup of `ZMod 2 × ZMod 3`. -/
def inSub (x : ZMod 2 × ZMod 3) : Prop := x.1 = 0

instance : DecidablePred inSub := fun x => inferInstanceAs (Decidable (x.1 = 0))

/-- Blinding a subgroup instance by an ambient random element can leave the
subgroup, so the blinded instance is no longer an instance of the problem. -/
theorem ambient_blinding_leaves_subgroup :
    ∃ q a : ZMod 2 × ZMod 3, inSub q ∧ ¬ inSub (q + a) := by
  decide

/-- Repair: in a cyclic group of prime order, blinding by a uniformly random
group element is a bijection, so the blinded instance is uniform. -/
theorem subgroup_blinding_bijective {r : ℕ} (q : ZMod r) :
    Function.Bijective (fun a : ZMod r => q + a) :=
  (Equiv.addLeft q).bijective

/-! ## ZK-SEPARATION: completeness and zero knowledge do not give soundness -/

/-- An interactive proof abstracted to its verdict: `accepts x` for statement `x`. -/
structure Protocol (X : Type*) where
  accepts : X → Prop

/-- Always-accept protocol. -/
def alwaysAccept (X : Type*) : Protocol X := ⟨fun _ => True⟩

/-- It is complete for any language `L` (every true statement is accepted) and its
transcripts are trivially simulatable (they carry no witness), yet it is unsound
for every language with a false statement. -/
theorem always_accept_complete_unsound {X : Type*} (L : X → Prop) (x₀ : X) (hx₀ : ¬ L x₀) :
    (∀ x, L x → (alwaysAccept X).accepts x) ∧ ∃ x, ¬ L x ∧ (alwaysAccept X).accepts x :=
  ⟨fun _ _ => trivial, ⟨x₀, hx₀, trivial⟩⟩

end ProtocolCountermodels
