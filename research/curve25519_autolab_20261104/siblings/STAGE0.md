# Sibling points, Stage 0 results

Status: [pcc:unverified]. No pcc claim id. Zero group operations: scalar Monte Carlo only, no ecbench run, no cost
model (the 0.59-of-a-step pricing of a sibling child is not used here) [pcc:unverified].

Question [pcc:unverified]: in a stored-table walk where every parent `P_t` emits its 2m keyed children `P_t +- J_h`
(h = 1..m) and the walk continues from the child `P_t + J_{h_t}`, is the collision count among stored points the
same Poisson law as for independent random points, so that E in stored points per sqrt(r) equals the D0 value?

Answer [pcc:unverified]: no. The pre-registered falsification line is hit at every m <= 8, for C = 1 and C = 2, at all
three sizes. E_points is above 1.08 x D0 by a wide margin (m = 8: 1.861 to 1.914 x D0; m = 16 at k = 20:
1.563 x D0 at C = 1, 1.561 x D0 at C = 2). The prediction E_points = D0 x (1.00 +- 0.03) is falsified.
The sibling set loses informative pairs. Every E_points below is above D0 [pcc:unverified].

## Tool, commands, seeds

Tool: `src/bin/nc2_stage0.rs --siblings m1,m2,..` (design, prediction and falsification line pre-registered in the
source header; header commit 05c30d6cf5, then an amended dedup rule in 2257554304 before any reported run, see the
header) [pcc:unverified]. Existing modes are unchanged (`evaluate` now wraps `evaluate_raw`; a unit test checks it).
Tests: `cargo test --release --bin nc2_stage0` (15 passed); `cargo clippy --release --bin nc2_stage0 --tests -- -D
warnings` clean; rustfmt on that file only.

Commands (from the repository root, after `cargo build --release --bin nc2_stage0`), seed 20261108 (the default),
n = 2000 samples per curve, all in `run.sh` [pcc:unverified]:

```
./target/release/nc2_stage0 --r 131203,131317,131111 --c 1,2 --n 2000 --seed 20261108 --siblings 1,2,4,8,16 > raw/k20.txt
./target/release/nc2_stage0 --r 2097349,2097559,2097523 --c 1,2 --n 2000 --seed 20261108 --siblings 1,2,4,8 > raw/k24.txt
./target/release/nc2_stage0 --r 33558163,33554761,33558059 --c 1,2 --n 2000 --seed 20261108 --siblings 1,2,4,8 > raw/k28.txt
```

Raw outputs (per-curve lines and the AGG lines used below): `raw/k20.txt`, `raw/k24.txt`, `raw/k28.txt`. Tables are
built from them by `gen_stage0.py` [pcc:unverified]. Per-k value = mean of the three curve means, SE = sqrt(sum of
squared SEs)/3. Samples are the NC-2 held-out samples (seed ^ 0xAAAA0001); D0, D0i and every m share them; start and
jump prefix `J_1..J_m` are shared across m within a sample.

Design as run [pcc:unverified]: m uniform jumps in `Z_r^(2+C)`, the walk uses the same m jumps with a point-independent
stream, start stored, all children of every parent stored and keyed, rank K = C + 1 stop with the seed-part guard, cap
10 sqrt(r) points. Two children of each new parent are copies of stored points by construction
(`P_{t+1} - J_{h_t} = P_t`, `P_{t+1} - J_{h_{t-1}} = P_{t-1} + J_{h_t}`) and are not emitted, so E_points counts
distinct emitted points. E_parents = parent expansions begun at stop / sqrt(r).

## Controls

Both controls are inside their pre-registered intervals (D0i within 3 aggregate SE of the Poisson floor; D0 per-k in
[1.34, 1.43] at C = 1 and [1.68, 1.77] at C = 2) [pcc:unverified]. D0i: C = 1 values 1.3296, 1.3296, 1.3272 (floor
1.329, SE 0.006); C = 2 values 1.6719, 1.6567, 1.6543 (floor 1.662, SE 0.006). D0: C = 1 values 1.3816, 1.3793, 1.3836;
C = 2 values 1.7303, 1.7119, 1.7297; these equal the `--chained` D0 aggregates of `../snc/RESULTS.md` at C = 2
[pcc:unverified].

## Results

E_points = stored points at stop / sqrt(r); "hit" is the share of samples reaching rank K before the cap; rows with
hit below 1 are censored at the cap, so their E is a lower bound (marked `>=`) [pcc:unverified]. Ratio columns are
point ratios (the SE of a ratio is about its E_points SE over D0, 0.5 percent at k = 20 to 28 for m >= 8)
[pcc:unverified].

### C = 1 (K = 2), Poisson floor 1.329 [pcc:unverified]

