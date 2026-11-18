# R129 — normalized degree-7 base fiber

The direct lift at the degree-7 double source-order root converges to a unique series `s(u)∈F128[[u]]` with `F(s(u),u)=0` through `u^127` and first terms `s=6u²+21u³+76u⁴+48u⁵+…`. Here `x=a+s` and `y=90+u`, with `a` a root of `x⁷+x⁶+x³+x+1`. The pinned R128 equation has `∂F/∂s(0,0)=84≠0`, so the simple-root/implicit-function criterion gives one smooth local branch with uniformizer `u`. Since the leading coefficient 6 is nonzero, `v_u(x-a)=2`. This certifies ramification index **2** at the formerly double base-order root.

Combining this branch with R127's simple root and irreducible quartic yields the complete normalized degree-7 **base** fiber:

| Source factor | F2 residue degree | Ramification index | Weighted degree |
|---|---:|---:|---:|
| `Y+123` | 7 | 1 | 7 |
| `Y+90` | 7 | 2 | 14 |
| `Q4` | 28 | 1 | 28 |

The weighted sum `7+14+28=49` equals the degree-7 map's fiber over a degree-7 closed point. The quartic and simple linear factors have nonzero `∂f/∂y`, so they lift without ramification over the base parameter. This is the base-cover census; the degree-16 lambda extension and N51 valuations remain to compute.

The producer uses the SHA-pinned R128 52-term expansion, solves the fixed-point equation through `u^127`, and checks all 128 residual coefficients. It converged in 33 updates. The independent verifier expands the tracked R51 equation by a different Horner route and solves each coefficient sequentially; all 125 nonzero series terms match. Byte replay, changed-source rejection and Lean 4.34.1 finite leading/degree checks passed. The first published producer stopped at a 16-update cap before output; the preserved amendment raised the cap to 130 before this accepted run. See `FAILURE-01.md`.

Source/protocol commit `e044f3da2`; loop-bound amendment `38551e66a`. Result SHA-256: `386eb38c51c8f0da0bb8d184ab91709bb1865235d6f6ea2068292713bd8d9e52`. Local exact computation, zero cloud cost. The branch uniqueness argument uses [Stacks' Henselian simple-root criterion](https://stacks.math.columbia.edu/tag/04GE) and the [normal curve/DVR criterion](https://stacks.math.columbia.edu/tag/0B8Y); Lean checks the finite leading and degree arithmetic, not those general theorems.

Next specialize and normalize the degree-16 lambda equation over these three base completions, then evaluate N51 on every resulting place. The degree-11 and degree-22 lambda-discriminant fibers and infinity remain separate.
