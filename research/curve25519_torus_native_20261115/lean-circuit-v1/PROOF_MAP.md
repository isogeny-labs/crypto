# Selector numeric instance gate

1. Structure above: Core Nat and Mathlib ZMod, natural casts and valuation.
2. Laws used as stated: `Nat.cast_add`, `ZMod.val_natCast_of_lt`,
   `ZMod.val_ofNat_of_lt`, and
   congruence of equal values. Closed finite residue arithmetic uses `decide`.
3. Object: the actual p=2^255-19, selector x and slack d below32768, cap20632.
   This is a numeric cast-injectivity instance, not a generic circuit law.
4. Assembly probe: apply `ZMod.val_natCast_of_lt` to x+d and cap under the
   concrete p bound; read off equality from the existing cast law. Then
   obtain x<=20632 from nonnegative slack. The finite p17 counterexample
   is a direct decidable arithmetic instance.
5. Missing components: Boolean-vector decoding/bounds, the complete
   compiler semantics, actual Fp2/map/group instance bridges and any solving
   cost bound. Do not claim these from the numeric endpoints.
6. Inheritance: Nat addition/order/bit bounds from Core; ZMod ring/cast and
   valuation from Mathlib. No new field/group operations, axioms, native
   exceptions, limit overrides or frozen configuration edits.

| Statement | Kind | Owner | Existing law |
|---|---|---|---|
| actual_selector_no_wrap | Numeric endpoint | Study | Nat.cast_add, val_natCast_of_lt |
| actual_selector_range | Numeric endpoint | Study | Preceding equality and Nat order |
| small_modulus_counterexample | Numeric instance | Study | Decidable residue arithmetic |

The new project will reuse the already-pinned CryptoLean5f2ca251cb34806f69fd5455175434cac3567dfa,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2 and Lean4.35 contexts.
The source/manifest/toolchain are pinned together only after the final
focused audit. The endpoints do not establish a primality certificate.
