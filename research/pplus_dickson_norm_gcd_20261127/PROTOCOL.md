# Frozen protocol: target-specialized Dickson norm-gcd query

Date frozen: 2026-11-27 UTC. Branch:
`codex/pplus-dickson-norm-gcd-20261127`. Draft PR: #5275.
Standing: deterministic structural and counted-work experiment,
`[pcc:unverified]`.

This protocol is frozen before candidate source, production target
classification, depth-1--6 query output, or operation measurements exist.

## Question and scope boundary

For the fixed signed Dickson factor bases from the sealed oriented-pair study,
can a target-specialized quadratic norm compute the complete two-leaf predicate

```text
R in A_t + A_t
```

with exact signed recovery, exact misses, and subquadratic polynomial work?
Does that executable primitive beat a strong batched linear leaf scan and admit
a concrete four-leaf composition whose complete cold one-target projection is
at most 1,600,000 field-equivalent operations?

The first predicate is a **two-leaf primitive**. It is not itself the archived
m=7 four-leaf oracle. Promotion requires a separately specified composition
that avoids rebuilding or scanning the essentially quadratic `A_t+A_t`
support. Low polynomial degree, a gcd identity, or a fast planted hit without
complete misses and recovery does not pass.

No cross-target amortization, generic factor-base replacement, hidden scalar,
or warm state from an unrelated DLP target is permitted. A later batch may
share work only across the frozen residual cohort belonging to this one public
target, and only after the gates below pass.

## Literature and prior-evidence boundary

The repository research-KB MCP is not exposed in this runtime, and
`scripts/research-kb status` reports that no executable is available. The
bounded fallback consisted of the primary papers and committed records below;
no exhaustive novelty claim is made.

- Semaev, *Summation polynomials and the discrete logarithm problem on
  elliptic curves* (2004), Theorem 1 and Section 3,
  <https://eprint.iacr.org/2004/031.pdf>. The zero criterion is over the
  algebraic closure and therefore does not replace rational signed recovery.
- Petit--Kosters--Messeng, *Algebraic approaches for the Elliptic Curve
  Discrete Logarithm Problem over prime fields* (PKC 2016), Sections 3--5,
  <https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf>, local
  review SHA-256
  `a53030a6cde720c354aebfb65b1eb01e07766ce2d26e543d77f2516c0a49ef5b`.
  Their low-degree selector composition is direct prior art. Their cost keeps
  the relation solve as the open term `T(E,m,L)`, and their reported
  prime-field experiments solve two-summand systems rather than an m=7
  extractor.
- Amadori--Pintore--Sala, *On the discrete logarithm problem for prime-field
  elliptic curves* (FFA 2018), Sections 3--4,
  <https://doi.org/10.1016/j.ffa.2018.01.009>, author-manuscript review
  SHA-256
  `67442c30c2f14ab7891b6285f9b6d3c77cf5b1a6d2e838ec25b815a280029ba1`.
  Their dynamic factor base seeks one dependency and executes `m=3,4,5`.
  The displayed `m=7` crossover assumes `T(E,m,V) approximately equals
  T'(E,m,V)`; the paper explicitly says this is unproved. It supplies neither
  an executed m=7 algorithm nor this query.
- Bostan--Flajolet--Salvy--Schost, *Fast computation of special resultants*,
  introduction and Section 4,
  <https://algo.inria.fr/flajolet/Publications/BoFlSaSc03.pdf>, local review
  SHA-256
  `e0a977ad0ea0c2eca6f5f3468cf6c564983a624c6674430a06a30f5ad4bf3865`.
  Their quotient-algebra and diamond-product algorithms price the retained
  output degree; they are not an empty-inclusive sublinear zero-query theorem.

Committed controls are:

- `research/pplus_oriented_pair_transducer_20261127/REPORT.md` and its sealed
  `evidence/candidate.txt`;
- `problems/prime31-exact-pair-spectrum-query.md`;
- `research/prime31_summation_compression_20261122/iteration9/
  PAIR_SPECTRUM_RESULTANT.md`;
- `research/prime31_summation_compression_20261122/iteration10/
  S3_PENCIL_CONTRACTION.md`; and
- `research/prime31_summation_compression_20261122/iteration10/
  quotient_followup/DICKSON_PAIR_NORM_REVIEW.md`.

The last record proves the leaf-to-pair norm identity and gives an `F_7`
counterexample to automatic normalized Dickson-tower inheritance by the pair
spectrum. This experiment may exploit the original Dickson selector; it may
not assume that the resulting pair spectrum is another Dickson fibre.

## Fixed public source and sealed truth

