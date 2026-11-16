# R119: first F4 lambda translation at x=0

The R118 lambda edge polynomial `1+T^8+T^16=(1+T+T^2)^8` has two conjugate roots in F4. For each of the four x=0 base places, R119 substitutes `lambda=s^(-k)(alpha+Z)` for each root `alpha`, divides by the old edge order, and combines the five actual coefficient series with source-specific precision cutoffs. The translated coefficient orders at `Z` powers `[0,1,4,8,16]` are exact:

| base place | translated orders | positive lower edges near each F4 root | leading constant in F4 for alpha=a |
|---|---|---|---|
| y0/e1 | `[8,23,23,0,0]` | `(0,8)` to `(8,0)` | `a` |
| y0/e3 | `[8,23,23,0,0]` | `(0,8)` to `(8,0)` | `a^2` |
| y1/e1 | `[28,23,23,0,0]` | `(0,28)` to `(1,23)` to `(8,0)` | `a` |
| y1/e2 | `[16,23,23,0,0]` | `(0,16)` to `(8,0)` | `a^2` |

The conjugate root `alpha=a^2` has the same orders and Frobenius-conjugate coefficients. The Newton polygon gives eight corrections of order 1 on each y0 place, eight of order 2 on y1/e2, and a `1+7` split on y1/e1: one correction of order 5 and seven of order `23/7`. The seven-root edge has leading relation `W^7=1` over F4; its denominator 7 is a concrete tame ramification lead. The remaining eight-root edges still have repeated initial polynomials and need further translation before their normalized branches or N51 valuations are determined.

The source/protocol were committed before calculation. Checks passed: changed R118 input rejected, byte-identical replay, independent two-bit-plane F4 evaluation of every translated constant and every coefficient support, Frobenius conjugacy, and Lean 4.34.1 checks for all finite lower-edge inequalities and on-edge index sets. The Lean file is generated from the recorded orders and does not formalize the entire rational substitution or the Newton polygon theorem. The one-page PDF was rendered and visually inspected.

Result SHA-256: `712278837f84ed1eba630a5aa368a534be7bdd381196ff27ff3fd1a0f52ffa71`. Replay: `python3 research/koblitz_modular_ic_20261018/rounds/119/translate119.py <output-directory>`. Independent check: `python3 research/koblitz_modular_ic_20261018/rounds/119/verify119.py`. All work local, zero cloud cost.

Sources: R116–R118 source-bound local series; [Stacks Henselian local rings](https://stacks.math.columbia.edu/tag/04GE) for the simple-root criterion; [Stoll's Newton polygon notes](https://www.mathe2.uni-bayreuth.de/stoll/teaching/pAdicAnalysis-WS2015/Skript-pAdicAnalysis-pub-print.pdf) for root valuations in characteristic p.
