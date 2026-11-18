#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= R49: source charts and normalization attempts
These are exact-computation observations and execution receipts, not an independent scientific verdict or a performance comparison.
#image("round49.svg", width: 100%)

== Recorded counters
#text("normalization control cases: 2")
#linebreak()
#text("source preparation cases: 1")
#linebreak()
#text("registered generic units: 8")
#linebreak()
#text("generic source degree: 32")
#linebreak()
#text("generic source prime: 1")
#linebreak()
#text("integral lambda plane models: 0")
#linebreak()
#text("primitive presentations: 0")
#linebreak()
#text("compact source preparation cases: 1")
#linebreak()
#text("normalization outputs computed: 0")
#linebreak()
#text("verified affine normalizations: 0")
#linebreak()
#text("audited conditional declarations: 2")
#linebreak()

== Method and scope
The smooth and cusp controls exercise positive-characteristic normalP and full norTest. The source preparation reconstructs the twisted source from the original Tate/QR equations, checks both generic ideal inclusions, eight registered units, and degree 32 over F2(u). A compact chart retains b,c, descends all five equations to F2, and agrees with the accepted plane generic ideal at degree 32. These are input and function-field checks; neither preparation is a normalization result.

Both affine normalization attempts timed out in minimal-prime decomposition before exporting a selected component or reaching normalP. Their frozen programs require all minimal primes, a unique component mapping to the degree-32 source field, and normalP without an assumed-prime shortcut. A computed output would still need the separately frozen verifier: source identities, corruption control, prime equality, full norTest, and normalized generic degree. Unfinished decomposition is an incomplete calculation. It is not a geometric impossibility or a mathematical null result.

The conditional Lean lemmas give relative degree one and bijectivity for a field tower with equal positive degrees. Actual fraction fields, compatible maps, native ideals and generic-degree values are hypotheses to instantiate, not formalized CAS data. The first compile failed because a narrow import omitted the standard vector-space freeness instance; the repaired source imports it. A subsequent launch failed at a checkpoint upload HTTP 503 before compiler start; no compile or audit receipt exists for that launch. All failures remain indexed.

Independent exact CAS stages overlapped on a shared host. A live observation found each of the two active presentation processes receiving about half a CPU. Wall-cap outcomes are censored under that co-tenancy; no isolated runtime comparison or feasibility bound is inferred. Subsequent normalization is admitted after these processes terminate. Native runs keep the 4 GiB virtual-memory and 1200 s per-invocation limits. Lean uses 4.34.1, mathlib d13f23b723b8a846827a245b89c10fc7d3f11612, one thread, 4096 MiB and 1200 s per compile/audit. No new EKS job, Diem restart, freeze restart, Conductor claim or merge occurred. Independent scientific replay and semantic review remain pending.

Proper projective charts/gluing, complete pole divisors and prescribed RR spaces, the chosen level-77 factor map and nonzero order-11 action, full integral four-map recovery, arbitrary reverse principal divisors, quotient saturation/polarization, theta/node187 and genuine larger-n transfer remain open. Existing n=3/n=5 controls remain retained. No faster-IC assessment is inferred.

A partial primitive shape line is a candidate only. It is not counted as a field presentation unless the entire producer passes its original-equation and reverse-identity checks. Missing lambda completion data stays missing; an empty log does not identify the step consuming the cap.

Generic source receipt: GENERIC49_COMPONENT index=1 degree=32; GENERIC49_RESULT components=1 original_ideal_prime=1 affine_contraction_checked=false normalization_performed=false

Primitive presentation receipt: PRIMITIVE49_SHAPE fixed_element_primitive=1 univariate_generators=1 minimal_degree=32

Lambda construction receipt: No completed result line; wall cap reached.
