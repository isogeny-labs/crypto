# Selector resultant: proof map and Lean instance gate

Date: 2026-11-22. Scope: the propositions and doubling identities in [REPORT.md](REPORT.md). Proof standing: written derivations only; no study Lean declaration, compiled theorem, axiom report or signed PCC verdict is asserted. The [15 passing focused native tests](evidence/attempt3/tests.stdout.log) are bounded implementation evidence and do not change these formal obligations. [pcc:unverified]

## Six-field instance statement

1. **Structure above.** `CommRing R`, `Polynomial R`, ring homomorphisms, polynomial evaluation and `Polynomial.dickson` from `Mathlib.RingTheory.Polynomial.Dickson`. For the selector instance use `Fp=ZMod p` with primality and the existing quotient ring `Fp[X]/(X^d-1)`. This quotient is generally **not a field**. Field root/gcd assertions live separately over `Fp`, with `d>=1` and `d | p-1`.
2. **Theorems used as stated.** The inspected Mathlib revision supplies `Polynomial.dickson_zero`, `Polynomial.dickson_one`, `Polynomial.dickson_add_two` and `Polynomial.map_dickson`. They identify the recurrence after evaluating `Polynomial.dickson 1 (A*C) n` at `-B` and transport it through ring maps. The inspected parameter-1 theorem `Polynomial.dickson_one_one_eval_add_inv` assumes a product equal to one; it does not supply the arbitrary-parameter power-sum law required here. The universal selector resultant and general Dickson doubling bridges remain generic upstream obligations, with no invented existing theorem name.
3. **Object as instance.** `T_n := eval (-B) (Polynomial.dickson 1 (A*C) n)` over an arbitrary commutative ring; then instantiate coefficients by the S3 polynomial expressions and map into the selector quotient. The semantic object is a polynomial or ring element, not a new syntax tree or finite tested table.
4. **Assembly probe.** Not type-checked: this runtime has neither `lean` nor `lake`. Required future probe: assemble the universal norm endpoint from the generic resultant bridge, first over abstract coefficients and then in the quotient. Separately instantiate the field/root theorem at `ZMod p`. Confirm exact imported API and hypotheses under the chosen dependency pin before any study Lean edit. No speculative uncompiled `.lean` file is included.
5. **Missing components only.** Generic arbitrary-parameter Dickson power sums/doubling; monic-selector resultant bridge preserving leading-coefficient degree drops; split-selector root and gcd bridge; exact identification of production S3 coefficients with the shared semantic curve definition; finite-root/point-lifting completeness and its API contract. Each reusable law belongs in crypto-lean (or upstream Mathlib), then the study supplies only instance equalities and endpoints.
6. **Inheritance table.** Below. Any missing bridge remains open regardless of native tests or manuscript agreement.

