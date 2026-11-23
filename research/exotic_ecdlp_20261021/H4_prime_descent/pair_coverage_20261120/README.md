# Exact H4 three-sum coverage — 2026-11-20 UTC

The fixed `n=19` lifted base reaches **216,363 of 524,447 nonidentity
points** on its curve through three-point addition. Thus a uniformly
chosen nonidentity point on this exact curve has a `216363/524447`, or
**41.2555%**, chance of admitting a three-summand relation from this
119-point base. The complementary 308,084 points have zero such relations.
This is an exact finite-group count, with no sampling uncertainty.

| Quantity | Exact value |
| --- | ---: |
| Curve order | 524,448 |
| Lifted base points | 119 |
| Ordered pair entries | 14,161 |
| Distinct pair sums | 7,019 |
| Ordered triples, with multiplicity | 1,685,159 |
| Distinct three-sum targets | 216,363 |
| Identity in the three-sum image | no |
| Uniform nonidentity target hit fraction | 216,363 / 524,447 |
| Previously recorded planted/ordinary target triple counts | 6 / 0 |

The curve is `y²+xy=x³+a₂x²+a₆` over `F₂¹⁹`, with polynomial-basis
modulus `524327`, `a₂=103588`, and `a₆=419887` in that basis. The frozen
subspace basis is `[49377,209697,403728,164783,422496,216178,267660]`.
All points over these 128 x-coordinates are included, with no subgroup
filter. [`protocol.json`](protocol.json) fixes the source hash, input law,
method, 180-second cap, and claim boundary before execution.

The [runner](run.py) forms an ordered pair-sum multiplicity table and
adds each base point to each distinct pair sum. It counts every target's
ordered-triple multiplicity, checks that the total is `119³`, and replays
the two [earlier targets](../pair_index_20261120/README.md). Pair-table
setup took 0.226 seconds; full image enumeration took 8.707 seconds,
with 380,108,800 bytes peak RSS in one ordinary local run. These are
diagnostic stage times, without a host-isolation claim.

The independent [audit](audit.py) reconstructs the curve from the recorded
integers. It counts points using the binary-field trace criterion, obtaining
524,448 curve points and 119 lifted base points. It then loops directly
over all 1,685,159 ordered triples, reproducing the 216,363-point image
and the complete 25-bin multiplicity histogram. Its direct triple pass
took 17.429 seconds. [Result](result.json), [audit receipt](audit.json),
and [checked Sage runtime](sage-runtime-info.json) retain the evidence.

The group-law reason for the pair lookup is formalized in
[`H4PairIndex.pair_lookup_iff`](formal/PairIndexCoverage.lean): for any
base predicate and any group satisfying the explicit associativity,
identity, inverse, and subtraction laws, `a+b+c=R` holds exactly when
the pair sum `a+b` equals `R-c`. The readable proof adds `-c` to the
forward relation; the reverse proof adds `c` to the lookup equality.
`lean PairIndexCoverage.lean` under pinned Lean `v4.36.0` passed, and
[`#print axioms`](formal/build-output.txt) reports none. This theorem
justifies the index query structurally.

**Proof map:** the Lean declaration proves the pair-index equivalence
for every group obeying its stated laws. The exact `216363` image
cardinality and `524448` curve order are independently checked finite
enumerations in Python/Sage. Their Lean formalization is partial: a
Lean definition of this binary field and elliptic point group, a checked
bridge from the integer encodings to those points, and a kernel-checked
image-cardinality certificate are still needed. The arithmetic count is
therefore recorded as an audited computational result, not a Lean theorem.
`lean4checker --fresh` is unavailable on this host.

Reproduce from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/H4_prime_descent/pair_coverage_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/pair_coverage_20261120/run.py
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/pair_coverage_20261120/audit.py
cd research/exotic_ecdlp_20261021/H4_prime_descent/pair_coverage_20261120/formal && lean PairIndexCoverage.lean
```

The exact hit fraction applies to a uniform point of this full curve and
this unfiltered lifted base. It does not price relation collection,
subgroup filtering, final matrix solving, or target descent. The next
experiment should fix the DLP subgroup and candidate policy, then charge
the pair index and every failed natural query under that workload.
