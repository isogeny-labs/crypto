# H116 finite rational-map separation proof

The pinned Lean artifact proves a finite evaluation certificate for supplied
rational functions.  If `N1/D1` and `N2/D2` agree on a finite set `S`, neither
denominator vanishes on `S`, and

```text
max(deg(N1*D2), deg(N2*D1)) < |S|,
```

then `N1*D2=N2*D1`.  The proof clears denominators pointwise and applies the
polynomial root bound.  The companion control proves over `ZMod 3` that
`X^3-X` is nonzero, has degree three, and vanishes on all three field elements;
therefore replacing the strict degree bound by `<= |S|` is unsound.

The source has SHA-256
`63ba08391dcf0de78d97df5b173d400c85f212cbd4f615f74d25573c6bde5588`.
The pinned build succeeded and the exact repository auditor reports 121
theorems, zero failures, and zero trusted native decisions.

| Requirement | Evidence | State |
|---|---|---|
| Finite regular evaluations imply rational-function equality below the strict degree bound | `rationalMap_eq_of_eval_finset` | proved |
| The strict inequality cannot generally be weakened | `rationalMap_separation_strict_control` | proved |
| Frozen-panel distinct-x capacity | `H116-RATIONAL-SEPARATION-CAPACITY-001` | preregistered, not yet run |
| Actual H116 numerators and denominators | no construction supplied by this checkpoint | open |
| Denominator nonvanishing/pole audit on the sample set | explicit theorem hypothesis | open |
| Cross-product degree below measured capacity | explicit theorem hypothesis | open |

This is a verification bridge, not a construction of the missing rational or
scheme morphism and not an ECDLP speedup claim. [pcc:unverified]

