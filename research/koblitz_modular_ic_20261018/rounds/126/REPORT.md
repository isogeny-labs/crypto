# R126 — N51 on the four x=0 tame-seven places

R126 lifts the four tame-seven G51 roots over `s=t^7` and certifies the first nonzero N51 coefficient. The normalized N51 orders are:

| Base place | Seven-edge q | Candidate order after R125 cancellation | First nonzero t-offset | Exact N51 order | First coefficient at F4 roots 2,3 |
|---|---:|---:|---:|---:|---|
| y0/e1 | 2 | -210 | 69 | **-141** | 1,1 |
| y0/e3 | 2 | -147 | 69 | **-78** | 1,1 |
| y1/e1 | 23 | 0 | 42 | **42** | 2,3 |
| y1/e2 | 9 | 0 | 49 | **49** | 2,3 |

Each pair of F4 roots represents one residue-degree-2 place and gives the same order. R125's four simple-place orders are -30, -21, 0, 0 in the same base-place order. Thus the complete x=0 local contribution to `div(N51)` on the R123 normalization is `2(-30-21-141-78+42+49)=-358`. This is local divisor data; the other finite supports and infinity are separate.

The calculation uses the SHA-pinned R119–R125 results and R125 source. It substitutes `W=t^q(1+U)`, solves the simple-root equation for `U` modulo `t^256`, checks the G51 residual vanishes, reconstructs all 15 branch-specific N51 coefficient rows and matches their R125 support hashes, then evaluates N51. The smallest G51 input cutoff is 1699 and the smallest N51 cutoff is 2380, both well beyond the first nonzero offsets. The independent verifier uses an F4 multiplication table, direct binomial translation and coefficient-by-coefficient Hensel lifting. All eight orders match. Lean 4.34.1 checks the finite G51 zero and N51 zero-prefix/first-coefficient equalities for each residue root and the degree-weighted sum. Byte replay matched `result.json`; a changed R119 input was rejected before any output.

Source/protocol commit: `a56e7b28ca9ededc8d7e62be2e4955b29a929445`. Result SHA-256: `40cb7e162ee63d0ff61c3774e2a1c1e0893b7a61e7b18d4a4fd32e3437318672`. Commands: `python3 rounds/126/lift126.py rounds/126`, `python3 rounds/126/verify126.py`, `python3 rounds/126/make_lean126.py`, and `lean rounds/126/lean/N51Valuations126.lean` from the study root's parent. Local exact computation; no cloud cost.

The repository-wide `cargo test --release --lib` attempt stopped at missing tracked `include_str!` fixtures in this sparse checkout before reaching tests. The touched code is Python research code, and its source-bound, independent and Lean checks above passed.

The next geometric step is to combine this x=0 divisor data with the x=1, degree-7/11/22 and infinite normalized places, then compute full divisors and a complete prescribed Riemann–Roch space for the level-77 map. The finite certificate here proves the stated local orders under the pinned equations and cutoffs; it does not by itself construct the proper modular-factor transfer.
