# PROTOCOL: exact block-Lanczos gram reuse in the plane-quartic double-large-prime IC (counted cost)

Frozen in the commit that adds this file, before any unit of this study runs. Not edited afterwards (the commit
hash is recorded in README.md). Every number is an unverified assertion `[pcc:unverified]`.

## Hypothesis

H_blr. Two exact identities of block Lanczos remove two n*k-sized products per iteration from the linear algebra
of the DL record without changing any other counted work or the solution:

1. `MV_{i-1}^T MV_i` equals the columns `sel_used` of the same iteration's full gram `V_full^T M V`
   (M-orthogonality of the Lanczos vectors); it replaces `gram(mvpr, mv)` when the dimensions match.
2. For iterations i >= 2 without narrowing at iteration 0, `V_i^T V_0` follows from the recurrence
   `V_full = MV_{i-1} - V_{i-1} c1 - V_{i-2} c2` (as `V_{i-1}^T M V_0 = 0`); it replaces `gram(v, v0)`.

Both fall back to the direct grams otherwise. Change set: `src/la2.rs` `block_lanczos` only (diff against
`../ic_gw_fasthash_20261114/src/la2.rs`; all other source files are byte-identical). Pilot (main session,
not data): DL cold M-eq BLR/REF 0.82, 0.815, 0.813 at k = 18, 20, 22 seed 1 (additions free), 0.88-0.89 with additions
at 1 M-eq; every other counted field identical, same bl_iters.

## Arms (binaries, built `(cd src && rustc --edition 2021 -O v3.rs --cfg combo -o OUT)`, rustc 1.97.0)

* **REF**: `../ic_gw_fasthash_20261114/src` (COMBO build). Its recorded sha256 must be
  `00355034924e06661aed665f433063524c2e3964839c3ca671d54d64aaaa19a9`.
* **BLR**: this study's `src/`. sha256 recorded in the results (`binary.sha256`) and README.md.

Unit: `<bin> ic <q> <seed> D 3.0 1.7` (c_D = 1.7, block width 4 default).

## Units (units.tsv from make-units.sh; 60 units)

q in {262111, 524287, 1048507, 2097091, 4194271} (k = 18..22), seeds 1-6 (1-3 match the earlier studies, 4-6 are
fresh), arms REF and BLR: 30 (q, seed) pairs, 60 units, each unit one process. Counted operations are
host-independent, so there is no pass structure, no A/A arm and no lock. Up to 3 units run concurrently
(`xargs -P 3`); this is acceptable only because the metric is counted work; CPU and wall seconds in the records
are diagnostics at most (shared host, concurrent units) and carry no claim.

## Route deviation

Default is Iso Lab EKS Spot. The account's monthly budget is exceeded
(`../quartic_dl1_cd_sqrt_20261113/launch/BUDGET-HOLD.md`); the owner chose local runs. This is an L0 run on the
session container (4 vCPU). No replay or matched-rho admission: unverified, no leaderboard row moves.

## Gates (all must hold; else the unit set is invalid for the claim)

* **G1** every counted field (all keys except those ending in `wall`/`cpu`, `load1`, `s.controls*`) of every `ic`
  record, in order, identical between REF and BLR for every (q, seed), except `linear_algebra.lmul` and
  `linear_algebra.ladd` of the DL record (the record count per arm must match too).
* **G2** identical `s.bl_iters` and `s.bl_narrowed` (DL record) between REF and BLR.
* **G3** every `ic` record `ok` and `k_found == k`; every unit exit code 0 (the `unit` record).
* **G4** REF reproduces the committed COMBO records of `../ic_gw_fasthash_20261114` (raw tarball
  `runs-fasthash-20261114.tar.gz`, `units/p1_COMBO_*`, `p2_COMBO_*`) for seeds 1-3 at all five sizes: counted fields
  of every `ic` record equal.

A failing unit is rerun once (both runs kept) and reported; no data are dropped for being unfavourable.

## Metric

DL-record cold M-eq with the frozen constants `r_inv`, `r_lmul` per size from
`../quartic_n4v2_20261020/results/summary.json` (`kind == ic_size`, `arm == DL`), model exactly as
`../quartic_dl1_cd_sqrt_20261113/combo/analyze.py`:
`sum over the records of the unit (attempts) and phases fb, relations, matrix, linear_algebra, log_recovery, descent, verify of
mul + r_inv*inv + r_lmul*lmul + w_add*(add + ladd) + w_h*gae*(hash + cmp)`,
with `w_add` in {0, 0.5, 1} and, as the section 5a sensitivity, `w_h` in {0, 0.1, 1} (`gae` per size from the same
file). Primary cell: `w_h = 0`. Ratio BLR/REF per (q, seed). Reported also: DL1 and D (expected exactly 1) and the
ratio of `linear_algebra.lmul` of the DL record.

## Decision rule (fixed now)

**SUPPORTED** (an improvement over REF) iff the DL ratio is < 1 at every (q, seed) (30 cells) and every
`w_add` in {0, 0.5, 1} (primary `w_h = 0`); reported additionally whether it also holds for every `w_h` in
{0, 0.1, 1}. Otherwise NOT SUPPORTED. Requires G1-G4. Headline: per size the geomean and max of the DL ratio over the six
seeds at each `w_add`.

## Gap to the matched negation-map rho (distance remaining, not a gate)

At k = 22 the frozen negation-map rho mean cold M-eq are the `gae_band` rho values of
`../quartic_dl1_cd_sqrt_20261113/combo/validation/VALIDATION.json` (w = 0, 0.1, 1:
3.795915e11, 4.909138e11, 1.4928140e12). The IC side is the DL M-eq mean over seeds 1-3 at q = 4194271 with
`w_add = 0` and hash+cmp at `w*gae`, as that file. REF must first reproduce that file's `ic` numbers (combo
arm) to 1e-9 relative; if it does, the BLR gap is reported as rho/IC at the three w; otherwise only the ratio
change is reported.

## Predictions (before data)

DL ratio 0.80-0.84 at `w_add = 0` and 0.87-0.90 at `w_add = 1`, flat in k; D and DL1 exactly 1.000; linear-algebra
lmul ratio about 0.75-0.80; gates hold.

## Analysis

`analyze.py RESULTS_DIR` writes `analysis.json` and `tables.md`; `plot.py` and `REPORT.typ` render them.
