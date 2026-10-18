# Open Lean targets for `papers/ic-negative-results/`

These are stated, not proved. They live in this file rather than in compiled
sources because the repository proof gate (`tools/lean/proof-gate.sh`) fails any
Lean file that contains `sorry`. Each block is Lean-syntax pseudocode and has not
been elaborated.

## O1. Theorem 15.1 in full (oracle model, simulator, hybrid step)

`IcNegative/AffineCore.lean` proves the counting step only. Open: define the
isogeny-augmented generic group model (encodings `σ_j : ZMod r → Bitstring`,
oracles for group laws and isogenies with known scalars `λ_ψ`), the adversary as
a `Q`-query decision tree, and prove

```lean
theorem isogeny_ggm_success_le (r Q : ℕ) [Fact r.Prime] (adv : Adversary r Q) :
    successProb adv ≤ ((Q + 2) ^ 2 : ℚ) / r + 1 / r
```

Needed: a probability monad over the random injections, or an explicit counting
over injection tables; the simulator argument (answers are fresh unless two
distinct degree-1 polynomials coincide at the secret, which is
`IcNegative.injOn_of_not_bad`); the final guessing term. Hypothesis to expose
exactly as in the note: `r` prime, `r ∤ h`, each `ψ` restricted to `G_i` is
either an isomorphism onto `G_j` or zero.

## O2. Yield conservation for multisets of a base

`IcNegative.sum_yield` is the double count for any map. The instance in
`KN-FIND-007` is the domain `Sym D m` for a base `D : Finset G`:

```lean
theorem mean_yield_multiset (D : Finset G) (m : ℕ) :
    ∑ g, yieldAt (fun s : Sym D m => (s.1.map (↑)).sum) g = Nat.choose (D.card + m - 1) m
```

Needs `Sym.card_sym_eq_choose` and a `[AddCommGroup G]` instance on the sum map.
The list form without the binomial count is already kernel-checked in
`research/factor_base_designs_20261015/lean/FactorBaseDesigns/Yield.lean`.

## O3. The Lemma 4 witnesses as kernel-checked statements

Definitions 5 and 6 of Nagao ePrint 2015/984 (equality reading) over
`MvPolynomial (Fin n) (ZMod 2)`:

```lean
def FirstFall (gens : List (MvPolynomial (Fin n) (ZMod 2))) (d : ℕ) : Prop := ...
def FakeFirstFall (gens : List (MvPolynomial (Fin n) (ZMod 2))) (d : ℕ) : Prop := ...
theorem witnessA_fake : IsLeastFall FakeFirstFall [X0^2 * X1, X0 * X1 + X0] 2
theorem witnessA_true : IsLeastFall FirstFall [X0^2 * X1, X0 * X1 + X0] 3
```

The finite check is done natively (`examples/blindspots_19_checks.rs lemma4`,
`evidence/checks.txt`); the open work is the Lean encoding of the definitions and
a `decide`-style certificate over the finite multiplier spaces.

## O4. Monodromy `(Z/2)^{m-2}` of the Semaev cover

Statement: the splitting group of `S_m(t_1, ..., t_{m-1}, T)` over
`K(t_1, ..., t_{m-1})` is `(ZMod 2)^(m-2)`, acting simply transitively on the
`2^(m-2)` roots. Needs Galois theory of function fields of `E^{m-1}` (Artin's
theorem for a quotient by a finite group; properness of the preimage of `E[2]`
in characteristic 2). Out of reach of the current Mathlib API without a large
development; the paper's proof stands as a written derivation.
