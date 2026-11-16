#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set heading(numbering: none)
= RR Dickson matrix-reduction continuation
10 November 2026 · Implementation report · Producer-unverified

The opt-in native candidate eliminates constant-coefficient pivots only when
every remaining input equation stays within the original degree bound. It
emits a reverse reconstruction map with original/reduced input hashes. A
production lift command reconstructs arbitrary reduced solutions and checks
every original equation. No F4 experiment or cost comparison has run.

#image("route.svg", width: 100%)

== Why this candidate
The November 7–8 P1 follow-ups report lower RR solving degree with Dickson
membership at m = 3, but arm F has narrower matrices. The m = 5 tower cell
remains censored. Those observations motivate testing redundant-variable
elimination; they do not establish that this candidate helps. msolve may
already perform equivalent preprocessing.

== Implemented and checked
Two explicit modes: affine pivots only, or constant pivots from equations up
to degree two. Neither divides by a variable. Every accepted substitution is
checked against all remaining equations for input-degree growth. Native unit
tests cover exhaustive small-field solution/lift equivalence, degree-cap and
nonconstant-pivot rejection, inconsistent constants, inverse scaling,
dependent reconstruction, malformed maps and an existing tiny Dickson fixture.
Historical emitter calibration remains in the upstream selftest. These are
development checks, not signed PCC verification or a performance measurement.

== Missing measurement and qualification
Input degree does not bound F4 solving degree. F has an x-only target; RR fixes
sign and uses a monic-function chart. Complete point extraction and chart
normalization must precede an equal-output claim. The protocol compares RR,
RR-affine, RR-quadratic, F and T on identical inputs, charging transformation,
solve, extraction, reconstruction, verification and failed queries.

The checkpoint gate failed after the task resume was filled: no AWS profile
or checkpoint upload token is configured. GitHub authentication exists, but
that submission route does not waive custody or live budget/pool admission.
Budget headroom is unknown. No paid run was dispatched and no local benchmark
substituted for it. Development and review continued through this blocker.

== Sources and exact next action
Repository sources: research/split_prime_ecdlp_20261107/followups/p1_rr_dickson/
RESULTS.md, PROTOCOL-R2.md, p1tool/README.md; research/ic_tree_gluing_20261103/
round7_rr/README.md and STATUS.md. Primary source:
#link("https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf")[Petit–Kosters–Messeng, PKC 2016].
KB review unavailable: no loaded MCP and repository wrapper reports no executable.

Resume from HANDOFF.md. Restore checkpoint transport, verify live budget and
pool capacity, qualify the complete-output normalizer, then freeze the input
manifest and source-built image before the matched m3 panel. Success requires
retained lower solving degree, smaller matrices and lower complete equal-output
cost. No such outcome is asserted here. Full details: README.md, PROTOCOL.md,
checks/native-tests.txt and the linked problem record. Historical scoreboards
remain unchanged because there is no new scientific measurement to plot.
