# Factor-base factory

Native Rust generation, content-addressed archives, bounded relation evaluation,
and exact bounded binary leading-form regularity. This is a standalone Cargo
package so the precomputation worker does not need to link the entire research
library. It uses the same mathematical relation and regularity definitions as
`examples/factor_base_closure_sweep.rs`; archived historical outputs are unchanged.

## Build and use

```sh
cargo build --release --locked --manifest-path tools/factor-base/Cargo.toml
FB=tools/factor-base/target/release/factor-base
$FB --help
$FB requirements tools/factor-base/examples/binary-prime-degree83.json
$FB build tools/factor-base/examples/binary-prime-degree83.json /tmp/factor-bases
```

`build` returns a `manifest_sha256`. Substitute that value for `MANIFEST` below.
All numbers in inputs accept decimal or `0x` hexadecimal strings. Canonical
archives use lowercase hexadecimal. Unknown JSON fields are rejected.

```sh
$FB verify /tmp/factor-bases MANIFEST
$FB evaluate /tmp/factor-bases MANIFEST tools/factor-base/examples/workload.json /tmp/evaluation.json
$FB replay /tmp/factor-bases MANIFEST /tmp/evaluation.json
$FB compare /tmp/evaluation.json /tmp/another-evaluation.json
$FB batch tools/factor-base/examples/grid.json /tmp/factor-bases /tmp/batch.json
```

The supplied workload is a functional fixture. Change its budgets deliberately
before using it as a research protocol. `batch` preserves a row for every recipe,
including empty bases and failed construction. Output receipts are immutable:
use a fresh path for each execution. `verify` and `replay` regenerate with the same
native implementation; their receipts do not claim independent implementation or
independent-host validation.

## Supported constructions

| Recipe | Parameters | Meaning |
|---|---|---|
| `binary_subspace`, `coordinate` | dimension, seed, offsets | Span of initial polynomial-basis coordinates, with an explicit union of affine shifts |
| `binary_subspace`, `geometric` | dimension, seed, offsets | Independent powers of a deterministically selected field element |
| `binary_subspace`, `random_linear` | dimension, seed, offsets | Seeded independent vectors over F2 |
| `binary_subspace`, `trace_zero` | dimension, seed, offsets | Seeded subspace of the absolute-trace kernel; shifts remain explicit |
| `binary_explicit` | basis, offsets | Import an independent basis in the exact polynomial representation |
| `frobenius_orbits` | seeds, seed | Distinct Koblitz point orbits, including both signs and every Frobenius image |
| `prime_progression` | start, step, count | An interval when step=1, otherwise a progression modulo p |
| `random_x` | count, seed | Distinct seeded x-coordinates, over either supported field type |

For every candidate x, the generator solves the curve equation, stores both
rational signs, checks the curve equation again, and retains only points with
`[r]P = O`. It never silently cofactor-projects a point away from its x-coordinate
constraint. Generator construction for the public binary fixture is separate
from factor-base generation. Factor-base recipes do not use multiples of the
generator to manufacture known logarithms.

For binary affine recipes:

```text
X = union over offsets c of (c + span_F2(v_1,...,v_l))
B = { (x,y) in E(F_q) : x in X and [r](x,y) = O }
```

Point sets are materialized rather than folded by Frobenius. A future quotient
format must certify the subgroup eigenvalue and retain every sign/phase needed
for relation coefficients. Ordinary binary curves do not receive Koblitz folding.

## Curve validation and identity

The input binds the field type, exact modulus/polynomial, model coefficients,
whole-group order, subgroup order, cofactor, and generator. SHA-256 of its canonical
JSON identifies this representation for the archive. This digest is an artifact
identity; it does not replace the repository's ICV1/EC1 registry or infer a registry
join. Cross-repository registry integration remains separate.

The implementation supports ordinary binary models `y^2+xy=x^3+a*x^2+b` in
degrees 2 through 571 and short Weierstrass prime-field models `y^2=x^3+a*x+b`
for p>3, up to 576 bits. It rejects singular models, reducible binary moduli,
noncanonical coordinates, inconsistent N=r*h, non-coprime r and h, invalid
generators, and claimed group orders outside the Hasse interval.

Binary irreducibility is checked exactly. Koblitz group orders are checked from
the Frobenius trace recurrence. Other group orders are counted exactly when the
field has at most 4096 elements; larger generic orders remain `caller_asserted`.
The deterministic Miller–Rabin base set certifies primes below 2^64. Larger field
moduli and subgroup orders receive additional fixed-base tests and remain
`probable_prime_only`; this release does not import primality certificates.
Mathematical conclusions depending on those inputs remain conditional on their
primality and, where relevant, the supplied whole-group order.

