# PROTOCOL: incremental factor-base growth instead of a charged restart (plane-quartic double-large-prime IC, GHS-weak family)

Frozen 2026-11-14 in the commit that adds this file, before any data of this study. Not edited afterwards (the
commit hash is recorded in `README.md`). Every result is an unverified assertion `[pcc:unverified]`; no leaderboard
or `benchmarks/README.md` row moves (validation gate). Scope: `E0/F_{q^3}`, `#E0(K) = 4 ell`, family D with the
double-large-prime stage, `D`, `DL1`, `DL` arms of the shared run, k = 18..22.

## Question

Task "J2: incremental factor-base growth instead of charged restart". Below the robust c_D (1.7) the first attempt
fails sometimes, and the frozen driver then restarts from scratch (all work of the failed attempt charged). The
candidate keeps the stored relations, grows the factor base by 5% at a time, enumerates only the new pairs, and
triggers growth when the mean cycle-row weight exceeds W (heavy rows), instead of waiting for a failure. A pilot
(workflow wf_f57f915b-f9f, L0, not a claim) found no robust counted gain and 1.16-1.52x CPU. This is the
confirmatory, pre-registered run of that null, so the task can be closed either way.

## Documentation step (existing records, no new runs): do earlier studies show retries at c_D = 1.7?

`audit_retries.py` (run as `python3 -I`) scans every committed `ic` record at q >= 262111 in
`../quartic_dl1_cd_sqrt_20261113/runs`, `../quartic_dl1_cd_sqrt_20261113/cd_sweep/runs`,
`../quartic_dl1_cd_sqrt_20261113/combo/runs` and `../quartic_n4v2_20261020` (loose files and the tar archives; the
store-managed files that matter, `grid-20261025/units.tar.gz` and `runs-local-20261113.tar.gz`, are present and match
their `.artifacts.json` SHA-256, so no `artifacts.py pull` was needed; the stubs absent are only `*.png`). It writes
`retries.json`. Result (before the new data): 675 (unit, IC arm) records at q >= 262111; **no D, DL1 or DL record at
c_D >= 1.7 has a second attempt or a failed attempt**. The only 8 records with retries are 6 pilot records at
c_D = 1.5 (q = 262111, seeds 101, 102: D, DL1, DL, retried at 1.75) and 2 records of the S/L0 families at q = 262111,
seed 2 (3.0 retried at 3.4). So at the REF setting the charged-restart cost is zero at every k = 18..22 and
seed in the frozen data, and growth can only help by lowering the starting c_D below 1.7 and paying less than a restart
(or avoiding it through heavy-row growth).

## Arms and units

Binaries: REF = published `v3` (`de500bc154f3074359badf434255605ae103b974f4ac2b00159fc84bd3a76bb6`, the
`../quartic_dl1_cd_sqrt_20261113` image, chain on); candidate `v3g` built from `src/` (`rustc --edition 2021 -O v3.rs`,
`v3.rs` = the study's `v3.rs` plus the growth driver `solver2::run_family_grow` and `ic2::FbSampler/RelStore`; its sha256 is
in `runs/*/binary.sha256`). Instances: the frozen N4 v2 instances, seeds 1-3, q = 262111, 524287, 1048507, 2097091,
4194271 (k = 18..22), `ic <q> <seed> D 3.0 <c_D> ...`.

| arm | binary | command tail |
|:--|:--|:--|
| REF | v3 | `D 3.0 1.7` (charged restart, no growth) |
| G160 | v3g | `D 3.0 1.6 4.0 4 0.05 12 50` (start c_D 1.6, +5% steps, at most 12 attempts, grow while mean cycle-row weight > 50 or on failure) |
| G155 | v3g | `D 3.0 1.55 4.0 4 0.05 12 60` (start c_D 1.55, W = 60) |

45 units (3 arms x 5 sizes x 3 seeds), one pass, `units.tsv` from `make-units.sh`: largest size first, within a
(q, seed) the three arms in rotated order (rotation by seed). Each unit runs alone under the host lock
(`run-local.sh`), `load1` recorded inside the lock. A unit with rc != 0 is rerun once in the same slot and both
attempts are logged; nothing is excluded after the fact.

