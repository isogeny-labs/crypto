# j=0 order-3 automorphism factor base (2026-11-11) — all `[pcc:unverified]`

Example: `examples/prime_fast_j0_sym.rs`. Curves y²=x³+b, p≡1 mod 3, prime order n (26 and 30 bits, two curves each),
single target, 1 thread, same host. Orbit factor base: B representatives, 6B covered points (3B x-values), B unknowns,
relation R = ±λ^k F_i ± λ^m F_j (2 summands). Every solve verified kG=Q.

| comparison | result |
|---|---|
| sym (B) vs plain `ic_solve` 2-summand at 3B x-values (48 solves) | sym 1.9x–2.6x faster; all verified (`run_26.jsonl`, `run_30.jsonl`) |
| sym (2-summand) vs `ic_solve` **3-summand**, c=0.5 (the strongest implemented baseline), 26 bits | 3-summand 25–31 ms vs sym 420–624 ms: **sym ~15–20x slower** (`run3_26.jsonl`; 30-bit run stopped) |
| sym vs single-target rho, negation only | rho 3–5 ms: sym 120x (26b) – 1400x+ (30b) behind |

Verdict: the symmetry gives a real constant (≈2x) inside the 2-summand family (fewer unknowns, ~3x fewer relations needed
at equal per-target cost) but 2-summand IC is far off the 3-summand baseline, so **no improvement over the strongest baseline**.
Not an improvement; the sym gain is bounded by the 3x unknown reduction already priced in.
Successor (not built): symmetric 3-summand (orbit-reduced pair table, B² entries over orbit elements) — needs lookup of
x(F_j±λ^m F_k) modulo the x-orbit; expected gain ≤ ~2x on a baseline that is already 100x+ behind rho, and rho gains √3 from
the same symmetry. Matched rho with automorphism (`aut_folded_rho.rs`) not run. Unfrozen exploratory run, no holdout protocol.

## Addendum: symmetric 3-summand (2026-11-11) — `[pcc:unverified]`

`sym_solve(summands=3)`: orbit pair table over canonical x-orbit keys of F_i ± λ^m F_j (≈3B² entries vs (3B)² for a plain
table at the same covered x-set), 3-term relations ε λ^k F_a + τ λ^s F_i ± τ λ^{s+m} F_j, rows deduped per target, every row
re-added in the group, final kG=Q check. Total time includes FB, pair table, collection, elimination, verification.

Tuning (curves idx 0,1 = b15,b17, 26 bit, not the holdout): sym3 B≈100 best (8–10 ms); strongest baseline `ic_solve` 3-summand
best at c=0.5–1.0 (21–23 ms; c=2 gives 63–85 ms). Frozen before the holdout: `results/pf_j0/frozen_holdout.json`
(B=round(100·(p/2^26)^(1/3)), prediction ≥1.5x at every size, abandon <1.2x).

Holdout (fresh curves idx 2,3, 6 targets each, 72 solves, all verified, 1 thread, same host, median ms):

| bits | curve | sym3 | plain3 c=0.5 | ratio | plain3 c=1.0 | ratio | sym3 / negation-rho |
|---|---|---|---|---|---|---|---|
| 26 | b18 | 11.6 | 25.0 | 2.14 | 28.6 | 2.74 | 4.0 |
| 26 | b22 | 8.9 | 20.8 | 2.34 | 29.8 | 2.56 | 3.2 |
| 28 | b18 | 25.7 | 55.9 | 2.18 | 90.3 | 2.50 | 7.9 |
| 28 | b22 | 35.5 | 66.4 | 1.87 | 76.4 | 2.93 | 9.9 |
| 30 | b28 | 93.5 | 195.0 | 2.09 | 225.0 | 2.46 | 19.9 |
| 30 | b41 | 101.8 | 222.8 | 2.19 | 220.6 | 2.37 | 23.1 |

Prediction met (min 1.87x ≥ 1.5x vs the better plain3 arm; abandon rule not triggered). Raw: `results/pf_j0/hold_*.jsonl`.

Reading: on j=0 curves a symmetric 3-summand factor base is ~1.9x–2.9x faster than the strongest implemented single-target
IC (`ic_solve`, 3 summands) with every cost counted, a measured improvement over the baseline, not relabelling.
Limits: j=0 curves only; toy sizes 26–30 bits; one host/thread, wall clock, tuned on 2 curves; plain3 tuned on c∈{0.5,1,2} only.
Distance to rho remains: sym3 is 3x–23x behind negation-only rho and the ratio grows with size (the exponent argument is unchanged);
an automorphism-aware rho would be a further ≈√3 faster, not run. No replay certificate, validator report, §5a band, S3, ecdb
(none possible here), so it cannot enter the leaderboard.
