# Implicit Split4 pilot

The matrix-free diagonally evaluated quotient agreed with the signed group-law join on all 60 fixtures. The three modes each found 36 signed decompositions across 19 positive fixtures. Twelve smallest fixtures also passed an independently enumerated group-law check. Across all bases, 174,592 generated half roots passed both the affine group-law oracle and direct S3 evaluation. The target equal to the first right pair sum explicitly exercises an infinite complement, whose contribution is zero because the left bases have disjoint x-coordinates.

## How the operator becomes a join

The reduced split domain is the tensor product of two split squarefree coordinate rings. Its evaluation basis diagonalizes multiplication by S3(x,y,t). For distinct rational x lifts P and Q, the two zeros in t of the diagonal entry are x(P+Q) and x(P-Q). They can be generated in rows with shared batch inversion and retained as x-only keys without either the quotient matrix or a Cartesian-length evaluation vector. The right pass generates both root lifts, then computes x(R plus or minus each lift). A finite x hit counts one matching orientation of each left representative since the full curve has odd order. Key multiplicities preserve exact counts even if several assignments share an x key.

The full x map still stores the entire projected support. Every tested left pair root was distinct for both base laws and all B. The squarefree support polynomial therefore had degree exactly 2B² in every case. A representation requiring an explicit list of its coefficients or roots needs quadratic output size on this panel. This observation leaves implicit compressed formulas and non-enumerative certificates open.

## B = 128

All numbers cover both base laws and six targets each. Wall time is the median of 12 cold component calls on the local shared host. Field multiplications are means, including exponentiation for inversions. Left keys exclude O(B) batch-inversion vectors, the input bases and hash table overhead. No process RSS or calibrated cost is claimed.

| Method | Peak left keys | Mean field multiplications | Complement visits | Median wall |
|---|---:|---:|---:|---:|
| Full signed group-point join | 65,536 | 613,258 | 65,536 | 10.975 ms |
| Full matrix-free x-only join | 32,768 | 514,954 | 65,536 | 6.328 ms |
| Matrix-free x-only join, four-row blocks | 1,024 | 12,681,100 | 2,097,152 | 124.227 ms |

The full x-only form saves some y-coordinate arithmetic and halves the number of keys. Its median paired time ratio was about 1.73×. This is a useful generic constant/memory improvement for signed-pair support matching. It misses the subquadratic advancement criterion. Blocking guarantees linear entry storage but scans the right side once for every block, paying cubic work. The x-only method uses rational lifts that were common inputs to all arms. Timing excludes generating those bases and the explicit post-build correctness oracle. The calculation collects relations for fixed public residuals and recovers no logarithms. [pcc:unverified]

## Replay

```sh
g++ -std=c++20 -O3 -Wall -Wextra -Werror split4.cpp -o split4-replay
timeout 120 ./split4-replay > measurements-replay.csv
diff <(cut -d, -f1-13 measurements.csv) <(cut -d, -f1-13 measurements-replay.csv)
```

The work counts and algorithm assertions are deterministic. The final column in the CSV is elapsed nanoseconds. Modes 0, 1 and 2 are respectively the full signed group join, full x-only diagonal join and four-row blocked x-only diagonal join. Target indexes are zero-based, with the planted positive first and the infinity-boundary second.
