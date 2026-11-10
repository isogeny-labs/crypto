# ECDLP hardness ledger

Working document for attacking the foundations that ECDLP hardness rests on,
one layer at a time. Every entry has a statement, the role it plays, the known
way it breaks, and its formalization status. The rule is: **a claim moves to
"formal" only when a machine-checked proof exists**; tests and implementations
are listed as such, never as proofs.

Formal artifacts live in `research/ecdlp_foundations_20261109/`.

## Layer 0 — abstract group and scalar action

Status: **formal** (`Foundations.lean`, 55 theorems, 0 trusted).

| Claim | Why hardness needs it | Status |
| --- | --- | --- |
| Addition is associative, commutative, with identity and inverse | Without a group, "discrete log" is not defined | Formal |
| Cancellation, uniqueness of identity/inverse, `neg_neg` | Needed by every reduction and algorithm correctness proof | Formal |
| `nsmul` is an action: `(m+n)·P = m·P + n·P`, `(mn)·P = m·(n·P)` | Scalar multiplication is the map ECDLP inverts | Formal |
| Exact order: positive, annihilating, minimal | Defines the subgroup order used in every complexity statement | Formal |
| Any annihilating exponent is a multiple of the order | "DLP is well defined mod n" | Formal |
| Equal scalar multiples differ by a multiple of the order | Uniqueness of the discrete log modulo `n` | Formal |
| `k·P = (k mod n)·P` | Reductions work modulo the order | Formal |
| Distinct exponents below `n` give distinct elements | The map `Z/n → ⟨P⟩` is injective | Formal |
| Homomorphisms commute with `nsmul` and `neg` | Every reduction (MOV, Pohlig–Hellman, descent) transports scalars | Formal |
| Injective homomorphism preserves DLP uniqueness; any solution reduces mod `n` | Hardness transfers only under injective reductions; non-injective maps leave a coset ambiguity | Formal |

Attack surface at this layer: none algebraically — these are the axioms every
attack must respect. The only way to "break" the layer is to instantiate it
with a structure that is not a group, which is exactly what implementation bugs
(singular curves, wrong cofactor, invalid points) do.

## Layer 1 — the curve group law

Status: **closure and x-commutativity formal; associativity inherited from
Mathlib's `AddCommGroup`** (Mathlib project `lean/Closure.lean`).

| Claim | Role | Known failure mode |
| --- | --- | --- |
| Chord–tangent formulas return a point on the curve (closure) | Makes the curve a group | Singular curves; characteristic 2/3 formulas — **formal for affine add/double over a field, char ≠ 2 for doubling** |
| The law is associative | Group axiom, needed for scalar multiplication | **Formal via the Mathlib bridge**: our chord slope, `addX` and `addY` are proved equal to Mathlib's `Affine.slope/addX/addY` for distinct x, and Mathlib's `AddCommGroup` on `Point` proves associativity |
| Doubling/negation/identity formulas agree with the law | Basis of all coordinate systems | Incomplete formulas (exception at P=Q, P=-Q, O) — closure of doubling formal |
| Every imported EFD chart round-trips to the affine law | Correct coordinates for the implemented arithmetic | Chart/scale errors |

What is already machine-checked: the 33+1 chart round trips and the cache
relations (Lean, `Charts.lean`), plus 486/486 formula programs symbolically
against the pinned affine law (CAS, not kernel). Closure of the affine law,
the x-commutativity of the chord sum, doubling, the opposite-point case and
associativity (through the Mathlib bridge) are now kernel-checked in
`lean/Closure.lean` (`PASS 17 thm 0 trusted`).

Infrastructure: layer 1a uses a pinned Mathlib Lake project
(`research/ecdlp_foundations_20261109/lean`, rev `d13f23b7...`) with `ring`,
`field_simp` and `linear_combination`. Certificate coefficients for closure
were computed externally and checked by Lean's ring normaliser, so the kernel
still checks the proof.

