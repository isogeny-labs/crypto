# Proof map: five adversarial cryptographic assumption boundaries

Date: 2026-11-17. Tracking PR: #4597.

This map freezes the formal scope before the first Lean edit. Each target
separates a counterexample to an omitted hypothesis from the theorem obtained
after adding that hypothesis. It does not claim an attack on a correctly
parameterized protocol.

## Requirement-to-evidence map

| Requested target | Lean module | Counterexample endpoint | Repair endpoint |
|---|---|---|---|
| ZK separation | `ZKSeparation` | perfect deterministic transcript simulation and completeness coexist with failed soundness and failed global accepted-transcript extraction | global accepted-transcript extraction implies soundness |
| Schnorr group/scalar bridge | `SchnorrScalarBridge` | the scalar map `k ↦ k * 2` on `ZMod 6` is noninjective | over prime `ZMod q`, multiplication by a nonzero generator scalar is injective |
| Fiat–Shamir transcript encoding | `FiatShamirEncoding` | two distinct typed pairs have the same raw concatenation and therefore the same hash input for every hash function | a tagged, length-framed codec has a left inverse and hence is injective |
| KZG toxic waste | `KZGToxicWaste` | knowing `tau` constructs an accepting opening for any claimed value when `z ≠ tau` | verification at `z = tau` is equivalent to `commitment = value`, independent of the proof |
| Survival tail versus expectation | `SurvivalTailExpectation` | `1/(n+1)` tends to zero while its partial sums diverge | a uniform lower stopping hazard gives a geometric survival upper bound and finite truncated-tail bound |

## Lean instance gate

### 1. ZK separation

1. **Structure above.** Existing `ProtocolCountermodels.Protocol` supplies the
   verdict-only protocol. The new semantic layer consists only of a relation,
   transcript generator, simulator, and acceptance predicate over existing
   types and functions.
2. **Theorem used as stated.** Reuse
   `ProtocolCountermodels.always_accept_complete_unsound` for the verdict-level
   boundary. The new structural implication is the one-line generic theorem
   `globalAcceptedTranscriptWitness_implies_sound`.
3. **Object as instance.** Statements are `Bool`, witnesses are `Unit`, and a
   transcript is the statement itself. Only `true` has a witness; acceptance
   is unconditional.
4. **Assembly probe.** From an extractor producing a related witness for every
   accepted transcript, apply it to any accepted transcript and conclude that
   the statement lies in the relation's language.
5. **Missing components only.** Definitions of completeness, deterministic
   perfect transcript simulation, soundness, and the deliberately stronger
   global extraction property; evaluation of the two-element model.
6. **Inheritance.** Function equality is `funext`; finite propositions close
   by `decide`/`simp`; the verdict-only separation is inherited from the
   existing theorem.

### 2. Schnorr group/scalar bridge

1. **Structure above.** `ZMod n` with its existing commutative-ring and
   additive-group structure; the scalar representation is multiplication by a
   fixed element.
2. **Theorem used as stated.** `mul_right_cancel₀` supplies injectivity over a
   field once the generator scalar is nonzero.
3. **Object as instance.** `ZMod 6` with generator scalar `2` is the negative
   instance; prime `ZMod q` with nonzero generator scalar is the repair.
4. **Assembly probe.** `3 ≠ 0` but `3 * 2 = 0 * 2` in `ZMod 6`; over prime
   modulus, cancel a nonzero right factor.
5. **Missing components only.** The named scalar map and its two endpoints.
6. **Inheritance.** Ring operations and normalization come from `ZMod`;
   cancellation comes from its field instance under `[Fact q.Prime]`.

### 3. Fiat–Shamir transcript encoding

1. **Structure above.** Lists, products, `Function.LeftInverse`, and
   `Function.Injective` from Mathlib.
2. **Theorem used as stated.** `Function.LeftInverse.injective`.
3. **Object as instance.** A transcript contains four byte-list fields and a
   finite domain tag. Encoding frames every field by a tag and its natural
   length. Decoding consumes the exact ordered frames and rejects trailing
   input.
4. **Assembly probe.** Prove `decode (encode t) = some t`; injectivity is then
   exactly `Function.LeftInverse.injective` after removing `some`.
5. **Missing components only.** Raw concatenation alias, framed encoder,
   frame parser, and the left-inverse lemma.
6. **Inheritance.** List append and length are inherited; domain equality is
   decidable; no hash property is assumed. Equal encodings imply equal inputs
   for every hash function by congruence.

### 4. KZG toxic waste

1. **Structure above.** A field for the scalarized verification equation and,
   separately, an additive commutative group/module for the group-form
   constructor.
2. **Theorem used as stated.** Field inverse/cancellation (`inv_mul_cancel₀`,
   `eq_div_iff`) and module distributivity.
3. **Object as instance.** Verification is
   `c = q * (tau - z) + y`. The trapdoor constructor is
   `q = (c-y)/(tau-z)` when `tau ≠ z`.
