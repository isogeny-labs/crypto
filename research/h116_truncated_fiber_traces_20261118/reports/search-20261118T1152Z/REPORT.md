---
title: "H116 function-field degree bridge"
subtitle: "Literature/API search and Lean checkpoint · 2026-11-18"
documentclass: extarticle
geometry: margin=0.68in
fontsize: 9pt
---

# Result

The search closes two algebraic layers of the H116 degree obligation.
[pcc:unverified]

1. `quadraticFunctionFieldSandwich_degree` proves that the degree of a curve
   map equals its coordinate-line degree once the two compatible function-field
   towers exist and both source and target curves are quadratic over their
   x-lines.
2. Mathlib's existing `RatFunc.finrank_eq_max_natDegree` proves that the
   coordinate-line extension degree is the maximum degree of a rational
   function's **canonical reduced** numerator and denominator. H116 exposes it
   as `rationalFunctionCoordinateExtension_degree`.
[pcc:unverified]

The pinned build succeeds. The exact repository auditor reports 137 theorems,
zero failures and zero trusted/native decisions. The Lean source SHA-256 is
`5457017ccc888e680065724d039ee73a4ccef3c5020ed263f814a39c50220c65`.
[pcc:unverified]

![Two compatible field towers and the unequal-projection control. Every arrow
is a field-extension degree, not a field-operation cost.](tower.png)

# The proof in one equation

Let `B` be the image of the target x-line inside the source curve field `E`.
Let `Xs` be the source x-line and `Xt` the image of the complete target curve
field. The two tower laws give

```text
[E:B] = [Xs:B][E:Xs] = d·2,
[E:B] = [Xt:B][E:Xt] = 2·[E:Xt].
```

Natural-number cancellation yields `[E:Xt]=d`. This conclusion uses total
function-field degree; it does not silently replace it by a rational-point
kernel count or by separable degree. Stacks Project Definition 29.52.8 defines
the degree of a dominant generically finite map by the corresponding finite
function-field extension, and Lemma 29.52.9 proves composition by field-tower
multiplicativity. Stacks Project Theorem 53.2.6 supplies the curves/function-
fields correspondence used to interpret the embeddings geometrically.
[pcc:unverified]

The negative arithmetic control is `4*3=2*6` with `4 != 6`. Thus a shared total
degree does not identify the adjacent map degrees when the two coordinate
projections have different degrees. This is why both quadratic hypotheses are
visible in the Lean theorem.
[pcc:unverified]

# What the rational-function theorem adds

For `f in K(X)`, Mathlib proves [pcc:unverified]

```text
[K(X):K(f)] = max(deg num(f), deg denom(f)),
```
[pcc:unverified]

where `num(f)` and `denom(f)` are canonical reduced polynomials. This is
stronger and safer than reading degrees from an unreduced displayed quotient:
any common polynomial factor has already been cancelled. It also pinpoints the
next scalar-specific proof obligation. For
`f = Phi_n / Psi_n^2`, the already-proved displayed degrees become the reduced
coordinate degree only after proving that `Phi_n` and `Psi_n^2` have no common
factor (or directly proving the canonical numerator/denominator degrees).
[pcc:unverified]

The pinned Mathlib division-polynomial API provides exact displayed degrees
and leading coefficients but the search found no elliptic division-polynomial
coprimality theorem in that directory. This is a statement about the pinned
revision and search terms, not a general absence claim. [pcc:unverified]

# Search receipt and sources

The research-KB MCP was not loaded and `scripts/research-kb` returned
`no executable found`; the fallback was the local lead atlas, repository-wide
`rg`, the pinned Mathlib source, and the primary Stacks Project pages.

- Stacks Project, [Curves and function fields, Theorem 53.2.6
  (0BY1)](https://stacks.math.columbia.edu/tag/0BY1): nonconstant morphisms of
  nonsingular projective curves correspond contravariantly to embeddings of
  one-variable function fields.
- Stacks Project, [degree of a generically finite map, Definition 29.52.8
  (02NY)](https://stacks.math.columbia.edu/tag/02NY): degree is the finite
  extension degree of function fields.
- Stacks Project, [multiplicativity, Lemma 29.52.9
  (02NZ)](https://stacks.math.columbia.edu/tag/02NZ): degrees multiply under
  composition by the field tower law.
- Pinned Mathlib revision `d13f23b723b8a846827a245b89c10fc7d3f11612`,
  `Mathlib/FieldTheory/RatFunc/IntermediateField.lean`, theorem
  `RatFunc.finrank_eq_max_natDegree`; its file cites Cohn, *Basic Algebra*,
  Theorem 11.3.4, and Jacobson, *Basic Algebra II*, Theorem 8.38.
- Repository precedent, `docs/curves/COVERS.md`: the existing prime and binary
  cover certificates use the same two-quadratic-function-fields tower
  argument for coordinate-line maps of degrees two and three.

Queries covered the exact mechanism, the cancellation/negative form, the
elliptic-map application, adjacent curve-cover arguments, and the Lean theorem
names. The decisive adjacent hit was Mathlib's rational-function
`finrank_eq_max_natDegree`; it changed the plan from assuming a coordinate
degree bridge to importing its exact reduced form.

# Scope and next technical step

This checkpoint does not yet construct the four H116 fields or their
embeddings, prove that the reconstructed point map is a dominant scheme map,
prove division-polynomial coprimality, prove duality of `V-1` and `pi-1`, or
establish any ECDLP runtime. The next frozen experiment counts regular and
branch fibers of `[2]` over splitting extensions, with under-split and
inseparable controls. In parallel, the next proof target is
`IsCoprime Phi_n Psi_n^2` under explicit characteristic hypotheses.
[pcc:unverified]