## Layer 2 — generic-group lower bound

Status: **partially formal** — the counting core is machine-checked in
`lean/Generic.lean`; the adversary/transcript model and the probability step
remain assumptions (Shoup 1997, `Ω(√p)`).

The lower bound is a statement about *algorithms that only use the group
operation*. It does not apply to index calculus, pairings, or any attack that
uses the curve's coordinate structure. Formalizing it needs an adversary model
and probability theory; the combinatorial half is now done.

| Theorem | Statement |
| --- | --- |
| `collision_secret_unique` | For `bᵢ ≠ bⱼ` the equation `(aᵢ-aⱼ) + (bᵢ-bⱼ)·x = 0` has the unique solution `-(aᵢ-aⱼ)/(bᵢ-bⱼ)` |
| `revealed_card_le` | With `k` known elements, at most `k(k-1)` secrets are revealed by some colliding pair |
| `samePattern_of_not_revealed` | All secrets outside the revealed set have the same collision pattern |
| `correct_card_le` | A non-adaptive algorithm whose output depends only on the pattern is correct on at most `k(k-1)+1` of the `p` secrets |
| `correct_card_le_nt` | A decision tree with `k` nontrivial adaptive tests is correct on at most `k+1` secrets |
| `correct_card_le_depth` | A depth-`d` adaptive tree is correct on at most `2^d` secrets |

The remaining Layer 2 gap is the probability step: over a uniform secret, an
adaptive pair of queries collides with probability at most `1/p`, and a union
bound over the `t(t-1)/2` query pairs gives Shoup's `t²/p` shape. The decision
tree bound is deterministic; formalizing the query distribution and averaging
over it is the remaining model-building step.

## Layer 3 — algorithm costs

| Claim | Role | Status |
| --- | --- | --- |
| BSGS solves DLP in `O(√n)` time and memory | Baseline square-root attack | Implemented; **correctness formal** (`Algorithms.lean`): decomposition, match existence, candidate correctness, collision-free table. The `O(√n)` time/memory bound itself is a cost premise. |
| Pollard rho: expected `√(πn/2)` group operations | Best generic attack in practice | Implemented and measured; **algebraic correctness formal** (`Rho.lean`): finite-orbit collision, Floyd's meeting, collision-to-relation and DLP recovery. The expected-step bound remains a probability premise. |
| Pohlig–Hellman reduces DLP to prime-power order subgroups and CRT | Smooth-order curves are easy | Implemented; **subgroup step and CRT assembly formal** (`Algorithms.lean`): `d·a` has order `n/d`, projection to the quotient instance, coprime CRT combination, reconstruction modulo `n`. The `∑ √(pᵢ^{eᵢ})` cost bound is a cost premise. |
| Index calculus is subexponential when a smooth factor base exists | Breaks high-genus / small-characteristic settings | Implemented for the repo's IC campaigns; cost model empirical |

## Layer 3b — algorithm correctness (formal)

`lean/Algorithms.lean` (Mathlib project; the project gate reports
`PASS 114 thm 0 trusted`) proves that the two standard generic reductions
return the right answer, without assuming anything about cost.

| Theorem | Statement |
| --- | --- |
| `baby_eq_giant_iff` | A baby/giant match is equivalent to `(i·m+j)·P = Q` |
| `bsgs_decomposition` | `x < n ≤ m²` implies `x = i·m+j` with `i,j < m` |
| `bsgs_exists_match` | Some baby/giant pair matches for every target `x·P`, `x < n` |
| `bsgs_candidate_correct` | Any match yields a candidate `≡ x (mod n)` |
| `baby_injective`, `baby_table_card` | The `m`-entry baby table has no collisions |
| `modEq_mul_of_coprime` | Coprime congruences combine modulo the product |
| `ph_subgroup_order` | If `d ∣ n`, then `d·P` has order `n/d` |
| `ph_projection` | `(x mod m₁)·(m₂·P) = m₂·(x·P)`, so the quotient instance is correct |
| `ph_reconstruct` | Knowing `x mod m₁` and `x mod m₂` gives `x mod n` |
| `ph_prime_power_step` | The generator of the `pᵉ`-subgroup has order exactly `pᵉ` |

