#set page(paper: "a4", margin: 17mm, numbering: "1")
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.65em)
#align(center)[#text(size: 20pt, weight: "bold")[Structure inside the trace-compatible set]
  #v(3pt)Conditional S3 fibers · 24 November 2026 · PR #5127]

== Identified structure
Each binary S3 block is a linearized quadratic A u² + B u + C once its
two leaf coordinates are fixed. For nonzero distinct a,b, the kernel
direction is delta = ab/(a+b)² = s²+s, where s=a/(a+b). It has trace zero.
The operator restricted to ker(Tr) has rank n−2, and its two possible roots
have the same trace. A basis change cannot remove that ambiguity.
[pcc:unverified]

A conditioned chart puts delta first in a basis of ker(Tr), exposing one
zero column and n−2 active coordinates. The native constructor checks this
alignment; it does not change the invariant rank. [pcc:unverified]

#image("structure.svg", width: 100%)

== Coupled scalar elimination
Combining two blocks gives D u = N. For the pairs (a,b) and (z,t),
D factors as (az+bt)(at+bz). Nonzero D gives a unique candidate N/D;
both original equations must still be checked. Zero D is a real branch,
including linear and constant cases, and is retained by the native adapter.
This is conditional elimination using established algebra, not a new
summation-polynomial algorithm. [pcc:unverified]

== A set structure that could make reuse useful
With r=a/b, dividing by b² fixes the operator
(r+1)²u²+ru across all pairs of that ratio. A coordinate set S with few
quotients S/S can reuse operator reductions and image tests. An actual
subgroup/trace-compatible subset of a length-L multiplicative progression
retains the bound |S/S|≤2L−1. [pcc:unverified]

No retained density is assumed. Membership, ratio search, pair enumeration,
lifting, sign checks and independent relation rank may erase the saving.
Small coefficient support and fewer distinct operators are not measurements
of complete solving cost. The reference must include standard half-trace,
which already reuses a field-wide linear map, and point addition where applicable.

#pagebreak()
== Native development controls
The library reuses the unchanged field implementation from merged PR5121.
It adds binary Gaussian roots, an independent odd-degree half-trace route,
complete coupled scalar roots, trace filtering and ratio normalization.
Controls compare root sets against exhaustive small-field evaluation,
check all scalar degeneracies, verify restricted ranks and compare the
two root methods. Read the source-bound logs in evidence/ for command outcomes.
They are producer observations, not signed scientific standing.
[pcc:unverified]

== Scope and open work
These are fixed-coefficient scalar roots. They do not solve for unknown
leaves, select an actual complete factor base, produce signed group relations,
or recover a target logarithm. Identity partial sums require a group-level
adapter. No research panel, paid cloud job or runtime comparison was launched.

The next discriminating comparison holds actual factor-base cardinality and
trace information fixed, measures quotient-class reuse and charges every
operator build, failed query, root/point check and rank update. Existing
checkpoint/budget/EKS admission remains required. Canonical performance
graphs stay unchanged because this component has no admitted cost result.

== Proof standing and sources
DERIVATION.md gives the rank, fiber, determinant and ratio arguments and a
proof map. Lean/lake are unavailable. IDC1v1 evaluates only at a fixed odd
prime, so no wrong-field identity certificate is issued. No independently
signed verification or novelty claim is asserted.

Sources read: #link("https://arxiv.org/abs/1503.08001")[Kosters–Yeo, Notes on summation polynomials],
Proposition4.2; #link("https://cacr.uwaterloo.ca/techreports/2011/cacr2011-07.pdf")[Taverne et al., Software implementation of binary elliptic curves],
Section2.4; repository target-conditioned symmetric-kernel and shape-search
notes. KB and Conductor are unavailable; primary/source-file fallback used.

Reproduce the component with Cargo's locked release library tests and strict
Clippy, using research/trace_linearized_fibers_20261124/native/Cargo.toml.
