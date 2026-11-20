# H116 division-torsor Frobenius proof map

Written after the frozen torsor experiment returned the constant zero
cocycle, and before modifying the Lean source. [pcc:unverified]

## Claim boundary

Let `G` be an additive commutative group, `sigma : G ->+ G`, and let `R` be
fixed by `sigma`.  Translation by `R` does not change the cocycle
`sigma(S)-S`:

```text
sigma(R+T) - (R+T) = sigma(T)-T.
```

If `[n]R=Q`, translating the `n`-torsion by `R` supplies the entire algebraic
shape of the division fiber, and subtracting `R` takes any fiber point back to
`n`-torsion.  Consequently the cocycle multiset on the translated torsion is
target-independent.  Selecting the rational point `R` itself forces cocycle
zero.

This proves the mechanism observed by the frozen selector.  It does not say
that every non-geometric selector chooses `R`, nor does it rule out a selector
using additional target-dependent structure.

## Dependency chain

1. Additive homomorphisms preserve addition.
2. The fixed-point premise rewrites `sigma(R)` to `R`; commutative-group
   cancellation leaves `sigma(T)-T`.
3. `nsmul` distributes over addition and subtraction, so `[n](R+T)=Q` for
   `[n]R=Q,[n]T=0`, and `[n](S-R)=0` for two points in the same fiber.
4. Pointwise cocycle equality lifts by function extensionality to equality of
   mapped multisets, preserving multiplicity.
5. Subtracting a fixed point from itself gives zero.

## Expected theorem surface

- `divisionFiber_of_torsionTranslate`
- `divisionFiber_difference_is_torsion`
- `frobeniusCocycle_fixedTranslate`
- `frobeniusCocycle_multiset_translate`
- `fixedPoint_frobeniusCocycle_zero`

The exact auditor must report zero failures and zero trusted/native decisions.
