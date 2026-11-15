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

This is a hand mathematical assessment with supporting local Lean laws.
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

*Formal scope.* Shared source ad0b100 checks the mixed-label fibre cardinality,
weighted-relation recovery and factorial/binomial inequality. Its canonical
local gate audits 111 theorems, 0 failures, 0 trusted native exceptions. The first
build lacked two explicit imports; the corrected second build and all logs
are retained. The second moment, curve interpretation, asymptotics and
native programs are not formalized by these supporting laws.

Sources: APS, FFA51(2018), sections3--3.2/equations7--11; exact native Index
source; existing oracle/code/GPU contracts. Spaenlehauer arXiv2610.32508v1,
Definition1.2/Theorem1.3 charges a univariate polynomial's degree and separate
elliptic preprocessing; it does not supply this multivariate selector. KB is
unavailable. Exact links/hashes and all scope distinctions are in SCOPE.md,
REPORT.md and checks/. No novelty or exhaustive-search claim is made.
