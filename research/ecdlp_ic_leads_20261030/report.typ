#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Applying the Hexagon leads to ECDLP index calculus
2026-10-30 · native integration and development validation

An opt-in archived-base oracle now enumerates m−1 prefix points and derives
the last point from the target sum. It retains multiset order, repeated points,
inverse-pair exclusion, subgroup identities and exact witness replay. This is
the existing generic final-slot technique applied to multiplicative-coset,
norm-one-trace and Dickson archives, not a new algebraic PDP algorithm.

#image("integration.svg", width: 100%)

*Interface.* The native factor-base CLI adds decompose and replay-decomposition.
An IC caller supplies only the public query R; its private random relation labels
remain outside the oracle. Returned indices can become a labelled row modulo
the subgroup prime. Cap exhaustion is incomplete, never a zero-yield proof.
Index construction, validation, group arithmetic and setup remain additional
costs; prefix/lookup counters do not establish an end-to-end speedup.

*Other leads.* Actual selector-graph width and cutsets can inform contractor
choice. Weighted local-cost profiles provide finite controls, with no inherited
subexponential implementation. Partial-relation elimination must retain odd-prime
coefficients, slot identities, raw provenance and the target coefficient.
Deduplicated union coverage and useful rank must replace raw component counts.
Algebraic PKM work still requires actual constrained PDP solvers: the existing
torus leading-form obstruction and certificate-degree cautions remain applicable.

*Sources.* The dated 22-paper Hexagon catalog; primary PKM sections 3.1–3.4;
prime_ic_program_20261029; pkm_dickson_torus_20261015. Source statements remain
reported. Full source/Lean audits and new performance comparisons are unperformed.

*Checks.* Factory release tests: 23 pass, one existing fixture writer ignored.
Strict Clippy and formatting pass. New witnesses agree with exhaustive search
on three prime recipes and a binary base at arities 2–4, with duplicate targets.
All three native CLI fixtures replay, as do six frozen archives and the old
evaluation receipt. The toy targets have known logs. Both traversals share curve
arithmetic; no independent arithmetic audit or unknown-target solve is claimed.

The README and leads.json record applicability and next controls. Existing
performance/frontier graphs receive no new data. Full ECDLP cost, useful-rank
campaigns, unknown-target recovery and improved algebraic complexity remain open.