Use registered curve `icv1-fp31-tm3989-ae9db972`:

```text
p = 2147483647
E: y^2 = x^3 + 2147483644 x + 21
r = 2147487637, cofactor 1
G = (1391443431, 177979044)
```

For each depth `t=1..=6`, use

```text
F_t(X) = D_(2^t)(X,1),
D_1(X,1)=X,
D_(2n)(X,1)=D_n(X,1)^2-2.
```

Enumerate its full zero fibre with the same backwards square-root convention
as the sealed study: at each step the smaller square root is branch zero and
its negative is branch one. Retain an abscissa only when the curve right-hand
side is a nonzero square; retain both point signs. Infinity is not a leaf and
repetition is allowed.

The independently reconstructed unordered signed two-leaf support, including
multiplicity, must match these sealed rows before any query result is accepted:

| depth | signed leaves | support | sealed SHA-256 |
|---:|---:|---:|:---|
| 1 | 2 | 3 | `14fe07ba12f8aed983d023adbb9e5c9570e8ad3dcb8618e174b6f47d2c224cc1` |
| 2 | 4 | 9 | `be147b1a4f2593925cc5f4228495eef0c81319914e6bdfa38b2514ccaed60427` |
| 3 | 8 | 33 | `b584bde8931ae3db613a74455de12620b022e73f4b3f91668d6151b1ea14ac1c` |
| 4 | 16 | 129 | `57205f062368d4877c3bc980ccf22c2958efaa9149992e38ccbbd6360ab5a4c0` |
| 5 | 30 | 451 | `42135e2a95d7b28d5902cfff2b96d92b7c88efac17d9693843f6944c29eb3132` |
| 6 | 68 | 2,313 | `ff67c668a68bcc407531add6028b55f706fc9a027991f1f40a1af2dcd5b0adcb` |

There is no random seed. Query order, square-root branches, and canonical
witness ordering are deterministic.

## Candidate construction: quotient before norm

Let a finite target be `R=(u,v)`, let `P=(x,y)`, and put

```text
f(x) = x^3 + a*x + b
d(x) = (u-x)^2
C(x) = v^2 + f(x) - (u+x)*d(x)
D    = 2*v.
```

Away from `x=u`,

```text
x(R-P) = (C(x) + D*y) / d(x).
```

Work from the start in the split quotient
`Q_t=Fp[x]/(F_t)`, retaining a pair `(A,B)` for `A+yB` and reducing every
polynomial product modulo `F_t`. Initialize

```text
A_0=C, B_0=D, delta_0=d.
```

For `j=0..t-1`, use the homogenized Dickson doubling recurrence

```text
A_(j+1) = A_j^2 + f*B_j^2 - 2*delta_j^2
B_(j+1) = 2*A_j*B_j
delta_(j+1) = delta_j^2
```

in `Q_t`. Then `A_t+yB_t` is
`d(x)^(2^t) F_t(x(R-P))` on the regular chart. Form

```text
N_R = A_t^2 - f*B_t^2 mod F_t.
```

Compute the exact split gcd with the known squarefree roots of `F_t`:
evaluate `N_R` through a balanced subproduct/remainder tree, retain precisely
the zero leaves, and multiply their known linear factors. Karatsuba
multiplication and reversed-polynomial/Newton division may be used, but every
base-field coefficient operation remains counted. A generic factorization or
an unpriced root oracle is forbidden.

For a regular gcd root with `B_t(x)!=0`, recover the only compatible
orientation as `y=-A_t(x)/B_t(x)`. If `A_t(x)=B_t(x)=0`, test both rational
signs. Every candidate then computes `Q=R-P`, requires `Q` to be an actual
signed leaf, and replays `P+Q=R`. Full-fibre roots that do not lift to the
curve, or whose translated root does not lift into the selected base, are
reported and rejected rather than silently counted as relations.

## Exceptional charts

- `R=O`: return the canonical inverse leaf pair if the base is nonempty;
  this chart does not invoke an affine target coordinate.
- `x=u`: denominator clearing makes `P=-R` an automatic algebraic zero and
  can introduce a spurious gcd root. Exclude it from the regular formula and
  directly test every actual leaf over `x=u`, including `P=R` (whose
  complement is `O`) and `P=-R` (whose complement is `2R`).
- `B_t(x)=0`: a norm zero then requires `A_t(x)=0`; test both actual signs.
- Curve two-torsion, zero denominators, nonlifting full roots, a constant norm,
  and an all-zero norm are explicit outcomes. The registered odd-order curve
  should exclude nonzero rational two-torsion, but the Rust unit controls must
  exercise the branch logic rather than rely only on that fact.

