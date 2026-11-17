# Rust lane: static solving degree via Fourier inversion (sd_static) [pcc:unverified]

Program: `src/main.rs` (no crates, edition 2021, built `cargo build --release -j 2`, build OK, no warnings of note).
Computes, mod p < 2^31 with 2^k | p-1, w a random element of exact order 2^k (printed as `w`),
g(x) = S_3(x1,x2,t) (m=2) or Res_X(S_3(a,b,X), S_3(c,t,X)) (m=3) on H^m, unit test (no zero on H^m),
g^{-1} by pointwise inversion and exact inverse DFT (radix-2 FFT, root w^{-1}, scale N^{-m}),
maxwt = max over supp g^{-1} of sum of popcounts of the exponent coordinates, delta = 2 (m=2) or weight of g
(inverse transform of g's values; m=2 also reported as `gweight` for k <= 10, always 2), sd_static = delta + maxwt.
Labelling: coefficients are those of g^{-1} as a polynomial in x_i (x_i^N = 1), evaluated at w^j with the same w in the twiddles, so they are independent of w (checked by `--selftest-w`, below); w is random per instance, recorded only for reproducibility.

## Self-test (`sd_static --selftest`, all pass) [pcc:unverified]
* S_3 vanishes at x(P1+P2) and x(P1-P2) for random points: 4000/4000 (p=12289), 3938/3938 (p=97); no failures.
* S_4 (resultant formula) vanishes at x(P1+P2+P3) with the chain (x1,x2,x3,xQ): 2000/2000 and 1953/1953; no failures.
* FFT equals the naive O(N^2) DFT (n=16).
* k=2: g * g^{-1} = 1 in K[(Z/4)^m] by direct convolution: m=2 p=17 10/10, p=12289 20/20; m=3 p=17 11/11, p=12289 20/20.
* k=1 reproduction: p=101 a=17 b=72 t=97 (w=100): unit, g^{-1} coefficients (0,0)=91, (0,1)=81, (1,0)=81, (1,1)=95,
  maxwt=2, sd_static=4. Exactly the main session's values.

## w-independence and labelling test (`sd_static --selftest-w`, added on coordinator request) [pcc:unverified]
For 5 random unit instances each at p=12289 (m=2: k=3, k=5; m=3: k=2, k=3), the full coefficient arrays of g^{-1} and of g,
and maxwt, computed with w and with w^s (s random odd) are identical: 5/5 in every case. The k=2 direct-convolution test in
`--selftest` (exponent addition mod N, sum_e c_e x^e times g = 1) also passes (see above). No labelling bug found; no rerun
needed, so all numbers below are unchanged: the p=17, k=3 exception count (111/1978 in the hunt, 112/1983 pooled) and the
100% fractions stand. An earlier caveat in this lane that maxwt depends on w was wrong and is withdrawn.

## Main preregistered runs (commands in `run_all.sh`; data `data/m2_p12289.jsonl`, `data/m2_pmin.jsonl`, `data/m3_p12289.jsonl`)

Target: sd = 2k+2 (m=2), delta+3k (m=3). Counts are unit instances out of generated (20 or 50). Instances with a zero on H^m
are "proper" (not unit) and skipped. Unit instances become rare once N^m >> p (probability about exp(-N^m/p)); that is
why the requested p=12289 sweep has no unit instance for k >= 9 (k=9..12: 0/20, N^2 = 262144..16.8M > p).

| run | k | unit/gen | sd = target | other sd |
|---|---|---|---|---|
| m=2 p=12289 | 3 | 19/20 | 19 | none |
| | 4 | 20/20 | 20 | none |
| | 5 | 19/20 | 19 | none |
| | 6 | 18/20 | 18 | none |
| | 7 | 10/20 | 10 | none |
| | 8 | 2/20 | 2 | none |
| | 9-12 | 0/20 each | n/a | n/a |
| m=2 p=min (17,17,97,193,257,257) | 3 (p=17) | 5/50 | 4 | 1 with sd=7 (top coeff zero) |
| | 4 (p=17), 5 (97), 6 (193), 7 (257), 8 (257) | 0/50 each | n/a | n/a |
| m=3 p=12289 | 2 | 20/20 | 20 (sd=12) | none |
| | 3 | 20/20 | 20 (sd=15) | none |
| | 4 | 19/20 | 19 (sd=18) | none |
| | 5 | 11/20 | 11 (sd=21) | none |
| | 6 | 0/20 | n/a | n/a |

delta = 6 on every unit m=3 instance (also the t with 5t^3+3at+6b=0 case was not specifically probed). delta = 2 for m=2 (gweight 2).

## Supplementary runs with larger p (to get unit instances at large k; `run_supp.sh`, seeds 7000+...)

p = 1073750017 (of the form c*2^12+1, so 2^12 | p-1), 20 instances each:
* m=2, k=3..12: unit 20/20 for k=3..11, 19/20 at k=12; every unit instance has sd = 2k+2 (8,10,...,26). No exception.
* m=3, k=2..6: unit 20/20 each; every one has delta=6 and sd = 6+3k (12,15,18,21,24). No exception.

p = 1073153 (= 262*4096+1), m=2, 20 each: k=3..7 unit 20/20; k=8 18, k=9 19, k=10 12 unit; k=12 0/20 (k=11 not run);
all unit instances sd = 2k+2.

p = 257, m=2, 200 each: unit counts k=3: 174, k=4: 124, k=5: 27, k=6: 1, k=7,8: 0; all unit have sd = 2k+2 (top coeff nonzero).

p = 17, k=3, 20000 instances (exception hunt): 1978 unit; 1867 with sd=8 = 2k+2, 111 with sd=7 = 2k+1 (5.6%, about 1/p = 5.9%);
in all 111 the top coefficient of g^{-1} is zero. No instance had sd < 7. (Deltas all 2.)

## Per-k fraction having sd = target, all runs pooled (unit instances) [pcc:unverified]

m=2: k=3: p=12289 19/19, p=1073750017 20/20, p=1073153 20/20, p=257 174/174, p=17 (20000-run + 50-run) 1871/1983
(0.944; exceptions all sd = 2k+1); k=4..11: 100% of all unit instances found (no exceptions) at every prime tried
(counts above); k=12: 19/19 at p=1073750017. m=3: k=2..5 (p=12289: 20,20,19,11 of 20,20,19,11 unit), k=2..6 at p=1073750017: 100%.
Exceptions with sd < 2k+2 found: only p=17, k=3 (111 of 1978 unit instances, sd = 7). Fractions with sd = 2k+2 are >= 1 - 3/p
at every (p, k) tested where p >= 257 (observed 100%); at p=17 the observed exception rate 0.056 is below 3/p = 0.176.
This is consistent with P2 and P4 (not falsified in this lane), but p=12289 yields no unit k >= 9 data; the large-p supplement
covers k=9..12. The values were not compared with the Python lane or a brute-force Macaulay computation here
(that is P1/lane A); the group-algebra convolution check at k=2 and the group-law checks are the only internal validation.

## Wall times (single thread, this host)
Main runs (`data/wall_times.txt`, each count=20 or 50 batch): m=2 p=12289: k=3..9 < 0.2 s, k=10 0.74 s, k=11 2.96 s, k=12 12.4 s.
m=2 pmin: all < 0.12 s. m=3 p=12289: k=6 0.13 s. Supplement (`data/wall_times_supp.txt`): p=1073750017 m=2 k=10 3.7 s, k=11 12.4 s,
k=12 55.3 s (20 instances each); whole `run_supp.sh` 100 s; `run_all.sh` 17 s.

## Commands
Build: `cd rust_check && cargo build --release -j 2`.
Self-test: `./target/release/sd_static --selftest`.
Batch: `./target/release/sd_static --m 2 --k K --p P --count C --seed S` (seed used = 1000*m+k in run_all.sh, 7000+1000*m+k in run_supp.sh);
instance: `./target/release/sd_static --m 2 --k K --p P --a A --b B --t T [--w W]`.
Run all: `bash run_all.sh; bash run_supp.sh`. Cross-check instances: `data/CHECK_LIST.md`.
Output fields: p,k,m,w,a,b,t,unit,maxwt,delta,gweight,sd_static,top_coeff_zero,zero_count_by_weight,(ginv_coeffs if N^m<=16),ms.
