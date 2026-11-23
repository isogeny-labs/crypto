# Round 7, tests 2 and 3: kill tests from `../round7_beyond/DIRECTIONS.md`

Class: closure experiments, no S value, no rho comparison, no leaderboard row. Rule: no Python for computation (gp only). Budget per test: 1 core, `ulimit -v 524288`, 30 min.

## Pre-registration (written before any run of the tests)

Registered at 2026-11-04T05:57:08Z (`date -u`).

### Test 2 (rank 2, covers). Construction chosen (DIRECTIONS leaves it open)
- E: y^2 = f(x), f monic cubic over F_p. C: y^2 = F(t) = f(t^2+s), genus 2, F monic sextic, both points at infinity (inf+, inf-) rational. pi: C -> E, (t,y) -> (t^2+s, y), degree 2 (the `enge_primefield_cover` construction named in DIRECTIONS). Jac(C) ~ E x E' with E': v^2 = w f(w+s).
- Factor base on C: all affine F_p-points. B_s = pi(C(F_p)) = affine E-points with x-s a square (or 0).
- Part 2a (exhaustive degree 2): all unordered pairs {P1,P2} of affine C(F_p)-points with P2 != iota(P1) = all split reduced degree-2 Mumford divisors. Project to {pi P1, pi P2}. Direct elliptic set S_E = all unordered pairs {Q1,Q2} in B_s x B_s (equivalently lines through -(Q1+Q2) in L(3O) with both other zeros in B_s). Compare S_C = pi(split divisors) with S_E; split rate = (#split classes)/#Jac(C)(F_p) against 1/2 and a random-sextic control (>= 20 curves).
- Part 2b (sampled, DIRECTIONS' "h with deg a<=3, deg b<=1"): h = a(t)+b(t)y, zeros are the roots of N(t)=a^2-F b^2 (degree 8); fully split = N squarefree and splits over F_p. Also the k=6 family h = a(t)+c y (deg a<=3, c const). Arms: cover; random genus-2 control y^2=g(t), g random monic sextic, same function family; direct E functions A+Cy in L(kO) (k=6) with zeros all in B_s and all in E(F_p).
- Prediction (derived from DIRECTIONS): every fully split projected relation is a zero set of an elliptic function A+Cy (norm inclusion), S_C = S_E up to explained edge cases, and the split rate per sample is within 2x of |B_s|^k/(k! p^k) = 1/(2^k k!).
- Kill criterion (DIRECTIONS rank 2, both must hold for "closed"): (K2a) every fully split projected relation lies in the direct elliptic set; (K2b) the split rate per sample is within 2x of 1/(2^k k!). If both hold the direction is closed. If K2a holds and K2b fails, the cover is a constant-factor sampling effect only, to be reported as such.
- My own side prediction, stated so it can fail: a cover sample is a divisor on C, so its split rate is about 1/k! (all C(F_p) is the base), i.e. 2^k above the K2b formula, and the cover family is a p-times smaller subfamily of the elliptic one. [pcc:unverified]

### Test 3 (rank 3, list decoding). Counted quantity
- Curve of round 1: p=786433, y^2 = x^3 + 779083 x + 163026, g = 780070. d=2^t, c=g^d, V = {x : x^d = c} = g*mu_d (Kummer set), t = 6..9. m=3 (h=2): L(4O) = <1,x,x^2,y>.
- Counted quantity N(S,R): number of functions phi in P(L(4O)) with phi(-R)=0 whose three other zeros are all F_p-points with x in S. Equivalently the number of unordered triples {Q1,Q2,Q3} of E(F_p)-points with x(Q_i) in S and Q1+Q2+Q3 = R (multiset, all points of E(F_p)). Computed exactly by pair-sum table + lookup. Target R: T random points of E(F_p) (T=256), counts summed; also totals sum_R N(S,R) = C(n_S,3) exactly over the group, so expectation per R is C(n_S,3)/#E.
- Controls: >= 20 random x-sets of the same size d (uniform in F_p^*, same procedure, same targets), their counts normalised by C(n_S,3)/#E; Poisson band on the total; empirical band from the 20 controls. Positive control: an arithmetic progression {iG} of the same size (must be far above the band). 
- Prediction: Kummer count within the Poisson 95% band (and the controls' empirical range) of the random expectation at every t. Johnson check (derived): agreement 3 << sqrt(d*(2h-1)) for all t, tabulated.
- Kill criterion (DIRECTIONS rank 3): Kummer agreement-m list size matches the random set within the Poisson 95% band at every t. The Popov-degree half of DIRECTIONS' criterion is NOT run here (no interpolation-module code); stated as not covered.

## Files and exact commands
Scripts: `t2a.gp`, `run2a.sh` (exhaustive degree 2), `t2b.gp`, `run2b.sh` (sampled), `t3.gp`, `run3.sh`. Logs in `logs/`. Every run: `ulimit -v 524288`, one gp process, gp 2.15.4, no Python. Scratch dir `/tmp/claude-0/-home-user/8900182d-dd1d-52b2-9106-78048929cee5/scratchpad/t23/` unused for data.
```
./run2a.sh P SEED EXH NCTL      # runs used: 101 seeds 1..6 (EXH=1); 1009 seeds 11,12; 3001 seed 21; 1009 seed 31 and 3001 seed 32 with NCTL=20 (EXH=0)
./run2b.sh P SEED K NS NB VER   # 1009 101 6 250000 20 300 ; 10007 102 6 ... ; 101 103 6 ... ; 1009 201 8 750000 20 300
./run3.sh "6" 7 20 256 ; ./run3.sh "7,8,9" 7 20 256      # t list, seed, controls, targets
```
Each 2b block draws a fresh cover curve (random a,b,s) and a fresh random monic squarefree sextic; the 2b "verified" column is, per hit (first 300), a check that the projected points are zeros of A+Cy computed from the norm formula (A+Cy = Norm h written in x=t^2+s), lie on E, and sum to O.

## Test 2 results

### 2a exhaustive degree-2 divisors (log `logs/t2a.log`)
Split classes = unordered pairs of affine C(F_p)-points with P2 != iota(P1); #Jac from `hyperellcharpoly`, cross-checked as #E * #E' (E': v^2 = w f(w+s)); the cross-check and the pair-count formula both matched in all 9 exhaustive runs.

| p | seed | #C(F_p) | split classes | #Jac | rate | \|B_s\| | \|S_E\| | \|S_C\| | \|S_E \ S_C\| |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 101 | 1 | 90 | 4050 | 9360 | 0.433 | 46 | 1081 | 1080 | 1 |
| 101 | 2 | 132 | 8710 | 13920 | 0.626 | 66 | 2211 | 2211 | 0 |
| 101 | 3 | 116 | 6727 | 12084 | 0.557 | 59 | 1770 | 1769 | 1 |
| 101 | 4 | 104 | 5408 | 10791 | 0.501 | 52 | 1378 | 1378 | 0 |
| 101 | 5 | 78 | 3042 | 8277 | 0.368 | 40 | 820 | 819 | 1 |
| 101 | 6 | 96 | 4605 | 9936 | 0.463 | 49 | 1225 | 1224 | 1 |
| 1009 | 11 | 968 | 468511 | 980000 | 0.478 | 485 | 117855 | 117854 | 1 |
| 1009 | 12 | 1040 | 540800 | 1052387 | 0.514 | 520 | 135460 | 135460 | 0 |
| 3001 | 21 | 3064 | 4694048 | 9204756 | 0.510 | 1532 | 1174278 | 1174278 | 0 |

S_C = set of unordered pairs {pi P1, pi P2} from split divisors; S_E = all unordered pairs in B_s x B_s (= lines through -(Q1+Q2) with both other zeros in B_s). S_C is contained in S_E by construction. Every unreached element of S_E (6 in total) is a pair {Q,-Q}, the only lift of which is {P, iota P} (the trivial class), consistent with Q having x=s (single preimage). I did not check the x=s condition individually. [pcc:unverified]

Split-rate bands, 20 curves each (rate = split classes / #Jac): p=1009 cover mean 0.485 (sd 0.022, range 0.445..0.529), random sextic 0.497 (sd 0.015, 0.459..0.522); p=3001 cover 0.500 (sd 0.0125, 0.471..0.528), random sextic 0.497 (sd 0.0054, 0.483..0.504). The prediction 1/g! = 1/2 holds for both; the K2b formula for k=2 is 1/(2^2 2!) = 1/8, a factor 4 below. [pcc:unverified]

### 2b sampled functions (logs `logs/t2b_k6.log`, `logs/t2b_k8.log`)
Counts of fully split samples (N squarefree, all roots in F_p). Expected counts use 1/k! and 1/(2^k k!). 20 independent (curve, control) blocks per row, pooled.

| k | p | samples | cover | random-sextic control | E-direct, any E-base | E-direct, B_s only | exp. 1/k! | exp. 1/(2^k k!) |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 6 | 101 | 5e6 | 7942 | 6827 | 4965 | 78 | 6944 | 108 |
| 6 | 1009 | 5e6 | 7442 | 6844 | 6968 | 115 | 6944 | 108 |
| 6 | 10007 | 5e6 | 7015 | 7103 | 6958 | 116 | 6944 | 108 |
| 8 | 1009 | 1.5e7 | 362 | 361 | not sampled | not sampled | 372 | 1.45 |

Poisson sd at 6944 is 83, at 372 is 19. At p=10007 the cover and the controls sit within 1 sd of 1/k!; at p=1009 and p=101 the cover (and at p=101 the E-direct "any base" arm) deviate by several sd, which I read as 1/p-size effects (repeated roots, points at infinity, 2-torsion), not tested further. Ratio cover / E-direct-B_s: 64.7 (p=1009), 60.5 (p=10007), 101.8 (p=101); formula 2^6 = 64. At k=8 the cover rate is 2.41e-5, 249 times the K2b formula 9.69e-8. All 1200 verified hits (300 per run, 4 runs) passed the norm-inclusion check. [pcc:unverified]

### Reading against the kill criterion
- K2a (every fully split projected relation is an elliptic zero set, S_C inside S_E): holds in all runs: 2a inclusion with 6 explained edge cases in 9 curves, 2b 1200/1200 verified. [pcc:unverified]
- K2b (split rate per sample within 2x of |B_s|^k/(k! p^k)): FAILS. Cover rate is about 1/k!, i.e. 2^k times (64x at k=6, about 250x at k=8, 4x at k=2) above the formula, and equal to the random-sextic control and to the E-direct sampler over the full base E(F_p). The formula compares the cover with an elliptic sampler restricted to the same half base B_s, and against that arm the cover is 2^k more efficient per sample. [pcc:unverified]
- So by DIRECTIONS' own rule the direction is not closed by both conditions; what is measured is a per-sample constant 2^k gain on the half base B_s, with no new relation family. The cover family is a p-times smaller subfamily of L(kO) (about p^4 against p^5 functions at k=6). This test does not price the gain (no matched rho, no S value, no total cost per relation, no linear-algebra cost, and it was not checked whether the cover relations still generate the DL-relevant lattice mod the point order), so it is not an improvement claim. [pcc:unverified]
- Not run: k=8 E-direct arms (expected 1.45 hits in 1.5e7 samples), p > 10007, the exhaustive (a deg<=3, b deg<=1) enumeration (about p^5 functions).

## Test 3 results (logs `logs/t3_t6.log`, `logs/t3_t789.log`; seed 7, 256 random targets R, 20 random controls)
N(S,R) summed over the 256 targets. n = number of E(F_p) points with x in S (#E(F_p) = 786627). Expected total = 256 C(n,3)/#E. Poisson95 is the exact central 95% Poisson interval for that expectation. Controls: random x-sets of size d in F_p^*, same targets; "ratio" = count / own expectation.

| t | d | n_V | expected | Poisson95 | Kummer | Kummer/exp | controls count mean (min..max) | controls ratio mean (sd) | Kummer ratio rank among 21 (0=lowest) | controls outside own Poisson95 |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 6 | 64 | 54 | 8.07 | 3..14 | 8 | 0.991 | 14.5 (5..33) | 1.058 (0.278) | 7 | 1 of 20 |
| 7 | 128 | 132 | 121.9 | 101..144 | 140 | 1.148 | 109.3 (74..161) | 1.025 (0.202) | 19 | 1 of 20 |
| 8 | 256 | 252 | 857.7 | 801..916 | 860 | 1.003 | 851.2 (598..1067) | 1.010 (0.052) | 12 | 3 of 20 |
| 9 | 512 | 516 | 7408.7 | 7240..7578 | 7386 | 0.997 | 7070 (5861..9466) | 1.001 (0.018) | 9 | 2 of 20 |

Positive control (arithmetic progression x-set {x(iG)}, i<=d/2, targets sG with |s|<=d/2): counts 110471, 451104, 1841892, 7346247 against random expectations 13.6, 111, 899, 7237 for t=6..9; the counting detects additive structure, about 8000x at t=6 and about 1000x at t=9. The control counts are wider than Poisson (about 2 to 3 of 20 outside their own Poisson95, from variation of n and the shared targets), so the empirical ratio band (sd 0.28 at t=6 down to 0.018 at t=9) is the one to read. Johnson: agreement needed is 3 while sqrt(3d) = 13.9, 19.6, 27.7, 39.2 for t=6..9, so the target lies far beyond the Johnson radius. [pcc:unverified]

Reading against the kill criterion: the Kummer count lies inside the Poisson 95% band at t=6,7,8,9 (8 in 3..14, 140 in 101..144, 860 in 801..916, 7386 in 7240..7578) and inside the empirical control range at every t (ratio ranks 7, 19, 12, 9 of 21; the t=7 value 1.148 is the 19th of 21, within 1 sd of the control sd 0.20). The first condition of the kill criterion is met; the Popov-degree condition was not run. So the Kummer list size shows no excess over a random evaluation set at m=3 for t<=9 at this power (about 1.5 to 2% sd at t=9). Single seed, one curve, one set of targets. [pcc:unverified]
