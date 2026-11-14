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

## Experiment: setup-optimal B for the symmetric multi-target arm (2026-11-11/12) — `[pcc:unverified]`
28 bits, T=20, 2 curves, interleaved rounds (5 rounds x 3 B values, 20 rows per B, all targets verified in both arms; `ic_multi_solve_ex` c=0.5 as the in-run noise reference):

| B | sym setup ms (table / collection) | baseline setup ms | sym/baseline setup (median of per-run ratios) |
|---|---|---|---|
| 159 | 19.5 (5.7 / 13.4) | 26.6 | 0.73 |
| 190 | 20.9 (7.8 / 12.1) | 26.3 | 0.73 |
| 230 | 22.4 (11.6 / 10.6) | 25.9 | 0.86 |

Reading: collection falls roughly as 1/B and the table build grows as B^2, so the setup is flat between B=159 and 190 and worse above; the balance point is at B ~ p^(1/3) x 0.25-0.3, as in the single-target tuning. Setup is
about 1.37x lower than the batched baseline's. A lock-step batched collector would not help: one inversion per scan is about 2% of a scan's cost (3B multiplications dominate), unlike the plain arm whose per-target step is cheap. So the symmetric
setup cost is bounded by about 0.73x of the baseline's here; no further constant is visible without changing the relation yield. Not a holdout run; no claim.

## Experiment: native x6-folded rho with short-cycle escape (2026-11-12) — `[pcc:unverified]`
`examples/prime_fast_j0_rho.rs` with `CYCFIX=1`: remember the last 8 canonical x of the walk; when the next point repeats one, leave the cycle with a different table jump
(coefficients stay exact). 60 targets per curve, 360/360 verified. Median over 60 targets; expected steps sqrt(pi n / 12).

| bits | curve | steps (expected) | folded ms (unbatched, 1 thread) | `rho_parallel` ms (negation, batched) |
|---|---|---|---|---|
| 26 | b15 / b17 | 3.7k / 3.1k (4.2k) | 2.1 / 1.9 | 2.8 / 2.9 |
| 26 | b18 / b22 (fresh) | 4.3k / 3.5k (4.2k) | 2.6 / 2.6 | 3.0 / 3.2 |
| 28 | b3 / b5 | 8.3k / 8.1k (8.4k) | 4.0 / 3.8 | 3.5 / 3.4 |
| 30 | b17 / b26 | 14.8k / 16.6k (16.8k) | 5.5 / 6.8 | 4.4 / 4.3 |
| 30 | b28 / b41 (fresh) | 15.4k / 14.0k (16.8k) | 5.9 / 5.9 | 4.4 / 4.4 |

Without the fix the same walk needed 2x-8x more steps beyond 26 bits (fruitless cycles), so the earlier "no valid folded rho" reading is superseded: the fold reaches the sqrt(pi n/12) step count. Its time is tied with
`rho_parallel` (unbatched inversion per step; batching would lower it further), so the matched rho for the symmetric IC is min(folded, `rho_parallel`): sym3 / matched rho = 3.5x-5x at 26 bits (8.9-11.6 ms against 1.9-2.6 ms),
about 21x at 30 bits (93-102 ms against 4.3-4.4 ms). Gap unchanged at 28-30 bits where `rho_parallel` is the matched reference; grows with size. No claim.

## Experiment: fingerprinted linear-probe pair table (2026-11-12) — `[pcc:unverified]`
Hypothesis: the setup collection (about 70% of the symmetric setup) is memory-bound on the pair-table probe, so a linear-probe slot table with a 32-bit fingerprint (dereference the entry only on a fingerprint match) should cut cache misses.
`LINPROBE=1` in `examples/prime_fast_j0_multi.rs`; 28 bits, B=159, T=20, 2 curves, 5 interleaved rounds (20 rows per arm), all targets verified in both arms:

| layout | table ms | collection ms | setup ms | sym/baseline setup |
|---|---|---|---|---|
| chained (`heads`/`next`) | 6.2 | 13.6 | 20.5 | 0.75 |
| linear probe + fingerprint | 6.3 | 14.0 | 20.8 | 0.74 |

No difference: the probe is not the bottleneck (the table is about 1-2 MB and cache-resident); the per-candidate field arithmetic is. The symmetric setup stays at about 0.75x of the baseline's. Setup-cost levers tried and exhausted: B retune (flat 159-190), rotated keys (no gain), batched collector (cannot help), fingerprinted layout (no gain).

