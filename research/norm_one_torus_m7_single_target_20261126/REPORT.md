# Exact single-target m=7 decomposition with an affine leaf

- Date: 2026-11-26
- Curve: production Prime31 instance over `p = 2^31 - 1`
- Track: generic decomposition
- Implementation: Rust
- Claim: `PCC-20261126-cc71bb`, standing `unverified`

## Result

The staged affine-leaf contraction is a strict component-level improvement on
the frozen public target. It searches the same 971,635 terminal candidates,
performs the same 1,028,789 query additions, returns the same valid seven-point
witness, and reduces query multiplications by 7,444,643 (57.1350%) and
squarings by 3,831,800 (75.3284%). It uses one additional batch inversion.
This result is producer evidence pending independent PCC signatures.
[pcc:unverified]

The archived non-isolated replay has a 59.017 ms cold-total median for the new
arm and 71.372 ms for the prior retained-3 arm, an apparent 1.209346x local
ratio. That wall-time number is a diagnostic, not an admitted isolated timing
claim. The exact operation-vector reduction is the primary result.
[pcc:unverified]

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
4. probe the same exact 16-bit prefix directory using full coordinates.

Identity, inverse, and doubling cases are classified before the generic
denominators enter a batch. On this fixed target all 971,635 terminal branches
were generic. The prefix is only an exact directory: it never serves as a
probabilistic fingerprint.

The design was frozen after the first four-arm result and before this arm was
implemented or executed. The earlier result is disclosed in
`PROTOCOL_AFFINE_LEAF.md`; this is a paired follow-up, not a prediction made
before the original run.

## Exact comparison

Both retained-3 arms rebuild 27,370 quotient states with 26,248 exact x keys
inside every cold measurement. Setup is identical: 1 inversion, 368,098
multiplications, 144,160 squarings, 29,716 additions, and 68 doublings.

| Query metric | Jacobian centered | Affine leaf | Change |
| --- | ---: | ---: | ---: |
| Candidates | 971,635 | 971,635 | 0 |
| Exact table probes | 971,635 | 971,635 | 0 |
| Key comparisons | 388,647 | 388,647 | 0 |
| Additions | 1,028,789 | 1,028,789 | 0 |
| Doublings | 0 | 0 | 0 |
| Inversions | 475 | 476 | +1 |
| Multiplications | 13,029,908 | 5,585,265 | -7,444,643 (57.1350%) |
| Squarings | 5,086,791 | 1,254,991 | -3,831,800 (75.3284%) |
| Compact payload | 314,976 B | 314,976 B | 0 |
| Prefix index | 262,148 B | 262,148 B | 0 |

Equivalently, the baseline uses 2.332908x as many query multiplications and
4.053249x as many query squarings. The exact candidate set and group-addition
count did not shrink, so the gain is a coordinate-contraction improvement, not
hidden pruning. [pcc:unverified]

## Cold single-target panel

The retained table is charged to the only target in every row. The archive
contains nine cyclic cold rounds per arm.

| Arm | Median setup (ms) | Median query (ms) | Median total (ms) |
| --- | ---: | ---: | ---: |
| `retain4-hash-sum` | 122.426 | 5.825 | 129.607 |
| `retain4-hash-centered` | 96.895 | 3.947 | 100.724 |
| `retain4-prefix16-centered` | 79.314 | 5.750 | 84.744 |
| `retain3-prefix16-centered` | 4.972 | 65.721 | 71.372 |
| `retain3-prefix16-affine-leaf` | 3.839 | 54.271 | 59.017 |

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

The staged affine leaf is therefore reported as a scoped implementation idea,
not as a literature-wide novelty theorem or an asymptotic algorithmic result.

## Independent replay and proof boundary

The separate verifier does not call the candidate Jacobian arithmetic, retained
table, prefix directory, or solver. It derives the factor base and target from
the public seed, replays all five witnesses, enumerates all 54,740 affine
triples and 971,635 affine quadruples, and verifies the exact key counts. The
archived verifier report records a passing full scan and result digest
`1a73b0dd24a4cac77d9fe4aefa61e9980da622f1337a464a5ce61447c6c1f2ce`.
[pcc:unverified]

There is intentionally no zero-knowledge proof here. The seed, witness, source
hashes, certificates, and full affine replay are transparent evidence. A
Schnorr-style proof could hide the known witness, but would not prove unbiased
seed selection, native execution, or timing. The PCC replay and recomputation
artifacts are unsigned, so claim `PCC-20261126-cc71bb` remains `unverified`.
[pcc:unverified]

## Attempts retained

The first release build exhausted local disk before producing any benchmark
row. Reproducible development artifacts were cleaned. A one-codegen-unit retry
was stopped before execution because it was nonstandard and impractically slow;
the normal release build then succeeded. Neither failed attempt expanded the
seed nor produced data. The nine-round archival replay is the retained raw
panel.

## Reproduce

From the repository root, at the source revision pinned in
`affine-leaf-source-manifest.json`:

```sh
cargo run --release --example norm_one_torus_m7 -- \
  --manifest research/norm_one_torus_m7_single_target_20261126/public-precommit/affine-leaf-manifest.json \
  --certificates /tmp/norm-one-torus-m7-certificates.json

cargo run --release --example norm_one_torus_m7_verify -- \
  --manifest research/norm_one_torus_m7_single_target_20261126/public-precommit/affine-leaf-manifest.json \
  --certificates research/norm_one_torus_m7_single_target_20261126/public-precommit/certificates.json

sha256sum -c \
  research/norm_one_torus_m7_single_target_20261126/public-precommit/artifacts.sha256
```

Replayed wall times may differ. Candidate counts, operation vectors, witnesses,
artifact digests, and the result digest must not.

## Next goal for p+1

The next cold single-target rung should defer affine y-coordinate construction
until an exact x key matches. The x coordinate needs the batched denominator,
but the final y multiplication is unnecessary on almost every miss. The frozen
acceptance target should preserve all 971,635 candidates and exceptional-case
coverage while removing about one multiplication per terminal candidate. This
is the nearest low-risk continuation of the measured contraction.

The more consequential p+1-specific goal is an algebraic m=7 arm: substitute
the Dickson selector into an Amadori-style `S3`/`S4` tree, charge system
construction and solving to the same target, and require at least a fourfold
reduction in exact field-equivalent work against this generic kernel before
calling it a breakthrough. No cross-target table, learned ordering, or solver
state may be amortized into that comparison. [pcc:unverified]
