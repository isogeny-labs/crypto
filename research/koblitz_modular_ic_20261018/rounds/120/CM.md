# R120: second F4 translation of the three repeated edges

R120 follows the eight-root R119 edges on y0/e1, y0/e3 and y1/e2. For each F4 residue root, its leading correction `beta` is uniquely determined in F4 by `beta^8` equal to the prior constant's leading coefficient. Substituting `Z=s^m(beta+W)` and dividing by `s^(8m)` gives exact coefficient orders at powers `[0,1,4,8,16]`:

| base place | m | translated orders | positive lower edges |
|---|---:|---|---|
| y0/e1 | 1 | `[16,16,19,0,8]` | `(0,16)` to `(8,0)` |
| y0/e3 | 1 | `[8,16,19,0,8]` | `(0,8)` to `(8,0)` |
| y1/e2 | 2 | `[13,9,15,0,16]` | `(0,13)` to `(1,9)` to `(8,0)` |

Thus the y0/e1 and y0/e3 repeated clusters have next correction orders 2 and 1 in W. The y1/e2 cluster splits `1+7`: one W correction has order 4, seven have order `9/7`. In the original lambda coordinate, the seven roots have correction order `2+9/7=23/7`, matching the seven-root exponent from y1/e1 in R119. The simple y1/e2 correction has lambda order `2+4=6`; the simple y1/e1 correction has order 5. This shared denominator-7 pattern across both y1 base places is a concrete ramification lead for the degree-112 source. The y0 clusters still require further translation to resolve normalized branches.

The source/protocol were committed before data. Both F4 conjugates agree on orders. Independent F4 multiplication tables reproduce every translated support and cutoff. Changed R119 input is rejected; fresh replay is byte-identical. Lean 4.34.1 proves the finite lower-edge inequalities and on-edge index sets from generated result-bound constants. It does not formalize the full rational substitution or Newton polygon theorem. The PDF was rendered and visually inspected. No producer, checker, Lean or rendering failure was observed.

Result SHA-256: `97c55481691f06b5b294956842a1d162c7d83d61722db1c5b812d0953146d297`. Replay: `python3 research/koblitz_modular_ic_20261018/rounds/120/translate120.py <output-directory>`. Independent check: `python3 research/koblitz_modular_ic_20261018/rounds/120/verify120.py`. All work local, zero cloud cost.

Sources: R116–R119 local series and translated edges; [Stacks Henselian local rings](https://stacks.math.columbia.edu/tag/04GE) and [Stoll's Newton polygon notes](https://www.mathe2.uni-bayreuth.de/stoll/teaching/pAdicAnalysis-WS2015/Skript-pAdicAnalysis-pub-print.pdf).
