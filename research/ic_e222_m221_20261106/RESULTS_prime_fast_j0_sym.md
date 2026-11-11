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

## Correction after independent validation (2026-11-11)
See `results/pf_j0/VALIDATOR_REPORT.md`. The "1.87x-2.93x" range above mixed pairings of runs; the supported statement is **about 2.0x-2.4x** faster than the best 3-summand `ic_solve` arm (c grid optimum ~0.7, not covered by the frozen {0.5,1.0}), and the holdout curves at one size form one twist group; fresh primes give 2.1x-2.6x. 490/490 independent certificates pass. Gap to negation-only rho grows 3x -> 23x from 26 to 30 bits. `[pcc:unverified]`.

## Experiment: store the entry's x-shift in the orbit pair table (2026-11-11) — `[pcc:unverified]`
Validator observation: 2/3 of pair-table hits fail the group check because the entry's own x-shift was not stored. `SHIFTFIX=1`
stores it (`sft = s - s_e`). Tuning curves only (idx 0,1, 26 bit, 4 targets, baseline `ic_solve` c=0.7, 48 solves all verified):

| B | sym3 no-fix ms | sym3 shift-fix ms | plain3 c=0.7 ms | ratio fix | relations-yield (targets used) |
|---|---|---|---|---|---|
| 68 | 15.0 | 9.0 | 16.4-26.3 | 1.82 | 671 = 671 |
| 100 | 12.7 | 8.6 | 18.3-25.2 | 2.11 | 273 = 273 |
| 150 | – | 9.6 | 17.7 | 1.84 | 119 |
| 40 | – | 16.8 | 17.4 | 1.03 | 2415 |

Result: the number of targets needed is identical with and without the fix (the three rotated entries already cover every orbit pair), so the fix does not
raise relation yield; it only avoids the failing group re-additions (about 1.4x lower sym3 time at B=68/100 in separate runs on the same host; baseline medians differed 16-26 ms between the two
runs, so host noise is of the same size; not claimed). Not a holdout run; no new claim. The headline stays at about 2.0x-2.4x.

## Experiment: multi-target symmetric factor base (2026-11-11) — `[pcc:unverified]`
`examples/prime_fast_j0_multi.rs`: orbit factor base (B=100), setup relations against G once (Gauss-Jordan, log determined when its pivot row is otherwise zero), then one
verified 2/3-term relation per target (unbatched scan); baseline `ic_multi_solve_ex` (table + batched collection, c=0.5, the strongest multi-target IC). Tuning
curves idx 0,1 (26 bit, two curves), all targets verified against the planted k in both arms (every row `sym_ok == base_ok == T`).

| T | rows | sym total ms | baseline total ms | baseline/sym | setup sym / base ms |
|---|---|---|---|---|---|
| 20 | 6 | 9 | 14 | 1.63 | 7.4 / 11.6 |
| 200 | 4 | 22 | 26 | 1.18 | 7.2 / 10.4 |
| 1000 | 6 | 74 | 76 | 1.03 | 6.7 / 9.5 |

Reading: the symmetry cuts the setup by only about 1.4x (not 3x: the orbit pair table, scans and the O(B³) elimination are not all reduced), and the per-target phase, which dominates for T>=200, is
unchanged (same covered x-set, same yield) and my per-target scan is unbatched while the baseline's is lock-step batched. The advantage vanishes with T. Not a holdout run and not a claim; no j=0
BSGS/rho multi-target comparator was built, so no ranking against them (the a=-3 results, 2x-4x behind tuned BSGS, are unchanged). The single-target result (2.0x-2.4x) stays the only improvement.

## Experiment: native automorphism-folded rho on j=0 curves (2026-11-11) — `[pcc:unverified]`
`examples/prime_fast_j0_rho.rs`: single-target r-adding rho (32 steps) on canonical x6 orbit representatives (negation + order-3 rotation, coefficients
tracked through the automorphism), DP restart, walk length cap 40·2^d, unbatched, one thread; 40 targets per curve, all verified (240/240).
Median ms (steps) vs `rho_parallel` (negation, batched, same host):

| bits | curve | folded ms (steps) | rho_parallel ms |
|---|---|---|---|
| 26 | b15 / b17 | 2.8 (8.3k) / 3.3 (9.9k) | 3.3 / 3.4 |
| 26 | b18 / b22 (fresh) | 3.5 (10.0k) / 2.3 (8.5k) | 3.0 / 2.9 |
| 28 | b3 / b5 | 7.0 (32.8k) / 6.7 (35.0k) | 3.4 / 3.6 |
| 30 | b17 / b26 | 19.5 (127k) / 17.2 (98k) | 4.3 / 4.5 |

Reading: the naive folded walk does not realise the expected sqrt(3) (steps blow up beyond 26 bits: fruitless short cycles and the walk cap, the known folding problem, not handled);
at 26 bits it ties `rho_parallel`. So no valid automorphism-aware matched rho exists in this repository's native engine yet; the matched reference stays `rho_parallel`
(sym3 3x-23x behind it). An untuned folded rho is not evidence about a tuned one; building the tuned one (cycle detection, batched steps) is the open comparator task. No claim.

## Experiment: where the symmetric multi-target setup cost goes (2026-11-11) — `[pcc:unverified]`
Phase timing added to `examples/prime_fast_j0_multi.rs` (28 bit, B=159, 20 targets, 2 curves, all targets verified): setup 23-28 ms = orbit pair table 6-7 ms + relation collection 16-22 ms + Gauss-Jordan 0.5 ms; `ic_multi_solve_ex` setup (batched, bucketed) 31-35 ms. The setup is collection-dominated (about 400 unbatched scans of 3B candidates), not elimination.
Tried: exact-x keys for all three rotations of each entry (`ROTKEYS=1`, no canonicalisation at probe): table build +30%, collection unchanged within noise; sym/base setup ratio 0.76 (canonical) vs 0.92 (rot keys), 24 runs each: no gain. A B sweep (100,159,230,330) was inconclusive: the same baseline config varied 25-45 ms setup between runs (host noise ~1.8x), so no setup-optimal B is claimed.
Lever left: a lock-step batched per-scan collector for the symmetric arm (as `collect_setup_batched` does for the plain arm) needs a quiet host to measure; nothing measured here beats noise.
