#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(justify:false,leading:0.55em)
#text(size:18pt,weight:"bold")[One-relation alternative]
#v(5pt)
#text(size:12pt)[Exact coverage argument and the charged selector cost]
#v(7pt)
The published Amadori–Pintore–Sala (APS) route builds a random labelled base
and seeks one zero-sum relation. It removes auxiliary-curve setup and the final
relation matrix. This source review derives a finite existence bound and
checks recovery/accounting, while leaving the required fast selector open.
*PCC-20261115-3dfae3 remains unverified.*

#image("pipeline.svg",width:100%)

Let G=〈P〉 have odd prime order r, Q=kP. Draw all a,b independently and
uniformly modulo r. The m disjoint parts contain b records each. They are
records with private labels, not supplied base logarithms. The selector sees
only public points and independent coins. Actual group relations and the
final scalar are checked. A larger elliptic cofactor is compatible with this
promise-problem reduction.

This assessment includes a complete local Lean check of finite-group coverage.
It includes no native numerical run, timing, fitted exponent or complete
classical subexponential ECDLP algorithm.

#pagebreak()
#text(size:16pt,weight:"bold")[What follows, and what remains open]
#v(7pt)
Count positive zero-sum tuples choosing one point per part. With L=b^m,
distinct tuples have pairwise independent zero-sum events: in a differing
part, each tuple has a uniform point unused by the other. Thus
#raw("E Z = L/r;  E Z² = L/r + L(L−1)/r²",block:true)
Cauchy–Schwarz gives Pr[Z>0]≥L/(r+L−1), hence at least 1/2 when L≥r.
With B=mb, infinity/repeated-abscissa rejection loses at most B²/r. For
B²/r≤1/4, a retained clean base is covered with probability at least 1/4.
This subtraction does not assume independence after rejection.

At fixed public points, (a,b)↦(a+kb,b) is bijective. Point-only selection
therefore leaves private b labels independent uniform. One selected signed
tuple has a nonzero record coefficient vector, so its β is uniform modulo r.
If β≠0, return −α/β and check the scalar; otherwise restart. A selector that
uses private labels falls outside this argument.

Choose n=ceil(log2 r), m=max(3,ceil(sqrt(n))), b=ceil(r^(1/m)). Test 4B²≤r
explicitly; use exhaustive scalar search on its finitely many failing r.
If a known capped selector answers with probability at least theta on clean
bases, a safe expected bit-cost envelope is
#raw("O((2^(O(sqrt(n)))*poly(input_bits) + T) / (theta*(1−1/r)))",block:true)
Both T and theta^-1 need effective subexponential bounds. Existence supplies
neither for an unspecified selector. Generic meet-in-the-middle retains an
exponential scale in n.

*Direct index assessment.* APS membership polynomials have degree b. The
current complete monomial Index architecture requires D≥b and allocates
#raw("M = binom(m+D,m) ≥ b^m/m! ≥ r/m! = 2^(n−o(n))",block:true)
at balanced m. This rejects that named direct allocation as a subexponential
implementation. It gives no lower bound for every sparse/circuit/GB method
and no solving-degree lower bound for the earlier auxiliary norm systems.

*Formal scope.* Shared source 2e4b08b constructs the actual finite-group
tuple-sum maps, counts single/pair events, derives both moments and proves
the integer support bound above. Its canonical all-source/axiom gate audits
134 theorems, 0 failures, 0 trusted native exceptions. All six build logs and
source hashes are retained. Earlier source ad0b100 checks label fibres,
recovery and the index inequality. Curve conditioning, asymptotics and
native programs remain outside these formal results. Hosted jobs ran zero
steps due to the explicit account billing lock; no hosted pass is asserted.

Sources: APS, FFA51(2018), sections3--3.2/equations7--11; exact native Index
source; existing oracle/code/GPU contracts. Spaenlehauer arXiv2610.32508v1,
Definition1.2/Theorem1.3 charges a univariate polynomial's degree and separate
elliptic preprocessing; it does not supply this multivariate selector. KB is
unavailable. Exact links/hashes and all scope distinctions are in SCOPE.md,
REPORT.md and checks/. No novelty or exhaustive-search claim is made.

