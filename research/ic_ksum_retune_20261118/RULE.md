# Frozen sizing rule (committed before any pilot run) [pcc:unverified]

`rule.py` picks (k, N) for the `ksum:m=<k>,prune=1,split=1` + `targets=stride-renew` arm from r and the field family alone:
minimise `C0(N, k; r) = build(N, k) + (N + 0.6) r / (2 T)`, `T = 2^(k-1) C(N, k)`, over k = 3..7. The constants (hit rate
2T/r, relations N + 0.6, build adds) are fitted on the committed split study's `-psr` arms (16 arm-rows); `build(N, k)`
reproduces B24 ksum5-psr's 2,216 set-up adds exactly. No target or seed enters the rule.

Picks on Table B (incumbent = the split study's cheaper `-psr` arm at w = 0, producer N):

| row | log2 r | incumbent | rule | model S0 incumbent -> rule |
|:--|--:|:--|:--|:--|
| P20 | 20.00 | k5 N9 | k5 N9 (same) | 4.60 -> 4.60 |
| P22 | 21.65 | k5 N10 | k5 N10 (same) | 4.78 -> 4.78 |
| P24 | 23.38 | k5 N11 | **k6 N10** | 4.97 -> 4.87 |
| B15 | 13.99 | k4 N7 | k4 N7 (same) | 4.24 -> 4.24 |
| B18 | 15.00 | k4 N7 | k4 N7 (same) | 4.22 -> 4.22 |
| B21 | 20.00 | k5 N9 | k5 N9 (same) | 4.60 -> 4.60 |
| B24 | 21.00 | k5 N9 | **k6 N9** | 4.98 -> 4.72 |
| B27 | 24.41 | k5 N12 | **k7 N10** | 5.16 -> 5.16 (tie) |

Model S0 excludes the walk set-up (6 scalar multiplications) and factor base, which do not depend on (k, N).
Next: a pilot on fresh seeds (PILOT section of PROTOCOL.md) checks the model's ranking around the picks; the confirmatory
study is frozen only after it, on different seeds.

**Revised after the pilot (R1, PROTOCOL.md section 2):** the objective is C at w = 0.1; picks change on P24 (6, 10), B18 (5, 7), B24 (6, 9); B27 stays (5, 12).
