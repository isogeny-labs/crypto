# Bounded native polynomial-block processing

The native stage consumes hash-verified root chunks, prepares one monic
polynomial block at a time, and streams six reciprocal-power sums into a
fixed-size accumulator. It never builds the global baby polynomial.
For the 4,096-root fixture, all 192 values (32 source-point contexts × six
powers) agree exactly with direct root-by-root inversion. Splitting into
two 2,048-root blocks, in either order, gives the same result. PARI separately
verified all 32 known-scalar contexts, the generator's q-order and its
non-membership in the ell-kernel. See [checks.json](checks.json) and
[contexts-check.json](contexts-check.json).

A second batch uses the actual first 32 giant abscissae
`x([(4*i+2)*B]K)`, i=0..31, with exact B=427154957034 for the unchanged
80-bit degree. PARI independently verified those contexts too. Their 192
sums also match direct inversion and the split-block result: 384 compared
values across the two batches. This uses only the 4,096-root baby prefix,
not the full B-root window. See `giant-contexts-check.json` and the retained
giant direct/streamed outputs.

This is a native development correctness test on the stored baby-root
fixture, not a complete target-size isogeny, point image or timing experiment.
The previous S3 root object is identified by SHA-256 in `checks.json`; its
hash-matched materialization came from the previously restored source/input
snapshot. S3 transport is handled by the repository artifact tool, separately
from the arithmetic executable. No fresh direct S3 GET by this executor is
claimed; the trusted storage workflow performs and records full-byte readback.

## Native commands and format

From the study directory, build with `sh native/build.sh` (set
`NATIVE_PREFIX` when the library prefix differs), then:

```text
native/build/poly_blocks build ROOT_CHUNK PRODUCT_BLOCK
native/build/poly_blocks eval POINT_CONTEXTS PRODUCT_BLOCK...
native/build/poly_blocks oracle POINT_CONTEXTS ROOT_CHUNK
```

The field is fixed to the frozen 202-bit prime in `parameters.json`.
Roots and point x-coordinates use the previous unsigned big-endian 32-byte
encoding. A product block has 8-byte ASCII magic `G80POLY1`, a 4-byte
big-endian degree, the 32-byte field modulus, then degree+1 ascending
coefficients, each 32 bytes. It must be monic, canonical and exact-length.
One block holds at most 65,536 roots; a context file holds at most 64 points.

`product-000000.bin` is the monic product for the stored 4,096-root fixture,
131,148 bytes. It is selected for immutable S3 upload and full-byte readback;
the artifact pointer commit establishes completion, not its creation-time
manifest. To hydrate after migration, from the repository root:

Upload/full-byte readback completed in migration commit `80c8c992d`,
artifact-autofix run `41258219926`. The committed artifact pointer matches
the locally validated product digest in `checks.json`; `storage-receipt.json`
records that outcome separately from the creation-time manifest.

```sh
python3 tools/artifacts/artifacts.py pull research/prime_conductor_gap80_20261031/poly-blocks
```

Verify the adjacent checksum before use. The original root chunk has its own
hash and ordinal manifest; prepared-block hashes do not replace coverage
metadata. A future controller must reject missing/overlapping intervals.
The low-level evaluator sums the supplied polynomials with multiplicity;
it does not certify global coverage or deduplicate filenames.

## What is implemented

For each supplied block, FLINT constructs/evaluates the block polynomial and
six derivatives at the bounded context vector. The existing constructor's
Taylor/logarithmic-derivative recurrence yields the six reciprocal-power
sums; those are added across blocks. Only one block, derivative evaluations
and the small accumulator are retained. The direct oracle uses scalar field
inversion for every root/context and does not share the polynomial path.

Nine rejection controls cover truncated blocks, magic, degree cap, wrong
field, nonmonicity, extra bytes, root collisions, noncanonical contexts and
context-count limits. They are expected failures, retained in `checks.json`.
The 32 contexts are x([s]G), s=1..32, on the same source curve; these are not
transported isogeny images. Point-context and original trace-helper checks
also passed independently.

## Remaining integration and costs

This is a bounded block-product/multipoint stage, not a globally fast
external-memory product/remainder tree. Repeated passes over many blocks
and point batches may require excessive total work and S3 I/O. No total
runtime or memory measurement was taken, and no end-to-end feasibility or
speedup follows from this fixture. Existing FLINT polynomial work remains
bounded per block rather than transparently swapping a global polynomial.

Next: integrate the prepared blocks with actual giant-point batches and
the codomain moment formulas over complete coverage (the first giant batch
is now checked at the primitive level), with durable cost accounting, then
implement the complete point map and paired endpoint measurement protocol.
Construction, serialization, cache hydration, I/O and verification all need
separate measurement. Large experiments remain subject to native EKS,
checkpoint, isolation and admission rules.

Sources: the existing CM constructor's `CONSTRUCTOR.md` reciprocal-power
derivation, FLINT `fmpz_mod_poly` root-product/fast-evaluation API contracts,
and the frozen source/kernel and previously independent PARI checks. This
implements those contracts; no new algebraic identity or formal proof is
promoted. See [the source-linked report](../reports/poly-blocks.pdf).
