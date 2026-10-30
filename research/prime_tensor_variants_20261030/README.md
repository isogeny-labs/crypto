# Packet variants and the prime-field ECDLP research boundary

Read the [four-page illustrated report](reports/report.pdf),
[original requirement mapping](requirements.md), and
[evidence/storage notes](receipts/INTEGRATION.md).
Repository-wide validation has a separate
[completed run and fixture-replay receipt](receipts/ROOT-VALIDATION.md);
the original nonzero full-run exit is preserved.

2026-10-30 UTC. **The requested essentially new sub-exponential ECDLP
algorithm has not been achieved.** This continuation develops a three-adic
variant of an existing tensor-contraction method and checks its central
operations independently. It retains unsuccessful alternatives and corrections.
There is no measured speedup, general implementation certificate or novelty
claim.

The existing prime-field result and its exact source pins were established in
the [previous study](../prime_one_shot_20261029/continuation/tensor-review/README.md).
Its claimed expected bound is
`2^{O(b/(log b)^(1/4))}`, with `b=ceil(log2 p)`, under its stated curve/order
assumptions. It is prior art. The variant here changes the internal packet
action and coefficient prime; it does not improve that exponent or introduce
a distinct ECDLP reduction.

## What this continuation establishes

- A ternary packet action with two components per coordinate, together with
  target-specific crossing modes. Review caught a necessary correction:
  each source tile can cross to two targets, requiring at most `18M/D`
  modes per weight instead of the initially proposed `9M/D`.
- An exact native packet/orbit/coverage unit and a 192-case local mode unit.
  The corrected factorization agrees throughout. The incorrect unsplit
  factorization fails 14 cases modulo 27, including a retained `9` versus `0`
  witness. [Protocol, source, records and reviews](packet-unit/README.md).
- A separate native one-level unit combining actual child sums, extension-ring
  interpolation and rational Hermite recovery. Its final checked scope is
  recorded in [the unit README](onelevel-unit/README.md).
- A written [derivation](ternary-derivation.md), independent
  [packet](reviews/ternary-packets.md), [precision](reviews/ternary-precision.md)
  and [global review](reviews/ternary-global.md), with a [proof map](proof-map.md) distinguishing checked
  Lean declarations from unformalized arguments.
- Twenty-eight unique Lean theorems now include the actual integer-to-digit
  bridge and bounded-slot coordinate, in a separately checked
  [versioned addition](lean-digits/README.md). The original 20-theorem package
  remains byte-for-byte unchanged.

These are development correctness units, deliberately forced at tiny
parameters. They are not admitted asymptotic-sized calls, scientific timing
experiments, or complete ECDLP executions. CPU contention cannot explain an
exact modular disagreement; the retained negative control was an algebraic
factorization defect. No runtime inference is made from the passing units.

## Other proposals checked, with the actual scope of each closure

| Proposal | Finding | Evidence |
| --- | --- | --- |
| Sharper mode valuations and a nonlinear Hermite taper | The existing mode enumeration still contains exponentially many candidate tuples at its present exponent scale. This is not a lower bound on nonzero coefficients or on arbitrary algorithms. | [Taper review](reviews/taper.md) |
| Matched-level two-axis deformation | Origin recovery has the desired precision. Simultaneous bit constraints lose density; a constructive moment rewrite recovers the old enumeration envelope. Changing packet geometry by level still needs a common-family/jet identity. | [Analysis](reviews/matched-level.md), [counterfamily](reviews/levelwise-binomial-counterexample.md) |
| Product deformations in several coordinates | A passive coordinate restores channels in the direct product construction. A proposed general code-occupancy barrier was too strong: repeated-slot counterexamples invalidate its premise. No general multidimensional impossibility follows. | [Correction](reviews/multivariate-code-correction.md), [route notes](route-decisions.md) |
| Compact scalar verification | The explicit window/chunk encoding has `O(b^(20/19))` Boolean edges and meets the cited degree cap, but that theorem then gives no `exp(o(b))` guarantee. Formal rank-one support is not a public factorization procedure. | [Scalar review](reviews/compact-scalar.md), [division-polynomial review](reviews/division-polynomials.md) |
| Few explicitly programmed high-degree predicates | The sharper degree-moment collision estimate does not remove indivisible-port/padding restrictions. Occupied-tile counting alone also leaves exponentially many terms. This concerns the unchanged proof template. | [Heavy-predicate review](reviews/heavy-predicates.md), [independent check](reviews/heavy-variance-independent.md) |
| Contract the Fourier gadget densely | Its literal graph forces an `Omega(b)` live-index cut in an ordinary dense binary contraction. This is representation-specific and does not exclude implicit contraction. | [Width review](reviews/fourier-width.md) |

The evaluated rotation identity certificate is `IDC1h90427cfbed72e56b`,
in [rotation.identity.json](rotation.identity.json); its
[native client and scope](identity/README.md) distinguish the v1 evaluation
field from both label and coefficient fields. The separate general Lean
rotation theorem is not replaced by that finite-field record.

## Reproduction and unresolved requirements

Run `bash check.sh /tmp/prime-ternary-check` from this directory. The helper
builds native C++ sources, regenerates fixed records and checks their frozen
hashes. Lean checks are separate and pinned in [lean/README.md](lean/README.md)
and [lean-digits/README.md](lean-digits/README.md).
The separate [nine-theorem binomial project](lean-binomial/README.md) checks
the counterfamily's signed parity, polynomial evaluation, origin and node
values; it does not formalize all chart coefficients or the full obstruction.
Raw-table hashes permit replay after automatic artifact migration; there are
no downloaded answers or external services in the native checks.
The shell metadata checks also require `jq`. Stored pointers are checked
against frozen pre-migration hashes; this does not claim a local S3 readback.

Still open: full formalization of the ternary contraction theorem, a general
recursive native implementation and proof-to-code audit, an essentially new
ECDLP mechanism, exhaustive historical six-repository review, and prospective
isolated scaling experiments on any implemented candidate. Earlier corpus
coverage and access limits remain as recorded; this continuation does not
retroactively make that audit exhaustive. Private ML/RL repository contents
are not copied into this public package.

## The one next bet selected from this round

Pursue [correlated packets with one common deformation](../../problems/correlated-packet-common-kernel.md).
The reason is specific: matched-level recovery already has the needed precision,
and finite-support moment costs can be controlled; the missing object is a
single kernel with compatible, sufficiently dense local factorizations. This
is a matrix/packing problem independent of a secret scalar. Its conditional
target would improve the contraction exponent from a square-root logarithmic
saving to a two-thirds power, if every stated obligation closes. None does so
automatically. A new counterfamily rules out a generic data-only shortcut.

The [exact scalar-splitting problem](../../problems/prime-scalar-exact-splitting.md)
remains an alternative. The ternary construction is a supported internal
adaptation, not the original breakthrough. These choices and targets are
research judgments; the requested new algorithm remains unachieved.
