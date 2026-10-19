# E3 protocol: does a Semaev oracle change the picture? (frozen)

**Status: FROZEN 2026-10-19, before any measured E3 run.** This is the
PROTOCOL section for experiment E3 of [`PLAN.md`](PLAN.md) §4. It is written
after the pilot listed under "Disclosure" and before the measured sessions.
Edits after the freeze go in a dated "Deviations" section at the end. They
never rewrite the text above it.

**No security claim.** Nothing here concerns P-192 or P-224 beyond PLAN §7's
fixed wording.

## Question

PR #2297 found that with the MITM oracle a random-scalar known-log base
(`known-log:scalars=random`, multiples of `G` with free logarithms) matches
an abscissa base's yield at equal B. The reason is that MITM never uses the
base's algebraic description. E3 asks whether an oracle that does use that
description changes the comparison.

The oracle is `pkm-semaev` (this PR, `src/cryptanalysis/ic_framework/pkm_semaev.rs`).
For every target it solves Semaev's `S₄(x₁, x₂, x₃, x_R) = 0` with each `x_i`
on the Kummer tower `(x/g)^{2^t} = 1` of the `prime-kummer:t=T` base (PKM).
It runs F4 over the tower quotient ring (`f4_fp_tower`). The abscissa bound of
`prime-abscissa` has no low-degree `F_p` description, so the algebraic base is
`prime-kummer` (PLAN §4 E3 named `ic-abscissa`; this is the substitution the
E3 gate note asked for).

## Population

- **Curves:** three 24-bit prime-order curves `y² = x³ − 3x + b` (cofactor 1),
  the first three of `a_minus_3_prime_order_curves 24 4 20261019 6` on which
  the `t = 2` tower carries a point. Generator = smallest-abscissa point.

  | id | p | b | r | G | v₂(p − 1) |
  |:--|--:|--:|--:|:--|--:|
  | e3-c1 | 9061313 | 1749481 | 9059959 | (0, 3566041) | 6 |
  | e3-c2 | 13040257 | 1901252 | 13043059 | (0, 6024490) | 7 |
  | e3-c3 | 11223809 | 10934121 | 11224091 | (0, 1525693) | 8 |

  `2^t | p − 1` is a filter on p. These curves are a **restricted prime
  family**, and every statement says so (PKM note §4.5).
- **Tower heights:** `t ∈ {2, 3, 4, 5}`. B is the number of columns of
  `prime-kummer:t`, measured exactly:

  | t | e3-c1 B | e3-c2 B | e3-c3 B |
  |--:|--:|--:|--:|
  | 2 | 2 | 3 | 1 |
  | 3 | 4 | 5 | 3 |
  | 4 | 9 | 10 | 9 |
  | 5 | 19 | 19 | 18 |

  `t = 6` (N = 18 tower variables) is out of reach. The PKM note §16 measured
  44 minutes and a 14 GB cap per system on its host, so `t = 6` is not run.
- **Summands:** `m = 3` in every arm. At `m = 2` the question has a definite
  answer (`E3-blocked.md`: any S₃ solve is the O(B) sweep).
- **Control curve** (end-to-end check only): `p = 2393`, `b = 1614`,
  `r = 2377`, `G = (0, 80)`, a 12-bit curve with `8 | p − 1`.

## Arms (ecbench, `ic.pipeline` unless stated)

For each curve and each t, with B from the table:

| arm | factor base | oracle | trials |
|:--|:--|:--|:--|
| `sem-chain-t{t}` | `prime-kummer:t={t}` | `pkm-semaev:m=3,crosscheck=1` (chain presentation, F4) | capped, `max_trials = K_t` |
| `sem-full-t{t}` (t ≤ 4) | `prime-kummer:t={t}` | `pkm-semaev:m=3,mode=full,crosscheck=1` (S₄, F4) | capped, `K_t` |
| `sem-grid-t{t}` | `prime-kummer:t={t}` | `pkm-semaev:m=3,engine=exhaustive` (V³ grid reference) | **full solve** |
| `kl-mitm3-t{t}` | `known-log:size=B,scalars=random` | `mitm:m=3,negation_folded=1` | **full solve** |
| `rho-neg` | — | `rho.negation` | full solve |

`K_t = 400, 100, 10, 1` for `t = 2, 3, 4, 5`. `sem-full-t5` is not run: its
pilot cost is unknown, and `sem-chain-t5` already takes about 2 minutes per
system.

- **Targets:** 8 `planted_shared` per curve, `target_seed = 20261019`;
  1 round, no warm-up, `order = alternate`, `seed = 20261019`. Host L0. The
  metrics are operation counts, which are deterministic; wall time is a
  practicality note only.
- **Control session:** on the control curve, `sem-chain-t2` and
  `sem-grid-t2` both run to a full solve, plus `kl-mitm3-t2` (B = 3) and
  `rho-neg`, with 8 targets.

## Why F4 is capped and how its full-solve cost is formed (substitution)

A full F4 solve is out of reach on the 24-bit curves. At `t = 4` it would
take about `(B + 1)/h ≈ 10⁵` trials at about 1 s each. The arms are coupled:
`sem-chain`, `sem-full` and `sem-grid` run on the same base with the same
algorithm seed. They therefore draw the same trial stream and get the same
verdict on every trial, which `crosscheck=1` checks on every F4 refutation
and the recovery checks on every consistent system. **The F4 arm's full-solve
cost is the grid arm's measured full-solve record, with the grid's
multiplications replaced by F4's multiply-adds per trial times the trials:**

