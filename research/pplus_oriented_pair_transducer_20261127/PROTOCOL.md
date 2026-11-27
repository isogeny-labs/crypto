# Frozen protocol: oriented quadratic-norm pair transducer

Date frozen: 2026-11-27 UTC. Branch:
`codex/pplus-oriented-pair-transducer-20261127`. Draft PR: #5267.

Standing: deterministic structural and counted-work preflight,
`[pcc:unverified]`. This protocol is frozen before candidate source, target
classification, or depth-census output exists.

## Question

Does the quadratic Dickson source admit an **exact oriented pair-support norm
transducer** whose semantic state grows subquadratically through depths 1--6
and whose complete cold single-target projection can replace the four-leaf
side of the archived m=7 split below 1.6 million field-equivalent operations?

The experiment tests one concrete bounded-state mechanism. It is not another
factor-base comparison, and a compact polynomial identity or low ordinary
rank without an executable query, recovery, and miss path does not pass.

## Literature and prior-evidence boundary

Petit--Kosters--Messeng (PKC 2016, Sections 3.1--3.4) replace a high-degree
factor-base selector by a composition of low-degree maps. Their partial cost
formula leaves the relation-system solve as `T(E,m,L)`, explicitly calls its
complexity open, and their reported prime-field systems decompose two points.

Amadori--Pintore--Sala (2018, Sections 3--4) make the factor base dynamic and
seek one dependency rather than a full relation matrix. Their executed
prime-field experiments cover `m` in `{3,4,5}`; the `m=5` system is split as
`S3` and `S4`. Their displayed `m=7` crossover is conditional on the unproved
approximation that their system solve costs the same as PKM's. It is not an
m=7 algorithm or measurement.

Semaev supplies the summation-polynomial zero criterion and recursive
resultants. Bostan--Flajolet--Salvy--Schost supply fast composed and diamond
products, but their result is a polynomial-construction bound whose output
degree is retained; it is not an empty-inclusive sublinear pair query.

Repository-pinned controls are:

- `problems/prime31-exact-pair-spectrum-query.md`;
- the exact identity `R_M J_M = kappa C_M^2` and F7 failure of automatic
  normalized `D2` inheritance in
  `iteration10/quotient_followup/DICKSON_PAIR_NORM_REVIEW.md`;
- the no-single-cover torus-shift obstruction in
  `iteration10/torus_rotation_followup/NO_SINGLE_COVER_FFT.md`;
- the exact squarefree pair-resultant baseline in
  `iteration9/PAIR_SPECTRUM_RESULTANT.md`; and
- the oriented Dickson-S3 signed-recovery work in
  `research/dickson_s3_quadratic_20261122/oriented_accounting/`.

The research-KB wrapper returned no executable for status and four targeted
searches. Authenticated ecdb is also unavailable. The registered curve source
and primary papers are the bounded fallback; no novelty or literature-
exhaustiveness claim is made.

## Fixed public source

Use registered curve `icv1-fp31-t36817-76528ee1`:

```text
p = 2147483647
E: y^2 = x^3 + 847514846 x + 2087444724
r = 2147446831, cofactor 1
G = (625037715, 2100627251)
```

For every depth `t` in `1..=6`, enumerate the full zero fibre of
`D_(2^t)(X,1)` by the fixed backwards quadratic tower

```text
z_t = 0,
z_j^2 = z_(j+1) + 2,  j=t-1,...,0.
```

At every square root, the smaller residue is branch zero and its negative is
branch one. Abort unless there are exactly `2^t` distinct abscissae and every
tower equation replays. Retain an abscissa only when the curve right-hand side
is a nonzero square; retain both point signs. Call the resulting signed set
`A_t`. Infinity is not a leaf. Repetition is allowed in decompositions.

No randomness, hidden seed, learned order, warm basis, or state from another
target is used. The full depth ladder and target rules below are fixed now.

## Exact semantic transducer

The Dickson root paths form a binary tree. For a node `v`, let `A(v)` be the
signed rational leaves below it. For every unordered node pair `v <= w` at
the same level, define the oriented support state

```text
Sigma(v,w) = multiset { P + Q : P in A(v), Q in A(w) }.
```

When `v=w`, unordered signed-leaf pairs with repetition are used once. When
`v<w`, every cross pair is used once. Each support entry stores its exact
affine point, multiplicity, and a canonical two-leaf recovery backpointer.
Infinity, inverse pairs, doublings, collisions, and repeated leaves remain
explicit.

If `v` and `w` have quadratic children, the parent state is the multiset union
of the three same-parent or four cross-parent child-pair states. This is the
zero-set semantics of multiplying the conjugate factors in a quadratic norm:
the zero set of the product is the union of the child zero sets. The candidate
therefore transports exact norm zeros and witnesses without expanding a dense
resultant. It is not allowed to replace a state by an x-only set or discard a
collision.

Semantic states are hash-consed only after comparing their complete sorted
`(point,multiplicity)` vectors. Node-local recovery backpointers remain
available even when the semantic vector is shared. The root state is the
exact oriented unordered pair spectrum `U_t=A_t+A_t`.

For a finite public target `R`, the four-leaf query scans the complete root
support and tests

```text
S in U_t  and  R-S in U_t.
```

