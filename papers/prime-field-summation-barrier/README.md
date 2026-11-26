# Prime-Field Summation-Polynomial Index Calculus

> **Status (2026-11-26): imported research draft under audit in PR #5222.**
> Anonymous, not peer reviewed, not submitted. Every new mathematical claim is
> `[pcc:unverified]` until the proof map and independent review are complete.

## What the draft says

The paper gives an exact relation-collection accounting model and proves a [pcc:unverified]
barrier for two explicitly named algorithm classes:

- explicit candidate/collision search, including target batching; and
- faithful quotient-materializing Macaulay, resultant, multiplication-matrix,
  or normal-form elimination.

Within those classes, fixed-arity relation collection has optimized exponent
`m/(2m-1)`, which is `3/5`, `4/7`, and `5/9` for `m = 3, 4, 5`.  The paper also
audits common interval, rational-reconstruction, subgroup/coset, bounded-fibre,
split-tree, and multipoint-evaluation proposals against the same accounting.

## What it does not say

It does not prove a lower bound for unrestricted arithmetic/rational circuits, [pcc:unverified]
for all Semaev-based algorithms, or for ECDLP.  Low-degree circuit liftings can
avoid the syntactic degree argument, so their cost must be measured by a
separate invariant.  The target-adaptive non-materializing direct-sum statement
in [`SUCCESSOR_PROGRAM.md`](SUCCESSOR_PROGRAM.md) is open.

The source workspace included Python diagnostics.  They are not imported as
repository evidence.  Any retained finite check must be replaced by a bounded
native Rust implementation with exact fixtures and replay instructions.

The successor programme now includes an exact direct-sum theorem for
static-provenance product trees with fully target-adaptive query branching.
It obtains the same `m/(2m-1)` exponent without treating the tuple-sum map as
random.  Its explicit limitation is that every recoverable witness is a
pre-target materialized leaf; compact coordinate circuits that generate new
target-dependent subtrees remain outside the theorem. `[pcc:unverified]`

Proposition 19 identifies that remaining boundary with group `k`SUM-Indexing.
It audits the strongest 2026 indexing upper-bound family in a deliberately
attacker-favourable model and finds that the complete relation-collection
exponent still stays strictly above one-half.  The proposition also explains
why an unconditional extension is difficult: strong adaptive lower bounds for
ordinary integer 3SUM-Indexing are themselves open. `[pcc:unverified]`

Theorem 20 gives the exact piecewise decision line for future constructions.
If preprocessing costs `N^s` and amortized target-dependent batch work is
`N^t` per target, the balanced sparse branch (`s-t >= 1`) crosses rho exactly
when `s+t < m-1`; the
factor-base boundary branch (`s-t < 1`) requires `t < m/2-1`.  Under the
conventional `N^2` sparse linear-algebra model, replace `s` by `max(s,2)` in
that case split. `[pcc:unverified]`

## Source and claim crosswalk

- Semaev's original prime-field bounded-solution proposal: IACR ePrint
  2004/031.
- Existing repository implementation boundary: merged PR #4829 and
  `research/prime_field_summation_polynomials_20261119/`.
- Existing prime-field cost and solving-degree boundary:
  `research/prime_ic_program_20261029/`,
  `research/solving_degree_tower_frontier_20261124/`, and
  `research/degree_slope_two_block_20261124/`.
- Existing broad synthesis and negative-result boundaries:
  `papers/ecdlp-index-calculus-treatise/`,
  `papers/ic-negative-results/`, and
  `papers/rootfinding-decomposition/`.

See [`PROOF_MAP.md`](PROOF_MAP.md) for the exact verification status of each
claim family and [`LITERATURE.md`](LITERATURE.md) for the data-structure
boundary.

## Building

```sh
make
make check
```

The paper has an inline bibliography, so no BibTeX step is required.

The bounded native counting checks are standalone and dependency-free:

```sh
rustc --edition 2021 --test examples/finite_count_check.rs -o /tmp/prime-field-count-check
/tmp/prime-field-count-check
```

They verify finite instances of the exact uniform-target identity, the
multiset-count upper bound, and the displayed fixed-arity exponent arithmetic.
They do not verify an asymptotic lower bound or an algebraic solver model.

Native replay on 2026-11-26 used the `rust:1.83-alpine` container image at
digest `sha256:0ac946ed7597a9f053a1be2ce38c09aa88b3d7079a91ea491493615294b1f699`:
four tests passed, zero failed. The suite grid-checks Theorem 20's piecewise
optimization against direct minimization and verifies the exact `m=3,4,5`
fractions in the indexing-family audit. `[pcc:unverified]`

## Owed before review-ready status

- [ ] Independent proof review of the entropy/length and batching arguments.
- [ ] Resolve the theorem-to-repository crosswalk and remove duplicate claims.
- [x] Add native exact sanity checks for the finite counting identities that
      materially support the exposition.
- [ ] Decide which algebraic lemmas are feasible and useful to formalize in
      Lean; keep unformalized claims clearly labeled.
- [ ] Pass `make check`, scoped prose/PCC/research gates, and native checks.
- [ ] Record reviewer sign-off before moving the PR out of draft.
- [ ] Authors, affiliations, and email supplied by a human before submission.
