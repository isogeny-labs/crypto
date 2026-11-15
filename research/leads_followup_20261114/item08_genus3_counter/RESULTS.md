# Item 8 RESULTS: genus-3 point counter and generic-Jacobian D family (lead J1)

Run 2026-11-14/15. Every number below is a [pcc:unverified] assertion from the logs in `data/`. No leaderboard or
benchmarks/README rows. Protocol: `PROTOCOL.md` (frozen before data; no amendment).

Repos: crypto `44d35033fa` (branch claude/cool-davinci-szm3om), ml-cryptanalysis `27a67b9` (not used by this item),
cryptanalysis (not used; the L-polynomial formula was taken from `src/curve_construction/lpoly.rs` of the crypto
repo, `char_poly_from_counts`). Host: 4 vCPU shared, at most 2 threads used, no run over 6 min.

Files: `jcount.rs` (counter), `ghsgen.rs` (GHS instance generator using unmodified N4 `Inst::search`), `adapter/`
(copy of N4v2 sources with the generic-Jacobian adapter), `cost.py` (M-eq from records), `data/` (all raw logs).
Source sha256: jcount.rs `8a9a77be`, adapter/solver2.rs `c434b947`, adapter/v2.rs `2c899876`;
original N4v2 solver2.rs `f3cbc75c`, v2.rs `3a022304`, instances.jsonl `94bfd391`
(full hashes by `sha256sum`; binaries were deleted after use: jcount `9aae3690`, v2x `f5d8d942`).
Builds: `rustc --edition 2021 -O jcount.rs`; `cd adapter && rustc --edition 2021 -O v2.rs -o v2x`.

## 1. The counter (`jcount.rs`)

Standalone, std only. `jcount count --q Q --f f0,..,f14 [--threads T]` prints N1,N2,N3, the Frobenius characteristic
polynomial coefficients (T^6 down; same Newton recursion as `char_poly_from_counts`), `J = P(1)`, its factorisation
(trial division, Miller-Rabin, Pollard rho in u64) and a Weil-interval check. Smoothness is checked first (port of
`pic::smoothness_certificate`: rank of (F_X,F_Y,F_Z) in degree 7 equals 36). Other subcommands: `brute`, `rand`
(counter vs. brute force), `jsonl` (GHS check), `time`, `search` (cofactor <= 8), `points` (Tonelli-Shanks self-test).

Method: for x over F_{q^k} (one representative per Frobenius orbit, weight k) count the distinct roots of
g(y) = F(x,y,1) as deg gcd(g, y^Q - y). y^Q mod g = one square-and-multiply for y^q plus (k-1) Frobenius applications
as d x d linear maps in the powers (y^q)^i mod g; gcd degree by pseudo-remainder sequence (no inversions beyond one
monic normalisation per x). Points at z = 0 from the binary quartic F(X,Y,0). Barrett reduction for q < 2^31. The
counter uses no square roots, so q = 1 mod 4 and q = 3 mod 4 are treated alike (measured: q = 5, 13, 17 and 7, 11, 19 below).
Tonelli-Shanks is implemented (`points`, self-test q = 7, 13, 17, 1000003: 0 failures) but not needed by the counter;
it is what would be needed to lift the `Fp::sqrt` q = 3 mod 4 restriction in v2 (not done, see Open).

## 2. Validation

V1, brute force (`data/v1_brute.jsonl`, 136 random smooth quartics, all fields of `rand` records): fast counter vs. direct
evaluation of F at every point of P^2(F_{q^k}), k = 1,2,3, plus Newton integrality of the L-polynomial.
q = 5, 7, 11, 13: 30 curves each; q = 17, 19: 8 curves each. Result: 136/136 match for N1,N2,N3, 0 Newton failures.
(Brute force shares only the field class `Ext` with the counter.)

V2, GHS instances (`data/v2_ghs_*.jsonl`, `data/ghs_extra.jsonl`): expected `#J = order` (= #E0(F_{q^3}), the relation
`v2 homcheck` tests as `#J * x = 0`). Result: 24/24 equal, all smooth, all Weil-interval checks inside.
Distinct q: 67, 79, 103, 127, 139, 151 (3 instances each, fresh from the unmodified N4 builder via `ghsgen`), and
q = 223 (all 5 seeds of results/instances.jsonl) and q = 499 (seed 1 of results/instances.jsonl), 8 q-values in all
over the 3 required. Example: q = 499 seed 1: N = [500, 249002, 124253324], J = 124252108 = order.
Not run: q = 991 and above of the instance file (single N3 about 40 min for 1 thread; dropped for the time cap), seeds 2 to 5 of q = 499.

