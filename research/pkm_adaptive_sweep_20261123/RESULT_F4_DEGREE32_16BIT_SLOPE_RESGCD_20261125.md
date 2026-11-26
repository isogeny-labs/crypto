# 16-bit rung, 13→16-bit slope, and the resultant–GCD engine — result

Protocol: `PROTOCOL_F4_DEGREE32_16BIT_SLOPE_RESGCD_20261125.md` (frozen before
data), amendment `AMENDMENT_F4_DEGREE32_16BIT_ONE_CURVE_20261125.md` (one
roster curve at 16 bits, recorded before any timed call). Claude Code, local
4-vCPU container, `RAYON_NUM_THREADS=1`. Every claim is [pcc:unverified].

Figure: `FIG_SLOPE_RESGCD_20261125.{svg,png,pdf}` (generator
`plot_slope_resgcd.awk` over `t_scaling_per_call.tsv` and `slope_points.tsv`).

## Verdict

1. **No downward bend.** The degree-32 Vélu base's advantage over the
   equal-size interval base is a constant F4 *formulation* factor of about
   1/32 in multiply-adds, flat from 13 to 16 bits. Under the frozen rule the
   lead closes as a constant factor.
2. **The signal belongs to F4, not to the base.** With F4 replaced by a
   univariate resultant–GCD elimination, both bases cost the same per call to
   within 0.04 % at every t = 5…12, and 650× less than F4 on the Vélu base
   at t = 5.
3. **No solver changes the exponent at m = 2.** The best per-target engine
   (`scan`) costs Θ(|V|) per call, and the closed pipeline's cost relative to
   rho still grows with field size (`s_ops/rho` slope +0.52 to +0.64 bits per
   bit). Even the group-law `subtract` oracle, with no algebraic solver at
   all, sits 160–242× above rho at 16 bits.

## A. Per-call solver cost against tower height t

16-bit curve `icv1-fp16-t351-d433f24e`, CLI seed 20261400, first aux seed that
builds per t; F_p multiplications per solver call (F4: multiply-adds). Binary
`b74562b0…` (Karatsuba fix). Data: `t_scaling_engines_16bit_20261400.jsonl`.

| t | \|V\| | F4, Vélu | F4, interval | resgcd, Vélu | resgcd, interval | scan | grid |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 5 | 32 | 3,833,802 | 127,286,074 | 5,840 | 5,840 | 1,856 | 1,248 |
| 6 | 64 | 54,997,708 | 1,684,486,114 | 19,238 | 19,237 | 3,616 | 4,544 |
| 7 | 128 | 744,851,483 | — | 66,146 | 66,146 | 7,200 | 17,280 |
| 8 | 256 | — | — | 235,714 | 235,627 | 15,040 | 67,328 |
| 9 | 512 | — | — | 863,632 | 863,582 | 29,152 | 265,728 |
| 10 | 1024 | — | — | 3,237,596 | 3,238,132 | 58,975 | 1,055,744 |
| 11 | 2048 | — | — | 12,374,428 | 12,373,906 | 118,079 | 4,208,640 |
| 12 | 4096 | — | — | 48,132,123 | 48,122,284 | 237,117 | 16,805,888 |

Growth per unit t: F4 ×14.3 and ×13.5 (Vélu), ×13.2 (interval), about
|V|^{3.8}; resgcd ×3.3 → ×3.9 (classical Euclid, |V|²; a half-GCD would give
O(M(|V|) log |V|)); grid ×3.6 → ×4.0; scan ×1.95 → ×2.01 (|V|¹). All engines
see the same targets and find the same relations at every t (F4 to t = 7).

**Correctness.** The first sweep (binary `4aa81d9c…`, preserved as
`t_scaling_engines_16bit_20261400_PREFIX_karatsuba_bug.jsonl`) found 199/191
relations at t = 11/12 against 200/200 for scan and grid; `crosscheck=1`
reported 2 disagreements at t = 11. Cause: the Karatsuba middle term ran only
over `z1`'s trimmed length. After the fix, `crosscheck=1` gives 200/200
systems and 0 disagreements at both t = 11 and 12, and the regression test
`karatsuba_matches_schoolbook_under_cancellation` covers it.

## B. 16-bit closed pipeline (one curve × three target seeds)

Base 17 abscissae on both sides (`prime-abscissa:below=32` against
`prime-isogeny:t=5,seed=15,aux=ordinary`), 16 cold solves per call. All 18
calls exit 0; 288/288 rows verified with `full_rank_reached`, none exhausted;
recovered scalars identical across all six arms at every repeat index. F4
arms on binary `cbb657c4…` (pre-change; F4 path unchanged), other arms on
`b74562b0…`. Data: `e2e_full_rank_16bit_<seed>_{f4,engines}.jsonl`.