```text
U_F4 = U_grid − grid_mults_on_refuted_trials + trials · c̄_F4 + (system mults per trial) · trials
```

where `c̄_F4` is the capped arm's measured mean multiply-adds per F4 call.
The grid multiplications on consistent trials stay in: the F4 arm also reads
a consistent system's solutions from the grid. The group work is the
measured group work of the grid record, which equals the F4 arm's on the
same stream (checked on the capped trials). **This is a substitution, not a
run, and every table labels it as such.** It rests on two checked facts:
(a) identical trial streams and verdicts (checked on all capped trials);
(b) F4's cost per refutation depends on the system's shape only. The pilot
saw byte-identical counts across targets at each t; the measured spread over
all capped calls is reported.

## Metrics

1. **Yield at equal B.** The exact count `N₃` of points that are a sum of
   three base points, for `prime-kummer:t` and `known-log:size=B,scalars=random`
   (`generator_effect_e3`), and `h = N₃ / r`. The measured hit rates
   `relations_found / targets_tried` of `kl-mitm3` and `sem-grid` are pooled
   per (curve, t) with a binomial 90% interval.
2. **F4 cost per call:** mean, min and max multiply-adds over all capped
   calls per (curve, t, presentation); the solving degree; refutations and
   consistent systems separated through `generator_effect_e3 f4cost` (4 random
   and 4 planted systems per curve and `t ≤ 4`, 1 + 1 at `t = 5` on e3-c1).
3. **Per-arm S at w ∈ {0, 0.1, 1}** GAE per unpriced native operation
   (AGENTS.md §5a, `ecbench::band` arithmetic):
   `S(w) = (G − C + w·(U + n_C)) / √r`. This is measured for `kl-mitm3`,
   `sem-grid` and `rho-neg`, and substituted for `sem-chain` and `sem-full`.
4. **Crossover.** For each (curve, t, w), `S_sem / S_kl` and each arm's ratio
   to `rho-neg`. The break-even weight `w*(B)` is where `S_sem(w) = S_kl(w)`;
   both are affine in w, so `w*` is a closed form. The crossover `B*` at
   fixed w is where `S_sem / S_kl` crosses 1 along t, if it does inside the
   band, interpolated in log B between adjacent t.
5. **Intervals:** 90% percentile bootstrap over the 8 targets (10 000
   resamples, seed 20261019). The F4 and known-log arms share targets per
   curve, so ratios are paired by target. `c̄_F4` enters with its measured
   per-curve spread.

The aggregation is Rust (`examples/generator_effect_e3_summary.rs`). Python
only plots recorded or aggregated values (AGENTS.md).

## Pre-registered predictions

- **P1 (yield).** At equal B the two bases have the same `N₃` to within 1%,
  and the measured hit rates of `kl-mitm3` and `sem-grid` agree with
  `N₃ / r` (binomial 90% interval covers it). The algebraic description buys
  no yield.
- **P2 (F4 shape).** At each (curve, t, presentation) every F4 refutation
  costs the same number of multiply-adds; consistent systems cost at most 2×
  a refutation.
- **P3 (band flip).** At w = 0 every Semaev arm's S is below `kl-mitm3`'s at
  every t: F4 work is free, and the arm pays only for target draws and lifts,
  against MITM's 2B probes per trial. At w = 0.1 and w = 1 the F4 arms are
  above `kl-mitm3` by at least 10⁴ at every t ≥ 3. **The order of `sem-*`
  against `kl-mitm3` therefore changes inside the band, and §5a leaves it
  unresolved at every t.**
- **P4 (no crossover in B).** `w*(B)` falls as B grows: F4's cost per call
  grows by at least 50× per tower level, while MITM's per-trial cost grows
  2×. `w*` is below 10⁻⁴ for every t ≥ 3. No crossover `B*` exists in the
  band at w ≥ 0.1, and the extrapolation in t points away from one.
- **P5 (the reference).** The grid reference on the same system is cheaper
  than F4 by at least 10³ per trial at every t ≥ 3, at every w > 0.
- **P6 (rho).** Every IC arm is above `rho-neg` at every w (distance
  remaining, not a gate).

There is no decision rule (PLAN §4 E3). The result is reported as a crossover
estimate with intervals.

## Disclosure: what ran before the freeze

- Smoke session `ECBS1hbdbd239c0809` (scratch, e3-c1, t = 3, 1 target,
  `max_trials = 40`): the pipeline runs and records F4 as unpriced solver work.
- `generator_effect_e3` sizes for all three curves (the B table above) and
  `N₃` for t = 2…6.
- `f4cost` pilot on e3-c1:
  - chain, 3 random + 3 planted systems at t = 2, 3, 4: refutations cost
    3.91·10⁵, 1.59·10⁸ and 1.06·10¹⁰ multiply-adds, identical across targets;
  - full, 1 + 1 systems at t = 3, 4: 1.31·10⁷ and 1.68·10¹⁰;
  - chain, 1 refutation at t = 5: 3.19·10¹² multiply-adds, 111 s.
- Unit and harness tests (planted decompositions, F4 against the grid).

None of these runs is data. The predictions above were written after seeing
them.

## Deviations

(none yet)
