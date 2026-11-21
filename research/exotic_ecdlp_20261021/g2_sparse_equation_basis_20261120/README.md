# Exact equation sparsification preserves the N31 F4 solve

Sixty-one invertible XOR row operations reduced the frozen ordinary-residual S₃ input from **3,768 to 2,936 monomial terms** (22.1% fewer) while leaving its 30 Boolean field equations unchanged. Reverse replay recovers every original equation. Fresh msolve 0.11.0 processes on the control and sparse inputs completed and produced **byte-identical reduced Gröbner bases**. Checked-Sage validation recovered the same two ordered Boolean roots and verified their factor-base point lifts, sum to the frozen residual, and relation query.

| Same residual and solver settings | Control | Sparse equation basis |
| --- | ---: | ---: |
| S₃ input terms | 3,768 | 2,936 |
| Reduced pairs | 17,735 | 17,735 |
| Rows reduced | 35,540 | 35,540 |
| Zero reductions | 8,189 | 8,189 |
| Largest logged matrix | 29,163 × 32,752 | 29,183 × 32,772 |
| Local process wall | 92.416 s | 90.633 s |
| Sampled process-tree peak RSS | 661,028,864 B | 668,778,496 B |
| Reduced basis SHA-256 | `3ddcf15591e3…` | `3ddcf15591e3…` |

The [frozen protocol](PROTOCOL.md), [builder](build.py), [61 row operations](build_result.json), and [sparse input](sparsified.ms) fix the exact transformation. The [solver receipt](run_receipt.json) binds both fresh commands, installed msolve and FLINT hashes, caps, raw logs, and basis files. The [validation](validation.json) checks source hashes, inverse row operations, both returned roots against both input systems, and curve-point reconstruction. The checked Sage runtime is saved in [sage_runtime_info.json](sage_runtime_info.json). The [amendment](AMENDMENT.md) and `startup_failure/` preserve the Python-monitoring and dyld dependency preflights before the accepted pair.

This comparison shows that reducing input equation incidence alone does not reduce msolve's recorded F4 row or pair counts on this fixed residual; its largest matrix is slightly larger. The two local wall intervals are exploratory under the CPU isolation contract, so their small difference is not a controlled speedup. The next G2 solver experiment should change the dynamic matrix work itself, such as a rank-preserving internal representation or scheduling rule, and verify the complete root and point path under matched resources.
