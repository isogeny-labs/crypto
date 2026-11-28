# Results: lines through P0 in quartic Diem `2lp-sfb-sge-nofb`

Every number here is `[pcc:unverified]`. They come from one shared host and planted targets, and no validator report exists yet.

- **Protocol.** PROTOCOL.md was frozen in 7f67f384ece, before any grid run, together with `seeds.json`, `analyze.py` and the run scripts. `frozen.sha256` still verifies.
- **Binary.** `genus_ladder.p0`, sha256 `283c1510…23723cd`, built from 7f67f384ece (`binary.sha256`, `source.sha256`).
- **Runs.** 2026-11-28, 03:45–03:52 UTC (`run_main.sh`) and 03:52–04:21 UTC (`run_131101.sh`). `raw/`, hashed in `data.sha256`. Bench record `benchmarks/records/20261128T0345Z-quartic-diem-p0lines.json`.

## Integrity

- 360 runs (120 per arm), all certified, with `recovered == t_secret` and the same log in all three arms.
- Every (curve, seed) has all three arms.
- **C0:** the `nofb` rows for seeds 1–4 equal the parent's on all 24 fields in 80 of 80 cases.
- Every `p0`/`p0i3` row holds P0-line relations, and no `nofb` row does. These counts are means over all seeds:

  | p | P0 lines | P0-line relations |
  |---|---|---|
  | 4099 | 193 | 92 |
  | 16411 | 385 | 192 |
  | 65537 | 769 | 378 |
  | 131101 | 1087 | 555 |

## Decision (frozen rule, primary panel = held-out seeds 5–6): **INCONCLUSIVE**

Pooled paired GAE_w ratio, arm / nofb, with cluster boot95:

| w | p0 / nofb, seeds 5–6 | p0 / nofb, all seeds 1–6 | p0i3 / nofb, seeds 5–6 | p0i3 / nofb, all seeds 1–6 |
|---|---|---|---|---|
| 1 | 0.990 [0.934, 1.045] | 0.951 [0.921, 0.981] | 0.990 [0.941, 1.038] | 0.953 [0.930, 0.976] |
| 0.1 | 0.990 [0.933, 1.045] | 0.951 [0.921, 0.982] | 0.990 [0.943, 1.037] | 0.953 [0.930, 0.976] |
| 0 | 1.018 [0.861, 1.181] | 0.937 [0.873, 1.002] | 0.902 [0.781, 1.035] | 0.784 [0.731, 0.837] |

- **Verdicts.** The primary verdict for `p0` is INCONCLUSIVE: the upper bounds are above 1, and the w = 1 lower bound (0.934) is below the 0.98 kill line. The secondary verdict for `p0i3` is also INCONCLUSIVE.
- **All seeds.** On all six seeds, the `p0` interval lies below 1 at w = 1 and w = 0.1. That panel is secondary and includes the four seeds that suggested the lead, so it decides nothing.

Per prime, p0 / nofb at w = 1:

| p | seeds 5–6 | seeds 1–6 |
|---|---|---|
| 4099 | 1.050 [0.988, 1.141] | 1.013 [0.997, 1.031] |
| 16411 | 0.999 [0.854, 1.104] | 0.897 [0.823, 0.977] |
| 65537 | 0.911 [0.811, 1.013] | 0.950 [0.899, 0.999] |
| 131101 | 1.004 [0.876, 1.148] | 0.946 [0.879, 1.017] |

## Second attempts

Runs needing more than one attempt:

| arm | seeds 5–6 | seeds 1–6 | per prime, seeds 1–6 (4099 / 16411 / 65537 / 131101, of 30 each) |
|---|---|---|---|
| nofb | 10 / 40 | 25 / 120 | 0 / 9 / 7 / 9 |
| p0 | 8 / 40 | 12 / 120 | 1 / 1 / 4 / 6 |
| p0i3 | 10 / 40 | 19 / 120 | 1 / 2 / 5 / 11 |

