# Lane xor-ribbon-build: static XOR / ribbon filters over pair-sum keys

All numbers below are exact counts or measurements from `raw_output.txt` / `model_output.txt`
in this directory, labelled [pcc:unverified]. Nothing here was run through ecbench, replay or
the validation gate; it is not a leaderboard row.

## Method
- `examples/ic_alt_xor_ribbon_build.rs` (self-contained, no crate imports). Curve arithmetic is
  affine mod p (u64). Factor base = smallest |F| on-curve prime abscissae (x prime, y^2=f(x) a QR),
  one point per abscissa. Keys = negation-folded pair sums x(P_i+P_j), x(P_i-P_j), i<j (2 group adds per pair),
  deduplicated (u32 keys).
- Structures: XOR filter (3 segments, 1.23n+32 slots, peeling, f = 8/16 bit); ribbon filter (w=64, sorted
  insertion, banded Gaussian elimination, back-substitution, m=(1+eps)n slots of f bits); Bloom (m=1.4427 f n bits,
  k=f, i.e. sized for FPR 2^-f); exact `std::HashMap<u32,u32>`.
- Queries = uniformly random on-curve abscissae (random x kept if f(x) is a QR), i.e. the law of a random
  walk endpoint's x, not an actual rho walk. FPR = filter hits that the exact map rejects / non-members.
  Every filter was asserted to have no false negatives on all keys.
- Curves: p24 (p=16777199,a=p-3,b=31,G=(2,5412647)) and p28 (p=268435399,a=p-3,b=3,G=(1,1) from
  specs/p28.json); G on curve asserted. Sizes: p24 |F|=128,512; p28 |F|=323,1024.
- Commands (run in /home/user/crypto):
  `cargo build --release --example ic_alt_xor_ribbon_build`
  `./target/release/examples/ic_alt_xor_ribbon_build 8000000 100 > raw_output.txt`   (8M queries, 100 construction seeds; 25 at the larger |F|)
  `python3 memory_model.py > model_output.txt`
- An earlier run (12M queries) was killed because my retry loop for ribbon eps=0.05 never succeeds at 1M keys
  (0/25 first-attempt); eps=0.05 was then dropped from the FPR table (construction table still shows it).

## Measured results (|F|, distinct keys n, measured hit rate on random on-curve x)
| curve,|F| | n distinct (emitted) | hit rate measured | model n/((p+1)/2) |
|---|---|---|---|
| p24,128 | 16232 (16256) | 1.9448e-3 | 1.935e-3 |
| p24,512 | 257607 (261632) | 3.0657e-2 | 3.071e-2 |
| p28,323 | 103984 (104006) | 7.743e-4 | 7.747e-4 |
| p28,1024 | 1043444 (1047552) | 7.745e-3 | 7.774e-3 |

Discrepancy with the incumbent: ic.pipeline p24 record reports hit_rate 1.79e-3 and scanned 242 abscissae for
|F|=128; my smallest-128 prime abscissae scanned 279 primes (largest x=1801) and hit 1.945e-3. The incumbent's
factor base is therefore not identical to mine (or it counts hits differently); I did not resolve this. Cost model
below uses my measured/model hit rate, the incumbent's numbers are quoted separately.

Bits per key (measured, identical at all four sizes to +-0.02): xor8 9.84-9.86 (1.23 f), xor16 19.68-19.71,
ribbon (1+eps)f: 8.8/9.2 (f=8), 17.6/18.4 (f=16), Bloom 11.54 (f=8) / 23.08 (f=16), std HashMap<u32,u32> 90.8-146.5
(capacity-based estimate), packed exact lower bound 28 + value bits at p28 (24 + value at p24).

False-positive rate (3 builds each, ~8M non-member queries): all of xor/ribbon/Bloom sit within 1-15% of 2^-f
(f=8: 3.87e-3 to 3.91e-3 vs 3.906e-3; f=16: 1.31e-5 to 1.74e-5 vs 1.526e-5, counts 300-390, so +-6% sampling error).
At equal FPR: xor8 uses 9.84 b/key, ribbon8 8.8 (eps=.10), Bloom 11.54: xor is 15% smaller than Bloom, ribbon 24% smaller
(f=16: 19.7 / 17.6 / 23.1).

Probe operations per negative lookup: xor 3 reads (+1 hash); ribbon 32.5 slot reads (avg popcount of the 64-bit
coefficient, within <=2 cache lines); Bloom 1.99 reads (f=8) / 2.00 (f=16), early exit; HashMap ~1 group probe.
Wall ns/lookup (shared noisy cores, diagnostic only): xor 3.6-11.5, ribbon 39.5-47.7, Bloom 25-45, HashMap 9.5-21.

