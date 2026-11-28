# H115 proof map

Freeze date: 2026-11-17. This map precedes Lean source and candidate data.

## Generic algebra layer

The Lean development must model an additive commutative group with commuting
endomorphisms `pi` and `V` and scalar actions satisfying

```text
pi*V=[p],   pi+V=[t],   r=p+1-t.
```

Required theorems:

1. `verschiebung_frobenius_factor`:
   `(V-1)(pi-1)=[r]` and the symmetric commuting order.
2. `cocycle_maps_to_target`:
   from `[r]R=Q`, `(V-1)((pi-1)R)=Q`.
3. `cocycle_change`:
   replacing `R` by `R+T` changes the cocycle by `(pi-1)T`.
4. `fiber_coset`:
   if `H=ker(V-1)` and `(V-1)c0=Q`, the fiber is exactly `c0+H`.
5. `rational_section`:
   under `pi Q=Q` and invertibility of `p-1 mod r`,
   `c0=[(p-1)^(-1)]Q` lies over `Q`.
6. `image_kernel_handoff`:
   under explicit finite-cardinality/rank premises,
   `image(pi-1|E[r])=ker(V-1|E[r])` and the cocycle map has frozen
   multiplicity `r`.

## Finite score and decoder layer

7. Define finite moment/channel/predicate tables abstractly; prove that a full
   score equal to row cardinality yields a pointwise radix-six digit oracle.
8. Prove radix-six reconstruction for a cyclic order-`r` action when
   `gcd(6,r)=1`, with independent final `[m]G=Q` verification.
9. State the end-to-end theorem conditionally on one unchanged predicate ID
   scoring perfectly on every row and the public moment implementation matching
   its specification.

## Cost boundary

The proof must not assert that kernel construction, moment evaluation or the
digit premise is efficient. Introduce named cost terms for extension
construction, torsion/kernel construction, `r` fiber terms, scoring and decoder
queries. A separate premise is required for any `exp(o(log r))` total bound.

## Non-goals

The proof does not establish external novelty, existence of an exact H115
predicate, a uniform moment shortcut, a generic prime-field algorithm, or a
subexponential runtime. The toy producer and independent verifier discharge
finite arithmetic only.
