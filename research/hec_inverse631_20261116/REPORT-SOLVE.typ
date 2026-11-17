#set page(paper:"a4",margin:24mm)
#set text(size:11pt)
= F631 exact inverse parametrizations
Genus 3 · 15 November 2026

#table(columns:(1fr,1fr,1fr),[Case],[Outcome],[Independent check],[Positive],[Degree 12],[12 equations; known point],[D1],[Degree 12],[12 equations],[B original],[Solver failure],[Retained degree-11 output],[B amended],[Degree 12],[12 amended equations])

== Separation correction
The first solve returns valid squarefree, zero-dimensional parametrizations for the positive control and D1. The original B attempt fails: msolve reports a degree-eleven eliminant and recommends changing the separating linear form. Its exit status and empty result are retained.

A separately frozen amendment changes only the last equation to t = x1 + 2y1 + 4x2 + 8y2 + 16x3 + 32y3. All other bytes match. This produces a degree-twelve B parametrization. The independent verifier substitutes rational coordinate polynomials modulo the eliminant into every equation exactly, checks denominator inversion and squarefreeness, and accepts one numerator convention.

== Custody and interpretation
Original freeze 3ee7fb6c13e5 and amendment freeze 0bc845562ea9 each precede their own S3 checkpoint and execution. Neither the positive control nor D1 was rerun. All raw failures and successful parametrizations remain available.

The alternative collector executes B only. Its inherited aggregate summary mentions three cases; the per-case log and independent verifier are authoritative.

== Remaining mathematical gate
Parametrizations are not yet recovered rational target classes. Reconstruct divisor classes, account for centering and the source translation, and check the public adjacent coordinates for class and sign consistency. The global map and full destination index calculus remain unverified. No timing or tenfold gain is claimed.
