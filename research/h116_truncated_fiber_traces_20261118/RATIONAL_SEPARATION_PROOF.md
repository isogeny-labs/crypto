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

The current source has SHA-256
`0399fa0d64f2d6915c67fc5530fa085eb249dc770a3942f0738ab1683b53a71e`.
The pinned build succeeded and the exact repository auditor reports 124
theorems, zero failures, and zero trusted native decisions.

| Requirement | Evidence | State |
|---|---|---|
| Finite regular evaluations imply rational-function equality below the strict degree bound | `rationalMap_eq_of_eval_finset` | proved |
| The strict inequality cannot generally be weakened | `rationalMap_separation_strict_control` | proved |
| Frozen-panel distinct-x capacity | `RATIONAL_SEPARATION_CAPACITY_EXPERIMENT.md` | 8/8 independently replayed |
| Actual H116 numerators and denominators | full candidates recovered for three small base curves in v2 | finite-panel only; generic construction open |
| Denominator nonvanishing/pole audit on the sample set | complete in all eight v2 cells | finite-panel only; generic audit open |
| Cross-product degree below measured capacity | strict margins 4 through 1,226 in v2 | finite-panel replayed; generic bound open |

This is a verification bridge, not a construction of the missing rational or
scheme morphism and not an ECDLP speedup claim. [pcc:unverified]

