# R122: final repeated x=0 edge splits 1+7

R122 follows the y0/e3 eight-root edge left by R121. Its next F4 correction is `delta=1` for both conjugate initial roots, because the previous leading constant is the Frobenius-fixed element 1. Substituting `U=s(1+V)` and dividing by `s^8` gives exact coefficient orders at V-powers `[0,1,4,8,16]` of `[4,2,11,0,24]`. The lower Newton polygon has positive edges `(0,4)` to `(1,2)` and `(1,2)` to `(8,0)`. Hence one V correction has order 2 and seven have order `2/7`. Every one of the four x=0 base places now exposes a `1+7` split near each F4 lambda residue root, with a tame denominator-7 branch. A complete normalized branch census and the N51 pullback remain the next calculation.

The first local producer run failed before writing a result because its control incorrectly required the two conjugate corrections to differ. The true correction is 1, fixed under Frobenius. The corrected source checks Frobenius conjugacy, was committed at `42032ff4b`, and then produced the accepted result. This failure and correction are preserved in the record.

Independent F4 tables reproduce all translated supports and cutoffs. Changed R121 input is rejected, fresh replay is byte-identical, and Lean 4.34.1 checks the finite lower-edge inequalities and on-edge index sets from generated result-bound constants. Lean does not formalize the full rational substitutions or Newton polygon theorem. The PDF was rendered and visually inspected. No accepted-run verifier, Lean or rendering failure occurred.

Result SHA-256: `49710620cb89fb763fe13f952f64cfdc725094d87786c991d844a331fdd840fa`. Replay: `python3 research/koblitz_modular_ic_20261018/rounds/122/translate122.py <output-directory>`. Independent check: `python3 research/koblitz_modular_ic_20261018/rounds/122/verify122.py`. All work local, zero cloud cost.

Sources: R116–R121 local series and translated edges; [Stacks Henselian local rings](https://stacks.math.columbia.edu/tag/04GE) and [Stoll's Newton polygon notes](https://www.mathe2.uni-bayreuth.de/stoll/teaching/pAdicAnalysis-WS2015/Skript-pAdicAnalysis-pub-print.pdf).
