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

Theorem 21 closes one natural implicit-generator family unconditionally. An
arbitrary exact addition predicate based on separate summand/target labels
forces every label map to be injective. Prime-order elliptic subgroups also
admit no proper nontrivial exact compositional bucket quotient, even when the
label operation is not a group law;
algebraic-group maps to additive or multiplicative field groups are trivial;
and isogenies of degree below the subgroup order are injective on the subgroup.
One-sided-error filters, target-specific functions, and joint encodings remain
open.
`[pcc:unverified]`

Theorem 22 quantifies the remaining one-sided separate-label case.  If either
partial-sum side or the attainable target set is compressed into fewer labels,
collisions in its label fibres clone genuine triples into distinct false
accepted triples.  The exact lower bound is controlled by the fibre-size
second moment; over a uniform target batch it becomes a corresponding
expected verification cost.  The result still does not cover joint encodings,
target-dependent labels, or non-enumerating witness recovery.
`[pcc:unverified]`

Proposition 23 then audits a concrete target-dependent escape.  Hashing one
partial-sum side by \(x(Q)\) and scanning the other with the label \(x(T-P)\)
has only the elliptic sign ambiguity as a false positive.  Its search cost,
not verification, is decisive: the most favourable separate-query split has
complete relation exponent \((m-1)/(2m-3)>1/2\), namely `2/3`, `3/5`, and
`4/7` for `m = 3, 4, 5`.  This does not rule out a shared multi-target or
non-scanning construction. `[pcc:unverified]`

Theorem 24 proves the desired square-root direct sum for a new explicit class:
degree-explicit algebraic selector correspondences.  A nonconstant
degree-\(d\) leaf morphism can land in an \(N\)-point factor base at no more
than \(dN\) witness points.  Summing this fibre bound over a fixed catalogue
of charged projection-degree mass \(\Delta\) gives
\(\mathbb E Z\le BN\Delta/n\).  Producing \(\Theta(N)\) rows therefore forces
\(B\Delta=\Omega(n)\) and total cost \(\Omega(\sqrt n)\).
Target-adaptive branch selection, multivalued root recovery, and direct
non-materializing leaf output are allowed.  Compact shared DAGs that encode
many paths without paying their expanded degree mass remain outside the
theorem. `[pcc:unverified]`

Proposition 25 explains why Theorem 24's expanded-degree charge cannot simply
be imposed on compact DAGs: multiplication-by-`k` has degree `k^2` but an
`O(log k)` double-and-add circuit.  Theorem 26 then removes the degree charge
for single-valued global maps entirely.  Every rational map `E -> E` is a
translation of an endomorphism; on a prime-order subgroup its restriction is
constant or injective.  Consequently each compact selector branch succeeds
on at most `N` targets, so an explicit `r`-branch catalogue obeys
`E[Z] <= BNr/n` and again requires `Omega(sqrt(n))` total work.  Arbitrary
map degree, compact group-law DAGs, and shared straight-line computation are
allowed.  Succinct multivalued correspondences and implicitly exponential
branching remain open. `[pcc:unverified]`

Proposition 27 marks the exact finite-domain boundary.  On the reduced scheme
of subgroup points, the coordinate algebra is the full function algebra
`Map(G, F_p)`.  An arbitrary relation selector therefore has an explicit
`O(n)` lookup representation whose leaf functions need not be constant or
injective.  This costs far more than rho, but proves that elliptic morphism
rigidity alone cannot handle finite-domain formulas.  The remaining problem
is genuinely one of compressing and querying those selector tables or
recovering witnesses from succinct correspondences. `[pcc:unverified]`

Theorem 28 closes symbolic straight-line recovery from those correspondences.
If a query program stays inside the target function field and uses only field
operations and inversions, a generically correct recovered root is a rational
section of the correspondence.  Its leaf outputs are global rational maps,
so Theorem 26 applies without a degree charge.  Any remaining succinct
correspondence attack must use genuine algebraic root extraction,
finite-field-specific branching, or correctness restricted to a finite target
set. `[pcc:unverified]`

Proposition 29 shows why the last step is substantive.  For
`p = 3 mod 4`, the generic correspondence `y^2 = t` has no rational section,
but `y = t^((p+1)/4)` recovers a root on every quadratic-residue field target
with an `O(log p)` powering circuit.  This is not an elliptic decomposition
algorithm, but it proves that finite-field root extraction cannot be reduced
to generic rational sections by degree or circuit size alone.  A final
barrier must charge root extraction or bound how frequently its outputs land
in the elliptic factor base. `[pcc:unverified]`

Proposition 30 rules out even an individual-leaf hit bound for unrestricted
finite-domain circuits.  For `d | p-1`, the logarithmic-size power map
`t -> t^((p-1)/d)` sends every target into `{0} union mu_d`, a set of size
`d+1`; fixed branches can relabel those values by chosen elliptic factor-base
points.  This still does not produce a decomposition.  It proves that the
remaining barrier must use the joint `m`-leaf sum constraint or a genuine
preprocessing/query recovery argument, rather than bounding one leaf's
factor-base hit rate. `[pcc:unverified]`

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
fourteen tests passed, zero failed. The suite grid-checks Theorem 20's piecewise
optimization against direct minimization and verifies the exact `m=3,4,5`
fractions in the indexing-family audit plus the finite cyclic-group kernel of
Theorem 21, including exhaustive arbitrary label algebras through order seven
and exact separate-label predicates for small groups. It also checks all three
one-sided false-positive lower bounds of Theorem 22 on bounded cyclic-group
fixtures and the target-shift exponent arithmetic of Proposition 23.
It also replays the finite fibre-count kernel of Theorem 24.
The final check replays Theorem 26's constant-or-injective compact-map kernel.
It also constructs the finite-domain selector counterexample of Proposition
27 on a small cyclic group.
The final fixture checks Proposition 29's finite-field square-root selector.
It also checks Proposition 30's exact power-map image sizes.
`[pcc:unverified]`

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