#pagebreak()
#text(size:16pt,weight:"bold")[A fixed-slope coordinate join]
#v(7pt)
Wagner's birthday tree needs efficiently enumerable partial joins. As one
elliptic-coordinate candidate, choose a family Λ of s slopes independently
of the current random base and hash each list by y−λx. Equal buckets yield
points on one nonvertical line. This is a concrete join; its first-stage
admission must be charged.

#align(center)[#image("joins.svg",width:78%)]

Assume a nonsingular short-Weierstrass curve in characteristic >3 and
independent uniform subgroup point records before rejection. A line through
a known point leaves at most two other distinct-abscissa intersections.
The finite slope-family count is checked on Mathlib's actual Weierstrass
Equation and addition polynomial, without a new curve model.

At b=2^(O(sqrt(n))), J≤m=O(sqrt(n)) and subexponential s,W, the displayed
upper bound decays as 2^(−n+o(n)). It cannot supply inverse-subexponential
useful yield for this fixed-slope schedule. The probability, conditioning
and cost interpretation are hand arguments.

*Scope matters.* Choosing a slope after seeing a pair always admits that
pair; the fixed-family bound does not apply. Large implicit slope sets and
other coordinate/adaptive joins also remain outside this assessment.

*Formal evidence.* Shared source 867527c passes the canonical gate:
141 theorems, 0 failures, 0 trusted native exceptions. All three builds and
source hashes are retained. Hosted jobs ran zero steps due to the billing
lock. Claim6186b7 is unverified. PROOF-MAP.md records hypotheses, inheritance
and the retrospective instance-gate timing gap.

Sources: Wagner2002 full author version, sections2–3 and the scalar ROS
subsection4; Sutherland, MIT18.783 Spring2013 Lecture2, section2.1.2.
SELECTOR-JOINS.md gives the full derivation, controls and exact limitations.

#pagebreak()
#text(size:16pt,weight:"bold")[Implicit intervals: checking retention]
#v(7pt)
A large coordinate interval has a short description, so the small explicit
slope-family bound does not settle it. The next check asks whether old
interval constraints survive addition. Its model is two independent uniform
points in the interval point set; support membership alone does not establish
that model for a solver's selected outputs.

#align(center)[#image("intervals.svg",width:78%)]

Take a nonsingular short-Weierstrass E over F_p, p>3, whose full group has
prime order r. Set S={P≠O:x(P)∈I} for an abscissa interval I. The cited
character and interval estimates give the explicit safe input
#raw("M = 1 + 4*sqrt(p)*(1+ln p)",block:true)
and s∼r/b for |I|=floor(p/b), with b=2^(O(sqrt(log2 r))) growing.
The bound M/s is o(1/b), hence the return rate is (1+o(1))/b.

This refutes near-certain retention in that independent uniform subset
model. It prevents importing the XOR tree's automatic retention of old bits
without a new argument. The early identity probability is 1/s, also small.
Correlated/adaptive lists, other filters and actual join-enumeration costs
remain outside this inference; no general interval-selector lower bound is
claimed.

*Formal evidence.* Shared source 5b7a9d2 checks the finite-group energy,
Parseval, actual triple counts and flatness-conditioned deviation. The
canonical all-source/axiom gate audits 149 theorems, 0 failures, 0 trusted
native exceptions. Four builds and both assembly probes are retained.
The curve's analytic character estimate and sampling law remain external
hand bridges; no analytic axiom was inserted. Claims730397/ab6d22 are
unverified. This A3 inheritance plan and floor probe preceded its first edit.

Sources: Kohel–Shparlinski, ANTS-IV2000, sections3–4; Kim–Tibouchi,
NutMiC2019 conference manuscript, section2/Lemma3.1. Only f=x with pole order2
and p>3 is used, with trivial character terms handled separately.
INTERVAL-JOINS.md contains the proof, controls and limits.