It retains the lexicographically first four-leaf witness after the full scan.
Every returned witness is replayed by the original group law. A complete scan
with no complement is an exact miss. This is an executable pair-spectrum
query, not a rank surrogate.

## Fixed hit and blind target panel

At every depth:

1. The planted hit is the first lexicographic four-leaf multiset whose sum is
   finite and nonzero. Its construction is outside the candidate interval,
   but its full query and recovery are inside.
2. Query the eight fixed public targets `[k]G` for `k=1,...,8`. Their hit/miss
   labels are not assumed in advance. Every observed hit needs a recovered
   witness; every observed miss needs the complete exact scan.
3. Query infinity separately as a chart control. It is reported but excluded
   from the finite-target growth projection.

The candidate may not replace this panel after observing a classification.
At least one exact finite miss is required before any promotion; absence of a
miss is an unresolved correctness ladder, not permission to infer one.

## State rank and circuit census

For every tree level, form the support-incidence matrix whose rows are the
actual node-pair states and whose columns are the oriented group points seen at
that level. An entry is one exactly when the point occurs, regardless of its
positive multiplicity. Compute its exact rank over `GF(2)` with deterministic
bitset Gaussian elimination.

This rank is a diagnostic of linear state sharing only. It is not called a
Boolean rank, a circuit lower bound, or a proof that another representation is
impossible. Multiplicity vectors, semantic entries, and the executable query
remain the primary objects.

Record, per depth and level:

- full roots, lifting abscissae, and signed leaves;
- node-pair rows, nonempty rows, distinct support columns, and exact GF(2)
  incidence rank;
- actual circuit nodes, hash-consed semantic states, union edges, semantic
  entries, collision merges, and root-support size;
- peak live support entries and deterministic rank word-XORs.

The three frozen growth metrics are the leaf-level incidence rank, total
hash-consed semantic entries, and root-support size. Against signed leaf count
`L_t=|A_t|`, each must have both depth transitions 4->5 and 5->6 strictly
below the corresponding quadratic ratio, and the least-squares log-log slope
over depths 3--6 must be below 2. A zero or missing row fails the fit rather
than being dropped. Passing these diagnostics is necessary, not sufficient.

## Counted operations and cold projection

The Rust candidate counts exact base-field additions/subtractions,
multiplications, squarings, and inversions in root construction, curve lifts,
pair construction, target complement scans, and replay. It also counts group
additions/doublings, map comparisons/inserts, support merges, query probes,
and rank word-XORs. Wall time and allocator bytes are diagnostics only.

Use the six sensitivity rows already frozen by the parent m=7 study:

```text
square price:       0.8, 1.0, 1.0
inversion price:     50, 100, 200
multiply-add price:   1 or 2 where applicable
```

As in the parent panel, field additions and group-operation labels remain
visible but receive no invented multiplication price. The candidate has no F4
multiply-add counter. Its field equivalent is therefore
`mul + square_price*square + inversion_price*inverse`; all omitted work is
also reported and cannot support a favorable claim.

The m=7 projection uses the archived retained-three quotient size `27,370`.
The transducer setup is charged once to the one public target; state may be
reused only across residual queries generated inside that same target. The
complete four-leaf side is projected as `27,370` exact finite miss queries,
using the observed depth-six complete-miss operation vector. Also report the
optimistic floor `27,370 * root_support_size` support probes before any field
arithmetic. The retained-three setup from the archived control is then added.

Cross-target amortization, a warm norm circuit, target batching across DLP
instances, learned state, and division by a target count are forbidden.

## Promotion and stop rule

Continue to an m=7 integration only if **all** of the following hold:

- the candidate and independent verifier agree at every depth on the fibre,
  pair support, target classifications, recovered witnesses, and misses;
- every witness replays and every miss is complete;
- all three frozen state/circuit metrics pass the subquadratic growth rule;
- the measured depth-six transducer is the executable mechanism used in the
  projection, with no unimplemented rank-to-query step;
- the complete projected cold total is below 1.6 million field-equivalent
  operations under every sensitivity row; and
- its optimistic support-probe floor is also below 1.6 million.

Failure of any condition stops before m=7 integration and is retained as a
scoped null result. In particular, a quadratic semantic table, a linear scan
of a quadratic root support, or a low GF(2) rank with no faster exact query is
not promoted. This does not prove that every implicit resultant, separator
coordinate, or pair-spectrum data structure is impossible. [pcc:unverified]

## Independent replay and source custody

The verifier is separately coded in Rust and may not import the candidate
transducer, rank, state interning, or point-addition modules. It reconstructs
the tower and curve lifts, builds the exact signed pair support by a separate
flat enumeration, classifies every fixed target, and replays every disclosed
witness. It compares a stable digest of each complete support and result row.

Before the census, commit and push candidate source plus a manifest of source
and protocol SHA-256 hashes. The run emits deterministic text/JSON records,
source hashes, candidate output, verifier output, and analysis. No randomized
seed policy applies because there is no random choice.

The evidence is transparent replay evidence, not a zero-knowledge proof or a
host attestation. A ZK proof of one public witness would not establish state
rank, complete misses, unbiased target rules, operation counts, or source
execution. No PCC claim is minted by this preflight.
