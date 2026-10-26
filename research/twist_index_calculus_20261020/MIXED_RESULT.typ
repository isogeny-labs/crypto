#set page(width: 210mm, height: 297mm, margin: 17mm)

#set text(font: "DejaVu Sans", size: 10pt)

= Mixed-component quadratic-twist index calculus

On six prime-order E/Fp curves, three base sizes, and every nonzero target, the exact mixed extension-field solver recovered all 49,248 IC target-arm scalars. The full relation census also covered every nonzero target for all four arms. Mixed-unique trace rank was B−2 in all 18 cells; mixed-rect trace rank was B/3. The native projected base admits every exact mixed relation by Frobenius projection.

Complete cold L0 time is descriptive. Mixed-rect median time was 2.71–7.78 times its matched native-trace median across 18 cells. Nine sealed ecbench native IC/rho sessions reproduced their counted work; one strict B=36 session retained 18 exhausted IC runs, and the later target-pin session recovered them. There is no isolated mixed-versus-rho speed claim: the mixed candidate has not been ported into ecbench.

The matched rational-only mimic control reproduced the complete mixed-unique relation set and all 12,312 target descent paths with identical rational additions. It used zero twist additions; mixed-unique used positive twist additions in every paired run. Its L0 median time was lower in all 18 curve/base cells.

#image("MIXED_COST.svg", width: 100%)

Figure: complete cold mixed-rect and native-trace runtimes for every nonzero target. Error bars show nearest-rank 5th–95th percentiles; L0 wall time is descriptive.

#image("MIMIC_COST.svg", width: 100%)

Figure: matched rational-only mimic and mixed-unique complete cold medians; L0 time is descriptive.

Full records, methods, exact tables, audit receipts, and caveats are in MIXED_REPORT.md.
