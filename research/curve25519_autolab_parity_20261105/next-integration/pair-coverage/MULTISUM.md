# Exact signed-multisum coverage envelope

This extends the source-specific pair coverage calculation. It is an exact combinatorial envelope followed by a probability bound under the same ideal-digest and uniform-target assumptions as ANALYSIS.md. It is not a recovery experiment or a lower bound for every decomposition algorithm. [pcc:unverified]

## Counting coefficient vectors

With B signed base classes, an m-term sum with repetition has a coefficient vector c in Z^B with sum |c_i| <= m and sum |c_i| congruent to m modulo2. Conversely every such vector can be represented with exactly m terms by adding cancelling pairs (B>=1). Group collisions may identify different coefficient vectors, so their count is an upper bound on decomposable group points, not an assertion that all sums are distinct. [pcc:unverified]

Let f_B(k) count vectors with exact L1 norm k. One coordinate contributes the generating series 1+2x+2x²+..., so

    f_0(0)=1; f_0(k>0)=0,
    f_(B+1)(k)=f_B(k)+2 sum_{j=0}^{k-1} f_B(j),
    N_B(m)=f_B(m)+f_B(m-2)+f_B(m-4)+... .

The native BigUint implementation uses prefix sums, with no floating-point arithmetic. Checks verify N_1(m)=m+1 and N_2(m)=(m+1)² for0<=m<=128, and N_B(2)=2B²+1 for1<=B<=128. The general recurrence follows from the coordinate generating series; these finite identities are sanity checks. [pcc:unverified]

## Registered base size and probe budget

For B=128, the loose per-target union bound is T*N_B(m)/(r-1), capped at1, with T=1,000,000. This upper bound first reaches1 at m=72. At m=71 it is still below1. This is only the point at which this bound becomes vacuous: it neither establishes full coverage nor predicts a successful solve, and it does not say every smaller m has negligible hit probability. The complete exact counts are in multisum.log. [pcc:unverified]

| m | N_128(m) |
|---:|---:|
| 2 | 32,769 |
| 4 | 179,011,585 |
| 8 | 458,682,026,098,689 |
| 16 | 16,603,667,194,634,318,815,559,681 |
| 32 | 518,651,887,222,042,530,038,013,527,420,420,622,123,009 |

For a direct36+36 split, each coefficient-vector list has

    N_128(36)=1688048373042070153619500350584701786778992641

entries, a151-bit integer (approximately1.69e45). This is the enumeration size of that specific coefficient-list construction. It is not a memory lower bound for compressed representations or a lower bound for arbitrary algorithms. Deduplication and algebraic structure would need their own demonstrated construction and cost. [pcc:unverified]

## Next construction criterion

The immediate question is an exact decomposition oracle that exploits a compressed or algebraic representation of a broad sumset without enumerating these coefficient lists. It must output public coefficients whose group equation verifies, retain its decomposition work and setup cost, and preserve the full-size target distribution. Changing only summand count with direct coefficient enumeration does not provide such an oracle. The existing miniature obstruction and structure notes were consulted as leads; their earlier heuristic and measured claims are not revalidated here. [pcc:unverified]

Replay uses multisum.rs with the same num_bigint dependency pinned in multisum-receipt.json. The native calculation implements no group solver, makes no timing comparison and changes no experiment protocol.
