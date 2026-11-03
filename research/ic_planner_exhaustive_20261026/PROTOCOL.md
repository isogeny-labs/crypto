# Protocol: an exhaustive re-test of the three planner studies, with Lean proofs

Frozen 2026-10-26, before any recorded session of this study. The owner asked
(2026-10-26) for the three results of `research/ic_prime_residual_lp_20261026`,
`research/ic_koblitz_descent_routing_20261026` and
`research/ic_planner_weight_rule_binkob_20261026` to be re-tested more
exhaustively and proven in Lean. Planner `ic-planner/3`. ecbench band now
reports exact per-arm sums (`charged_sum`, `native_sum`).

## What is measured (`specs/*.json` from `scripts/make_specs.sh`)

Every IC arm is the `ic plan` arm, with only the stated parameter changed.

### R: the residual collector and the prime weight rule at new sizes

Four registered prime-order curves outside the earlier ladder:

| tag | slug | log2 r |
|:--|:--|--:|
| p16 | `icv1-fp16-t295-8d3c3165` | 15.70 |
| p18 | `icv1-fp18-tm337-d28d5e09` | 17.70 |
| p36 | `icv1-fp36-t454193-44a20818` | 36.00 |
| p37 | `icv1-fp37-t277-91fd6e29` | 36.25 |

Arms:
- `rho` (`rho.negation`);
- `plan` (`N₀`);
- `m141` (`round(1.41·N₀)`, the prime w = 1 rule, held out);
- `res2`, `res4`, `res8`, `res16`, `res32`: the residual collector at B
  abscissae.

Each session has 16 targets.

### B: the binary rule, held out

The four held-out curves named in the binary/Koblitz protocol: hb19, hb23,
hb25 and hb27. Arms: `rho`, `plan` (d₀), `dm1` (d₀ − 1) and `dp1` (d₀ + 1).
8 targets.

### K: Koblitz routing at all six §23 sizes, and the Koblitz rule held out at large sizes

Curves: k47 (a = 1, n = 47), k57 (0, 57), k41 (0, 41), k53 (0, 53),
k59 (1, 59) and k61 (0, 61). Arms:
- `rho`: `rho.signed_frobenius_strong`;
- `pipeline`: the `ic plan` arm, c₀ columns;
- `pipeline71`: `max(1, round(0.71·c₀))` columns, the w ≥ 0.3 rule;
- `descent`: `ic.koblitz_descent`, rule v8 defaults.

8 targets. k41, k47 and k53 repeat the routing study's sizes, with new seeds.

### N: the curve-free null, wider

`examples/ic_residual_curve_free_null.rs` runs at p16, p24 (`icv1-fp24-tm5099-a6f137c6`),
p32 (`icv1-fp32-t7319-90da43e9`) and p36, for B ∈ {2, 4, 16, 64}. Each case
has 128 solves on each carrier, seed 20261107.

### Design

Every session:
- target seed 20261105;
- 3 rounds + 1 warm-up, `alternate_measured`, measurement seed 20261106;
- L2, timeout 1,800 s.

Measures:
- `ecbench band --weights 0,0.01,0.03,0.1,0.3,1` with the exact sums;
- `ecbench compare` (`--weight` where w > 0) of each named pair;
- `ecbench verify --replay 12`.

## Predictions

Residual (R):
- R1. At every new size, `res4/plan` is below 1 at w = 0 with the interval
  below 1.
- R2. At p36 and p37, `S_1(res4)/S_1(plan)` is below 1.33, its 32-bit value.
- R3 (risky). At p37 the w = 1 point estimate of `res4/plan` is below 1. That
  is the extrapolated crossover near log2 r ≈ 35. Wherever the w = 0 and w = 1
  orders differ, the arms stay "not ranked".
- R4. Over all ten sizes (the six earlier ones and these four), the fitted
  ln GAE on ln r slope of res4 is in [0.45, 0.55] and the plan's in
  [0.60, 0.70].
- R5. The cheapest residual arm at w = 0 is `res2` or `res4` at every size.

Null (N):
- N1. At most 2 of the 16 null intervals exclude 1. 16 intervals at 95% admit
  about 0.8 by chance.

