# Original Prime31: signed weighted-eight formal-support bound and RR9 experiment gate

2026-11-18 UTC · existing [formal-support lane #4722](https://github.com/isogeny-labs/crypto/pull/4722) · prospective [weighted RR9 algebraic lane #4728](https://github.com/isogeny-labs/crypto/pull/4728) · [frozen protocol](PRIME31_W8_SIGNED_FORMAL_PROTOCOL_20261118.md).

**Status:** Exact finite coefficient-vector census implemented in the pinned original Prime31 Rust crate, with a separately written Python bitmask-partition/BigInt replay and source-specific GitHub Actions. **Source-specific validation PASSED twice** on pinned original Rust 1.100.0: [GitHub Actions run #44354034858](https://github.com/isogeny-labs/crypto/actions/runs/44354034858) (full original checkout) and [run #44354870140](https://github.com/isogeny-labs/crypto/actions/runs/44354870140) (optimized sparse checkout). Each passed native library tests, two focused signed W8 tests, emitted the exact finite-count JSON and passed a separately written Python bitmask-partition/BigInt group-law replay. [Raw first-run JSON artifact #14451785001](https://github.com/isogeny-labs/crypto/actions/runs/44354034858/artifacts/14451785001). The tests are producer-side source-specific evidence; no independent signed PCC has been issued. This is a structural upper bound on natural signed relation yield, *not* a nonenumerative extractor or a faster ECDLP solver. Independent signed PCC remains outstanding.

## Curve, target, and exact count

Original ordinary cofactor-one curve, unchanged throughout:

- `p = 2147483647 = 2^31-1`, `E: y^2=x^3+847514846*x+2087444724`;
- `n=2147446831` prime, `G=(625037715,2100627251)`;
- canonical first-B liftable x-coordinate basis, weighted slots `w=(1,3,5,7,11,13,17,19)`;
- for every slot, either sign is permitted, as in #4728. The **signed** and positive-only counts are not interchangeable.

For `B` formal basis symbols `P_i`, let

```
F_B^±(w) = #{ (c_0,...,c_(B-1)) in Z^B :
               c_i = sum_{j : index_j=i} epsilon_j*w_j,
               index_j in {0,...,B-1}, epsilon_j in {-1,+1} } .
```

No ECDLP scalar appears in this definition. Each formal vector maps to **at most one** EC group sum; distinct formal vectors might map to the same group point. For a uniformly distributed public residual `R`:

```
Pr[ R has a signed eight-slot decomposition ] <= min(1,F_B^±(w)/n).
```

This does not assume independence of the elliptic-curve sums. It is a rigorous necessary-yield bound, not an estimate of the exact EC support.

### Exact finite counts

| B | All signed words (256 B^8) | Distinct signed formal vectors | Upper bound for uniform hit | Positive-only formal vectors (NOT signed bound) |
|---:|---:|---:|---:|---:|
| 1 | 256 | 75 | 0.00000349% | 1 |
| 2 | 65,536 | 5,625 | 0.00026194% | 75 |
| 3 | 1,679,616 | 258,027 | 0.0120155% | 2,460 |
| **4** | **16,777,216** | **5,491,889** | **0.2557404%** | **35,428** |
| 8 | 4,294,967,296 | 3,240,125,537 | vacuous (formal count exceeds n) | 14,233,920 |
| 12 | 110,075,314,176 | 96,962,798,865 | vacuous | 399,246,420 |
| 16 | 1,099,511,627,776 | 1,023,109,635,265 | vacuous | 4,118,222,176 |
| 31 | 218,340,105,584,896 | 214,117,016,830,955 | vacuous | 843,295,566,676 |

The B4 positive-only count would imply a much rarer hit rate, but **cannot** be used for the signed RR9 system. The signed count explicitly includes `2^8` independent orientation choices and tracks cancellations, so its B4 upper bound is the relevant one.

### Proof: quotient by signed coefficient vectors

For a word, partition its eight slot positions by which common base index they use. There are `Bell(8)=4140` set partitions for each fixed choice of signs, giving `256*4140=1,059,840` signed partition/sign candidates. For each partition, add the signed weights in each block, discard blocks whose coefficient is zero, and sort the remaining nonzero integer coefficients to get a canonical multiset signature. Preserve the **minimum total number h of blocks** that realizes each signature: vanished blocks still consume base indices in that realization.

For a signature with `k` nonzero entries and multiplicities `m_v` of equal coefficients, its number of distinct B-coordinate assignments is exactly

```
  (B)_k / product_v(m_v!)       provided B >= h,
  (B)_k = B*(B-1)*...*(B-k+1).
```

Signatures are disjoint classes of coefficient vectors, and every vector is represented by at least one signature with `h<=B`. Summation gives the exact formal count for each B. Zero signed sums produce the all-zero vector (when admissible), counted once. Neither independence nor elliptic-curve arithmetic is used to infer this quotient.

The native Rust program reuses `prime31-algebraic-ic::{Arithmetic,PRIME31,check_curve}`, checks the 31 original basis points and the actual EC collision `1+7=3+5`. The independent Python verifier uses a different partition generator (subsets/masks, not restricted-growth labels), exact Python integers and its own affine EC group law. The B2 signed word space is directly brute-enumerated, and the B1 cancellation case and tampered-count rejection are tested. The observed exact counts are mathematical enumerations; they are not timing measurements.

## New experiment decision for #4728

**B4 natural positive controls are statistically scarce, even for a perfect extractor.** For uniform public shifted residuals the upper hit probability is `5491889/2147446831 = 0.002557403946...`. Sixteen such marginally uniform residuals have probability of *any* signed B4 representation at most `16*5491889/n = 0.04091846...` by the union bound. For independent uniform shifts, the expected number of oracle calls needed for the first hit is **at least 392**; at least **12,121.67** calls in expectation to obtain 31 hits, even if every positive hit yields a new independent row. This is a bound on yields, not a claim that 31 hits imply rank.

The existing RR9 B4/B8 *planted* positive controls remain valid for checking soundness. However, a test that uses only 16 random unplanted B4 targets and finds no witness would not falsify an otherwise correct extractor. **Do not confuse no relation, incomplete F4, and timeout.** Retain grader-only exhaustive enumeration at tiny B, public-only coefficient extraction in the candidate, and separate sealed planted fixtures.

At B8 the signed *formal* bound is above the group order and tells us nothing about true natural coverage; the positive-only 0.663% bound cannot justify rejecting signed B8 runs. To sharpen B8 requires actual EC collision/energy measurements or a stronger algebraic structural bound without using generic sum tables in the candidate.

## Prior work and prioritized plan

- **Already measured:** [#4714](https://github.com/isogeny-labs/crypto/pull/4714) has a complete original-curve 1,024-target Rust weighted-M6 solver, but its generic 3+3 enumerator lost to matched shared BSGS by roughly 21x. [#4722](https://github.com/isogeny-labs/crypto/pull/4722) previously counted *positive six-weight* signatures. [#4716](https://github.com/isogeny-labs/crypto/pull/4716) bounded mask-only eight-slot support. None establishes a non-generic solver.
- **First next measurement:** [#4728](https://github.com/isogeny-labs/crypto/pull/4728) should run public-residual-only signed W8 B4/B8 unknown-chart extraction with separate sealed planted positives and exhaustive grader-only truth, using reused Rust F4 and rootfinding. Show actual extracted charts and replayed signed witnesses; any algebraic unsolved instance is *undetermined*, not a no-relation.
- **Second gate:** only after that, use natural B31 signed relations for rank-31 and 1,024 held-out original-curve logarithms, no scalar injection and no generic sum table/fallback. Count every failed/censored extraction and its F4 work.
- **Decisive full-cost panel:** same-runner cold 1/64/1024 Q versus reused mixed-4+4 IC, cached/cold BSGS, and optimized negation-map rho. The historical best BSGS was 144.049 ms for 1024 Q; the previous M6 3,766-query schedule would need **less than 38.25 microseconds per call before any setup** just to match that reference. A new W8 schedule has *different yield* and must receive its own cost budget, not inherit this number.

Relevant publications reviewed: Semaev's summation-polynomial work (2004), Amadori–Pintore–Sala prime-field method (2018), Yokoyama et al. on naive Semaev complexity (2020), and the Bochum prime-field tensor-network manuscript (2026). #4698's stronger subexponential ECDLP theorem remains a separate unverified, unbenchmarked result. The repo research-KB service was inaccessible in this connector session; this is a bounded direct-file/prior-literature review, not proof of novelty.

**Conclusion:** New exact finite support bounds make the B4 natural-control design realistic and avoid falsely treating an expected zero yield as failure. The urgent unresolved component is still **public-target-only RR9 coefficient discovery**. This report does not claim to have solved it.
