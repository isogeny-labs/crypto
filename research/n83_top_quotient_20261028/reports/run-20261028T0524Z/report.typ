#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
#let rows=json("../../evidence/summary.json").rows
#let p=json("../../protocol.json")
#let terminal=json("../../terminal.json")
= Structural regularity bounds for fixed n83 PDPs
#text(fill:rgb("287d9c"))[All three curves · fixed ell27 · m=2–9 · 28 October 2026]

Observations and explicit mathematical certificates for the exact saved polynomial presentations. Producer replay passes; independent scientific/formal review remains pending. No elite factor-base or performance verdict is made.

All #rows.len() input certificates and their fresh-process replays completed. There are #rows.filter(r=>r.status=="finite_interval").len() finite-interval certificates and #rows.filter(r=>r.status=="positive_dimensional_top_ideal").len() certificates of positive-dimensional top-form quotients.

#image("bounds-grid.svg",width:100%)

== What the certificates establish
For binary/Koblitz m2, the original top-form degree of regularity lies in [28,55]. For m3–9, the lower bound is 27m+1 and the upper bound is the number of original variables plus one. Both target kinds give the displayed bounds. These are exact bounds from quotient maps, not estimates extracted from timeouts.

For prime m2, the interval is [29,55]. For prime m3–9, the top ideal maps onto a polynomial ring in one intermediate variable. Its quotient therefore has Krull dimension at least one. There is no finite first degree where that graded quotient vanishes under this study's degree-of-regularity convention. This does not assert infinite Castelnuovo–Mumford regularity or positive dimension of the original affine ideal.

== Connection to the preceding FFD/LFD data
The preceding fixed-base batch certified FFD=LFD=2 for the four binary/Koblitz m2 inputs. Their original top-form regularity is now bounded below by28. Thus these quantities cannot be identified for those exact presentations. This is an input-specific separation, not a general security statement or universal analysis of index calculus.

The same previous dataset has FFD2 for the larger binary/Koblitz inputs and prime FFD at least5. Their LFD remains unresolved. No new LFD value is inferred from a regularity bound or positive-dimensional top ideal.

#pagebreak()
== Certificate construction
Let H contain the highest homogeneous component of each original saved input equation. A graded surjection from R/H onto a square-zero polynomial ring on k variables preserves a nonzero squarefree monomial of degree k in the target. This proves dreg at least k+1. When every original variable square belongs to H, every degree above the total variable count vanishes, giving the upper bound.

Binary/Koblitz certificates parse all original terms, cancel before extracting tops, preserve field squares, and test two fixed retained-variable sets. Every top generator must vanish under the stated map. Failed maps retain surviving monomials. Prime certificates check the literal source presentation, exact degree-four top formulas and a coefficient-one monomial proving each top is nonzero. For m3–9, retain u0 and kill every selector and other intermediate. Every top generator vanishes, and the saved source has no field equation on u0.

Full arguments, assumptions and scope: DESIGN.md. File fields and monomial-hash encoding: FORMAT.json. The original equations and bases are unchanged; all hashes remain linked to the previous fixed-base dataset.

== Execution and validation
Started #terminal.started_at; finished #terminal.finished_at. Sequential native Rust on the existing host after the recorded EKS shared-bucket authorization failure. Eight GiB virtual-memory cap;600 seconds each producer/replay pass;1500-second outer budget; no retries. Times are diagnostics, not qualified performance measurements.

Five native tests pass, covering positive/negative projections, preservation of linear tops, missing-field rejection, cancellation order, and prime source tampering. All48 source identities and complete certificates pass fresh recomputation. The unified export validates; all raw files and the exact executable are preserved in S3 with a manifest and content-addressed archive.

Source: #text(size:8pt)[#p.arms.at(0).instrument.commit]
#linebreak()
Frozen protocol: #text(size:8pt)[#p.freeze.protocol_sha256]

== Implication for the elite-base search
The requested optimization remains open. These certificates identify structural obstructions in the saved polynomial presentation that a factor-base search must distinguish from solver behavior. Adding derived low-degree equations can change the top ideal while leaving solutions unchanged; that is a presentation change, not evidence of a better factor base. Future candidate comparisons must preserve both presentations, match base size, use held-out targets, and record regularity, solving degree and LFD separately.

#text("s3://setec-astronomy-artifacts/runs/n83-top-quotient-20261028-r1/local/",size:8pt)

Primary references: Caminata–Gorla, arxiv.org/html/2112.05579; Kosters–Yeo, arxiv.org/html/1503.08001. Earlier FFD/LFD dataset: research/n83_fixed_fall_catalog_20261028.
