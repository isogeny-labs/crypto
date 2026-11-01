#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R43: actual larger source and section spaces
Observations report: three accepted native computations, one accepted Lean
workflow and two retained rejected attempts. No independent evidence verdict
or index-calculus performance comparison.

#image("round43.svg", width: 100%)

== Source-bound precision615 expansion
At the certified degree5point Q, construct all615coefficients of
u,v,b,c,lambda,nu. Every Newton stage
checks the five actual source equations and preserves old coefficients.
All eight units/inverses, five coefficient-descent conjugates and closure
pass; a perturbed top nu coefficient is rejected.

All610saved reference rows agree through precision154: six coordinates,
eight units, eight inverses,68sections,196quadratics,324cubics. Grouped and
direct monomial evaluation agree for204original/twist values through154.
The actual descended functions agree with both original orientations at615.
The old section ranks68/196/324are recovered in dimension3075.

== Larger complete product bases
#table(columns: (1fr, 1fr, 1fr, 2fr), inset: 5pt,
  [Space], [Input rank], [Output rank], [Products independently checked],
  [L4D0], [196], [452], [14746],
  [L6D0], [324], [708], [48090],
  [L12D0], [708], [1476], [246426],
  [L24D0], [1476], [3012], [1085466])

All attempted products and pivots reconstruct from selected original columns.
Each selected function has its parent pair and full615coefficients. Enumeration
stops upon reaching the prescribed dimension; it is not an all-pair census.

#pagebreak()
= Completeness bridge, proof scope and remaining work
The retained proper source has genus61and D0has degree128. Products satisfy
the indicated pole bounds. Evaluation at615Qhas degree3075, exceeding the
largest pole bound3072. Thus evaluation is injective under the retained
proper/pole bridge. Each independent native rank meets its Riemann–Roch
upper bound. This yields complete spaces under that bridge, without assuming
surjectivity of an untested product pool.

With D1=4D0these are the requested L(D1),L(3D1),L(6D1)dimensions452,1476,3012.
This removes a concrete setup mismatch for ordinary367/460divisors; it does
not construct or evaluate those complete fibers.

== Lean and retained errors
Three named mathematical theorems prove characteristic-two AS telescoping,
the ten-term solution under nilpotence615, and coefficient extraction at615.
The last proof is reused from R38. Audit also includes three generated
recursion equations: six declarations total, zero failing/trusted.
No native evaluation. Native arrays, matrix ranks, proper normalization and
RR completeness are not formalized by these generic lemmas.

jets43-01failed in Rust compilation because a series edit targeted Field
instead of Jet; no native trial ran. lean43-01failed an implicit characteristic
two cast tactic. Committed repairs used fresh attempts with unchanged caps.
No accepted producer reran. Native build/run caps:4GiBvirtual/1200seconds;
Lean4.34.1/mathlibd13f23b,one job,4096MiBheap,1200seconds compile/audit.

== Next actual constructions
Complete ordinary367/460fibers and exceptional multiplicities; certify
padding disjointness; construct complete small and large vanishing spaces
inside the new bases. Then extend full-generator division/class evaluation
and check actual factor images. Never invert a jet vanishing on padding.

Proper level77target binding, the chosen Koblitz-model isogeny, full quotient
saturation/polarization, node187 coupled valuations and theta descent remain
separate. There is no evaluated larger-n transfer, IC relation rank,
unknown-scalar recovery or speedup. Independent scientific/semantic review
remains pending; artifact byte verification is a separate check.

Sources: retained R27/R34/R35/R38/R40 and Khuri-Makdisi math/0409209v3
representation-B setup, plus Stacks01V4; links in LITERATURE.md. Full raw
formats, matrices, attempted product coordinates and lineage are documented
in FORMAT.md and the content-addressed artifact.
