# H113 Lean proof map and instance gate

Status: design gate completed before the first Lean edit. Lean 4.34.1 and
Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612` build all declarations.
The repository proof gate passes 14 audited declarations with zero failing
and zero trusted-native; retained receipt:
`proof-gate-v1/research_h113_binary_trinary_winding_20261117/`.

This formalization covers the structural theorem obligations in `PROTOCOL.md`.
It does not assert that any measured winding predicate is exact.

## Six-field instance statement

1. **Structure above.** The path and decoder statements quantify over
   Mathlib's `AddCommGroup A`, using inherited addition, subtraction, zero and
   the natural scalar action. The representative statement quantifies over
   Mathlib's ordered commutative ring `ℤ`, divisibility and linear order. The
   integer decoder uses `ℕ`, `%`, `/`, strong induction and the standard radix
   decomposition law.
2. **Theorems used as stated.** Path normalization uses `nsmul_add`,
   `nsmul_sub`, `add_assoc`, `add_sub_cancel_right` and numeral normalization.
   Representative bounds use the witness supplied by `p ∣ N` and Presburger
   arithmetic over `ℤ`. Radix reconstruction uses `Nat.mod_add_div` together
   with strong induction. The group decoder endpoint uses the exact-oracle
   hypothesis, `Nat.mod_add_div`, `nsmul_add`, `nsmul_nsmul`, and a supplied
   inverse-action law for multiplication by six.
3. **Object as an instance.** The lifted curve group is used only through its
   `AddCommGroup` instance. A set-theoretic section is a function `Q → A`; its
   carry is an expression in `A`, not a new curve representation. Canonical
   carry representatives are four integers satisfying interval and congruence
   hypotheses. The public digit predicate is a function whose exactness is a
   hypothesis. No syntax type or coordinate implementation is primary.
4. **Assembly probe.** The generic target chain is:

   ```text
   2 • (3 • s(h6) - s(h2)) + (2 • s(h2) - s(q))
     = 6 • s(h6) - s(q)
     = 3 • (2 • s(h6) - s(h3)) + (3 • s(h3) - s(q));

   N = 2*a + b - 3*c - d, p ∣ N, 0 ≤ a,b,c,d < p
     => ∃ w, N = p*w ∧ -3 ≤ w ∧ w ≤ 2;

   digit(m)=m%6 and inv6 • (6 • X)=X
     => inv6 • ([m]G - [digit(m)]G) = [m/6]G,
   ```

   followed by generic radix-six reconstruction and an independent injective
   scalar-action verification endpoint. The focused Lean file itself is the
   type-checked assembly probe; no curve-specific coordinates enter it.
5. **Missing components only.** No Mathlib theorem packages the exact
   binary/trinary carry diamond, the four-representative quotient range, or a
   recursive public-oracle radix decoder. They will be stated generically over
   the structures above. These are upstream candidates for crypto-lean; this
   study keeps a single generic declaration for each because the current
   crypto tree does not depend on crypto-lean and no second consumer is being
   added here. The bridge proving that a concrete finite-local-ring formal
   parameter supplies the four integer representatives is intentionally open;
   it is checked by the native arithmetic verifier, not assumed by the Lean
   endpoints.
6. **Inheritance table.** Every operation and law used by the report is listed
   below. A missing entry means the formalization is unfinished.

## Statement table

| Statement | Kind | Owner module | Structure | Hypotheses |
| --- | --- | --- | --- | --- |
| `carry_two_three_path` | generic law; crypto-lean upstream candidate | `H113BinaryTrinaryWinding` | `AddCommGroup A` | none beyond the structure |
| `carry_three_two_path` | generic law; crypto-lean upstream candidate | `H113BinaryTrinaryWinding` | `AddCommGroup A` | none beyond the structure |
| `commuting_carry_paths` | endpoint | `H113BinaryTrinaryWinding` | `AddCommGroup A` | none beyond the structure |
| `representative_winding_range` | generic law; crypto-lean upstream candidate | `H113BinaryTrinaryWinding` | `ℤ` | `0 < p`, four canonical intervals, `p ∣ N` |
| `exact_predicate_supplies_baseSix` | endpoint | `H113BinaryTrinaryWinding` | functions on semantic public points | exactness hypothesis |
| `radixReconstruct_eq` | generic law; crypto-lean upstream candidate | `H113BinaryTrinaryWinding` | `ℕ` | radix at least two; exact remainder oracle |
| `baseSixReconstruct_eq` | endpoint | `H113BinaryTrinaryWinding` | `ℕ` | exact base-six oracle |
| `baseSix_group_step` | endpoint | `H113BinaryTrinaryWinding` | `AddCommGroup A` | exact digit and inverse-six action |
| `verified_scalar_unique` | endpoint using injectivity | `H113BinaryTrinaryWinding` | arbitrary scalar/point types | candidate and true evaluations; injective action |

## Inheritance table

| Report operation/law | Lean object | Inheritance line |
| --- | --- | --- |
| lifted group addition | `HAdd.hAdd` on `A` | `[AddCommGroup A]` instance |
| lifted group subtraction | `Sub.sub` on `A` | `[AddCommGroup A]` instance |
| multiplication by 2, 3, 6 | `n • x` | `nsmul` from `[AddCommGroup A]` |
| distribution over carry expressions | `nsmul_add`, `nsmul_sub` | existing Mathlib lemmas |
| associativity/commutativity/cancellation | additive group laws | `[AddCommGroup A]` instance |
| integer divisibility witness | `p ∣ N` | `dvd_def`/witness elimination |
| integer linear bounds | order and ring operations on `ℤ` | existing `ℤ` instances; `omega` closes the stated linear chain |
| canonical base-six digit | `m % 6` | `Nat.mod` |
| quotient update | `m / 6` | `Nat.div` |
| radix decomposition | `(m % 6) + 6 * (m / 6) = m` | `Nat.mod_add_div` plus commutativity normalization |
| termination of recursive decoding | quotient is smaller for radix `≥ 2` | existing natural division bound, discharged by `omega` |
| inverse multiplication by six | `inv6 • (6 • X) = X` | explicit hypothesis; concrete subgroup instance remains a separate obligation |
| final EC equality implies scalar equality | injectivity of `fun k => k • G` on admitted domain | explicit `Function.Injective` hypothesis |

## Proof chains

Each Lean `calc` line will perform only one restructuring or replacement.

### Carry diamond

```text
2 • C3(h2) + C2(q)
= 2 • (3 • s(h6) - s(h2)) + (2 • s(h2) - s(q))   [definition]
= 6 • s(h6) - s(q)                                 [group/nsmul laws]
= 3 • (2 • s(h6) - s(h3)) + (3 • s(h3) - s(q))   [group/nsmul laws]
= 3 • C2(h3) + C3(q).                               [definition]
```

### Representative quotient

```text
N = p*w                                                   [divisibility witness]
-4*p < 2*a+b-3*c-d and 2*a+b-3*c-d < 3*p                [four interval hypotheses]
-4*p < p*w and p*w < 3*p                                 [replace N]
-4 < w and w < 3                                         [p>0]
-3 ≤ w and w ≤ 2.                                        [integer discreteness]
```

### Conditional decoder

```text
inv6 • ([m]G - [digit(m)]G)
= inv6 • ([m]G - [m%6]G)                                 [exact predicate]
= inv6 • ([6*(m/6)]G)                                    [Nat.mod_add_div, nsmul laws]
= inv6 • (6 • ([m/6]G))                                  [nsmul_nsmul]
= [m/6]G.                                                 [inverse-six action]
```

Repeated application is represented by `radixReconstruct`; its generic
strong-induction theorem is instantiated at radix six. Final EC verification
is a separate injectivity endpoint.

## Open bridge and claim boundary

The Lean result is conditional on: the four extracted carries really are the
canonical representatives of one first-formal-layer equality; the public
predicate returns the current scalar remainder modulo six; multiplication by
six is invertible on the admitted subgroup; and the scalar-action verification
map is injective on the admitted domain. Native producer/verifier checks cover
the first premise on the frozen toy cohort. Only experiment data can address
the second. The proof therefore establishes the algorithmic implication of an
exact oracle, never its empirical exactness or subexponential applicability to
generic prime-field curves.

## Verification receipt

```text
ELAN_HOME=/workspace/.elan-h113 XDG_CACHE_HOME=/workspace/.cache-h113 \
  lake build
# Build completed successfully (3093 jobs).

PATH=/workspace/.elan-h113/bin:$PATH \
ELAN_HOME=/workspace/.elan-h113 XDG_CACHE_HOME=/workspace/.cache-h113 \
  tools/lean/proof-gate.sh \
  --out research/h113_binary_trinary_winding_20261117/proof-gate-v1 \
  research/h113_binary_trinary_winding_20261117
# PASS  14 thm  0 trusted
```

The transitive axiom audit reports only `propext`, `Classical.choice`, and
`Quot.sound`; no declaration depends on `sorryAx`, a user axiom, or
`native_decide`.
