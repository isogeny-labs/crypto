#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:10pt)
#set heading(numbering:none)
= Factor-base shape leads
Native exploratory search · 2026-10-18

The requested search pursued structures that preserve useful successful
decompositions and independent relations. It did not search for cheap
rejection alone. Native sweeper source b3ed8889 ran the frozen public-fixture
plans; a separate Rust field/group implementation then exhaustively audited
their two-point support. No scalar logarithms were recovered.

== Main observed lead
On *icv1-f2m13-t181-515ee569*, trace-zero geometric progression (GP)
bases improve supported-target coverage in this finite panel. Degree13 is
secondary evidence; the degree 31 primary results are described below.

#table(columns:(1.4fr,0.6fr,0.9fr,1fr,0.65fr),inset:5pt,
 [Constructor],[Cols],[Holdout /128],[Coverage /2002],[Train rank],
 [Coordinate],[16],[4],[144],[5],
 [Random seed 17],[12],[4],[78],[3],
 [Random seed 29],[17],[13],[152],[2],
 [Trace-zero GP seed 17],[15],[15],[226],[5],
 [Trace-zero GP seed 29],[15],[17],[214],[12],
 [Trace-zero GP seed 43],[21],[30],[482],[15],
)
The same 128 fresh draws contain 122 distinct targets. GP seeds 17/29 have 15
distinct successful holdout targets each; GP seed 43 has 30. Different inventory
sizes are explicit. These are observed counts and exact finite support,
not a significance test, a total-cost ranking, or a fastest result.
Successful observed solving degree remains 3; no regularity improvement
is established.

== The subgroup gate matters
On *icv1-f2m31-tm90707-c95f16f5*, three coordinate parameterizations
and the tested random base support *zero* nonidentity subgroup targets.
The tested trace-zero/Frobenius span supports 2 of 1,439,392. Every old
whole-curve positive control passed, but every random probe missed.
The proper-subfield degree 9 control likewise supports zero of 36 targets.
The new CLI requires a finite positive control annihilated by the actual
prime subgroup order; earlier evidence keeps its historical scope.

#image("decision-flow.svg",width:100%,height:49mm,fit:"contain")
#text(size:8pt)[Solid boxes show observed checks; dashed routes remain proposed.
Complete native pricing and certified regularity remain unknown.]

#pagebreak()
== Fresh confirmation and independent arithmetic
#image("confirmation.svg",width:100%,height:132mm,fit:"contain")

The sampled screen and fresh confirmation are separate, frozen rounds.
Constructor seeds 17/29 were retained; seed 43 is a new GP construction.
The independent implementation checks the registered polynomial basis,
curve equation, recorded generator order, every native truth verdict, and
every recorded point-witness sum. It audits 3,923 original trial records and
1006 witnesses on 60 sealed attempts (59 completed, one failed empty base).
The failed case remains null, not zero-yield completion.

The exact census enumerates signed point pairs and restricts their sums to
the prime subgroup. It is a costly diagnostic, not an available-for-free
large-field oracle. This is second-implementation checking on the same host,
not an independent-host certificate or an admitted benchmark.

Primary degree 31 is exploratory E0 evidence, with r = 1,439,393 and
cofactor 1,492; field degree is not subgroup bit length. The degree 7 fixture
uses a=1, while the degree 13 secondary uses a=0. The degree 9 control has a
proper intermediate subfield and cannot support a prime-degree conclusion.
The degree 31 Frobenius block differs from the irreducible blocks described
for degrees 53/83/131 in AGENTS.md. The degree 83 confidence gate and exact
challenge remain unperformed. Frobenius-generated prefixes are not
automatically Frobenius-stable spaces.

#pagebreak()
== Representation and rank tradeoff
#image("representation.svg",width:100%,height:150mm,fit:"contain")

Changing the basis preserves the exact signed point set and random-probe
outcomes, but does not lower aggregate successful F4 work here. All changed
bases fail the preregistered strict per-success component-improvement filter.
On the degree 7 fixture, reversed order raises training rank 10 to 11 at higher
F4 work. The current first-witness policy is therefore a separate lead;
more rank alone is not cheaper relation collection.

== Follow-up and evidence
The new draft card is XP-20261018-factor-base-shape-witness-leads:
cofactor-aware shapes and rank-aware witness selection. It remains
unrun, zero-funded and unleased, with no frozen executable protocol.
The older card from #1963 is unchanged.

Sources: PROTOCOL.md, FOLLOWUP_PROTOCOL.md, COVERAGE_PROTOCOL.md;
observations.json, exact-coverage.json; analysis/src; reports/run-20261018T0437Z
and reports/run-20261018T0443Z; validation.md and runs.tar.xz.
No accepted wall time, full-pipeline speedup or certified regularity is claimed.
