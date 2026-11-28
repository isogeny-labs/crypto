# R13-C: is the genus-3 linear-algebra exponent p^(3/2) or p^(4/3)(ln p)^c?

Status: protocol frozen before the measured grid. Every number is `[pcc:unverified]`.
This is a measurement of an unchanged arm (`genus_ladder` at `f49709baf2`, R11), not a
new algorithm and not a leaderboard row.

## Question

The campaign's best genus-3 hyperelliptic arm (`2LP + prefilter + SGE(8,3)`) sits at
0.300 / 0.399 / 0.377 of expected rho at p = 65537 / 131101 / 262147, and its fitted
FPM exponent (1.422) is not separated from rho's (1.360). The linear-algebra share rises
to 0.67 and `la_mul` scales as p^1.52 over 8209…262147. R13-C asks whether that local
p^1.52 is the leading p^{3/2}, or the Diem/GTTD p^{4/3} times a slowly growing
`(ln p)^c` with c ≈ 2. If the latter holds above 2^20, the arm's asymptotic exponent is
4/3 against rho's 3/2 and the H-arm is an exponent story, not only a constant win.

## Instrument and freeze

- Source: `f49709baf2` (the R11 commit, before the `72b9183914` merge that truncated
  `genus_ladder.rs`/`quartic_ladder.rs`). No source change.
- Binary: `target/release/genus_ladder`, SHA-256 `97cd8fd27163a380b46a28be300668b7befb1c3e5ea69cf7e24ab834b47b6f43`
  (built 2026-11-23 from `f49709baf2` on this host).
- The existing p = 262147 rows (`research/genus_ladder_g3_highp_20261121`) were produced
  by a different binary (`6f40afb3…`); they are used only as context. The fit uses rows
  from this binary at all three primes.

## Grid

```sh
genus_ladder run --primes P --genera 3 --curves 6 --seeds 2 --arms c \
  --sge 8,3 --cc 2.0 --max-h 8 --threads 4 --max-trials 4e8 \
  --seed-search 600 --order-method bsgs1 --rho-cap 1e6 --out run_P.jsonl
```

- `P ∈ {262147, 1048583, 1572869}`. `1048583` is the first prime above 2^20;
  `1572869` is a prime below the BSGS1 order-search reach (about 2·10^6).
- `--arms c` only: this is an LA-scaling measurement, not a rho comparison. The
  expected-rho reference is the campaign's `0.886·sqrt(r)` per instance, using the
  measured per-step `cantor_mul + 10·cantor_inv` of the same run.
- `--rho-cap 1e6` skips the measured walk above the small primes; no rho rows are claimed.
- Curves and seeds are the same seed-search order as the campaign, so the first admitted
  curves at 262147 reproduce the campaign's instances where the filters agree.

## Estimands

- `la_mul` cell mean per prime over admitted curves and seeds.
- Local exponent `s(a,b) = log(mean_b / mean_a) / log(p_b / p_a)` for the intervals
  262147→1048583 and 1048583→1572869.
- Fit `la_mul = A · p^{4/3} · (ln p)^c` by least squares, report `c`; bootstrap over
  curves (4000 draws) for 95% intervals.
- Secondary: FPM_mid exponent over the three primes, and the LA share `9·la_mul/FPM_mid`.

## Predictions and falsifier (fixed before the grid)

| id | prediction | falsified if |
|---|---|---|
| P1 | every admitted run certifies `[t]G = Q` | any uncertified row |
| P2 | the top-octave local `la_mul` exponent is ≤ 1.45 | point estimate ≥ 1.50 |
| P3 | the fitted `c` in p^{4/3}(ln p)^c is in [1, 3] | 95% lower bound on `c` ≥ 4 |
| P4 | the FPM_mid exponent over the three primes is ≤ 1.42 | point estimate ≥ 1.48 |

P2 and P3 are the R13-C decision. A pass says the H-arm's LA is consistent with the
4/3 exponent plus a log factor at these sizes; it does not prove the asymptotic exponent.
A fail closes R13-C's reading and leaves the H-arm at the measured p^{3/2}-like growth.

## Limits

Shared host, 4 threads, no isolated clocks, no independent replay; six curves per prime
is a small sample of the h ≤ 8 admitted population. `la_mul` is a counted operation, not
a wall time; the wall time is recorded but is not the metric. The pilot at
p = 1048583 (2 curves, 1 seed) is development data and is not part of the grid.

## Cost

Pilot: about 3 minutes for the first admitted curve's order search and a few minutes of
relations at p = 1048583. Grid: 36 runs per prime, expected a few hours at the top prime
on four threads; about 0.3 container-days.
