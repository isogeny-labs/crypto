# Candidate spike40: complete point dataset before PDP comparison

The active objective is matched n83 factor bases that reduce PDP difficulty,
with FFD, LFD, top-form regularity and solving degree kept separate. This stage
fully enumerates one predefined candidate on all three original curves; it does
not select a winner or claim a degree improvement. The baseline remains the
complete low-coordinate base in ../n83_fixed_factor_base_20261028.

## Candidate and scope

For selector index i in [0,2^27), define
x(i) = (i & (2^26-1)) | ((i >> 26) << 40).
The binary-field basis is {1,z,...,z^25,z^40}; the prime-field domain uses the
same integer weights {1,2,...,2^25,2^40}. This is an injective map on all indices,
so there are exactly2^27 candidate x-values in each family. For prime, every
x is below2^41, well below the unchanged83-bit prime, with no modular collisions.
The same candidate manifest will be used for m2..9. Every affine lift and both
distinct signs are retained; no infinity, subgroup filtering or cofactor change.
Actual liftable-x and point counts are measured, not assumed equal to baseline.

Moving one basis coordinate is a controlled change in the domain's multiplication
structure. It is not a novel mechanism claim or evidence of an elite base. Keep
this predefined candidate even if every later degree comparison is unfavorable.
All earlier datasets and every failed attempt remain preserved.

## Prior work and interpretation

The repository's existing koblitz_n83_factor_base_sweep_20261008 README already
covers polynomial, random and affine subspaces and explains the n83 Frobenius
invariant-subspace obstruction. This candidate is a bounded follow-up in that
established family, not a rediscovery of a new attack. The current lead-atlas
helper remains absent in the adjacent checkout; existing records were consulted.
Primary literature checked before choosing this construction:

- Kousidis–Wiemers, https://arxiv.org/html/1906.05594: factor-base subspaces,
  summation polynomials and first-fall limitations; terminology can differ from
  the top-form regularity convention used in our preceding datasets.
- Trimoska–Ionica–Dequen, https://pmc.ncbi.nlm.nih.gov/articles/PMC7334981/:
  PDP encodings and subspace factor bases; these are not complete IC/rho results.
- Prior fixed-base certificate datasets in this branch: FFD/LFD2 does not imply
  low original top-form regularity, and prime chain tops can be positive-dimensional.

No new ideation/novelty ledger promotion is made in this enumeration stage.

## Encoding and exhaustive validation

Native factor_base adapts the previously validated generator. Arithmetic is
unchanged except batch inversion receives mapped x-values. Prime x^3+2x+3 fits
u128 because x<2^41; sampled slow modular arithmetic checks that assumption.
N83FB02 records store the selector index and y; actual x is reconstructed using
the manifest's bit positions. The header binds the moved coordinate bit40.
FORMAT.json specifies every byte, encoding and count. This differs intentionally
from N83FB01, whose first record field was the actual small x-coordinate.

Every candidate is checked after writing, including omissions. Binary omissions
use the trace criterion; prime omissions use Euler's criterion. Stored points,
complete sign pairs, ordering and count coverage are verified. Distributed
samples use slow independent field arithmetic. Controls cover the discontinuity
at selector2^26, the final candidate, altered y, removed records and shard joins.

## Frozen execution

384 shards of2^20 candidates,128 per curve. Six native worker threads,8GiB
virtual-memory cap,5400-second producer cap plus20-second termination grace,
5700-second outer limit. Every completed shard and receipt uploads every30s;
partial files are retained as well. No retries or duplicate live runs. Complete
means all384 receipts, family manifests and final index exist and verify.
Partial terminal output never counts as full enumeration.

Use the existing-host fallback after the recorded EKS shared-bucket failure,
without altering IAM or quotas. Refresh EKS inventory first. No timing claim.
Freeze protocol and commit instrument source before target data.

## Subsequent PDP comparison requirements

After complete bases are available, generate all m2..9 systems for all three
curves. Pin each planted point to a stored shard. Reuse baseline natural targets
for paired comparisons; planted targets are separate correctness controls whose
targets may differ. Hold generator/preprocessing/solver settings fixed. Preserve
all original and transformed equations, failures and bounds. A new protocol
must precede those measurements; this enumeration does not measure degrees.