| k | m | E_points +- SE | E_parents +- SE | rank-K hit | E_points / D0 | E_points / floor | E_points / E_parents |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 20 | D0 | 1.3816 +- 0.0065 | n/a | 1.0000 | 1.000 | 1.040 | n/a |
| 20 | D0i | 1.3296 +- 0.0062 | n/a | 1.0000 | 0.962 | 1.000 | n/a |
| 20 | 1 | >= 9.9983 +- 0.0000 | >= 9.9927 +- 0.0000 | 0.0000 | >= 7.237 | >= 7.523 | 1.00 |
| 20 | 2 | >= 9.8179 +- 0.0105 | >= 3.9249 +- 0.0042 | 0.0757 | >= 7.106 | >= 7.387 | 2.50 |
| 20 | 4 | >= 4.6089 +- 0.0207 | >= 0.7375 +- 0.0033 | 0.9987 | >= 3.336 | >= 3.468 | 6.25 |
| 20 | 8 | 2.5715 +- 0.0119 | 0.1830 +- 0.0008 | 1.0000 | 1.861 | 1.935 | 14.05 |
| 20 | 16 | 2.1590 +- 0.0102 | 0.0732 +- 0.0003 | 1.0000 | 1.563 | 1.625 | 29.49 |
| 24 | D0 | 1.3793 +- 0.0065 | n/a | 1.0000 | 1.000 | 1.038 | n/a |
| 24 | D0i | 1.3296 +- 0.0061 | n/a | 1.0000 | 0.964 | 1.000 | n/a |
| 24 | 1 | >= 9.9995 +- 0.0000 | >= 9.9982 +- 0.0000 | 0.0000 | >= 7.250 | >= 7.524 | 1.00 |
| 24 | 2 | >= 9.9527 +- 0.0059 | >= 3.9806 +- 0.0024 | 0.0173 | >= 7.216 | >= 7.489 | 2.50 |
| 24 | 4 | >= 4.9401 +- 0.0221 | >= 0.7904 +- 0.0035 | 0.9960 | >= 3.582 | >= 3.717 | 6.25 |
| 24 | 8 | 2.6036 +- 0.0121 | 0.1846 +- 0.0009 | 1.0000 | 1.888 | 1.959 | 14.10 |
| 28 | D0 | 1.3836 +- 0.0064 | n/a | 1.0000 | 1.000 | 1.041 | n/a |
| 28 | D0i | 1.3272 +- 0.0063 | n/a | 1.0000 | 0.959 | 0.999 | n/a |
| 28 | 1 | >= 9.9999 +- 0.0000 | >= 9.9996 +- 0.0000 | 0.0000 | >= 7.227 | >= 7.524 | 1.00 |
| 28 | 2 | >= 9.9913 +- 0.0022 | >= 3.9966 +- 0.0009 | 0.0045 | >= 7.221 | >= 7.518 | 2.50 |
| 28 | 4 | >= 5.0727 +- 0.0229 | >= 0.8116 +- 0.0037 | 0.9930 | >= 3.666 | >= 3.817 | 6.25 |
| 28 | 8 | 2.5958 +- 0.0121 | 0.1838 +- 0.0009 | 1.0000 | 1.876 | 1.953 | 14.12 |

### C = 2 (K = 3), Poisson floor 1.662 [pcc:unverified]

| k | m | E_points +- SE | E_parents +- SE | rank-K hit | E_points / D0 | E_points / floor | E_points / E_parents |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 20 | D0 | 1.7303 +- 0.0066 | n/a | 1.0000 | 1.000 | 1.041 | n/a |
| 20 | D0i | 1.6719 +- 0.0062 | n/a | 1.0000 | 0.966 | 1.006 | n/a |
| 20 | 1 | >= 9.9983 +- 0.0000 | >= 9.9927 +- 0.0000 | 0.0000 | >= 5.778 | >= 6.016 | 1.00 |
| 20 | 2 | >= 9.9983 +- 0.0000 | >= 3.9971 +- 0.0003 | 0.0000 | >= 5.778 | >= 6.016 | 2.50 |
| 20 | 4 | >= 6.1579 +- 0.0210 | >= 0.9853 +- 0.0034 | 0.9828 | >= 3.559 | >= 3.705 | 6.25 |
| 20 | 8 | 3.2575 +- 0.0120 | 0.2316 +- 0.0008 | 1.0000 | 1.883 | 1.960 | 14.07 |
| 20 | 16 | 2.7016 +- 0.0101 | 0.0913 +- 0.0003 | 1.0000 | 1.561 | 1.626 | 29.59 |
| 24 | D0 | 1.7119 +- 0.0065 | n/a | 1.0000 | 1.000 | 1.030 | n/a |
| 24 | D0i | 1.6567 +- 0.0062 | n/a | 1.0000 | 0.968 | 0.997 | n/a |
| 24 | 1 | >= 9.9995 +- 0.0000 | >= 9.9982 +- 0.0000 | 0.0000 | >= 5.841 | >= 6.017 | 1.00 |
| 24 | 2 | >= 9.9995 +- 0.0000 | >= 3.9993 +- 0.0001 | 0.0000 | >= 5.841 | >= 6.017 | 2.50 |
| 24 | 4 | >= 6.4738 +- 0.0215 | >= 1.0359 +- 0.0034 | 0.9743 | >= 3.782 | >= 3.895 | 6.25 |
| 24 | 8 | 3.2767 +- 0.0123 | 0.2322 +- 0.0009 | 1.0000 | 1.914 | 1.972 | 14.11 |
| 28 | D0 | 1.7297 +- 0.0065 | n/a | 1.0000 | 1.000 | 1.041 | n/a |
| 28 | D0i | 1.6543 +- 0.0064 | n/a | 1.0000 | 0.956 | 0.995 | n/a |
| 28 | 1 | >= 9.9999 +- 0.0000 | >= 9.9996 +- 0.0000 | 0.0000 | >= 5.781 | >= 6.017 | 1.00 |
| 28 | 2 | >= 9.9999 +- 0.0000 | >= 4.0000 +- 0.0001 | 0.0000 | >= 5.781 | >= 6.017 | 2.50 |
| 28 | 4 | >= 6.5476 +- 0.0223 | >= 1.0476 +- 0.0036 | 0.9675 | >= 3.785 | >= 3.940 | 6.25 |
| 28 | 8 | 3.2772 +- 0.0124 | 0.2321 +- 0.0009 | 1.0000 | 1.895 | 1.972 | 14.12 |

