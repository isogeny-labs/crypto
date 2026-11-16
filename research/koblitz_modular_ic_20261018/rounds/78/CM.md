# R78: the full degree-32 source-model fiber at z=infinity

The R77 quadratic infinity coordinate is `U=z^-2 V`. In the retained
R50 overlap, `z=1/u` and `V=z^2 v`, so **`U=v`** wherever `u*z=1`.
Thus R77's `z=infinity` is `u=0` on R50's **finite-`u` chart**.
The R50 chart called “infinity” has `z=1/u` finite at zero and is
the wrong chart for this particular continuation. This corrects the
chart name in R77 `NEXT.md` without changing sealed R77 files.

The accepted R78 Singular program loads the complete R50 relative
equation over

```text
v^2+v+u^3+u = 0,
F(lambda,v,u) = 0,  deg_lambda F = 16.
```

It specializes **the full polynomial** at `u=0`, checks

```text
F(lambda,v,0) = lambda^16 + (v+1)lambda^8 + v+1,
F(lambda,0,0) = (lambda^2+lambda+1)^8,
F(lambda,1,0) = lambda^16,
```

and rejects an incorrect factor. The complete `u=0` algebra has
dimension 32 over `F2`; its `v=0` and `v=1` components each have
dimension 16. Their reduced residue algebras are `F4` and `F2`,
respectively. The two repeated factors account for every source-model
column, rather than merely a selected local point. A pinned Lean
theorem checks the exact chart overlap, a second proves the
characteristic-two eighth-power identity, and a third checks the
`16+16=32` arithmetic. All three compiled and passed an axiom audit
with zero failing and zero trusted. One native and one Lean attempt
passed; eleven historical source-byte pins and six donor hashes match.

These are facts about the **R50 source model fiber**. It has not been
shown normal at `u=0`; its exponents 8 and 16 are model-algebra
nilpotency lengths, not certified ramification indices or counts of
degree-32 branches. Every normal place above `v=0` has residue field
containing `F4`, while the full local degree budget there is 16;
the other quadratic place has the same budget 16. The next step is
the local integral closure and its primitive idempotents, followed by
individual `b,c,lambda,nu` and denominator orders.
