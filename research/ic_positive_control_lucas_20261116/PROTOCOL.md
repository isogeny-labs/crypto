# Positive control with the Cardano–Lucas splitting test: protocol

Frozen 2026-11-16 before any measured run (`frozen.sha256`). Status of every
number this study produces: `[pcc:unverified]` until a validator replays it.

## 1. Question

The genus-3 positive control (`research/ic_positive_control_20261112`, below
"the 11-12 control") found its double-large-prime arm C under the matched
negation rho R at `w = 0` but 5.8× above at `w = 0.1` and 31.5× above at
`w = 1` (p = 65537). Its diagnosis: about 362 native `F_p` multiplications
per relation trial, mostly `x^p mod u` and Cantor–Zassenhaus roots.

Round 4 item 01 (`research/ic_improve4_20261015/01-genus3-relations`, C1)
already built a field-level test with identical decisions in a different
instrument (GHS genus 3, no rho arm). This study ports C1 into the positive
control's instrument (`ic_control.rs`) and asks how far it moves C against
the matched rho on the §5a band. It is a component port with a matched
reference, not a new algorithm.

## 2. Arms (same binary, same instances, same seeds)

| arm | splitting test | root finding | walk |
|---|---|---|---|
| C | `x^p mod u` (baseline) | Cantor–Zassenhaus (rng) | baseline |
| L | Cardano–Lucas (Stickelberger + `V_{n/3}(y+1/y) = 2`) | Cantor–Zassenhaus (rng) | identical to C by construction |
| M | Cardano–Lucas | Cardano cube root via `V_e`, else CZ | own (root finder draws no rng) |
| R | negation-map rho on the order-`r` subgroup | – | – |

Factor base, large-prime rule (`nb = ceil(2 p^{2/3})`), Wiedemann and every
counter are unchanged from the 11-12 control. Arms A and B are not run.

## 3. Instances and seeds

As the 11-12 control: primes the first prime ≥ `2^7 … 2^16`; `curve_seed0 = 1`,
`max_h = 64`; curves per prime 5 (≤ 2053), 3 (4099, 8209), 2 (16411, 32771),
1 (65537); 4 seeds per curve and arm, `1000 (s+1) + curve_index`.

Command (`threads 3`, shared 4-vCPU host; counts are deterministic in the seed,
so the host does not enter any count):

```
target/release/ic_positive_control run \
  --primes 128,256,512,1024,2048,4096,8192,16384,32768,65536 \
  --curves 5 --curves-map 4099=3,8209=3,16411=2,32771=2,65537=1 \
  --seeds 4 --arms CLMR --threads 3 \
  --json research/ic_positive_control_lucas_20261116/run.json
```

## 4. Gates (checked before any comparison is read)

- G1 reproduction: every C and R row equals the 11-12 control's row for the
  same (p, curve, seed) in `group_ops`, `rel_mul`, `la_mul`, `trials`,
  `relations` and `certified`. A mismatch stops the study.
- G2 pairing: every L row equals its C row in `trials`, `splits`,
  `relations`, `group_ops`, `la_mul`, `la_add` and the recovered log, and has
  `rel_mul` strictly lower.
- G3 every log certified in every arm.
- The exhaustive oracle tests (`lucas_oracle_matches_frobenius_on_every_cubic`,
  `lucas_oracle_keeps_the_trajectory_and_cuts_relation_work`) pass at the
  frozen commit.

## 5. Metrics

- Component: relation-phase native multiplications per trial, per arm and p;
  paired ratio L/C (exact pairing) and M/C (paired on curve and seed).
- End to end: arm total ÷ R total at `w = 0, 0.1, 1` GAE per native
  multiplication (§5a), per p, with the 11-12 control's two-stage bootstrap
  (`uncertainty.py`, curves then seeds, 2000 resamples, 95%).
- Share of native multiplications in the linear algebra, per arm at p = 65537.

## 6. Predictions (written before data)

- P1 per-trial relation multiplications at p = 65537: C ≈ 362 (reproduced);
  L in [140, 230]; M in [70, 150].
- P2 L/C relation multiplications ≤ 0.65 at every p ≥ 1031.
- P3 M/R at `w = 0.1`, p = 65537, in [2.5, 4.5] (C was 5.82); M/R at `w = 1`
  in [12, 25] (C was 31.5).
- P4 no arm is below R at all three prices at any p: the §5a verdict stays
  **unresolved**. (A pass of P4 is the expected, unexciting outcome.)
- P5 at p = 65537 the linear algebra holds ≥ 45% of M's native
  multiplications, so it becomes the next lever.

## 7. Decision rule

- The port **advances the component** if G1–G3 hold and L/C relation
  multiplications ≤ 0.8 at every p ≥ 1031 (paired, exact).
- The end-to-end distance to rho is reported per price as M/R and L/R with
  intervals, as the distance remaining, not as pass/fail.
- If any arm is below R at all three prices at some p (P4 fails), the result
  goes to red-team and validator review before any wording beyond
  `[pcc:unverified]`.

## 8. Disclosed deviations from the lab defaults

- Execution route: the lab default for x86 benchmarks is `isolab-eks`. This
  container has no AWS CLI, boto3 or IsoLab client, so the run is local on the
  shared host, as the 11-12 control was. Only operation counts enter the
  verdict; they are deterministic in the seed and host-independent. Wall
  times are recorded and not used.
- The S3 legs of the resume checkpoint and the bench record cannot be pushed
  from this container; the record stays local and is committed, with raw
  files hashed.
- No Conductor client is installed here; scope is the files listed in
  `frozen.sha256`, plus one unrelated compile fix on `main`
  (`koblitz_index_calculus.rs`, `lift_x_packed`), which is required to build.