Construction success (first attempt, seeds 0..N):
- xor: 100/100 at p28 n=104k, 25/25 at n=258k and 1.04M, but 95/100 at p24 |F|=128 (n=16k; mean attempts 1.040).
  So small tables need retries (about 1 in 20); retries cost nothing in group operations.
- ribbon w=64: eps=0.02 : 4/100 (n=16k), 0/100 (n=104k); eps=0.05: 78/100 (16k), 24/100 (104k), 0/25 (258k, 1.04M);
  eps=0.08: 99/100, 86/100, 18/25, 6/25; eps=0.10: 99/100, 98/100, 24/25, 21/25; eps=0.15: 100% everywhere.
  Plain (non-bumped) ribbon needs eps ~0.10-0.15 here; bits/key are therefore 8.8-9.2 at f=8, only 7-11% below xor (9.84).

## Memory-budget cost model (`memory_model.py`, `model_output.txt`) [pcc:unverified, a model not a run]
Incumbent model: setup = F^2 GAE; relations needed 0.68 F; trials = 0.68 F / h(F), h = F(F-1)/((p+1)/2)
(validated against the measured hit rates above to 0.5-4%); GAE per trial 1.0479 (p24 incremental-gauss record,
50,875 relation-phase GAE / 48,550 trials; that record's total is 67,420 GAE = 16,512 setup + 50,875 relations,
not the 56.6k in the lane brief). Structures priced (bits/key): exact HashMap 124; exact packed (ceil log2((p+1)/2) +
2 ceil log2 F + 1); ribbon f=12 + value (1.10 (f+v)), verify 1 GAE per candidate; xor f=12 + value (1.23 (f+v)), verify;
xor f=12 membership only, resolving (i,j) with F adds per candidate. Native probes priced at w GAE each.

Findings:
1. Where memory does not bind (p24, p28 at any budget >= ~1 MiB), all structures give the same F* and GAE:
   p24 F*=144, model total 6.25e4 GAE; p28 F*=365, 3.96e5 GAE. Exact table at F* needs 1.0e5 B (p24) / 7.6e5 B (p28).
   The filters cannot reduce trials or setup there: trials depend only on h(F), which depends only on F, and the setup
   F^2 adds are needed to compute every key. At w=0 the filter variants are equal (1.00x) or worse (verification
   GAE; membership-only 0.80-0.82x). At w=0.1 and 1 they are worse (ribbon 32.5 reads/lookup: 0.07x-0.41x; xor 0.56-0.92x).
   Gain at p24/p28 in counted GAE: none. Memory-only gain: a smaller table, not needed.
2. Where memory binds (budget below the F* footprint), at w=0 a retrieval filter allows a larger F and fewer trials:
   p24 32 KiB: 1.06-1.09x; p28 32 KiB: ribbon 1.18x, xor 1.13x, membership-only 1.59x; p40 1 MiB 1.22-1.29x,
   p56 16 GiB 1.26-1.34x; p83 (prime-field extrapolation only, not the N83 binary curve) 16 GiB 1.46-1.55x,
   32 KiB 1.69-1.77x. Gains scale as sqrt(bits ratio) because F ~ sqrt(M/b); they shrink to <1x once native probes are
   priced (w=0.1: xor 1.24-1.44x at p83, ribbon 0.41-0.47x; w=1: 0.74-0.86x for xor, ribbon 0.09-0.11x).
   Membership-only is fragile: its F-adds-per-candidate resolution is 0.12x at p83/16 GiB.
3. The best structure when memory binds is the xor filter with stored value (cheap 3 probes), not the ribbon (32 reads)
   and not Bloom (no value).

## Verdict
The lever reaches counted GAE at w=0 only in the memory-bound regime, via a larger |F| (fewer trials, same per-trial cost);
at p24/p28, where memory does not bind, it gives no counted-GAE gain and cannot, since the filter replaces storage, not group
operations. With native work priced (w=0.1, 1) it is a loss at p24/p28 and only xor-with-value survives at large memory-bound
sizes. Nothing here beats rho (p24 rho.negation ~5.0k GAE vs incumbent ~67k); the gap to rho is unchanged.
Limits: random on-curve x, not real walks; my factor base differs from the incumbent's (hit rate 1.94e-3 vs 1.79e-3);
the memory model is analytic; wall times are noisy.
