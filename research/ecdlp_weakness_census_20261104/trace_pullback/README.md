# trace_pullback census (GAP-PF-TRACE-PULLBACK-FACTOR-BASE)

Command (from this directory): `cargo run --release`. Rust only, no external crates, single thread, about 11 s.
Seed: `0x5eed202611040001` (xorshift64*), printed at start. Outputs: `results/trace_pullback.json`, `results/trace_pullback.csv` (one row per p, curve, V), `results/summary_table.md`.

## Conventions

- p in {103, 211, 503, 1019, 2003}, all p = 3 mod 4. F_{p^2} = F_p[i], i^2 = -1; Frobenius (a+bi)^p = a-bi.
- Two curves y^2 = x^3 + Ax + B per p with prime #E(F_p) (not equal to p), found by seeded search and counted by enumeration.
- #E(F_{p^2}) = (p+1)^2 - t^2. The enumeration of E(F_{p^2}) over all p^2 x-values (each x lies in exactly one V, except x = 0 which lies in all) was asserted equal to this for every curve.
- V_c = F_p(1 + c i) for c in F_p, and V_inf = F_p i. F'_V includes both points over each x in V (x = 0 belongs to every V). T_V = {Tr(P)} deduplicated, with Tr(P) = P + pi(P); the point at infinity (from points with x in F_p and y purely imaginary, and from P + pi(P) = O) is kept as an element of T_V.
- Decomposable fraction: share of E(F_p) equal to T1 + T2 over all ordered pairs of T_V. Because #E(F_p) is prime, E(F_p) is cyclic; each point is mapped to its discrete log to a fixed generator (table by enumeration) and the sumset is computed in Z/n over all |T_V|^2 ordered pairs. The log table was checked against the curve group law on 200 random pairs per curve. Prediction: 1 - exp(-|T_V|^2 / #E(F_p)).
- Self-tests asserted in the program: x^(p^2) = x and x^p = a - bi on random elements, inverse, sqrt (y*y = rhs), Tr(P) with zero i-component, and N*P = O on 20 random points of E(F_{p^2}) per curve.
- Square roots in F_{p^2} use the norm/closed form for p = 3 mod 4 (not Tonelli-Shanks); each root is checked by y*y = rhs.

## Summary (columns: min/median/max of |T_V| over the p+1 subspaces V)

| p | A | B | #E(F_p) | #E(F_p^2) | min T | median T | max T | argmin c | min T / #E(F_p) | decomp at min | pred at min |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 103 | 71 | 55 | 109 | 10791 | 42 | 68 | 109 | inf | 0.3853 | 1.0000 | 1.0000 |
| 103 | 83 | 56 | 97 | 10767 | 34 | 64 | 97 | inf | 0.3505 | 1.0000 | 1.0000 |
| 211 | 143 | 200 | 191 | 44503 | 69 | 127 | 191 | inf | 0.3613 | 1.0000 | 1.0000 |
| 211 | 168 | 190 | 229 | 44655 | 73 | 137 | 229 | inf | 0.3188 | 1.0000 | 1.0000 |
| 503 | 378 | 235 | 487 | 253727 | 187 | 311 | 487 | inf | 0.3840 | 1.0000 | 1.0000 |
| 503 | 494 | 387 | 487 | 253727 | 178 | 312 | 487 | inf | 0.3655 | 1.0000 | 1.0000 |
| 1019 | 510 | 513 | 1009 | 1040279 | 392 | 636 | 1009 | inf | 0.3885 | 1.0000 | 1.0000 |
| 1019 | 31 | 290 | 1021 | 1040399 | 393 | 639 | 1021 | inf | 0.3849 | 1.0000 | 1.0000 |
| 2003 | 336 | 922 | 2039 | 4014791 | 770 | 1262 | 2039 | inf | 0.3776 | 1.0000 | 1.0000 |
| 2003 | 46 | 409 | 2081 | 4010087 | 753 | 1271 | 2081 | inf | 0.3618 | 1.0000 | 1.0000 |

## What the numbers show

For every one of the 10 curves, the minimum |T_V| is 0.319 to 0.389 times #E(F_p), and it is attained at V_inf (x purely imaginary) in all 10 cases; V_inf is the only one of the p+1 subspaces with |T_V| / #E(F_p) below 0.5 for each curve (1 of p+1). [pcc:unverified]

The median |T_V| is about 0.60 to 0.67 times #E(F_p), the maximum equals #E(F_p) in every row, and V_0 (x in F_p) gives |T_V| = #E(F_p) exactly. [pcc:unverified]

At the minimising V_inf, |F'_V| is 90 to 2106 (about p) while |T_V| is 34 to 770, so |T_V| / |F'_V| is about 0.33 to 0.40. [pcc:unverified]

The decomposable fraction is 1.0000 at the minimising V and at every other V in all 10 curves, matching the prediction 1 - exp(-|T_V|^2/#E(F_p)), which rounds to 1.0000 for every row (|T_V|^2 / #E(F_p) is at least 10 in all rows). This census does not separate the generic prediction from the observed fraction because both saturate. [pcc:unverified]

The ratio min|T_V| / #E(F_p) stays between 0.32 and 0.39 across p from 103 to 2003. [pcc:unverified]