## Measures

Counted cost of an IC arm (D, DL1, DL) in a unit: sum over **all attempt records** of that arm and all phases (fb,
relations, matrix, linear_algebra, log_recovery, descent, verify) of
`mul + r_inv(q) * inv + r_lmul(q) * lmul + w * (add + ladd) + h * hash`, with the frozen per-size constants
`r_inv, r_lmul` of `../quartic_n4v2_20261020/results/summary.json` (never the measured `ratios` record). `cmp` is never
priced; `hash` is priced at h in {0, 0.1, 1} M-eq per operation (the growth path repeats hash work, as the reviewer
required); w in {0, 0.5, 1}. The REF arm is the same function on the same instance.

* **Primary metric:** `rho_{G}(q, s; w, h) = cost(G, DL) / cost(REF, DL)` per cell (q, seed), every attempt of the DL arm summed.
* **Secondary:** total CPU of the DL arm over all attempts, G / REF per cell (L0 diagnostic: wall and CPU are
  diagnostics of a shared container; pilot A/A noise 0.1% on `relations.cpu` and up to 7% on total CPU); DL1 and D counted
  ratios; per-phase ratios; counted hash operations; attempts per unit; distance of DL to the frozen matched plain rho
  (frozen `summary.json`, k = 22 from `summary_supp.json`, rho at its w = 0 and w = 1 prices; additions and hash free for
  the IC side, so the quotient is the most favourable to IC; reported as distance remaining, not as a gate).

## Gates (every one must hold for the verdict to be valid)

* **G-id:** `v3g` counted output on q = 262111 seed 1 (G160 command) equals the scratch candidate's byte for byte
  on every counted field (`smoke/`, done before this freeze: 6 / 6 records identical).
* **G-A/A:** REF's counted fields equal the committed `combo/runs` B170 units (same binary v3, same c_D 1.7, same
  instance) for D, DL1, DL at every (q, seed): 45 comparisons, 0 mismatches.
* **G-ok:** every unit rc 0 and, for D, DL1, DL, the last record `ok` with `k_found == k`.
* **G-load:** load1 at lock acquisition <= 2.0 (reported; not a verdict condition for counted metrics).

## Decision rule (fixed now)

For each growth arm G separately, on the DL arm: **G is an improvement over REF only if `rho_G < 1` at all 15 cells
(5 sizes x 3 seeds) at every addition price w in {0, 0.5, 1} and at both hash prices h = 0 and h = 0.1**
(max over cells of the per-seed ratio < 1 on that 3 x 2 grid); and all gates hold and all 45 units are complete.
Otherwise G is **null**. The study's verdict: growth is an improvement if at least one arm is; otherwise **null**
(closing the task as "no gain at the REF setting"). h = 1, CPU ratios, DL1 and D, and per-phase tables are reported but
not decisive. The strict per-cell rule has no multiplicity correction because a single violating cell decides; the
measurement is deterministic in counted work (same instance, same rng), so counted ratios have no sampling noise
across repeats, only across the 3 seeds x 5 sizes.

## Prediction (written before data)

From the pilot at q = 1048507 and 2097091 (L0, not a claim): G160 about 0.98-0.99 and G155 about 0.93-0.95 of REF with
additions free, but above 1 at w >= 0.5 and for D; CPU 1.06-1.32 of REF. Predicted verdict: **null** for both arms.

## Execution (route deviation)

The default route for authorised x86 CPU experiments is Iso Lab EKS Spot. The owner chose local runs on the session
container because the Iso Lab EKS budget is exceeded (`../quartic_dl1_cd_sqrt_20261113/launch/BUDGET-HOLD.md`).
Local L0 (Intel Xeon, 4 vCPU), three parallel study lanes serialised by one host lock (one unit per acquisition).
Counted metrics are host-independent; CPU and wall are L0 diagnostics only. Analysis: `python3 analyze.py <dir>`
writes `analysis.json` and `tables.md`; the reported numbers come from that output.
