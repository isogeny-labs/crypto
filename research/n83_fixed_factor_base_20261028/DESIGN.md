# Fixed n83 factor bases, ell=27

The user explicitly selected 2^27 candidate x-coordinates per curve and one
unchanging factor base for all decomposition arities m=2 through 9. This replaces
the earlier balanced ell=floor(83/m) choice for future inputs; it does not alter
or reinterpret the earlier degree measurements.

Enumerate x=0,...,134217727 without sampling. For binary fields, the integer
encodes polynomial-basis coefficients. Retain every affine lift, including both
signs when distinct; omit infinity. Do not filter by a subgroup or multiply by a
cofactor. Distinct curves have distinct bases, each reused across all eight m.

Exact curves copied from the original frozen n83 generator:

- Koblitz: GF(2^83), modulus z^83+z^45+z^2+z+1, y^2+xy=x^3+1.
- Binary: the same field, y^2+xy=x^3+x^2+(2^82+0x51b).
- Prime: p=4835703278761981908090881, y^2=x^3+2x+3; original Proth
  certificate uses witness 3, with p-1=(2^40+29)*2^42.

## Algorithm and acceptance

Native Rust uses carry-less multiplication and three reduction folds for the
binary field; batch inversion in blocks of 4096; precomputed linear half-trace
and trace maps. Prime arithmetic uses two-limb Montgomery multiplication. The
square-root routine resolves six 7-bit digits in the 2^42-order Sylow subgroup,
then corrects the odd-part exponentiation. This is a fixed-prime expression of
the Tonelli-Shanks construction; no novelty or speed claim is made.

Produce 128 disjoint shards of 2^20 x-values for each family, 384 shards total.
Each shard is ordered by x, then encoded y. After writing and decompressing it,
the verifier traverses every candidate x. It validates every stored point and
complete sign pairs. For every absent prime x it checks Euler's criterion;
binary completeness uses the trace criterion, with special handling at x=0.
Independent shift-and-add arithmetic checks every 8191st x in each shard and
its final x, including absence decisions. Positive tiny-domain controls and
negative altered-coordinate/deleted-point controls must pass first.

No shard is admitted without validation. All 384 range certificates and three
family manifests must exist before index.json can state complete=true. SHA-256
binds both uncompressed records and compressed file bytes. The family manifest
hash is the identity that all m=2..9 reference. No timing, ECDLP, degree,
subgroup-order, or independent-scientific-review conclusion is requested.

The initial development tests rejected a two-fold binary reduction: degree88
terms can remain after the second fold. Added the required third fold before
all production data. All four native tests then passed, covering 5000 arithmetic
cases, square-root existence against independent Euler tests, chunk boundaries,
high-domain endpoints, and corruption/omission controls.

## Storage and resource limit

One ordinary EKS Spot job, 16 CPUs, 8 GiB, 3600-second deadline, zero automatic
retries. Upload completed immutable shards periodically, excluding temporary
files; upload metadata last. The ordinary job is not a CRIU-resumable job.
On failure, preserve completed shards and report the gap before a new attempt.
Only s3://setec-astronomy-artifacts is written. Full data is in the artifact
store; Git retains code, manifests, provenance, validation and report.

## Literature checked before implementation

- Original source: ../n83_ic_degree_sweep_20261027/src/main.rs, lift/on-curve
  routines and generate() curve parameters; input domains were implicit and
  only m planted points were retained.
- SEC 1 v2.0, sections 2.2 and 2.3.4,
  https://www.secg.org/sec1-v2.pdf : prime square-root lift, binary quadratic
  lift, x=0 exception, and point-equation validation. Read the actual algorithm.
- Taverne et al., Software implementation of binary elliptic curves: impact of
  the carry-less multiplier on scalar multiplication, section 2,
  https://cacr.uwaterloo.ca/techreports/2011/cacr2011-07.pdf : polynomial-basis
  carry-less multiplication and linear half-trace precomputation. Reviewed the
  odd-extension-degree condition. This study has degree83, so it applies.
