# Does a p+1 torus factor base accelerate streamed five-term decomposition?

Date: 2026-11-26  
Curve: production Prime31 instance over `p = 2^31 - 1`  
Implementation: Rust  
Standing: producer and replay evidence, `[pcc:unverified]`

## Answer

No, not in the generic streamed `2+3` algorithm tested here. At equal factor-
base cardinality, changing the ordinary base to the p+1 norm-one-torus Dickson
base changes which targets decompose but does not reduce the complete query's
work. The two arms have identical retained-table sizes and identical full-scan
operation vectors. [pcc:unverified]

This is a useful null result: torus parametrization by itself is not the big
speedup. A material improvement now requires an exact pair-spectrum query that
avoids enumerating all pairs, a batched cross-target scheme, or an algebraic
PKM/Semaev solver whose cost is changed by the factor-base equations.

## Matched construction

The arms use the production constructors:

- torus: `dickson_factor_base(instance, 6, 0, 0)`;
- ordinary: `prime_factor_base(instance, 34)`.

Both produce 34 abscissae and 68 signed curve points. The solver retains the
complete unordered three-sum set modulo elliptic negation, keyed by abscissa.
For each target it enumerates unordered two-sums in blocks of 1,024, forms the
target complement projectively, batch-normalizes the block, probes the table,
and discards the block. No pair vector or pair hash table survives a query.

The frozen public manifest uses one precommitted 256-bit seed, 256 common random
targets, eight arm-specific planted positive controls, and nine counterbalanced
primary/A-A rounds. Full-scan exact counts are the primary endpoint; the local
non-isolated wall clock is only a producer diagnostic.

## Exact result

The following values are invariant across all repetitions:

| Metric | Dickson torus | Ordinary small-x | Difference |
|---|---:|---:|---:|
| Signed points | 68 | 68 | 0 |
| Retained three-sum states | 27,370 | 27,370 | 0 |
| Distinct retained x keys | 26,248 | 26,248 | 0 |
| Setup inversions | 1 | 1 | 0 |
| Setup multiplications | 368,098 | 368,098 | 0 |
| Setup squarings | 144,160 | 144,160 | 0 |
| Setup additions | 29,716 | 29,716 | 0 |
| Setup doublings | 68 | 68 | 0 |
| Full-scan pair candidates | 600,576 | 600,576 | 0 |
| Full-scan table probes | 600,576 | 600,576 | 0 |
| Full-scan inversions | 768 | 768 | 0 |
| Full-scan multiplications | 11,896,064 | 11,896,064 | 0 |
| Full-scan squarings | 5,413,888 | 5,413,888 | 0 |
| Full-scan additions | 1,201,152 | 1,201,152 | 0 |
| Full-scan doublings | 17,408 | 17,408 | 0 |

The pair count is forced by the matched construction:
`256 * binomial(68 + 1, 2) = 600,576`. Streaming saves persistent pair-table
memory, but an exact full query still has quadratic pair time.

The separate first-hit pass found three of the 256 common targets with the
torus base and one with the ordinary base. It therefore performed 595,539
versus 599,115 table probes, a reduction of 3,576 probes or 0.597%. Pair
candidates fell by 1,620 or 0.270%. This is not a matched expected-yield gain:
the exact decomposition-count identity fixes the uniform-target mean at equal
signed-base cardinality, and four total hits cannot establish a distributional
advantage. It is retained only as finite-stream early-exit behavior.

Factor-base construction also has different primitive mixes. The torus arm
scanned 64 selectors and made 63 selector plus 64 curve square-root solves. The
ordinary arm scanned 71 abscissae and made 71 Legendre-symbol plus 34 square-
root solves. These are reported as a vector rather than collapsed into an
unsupported common cost.

## Wall-clock boundary

The run had no CPU isolation. Pooled local medians over 18 primary/A-A rows
were 63.878 ms (torus) and 64.537 ms (ordinary) for the full scan, an apparent
1.010x ratio. A/A direction reversals and large outliers make that difference
non-admissible. The first-hit pooled median was slightly slower for the torus
arm despite its fewer probes. No wall-clock speed claim is made. The exact
operation-vector equality is the decision evidence. [pcc:unverified]

## Independent replay and proof boundary

`norm_one_torus_base_compare_verify.rs` does not call the candidate projective
arithmetic or streaming solver. It rebuilds both production bases and targets,
constructs the affine three-sum set, streams affine two-sums, compares the full
264-target mask for each arm, and replays every recorded five-index witness.
It passed all 528 decisions, the three and one random hits, and all eight
planted controls per arm. [pcc:unverified]

The manifest, SHA-256 digests, disclosed seed, certificates, and affine replay
are transparent correctness and reproducibility evidence. They are **not** a
zero-knowledge proof, a signed attestation, or proof of producer-host execution.
No such claim is made.

## Reproduce

From the repository root, with the source revision named in the manifest:

```sh
cargo run --release --example norm_one_torus_base_compare -- \
  --manifest research/norm_one_torus_base_compare_20261126/public-precommit/manifest.json \
  --certificates /tmp/norm-one-torus-certificates.json

cargo run --release --example norm_one_torus_base_compare_verify -- \
  --manifest research/norm_one_torus_base_compare_20261126/public-precommit/manifest.json \
  --certificates research/norm_one_torus_base_compare_20261126/public-precommit/certificates.json

sha256sum -c \
  research/norm_one_torus_base_compare_20261126/public-precommit/artifacts.sha256
```

The first command prints CSV rows to standard output. Reproduced wall times are
expected to vary; exact operation vectors, result digests, masks, and witnesses
must not.

## Algorithmic decision

Do not spend the next optimization cycle merely replacing an equally sized
ordinary factor base with this torus base inside the same generic `2+3` loop.
The next falsifiable target should attack the 2,346 pair candidates per target:
query the exact pair spectrum without enumerating it, or amortize it over many
targets. Until that algorithm exists, the p+1 geometry is useful for defining
structured algebraic systems, not as a standalone generic-query speedup.
