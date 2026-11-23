#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= Implicit phase and observation fibers
T-1925 · Prime fields · 2026-11-08

The supplied-constraint phase kernel now avoids dense Dickson expansion. A separate counting argument identifies how much information the first compressed observation can retain and narrows the next annihilator construction.

#image("flow.svg",width:100%)

*Summary-only bound.* Rotations scale the two oriented modes reciprocally; reflection exchanges them. Their product and mode0 are therefore binary-bracelet invariants. With at most B_m summaries and L−δ images, some summary requires annihilator degree at least ceil((L−δ)/B_m). B₃=4 and B₅=8. The bound concerns summary-only candidate completeness; it does not restrict using the public point, full oriented transcript or other observations.

*Kernel.* Evaluate D_L modulo sf(q) by Dickson doubling, then take gcd(sf(q),D_L−2ε). Every product is reduced immediately. The supplied q is charged separately; it is not constructed from a target by this component.

*Qualification.* 3,120 dense remainders, 2,112 squarefree intersections and six large-exponent controls pass. Degree-24 constant-root constraints over F_998244353 cover L through 998,244,352, with at most 47 tracked recurrence coefficients and 66 quotient products. These are not peak-memory or timing measurements. 8,184 binary words, 180,208 invariance comparisons and 36 observation fibers pass. PCC-20261108-448fc5 is unsigned producer evidence.

*Failures and custody.* Validation01 lacked the separate GCD include. Validation02 passed kernel checks, then panicked because the fixture prime list omitted m=7 coverage. Both failures and snapshots remain. Adding the existing GCD module and prime29 gives a fresh successful validation03. Two copied sources and six dependencies are hash-bound; limits90/60 seconds. No recovery adapter changed.

*Next construction.* Use multiple radii, oriented modes or a direct public-point relation to obtain a useful annihilator with accounted construction cost and completeness. A uniformly small first-summary annihilator at fixed m cannot cover every compatible candidate.

*Sources.* NIST DLMF18.5.1–2; Mestrovic arXiv1804.00992v1, introduction and orbit-counting formula; native polynomial/GCD code. KB unavailable. Independent signed review pending.
