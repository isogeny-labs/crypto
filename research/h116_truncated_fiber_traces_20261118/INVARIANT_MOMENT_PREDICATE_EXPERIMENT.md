# H116 invariant-moment predicate experiment

Protocol: `H116-INVARIANT-MOMENT-PREDICATE-001`

Source/launch gate commit: `13a88e33e6`

Disposition: **the frozen exact-predicate hypothesis is rejected on the two
excluded fixtures**. [pcc:unverified]

## Execution record

The source/launch checkpoint was pushed before the first affected execution.
The locked producer then generated 576 model-target records: 288 distinct
targets, each in the native and `lambda=2` isomorphic model. It reported:

```text
invariant moment producer: records=576 candidate_direct_mismatches=0 repeat_mismatches=0
```

The raw census contains the expected 36 targets for each of three generator
labels over `F_31`, and 60 targets for each of three labels over `F_47`, with
two model records per target. All jet/direct identities and repeated jet
evaluations agree. [pcc:unverified]

The scorer canonicalized each model pair to one target, recomputed all sixteen
discriminant-normalized channels, and found zero native/`lambda=2` invariant
mismatches. Its `records=288` JSON field therefore counts canonical targets;
the retained moment TSV contains the 576 model records required by the
protocol. [pcc:unverified]

## Frozen scores

| measurement | result |
| --- | ---: |
| model-target records | 576 |
| canonical targets | 288 |
| curve/generator rows | 6 |
| invariant channels | 16 |
| permutations per channel | 720 |
| frozen predicates | 11,520 |
| invariant mismatches | 0 |
| uniform exact predicates | 0 |
| exact predicates on `p=31` | 0 |
| exact predicates on `p=47` | 0 |
| unchanged cross-curve survivors | 0 |
| best target matches | 65/288 (0.225694444444) |
| best shuffled-control matches | 69/288 (0.239583333333) |
| exact shuffled-control predicates | 0 |

The per-channel best target counts, in order `IX0..IX7,IY0..IY7`, are:

```text
62, 60, 60, 65, 56, 57, 62, 65, 56, 59, 55, 54, 59, 62, 62, 54
```

Sixteen predicates tie for the target best, all in `IX3` or `IX7`; the fixed
shuffled control reaches a higher best score with one `IX5` predicate. This is
one preregistered shuffle, so the comparison is a control observation rather
than a calibrated p-value. Neither curve has even a curve-local exact
predicate, so the specified holdout diagnostic has no survivor to promote.
[pcc:unverified]

The synthetic positive scorer control passed before the run: the identity
permutation was the unique exact predicate when bin equaled digit, and a
single label mutation made every permutation inexact. A clean rebuild and
second scoring invocation reproduced the result JSON byte-for-byte. The
different replay binary hashes reflect a different output filename embedded
by `rustc`; source hashes and compiler identity are the frozen execution
gate. [pcc:unverified]

## Retained artifacts

```text
8d9791b149a5815c3536c8b7219e71806d6e3376286b7eacb414586d06d2135  INVARIANT_MOMENT_PREDICATE_MOMENTS.tsv
7e892aac8b67704ebb7c78f30ffc5e6c6ae8fafb5f960dc962c2da49dbd9f338  INVARIANT_MOMENT_PREDICATE_LABELS.tsv
6f9a1b3473223869c93bfdf7b7839bcee6bc416d902b025afd985f31b0f3a1b7  INVARIANT_MOMENT_PREDICATE_RESULT.json
```

## Interpretation and next boundary

This result closes only the specific single-channel, six-bin, global-
permutation decoder. It does not show that the invariant moment vector is
independent of the scalar, nor does it test relational or multi-channel
observables. Conversely, direct moment identities and coordinate invariance
do not provide a scalar decoder. No generic-prime-field ECDLP or
subexponential-runtime claim follows. [pcc:unverified]

A legitimate follow-up must be frozen before inspecting any new score. It
should compare a bounded relational observable against multiple shuffled-label
replicates and an information-preserving control, rather than adding
post-hoc bins or channel combinations to this run. [pcc:unverified]
