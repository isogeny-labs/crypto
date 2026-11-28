# R131 — N51 is a unit at all twelve degree-7 places

The actual 105-term R51 N51 coordinate has **valuation zero at every one of the twelve normalized F₂ places** above `x⁷+x⁶+x³+x+1`. All displayed N51 denominators specialize to units at the degree-7 x-residue. For each of R130's four lambda-factor groups, the N51 residue polynomial is coprime to the full factor product:

| Base factor | Lambda factor group | Places | N51 unit places | Zero-residue places |
|---|---|---:|---:|---:|
| `Y+123` | two degree-8 factors | 2 | 2 | 0 |
| `Y+90` | two degree-8 factors, inherited e=2 | 2 | 2 | 0 |
| `Q4` | four linear factors | 4 | 4 | 0 |
| `Q4` | four cubic factors | 4 | 4 | 0 |

Because every rational coefficient denominator is a unit and every residue is nonzero on its factor, `v_P(N51)=0` at all twelve DVRs. The complete degree-7 x-fiber contributes **zero** to `div(N51)`. This includes the two base-ramified places; their inherited ramification does not create a pole or zero of N51.

The committed `source-data/N51.sing` is the exact 31,516-byte N51 assignment excerpt from the SHA-pinned R51 donor. The producer pins the R102 parser and R130 fiber, evaluates all 105 rational terms over F128 and F₂²⁸, and computes polynomial gcds. An independent evaluator uses separate carryless field arithmetic and rational-expression evaluation, reproduces every N51 coefficient and remainder, and checks all four gcds equal one. It also matches the excerpt byte-for-byte to the available full R51 donor. Byte replay, changed-input rejection, and Lean 4.34.1 count/degree arithmetic passed.

Source/protocol commit `8c677cee8`; result SHA-256 `2b3180791d30cc2e659b02f6b738b2cfd25b619e2fd5ca7e1e1d63cf49d5f00b`. Local exact computation with zero cloud cost. The valuation interpretation uses the [normal-curve DVR criterion](https://stacks.math.columbia.edu/tag/0B8Y); Lean proves the finite counts, not the general criterion.

Next normalize and evaluate N51 at the degree-11 and degree-22 lambda-discriminant supports, then infinity. Together with R110 and R126 this advances the complete N51 divisor needed for the proper level-77 Riemann–Roch construction.
