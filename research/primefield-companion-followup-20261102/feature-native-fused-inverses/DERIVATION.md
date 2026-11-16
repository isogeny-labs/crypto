# Fused stride and adjacent denominator inversion

T-1875. Producer assertion; independent signed verdict pending. [pcc:unverified]

## Construction and invariant

The predecessor NeighborReader first batch-inverts the F1 stride denominators, then independently batch-inverts adjacent denominators for F2. Both lists are known from indices before any coefficient value is advanced. Concatenate them and use one standard prefix-product inversion pass.

For a chunk of C output pairs with remaining length R, let S=min(C,R−1). The S stride denominators are the interval products from the existing anchor recurrence. The C adjacent denominators are N_j+1 at the output indices. In the admitted prime-field domain all are units: interval factors lie in1..h, and1≤N_j<N_j+1≤h<r. The existing adjacent alignment proof is inherited.

For a denominator list d0,...,d_(K−1), define p0=1 and p_(j+1)=p_j*d_j. Invert p_K once. A reverse pass returns d_j^-1 as inverse*p_j and updates inverse*=d_j. This is the same inverse formula used separately in both predecessor passes. The implementation verifies p_K*inverse=1 and rejects zero/out-of-range inputs or a failed unit check.

Use the first S inverses to advance F1 by its signed long-stride recurrence and the last C inverses to derive F2 from each output F1 value. Preserve the extra transition after the last output of a nonfinal chunk: S=C there, so the anchor stores the first value of the next chunk. For the final chunk S=C−1. This boundary invariant is necessary for repeated calls to emit the same sequence. The new reader commits anchor value/index/remaining state after successful chunk construction. Outputs and anchor state are compared after every emitted pair.

The joint evaluator is generated from the predecessor unchanged except for its reader type. C0, canonical update, zero handling, caps, phase extraction and public-point verification retain previous paths. Initial binomial setup remains the same SharedCoeffStream constructor.

## Exact local accounting

Count modular multiplications outside inverse exponentiation explicitly through counted_fused_mul. For K=S+C denominators, prefix products use K multiplications, reverse inversion uses2K, and product verification uses1. Coefficient advancement uses2S and adjacent conversion2C. Total:5(S+C)+1 multiplications and one inverse exponentiation per nonempty chunk.

The predecessor uses5(S+C)+1+1_(S>0) multiplications and1+1_(S>0) inverse exponentiations over those same chunk operations. Thus every chunk with a stride transition removes exactly one inverse exponentiation and one product-verification multiplication. A final singleton chunk S=0 is unchanged. Reference counts are derived from the pinned predecessor's loops on the exact observed chunk boundaries; new multiplication counts are instrumented and asserted against the formula.

This accounting excludes interval-product generation, stream initialization and polynomial evaluation, which are unchanged and remain charged, as do additions, integer/index work, allocations and inverse exponentiation's internal multiplications. It is a local operation statement, not an end-to-end timing result. In the test stdout the cumulative_counter_observation_sum field inherited by the mode checker is only a sum of observations, not a product total, and is not used for claims.

C≤B, S≤B and at most2B denominators are retained. The product engine receives at most2B interval requests, preserving its prior request cap. The fused prefix vector has up to2B+1 entries, alongside denominator/inverse vectors, interval results, requests, output pairs, old-buffer overlap and joint polynomial values/caches. Actual Vec capacities and simultaneous lifetimes remain charged; B is not a process-byte ceiling. Original4,563cohort and prior evidence remain unchanged. No cloud or timed workload is run.

## Controls

Ten supplied primes, all admissible m≥3 and caps1,2,7,16,64 compare the fused reader with the published NeighborReader. Checks cover each output pair, anchor value/index/left state, reader position, each chunk boundary, exact multiplication/inversion formulas, exhaustion and maximum denominator-list size. Separate batch-inverse controls verify individual d*inverse=1 identities and reject empty, zero, out-of-range and nonunit examples.

Mode controls compare output values and per-branch scalar/pair product counters. State controls compare canonical triples, branch supports/counts, zero flags and materialized roots through four radii. Public recovery compares the neighbor and fused adapters at caps64/16. Successful paired-stage counters exclude other recovery stages. See RESULTS.md and validation01 for observed counts, source pins and any failures.

## Sources and next direction

Reviewed feature-native-neighbor-modes/neighbor.rs and SharedCoeffStream in context.rs. NIST DLMF26.3.E1/E6, https://dlmf.nist.gov/26.3, supplies the binomial identities underlying both recurrences. The batch-inversion formula is derived above and reuses the predecessor's standard prefix/reverse method; no novelty claim. KB MCP unavailable; scripts/research-kb status reports no executable. Exact predecessor and primary recurrence reference are the fallback.

Next reduce retained coefficient/evaluator state or derive direct constraints that avoid constructing the initial support. The fused reader addresses inverse overhead only; the initial support and public sign-source construction remain explicitly charged dependencies. A timing comparison requires separately admitted frozen workload and full accounting.
