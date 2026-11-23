#set page(paper: "a4", margin: 17mm)
#set text(size: 9.5pt)
#set heading(numbering: none)
= Prime n83: independent proof-bridge review
Technical packet · T-995 · 30 October 2026

Three independent reports support the degree-17 first-fall result for the stated prime-field selector/Semaev chain: 27 selectors per block, m = 3–9, characteristic above 27, and nonzero normalization weights. The graded reviewer verifies absence of non-Koszul highest-form syzygies through 16 and an explicit witness at 17. The filtered reviewer proves bounded representation descent and separates the saved remainder from V16 while deriving it in V17. The blind reviewer independently derives the onset.

#image("dependencies.svg", width: 100%)
#text(size: 8pt)[Dependency diagram, not a performance measurement. Sources: the three verbatim TASK-N83 reports, machine/verification.json and claim-status.json. Blue: reviewed mathematical joints; gray: executed checks; yellow: new construction and next check.]

== New construction: the centered target is covered
The blind derivation pairs centered selectors y_j = z_j − w_j/2. An alternating projection annihilates every bounded-product row through degree 16, but is nonzero on the new terminal remainder for every target. Its degree is 16 generically and 15 at t = sum(w_j)/2. At that target the surviving coefficient is (w_26/2)^2. The derivation is preserved in TASK-N83-BLIND/report.md; next, implement the centered projection in Rust and independently test the exceptional-target case.

== Exact checks completed
The independent Rust verifier checked one saved case, original-prime-m3-natural: exact primality certificate, 17 supplied file hashes, six protocol source hashes, full source reconstruction, polynomial codecs and factors, circuit, coefficient polynomial, full remainder and pairing. It recomputed degree 16 and the nonzero pairing 4087116827087420414291699. The existing producer panel has 42 records; this independent replay covers one.

Fresh Lean 4.34.1 compilation and declaration/axiom audit passed all eight arithmetic lemmas. Six controls correctly accepted the known counterexample and rejected a false bound, missing declaration, sorry, custom axiom and native bypass. The S3-restored source reproduced the Lean axiom log. The full polynomial formalization still requires the string decomposition, Koszul transport, filtered descent and centered projection; no independent kernel rechecker was available.

== Degree interpretation and retained scope
The reviewed template gives first strict fall 17 and LFD at least 17. No numerical LFD upper bound is supplied. The blind argument gives highest-form quotient dimension ceil((m−2)/2), so there is no first zero Hilbert-component cutoff for this m3–9 ideal. Castelnuovo–Mumford regularity and solving degree were not computed. These candidate x-domain equations are distinct from the expanded point-lookup ideal. The full fixed-base Koblitz/binary/prime m2–9 study is retained.

== Independence, deviations and custody
The precommitted independence plan passed all three attestations; the checker validates declared boundaries, not operating-system isolation. A separately committed supplement repaired the filtered snapshot's missing raw inputs. The initial handoff omitted the native-language policy: the Python attempt and failure are retained, and the accepted replay uses the independently implemented Rust verifier. Two projection roles are documented separately: the old syzygy-coordinate map kills U, while the functional retains U.

Producer revision: c64f02c176787dbf40d455f95b75bd2dd5bfe5bf. All reports, raw builds, controls and failures are retained in Git/S3 according to FORMAT.json and the custody records. Producer records remain immutable. This packet records a technical review without an official ledger transition.