The held-out seeds gave `nofb` few failures to remove. At p = 16411 `nofb` needed a second attempt in 1 of 10 runs on seeds 5–6, against 8 of 20 on seeds 1–4.

## Preregistered predictions

1. **Failed.** On the primary panel, `p0` second attempts are 8/40 against 10/40 for `nofb`, not at most half. On all seeds they are 12/120 against 25/120, which is about half.
2. **Failed.** The pooled p0 / nofb GAE₁ is 0.990, outside [0.75, 0.97].
3. **Held.** p0i3 / nofb is 0.990, inside [0.75, 1.02], and `p0i3` needs more second attempts than `p0` (10 against 8 on seeds 5–6, 19 against 12 on all seeds).

## Mechanism (exploratory, descriptive, not part of the decision: `explore.py` → `explore.json`)

GAE₁ ratio p0 / nofb by attempt pattern, all seeds, geometric mean (n):

| p | both 1 attempt | nofb 2, p0 1 (rescue) | both ≥ 2 | nofb 1, p0 2 |
|---|---|---|---|---|
| 4099 | 0.999 (29) | — | — | 1.555 (1) |
| 16411 | 1.027 (21) | 0.620 (8) | 1.002 (1) | — |
| 65537 | 0.993 (23) | 0.628 (3) | 1.004 (4) | — |
| 131101 | 1.008 (20) | 0.616 (4) | 0.933 (5) | 1.570 (1) |

- **Overhead.** The P0 lines themselves cost almost nothing: ≤ 3% on pairs where both arms need one attempt.
- **Rescues.** A rescued run costs about 0.62× its `nofb` pair. 15 of the 25 `nofb` multi-attempt runs were rescued.
- **Not rescued.** The share of unrescued failures grows with p (5 of 9 at 131101). That fits the second mechanism found in the diagnosis: factor-base columns beyond the pair-enumeration frontier that occur only in decomposition rows. The P0 lines do not touch it.
- **New failures.** Two runs that `nofb` finished in one attempt needed two with `p0`, because the P0 lines change the relation stream.
- **Net effect.** It is set by how many rescuable failures a panel contains. On two held-out seeds × 5 curves per prime, that count is small and noisy.

## IC / ideal negation rho (all seeds, boot95)

At w = 1:

| p | nofb | p0 | p0i3 |
|---|---|---|---|
| 4099 | 8.76 | 8.88 | 8.55 |
| 16411 | 4.96 | 4.45 | 4.41 |
| 65537 | 2.84 | 2.70 | 2.67 |
| 131101 | 2.02 [1.85, 2.20] | 1.91 [1.82, 2.00] | 2.04 [1.93, 2.14] |

- Every arm is UNRESOLVED at every prime: below rho at w = 0 (about 2·10⁻⁴ at 131101), and at w = 0.1 (0.19–0.20 at 131101). At w = 1 it is above rho.
- The distance remaining at 131101, w = 1, is 1.9–2.0×. The study does not change the verdict.

## What this settles and what it does not

- **Settled.**
  - Lines through P0 cost almost nothing.
  - They remove the base-point-shift cause of undetermined first attempts.
  - They roughly halve second attempts across all seeds.
- **Not settled.** A held-out cost improvement. The frozen primary panel gives INCONCLUSIVE.
- **Next lever.** The remaining, unrescued failures point to the frontier columns: points beyond about 55% of the factor base that the pair enumeration has not reached when the first attempt fires. Two candidate fixes:
  - order the pair enumeration so that it covers the whole factor base early;
  - accept a decomposition only when its factor-base points are covered.

  Either needs a new protocol, with a held-out panel large enough for the rescue rate to show (more seeds per curve).

Figure: `fig_p0lines.svg` and `fig_p0lines.png`. Report: `reports/run-20261128T0345Z/report.pdf`.
