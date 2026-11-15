#set page(paper:"a4",margin:(x:18mm,y:15mm),numbering:"1")
#set text(font:"DejaVu Sans",size:9.3pt)
#set par(justify:false)
#align(center)[#text(size:18pt,weight:"bold")[Capped Macaulay provenance]
#v(3pt)Native producer, symbolic checker and fixed controls • 15 November 2026]
#v(7pt)
The producer retains source-row identities. A separately compiled checker
requires those identities, all S-polynomial reductions and original-generator
reductions before accepting a basis. Six small ideals pass; four adversarial
certificates are rejected. The natural norm candidates remain incomplete.
*PCC-20261115-be28ec:unverified.*

#image("counts.svg",width:100%)
#text(size:8pt)[Same fixed F17 m3,t3,c1,R=(0,6) input; qualification-002.
Exact native counters, with no sampling intervals or timing claim. Sources:
counts.json,plot.py,complete raw stdout. Recorded pivots are not a separately
certified matrix-rank result.]

#table(columns:(1fr,1fr,1.5fr),inset:6pt,stroke:0.4pt+rgb("b3c2cd"),
 table.header([*Degree cap*],[*Rows / pivots*],[*Final checker outcome*]),
 [4],[451 /431],[Incomplete; unknown],
 [5],[1881 /1707],[Incomplete; unknown],
 [6],[6578 /5555],[Incomplete; unknown],
)

A nonzero remainder under an incomplete basis does not prove non-membership.
The initial diagnostic label was corrected by changing check order/wording;
acceptance still requires the same three obligations. Original panel, amended
panel and checker-only replay remain complete and unchanged.

#pagebreak()
#text(size:16pt,weight:"bold")[Verified scope and remaining work]

#table(columns:(1.1fr,1.9fr),inset:6pt,stroke:0.4pt+rgb("b3c2cd"),
 table.header([*Control*],[*Local outcome*]),
 [F5 grid],[Quotient4; all4 rational points],
 [F5 nonreduced],[Quotient4; one point],
 [F3 extension-only],[Quotient2; no rational point; nonunit ideal],
 [F5 unit / x / x,y],[Unit source identity and exact one-point ideals],
 [Wrong coefficient / forged unit],[Source identity rejects],
 [Missing original / incomplete GB],[Corresponding checker rejects],
 [Natural norm at D4/D5/D6],[No completed basis or norm relation],
)

The actual unit row is `1*x+4*(x−1)=1` over F5. Its semantic polynomial identity
and unit ideal are checked by a pinned Lean project:2 theorems, zero failing or
trusted native proofs, standard axioms only. Existing Mathlib Ideal laws are
used as stated. Native file/parser binding, general checker correctness and
actual grevlex/criterion formalization are separate missing bridges.

The instrument admits prime `p<2^31`, at most20 variables, degree1..6 and explicit
row/support/step/basis/certificate limits. Each native case has20s/2GiB bounds.
No scientific clock, speedup, population estimate, leaderboard change or paid
run is claimed. The new backend has not produced a norm relation or cold DLP
recovery. The parent msolve-based cold path is separate.

The conceptual monomial envelope `binom(v+D,D)` is subexponential at balanced
parameters if `D=O(m+t)`. The instrument does not establish sufficient useful
yield or an arbitrary full-width prime-field implementation. A failure here
is not a degree lower bound. The original complete subexponential-algorithm
criterion remains open.

Next: eliminate rowspace onto univariate columns, factor necessary constraints,
and directly verify candidate assignments, using a separately frozen witness-
only contract. This can avoid demanding a complete GB, while leaving cost/yield
as explicit obligations. No result for that route is asserted yet.

#text(size:8pt)[
*Custody:* source38c0d23859 (panel001),924cb61d26 (panel002),437c1abf16
(checker replay003); hashes precede affected calls. Three whole xz archives
25696/26068/2968 bytes, with SHA256SUMS. No failed candidate was dropped.

*Sources and reproduction:* README.md,PROTOCOL.md,PROOF-MAP.md,build.sh and
lean/. Existing norm selective-oracle/initial-elimination notes; Boolean Macaulay
and GBRL sources; Mathlib Ideal laws/monomial division module. Standard methods;
no novelty or exhaustive-review claim. Native checks remain producer-local,
not a signed independent PCC verdict.
]
