#set page(paper: "a4", margin: 15mm)
#set text(size: 9.2pt)
= Triple orbit constraints across the coefficient boundary
T-1809 • 2026-11-07 • PCC-20261107-21bac6 • unsigned producer evidence

Triple cyclic correlations now refine the power image using signs already collected for an orbit. A reciprocal-coefficient compiler avoids the zero recurrence denominator at characteristic r. The adapter preserves the existing pair constraints, retained-sign fiber stage and final public-point check.

#image("flow.svg", width: 90%)

== Feature and coefficient reconstruction
Let m divide r−1, d=(r−1)/m be even, ζ have exact order m and h=(r−1)/2. For distinct nonzero lags u,v, use q(X)=(X−1)(X−ζ^u)(X−ζ^v). The triple correlation is χ(b) times m times the stride-m coefficient projection of q^h, evaluated at (k+c)^m/b^m. The radius factor χ(b) is essential. The projected degree3d/2 reduces to at most d modulo V(V^d−1), including the zero image.

A3×3 polynomial-matrix recurrence computes stride coefficients only through r−1. For reciprocal roots, X³q(1/X)=q0 q_inv(X), so coefficient n equals q0^h times reciprocal coefficient3h−n. Every required higher index reflects below r. Two bounded prefixes therefore supply the complete projected feature without characteristic lifting or division by zero; their overlap is checked.

== Recorded native checks
Validation-02 passes four selected methods,157 imported filtered.22 compiler outputs match independently expanded schoolbook cubic powers.3,150 direct orbit values and1,797 complete image sets agree;648 image sets are strictly refined beyond all pair correlations.306 controls reject omission of the radius-character factor.

Three larger cases at r65537 and widths128,256,512 compare both compiler prefixes to scalar recurrences. They reconstruct448 high stride coefficients. These larger references share the recurrence identity; small dense references are independent of it.

Of624 ordered controls with distinct reciprocal images and identical product/all-pair aggregates,360 are distinguished by lag1,3 triples and264 remain equal. The remaining collisions are retained. No claim of complete image separation is made.

All63 public inputs recover: the preceding46 cases plus17 fixed known-answer points on E/F701, y²=x³+2x+8, P=(2,106), r683,m11. The earlier cases include the frozen20 on F509/r479 and all13 small F7/r13 targets with both m3 and m6.38 finish through retained-sign replay;778 source calls are recorded as path accounting. This is not the full4,563-case cohort or a timing comparison.

== Costs, domain and failure record
Two fixed-size matrix compilers preserve the safe per-feature bound O(M(m+d) log(m+d)+log r), with larger constants than the pair feature. Stored coefficients are O(d); conservative workspace is O(m log(m+1)+d log(d+1)). Each selected feature is charged. The public path uses one pair and at most one triple. Each orbit still costs m separate source evaluations; source setup, validation, shifts, retained history and final verification remain charged.

The triple feature rejects odd d; pair-only recovery remains available. The original source prime-field restrictions remain unchanged. General polynomial inverses use extended Euclid; the explicit batch inverses use exponentiation, following the preceding accounting correction.

Validation-01 stopped in public fixture setup because offsets13–16 were invalid in F13. The API correctly rejected them. Exact failed source snapshots and logs remain. Validation-02 changes only the test schedule to canonical offsets; production arithmetic and domain guards are unchanged.33 source/input hashes and generation hashes match. No cloud work or timing panel ran.

== Formal boundary and next construction
Lean4.34.1 proves that an index at or above r reflects below r when2N=3(r−1). Its only axioms are standard propext and Quot.sound. The polynomial reversal identity, projection, recurrence interpretation and source correctness remain written proofs. The report is unsigned producer evidence.

Sources: local pair/fiber code and lifted-coefficient notes; Bostan–Gaudry–Schost2007, DOI10.1137/S0097539704443793, coefficient application/main algorithm. KB unavailable; a historical REPORT path is missing in the sparse checkout. Established recurrence acceleration is reused. Next: find a smaller set of orbit invariants that breaks remaining equalities without a large collection of higher-order feature compilers.
