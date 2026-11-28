# Exact single-target m=7 decomposition with an x-first affine leaf

- Date: 2026-11-26
- Curve: production Prime31 instance over `p = 2^31 - 1`
- Track: generic decomposition
- Implementation: Rust
- Claims: `PCC-20261126-cc71bb` and `PCC-20261126-3c0b9a`, both `unverified`

## Result

The final x-first affine contraction is a strict component-level improvement on
the frozen public target. It searches the same 971,635 terminal candidates,
performs the same 1,028,789 query additions, returns the same valid seven-point
witness, and reduces query multiplications from 13,029,908 to 4,613,665
(8,416,243 removed, 64.5917%) and squarings from 5,086,791 to 1,254,991
(3,831,800 removed, 75.3284%). It uses one additional batch inversion.
This result is producer evidence pending independent PCC signatures.
[pcc:unverified]

Relative to the already-improved affine-leaf arm, exact-x-first materialized y
only 35 times and removed another 971,600 multiplications (17.3958%). The
archived non-isolated replay has a 48.141 ms cold-total median, versus 57.803 ms
for affine-leaf and 69.584 ms for the original retained-3 arm. Those apparent
1.200702x and 1.445421x local ratios are diagnostics, not admitted isolated
timing claims. Exact operation vectors are primary. [pcc:unverified]

This is not a full ECDLP recovery, a rho comparison, or an asymptotic
breakthrough. It is an exact m=7 decomposition kernel on one public toy target.
No state is reused across targets, arms, or rounds. [pcc:unverified]

## What changed algorithmically

The baseline retains the canonical three-sum quotient, then evaluates every
target-centered four-leaf prefix in Jacobian coordinates and batch-normalizes
the output. The new arm keeps the same retained table but changes the streamed
side:

1. enumerate all 54,740 target-centered three-prefix complements;
2. batch-normalize that frontier once;
3. add the fourth leaf in affine coordinates, batching the 971,635 nonzero
   denominators in fixed 2,048-element blocks; and
4. compute the terminal x coordinate, probe the exact 16-bit prefix directory,
   and materialize y only after full 31-bit x equality.

Identity, inverse, and doubling cases are classified before the generic
denominators enter a batch. On this fixed target all 971,635 terminal branches
were generic. Only 35 candidates had an x coordinate in the retained set; all
35 paid the final y multiplication and full-coordinate orientation check. The
prefix is only a directory and never serves as a probabilistic fingerprint.

The design was frozen after the first four-arm result and before this arm was
implemented or executed. The x-first design was then frozen after the affine
result and before its own implementation. Both adaptive steps are disclosed in
`PROTOCOL_AFFINE_LEAF.md` and `PROTOCOL_DEFERRED_Y.md`; neither is represented
as a prediction made before an earlier run.

## Exact comparison

All retained-3 arms rebuild 27,370 quotient states with 26,248 exact x keys
inside every cold measurement. Setup is identical: 1 inversion, 368,098
multiplications, 144,160 squarings, 29,716 additions, and 68 doublings.

| Query metric | Jacobian centered | Affine leaf | Exact x first |
| --- | ---: | ---: | ---: |
| Candidates | 971,635 | 971,635 | 971,635 |
| Exact table probes | 971,635 | 971,635 | 971,635 |
| Key comparisons | 388,647 | 388,647 | 388,647 |
| Additions | 1,028,789 | 1,028,789 | 1,028,789 |
| Doublings | 0 | 0 | 0 |
| Inversions | 475 | 476 | 476 |
| Multiplications | 13,029,908 | 5,585,265 | 4,613,665 |
| Squarings | 5,086,791 | 1,254,991 | 1,254,991 |
| y materializations | 971,635 | 971,635 | 35 |
| Compact payload | 314,976 B | 314,976 B | 314,976 B |
| Prefix index | 262,148 B | 262,148 B | 262,148 B |

Equivalently, the original baseline uses 2.824199x as many query
multiplications and 4.053249x as many query squarings. The exact candidate set
and group-addition count did not shrink, so the gain is a coordinate-contraction
improvement, not hidden pruning. [pcc:unverified]

## Cold single-target panel

The retained table is charged to the only target in every row. The archive
contains nine cyclic cold rounds per arm.

| Arm | Median setup (ms) | Median query (ms) | Median total (ms) |
| --- | ---: | ---: | ---: |
| `retain4-hash-sum` | 94.966 | 5.691 | 99.674 |
| `retain4-hash-centered` | 85.778 | 3.538 | 90.533 |
| `retain4-prefix16-centered` | 79.689 | 5.247 | 85.357 |
| `retain3-prefix16-centered` | 4.562 | 64.319 | 69.584 |
| `retain3-prefix16-affine-leaf` | 3.718 | 53.766 | 57.803 |
| `retain3-prefix16-affine-x-first` | 3.302 | 44.440 | 48.141 |

