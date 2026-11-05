# R121: third F4 translation on the two y0 clusters

R121 follows the two repeated R120 eight-root edges. Each leading F4 correction `gamma` is uniquely determined by `gamma^8` equal to the previous constant's leading coefficient. Substitution `W=s^m(gamma+U)` and division by `s^(8m)` gives exact coefficient orders at U-powers `[0,1,4,8,16]`:

| base place | m | exact translated orders | positive lower edges |
|---|---:|---|---|
| y0/e1 | 2 | `[6,2,11,0,24]` | `(0,6)` to `(1,2)` to `(8,0)` |
| y0/e3 | 1 | `[8,9,15,0,16]` | `(0,8)` to `(8,0)` |

The y0/e1 cluster now splits `1+7`: one U correction has order 4, seven have order `2/7`. Its degree-7 slope gives a third tame seven-root branch lead, alongside y1/e1 (R119) and y1/e2 (R120). The y0/e3 cluster remains repeated with eight U corrections of order 1, requiring one more lift. The current local equations do not yet yield a complete normalized divisor of N51.

The source/protocol were committed before data. Both F4 conjugates give the same orders. Independent F4 multiplication tables reproduce all translated supports and cutoffs. Changed R120 input is rejected; fresh replay is byte-identical. Lean 4.34.1 checks the finite lower-edge inequalities and on-edge index sets from generated result-bound constants. It does not formalize the complete rational substitutions or Newton polygon theorem. The PDF was rendered and visually inspected. No accepted-run producer, verifier, Lean or rendering failure occurred.

Result SHA-256: `2baa0323bee345326da7196889598dd752a75ed6f20575a2b3348c856eb44053`. Replay: `python3 research/koblitz_modular_ic_20261018/rounds/121/translate121.py <output-directory>`. Independent check: `python3 research/koblitz_modular_ic_20261018/rounds/121/verify121.py`. All work local, zero cloud cost.

Sources: R116–R120 local series and translated edges; [Stacks Henselian local rings](https://stacks.math.columbia.edu/tag/04GE) and [Stoll's Newton polygon notes](https://www.mathe2.uni-bayreuth.de/stoll/teaching/pAdicAnalysis-WS2015/Skript-pAdicAnalysis-pub-print.pdf).
