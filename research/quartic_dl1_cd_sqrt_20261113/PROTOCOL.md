# PROTOCOL: a smaller double-large-prime factor base and a fixed-exponent square root in the plane-quartic index calculus (GHS-weak family)

Frozen 2026-11-13 in the commit that adds this file, **before any data of this study**. Not edited
afterwards; deviations go to the README. Two local pilots preceded it (disclosed below; not data).
Every result is an unverified assertion `[pcc:unverified]` until the validation gate admits it.

## Question

`research/quartic_n4v2_20261020` (N4 v2, frozen grid 2026-10-25) measured the double-large-prime arms
D, DL1 and DL (DL is the pre-registered headline on the leaderboard) with factor-base constant
`c_D = 2.0`, picked from {1.5, 2.0, 2.5, 3.0} in its pilot. This study asks whether two changes lower
the cold one-target cost of D, DL1 and DL on the **same instances**, with every logarithm verified:

1. **c rule** (lead `LEAD-J-dl1-percolation-fb-size`, `research/leads_20261112`): `c_D = 1.55` for
   `k >= 18` (`q >= 2^17`); `c_D = 2.0` (unchanged) for `k <= 17`. The frozen retry rule is kept:
   a failed attempt is charged and retried at `c_D + 0.25`.
2. **Square-root chain** (new): the per-pair square root `disc^((q+1)/4)` in `ic2.rs::find_relations_2lp`
   is computed by a fixed straight-line chain built once per run from runs of ones
   (`T[l] = T[p]^(2^(l-p)) * T[l-p]`, `T[1] = a`), used only when shorter than square-and-multiply
   (`src/ic2.rs`: `Chain`, `pow_fixed`). Same value, fewer counted `F_q` multiplications. No other
   code changes (`src/la2.rs`, `rho2.rs`, `solver2.rs`, `summ2.rs` are byte-identical to the frozen ones).

## Instruments

* `v2`: the frozen driver `research/quartic_n4v2_20261020/v2.rs`, built unchanged (its `ic` path and
  every computational source match the hashes in that study's ADDENDUM; `v2.rs` and `summ2.rs` only
  gained commands after the grid).
* `v3`: `src/v3.rs` = `v2.rs` with `#[path]` prefixes adjusted and a `chaincheck` command, plus the
  chain in `src/ic2.rs`.
* Both built with `rustc --edition 2021 -O` from the commit that freezes this protocol (or a later
  commit with identical `src/`, N4 sources and `plane_quartic_pic.rs`); binary SHA-256 recorded in
  `launch/image.json` and checked in the pod (`binary.sha256`).

## Instances and units

The frozen N4 v2 instance list: `q` = largest prime `< 2^k`, `q = 7 (mod 12)`; this study uses
`k = 12..22` (q = 4051 ... 4194271), seeds 1..5 for `k <= 16` and 1..3 for `k >= 17` (the frozen
grid's). Each unit is one `ic q seed D 3.0 c_D` invocation, which emits the D, DL1 and DL records
from one shared relation stage. Arms (`launch/units.tsv`, from `launch/make-units.sh`):

| arm | binary | c_D | sizes |
|:--|:--|:--|:--|
| BASE | v2 | 2.0 | k = 12..22 |
| CHAIN | v3 | 2.0 | k = 12..22 |
| CRULE | v2 | 1.55 | k = 18..22 (at k <= 17 the rule gives 2.0, so CRULE = BASE by definition) |
| BOTH | v3 | 1.55 | k = 18..22 (at k <= 17, BOTH = CHAIN by definition) |

116 units. Seeds, targets and factor bases follow from `(q, seed, c_D)` exactly as in the frozen grid.

## Measure

Primary: **cold M-eq per unit**, the frozen summarizer's formula: for every attempt record of the
unit (failed attempts included) and every phase, `mul + r_inv * inv + r_lmul * lmul`, summed; the
section-5a band adds `w * gae * (hash + cmp)` for `w = 0, 0.1, 1`. The constants `r_inv`, `r_lmul`,
`gae` per size are **the frozen grid's** (`research/quartic_n4v2_20261020/results/summary.json`,
`ic_size` records), so every number is on the leaderboard row's scale. Sensitivity: the same
summary with the pod's own `v2 calib` medians (three per size).

