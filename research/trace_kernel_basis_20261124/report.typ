#set page(paper: "a4", margin: 17mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.65em)
#align(center)[
  #text(size: 20pt, weight: "bold")[Trace-kernel auxiliary basis selection]
  #v(3pt)
  Native source component · 24 November 2026 · PR #5121
]

== Construction
The Rust component keeps the auxiliary trace hyperplane and all three leaf
domains fixed. It applies a standard minimum-weight vector-matroid basis
algorithm to an explicitly additive coefficient-support objective in a
three-summand chained-S3 representation. This is a representation proxy,
not a relation solver or a measured runtime result. [pcc:unverified]

#image("basis-selection.svg", width: 100%)

== Why the objective separates
Write the auxiliary coordinate as u = o + sum of b_j z_j, where o is fixed
and the b_j form a basis of the trace kernel. In the Boolean quotient,
squaring is linear. Each S3 equation is affine in the auxiliary bits;
terms labeled by different z_j have disjoint monomials. Total descended
support is therefore C + sum of w(b_j). Sorting all nonzero kernel vectors
by weight and keeping rank increases applies the classical minimum-weight
basis algorithm to this objective. [pcc:unverified]

The candidate enumeration has 2^(n−1)−1 elements. Its preprocessing is
exponential; neither that search nor coefficient support establishes cheap
solving. Direct trace parameterization itself is exactly the ordinary trace
equation eliminated with the same pivot. [pcc:unverified]

#pagebreak()
== Evidence and next test
Development controls cover field arithmetic, exact trace hyperplanes,
coefficient equality after elimination, symbolic-versus-direct evaluation,
additive support, exhaustive small basis choices, and a wrong-trace mutation.
See the source-bound logs under evidence/ for actual command outcomes.
They are producer records, not signed independent verdicts. [pcc:unverified]

The three-size support panel is implemented and preregistered but not run.
It uses degrees 7, 9 and 11, a common fixed leaf domain, disjoint training
and held-out target coordinates, and standard/random/optimized kernel bases.
S3 checkpoint transport and a known paid-budget balance are absent; no
research job has been submitted or rerouted locally. Actual solving and
independent-relation costs require a subsequent matched native adapter.

== Proof and publication scope
PROOF.md gives the derivation, exchange argument and proof map. Lean/lake are
not installed. The existing IDC1 v1 checker only evaluates over the fixed odd
prime 2^61−1 and does not certify these binary-field identities. No incompatible
certificate or independently signed proof standing is asserted.

Canonical performance charts remain unchanged: there is no admitted timing,
priced pipeline result or new leaderboard row. The existing successful-work
problem record remains open.

== Sources and reproducibility
#link("https://arxiv.org/abs/1503.08001")[Kosters–Yeo, Notes on summation polynomials],
Proposition 4.2; #link("https://doi.org/10.1007/BF01584082")[Edmonds, Matroids and the greedy algorithm]
(1971), paragraphs 5–7 and 19–28; repository shape-search and Hamming-ideal
notes cited in README.md. KB unavailable; primary/source-file fallback used.

Source: research/trace_kernel_basis_20261124/native/. Run Cargo's locked release
tests and strict Clippy using the manifest in that directory. PROTOCOL.md
contains the separately admitted research commands and stopping conditions.
