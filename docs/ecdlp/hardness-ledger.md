# ECDLP hardness ledger

Working document for attacking the foundations that ECDLP hardness rests on,
one layer at a time. Every entry has a statement, the role it plays, the known
way it breaks, and its formalization status. The rule is: **a claim moves to
"formal" only when a machine-checked proof exists**; tests and implementations
are listed as such, never as proofs.

Formal artifacts live in `research/ecdlp_foundations_20261109/`.

## Layer 0 — abstract group and scalar action

Status: **formal** (`Foundations.lean`, 32 theorems, 0 trusted).

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

Status: implemented and tested; **formalization open**.

| Claim | Role | Known failure mode |
| --- | --- | --- |
| Chord–tangent formulas return a point on the curve (closure) | Makes the curve a group | Singular curves; characteristic 2/3 formulas |
| The law is associative | Group axiom, needed for scalar multiplication | Wrong exceptional-case handling |
| Doubling/negation/identity formulas agree with the law | Basis of all coordinate systems | Incomplete formulas (exception at P=Q, P=-Q, O) |
| Every imported EFD chart round-trips to the affine law | Correct coordinates for the implemented arithmetic | Chart/scale errors |

What is already machine-checked: the 33+1 chart round trips and the cache
relations (Lean, `Charts.lean`), plus 486/486 formula programs symbolically
against the pinned affine law (CAS, not kernel). Closure and associativity of
the affine law itself are the next formal targets.

Infrastructure note: core Lean 4 has no `ring`/`field_simp`, and no Mathlib
checkout is present (the Mathlib ltar cache exists, but the source clone is
missing). Generic closure/associativity proofs need either that checkout or a
verified polynomial normalizer; this is the gating decision for layer 1.

## Layer 2 — generic-group lower bound

Status: **assumed** (Shoup 1997, `Ω(√p)`).

The lower bound is a statement about *algorithms that only use the group
operation*. It does not apply to index calculus, pairings, or any attack that
uses the curve's coordinate structure. Formalizing it needs an adversary model
and probability theory; queued after Layer 1.

## Layer 3 — algorithm costs

| Claim | Role | Status |
| --- | --- | --- |
| BSGS solves DLP in `O(√n)` time and memory | Baseline square-root attack | Implemented; correctness formalization queued |
| Pollard rho: expected `√(πn/2)` group operations | Best generic attack in practice | Implemented and measured; probabilistic proof open |
| Pohlig–Hellman reduces DLP to prime-power order subgroups and CRT | Smooth-order curves are easy | Implemented; correctness formalization queued |
| Index calculus is subexponential when a smooth factor base exists | Breaks high-genus / small-characteristic settings | Implemented for the repo's IC campaigns; cost model empirical |

## Layer 4 — curve-specific attacks (each attacks a hypothesis, not the group)

| Attack | Hypothesis it breaks | Status |
| --- | --- | --- |
| Pohlig–Hellman | Order has small prime factors | Implemented |
| MOV / Frey–Rück | Embedding degree is small; pairing is non-degenerate and computable | Implemented; bilinearity/non-degeneracy formalization open |
| Smart–Satoh–Araki | `#E(F_p) = p` (anomalous); p-adic elliptic logarithm | Implemented; p-adic formalization open |
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
   homomorphism reduction lemmas (`Foundations.lean`, 43 theorems).
1. **Curve group law closure** for short Weierstrass affine addition and
   doubling, over an abstract field with curve equation hypotheses. Blocked on
   the ring-tactic/Mathlib decision above.
2. **Associativity** of the chord–tangent law (the hard one; likely via the
   divisor-class/linear-equivalence argument rather than brute algebra).
3. **BSGS correctness** and **Pohlig–Hellman** in the Layer 0 interface.
4. **Generic-group lower bound** statement and adversary model.
5. **Pairing bilinearity/non-degeneracy** for the MOV reduction, then the
   reduction theorem itself.
6. **Anomalous-curve attack** at the formal-group level.

Each step lands as a Lean file with a proof-gate receipt, plus an entry here.