Inspected Mathlib source: [Dickson.lean at 6ebcce37335bcb6bca64596e5d57bb3035648030](https://github.com/leanprover-community/mathlib4/blob/6ebcce37335bcb6bca64596e5d57bb3035648030/Mathlib/RingTheory/Polynomial/Dickson.lean). This is a source-audit reference, not an installed or compiled dependency. The repository's adopted toolchain and crypto-lean revision must be resolved and pinned before a future probe; no compilation on Lean 4.19 or another version is claimed.

## Statement ownership and exact quantifiers

| ID | Intended statement | Kind / owner | Hypotheses | Lean status |
|---|---|---|---|---|
| D0 | Dickson evaluation has initial values `2,-B` and recurrence `T_(n+2)=-B*T_(n+1)-A*C*T_n` for all n | Endpoint of existing Mathlib recurrence | Every `CommRing R`, `A,B,C:R`, `n:Nat` | Source laws identified; assembly not compiled |
| D1 | `D_n(u+w,u*w)=u^n+w^n` for every n | Generic law, crypto-lean upstream candidate if absent under final pin | Every `CommRing R`, `u,w:R` | Written argument; no checked declaration |
| D2 | Even and odd doubling laws for arbitrary parameter | Generic law, crypto-lean | Every `CommRing R`, `s,v:R`, `n:Nat` | Written polynomial-identity argument; formal bridge open |
| R1 | `Res(Y^d-1,A*Y^2+B*Y+C)=A^d+C^d-T_d` | Generic law, crypto-lean | Every `CommRing R`, all A,B,C, integer d>=1; selector is first argument | Written universal proof; all leading-degree drops included |
| Q1 | Reduce each arithmetic step modulo monic selector without changing its quotient value | Endpoint of ring-homomorphism laws | Every ring map; instantiated quotient exists | Inheritance intended; exact quotient API/probe pending |
| F1 | Gcd support equals existential selector-constrained coordinate support | Generic law, crypto-lean | Field Fp, p prime, d>=1 divides p-1; degree_Y q<=2 | Written proof; splitness/squarefreeness and resultant specialization bridges open |
| E1 | S3's quadratic has no identically-zero Y fiber on a nonsingular short-Weierstrass curve | Instance obligation of shared S3 semantics | Odd characteristic; nonzero discriminant | Written off-diagonal/diagonal proof; source binding open |
| E2 | Returned rational signed pairs are exactly selector points adding to the public finite target | Endpoint of complete extraction, square roots and shared group semantics | Nonsingular E/Fp, finite rational T, complete coordinate output; optional explicit annihilator | Native bounded panels passed; no structural Lean proof |
| I1 | 1302 divides 2147483646 | Instance arithmetic obligation | Exact integers | Written integer equality; no Lean certificate claimed |
| C1 | Norm stage uses `O(log d)` ring products, schoolbook `O(d² log d)` field work | Generic cost refinement | Exact binary algorithm and charged schoolbook backend | Analytical upper bound only; formal cost model open |

No claim relies on the false assumption that `[r]P=O` alone identifies a particular cyclic subgroup in every elliptic-curve group. An endpoint naming that subgroup must additionally import its actual order/decomposition certificate or an equivalent uniqueness argument.

## Inheritance table

| Operation / law | How the instance should inherit it | Remaining obligation |
|---|---|---|
| Ring addition, multiplication, powers | Existing `CommRing` instances and evaluation/ring-map laws | Type-check chosen coefficient and quotient types |
| Dickson recurrence | Existing `dickson_zero`, `dickson_one`, `dickson_add_two` followed by evaluation | Short assembly probe; no study-local reimplementation |
| Coefficient specialization | Existing `map_dickson` and polynomial mapping laws | Exact evaluation-map composition under pinned API |
| Arbitrary-parameter power sums | Generic D1 | Find an existing stronger law under final pin or build it once upstream |
| Binary even/odd step | Generic D2 plus ring power laws | Prove general law, then algorithm invariant |
| Selector norm | Generic R1 | Resultant product and universal specialization proof |
| Quotient reduction | Canonical quotient ring homomorphism | Coefficient-list implementation refinement |
| Gcd root support | Existing polynomial gcd/root infrastructure plus F1 | Locate exact theorems, then split-selector instance |
| Point addition and scalar multiplication | Shared Mathlib/crypto-lean elliptic-curve group | Bind implementation semantics without copying group laws |
| Membership and signed replay | E2 endpoint | Complete root enumeration and treatment of infinity/refusals |

## Proof chain before Lean

For R1, keep each future `calc` step confined to one rewrite:

1. Rewrite the monic-first resultant as the product of the quadratic at selector roots in a generic splitting field.
2. Replace the quadratic by its generic factorization `A(Y-alpha)(Y-beta)`.
3. Rewrite the two selector-root products using `alpha^d-1` and `beta^d-1`; keep both factors `(-1)^d` until they cancel.
4. Expand the product into `(alpha beta)^d-(alpha^d+beta^d)+1`.
5. Replace `alpha beta` by `C/A` and the scaled power sum by the Dickson evaluation.
6. Use injection from `Z[A,B,C]` into its fraction field to conclude the polynomial identity before specialization.
7. Apply the coefficient homomorphism into arbitrary R. Invertibility of A is no longer a hypothesis.

For F1: specialize R1 at a selector root; use the field zero-product equivalence; use splitness of `Y^d-1`; then apply gcd/root equivalence. These are separate steps, not one tactic that assumes the desired conclusion.

## Required completion evidence

The native checker issued evaluation-only controls [IDC1h902fa2049de745c9](identities/IDC1h902fa2049de745c9.json), [IDC1he093a3eae6a97efd](identities/IDC1he093a3eae6a97efd.json) and [IDC1h6d456b2a22da4771](identities/IDC1h6d456b2a22da4771.json). The [receipt](evidence/attempt2/certificates.stdout.json) records acceptance of 32 deterministic producer points plus 32 deterministic additional points per identity in `F_(2^61-1)`. These exercise low-degree polynomial controls only; they do not discharge D1, D2, R1, Q1 or E2, supply an adversarial soundness claim, or constitute a signed PCC verdict.

- Locate the generic owning modules and pin Lean, Mathlib and crypto-lean revisions.
- Type-check the abstract and actual-instance assembly probes before growing a study proof.
- Build missing reusable laws at the owning floor and retain exact statements/hypotheses.
- Build focused modules; run the repository proof gate and `#print axioms` for every endpoint.
- Reject `sorry`, `admit`, custom axioms encoding the target, hidden weaker statements, or unapproved native trust.
- Preserve all probe/build failures, revisions and receipts. Native examples, algebraic-identity test records and same-team agreement do not replace a kernel proof or signed PCC standing.

No heartbeat increase, private syntax calculus, duplicated group law, or speculative Lean source is proposed.