The remaining Layer 3 claims are cost/success statements: BSGS uses `2m` group
operations and `m` stored points; PH costs `∑ √(pᵢ^{eᵢ})`; rho's `√(πn/2)`
expectation is a probability claim. None of these are consequences of group
axioms alone, so they stay explicitly outside the kernel until a machine-checked
cost/probability model exists.

## Layer 3c — Pollard rho correctness (formal)

`lean/Rho.lean` (`PASS 114 thm 0 trusted` for the whole project) proves the
deterministic part of the rho attack.

| Theorem | Statement |
| --- | --- |
| `exists_collision_of_finite` | Every endofunction of a finite type has `f^[i] x = f^[j] x`, `i < j` |
| `exists_tail_cycle` | The orbit has a tail and a positive period |
| `floyd_meets` | Some `k > 0` satisfies `f^[k] x = f^[2k] x` (tortoise meets hare) |
| `rho_relation_mod` | Colliding `a·P + b·Q` states give `(a-a') + (b-b')·x ≡ 0` in `ZMod n` |
| `rho_solve`, `rho_dlp_recover` | If `n` is prime and `b ≠ b'`, the relation recovers `x` exactly |

The collision step is where the probability enters: the *number of steps* until
two walks meet is the birthday bound `√(πn/2)`, and that remains a premise.
What is proved is that once a collision with distinct `Q`-coefficients occurs,
the recovered scalar is the discrete log, not merely a candidate.

## Layer 4b — MOV reduction (formal interface)

`lean/Mov.lean` proves the algebraic facts the MOV/Frey–Rück reduction needs,
without constructing the pairing itself.

| Theorem | Statement |
| --- | --- |
| `target_order_dvd` | `addOrderOf (e(P,P))` divides `addOrderOf P` |
| `target_annihilator` | `x·e(P,P) = 0 ↔ addOrderOf (e(P,P)) ∣ x` |
| `mov_target_injective` | Full target order makes `x ↦ x·e(P,P)` injective below `n` |
| `mov_reduction` | A target solution below `n` is the unique source exponent |
| `mov_degenerate_ambiguity` | If the target order is a proper divisor, at least two exponents below `n` collide |

**Alternating correction.** `lean/Pairing.lean` records that the Weil pairing is
alternating, so `e(P,P) = 0` and `addOrderOf (e P P) = 1`
(`alt_diag_order_ne`).  The hypothesis `addOrderOf (e P P) = n > 1` of
`Mov.lean` is therefore an abstraction, not the Weil pairing itself.  The
supersingular attack uses a distortion map: `DistortedBilin` carries an
additive `φ : G →+ G'`, and `distorted_mov_reduction` proves that
`e(P, φ(P))` of full order still recovers the source exponent uniquely.

The open part is now exactly the construction: build the Weil/Tate pairing,
prove bilinearity and the distortion-map non-degeneracy, and identify the
embedding degree.

## Layer 4c — anomalous curves (formal reduction step)

`lean/Anomalous.lean` proves the endpoint of Smart–Satoh–Araki.

| Theorem | Statement |
| --- | --- |
| `anomalous_order_of_card` | In a group of cardinality `p`, every nonzero point has order `p` |
| `anomalous_generator` | Such a point has order equal to the whole group order |
| `log_reduction` | An additive map `L : G →+ ZMod p` sends `Q = x·P` to `x·L(P) = L(Q)` |
| `anomalous_recover` | With `L` injective and `P ≠ 0`, `x = (L(Q)/L(P)).val` for `x < p` |