| seed | arm | solver ops (16 solves) | trials | relations | median s_over_rho (host GAE) | wall, 1 core |
|--:|:--|--:|--:|--:|--:|--:|
| 20261400 | F4 interval | 6.408e12 | 51,618 | 448 | 6.62e6 | 4,933 s |
| | F4 Vélu | 2.256e11 | 60,573 | 511 | 1.07e6 | 755 s |
| | resgcd Vélu | 3.436e8 | 60,573 | 511 | 1,759 | 2 s |
| | scan Vélu | 1.111e8 | 60,573 | 511 | 1,885 | 1 s |
| | resgcd interval | 2.940e8 | 51,618 | 448 | 2,004 | 1 s |
| | subtract interval | 0 | 51,618 | 448 | 242 | <1 s |
| 20261401 | F4 interval | 6.072e12 | 48,914 | 446 | 6.36e6 | 4,692 s |
| | F4 Vélu | 1.972e11 | 52,788 | 435 | 9.89e5 | 707 s |
| | resgcd Vélu | 3.004e8 | 52,788 | 435 | 1,909 | 2 s |
| | scan Vélu | 9.711e7 | 52,788 | 435 | 1,465 | 1 s |
| | resgcd interval | 2.786e8 | 48,914 | 446 | 1,708 | 1 s |
| | subtract interval | 0 | 48,914 | 446 | 218 | <1 s |
| 20261402 | F4 interval | 6.481e12 | 52,279 | 471 | 4.00e6 | 5,010 s |
| | F4 Vélu | 1.981e11 | 53,194 | 441 | 2.71e5 | 671 s |
| | resgcd Vélu | 3.018e8 | 53,194 | 441 | 1,322 | 2 s |
| | scan Vélu | 9.757e7 | 53,194 | 441 | 863 | 1 s |
| | resgcd interval | 2.973e8 | 52,279 | 471 | 1,397 | 2 s |
| | subtract interval | 0 | 52,279 | 471 | 160 | <1 s |

Host-priced combined GAE ratio F4 Vélu/interval at 16 bits is 0.132 (per seed
0.161 / 0.153 / 0.082). It is **not comparable** with the 13–15-bit values
(0.27–0.38): those ran multi-threaded on a different host, and #5067 already
measured 0.136 single-threaded at 13 bits. The protocol's GAE threshold
(< 0.263) is met only through this host and threading change, which is why the
slope is read in deterministic multiply-adds.

## C. Slope, 13→16 bits

Instance-level points (13: 3 curves, 14: 1, 15: 3, 16: 1 curve × 3 targets).
`slope_points.tsv`, fit by least squares on log₂.

| quantity | slope (bits per bit) | leave-one-rung-out range |
|:--|--:|:--|
| F4 multiply-add ratio Vélu/interval | +0.067 | +0.048 … +0.105 |
| `s_ops/rho`, F4 Vélu (solver ops per solve ÷ ρ_s·√r) | +0.64 | +0.59 … +0.82 |
| `s_ops/rho`, F4 interval | +0.52 | +0.48 … +0.75 |

The 16-bit multiply-add ratios (0.0306, 0.0325, 0.0352) lie inside the 13–15
range (0.0277–0.0402). Frozen rule: a bend needs the 16-bit point below the
13–15 minimum *and* a negative slope stable under leave-one-out. Neither
holds. `s_over_rho` rises for both arms at about the rate that fixed t
predicts (trials ∝ p/|V| at constant per-call cost gives slope ≈ 0.5).

## Why this closes the lead

At m = 2, a relation is a pair in V² summing to the target, and with V fixed
the expected trials grow linearly in p. Balancing needs |V| ≈ √p, and then
even the Θ(|V|) scan costs ~√p per target over ~√p targets: total ≈ p, against
rho's √p. A cheaper per-system solver moves only the constant. F4 added
roughly 650–2000× on top of an exponent that was already wrong. The open
levers in `SUBEXP_COST_MODEL_20261125.md` (a sub-|V| join, or m ≥ 3 with a
subexponential solver) are untouched by this result.

## Claim boundary

Operation counts are deterministic; wall time and GAE are host-dependent
(local container, not Iso Lab EKS: no AWS route in this session; not the
host of the 13–15-bit rungs). 16 bits has one roster curve. Inversions and
square roots are charged at 32 multiplications. No deployed curve is
involved.