Any unresolved chart returns `INDETERMINATE`, never `EMPTY`.

## Fixed queries and maintained baseline

At every depth, in this order:

1. `planted_pair`: the first lexicographic unordered signed leaf pair whose
   sum is finite and nonzero;
2. `g1` through `g8`: `[k]G` for `k=1..=8`;
3. `first_leaf`: the first signed leaf itself, forcing `u` into the full
   Dickson root set and exercising denominator-zero rejection/recovery; and
4. `infinity`.

The candidate may not replace this panel after seeing its labels. At least one
finite miss at depth six is required. If no production query reaches the
`A=B=0` chart, a separately fixed toy-field unit control must do so.

Maintain a direct exact baseline on the same query. It scans distinct leaf
abscissae, batch-inverts all nonzero `u-x` denominators with one Montgomery
batch inversion, tests both signs, uses a canonical point set, handles `x=u`
directly, and replays its witness. Comparing against a per-leaf inversion loop
alone is not an admissible speedup claim.

## Public proof and independent replay

All inputs are public, so this study does not claim a zero-knowledge proof.
The candidate emits, for every query:

- the dense coefficient vectors of `A_t`, `B_t`, `N_R`, and the split gcd;
- `N_R(x)` for every full Dickson root in the frozen order;
- every algebraic zero-root index and every rejected/accepted chart label;
- the canonical signed witness or an explicit miss; and
- exact source-setup, online polynomial, recovery, and direct-baseline
  counters.

A separately written Rust verifier reconstructs the curve, roots, leaves, and
complete pair truth without importing candidate code. It evaluates the
homogenized recurrence **pointwise at every root**, checks all emitted
coefficient evaluations, rebuilds the split gcd, verifies every nonzero value
in a miss certificate, compares every query with direct enumeration, and
replays every witness. Thus a miss is a finite public certificate over the
complete known split selector, not an inference from timing or a hash.

The verifier also checks all six sealed support hashes. SHA-256 binds source
and evidence custody, but hashes are not substituted for the mathematical
replay above.

## Counted work and promotion rules

Record source setup and each online query separately:

- base-field additions, subtractions, multiplications, squarings, and
  inversions;
- polynomial multiply/square calls, Karatsuba nodes, modular reductions,
  Newton steps, remainder-tree nodes, peak input/output degrees, and peak
  coefficient slots;
- full roots, selected abscissae/leaves, algebraic gcd roots, nonlifting roots,
  accepted witnesses, denominator-zero roots, and `A=B=0` roots;
- direct-baseline batch-inversion work, comparisons, and replay; and
- elapsed time and peak RSS as diagnostics only.

Use all three frozen field-equivalent panels:

```text
panel 1: square=0.8, inverse=50
panel 2: square=1.0, inverse=100
panel 3: square=1.0, inverse=200
```

For each query, additions, subtractions, and multiplications cost one; squares
and inversions use the panel prices. Setup is charged once. Certificate output
and verifier work are reported separately and are never called free.

Promotion requires **all** of the following:

1. candidate and independent verifier accept every depth, query, coefficient
   evaluation, split-gcd root, hit, miss, exceptional chart, and witness;
2. the maximum finite-query polynomial work has both depth transitions
   `4->5` and `5->6` strictly below the corresponding full-root quadratic
   ratio, and its depth-3--6 log-log least-squares slope is below `2`;
3. at depth six the norm-gcd query is at least `2x` cheaper than the maintained
   batched direct scan in every field-equivalent panel; and
4. a written four-leaf composition, including setup, recovery, exact misses,
   and certificate traffic, projects the full `27,370`-residual cold target to
   at most `1,600,000` field-equivalent operations in every panel.

Failure of correctness stops immediately. Passing the slope but failing the
direct baseline or cold projection records a correct primitive-only null and
stops before same-target batching or m=7 integration. No post-hoc target,
metric, panel, or threshold replacement is allowed.

## Execution custody

Development is limited to unit tests and depths at most two. Before the sole
depth-1--6 production run, commit and push candidate, independent verifier,
analyzer, runner, this protocol, and an active SHA-256 source manifest. The
runner must require a clean named branch whose local head exactly equals the
same branch on `origin`; pushed Git is the checkpoint transport because the
owner has stated that S3 artifact storage is unavailable.

The production runner builds locked release binaries, refuses an existing
evidence directory, checks the active source manifest before and after build,
runs candidate once, verifier once, analyzer once, and hashes every evidence
file. A pre-candidate wrapper failure may be amended prospectively and retained
exactly; scientific reruns require an explicit amendment and may not silently
overwrite evidence.
