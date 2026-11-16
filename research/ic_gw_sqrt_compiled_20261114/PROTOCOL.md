# PROTOCOL: compiled fixed-exponent square root (lane "chain")

Frozen 2026-11-14 in the commit that adds this file, before any data of this study (the commit hash is
recorded in README.md, not here). Not edited afterwards. Every result is an unverified assertion
`[pcc:unverified]`.

## Hypothesis

In the plane-quartic double-large-prime index calculus on the GHS-weak family `E0/F_{q^3}`
(`../quartic_dl1_cd_sqrt_20261113/`), the per-pair square root of `find_relations_2lp` is an interpreted
op list plus hardware u64 division. Regrouping the same ops into segments `x -> x^(2^n) [* r]` and
reducing with an exact Barrett step (`FastChain`, q < 2^32) leaves every counted operation unchanged and
lowers the relation stage's CPU time. Pilot (wf_f57f915b-f9f, L0, not a claim): CPU 0.72-0.92x. A/A noise
on `relations.cpu` about 0.1%, on total CPU up to about 7%.

## Arms (identical instances, c_D = 1.7, `ic q seed D 3.0 1.7`)

* **REF**: `v3` of the published image (`../quartic_dl1_cd_sqrt_20261113/launch/image.json`), sha256 must
  start `de500bc1`.
* **CHAIN**: `v3c`, built from `src/` here (`rustc --edition 2021 -O v3.rs`; `ic2.rs` differs from the
  study's only by the `FastChain`/`pow_fast` block and the one call in `find_relations_2lp`; `v3.rs` adds the
  diagnostic subcommands `fastcheck`, `sqrtbench`). Binary sha256 is recorded in README.md and
  `runs/*/binary.sha256`.

## Units

Sizes q in {262111, 524287, 1048507, 2097091, 4194271} (k = 18..22), seeds 1-3, both arms: 30 units per
pass. Pass 1 order within a (q, seed) pair: REF then CHAIN for odd seeds, CHAIN then REF for even seeds;
pass 2 reverses every pair. A/A: REF twice more (labels AA1, AA2, adjacent) at q = 2097091 and 4194271,
seed 1. 64 units in `units.tsv` (from `make-units.sh`), run one at a time in file order: all of pass 1,
pass 2, then A/A. Every unit runs under the host lock `flock .../wf/timing.lock` (shared with the other
parallel lanes), one unit per acquisition; `load1` is recorded inside the lock (`run-local.sh`).

## Gates

* **G0 smoke (before pass 1, not analysed):** `v3c ic 262111 1 D 3.0 1.7` has every counted field equal to
  `v3`'s (as `fastcheck` and the scratch comparison showed).
* **G1:** every counted field (all keys except those ending in `wall`/`cpu`, `load1`, `s.controls*`, over
  all `ic` records and attempts) is identical in every REF/CHAIN pair, between REF pass 1 and pass 2, and
  among the A/A units and REF pass 1. Any mismatch fails the lane.
* **G2:** every unit has rc 0 and verifies `k_found == k` for D, DL1, DL. Failing units are reported, never dropped.

## Metrics

Primary: `relations.cpu` summed over the three IC arms (D, DL1, DL) and over all attempt records of a
unit; per (q, seed) the ratio CHAIN/REF within each pass, then the mean of the two passes (cell ratio).
Secondary: total CPU (sum of the seven phases) per arm DL, DL1, D, same pairing; relations.cpu of the DL
arm alone. A/A floor `f = max over the two A/A sizes of |AA2/AA1 - 1|` on summed `relations.cpu`; the REF
pass-2/pass-1 ratio is reported as a second noise indicator. Counted work is identical (G1), so M-eq is not
computed; the frozen per-size constants are not needed. Wall and CPU are L0 diagnostics (shared 4-vCPU
container); the primary metric is CPU at identical counted work, so the L0 caveat is stated, not removed.

## Decision rule (fixed now)

Let `m` be the maximum cell ratio over the 15 (q, seed) cells.

* **Improvement in CPU (L0):** G1 and G2 pass, `m < 1` and `m < 1 - f`.
* **Below 1 but not separated from noise:** `m < 1` and `m >= 1 - f`.
* Otherwise: no robust improvement. A failing G1 fails the lane regardless.

Sensitivity (reported, not the headline): drop every cell with a unit whose `load1` at lock acquisition
exceeded 2.0.

## Prediction (before data)

Cell ratios 0.72-0.92 on `relations.cpu`; A/A floor below 0.01; total CPU for DL 0.85-0.97 (relations is
about 20-35% of the total at these sizes).

## Route deviation

Default route is Iso Lab EKS Spot. The owner chose local runs because the Iso Lab EKS monthly budget is
exceeded (`../quartic_dl1_cd_sqrt_20261113/launch/BUDGET-HOLD.md`). Runs are on the session container
(L0), alongside two other lanes that are serialized with this one by the shared host lock.

## Exclusions

None from the headline. No unit is rerun after seeing data except for rc != 0 or a failed verification,
which would be reported with the rerun alongside.