## Experiment: native multi-target folded rho vs the symmetric multi-target IC (2026-11-13) — `[pcc:unverified]`
`examples/prime_fast_j0_rho_multi.rs`: multi-target distinguished-point rho on j=0 curves, x6 folding with the short-cycle escape, G-only r-adding steps (32), walks restart at a random
`aG + bQ_t` after each DP, affine union-find over the target logs (anchor Q=G, signed/affine relations b1 k_i - b2 k_j = c mod n). Unbatched (one inversion per step), one thread. Same curves and
planted targets as the symmetric multi-target IC (`prime_fast_j0_multi`: B=100, extra 8, c=0.5 baseline), 26 bits, medians; every target of every row verified (rho 18/18 rows, IC 18/18 rows).

| T | sym multi IC ms | folded multi rho ms (steps) | rho / IC |
|---|---|---|---|
| 20 | 8.7 | 9.2 (28k) | 1.06 |
| 200 | 20.2 | 47.0 (166k) | 2.3 |
| 1000 | 70.4 | 386 (1.24M) | 5.5 |

(28 bits, rho only: T=20 22 ms, T=200 103 ms; the IC arm was not re-run at 28 bits against it.)

Reading: at these toy sizes the symmetric multi-target IC is faster than an unbatched matched multi-target folded rho, with the advantage growing with T (rho steps grow about T^0.9, the IC total about T^0.4).
**Not a claim:** the rho is unbatched (about 310 ns per step at T=1000); a lock-step batched rho would be several times faster, which would shrink the ratio and could reverse it at small T. A batched
folded multi-target rho and the §5a pricing band are needed before ranking; the earlier tuned-BSGS comparison (a=-3 curves) is also not repeated on j=0. This is the first regime in the campaign where an IC arm is ahead of a rho-type arm on a measured total, and it is unverified and not yet fairly matched.

## Correction: batched multi-target folded rho (2026-11-14) — `[pcc:unverified]`
The previous section compared the symmetric multi-target IC with an **unbatched** folded rho and was flagged as not fairly matched. `examples/prime_fast_j0_rho_multi.rs` now runs the walks in lock-step (128 lanes, one batch inversion per round; the rare cycle-escape and
degenerate steps fall back to unbatched adds). Same curves and planted targets, 26 bits, medians, every target of every row verified (18/18 rho rows, 18/18 IC rows):

| T | sym multi IC ms | folded multi rho, batched ms (steps) | rho / IC | (unbatched rho, previous section) |
|---|---|---|---|---|
| 20 | 8.7 | 4.1 (35k) | 0.47 | 1.06 |
| 200 | 20.2 | 14.1 (160k) | 0.70 | 2.3 |
| 1000 | 70.4 | 80.0 (1.22M) | 1.14 | 5.5 |

Reading: the earlier "IC ahead of rho" was an artifact of the unbatched rho. Against a batched, folded multi-target rho the symmetric multi-target IC is **behind** at T=20 and T=200 (2.1x and 1.4x) and about tied at T=1000
(rho/IC 1.14, inside the host-noise scale seen in these runs). The trend is the one the exponent argument predicts (rho/IC grows with T: 0.47, 0.70, 1.14), which puts a crossover near T of a few hundred to a thousand at 26 bits;
at larger sizes it moves with the setup cost p^(2/3) against the rho's sqrt(nT). Not a claim: one host, toy size, no §5a band, no tuned BSGS on j=0, one lane count (128), and the IC arm is not tuned per T.

## Experiment: where the multi-target crossover sits at 28 and 30 bits (2026-11-14) — `[pcc:unverified]`
Same planted targets for both arms, 2 curves x 2 instances (n=4 rows per cell), all targets verified in both arms. Symmetric multi-target IC (B=159 at 28 bits, 252 at 30 bits, extra 8, not retuned per T) against the batched folded multi-target rho (128 lanes, dp bits from the same formula, 32-step table), medians:

| bits | T | IC ms | batched rho ms | rho / IC |
|---|---|---|---|---|
| 26 | 20 / 200 / 1000 | 8.7 / 20.2 / 70.4 | 4.1 / 14.1 / 80.0 | 0.47 / 0.70 / 1.14 |
| 28 | 200 | 42.8 | 24.3 | 0.57 |
| 28 | 1000 | 125.4 | 117.2 | 0.93 |
| 28 | 3000 | 357.7 | 909.1 | 2.54 |
| 30 | 200 | 95.2 | 42.9 | 0.45 |
| 30 | 1000 | 303.2 | 191.4 | 0.63 |
| 30 | 3000 | 836.6 | 1675.8 | 2.00 |

