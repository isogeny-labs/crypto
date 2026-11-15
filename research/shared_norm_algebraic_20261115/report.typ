#set page(paper: "a4", margin: (x: 18mm, y: 15mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9.2pt)
#set par(justify: false)
#align(center)[
  #text(size: 19pt, weight: "bold")[Shared norm algebraic IC]
  #v(3pt)
  Fixed-fixture end-to-end prototype • 15 November 2026
]
#v(6pt)
The native candidate builds a factor base algebraically, solves unknown
norm/remainder equations, extracts rational points and recovers a discrete
logarithm through signed relations. Separate OpenSSL arithmetic replays the
complete finite controls. These producer-local results have standing
*PCC-20261115-be28ec: unverified*.

#align(center)[#image("pipeline.svg", height: 151mm)]

#text(size: 8pt)[Diagram: actual fixed F17/order19 path; links in the SVG point
to source. Counts are exact software controls of different components, with
no comparable timing metric or statistical uncertainty model. Diagram used
instead of a performance graph. Source: pipeline.dot; raw records: evidence/.]

#pagebreak()
#text(size: 16pt, weight: "bold")[Encoding and checked scope]

For odd `m=2h−1`, monic A of degree h, C of degree at most `h−2`, and monic U
of degree m, encode

```text
A² − (X³+aX+b)C² = (X−x_R)U,
A(x_R) − y_R C(x_R) = 0,
V_0 = X; V_(j−1)² = Q_j U + V_j; V_t = c.
```

The target inverse enforces `U(x_R)≠0`. A separate witness-pruned mode omits
only two Bezout blocks: nonzero Kummer membership implies squarefree U;
target separation makes `(X−x_R)U` squarefree, forcing `gcd(U,C)=1`. This
removes 9,16,23 variables at arities3,5,7. Shared Lean supporting laws have a
focused build and standard-axiom audit. Degree-bounded witness lifting and
the complete retained-circuit endpoint remain unformalized.

#table(
 columns: (1.22fr, 1.78fr), inset: 6pt, stroke: 0.4pt + rgb("b3c2cd"),
 table.header([*Control*], [*Observed local outcome*]),
 [Quotient extraction], [F5 grid25; nonreduced, extension-only and unit quotients; malformed/capped inputs rejected.],
 [Retained explicit system], [28 models / quotient28 over F17; identical generator coefficient maps.],
 [Fresh m3 models], [90 cases /400 models in both encodings; complete sets agree with OpenSSL.],
 [Cold point-only IC], [18 targets;72 accepted relations;18 kernels replay in both encodings.],
 [Arity5,t3,c16], [2 complete models; quotient8 in both encodings.],
 [Arity5/t4; arity7/t4], [Unknown at unchanged20s backend caps in both encodings; reference counts88 and56.],
 [Tamper controls], [Wrong scalar and duplicate model-control case rejected.],
)

#v(5pt)
The cap is a configured limit, not a measured runtime. No speedup, scientific
benchmark, fitted exponent or leaderboard update is claimed. The original
criterion—a complete classical subexponential algorithm for prime fields—
remains unmet. Small quotient dimension controls extraction only after an
initial basis exists; its construction cost and effective useful-target yield
are still open. Supplied GB provenance is not independently certified.

#text(size: 8pt)[
*Reproduce:* README.md and build.sh. Full records and source/binary custody:
VALIDATION.md. Prospective controls: PROTOCOL.md (including timestamp/custody
corrections). Library pins and proof status: chart-pruning.md.

*Sources checked:* Petit–Kosters–Messeng, PKC2016, sections3.1–3.4
(people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf); existing one-shot
continuation / initial-elimination obligation; PRs4091,4147,4188. Code-decoding
boundary and Cheng cs/0507026 were checked without a new mechanism claim.

*Evidence custody:* source4930ca5e0 (final original encoding),574cc3b24
(witness-pruned mode); three whole-panel xz archives. Initial models-001
binary was not retained before instrumentation; models-002 corrects custody
without rewriting history. No signed external verdict is asserted.
]