V3, independent library check on generic curves: for each of the 6 generic instances of section 4 the adapter prep
(`icxprep`) tested `mul_scalar(x, #J) == zero` on 10 random Pic^0 classes with pic's own arithmetic, plus P = cof * D
nonzero and `ell * P == zero`: 0 failures in 60 classes and 6/6 instances (`data/adapter/inst/*.json`, fields
`jx_zero_failures`, `p_nonzero`, `ell_p_zero`). This checks the counter's #J on curves with no GHS structure.

Prediction scorecard: P1 holds (0 mismatches). P2 holds (0 mismatches, 24/24).

## 3. Timing (`data/timing_1thread.jsonl`, `timing_2thread.jsonl`, `timing_k2.jsonl`)

One random smooth quartic per q; wall seconds; N3 enumerates q^3/3 representatives.

| q | N1 s | N2 s | N3 s (1 thread) | ns per representative |
|--:|--:|--:|--:|--:|
| 31 | 0.0004 | 0.002 | 0.072 | 7267 |
| 61 | 0.0003 | 0.008 | 0.46 | 6087 |
| 101 | 0.0003 | 0.018 | 2.22 | 6477 |
| 151 | 0.0005 | 0.044 | 8.07 | 7030 |
| 211 | 0.0005 | 0.092 | 22.7 | 7264 |
| 307 | 0.0007 | 0.21 | 74.8 | 7752 |
| 401 | 0.0010 | 0.37 | 174.6 | 8122 |
| 607 (2 threads) | 0.0015 | 0.47 | 324.1 | 8696 per thread |
| 1009 | 0.0014 | 1.29 | not run | |
| 2003 | 0.0048 | 11.1 | not run | |
| 5003 | 0.014 | 78.6 | not run | |

(q = 223: 28 s, q = 499: 340 s for N3 in the GHS runs, also 1 thread. The 1-thread rows ran next to one other
1-thread job, the 607 row ran alone with 2 threads.) Cost per representative grows slowly with q (6.1 to 8.7 microseconds,
ratio 1.4 over q = 61..607; about log q), N3 time follows q^3/3 times that. N2 grows as q^2.

Extrapolation (derived, not measured beyond q = 607): with 2 threads one N3 takes about 25 min at q = 1009 and about
20 min at q about 940; 2.5 h corresponds to q about 1800. **Largest q measured for N3: 607 (324 s on 2 threads). Largest q
feasible under the protocol's 20-minute criterion: about 940 (derived).** N1,N2 alone are cheap to q = 5003 and beyond.
Note that #J > 2^64 does not occur in this range (q^3 < 2^64 for q < 2.6e6), so the factoriser is sufficient there.

Prediction scorecard: P3 constant within factor 1.5 (ratio 1.43 over q >= 61; true, ns per representative in 5 to 40 microseconds):
holds. The predicted absolute times were wrong in the pessimistic direction: q = 211 predicted 30 to 150 s, measured 22.7 s;
q = 401 predicted 4 to 20 min, measured 2.9 min; largest feasible q predicted 400 to 700, measured/derived about 940.

## 4. Adapter and D family on generic cofactor-<=8 quartics

Adapter (in `adapter/`, a copy; the N4v2 sources are untouched): `solver2.rs` gains `GenSrc` and an extra
`run_family(..., gen: Option<&GenSrc>)` argument (about 77 diff lines); `v2.rs` gains `icxprep` and `icx` subcommands
(about 149 diff lines). `icx <inst.json> <D,DL1,DL> [cD gamma block]` takes q, f, ell, cof, order, x and a base-point-free
divisor D (4 plus, 4 minus points). P = cof * D, Q = x * P are built with pic once the base point exists (uncharged
control block). The descent start classes are R, R + P, R + Q with R the sum of two random factor-base point classes, the
check is `mul_scalar(P, k) == Q`; E0 data are dummy fields (the generic path reads only q, ell, f). Relations, graph,
linear algebra, log recovery, descent loop unchanged. The planted x only builds Q; the solver never sees it.
Regression: on GHS q = 223 seed 1 the patched binary's `ic` output replays identical (counters) to the original binary
(`v2 replaycmp`: REPLAY IDENTICAL; `data/adapter/reg_*`).
Restrictions kept: q = 3 mod 4; the dummy `Inst` also needs q = 7 mod 12 (`Kf::new`), so the generic q were chosen 7 mod 12.

