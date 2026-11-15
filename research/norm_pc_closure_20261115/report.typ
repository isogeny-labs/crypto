#set page(paper:"a4",margin:(x:17mm,y:16mm),numbering:"1")
#set text(font:"DejaVu Sans",size:9.5pt)
#set par(justify:false)
#align(center)[#text(size:18pt,weight:"bold")[Prime-field degree-fall closure]
#v(3pt)Source-certified elimination and first cold recovery • 15 November 2026]
#v(7pt)
Degree-4 polynomial-calculus closure completes the preserved natural norm
system. Its derivation DAG passes a separate native checker. The unchanged
index-calculus recovery path then solves one fixed cold target, with independent
OpenSSL replay of queries, relations, kernel and final scalar.
*PCC-20261115-be28ec:unverified.*

#image("pipeline.svg",width:100%)
#v(5pt)
#table(columns:(1.2fr,1.8fr),inset:6pt,stroke:0.4pt+rgb("b3c2cd"),
 table.header([*Fixed control*],[*Outcome*]),
 [Six small ideals],[Complete point sets / quotient dimensions match],
 [Four adverse proofs],[Reject at intended check],
 [Natural norm at D4],[One rational point; quotient1; completed GB],
 [Same norm at D5],[Outer-cap exit 137; unknown],
 [Same norm at D6],[Reduction cap; exit 2; unknown],
 [Cold Q=(0,6), E/F17],[Scalar 7; 9 queries; 4 relations; 1 kernel],
 [Independent OpenSSL],[Cold checks and old 18-target replay pass],
)

*What changed.* The earlier flat Macaulay method generates only original-input
monomial multiples through its degree cap. This implementation also multiplies
newly derived low-degree pivots, preserving local source derivations. The
method is standard polynomial calculus / MutantXL, with ordinary prime-field
monomials and coefficients.

*What the checker requires.* Every original-input reference, variable product
and field-linear combination reconstructs exactly, with intermediate degree
bounded by D. Final polynomials must equal their proof nodes; every S-polynomial
and every original generator must reduce to zero. Source membership alone
does not establish completion. A failed check returns unknown.

#pagebreak()
#text(size:16pt,weight:"bold")[Evidence, cost envelope and next obligation]

The fixed natural D4 trace records 2861 attempted rows, 1000 pivots, 184708
reductions and 3861 generated nodes. Its compact proof retains 1692 nodes and
165262 bytes. These native counters do not constitute an independent matrix
rank certificate. Intermediate degree 4 does not bound the degree of expanded
original-input multipliers.

The cold control supplies only public Q=(0,6), with seed 1234. The oracle sees
the curve/domain and each mixed point query, without scalar, base logs or
mixing labels. All 9 returned bases pass source/completion/original checks;
four yield relations and five yield no relation. The recovered scalar 7 is
checked by both the production path and OpenSSL. This is one finite software
control, not a public challenge or a yield estimate.

*Per-attempt reasoning.* With v variables and degree D, the monomial bound is
M=binom(v+D,D). At most M independent pivots queue at most vM products. Sparse
row work, DAG replay and completion checks through degree 2D have a polynomial
field-operation envelope in s,v,M. At balanced parameters a sufficient
D=O(m+t) would make that envelope exp(O(sqrt(n)log n)). No such sufficient
degree/yield bound is established by this control.

*Remaining work.* A growing admissible family needs an explicit successful
degree/yield hypothesis, all cold setup and recovery costs, bounded extraction,
and arbitrary-prime arithmetic. The source currently admits `p<2^31`, at most 20
variables, `degree<=6`, and explicit row/support/step/proof/quotient limits.
There is no new Lean proof of the native parser, order or verifier program.

*Custody.* Producer/checker bf6d29e42d; cold driver/include guard/replayer
b31a2ca3a9. Hashes precede affected calls. Deterministic raw archives retain
all successful/unknown outcomes and initial build/setup failures. Qualification
archive: 40784 bytes, directly tracked. Cold archive: 283208 bytes, uploaded and
downloaded successfully by artifact-autofix run 43690919442. Its manifest and
ignore rules land in bot commit dcc5c0c677; local hydrated verification passes.
A post-run qualification-manifest overwrite and the
labelled reconstruction are disclosed in the archive's custody note.

*Sources.* PROTOCOL.md, README.md, COST.md and the parent selective-oracle and
initial-elimination contracts; existing Boolean closure sources and repository
fall-degree literature notes. KB unavailable; primary papers were not newly
retrieved. No novelty, performance comparison, growth fit or T3 result is claimed.

The complete subexponential-algorithm objective remains active. Next: test a
numerically stated degree/yield hypothesis on a preregistered admissible family,
after implementing the required field, base and extraction interfaces.
