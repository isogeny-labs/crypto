import TransferBound

/-!
# Transfer along a chain of isogenies, and the twist-order arithmetic

Companion to `research/deep_volcano_prime_gap_20261022/README.md`, §Theorems.

`TransferBound` (2026-10-10 conductor-gap study) proves the one-edge case:
an evaluable injective homomorphism `φ` turns any always-correct solver on
the target into one on the source at two evaluations' extra cost. The
curves of this study are joined by chains: at most `2d` Vélu 2-isogenies
and at most two Vélu `N`-isogenies. This file proves:

- **Composition.** The composite of two evaluable injective homomorphisms
  is one, and its evaluation cost is the sum (`Isogeny.comp`). So a chain
  transfers at twice the total cost of its edges (`transfer_comp_cost`,
  `chain_equivalence`). This is Theorem A of the report, in the abstract
  form `TransferBound` uses: groups as `MonoidOps`, an isogeny restricted to
  the prime-order subgroup as `Hom`.
- **Twist order.** `4·#E^t = (t + 2)² − (t² − 4p)`, so `N ∣ f` and
  `N ∣ t + 2` give `N² ∣ 4·#E^t` (`twist_order_sq_dvd`). That is why every
  cross-gap kernel is rational on the twist in this class (Proposition E).
- **The class used.** Its discriminant identity, `N ∣ t + 2`, `N ≡ 9 (mod 11)`
  (so `(−11/N) = +1` and `h(O_N) = N − 1`) and the per-side curve count
  `1 + 3 + … + 96 = 190`, all by `decide` in the kernel.

Not here: elliptic curves, endomorphism rings and Vélu's formulas
themselves. Mathlib has no isogenies of elliptic curves over finite fields
to state them against. The bridge from a Vélu map on `E(F_p)` to a `Hom`
on `⟨P⟩` is the hypothesis `n ∤ deg φ`. The native tool checks it on every
edge (image on the codomain, homomorphism on random pairs, `[n]φ(P) = O`,
`φ(P) ≠ O`). It is not proved here.
-/

namespace DeepGap

open TransferBound

variable {α β γ : Type} {opsG : MonoidOps α} {opsH : MonoidOps β} {opsK : MonoidOps γ}

/-- The composite of two injective `one`/`mul`-preserving maps. -/
def Hom.comp (ψ : Hom opsH opsK) (φ : Hom opsG opsH) : Hom opsG opsK where
  toFun := fun a => ψ.toFun (φ.toFun a)
  inj := fun a b h => φ.inj a b (ψ.inj _ _ h)
  map_one := by rw [φ.map_one, ψ.map_one]
  map_mul := by
    intro a b
    rw [φ.map_mul, ψ.map_mul]

/-- A chain of two evaluable maps: evaluate `φ`, then `ψ`; the cost adds. -/
def Isogeny.comp (ψ : Isogeny opsH opsK) (φ : Isogeny opsG opsH) : Isogeny opsG opsK :=
  { toHom := Hom.comp ψ.toHom φ.toHom, cost := φ.cost + ψ.cost }

/-- The cost of a composite chain is the sum of its edges' costs. -/
theorem comp_cost (ψ : Isogeny opsH opsK) (φ : Isogeny opsG opsH) :
    (Isogeny.comp ψ φ).cost = φ.cost + ψ.cost := rfl

/-- Transfer along a chain: solve `(g, h)` in `G` by solving its image two
edges away, at twice the chain's cost, with the same answer. -/
theorem transfer_comp_cost (ψ : Isogeny opsH opsK) (φ : Isogeny opsG opsH)
    (S : Solver opsK) (g h : α) :
    ((transfer (Isogeny.comp ψ φ) S).run g h).1 =
      (S.run (ψ.toFun (φ.toFun g)) (ψ.toFun (φ.toFun h))).1 + 2 * (φ.cost + ψ.cost) := rfl

theorem transfer_comp_answer (ψ : Isogeny opsH opsK) (φ : Isogeny opsG opsH)
    (S : Solver opsK) (g h : α) :
    ((transfer (Isogeny.comp ψ φ) S).run g h).2 =
      (S.run (ψ.toFun (φ.toFun g)) (ψ.toFun (φ.toFun h))).2 := rfl