Reading: the symmetric multi-target IC loses to the batched folded rho for T up to about 1000 and is 2.0x-2.5x faster at T=3000 (28 and 30 bits); the crossover moves up slowly with size (about T=800 at 26 bits, about T=1100 at 28 bits, between 1000 and 3000 at 30 bits), consistent with the IC total growing about T^0.4 against rho about T^0.9.
Not a claim: the IC arm is not retuned per T (a larger T wants a smaller relative setup, so the IC number is probably conservative), the rho is lightly tuned, one host, no isolation, no §5a band, and the comparison to a tuned, folded **BSGS** on j=0 curves is missing: on the a=-3 curves a tuned multi-target BSGS was 2x-4x ahead of the non-symmetric IC, and a x6-folded BSGS on j=0 would gain about sqrt(6), so the decisive comparator is still BSGS, not rho.

## Correction: the multi-target rho restarted walks from solved targets (2026-11-14) — `[pcc:unverified]`
Audit trigger: the batched folded rho above needed 8x-35x more steps than Kuhn-Struik's `sqrt(2 T n / 6)`, and the excess grew with T, exactly where the IC arm "won".
Cause: `new_lane` in `examples/prime_fast_j0_rho_multi.rs` drew each new walk's start from all T targets (plus G) uniformly, solved or not. Once most logs are known, almost every walk works on a known log, and the last unknown ones are reached coupon-collector style.
Fix: new walks start only from unsolved targets (plus G as the anchor), as in Kuhn-Struik. `RESTART=all` keeps the old policy. Same curves, same planted targets (seed `0xC0DE ^ inst ^ n ^ T` in both examples), 2 curves x 2 instances, 128 lanes, same dp bits. Every target of every row verified in both policies. One local 4-core host, not isolated, medians:

| bits | T | IC ms | rho, old restarts ms (steps) | rho, Kuhn-Struik ms (steps) | K-S `sqrt(2Tn/6)` | rho/IC old | **rho/IC fixed** |
|---|---|---|---|---|---|---|---|
| 26 | 20 | 8.7 | 4.5 (35.9k) | 4.9 (29.2k) | 21.2k | 0.51 | **0.56** |
| 26 | 200 | 20.2 | 14.2 (165k) | 16.8 (102k) | 66.9k | 0.70 | **0.83** |
| 26 | 1000 | 70.4 | 100.7 (1.22M) | 27.8 (340k) | 150k | 1.43 | **0.39** |
| 28 | 200 | 42.8 | 24.9 (335k) | 16.8 (203k) | 134k | 0.58 | **0.39** |
| 28 | 1000 | 125.4 | 161.7 (2.37M) | 45.7 (680k) | 299k | 1.29 | **0.36** |
| 28 | 3000 | 357.7 | 942.5 (16.8M) | 152.4 (2.48M) | 518k | 2.63 | **0.43** |
| 30 | 200 | 95.2 | 42.0 (701k) | 27.1 (430k) | 268k | 0.44 | **0.28** |
| 30 | 1000 | 303.2 | 274.0 (4.45M) | 85.7 (1.41M) | 598k | 0.90 | **0.28** |
| 30 | 3000 | 836.6 | 1736.9 (31.8M) | 286.7 (4.96M) | 1.04M | 2.08 | **0.34** |

IC ms are the committed `symmulti_*` rows (26 bits: the logged medians above; those rows were not committed). Raw: `results/pf_j0/rhoks_*.jsonl`, `rhoall_*.jsonl`. Graph: `figures/multirho_restart_fix.{svg,png,pdf}` (`figures/plot_multirho_restart.py`). Bench record: `benchmarks/records/20261114T0627Z-j0-multirho-restart-fix.json`.

Reading: the old policy reproduces the earlier rows (942 vs 909 ms at 28/3000, 1,737 vs 1,676 ms at 30/3000), so the earlier numbers were measured correctly; the rho was weak. With Kuhn-Struik restarts **rho is faster than the symmetric multi-target IC at every cell, by 1.2x-3.6x**, and the ratio no longer moves in IC's favour with T at 28-30 bits. The "IC 2.0x-2.5x ahead at T = 3000" reading and the crossover near T ≈ 1000 are withdrawn. The fixed rho is still 1.4x-4.8x above the K-S step count (dp-bit overhead with many short walks), so this rho is itself not optimal and the gap to a tuned rho is larger, not smaller. Replication under the bench record (30 bits, T = 3000, fixed rho): median 288 ms against 287 ms, all 12,000 targets verified (`rhoks_30_T3000_rep.jsonl`). Not a claim either way: one host, no isolation, no §5a band, no folded BSGS.

