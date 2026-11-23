# Genus-three signed-twist trace-parity obstruction

The frozen experiment's exact half balance is forced by a two-step theorem.

For the cover `z^2=y-y0`, let

```text
S(y0) = sum_(R in E(F_p), R != O) chi(y(R)-y0),
a(y0) = t_E-S(y0).
```

Changing variables by `R -> -R` and using `y(-R)=-y(R)` gives

```text
S(-y0) = chi(-1) S(y0).
```

Since `chi(-1)` is `1` or `-1`, the integers `t_E-S(y0)` and
`t_E-chi(-1)S(y0)` differ by either zero or twice `S(y0)`. Their residues
modulo two are equal. This applies independently to every multiplier `[m]Q`,
so the complete trace-parity feature vector is equal at `Q` and `-Q`.

If the subgroup has odd order `r`, `[-k]P=[r-k]P`, while `k` and `r-k` have
opposite parity. Any deterministic decoder receiving only the common feature
vector must therefore be wrong on at least one member of the pair.

Lean formalizes the signed-twist arithmetic interface, derives
negation-invariance, and composes it with the existing odd-orbit pair-failure
theorem. The modulus-three mutation proves that the key equality is special
to modulus two. It does not formalize the curve construction or claim a
subexponential decoder.

Verification with Lean 4.34.1 and the pinned Mathlib snapshot:

```text
Build completed successfully (3164 jobs).
audited 228 theorems in [H116FiberTrace]; 0 failing, 0 trusted (native_decide)
```

[pcc:unverified]