The open part is the p-adic elliptic logarithm: define the formal group of the
curve, construct the isomorphism `E(F_p) → pZ_p/p²Z_p`, and prove it is
injective and computable.

## Layer 4 — curve-specific attacks (each attacks a hypothesis, not the group)

| Attack | Hypothesis it breaks | Status |
| --- | --- | --- |
| Pohlig–Hellman | Order has small prime factors | Implemented |
| MOV / Frey–Rück | Embedding degree is small; pairing is non-degenerate and computable | Implemented; **reduction logic formal** (`Mov.lean`): target order divides the source, full order gives an injective target map, and a proper target order provably leaves multiple candidates. Weil pairing construction and its non-degeneracy remain open. |
| Smart–Satoh–Araki | `#E(F_p) = p` (anomalous); p-adic elliptic logarithm | Implemented; **reduction step formal** (`Anomalous.lean`): prime order forces order `p`, an injective log map turns the DLP into `x = L(Q)/L(P)`, and `x` is read off as a residue. Construction/injectivity of the p-adic logarithm remains open. |
| GHS / Weil descent | Curve defined over a proper subfield / weak cover | Implemented for the repo's descent campaigns |
| Singular curve | Discriminant zero: node or cusp | Handled by validation; group is multiplicative/additive |
| Invalid-curve / small-subgroup | Implementation does not validate points | Implementation assumption, not a math theorem |
| Twist attacks | Implementation does not check twist order | Implementation assumption |

## Layer 5 — implementation assumptions

Uniform scalar representation, blinding, constant-time execution and point
validation are not mathematical hardness assumptions. They are tracked
separately; a break here breaks deployments, not ECDLP.

## Queue (one at a time)

0. **Done:** modular scalar reduction, injectivity below the order, and the
   homomorphism reduction lemmas (`Foundations.lean`, 55 theorems).
1. **Done (1a/1b):** closure of affine addition and doubling over a field
   (`Closure.lean`, `PASS 8 thm 0 trusted`; doubling assumes `2 ≠ 0`), plus
   commutativity of the x-coordinate.
2. **Done (1b):** the explicit slope/`addX`/`addY` are bridged to Mathlib's
   `Affine.slope/addX/addY`, and associativity is inherited from Mathlib's
   proven `AddCommGroup` on `Point`. Exception-case wrapping (0, P = -Q,
   doubling) remains to be connected to the same interface.
3. **Done:** BSGS and Pohlig–Hellman correctness in `Algorithms.lean`, and the
   deterministic part of Pollard rho in `Rho.lean` (Mathlib project,
   `PASS 114 thm 0 trusted`): baby/giant match equivalence, existence, candidate
   correctness, collision-free baby table, coprime CRT combination, PH
   subgroup order, projection and reconstruction; finite-orbit collision,
   Floyd meeting, collision relation and DLP recovery. Only the cost/success
   premises remain outside the kernel.
4. **Partially done (2a/2c):** the Shoup counting core and the deterministic
   adaptive decision-tree bound are formal in `Generic.lean` and
   `Adaptive.lean` (`collision_secret_unique`, `revealed_card_le`,
   `correct_card_le_nt`, `correct_card_le_depth`). Remaining: the adaptive
   query distribution, the `1/p` collision probability and the union bound that
   turns these into `O(t²/p)`.
5. **Partially done (4b):** MOV reduction formal in `Mov.lean`, and
   `Pairing.lean` adds the alternating no-go and the distortion-map interface
   (`distorted_mov_reduction`). Remaining: construct the Weil/Tate pairing,
   prove bilinearity/non-degeneracy, and identify the embedding degree.
6. **Partially done (4c):** the anomalous reduction step is formal in
   `Anomalous.lean`. Remaining: the formal group, the p-adic elliptic
   logarithm and its injectivity/computability.

Each step lands as a Lean file with a proof-gate receipt, plus an entry here.
