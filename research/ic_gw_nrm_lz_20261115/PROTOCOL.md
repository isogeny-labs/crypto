# PROTOCOL: normalised-point multiplications and Lanczos dot-product reuse in the plane-quartic double-large-prime IC (counted cost)

Frozen in the commit that adds this file, before any unit of this study runs. Not edited afterwards (the commit
hash is recorded in README.md). Every number is an unverified assertion `[pcc:unverified]`.

## Hypotheses

* **H_nrm** (env `NRM=1`, `src/ic2.rs` `find_relations_2lp`). Factor-base and residual points are normalised with
  first coordinate 1, so the products by that coordinate (in g3, g1, the `lin()` x-coordinate, the normaliser) and by
  b0^2 in g2 are skipped, and 4*g1 is two additions. Same relations; only `relations.mul`/`relations.add` change.
* **H_lz** (env `LZ_REUSE=1`, `src/la2.rs` `lanczos`, the scalar Lanczos of arms D and DL1). `v^T v_prev = s` (local
  A-orthogonality) and `w_i^T y` by the three-term recurrence (global A-orthogonality to w_0 = y; scalar Lanczos never
  narrows). Same iterates; only D and DL1 `linear_algebra.lmul`/`ladd` change.

With neither variable set the candidate binary must reproduce REF exactly. `src/` is
`../ic_gw_bl_reuse_20261115/amendment-1/src` (BLR1) plus these two env-gated changes (`v3.rs` `#[path]` includes lose one
`../`). Pilot (main session, not data): q = 262111 and 1048507 seed 1: relations.mul 0.878/0.884; DL M-eq 0.959 at k = 20
(additions free); DL1 0.844 with both.

## Arms (binaries built `(cd src && rustc --edition 2021 -O v3.rs --cfg combo -o OUT)`)

* **REF**: BLR1 built from `../ic_gw_bl_reuse_20261115/amendment-1/src`; sha256 must start `e236855d7fef`.
* **A0**: CAND (this `src/`), no env. **NRM**: `NRM=1`. **LZ**: `LZ_REUSE=1`. **BOTH**: both.

Unit: `[env] <bin> ic <q> <seed> D 3.0 1.7`. q in {262111, 524287, 1048507, 2097091, 4194271} (k = 18..22), seeds 1-6:
30 (q, seed) cells x 5 arms = 150 units (units.tsv from make-units.sh). Counted operations are host-independent: no
pass structure, no timing claim. Up to 3 units run concurrently (`xargs -P 3`; counted only).

## Route deviation

Default is Iso Lab EKS Spot; the account budget is exceeded (`../quartic_dl1_cd_sqrt_20261113/launch/BUDGET-HOLD.md`)
and the owner chose local runs: L0 on the session container. No replay or matched-rho admission; no leaderboard row moves.

## Gates (all must hold)

* **G0** A0 identical to REF in every counted field (all keys except those ending `wall`/`cpu`, `load1`, `s.controls*`) of every `ic` record, in order.
* **G1** NRM differs from REF only in `relations.mul`/`relations.add` (all arms D, DL1, DL); LZ only in D and DL1
  `linear_algebra.lmul`/`ladd`; BOTH only in the union. Record counts equal.
* **G2** every `ic` record `ok`, `k_found == k`, every unit exit code 0.
* **G3** all `s.*` relation/solver statistics (except `s.controls*`) identical across the five arms.

A failing unit is rerun once (both kept) and reported; nothing is dropped for being unfavourable.

## Metric

Cold M-eq per arm D, DL1, DL, frozen constants `r_inv`, `r_lmul`, `gae` per size from
`../quartic_n4v2_20261020/results/summary.json` (`kind == ic_size`, `arm == DL`), model exactly as
`../ic_gw_bl_reuse_20261115/analyze.py`: sum over the unit's records and phases fb, relations, matrix, linear_algebra,
log_recovery, descent, verify of `mul + r_inv*inv + r_lmul*lmul + w_add*(add + ladd) + w_h*gae*(hash + cmp)`,
`w_add` in {0, 0.5, 1} (primary `w_h = 0`), sensitivity `w_h` in {0, 0.1, 1}. Ratios NRM/REF, LZ/REF, BOTH/REF per (q, seed).

## Decision rule (fixed now)

BOTH is an **improvement** for an arm (DL headline; DL1 and D reported) iff its ratio is < 1 at every (q, seed) (30
cells) and every `w_add` in {0, 0.5, 1} (`w_h = 0`) and G0-G3 hold; additionally reported whether it holds for every `w_h`.
NRM-only and LZ-only are reported the same way (descriptive).

## Gap to the matched negation-map rho at k = 22 (distance remaining, not a gate)

Frozen rho band: `rho_reference.gae_band` of `../ic_gw_bl_reuse_20261115/validation/VALIDATION.json` (w = 0, 0.1, 1).
IC side = mean over seeds 1-3 at q = 4194271, `w_add = 0`, hash+cmp at `w*gae`. REF must first reproduce that file's
`ic` values (8.44014769056e8, 1.0140119044561e9, 2.5439861230561e9) to 1e-9 relative; else only ratio changes are
reported. Gap reported for BOTH DL and DL1.

## Predictions (before data)

NRM relations.mul ratio 0.87-0.89; DL ratio about 0.96 (w_add = 0), somewhat higher with additions priced; LZ changes
only D/DL1; DL1 BOTH about 0.84 at w_add = 0; gates hold.

## Analysis

`analyze.py RESULTS_DIR` writes `analysis.json`, `tables.md`; `plot.py` and `REPORT.typ` render them.
