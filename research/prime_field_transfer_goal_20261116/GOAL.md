# Prime-field point transfer research goal

Started 2026-11-16. The thread goal is active. This file is a checkpoint,
not a completed reduction or a claim of novelty.

## Objective and acceptance criteria

Given an elliptic curve E/Fp, a public generator P of a substantial prime-order
subgroup of order r, and an arbitrary public Q in that subgroup, construct and
evaluate a nonzero scalar-preserving map into a demonstrably easier target.
The evaluator receives the point coordinates, never the unknown scalar.

| Requirement | Acceptance evidence |
| --- | --- |
| Arbitrary point input | Evaluator API has no scalar; fixed setup shared by all points |
| Scalar preserved and recoverable | Nonzero generator image, order r, homomorphism/injectivity proof |
| Efficient evaluator | Setup and per-point costs; no exhaustive subgroup or hidden DLP table |
| Easier destination | Actual target solver and cold end-to-end comparison |
| Nontrivial scale | Growing-order fixtures and explicit applicability conditions |
| Novelty | Primary-source comparison, narrowly stated differences |
| Reviewability | Native code, meaningful controls, formal obligations, raw receipts and PR |

The earlier order-seven example in merged PR 4308 remains a toy control and
does not satisfy these requirements. Completion is not inferred from a new
target, a curve lift, a conditional abstract isomorphism or this checkpoint.

## First round: target component group and compatible lifting

Competing routes:

1. Good-reduction global torsion bridge: retain as an open route; existing
   CT74–94 already isolate local lifting, selective recognition, place choices
   and target Frobenius restrictions. Do not repeat their tiny controls as a
   new algorithm.
2. Split multiplicative target: use a component quotient Z/rZ at 2. Test an
   explicit compressed integral-model construction and whether component
   labels factor through source reduction. A target quotient alone does not
   provide an arbitrary-source-point evaluator.
3. Local torsion/formal-log shortcut: the prime-to-p torsion lift preserves the
   scalar, but its formal logarithm vanishes. This is established prior art,
   not a new candidate or a universal impossibility claim.

The next discriminating check for route 2 is a kernel-compatibility control:
if rho is source reduction and kappa is the target component map, a map
Phi with kappa=Phi∘rho needs ker(rho) contained in ker(kappa). Search only
the fixed small fixture in PROTOCOL.md for a counterexample; keep the original
goal open if the proposed arbitrary-lift shortcut fails.

## Sources checked (KB unavailable)

No research-KB MCP is loaded. `scripts/research-kb status` exits 127 because
its executable is unavailable. Thus no KB source id or fresh-catalog claim is
made. Fallback sources read at main 0ead8ca49 include:

- `problems/prime-field-chained-transfer-no-advantage.md` and
  `research/chained_transfer_proof_20261112/PROOF.md`: norm/pullback and
  complementary-quotient restrictions; these do not close every transfer.
- `research/chained_transfer_next_20261115/NOTE.md`, `REDTEAM.md`: existing
  open cases and the necessity of complete costs and appropriate controls.
- `research/cross_characteristic_global_torsion_20261112/README.md` and
  CT83–85, CT93–94: finite precision, selective reconstruction, orientation
  and Frobenius restrictions. A dense field-degree bound does not by itself
  exclude compressed algorithms.
- [Silverman, Four Faces of Lifting (2007)](https://www.math.brown.edu/~jhs/Presentations/ECC4FacesOfLifts.pdf),
  pp. 6–14: local lifts and the distinction between preserving a relation and
  reaching an easy quotient. The slide's broad field-degree summary is not
  used as a single-point degree theorem.
- [Gaudry–Hess–Smart](https://www.kantmath.de/publications/papers/weil_descent.pdf),
  introduction and construction: same-characteristic extension-field descent
  and its kernel and genus conditions. It supplies no cross-prime bridge.
- [Cremona, Computing in Component Groups (2008)](https://johncremona.github.io/papers/component.pdf),
  sections 1–2: explicit component labels and their consistent orientation.
  The component map is classical, not a claimed new map.
- [Sage Tate-curve documentation](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_tate_curve.html)
  and [PARI elliptic-curve documentation](https://pari.math.u-bordeaux.fr/dochtml/html/Elliptic_curves.html):
  actual local uniformization interfaces and required input points.

Searches included cross-characteristic torsion transfer, negative lifting
results, GHS, split multiplicative reduction/Tamagawa component groups and
compressed coefficients. No exhaustive prior-art or novelty claim is made.
