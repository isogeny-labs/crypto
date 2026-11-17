# Curve25519: matched isogeny, carry and rank instruments

2026-11-04. Development implementation; experiment comparisons are pending.
All observations in this report are producer assertions [pcc:unverified].

## What is implemented

1. **Isogeny support controls.** The native instrument constructs source,
   transported-identical, target-native, mixed and random factor bases with
   equal numbers of sign-orbit columns. A GMP fixture exercises the actual
   RFC Curve25519 generator, subgroup, rational 2-isogeny, dual and inverse
   transport. The finite atlas experiments themselves are small-prime
   diagnostics; no 255-bit relation experiment is implemented or reported.
2. **Matched relation encodings.** Two QF_NIA emitters describe the same
   signed, distinct-column, three-summand group query: direct modular
   multiplication and radix-bounded schoolbook multiplication/reduction
   carries. Full affine addition includes infinity, doubling and inverse
   branches. Membership is an explicit point table. This is not compact
   membership or a new summation-polynomial construction.
3. **Training-only rank selection.** A generic decomposition enumerator
   replays hits by the group law and computes coefficient rank over ell.
   Candidate selection uses training rank per pair probe; disjoint holdout
   queries are reported separately. It does not optimize or measure full
   pipeline cost yet. The emitted rank is not target-recovery success.

![Prospective comparison and evidence boundaries](workflow.pdf)

Editable diagram: [workflow.dot](workflow.dot); vector: [workflow.svg](workflow.svg).
Dashed routes are pending. This is a workflow diagram, not a computed
isogeny graph, and contains no measured speed values.

The RFC model identity already in `docs/curves/registry.json` is
`icv1-fp255-tm221938542218978828286815502327069187962-5eb6d947`.
No destination registry identity is asserted for the development quotient.

## Source review and novelty correction

Read before choosing this implementation:

- [RFC 7748 section 4.1](https://www.rfc-editor.org/rfc/rfc7748#section-4.1):
  exact Curve25519 parameters and Montgomery/Edwards equivalence.
- [Galbraith, chapter 15, sections 15.8–15.9](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch15.pdf):
  summation-polynomial factor bases, difficult prime-field decomposition and
  extension-field descent assumptions. The source was read in the preceding
  review; this is not an exhaustive current-literature search.
- crypto-autoresearcher lead atlas:
  `LEAD-ICP-isogeny-transported-relation-yield`,
  `LEAD-STR-class-representative-ic-selection`,
  `LEAD-ICP-digit-presentation-solving-degree`. Respectively these already
  track transported supports, held-out representative selection and carry
  presentations. See `knowledge/leads/ecdlp/{ic-prime,structure}/` there.
- [Curve traits source](../../src/cryptanalysis/curve_traits/mod.rs) and
  [walk implementation](../../src/cryptanalysis/curve_traits/walk.rs):
  splitting traits are distinct from construction of an executable map.
  The independent development fixture does not extend the registry or ecdb.
- [Round-7 Edwards systems](../ic_tree_gluing_20261103/round7_edwards/README.md):
  existing E1/E1Q/TW/TQ encodings overlap the proposed representative search;
  do not count this study as a new version of that result.
- Existing [tower implementation gap](../../problems/curve25519-family-algebraic-ic-tower.md):
  a finite point-table query does not implement the missing scalable
  isogeny-tower algebraic membership oracle.

The original chat proposals were not audited for novelty. Repository
retrieval found direct precedents for all three. The deliverable here is a
matched development instrument and a prospective combination, not a claim
that the mechanisms have not been tried [pcc:unverified].

## Development checks and retained failures

Build with GCC/C++20 and GMP:

```
make test
bash test-products.sh /tmp/curve25519-atlas-carry-rank /path/to/z3 /tmp/fresh-products
bash test-encodings.sh /tmp/curve25519-atlas-carry-rank /path/to/z3 /tmp/fresh-relations
```

`make test` covers the RFC fixture, exhaustive small-curve homomorphism/dual
checks, inverse subgroup transport, carry witnesses, dependent/independent
rank rows, identical-support collection, and malformed-model rejection.
See [development checks](checks/README.md) for exact commands, raw output,
versions, scope and failures. Passing a producer's functional checks is not
independent verification [pcc:unverified].

External relation-encoding tests remain unresolved: initial direct-modular
UNSAT fixtures hit their 30-second development test limit. Both the p=1019
initial case and the p=101 follow-up are retained, as is the attempt after
removing redundant curve equations. No timeout is called UNSAT, and no
unexecuted arm is called a result. Product-only tests isolate the carry
component; they do not replace the full relation checks.

## Execution and remaining obligations

The requested work is **partial**: all three instruments exist, with the
checks scoped above; matched EKS comparisons and independent replay have not
run. No performance or scaling result is claimed.

[PROTOCOL.md](PROTOCOL.md) freezes the proposed panel, controls, classification,
budgets and stop rules. Its execution state is review_required. The checkpoint
upload and authenticated custom-image run route are unresolved; see the
additive [launch-gate record](../../problems/arity-completion-launch-gates.md).
The ordinary GitHub workflow builds ecbench, not this instrument. No paid
experiment was launched, and no job continues in the background.

Next technical work: integrate small-prime SAT-model replay into the frozen
solver runner, satisfy checkpoint/image/route admission, run the unchanged
panel, then obtain independent verification. Keep the actual 255-bit solver
integration and full-cost rank-selection comparison open.

No new verified graph or numeric leaderboard finding is asserted. The
canonical curve/isogeny registry and `docs/index-calculus-scoreboard.html`
are left unchanged; this study adds a separately labeled prospective diagram.
No Lean theorem is claimed: this is executable finite development checking,
not a new universal identity proof [pcc:unverified].