## Falsification line

Count of sizes (k) with E_points above 1.08 x D0, per C and m <= 8 [pcc:unverified]:

| C | m | sizes above 1.08 x D0 | count |
|--:|--:|--|--|
| 1 | 1 | 20, 24, 28 | 3 of 3 |
| 1 | 2 | 20, 24, 28 | 3 of 3 |
| 1 | 4 | 20, 24, 28 | 3 of 3 |
| 1 | 8 | 20, 24, 28 | 3 of 3 |
| 2 | 1 | 20, 24, 28 | 3 of 3 |
| 2 | 2 | 20, 24, 28 | 3 of 3 |
| 2 | 4 | 20, 24, 28 | 3 of 3 |
| 2 | 8 | 20, 24, 28 | 3 of 3 |

The falsification line (above 1.08 x D0 at two of three sizes for any m <= 8) is met at every m <= 8
[pcc:unverified]. The lower guard (E_points below 0.95 x D0, read as a bug): none [pcc:unverified].

## Reading (post hoc, not pre-registered)

* m = 1 and m = 2 are the walk degenerating, not only siblings: with m jumps the walk is an arithmetic progression
  (m = 1) or a two-generator lattice walk (m = 2), so almost every collision repeats one row and rank K is not reached
  (hit near 0 for m <= 2 at C = 2) [pcc:unverified]. m = 4 is still short of D0 because the 4-jump walk has few
  distinct rows. These three confound the sibling effect with the small jump set; D0 uses 16 jumps. The clean
  comparison is m = 16, where the walk has D0's 16 jumps [pcc:unverified].
* Same-parent correlation, mechanism [pcc:unverified]: a collision `P_t + J_a = P_s + J_b` (as scalars, up to sign)
  implies the second collision `P_t - J_b = P_s - J_a`, and both give the same row `(v_t - v_s + J_a - J_b)`. The
  second is rank-dependent, so informative events are about half of the raw count and E rises by about sqrt(2) at
  the same number of stored points; the two known copies per parent and walk-lattice repeats add the rest. Check
  with `diag_rows.py` (Python, 20 samples, r = 131203, C = 1, walk run to 12 sqrt(r) points; output in
  `raw/diag_rows.txt`): raw collisions per distinct row (up to sign) are 2.95 at m = 16 and 14.9 at m = 4
  [pcc:unverified]. This does not test whether the effect can be removed (for example by keying only the
  parents, or by sibling sets that break the swap symmetry); that is a separate experiment.
* The accounting that matters: E_parents at m = 16 is about 0.073 (C = 1) and 0.091 (C = 2) sqrt(r) against
  E_points about 2.16 and 2.70, ratio about 30 [pcc:unverified]. Whether the per-point price (about 0.59 of a step per
  child in the harness model) pays for E_points rising by the factor in the table is outside this Stage 0
  [pcc:unverified].

## Approximations and caveats

* Dedup of two known copies per parent is a choice made before the full runs and recorded in the header; without it
  E_points would count duplicate vectors [pcc:unverified].
* One n = 200 smoke run of the first dedup version was seen before the full runs (no table kept) [pcc:unverified].
* E_parents counts the parent expansion in progress at stop as a whole parent [pcc:unverified].
* Walk and children share the same m jumps, as specified; the variant where the walk uses 16 jumps and only m are
  child jumps was not run [pcc:unverified].
* D0 here is the same-run D0 at n = 2000; the Poisson floors (1.329, 1.662) are from the NC-2 engine
  [pcc:unverified].
