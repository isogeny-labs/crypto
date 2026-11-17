#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Reusable adaptive source and incremental extension
2026-11-05 · T-1659

Prepared public source state is now retained across recoveries. A target-specific cursor keeps the final projective grid and the last two homogeneous seed pairs, so an exceptional character can extend the prefix. The adaptive coefficients and sign predicates are unchanged. [pcc:unverified]

#image("pipeline.svg", width: 100%)

== State invariant
At prefix endpoint T, lane j represents [lambda_j]T. Add P to T and the cached affine [lambda_j]P to each lane, then batch-normalize the new point and lanes together. For seed pairs c(T)=N/D and c(T-P)=A/B, the next pair is [delta S N^2 B : D^2 A], with delta=x(P)-x(T) and S=c(P)^2. Anchors O, P, -P, 2P take precedence. A one-seed prefix can still require direct seeding. [pcc:unverified]

Cursor fields are private and lifetime-bound to preparation. Advance consumes the previous cursor; an error returns no replacement. An explicit remaining-step allowance is checked before advancing geometry. Identity steps skip normalization, while invalid lane configurations are rejected.

== Recorded checks
The optimized overflow-checked suite reports89 passing tests. All320 split-window extensions match uninterrupted signs, seed pairs, points and methods. A test-only counter checks actual calls to phase(). Another44 extensions on the F5/r7 and F7/r13 fixtures traverse repeated identity crossings:20 anchors and24 recurrences, with no additional direct phases. [pcc:unverified]

One prepared F509/r479 source serves all eight saved adaptive targets. Four require an extra sign; each uses recurrence,23 lane updates and one batch conversion of24 points. The split-window checks cover these same exceptional offsets without direct phase calls. A separate checker reexecutes the frozen pre-cursor source, accepts the eight outputs and query decisions, and rejects a corrupted recovered scalar. Its metadata checks do not prove producer execution. [pcc:unverified]

== Cost and limits
Seed recurrence adds no field inversion; nonidentity geometry extension uses the existing single-inverse batch routine. Initial grid construction, direct initial phases, value/division and polynomial work remain. The prefix API still returns the complete prefix; only cursor retention is compact. No whole-source timing or inversion-free claim is made. [pcc:unverified]

Twenty-three source/data hashes bind the run; eleven compiler warnings are retained. The original4563-case cohort is preserved. No cloud job or timing panel ran. Next bind complete-cost instrumentation and investigate shorter adaptive prefixes with representable collapsed targets.

== Evidence and prior work
Claims PCC-20261105-99d02c and PCC-20261105-cc8f6a await signed coverage. See DERIVATION.md, source-lock.json, origin-lock.json, receipts and raw logs. Source formulas follow the preserved shared-seed and pair-seed derivations. Jacobian conventions were checked against the #link("https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html")[Explicit-Formulas Database]. No novelty claim.