The full-width fixtures cover an ordinary Koblitz model over the prime-degree
binary extension of degree 83 and the published secp256k1 parameters. The latter
come from [Standards for Efficient Cryptography 2 (SEC 2), section 2.4.1](https://www.secg.org/sec2-v2.pdf).
Their presence validates construction paths; it does not establish a yield or
regularity improvement at those sizes.

## Measurements and selection

The structural report measures dimensions of V, VV, VVV, and the intersection
of V with its Frobenius image. These are candidate-screening features. Neither
small product span nor trace constraints imply low solving degree.

Relation evaluation uses a fixed seed and split independent of the recipe. Its
sampler selects finite subgroup points uniformly with replacement by rejection
sampling x and a root slot. Callers can instead supply an explicit target list.
Large cofactors can exhaust the sampler's explicit attempt budget; supplying
public targets is then required. Supplied targets receive no population interval.

For arities 2..8 the reference evaluator enumerates unordered multisets, permits
repetition, excludes inverse pairs, and verifies every retained sum. Each recursive
state counts against the cap. This implementation is a bounded reference
enumerator, not a scalable relation solver. An incomplete enumeration preserves
positive witnesses, reports a witnessed lower bound, and leaves the estimated
yield null unless every sampled query already has a witness. Completed seeded
panels include a Wilson interval; it is a per-candidate descriptive interval and
does not correct for selection across many candidates.

For a single binary affine subspace, the algebraic engine builds the exact
Boolean ANF of a Semaev S3 chain. For arity m and dimension l it has
`m*l + (m-2)*n` variables and `(m-1)*n` coordinate equations. It measures the
first saturated degree of the homogeneous leading-form ideal in
`F2[z_1,...,z_k]/(z_i^2)`, using exact rank up to the chosen degree cap.
The hard variable ceiling is 14. This is not affine solving degree, first-fall
degree, or a semi-regularity prediction. The encoded ideal does not include
rational lifting, subgroup constraints, or intermediate-infinity branches.
Verified group-relation yield is therefore measured separately.

Unions of affine spaces and prime-field bases have an explicit unsupported
algebraic-encoding status in this release. Higher-arity binary chains usually
reach the variable cap quickly; a cap is not a negative algebraic result.

`compare` requires identical curve, workload, and sampled targets. It returns
a descriptive Pareto frontier over verified yield, point count, and the maximum
measured regularity in the configured algebraic target panel. Missing metrics
are excluded with a reason. It never selects a total-runtime winner. Useful
relation-matrix rank, linear algebra, affine solve cost, and complete ECDLP cost
are null because this tool does not measure them.

## S3 storage and reuse

The requested destination is:

```text
s3://setec-astronomy-artifacts/factor-bases/factory/v1/
  objects/<sha256>.jsonl
  manifests/<sha256>.json
```

Each point shard is sorted, deduplicated, and hashed. The manifest binds the
recipe, normalized curve, construction source digest, structure, counts, and
every shard's digest and length. Local writes use a flushed temporary file and
atomic create-if-absent publication. Existing content must agree byte for byte.

```sh
$FB upload-plan /tmp/factor-bases MANIFEST factor-bases/factory/v1
AWS_PROFILE=ecc-challenges-new AWS_REGION=us-east-2 \
  $FB publish /tmp/factor-bases MANIFEST setec-astronomy-artifacts factor-bases/factory/v1 /tmp/publication.json
AWS_PROFILE=ecc-challenges-new AWS_REGION=us-east-2 \
  $FB fetch /tmp/factor-bases-fresh MANIFEST setec-astronomy-artifacts factor-bases/factory/v1
```

The profile above was identified as able to read this bucket's location; see
VALIDATION.md for actual publication status. AWS CLI v2 runs only on the trusted
controller. Offline workers need no AWS credentials: they return content and
an upload plan to that controller. A capability issuer can use the plan to issue
short-lived presigned PUTs bound to key, length, and SHA-256; a worker-side
presigned HTTP uploader is not implemented here.

Publication uses `If-None-Match: *`, server-side SHA-256 verification, and AES256
server-side encryption, and publishes the manifest after every shard succeeds.
A 412 response is accepted only after the existing object's checksum and length
match. Other failures stop publication. The receipt retains ETag and VersionId
where S3 returns them. Fetch pins the HEAD response by ETag and, when available,
VersionId, checks every downloaded SHA-256, and regenerates the point set.
See [AWS conditional-write semantics](https://docs.aws.amazon.com/AmazonS3/latest/userguide/conditional-writes.html).

Immutable keys and conditional writes prevent accidental replacement through this
client. Bucket versioning, deletion restrictions, retention, and KMS policy are
administrative settings and are not modified by the tool. S3 verifies bytes, while
the native replay verifies the mathematical contents.

## Limits and tests

Generation is bounded by one million x candidates, two million stored points,
and 65,536 points per shard. It keeps the point set in memory. Download is capped
at 2 GB. A batch is resumable by deterministic recipe identity and object reuse;
individual long enumerations do not have mid-search checkpoints. Parallel workers
can use disjoint recipe lists and the same object store.

```sh
cargo test --release --locked --manifest-path tools/factor-base/Cargo.toml
cargo clippy --all-targets --locked --manifest-path tools/factor-base/Cargo.toml -- -D warnings
cargo fmt --manifest-path tools/factor-base/Cargo.toml --check
```

The ignored `write_public_fixtures` utility regenerates checked-in public inputs;
it is not part of the ordinary test suite. Frozen validation data must not be
silently regenerated. See VALIDATION.md for execution evidence and the separate
repository-wide gate.
