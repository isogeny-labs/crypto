# Forty-two source pairs under an order-four/order-eight local policy

The frozen 1,722-pair source law contains 42 point pairs satisfying the coefficient-six relation on the fixed reduction over \(\mathbb F_{47}\). This packet applies the same admissible auxiliary policy to **all 42** pairs: search \([-3,3]^3\) for a nonsingular order-four bridge at 7 and \([-5,5]^3\) for an order-eight bridge at 11, taking the first passing triple at each prime and combining them modulo 77.

The 70,308 declared local attempts yield 22 pairs with both gates and 20 with no bridge inside the two search boxes. Each of the 22 first CRT representatives retains the two local relations, and none satisfies \(P+6Q=\mathcal O\) over \(\mathbb Q\). Thus the earlier pair-31 result extends to the complete reduced-compatible source set under this particular first-passing policy. Other congruence representatives and rational relations outside the boxes remain separate questions.

| Frozen stage | Exact result |
| --- | ---: |
| Source pairs | 1,722 |
| Fixed-reduction compatible pairs | 42 |
| Local coefficient attempts | 70,308 |
| Pairs with both local gates | 22 |
| Pairs without both gates in the declared boxes | 20 |
| Rational relations on first CRT representatives | 0 of 22 |

The [protocol](protocol.json) fixed the source digest, both local boxes, point-count gates, first-passing rule, CRT convention, and full attempt retention before execution (`e5ad563679a`). All attempts and pair outcomes are in numbered JSONL chunks under 48 KB each; their ordered digests, counts, and selected indices are in [result.json](result.json). The run completed under its 300-second cap with empty stderr.

The [independent arithmetic audit](audit.py) replays every attempt with quadratic-character point counts and double-and-add, verifies attempt order, per-pair and aggregate statuses, and checks the CRT congruences. Its [result](audit-result.json) is `PASS`. A separate [checked-Sage replay](sage_audit.py) constructs all 22 rational curves, checks their discriminants, exact group relations, and both finite-field reductions; its [result](sage-audit-result.json) is `PASS`, with zero rational relations. The accepted Sage runtime receipt and both empty audit stderr files are retained.

The pinned Lean [census module](formal/Census.lean) checks the exact finite counts 42, 22, and 0 from the saved outcome flags. Its generator binds those flags to the ordered outcome stream, and the independent audit verifies that binding. The actual [build output](formal/build-output.txt) reports no axioms for all three declarations. This is a **partial formalization** of the experiment: the elliptic-curve point counts, group-law results, and CRT map are independently replayed by the arithmetic and checked-Sage audits, but are not yet Lean theorems. A full formal theorem would require definitions and proofs for these concrete Weierstrass groups and their reductions.

The next experiment should test all-integer coefficient shifts for the 22 locally bridged pairs, using the full rational relation polynomial and retaining source-pair selection cost. A separate route is to vary the auxiliary policy beyond the first passing triples, since this sweep examined only one CRT representative per bridged pair.