Weight rules:
- W1 (prime, held out). `S_1(m141)/S_1(plan) < 1`, with the interval below 1,
  on at least 3 of the 4 R curves. At w = 0 the ratio is at least 1 on at
  least 3 of 4.
- W2 (binary, held out). At w = 0 and at w = 1, `dm1/plan` and `dp1/plan` are
  each above 1 on at least 3 of the 4 B curves, i.e. `m* = 1` out of sample.
- W3 (Koblitz, held out at large sizes). `S_1(pipeline71)/S_1(pipeline) < 1`
  with the interval below 1 on at least 4 of the 6 K curves. At w = 0 the
  ratio is above 1 on at least 4 of 6.

Routing (K):
- K1. `descent/pipeline > 1`, with the interval above 1, at all six sizes in
  counted S.
- K2. `descent/rho` and `pipeline/rho` each grow with log2 r.

## What is proven in Lean (`lean/`, Lean core only, `leanprover/lean4:v4.34.1`)

1. `Band.lean`: priced cost is affine in the weight; the theorems are
   stated for all integers.
   - **(a) Crossing.** If arm A is strictly below B at w = 0 and strictly above
     at w = 1, there is an exact rational crossing weight w* in (0, 1). Below
     it A is cheaper, above it B is cheaper. So "not ranked across the band"
     is forced.
   - **(b) Slope monotonicity.** If arm i is optimal at weight w and arm j at
     weight v > w, then j's native count is at most i's.

   This is why the optimal base moves in whichever direction lowers native
   work. On prime curves a larger base needs fewer trials, each with one
   lookup and one probe. On Koblitz curves a smaller base has cheaper
   canonicalisation and probes.
2. `KeyedInvariance.lean`: a keyed-generic model. An algorithm sees elements
   only through `add`, `neg`, a key function and equality of handles.
   - **Theorem.** Its whole transcript (operation count, answer, every key it
     reads) is identical on two carriers joined by an injective homomorphism
     that preserves keys.
   - **Corollary.** The residual collector on E behaves exactly as on `Z/r`
     carrying E's keys transported along `k ↦ [k]G`. It uses no curve
     structure except through its key function.
   - **Scope.** The theorem says nothing about a random-key carrier. That is
     what the empirical null (N) tests: whether E's key function is special.
     The model claim about `ic_framework::residual` (it uses only those
     operations) is checked by reading the code, not by Lean.
3. `Certificates.lean`: generated by `scripts/lean_certificates.sh` from the
   committed band sums. Every point verdict of the four studies is restated
   in integers and checked by `decide`:
   - each `m*(w)` argmin, as a product of priced sums over curves (G is a
     geometric mean, so its argmin is the argmin of the product);
   - each band reversal;
   - each "A cheaper than B at w";
   - each exact crossing weight's bracket.
4. `AxiomAudit.lean`, together with a CI workflow modelled on
   `.github/workflows/conductor-gap-lean.yml`:
   - no `sorry` or `admit`;
   - only `propext`, `Classical.choice` and `Quot.sound` allowed;
   - a leanchecker replay.

Not proven in Lean:
- the bootstrap intervals, which are statistics;
- the correspondence between the Rust code and the Lean models, which is
  stated per module.

## Decision rule

- **Residual (R1–R5, N1).** Pass: the earlier disposition (generic, not ranked
  across the band) is confirmed on ten sizes.
  - If R3 holds, the README reports the size from which res4 is cheaper at
    every band weight. Only there, and only as a generic collision search,
    may it be ranked.
- **Weight rules (W1–W3).** If a rule fails out of sample, `ic-planner/3`'s
  rule for that family is withdrawn in a follow-up commit and reported.
- **Routing (K1).** If K1 fails at any size, the planner's routing for that
  size is revisited.
- **Class.** Engineering and measurement; no leaderboard row moves here.

## Inadmissible

- Changing curves, arms, targets or seeds after seeing a session.
- Dropping a run.
- Editing any method between sessions.
- Restating a certificate with numbers not taken from the committed band sums.