The host had no CPU isolation, so these medians cannot establish a portable
timing speedup. They are retained in full rather than selecting a favorable
round. [pcc:unverified]

## Literature assessment

Petit--Kosters--Messeng express the factor-base constraint as a composition of
low-degree rational maps and solve the resulting Semaev system. Their p-1 and
auxiliary-isogeny constructions do not provide a p+1 elliptic-group
homomorphism or this seven-sum contraction.

Amadori--Pintore--Sala replace a fixed subspace base with one factor-base
dependency and report prime-field experiments for m in `{3,4,5}`. Their m=5
formulation splits the summation equation through an intermediate variable and
`S3`/`S4`; it does not give a measured m=7 extractor.

Wagner's generalized birthday method needs visible quotient bits at successive
merges. No useful small quotient is known for this prime-order elliptic
subgroup, so prefixing an x-coordinate is only table indexing and cannot be
used as Wagner pruning.

Primary sources:

- Petit--Kosters--Messeng, <https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf>
- Amadori--Pintore--Sala, <https://iris.unitn.it/retrieve/handle/11572/202653/186850/Amadori%2C%20Pintore%2C%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20%28rev%29.pdf>
- Wagner, <https://www.iacr.org/archive/crypto2002/24420288/24420288.pdf>

The staged affine leaf and exact-x-first refinement are therefore reported as
scoped implementation ideas, not as literature-wide novelty theorems or an
asymptotic algorithmic result.

## Independent replay and proof boundary

The separate verifier does not call the candidate Jacobian arithmetic, retained
table, prefix directory, or solver. It derives the factor base and target from
the public seed, replays all six witnesses, enumerates all 54,740 affine
triples and 971,635 affine quadruples, and verifies the exact key counts. The
archived verifier report records a passing full scan and result digest
`95350a6f7b88d1dd9e06129fe8e6497539a4da3cbe377e82f9107dda7ab4e729`.
[pcc:unverified]

There is intentionally no zero-knowledge proof here. The seed, witness, source
hashes, certificates, and full affine replay are transparent evidence. A
Schnorr-style proof could hide the known witness, but would not prove unbiased
seed selection, native execution, or timing. The PCC replay and recomputation
artifacts are unsigned, so claims `PCC-20261126-cc71bb` and
`PCC-20261126-3c0b9a` remain `unverified`.
[pcc:unverified]

## Attempts retained

The first release build exhausted local disk before producing any benchmark
row. Reproducible development artifacts were cleaned. A one-codegen-unit retry
was stopped before execution because it was nonstandard and impractically slow;
the normal release build then succeeded. Neither failed attempt expanded the
seed nor produced data. The first five- and six-arm runs were followed by
explicit complete archival replays; their nine-round panels are retained
without selecting favorable rows.

## Reproduce

From the repository root, at the source revision pinned in
`deferred-y-source-manifest.json`:

```sh
cargo run --release --example norm_one_torus_m7 -- \
  --manifest research/norm_one_torus_m7_single_target_20261126/public-precommit/deferred-y-manifest.json \
  --certificates /tmp/norm-one-torus-m7-certificates.json

cargo run --release --example norm_one_torus_m7_verify -- \
  --manifest research/norm_one_torus_m7_single_target_20261126/public-precommit/deferred-y-manifest.json \
  --certificates research/norm_one_torus_m7_single_target_20261126/public-precommit/deferred-y-certificates.json

sha256sum -c \
  research/norm_one_torus_m7_single_target_20261126/public-precommit/artifacts.sha256
```

Replayed wall times may differ. Candidate counts, operation vectors, witnesses,
artifact digests, and the result digest must not.

## Next goal for p+1

The next p+1-specific goal is an algebraic m=7 arm: substitute
the Dickson selector into an Amadori-style `S3`/`S4` tree, charge system
construction and solving to the same target, and require at least a fourfold
reduction in exact field-equivalent work against this generic kernel before
calling it a breakthrough. No cross-target table, learned ordering, or solver
state may be amortized into that comparison. [pcc:unverified]

Three concrete ideas should be tested in that rung:

1. eliminate the seventh leaf only after substituting the Dickson selector,
   so the last root problem is univariate rather than another group scan;
2. choose the `S3` tree by measured elimination width, not a balanced-tree
   heuristic, and retain both PKM-composed and Amadori dynamic-base controls;
3. use x-first rejection inside any remaining explicit boundary contraction,
   preserving the exact full-coordinate check developed here.

The pass condition is cold one-target work, including Gröbner/resultant setup,
below one quarter of the 4,613,665-multiplication generic query equivalent. A
miss, degree explosion, or solver timeout is retained as the decision result;
it cannot be hidden by amortizing a basis across targets. [pcc:unverified]
