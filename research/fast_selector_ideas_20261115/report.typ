#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:0.55em)
#text(size:19pt,weight:"bold")[Three fast-selector hypotheses]
#v(5pt)
Prime-field ECDLP · 15 November 2026 · research plan

*Status: unverified proposals; no implementation, experiment or speed result.*
The retained goal is a complete classical subexponential algorithm. This
document supplies candidate mechanisms and discriminating proof obligations.

#image("paths.svg",width:100%)

Use a nonsingular E: y²=x³+Ax+B over a prime field of characteristic >3 and
an odd prime-order subgroup of order r. Sample m disjoint lists of b
independent mixed-label records. Keep labels private, reject infinity and
repeated abscissas, and check every returned group relation and recovered
scalar. Proposals preserve that contract.

For n=ceil(log₂ r), m≈√n, b≈r^(1/m), B₀=mb and input length N, a useful
*target* is a capped attempt cost and answer probability satisfying
#raw("T <= 2^(C*m*log2(m+1))*B0^c*N^c\ntheta >= 2^(-C*m*log2(m+1))",block:true)
No such bounds are established here. Preprocessing, failed attempts,
branches, auxiliary points, output expansion and recovery must all be charged.

Full definitions, access failures, controls and source pins: PLAN.md.
Editable visual: paths.dot and paths.svg. No measured chart is changed.

#pagebreak()
#text(size:16pt,weight:"bold")[A. Rational interpolation; B. batched pencils]

*A — Search for a principal zero divisor through a small coefficient witness.*
Let Fᵢ be the product of the b abscissa factors of color i, and interpolate
Yᵢ(xᵢⱼ)=yᵢⱼ. Seek a nonzero f=u+yv in L(mO), with
#raw("deg u <= floor(m/2),  deg v <= floor((m-3)/2)\ndeg gcd(F_i, u + Y_i*v) >= 1 for every color i\nH = u^2 - (x^3 + Ax + B)*v^2",block:true)
The norm has degree at most m. Under the clean-point conditions, the
standard divisor criterion connects m such zeros to a zero-sum tuple.
This is a hand reformulation, not a newly verified theorem. Retain the
ordinate conditions: the norm alone forgets signs. [pcc:unverified]

Try factorized color constraints and approximation-module updates; compare
with colored singular-minor search in the L(mO) evaluation matrix.
*Missing mechanism:* a state representation with the target cap and useful
yield. A small coefficient witness does not imply a small search. First
write the exact merge/update operation and bound its retained states.

*B — Batch the two-point completion step of partial divisors.*
For m−2 anchors D, a basis f₀,f₁ of L(mO−D) supplies the projective key
[f₀(R):f₁(R)]. Compare keys in the final two colors, handle denominator
zeros, and check the actual relation. This is the degree-two pencil
interpretation after removing fixed zeros. [pcc:unverified]

One-anchor hashing is the baseline. Its unconditioned expected number of
completions is b²/r; fast updates do not remove the number of anchors.
The proposed advance would share updates and collision queries across an
implicit family, using modular composition or structured Schur complements.
*First test:* describe that family, exact query operations and every
expansion. Enumerating all anchor sums fails the desired cap.

#pagebreak()
#text(size:16pt,weight:"bold")[C. Correlated representations; research sequence]

*C — Use redundant representations rather than uniform-list closure.*
Add a charged pool of independent auxiliary records and replace a partial
X+Y by (X+T)+(Y−T). Distribute canceling shifts through a join tree so that
one original relation has many representations eligible for filters.
Track full primitive-record coefficient vectors. After cancellation the
answer must satisfy the original colored contract with a nonzero vector
modulo r. Derived local identities alone cannot count as useful answers.

Representation count alone is insufficient. For Y the number of surviving
representations of a useful relation, a required target is
#raw("E[Y]^2 / E[Y^2] >= 2^(-o(n))",block:true)
The needed moment estimate and construction cost are open. Start with an
exact four-leaf tree, its expanded record vectors and the survival moments.
Identify an operation using the curve's field equations; generic additions
alone supply no proposed cost improvement. Correlations must be charged.

*Sequence.* Start with A's exact module state. Use B as its first batching
subproblem. Keep C as an independent alternative. Implementations,
faithful formal statements, proof checks and numerical tests are pending.
No numerical, native, paid or frozen qualification run was started.

*Sources checked.* KB unavailable: no connector or repository-wrapper
executable. Read Cheng, arXiv cs/0507026, sections 2–5: the divisor/coding
connection is classical and worst-case decoding is hard. This does not
settle the proposed random-instance target.

#link("https://arxiv.org/html/cs/0507026")[Cheng primary text] ·
#link("https://doi.org/10.1007/978-3-030-31919-9_13")[Zhang–Liu 2019]

Zhang–Liu's publisher abstract was read; ePrint full text returned 403.
Delaplace–May's indexed primary excerpt concerns Fₚ²; full-paper access
failed, so no complexity claim is transferred to prime fields.
Earlier fixed-slope/uniform-interval assessments remain source-pinned at
crypto commit 204e6f7602; they do not analyze these proposed algorithms.
No novelty audit or implemented selector is asserted.

PR4311's later automatic merge changed its frozen PCC prefix. This plan is
published separately and does not rechain that history. Exact observation
pins and hashes are in publication-observation.json.