## Experiment: folded multi-target BSGS on j=0 curves — the decisive comparator (2026-11-14) — `[pcc:unverified]`
`examples/prime_fast_j0_bsgs_multi.rs`: multi-target BSGS folded by the order-6 automorphism group: baby table of the canonical orbit key of jG (j=1..m, open-addressing, 8-byte key + 5-byte value), giant steps P_i = Q_t - i*S (S = (2m+1) G) batched in lock-step across all unsolved targets (one inversion per round), a hit gives k = i*stride + (mu_j / mu') j with the stored/observed orbit multipliers, verified kG = Q for every target. m = MC * sqrt(T n / 6); MC tuned at 26 bits, T=1000 on the tuning curves (0.5 / 1.0 / 2.0: 15.8 / 14.2 / 15.0 ms, flat), MC=1 used throughout. Same curves and planted targets as the IC and rho arms; n=4 rows per cell, every target verified (36/36 rows).

| bits | T | folded BSGS ms | sym multi IC ms (x BSGS) | batched folded rho ms (x BSGS) |
|---|---|---|---|---|
| 26 | 20 | 2.0 | 8.7 (4.4x) | 4.1 (2.1x) |
| 26 | 200 | 5.0 | 20.2 (4.0x) | 14.1 (2.8x) |
| 26 | 1000 | 10.9 | 70.4 (6.5x) | 80.0 (7.3x) |
| 28 | 200 | 12.5 | 42.8 (3.4x) | 24.3 (1.9x) |
| 28 | 1000 | 30.4 | 125.4 (4.1x) | 117.2 (3.8x) |
| 28 | 3000 | 60.2 | 357.7 (5.9x) | 909.1 (15.1x) |
| 30 | 200 | 27.3 | 95.2 (3.5x) | 42.9 (1.6x) |
| 30 | 1000 | 58.7 | 303.2 (5.2x) | 191.4 (3.3x) |
| 30 | 3000 | 132.8 | 836.6 (6.3x) | 1675.8 (12.6x) |

Reading: the folded multi-target BSGS is the strongest implemented multi-target arm in every cell: 3.4x-6.5x ahead of the symmetric multi-target IC, and ahead of the batched folded rho by 1.6x-15x. The IC's advantage over the rho at T=3000 does not survive the right comparator, and the IC/BSGS gap grows with T and with size (4.4x -> 6.5x at 26 bits, T=20 -> 1000; 3.4x -> 6.3x at 28-30 bits, T=200 -> 3000), as the exponents predict (BSGS total about sqrt(T n), IC setup about p^(2/3) plus a per-target cost). So there is **no regime in this campaign where an IC arm beats the strongest implemented comparator**; the verified gain remains the single-target 2.0x-2.4x over the 3-summand `ic_solve` baseline. One host, toy sizes, no isolation, no §5a band: not a claim in either direction beyond this ordering of implemented arms.

## Experiment: the single-target gain at 32 and 34 bits, fresh curves (2026-11-14) — `[pcc:unverified]`
Same code and frozen B rule as the holdout (B = round(100 (p/2^26)^(1/3)): 400 at 32 bits, 635 at 34 bits), fresh curves (idx 7 and 8: b65/b69 at 32 bits, b116/b128 at 34 bits), 4 targets per curve, extra 8, every solve verified (kG=Q) in both arms (n=4 per cell, 24 rows per bit size per arm). `ic_solve` 3-summand baseline at c in {0.5, 0.7, 1.0} x p^(1/3); medians:

| bits | curve | sym3 ms | plain3 c=0.5 | c=0.7 | c=1.0 | ratio vs best of c=0.5/0.7 | rho (negation) ms | sym3 / rho |
|---|---|---|---|---|---|---|---|---|
| 32 | b65 | 230-240 | 492 | 454 | 660 | 1.96-2.14 | 5.2 | ~44x |
| 32 | b69 | 224-246 | 519 | 500 | 578 | 2.03-2.22 | 5.4-6.7 | ~40x |
| 34 | b116 | 684-825 | 1658 | 1782 | 2289 | 2.16-2.32 | 11.8 | ~60x |
| 34 | b128 | 738-798 | 1533 | 1858 | 2407 | 2.08-2.40 | 9.3-10.3 | ~72x |

Reading: the 2.0x-2.4x improvement over the strongest implemented single-target IC (3-summand `ic_solve`, best of the c grid) holds at 32 and 34 bits on fresh curves, with B extrapolated by the frozen rule and not retuned (c=1.0 arms: 2.6x-3.4x, a worse baseline). The gap to the matched negation rho keeps growing with size (about 4x at 26 bits, 21x at 30, about 40x at 32, 60x-70x at 34), consistent with an engineering constant gain on a worse exponent. Same limits as before: one host, no isolation, wall clock, one twist group per size, no §5a band.

## Correction and improvement: the single-target scan now applies the entry's orbit shift (2026-11-14) — `[pcc:unverified]`
**Defect found.** In `examples/prime_fast_j0_sym.rs` the 3-summand scan never used the stored orbit shift of a pair-table entry (`ent.5`): `SHIFTFIX=1` stored it but the scan ignored it, so only about one third of the table hits (those whose stored point already had shift 0) could yield a relation; the group re-addition rejected the rest. This is why the earlier "shift fix" A/B (section "pair-table shift fix") saw identical target counts (671 = 671, 273 = 273): that experiment did not exercise the fix, and its conclusion ("does not change relation yield; time within noise") is **withdrawn**. The multi-target example (`prime_fast_j0_multi.rs`) did apply the shift. All numbers before this section for the single-target arm (2.0x-2.4x at 26-34 bits) therefore describe the version that discards two thirds of the table hits, and are a lower bound.

**Fix.** `sft = (sft_probe + 3 - ent.5) % 3` when `SHIFTFIX=1`. Relations are still group-verified; every solve verified kG = Q.

A/B on the tuning curves (26 bits, B=100, 4 targets x 2 curves x 2 rounds, 16 rows per arm, all verified), same planted targets, baseline `ic_solve` c=0.7:

| scan | targets needed (median) | sym3 ms | plain3 ms | ratio |
|---|---|---|---|---|
| shift ignored (old) | 274 | 8.3 | 20.1 | 2.42 |
| shift applied (`SHIFTFIX=1`) | 192 (-30%) | 6.0 (-28%) | 18.1 | 3.01 |

Fresh holdout (curves idx 9 and 10; frozen B rule B = round(100 (p/2^26)^(1/3)); `SHIFTFIX=1`; 4 targets per curve; baseline c in {0.5, 0.7}; 64 rows, all verified), medians:

| bits | B | sym3 ms | plain3 ms | ratio (range over curves, c) | sym3 / negation rho |
|---|---|---|---|---|---|
| 28 | 159 | 17.8-25.5 | 52.5-65.4 | 2.57-2.95 | 6x-8x |
| 30 | 252 | 51.2-58.2 | 143-162 | 2.59-2.86 | 12x-16x |
| 32 | 400 | 156-175 | 447-522 | 2.61-3.19 | 24x-38x |
| 34 | 635 | 469-605 | 1541-1695 | 2.80-3.30 | 50x-59x |

Reading: with the shift applied the symmetric 3-summand IC is **2.6x-3.3x faster than the strongest implemented single-target IC** (3-summand `ic_solve`, best of the c grid) at 28-34 bits on fresh curves, up from 2.0x-2.4x. The gap to the matched negation rho keeps growing with size (about 7x at 28 bits to about 55x at 34), so this remains an engineering constant on a worse exponent. Same limits: one host, wall clock, one twist group per size, B extrapolated by the frozen rule and not retuned, not yet independently replayed (the validator report covers the pre-fix code), no §5a band.

### Independent replay of the fixed code (2026-11-14) — see `results/pf_j0/VALIDATOR_REPORT_shiftfix.md`
Validated: the shift logic (zero group-check rejections with the fix, 66.8% without), the 30% target reduction (probability 19/27 explained), timing scope, 32 solves / 6088 relations by an independent pure-python checker, and the withdrawal of the earlier null A/B; 64 of 64 committed `fix_hold_*` rows reproduce bit-for-bit with `SHIFTFIX=1`. **Corrected statement of the claim: about 2.8x-3.0x faster than the strongest implemented single-target IC (3-summand `ic_solve`, best of the c grid) at 28-34 bits, individual cells 2.5x-3.6x** (the "2.6x-3.3x" above is the range of 4-row cell medians). Qualifiers carried: one twist group per size (a different prime agrees), no isolation or A/A interval, wall clock, gap to the negation rho about 5x-65x and to the x6-folded rho larger; not leaderboard-eligible. The fix is now the default (`NOSHIFTFIX=1` restores the old scan); rows record `shiftfix`.
