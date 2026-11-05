# R106: explicit base-completion uniformizers above x=1

Write `t=x+1` and choose `z=y` at the y=0 point or `z=y+1` at the y=1
point. In both cases set `s=t/z`. Substitution into the complete R105
base equation gives

    f51(1+s z,z)       = z^2 H0(s,z),
    f51(1+s z,1+z)     = z^3 H1(s,z).

The source-bound expansion checks that each H has zero constant term
and coefficient 1 on z, so `dH/dz(0,0)=1`. Complete-local Hensel lifting
therefore gives a unique zero-constant formal series z(s) in F2[[s]].
At precision 32 the computed series begin

    y=0: z=s^2+s^3+s^4+s^6+s^7+s^8+s^9+...,
         t=s^3+s^4+s^5+s^7+s^8+s^9+s^10+...;
    y=1: z=s^3+s^4+s^6+s^7+s^8+s^9+s^10+...,
         t=s^4+s^5+s^7+s^8+s^9+s^10+s^11+....

The exact 32-bit supports are in `result.json`. Both the transformed
equations and the original f equation vanish modulo s^32. Consequently
`v_s(z),v_s(t)` are `(2,3)` and `(3,4)`. Together with R105's irreducible
degree-3 and degree-4 local factors, this identifies s as a uniformizer
on each **degree-7 base completion**. It does not resolve the lambda
extension, whose q-adic branches and N51 valuations remain next.

The source-bound process exited zero. A fresh process reproduced the JSON
byte-for-byte and a changed R105 hash was rejected. An independent bit
series evaluator bound the Lean constants to the committed R105/R106 JSON
and recomputed both source residuals. Lean 4.34.1 checked those two exact
residuals modulo s^32 and the first nonzero z bits. The formal existence
argument uses [complete-local Hensel's lemma](https://stacks.math.columbia.edu/tag/04GE);
the Lean artifact certifies the finite truncations, not that theorem.

The raw producer/replay archive has SHA-256
`315e624c9045f92392cbccf6a2b24d880761b7df8693da99fb38c8e6aab1eb50`.
`result.json` has SHA-256
`c112aeedde039a3b36376a0e720e00f62da64c5abe336d77296b97baa4cc12b8`.
Next substitute both base series into the full G51 equation and compute
lambda Newton polygons over F4((s)), then lift N51 to each resulting branch.

Sources: R51 donor through R105's source hash, R105 Newton edges,
[Stacks Hensel lemma](https://stacks.math.columbia.edu/tag/04GE), and
[Stacks normalization](https://stacks.math.columbia.edu/tag/0C44).