4. **Assembly probe.** Substitute the constructor and cancel `tau-z`; at
   `z=tau`, simplify verification to `c=y`.
5. **Missing components only.** Scalar verification, scalar/group trapdoor
   constructors, and exact exceptional-case theorems.
6. **Inheritance.** All algebraic steps inherit field/module laws. No pairing,
   polynomial-degree, ceremony, discrete-log, or computational assumption is
   encoded.

### 5. Survival tail versus expectation

1. **Structure above.** Real sequences, `Filter.Tendsto`, finite sums, and the
   geometric series identities already in Mathlib.
2. **Theorems used as stated.** `tendsto_one_div_add_atTop_nhds_zero_nat`,
   `Real.tendsto_sum_range_one_div_nat_succ_atTop`, and
   `geom_sum_mul_neg`.
3. **Object as instance.** The negative sequence is `S n = 1/(n+1)`. The
   repaired structure is a nonnegative normalized survival sequence satisfying
   `S(n+1) ≤ (1-delta) S(n)`.
4. **Assembly probe.** Pair the two existing harmonic theorems. For the repair,
   induct to `S n ≤ (1-delta)^n`, sum pointwise, and close the finite geometric
   identity.
5. **Missing components only.** The paired counterexample endpoint and generic
   finite-horizon recurrence theorem.
6. **Inheritance.** Limits and harmonic divergence are inherited verbatim from
   Mathlib. Calling the tail sum an expectation still requires a probability
   space, a waiting-time variable, and a tail-sum integral theorem; those are
   not silently assumed.

## Statement classification

| Statement family | Kind | Owner | Quantified structure |
|---|---|---|---|
| Transcript extraction implies soundness | law | study semantic interface | arbitrary statement/witness/transcript types |
| Always-accept transcript model | instance obligation/counterexample | study | finite `Bool`/`Unit` model |
| Prime scalar-map cancellation | endpoint of existing field law | study | `ZMod q`, prime `q` |
| Raw concatenation alias | counterexample | study | `List Nat × List Nat` |
| Framed codec left inverse | instance obligation | study | typed transcript codec |
| KZG trapdoor constructor | generic law | study | arbitrary field/module |
| Harmonic tail pair | endpoint of Mathlib theorems | study | `ℕ → ℝ` |
| Geometric survival repair | generic law | study | arbitrary real sequence |

No theorem here is a curve-specific restatement of a generic elliptic-curve
law. There are no new syntax trees, custom axioms, heartbeat overrides, or
native-decision dependencies.

## Sources checked

- Goldwasser--Micali--Rackoff, *The Knowledge Complexity of Interactive Proof
  Systems*, SIAM J. Comput. 18(1), 1989, DOI 10.1137/0218012, author-PDF
  SHA-256 `17b24f25b180ba64559a089efb443337c61c916078f73be4c496bf8d27410222`.
- Bellare--Goldreich, *On Defining Proofs of Knowledge*, CRYPTO 1992,
  DOI 10.1007/3-540-48071-4_28, author-PDF SHA-256
  `42c8ead7376db2f818cd975c6ab5c3026ff98c39d24e447faebc3626165848cb`.
- Schnorr, CRYPTO 1989, DOI 10.1007/0-387-34805-0_22, PDF SHA-256
  `0adf1be26ae4b14dd21b433c2f05f7d64cc47428de9670b82c600fe34aea01df`;
  Pointcheval--Stern, J. Cryptology 2000; RFC 8235.
- CFRG *Fiat--Shamir Transformation* draft -04 (2026-11-02), SHA-256
  `d0764022bb92b717ab7ecbf985235e88c9ed088363a77bacd44b8a6bf6fdc193`;
  FIPS 204, FIPS 186-5, and RFC 9497.
- Kate--Zaverucha--Goldberg, ASIACRYPT 2010, DOI
  10.1007/978-3-642-17373-8_11, publisher-PDF SHA-256
  `0cca25e00b56e7ebe731e9b0ec489315e469e4df8000aced3ead55f0165f72e9`.
- Jacobson--Koblitz--Silverman--Stein--Teske, *Analysis of the Xedni
  Calculus Attack*, DCC 20 (2000), DOI 10.1023/A:1008312401197, author-PDF
  SHA-256 `981bf955f7aecc5df4f271837f6653bd7666e5026f59df44196609d4a1dd6bc7`.
- Repository predecessors:
  `ProtocolCountermodels.always_accept_complete_unsound`,
  `JKSSTExpectedTrials.truncated_expectation_eq_survival_sum`, and
  `JKSSTOmegaRuntime.inverse_order_hazard_isOmega`.
- **KB unavailable:** `scripts/research-kb` returned exit 127, “no executable
  found,” for status, four mechanism/negative/application queries, and the
  theorem query. Primary sources and committed predecessor modules were read
  directly instead.