/-- **Theorem A, abstract form.** With a chain `G → H → K` (a descent by
`φ` and a 2-walk by `ψ`, say) and a chain back `K → H' → G`, every solver
on either end has a partner on the other end whose cost on every instance is
the first solver's cost on the transported instance plus twice the chain's
total edge cost. -/
theorem chain_equivalence {δ : Type} {opsH' : MonoidOps δ}
    (φ : Isogeny opsG opsH) (ψ : Isogeny opsH opsK)
    (φ' : Isogeny opsK opsH') (ψ' : Isogeny opsH' opsG) :
    (∀ S : Solver opsK, ∃ S' : Solver opsG, ∀ g h,
        (S'.run g h).1 = (S.run (ψ.toFun (φ.toFun g)) (ψ.toFun (φ.toFun h))).1 +
          2 * (φ.cost + ψ.cost)) ∧
    (∀ T : Solver opsG, ∃ T' : Solver opsK, ∀ g h,
        (T'.run g h).1 = (T.run (ψ'.toFun (φ'.toFun g)) (ψ'.toFun (φ'.toFun h))).1 +
          2 * (φ'.cost + ψ'.cost)) :=
  ⟨fun S => ⟨transfer (Isogeny.comp ψ φ) S, fun g h => transfer_comp_cost ψ φ S g h⟩,
   fun T => ⟨transfer (Isogeny.comp ψ' φ') T, fun g h => transfer_comp_cost ψ' φ' T g h⟩⟩

/-! ## The twist order -/

/-- `4·(p + 1 + t) = (t + 2)² − (t² − 4p)`: four times the order of the
quadratic twist, from the Frobenius discriminant. -/
theorem twist_order_identity (p t : Int) :
    4 * (p + 1 + t) = (t + 2) ^ 2 - (t ^ 2 - 4 * p) := by
  grind

/-- **Proposition E, arithmetic part.** If `t² − 4p = D·f²`, `N ∣ f` and
`N ∣ t + 2`, then `N² ∣ 4·#E^t`. -/
theorem twist_order_sq_dvd (p t D f N : Int) (hdisc : t ^ 2 - 4 * p = D * f ^ 2)
    (hf : N ∣ f) (ht : N ∣ t + 2) : N ^ 2 ∣ 4 * (p + 1 + t) := by
  obtain ⟨a, rfl⟩ := hf
  obtain ⟨b, hb⟩ := ht
  refine ⟨b ^ 2 - D * a ^ 2, ?_⟩
  have h1 : t = N * b - 2 := by omega
  subst h1
  grind

/-- `#E ≡ 4 (mod N)` under the same hypotheses: `N` does not divide the
curve's own order (for `N > 4`), only its twist's. -/
theorem order_mod (p t D f N : Int) (hdisc : t ^ 2 - 4 * p = D * f ^ 2)
    (hf : N ∣ f) (ht : N ∣ t + 2) : N ∣ 4 * (p + 1 - t) - 16 := by
  obtain ⟨a, rfl⟩ := hf
  obtain ⟨b, hb⟩ := ht
  -- 4(p + 1 − t) = (t − 2)² − (t² − 4p) and t − 2 = Nb − 4.
  refine ⟨N * b ^ 2 - 8 * b - D * N * a ^ 2, ?_⟩
  have h1 : t = N * b - 2 := by omega
  subst h1
  grind

/-! ## The class of this study (checked in the kernel) -/

def p : Int := 4497080234220953
def t : Int := -5051274
def N : Int := 631409

theorem class_discriminant : t ^ 2 - 4 * p = -11 * (2 ^ 6 * N) ^ 2 := by decide

theorem gap_prime_divides_t_plus_2 : N ∣ t + 2 := by decide

/-- So the twist order is divisible by `N²`, from the lemma above. -/
theorem class_twist_order : N ^ 2 ∣ 4 * (p + 1 + t) :=
  twist_order_sq_dvd p t (-11) (2 ^ 6 * N) N class_discriminant
    ⟨2 ^ 6, by decide⟩ gap_prime_divides_t_plus_2

/-- `N ≡ 9 (mod 11)` and `9 = 3²`, so `(−11/N) = (N/11) = +1`. -/
theorem gap_prime_mod_11 : N % 11 = 9 := by decide

/-- The curves on levels `0..d` of a one-vertex inert 2-volcano: `1`, then
`3·2^(k−1)` on level `k`, i.e. `3·(2^d − 1) + 1` in all. -/
def levelCount : Nat → Nat
  | 0 => 1
  | k + 1 => 3 * 2 ^ k

def volcanoSize : Nat → Nat
  | 0 => levelCount 0
  | d + 1 => volcanoSize d + levelCount (d + 1)

theorem volcanoSize_closed (d : Nat) : volcanoSize d + 2 = 3 * 2 ^ d := by
  induction d with
  | zero => decide
  | succ d ih =>
    simp only [volcanoSize, levelCount]
    rw [Nat.pow_succ]
    omega

theorem depth6_size : volcanoSize 6 = 190 := by decide

end DeepGap