Wall and CPU seconds are recorded and reported as **diagnostic only** (command Jobs have no L2
admission; see Execution).

## Correctness gates (all must hold; a failure is reported, not repaired)

* **H0 (instrument identity).** Every BASE unit's counted fields (all `*.mul/inv/add/lmul/ladd/hash/cmp`
  and `s.*` statistics except timing) equal the frozen grid unit for the same `(q, seed)`
  (`results/grid-20261025/units.tar.gz`, artifact store).
* **H1 (chain changes only the count).** For every `(q, seed)` with BASE and CHAIN (and CRULE and
  BOTH), all record fields except `relations.mul` and timing are identical, and `k_found` equal.
  In-pod `v3 chaincheck`: 0 mismatches against `Fp::pow` on 200,000 inputs per size.
* **H2 (verification).** Every unit ends with a verified `kG = Q` in all three arms D, DL1, DL.

## Decision rule (fixed now)

For arm X in {CHAIN, CRULE, BOTH} and IC arm Y in {DL (headline), DL1, D}, per size, the paired ratio
`r_s = M-eq(X, s) / M-eq(BASE, s)` for each seed `s`. Reported: the geometric mean over seeds and
the min-max over seeds (3 seeds at `k >= 17` are too few for a meaningful bootstrap; counted costs
are deterministic per seed, so the spread is between instances, not noise).

* **Improvement at a size**: `max_s r_s < 1` at `w = 0`, `0.1` and `1`.
* **Headline**: BOTH / BASE for DL at `k = 18..22`. "Improvement" if it holds at every one of those
  five sizes; "partial" if at some; "none" otherwise.
* CHAIN is expected below 1 at every size by construction; it is reported per size with H1.
* Distance to rho (reported, not gated): IC / rho for the improved arm against the frozen grid's
  `rho_size` records (plain and negation map, the frozen calibration rule's exclusions kept, the k = 22
  supplement as in that study), at `w = 0, 0.1, 1`.

## Predictions (from the pilots, before data)

BOTH / BASE, DL: 0.68 (k = 18), 0.61 (k = 20), 0.56 (k = 22); DL1: 0.71, 0.61, 0.54. CHAIN / BASE, DL:
about 0.94, 0.92, 0.90. CRULE first-attempt failures at `k >= 18`: none expected (pilot: 0 of 15 at
c = 1.55, k = 18..22, seeds 1..5); a failure would be charged and is reported. k = 19 and 21 were not
piloted.

## Pilots (not data)

Local, 4-core container, L0, 2026-11-13: (1) c_D in {1.40, 1.50, 1.60, 1.75, 2.00}, k = 16, 18, 20,
seeds 1..3 (c = 1.40 and 1.50 failed on every unit; 1.60 succeeded on every unit; at k = 16 c = 1.60
gave no gain because the descent phase grew from 0.35 M to 3.5 M M-eq); (2) c_D in {1.55, 1.60, 1.70,
1.80, 2.00}, k = 18, 20, 22, seeds 1..5, all first attempts succeeded. A pilot c_D = 2.0 unit at
q = 1048507 seed 1 reproduced every counted field of the frozen grid unit. Records are not committed
as data; the rule above was chosen from them.

## Execution

Iso Lab EKS (AGENTS.md section 9a), command Job route: image = ubuntu:24.04 plus one layer (the two
binaries, `launch/job.sh`, `launch/units.tsv`, AWS CLI v2), published to `isolab/tasks` by digest
with `taskq/deploy/eks-karpenter/oci-layer-publish.py`; one Job, `cpu-spot`, 4 vCPU, 8 GiB, deadline
3,600 s, retries 0, submitted with `taskq/deploy/eks-karpenter/remote/launch-remote-command`. Units run
4 at a time, largest first; records sync to `s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod>/`.
Deviation declared now: command Jobs have no L2 admission, native replay or A/A gate, so seconds are
diagnostic; the measured quantity (counted operations) is deterministic per unit and independent of
the host, and H0 is the replay check against an independent earlier run.

Budget: ceiling USD 15 for this study (image publication, CodeBuild launcher, one c8a/m-class Spot
node about 1 h, S3), charged including failures. One submission; an infrastructure loss is
resumed by a new Job that skips completed unit files; no measurement is re-run to change a result.
