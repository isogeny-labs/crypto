# E3b protocol: E3 at larger B, and F4 run to full solves (frozen)

**Status: FROZEN 2026-10-20, before any measured E3b run.** This extends
[`E3.md`](E3.md) (protocol [`E3-PROTOCOL.md`](E3-PROTOCOL.md), merged in
PR #2323) at its two stated limits. Edits after the freeze go in a dated
"Deviations" section at the end and never rewrite the text above it.

**No security claim.** Nothing here concerns P-192 or P-224. The curves are a
restricted prime family (`2^t | p − 1`).

## The two gaps E3 left

1. **B stopped at 19.** The F4 arms could not reach towers above `t = 5`, so
   E3 never got near the MITM optimum. **Part L (ladder)** runs the two
   arms that are measured end to end in E3, the `V³` grid reference on the
   Kummer base and known-log MITM, at `t = 4…9` (B ≈ 6 to 276), past the
   known-log arm's optimum. F4 is not run at `t ≥ 6`: one `t = 6` system
   took 44 minutes and a 14 GB cap in the PKM note §16. So F4's cost there
   stays an extrapolation, and the report says so.
2. **F4's full-solve cost was substituted.** E3 priced the F4 arms from the
   grid arm's records. **Part F (F4 full solves)** runs F4 on every trial to
   a full solve on E3's curve e3-c1, at `t = 2` and `t = 3` and in both
   presentations, on E3's targets 0 and 1. That checks the substitution
   on 24-bit curves. E3 checked it only on a 12-bit control.

## Part L: population and arms

- **Curves:** the first three of
  `a_minus_3_prime_order_curves 24 6 20261020 9` (prime order, cofactor 1,
  `2^9 | p − 1`). Generator = smallest-abscissa point.

  | id | p | b | r | G |
  |:--|--:|--:|--:|:--|
  | e3-c4 | 10398721 | 1458188 | 10402583 | (3, 4853243) |
  | e3-c5 | 15052801 | 1336933 | 15059021 | (1, 1376234) |
  | e3-c6 | 15161857 | 9333862 | 15163639 | (1, 258217) |

- **B per t** (`generator_effect_e3`, [`e3b/sizes.jsonl`](e3b/sizes.jsonl)):

  | t | e3-c4 | e3-c5 | e3-c6 |
  |--:|--:|--:|--:|
  | 4 | 6 | 9 | 10 |
  | 5 | 13 | 19 | 18 |
  | 6 | 29 | 35 | 30 |
  | 7 | 68 | 69 | 61 |
  | 8 | 128 | 134 | 129 |
  | 9 | 263 | 276 | 253 |

  The known-log MITM optimum for `m = 3` is near `B ≈ r^{1/4} ≈ 55–62`, a
  table of about `B²` points against `≈ 1.5 r / B²` probes. The `m = 2`
  optimum of E1, `r^{1/3} ≈ 220–250`, lies between `t = 8` and `t = 9`.
- **Binary:** every E3b session runs the E3 build (`ecbench` sha256
  `b72458fbfa802e88…`, the binary of every E3 record), so Part F is coupled
  to E3's records by construction.
- **Arms** (ecbench, `ic.pipeline`, full solves), per curve and t:
  `sem-grid-t{t}` (`prime-kummer:t={t}` + `pkm-semaev:m=3,engine=exhaustive`),
  `kl-mitm3-t{t}` (`known-log:size=B,scalars=random` +
  `mitm:m=3,negation_folded=1`), and `rho-neg` once per curve.
- **Targets:** 8 `planted_shared` per curve, target seed 20261019,
  algorithm seed 20261019, 1 round, no warm-up, `order = alternate`, host
  L0. Specs: [`e3b/specs/e3-c4.json`](e3b/specs/e3-c4.json), `e3-c5`, `e3-c6`.

## Part F: population and arms

- **Curve and targets:** E3's e3-c1 (`p = 9061313`), `planted_shared` targets
  0 and 1 of target seed 20261019. Targets are indexed by seed and index
  (`ecbench::workload`), so these are exactly E3's first two e3-c1 targets.
- **Arms:** `sem-chain-t2`, `sem-full-t2`, `sem-chain-t3` and `sem-full-t3`:
  E3's oracle strings with `crosscheck=1` and **no `max_trials`**, so F4 runs
  on every trial to a full solve. Each arm has its own spec and session
  (`e3b/specs/f4-c1-*.json`, each with `rho-neg`), and the four run
  concurrently.
- **Targets per arm:** 2 (targets 0 and 1) for `sem-chain-t2`, `sem-full-t2`
  and `sem-full-t3`; 1 (target 0) for `sem-chain-t3`, the dearest arm.
- **Expected wall time** (from the solver wall time in E3's capped records,
  about 1.6 / 1.4 ms per call at t = 2 and 108 / 55 ms at t = 3 for chain /
  full, and E3's trial counts, about 1.2·10⁶ per target at t = 2 and
  3.9·10⁵ at t = 3): about 0.5 h per target at t = 2, 6 h (full) and 12 h
  (chain) per target at t = 3, on an idle core; about 26 CPU-hours in all.
  The host is shared and loaded, so wall time will be longer. The spec
  timeout is 72 h per run. A run that times out is reported as such and is
  not replaced.

## Metrics

1. **Part L, as E3:**
   - `N₃` and the measured hit rates, with Wilson 90% intervals.
   - `S(w) = (G − C + w·(U + n_C)) / √r` at `w ∈ {0, 0.1, 1}` per arm.
   - Ratios grid / known-log and arm / rho, paired by target, with 90%
     percentile bootstrap intervals (10 000 resamples, seed 20261019).
   - The break-even weight `w*` of grid against known-log, and the
     crossover `B*` of grid / known-log along t at each w.

   Aggregated by `examples/generator_effect_e3_summary.rs`, extended
   additively with the grid crossover and `w*`.
2. **Part F, per arm and target:**
   - **Measured:** trials, relations, group GAE, F4 calls, F4 multiply-adds,
     system multiplications, cross-check disagreements, verification.
   - **Substituted:** the same quantities as E3 formed them, from E3's
     measured `sem-grid` record of that target and E3's capped mean
     `c̄_F4`. The ratio measured / substituted of `U` and of `S(w)`.

   Aggregated by `examples/generator_effect_e3b_f4check.rs`.

## Pre-registered predictions

- **L1 (yield).** The hit rates of both arms cover `N₃ / r` (Wilson 90%) in
  at least 34 of the 36 (curve, t, arm) cells. `N₃` of the two bases agree
  within 1% at every t. That part is already known from the sizes run and is
  disclosed below, so it is not a prediction.
- **L2 (known-log optimum).** At w = 0 the known-log arm's mean S is lowest
  at `t = 7` (B ≈ 61–69) on every curve, among `t = 4…9`. It is above
  `rho-neg` at every t and w, at between 1.5× and 10× rho at its optimum.
- **L3 (band flip persists).** At w = 0 the grid arm is below known-log at
  every t. At w = 0.1 and 1 it is above known-log by at least 10³ at every
  `t ≥ 6`. The order flips inside the band at every t, so it is unresolved
  under §5a.
- **L4 (no crossover).** No `B*` exists at any w along `t = 4…9`. The
  grid's `w*` against known-log is below 10⁻³ at every `t ≥ 6`.
- **L5 (grid against rho).** Written after the smoke run below, which showed
  the grid at 1.7× rho at t = 9, w = 0, on one target. The prediction: at
  w = 0 the grid arm is above `rho-neg` at every t, and closest at `t = 9`,
  within 3× rho. At w ≥ 0.1 it is above rho by at least 10³ at every t. So
  rho is ahead across the whole band.
- **L6 (F4 extrapolated, labelled).** F4's multiply-adds per call grew
  67–406× per level at `t = 2…5`. At `t ≥ 6` every F4 arm is reported only as
  "extrapolated, not run". No F4 number at `t ≥ 6` enters a measured table.
- **F1 (same stream).** For each Part F arm and target, the trial count,
  relations and group GAE equal those of E3's `sem-grid` record of the same
  target exactly. All 7 runs verify, and `pkm_crosscheck_disagreements = 0`.
- **F2 (substitution holds).** Measured `U` over substituted `U` lies within
  `[0.99, 1.01]` for every run, and so does `S(w)` at w = 0.1 and 1. At w = 0
  the two are equal exactly, because F4's work is unpriced and the group work
  is the grid's.

No decision rule. E3's conclusion changes only if L3 or L4 fails, or F2 fails
by more than a factor 2.

## Disclosure: what ran before the freeze

- The curve search and `generator_effect_e3` sizes for t = 2…9 on the three
  curves, giving the B and `N₃` tables (`e3b/sizes.jsonl`). `N₃` of the two
  bases agree within 0.3% at every t.
- One smoke run (scratch, not data): e3-c4, 1 target, `sem-grid-t9`,
  `kl-mitm3-t9` and `rho-neg`, to check that the grid runs at |V| = 512.
  Its outcome is recorded under "Smoke" below, written before the freeze.
- E3's own records, which F1 and F2 are checked against.

## Smoke

Scratch session `ECBS1h06a53a38e7a4` in the scratchpad, not data and not committed.
e3-c4, target 0, binary sha256 `b72458fb…` (the E3 build, the same binary
used for every E3b run):

- `rho-neg`: verified, 2 774 GAE (S = 0.86 at w = 0).
- `sem-grid-t9`: verified; 259 trials, 234 relations, 4 627 GAE
  (S = 1.43 at w = 0); 259 grid calls, about 3 s each (767 s of solver wall
  time).
- `kl-mitm3-t9`: verified; 1 trial, 1 relation, 78 923 GAE (S = 24.5 at
  w = 0), almost all of it the pair table.

So the ladder runs in about 13 minutes per grid target at t = 9 on this host,
and Part L takes a few hours per curve.

## Deviations

None yet.