Instances: `jcount search` on random smooth quartics (N1 >= 4), accept #J = cof * ell, cof in 1..8 (smallest first), ell prime
> 8. Hit rate: q = 223: 3 of 22 counted curves (seed 11); q = 367: 3 of 16 (seeds 21, 22, two processes), about 1 in 5 to 7.
Search cost: q = 223 616 s, q = 367 about 135 s per count (single thread each).

| q | inst | cof | ell | #J | attempts (c) | k recovered = planted | D M-eq w=0 (r_inv 15, r_lmul 1.1) |
|--:|--:|--:|--:|--:|--:|:-:|--:|
| 223 | 1 | 8 | 1428143 | 11425144 | 2 (2.00 failed "too few relations: 24 rows for 30 columns", 2.25 ok) | yes | 4.20e5 |
| 223 | 2 | 5 | 1962403 | 9812015 | 2 (14 rows for 30) | yes | 8.03e5 |
| 223 | 3 | 1 | 10966811 | 10966811 | 2 (18 rows for 30) | yes | 8.94e5 |
| 367 | 1 | 1 | 50806843 | 50806843 | 2 | yes | 6.15e5 |
| 367 | 2 | 5 | 9673177 | 48365885 | 2 | yes | 8.82e5 |
| 367 | 3 | 2 | 22603417 | 45206834 | 2 | yes | 7.07e5 |

DL1 and DL recovered the same planted log in the same attempt on all 6 instances (`data/adapter/icx_*.jsonl`).
Reference (results/summary_xeonconst.md, GHS D at the same constants): q = 223 5.37e5, q = 499 6.04e5. The generic values
are 0.8x to 1.7x of the GHS value at q = 223, and 1.0x to 1.5x of 6.04e5 at q = 367, i.e. inside the predicted factor 2.
GHS D at q = 223 also needs 2 attempts on average (summary: 2.00), so the escalation is the small-q effect of nfb = 2 sqrt(q),
not a generic-Jacobian effect. M-eq is w = 0 only: hash + cmp counts are in the records (2.7e3 to 5.6e3), but no GAE unit for a
Jacobian addition was measured, so no section-5a band and no rho comparison was made. Wall time of one D unit is 0.004 to 0.008 s.

Prediction scorecard: P4 holds at both q: 6/6 recoveries (3/3 at each q), verified by `x*P == Q` in Pic, cost within factor 2 of GHS D. [pcc:unverified]

## What was dropped, and what is not claimed

* The two "largest feasible q" for the adapter were q = 223 and 367, not about 940: a cofactor <= 8 search needs about
  5 to 7 counts per instance (22 and 16 counts here), so q = 499 (340 s per count) would have taken about 1 h per instance.
* No rho or GAE pricing for generic Jacobians; no larger q for D; only the D family (S family not run).
* GHS check at q = 991 and above not run; q = 499 only seed 1.
* Instances are random, hence q small and #J about q^3: these runs show the D family is not tied to the GHS structure at
  these sizes. They say nothing about asymptotics or about beating rho.
* The CLAUDE.md session log was not written (instruction limited writing to this directory); the main session should log this item.

## Open / next commands

* Lift the q = 3 mod 4 restriction: Tonelli-Shanks in `common.rs` `Fp::sqrt` and `ic2.rs:459` (see J1_adapter_report.md section 3), and
  replace `dummy_inst` by an `Inst`-free `Src` struct so q need not be 7 mod 12.
* Larger q for generic D: `jcount search --q <q> --seed S --want 1 --threads 2` then
  `adapter/v2x icxprep <inst> S` and `adapter/v2x icx <inst.json> D,DL1,DL` (q about 600 costs about 6 counts x 5 min on 2 threads).
* Prune the search: a cheap necessary filter on #J (e.g. from bitangent / 2-torsion counts) would cut the 5 to 7 counts per instance.
