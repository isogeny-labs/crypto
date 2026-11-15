# PROTOCOL: open-addressing hash tables and a compiled square root in the plane-quartic double-large-prime IC

Frozen in the commit that adds this file, before any data of this study. Not edited afterwards (the commit
hash is recorded in README.md). Every result is an unverified assertion `[pcc:unverified]`.

## Hypotheses

* **H_hash.** Replacing the std `HashMap`s of `find_relations_2lp` (factor-base index, large-prime vertex
  ids) by open-addressing tables, removing the per-candidate `Vec` allocations and prefetching the vertex
  table lowers `relations.cpu` at identical counted work.
* **H_combo.** The same changes plus the compiled (Barrett-reduced, segmented) fixed-exponent square root
  of the sibling lane (`../ic_gw_sqrt_compiled_20261114`) lower it further.

Scope: GHS-weak family `E0/F_{q^3}`, `#E0(K) = 4 ell` (`research/quartic_n4v2_20261020`), driver
`ic <q> <seed> D 3.0 1.7`. Pilot (workflow wf_f57f915b-f9f, L0, not a claim): fast hash 0.56-0.72x and
compiled chain 0.72-0.92x on relations CPU; A/A noise ~0.1% on relations.cpu, up to ~7% on total CPU.

## Arms (binaries)

* **REF**: published v3 `de500bc1...` (`../quartic_dl1_cd_sqrt_20261113/launch/image.json`), checked by sha256
  at run time. c_D = 1.7.
* **HASH**: `src/` built `rustc --edition 2021 -O v3.rs` (rustc 1.97.0): REF's sources with `find_relations_2lp`
  changed as in `src/ic2.rs` (`KeyIdx`, `VidMap`, `Pend`, prefetch distance 8; same pushes, order and
  counters).
* **COMBO**: the same sources built with `--cfg combo`: adds `FastChain`/`pow_fast` (copied from the chain
  lane) at the square-root call site and the `fastcheck` subcommand.
* **REF2**: a second run of the REF binary (A/A).

Binary sha256 are recorded in `binary.sha256` of the results and in README.md. Self test before data:
`bin/combo fastcheck 200000 <5 sizes>` must report 0 mismatches against both the interpreted chain and
modpow, and equal counted cost. A small unit (q = 262111, seed 1) must reproduce REF's counted output for
HASH and COMBO.

## Units (units.tsv, from make-units.sh; 94 units)

q in {262111, 524287, 1048507, 2097091, 4194271} (k = 18..22), seeds 1-3, c_D = 1.7, arms REF/HASH/COMBO,
two passes. Pass 1 rotates the arm order by seed-1, pass 2 reverses pass 1's order for the same (q, seed).
Sizes run largest first. A/A: REF2 at q = 2097091 and 4194271, seed 1, in each pass (4 units). One unit at a
time, each under the host lock `flock .../wf/timing.lock` shared with the two sibling lanes, with load1
recorded inside the lock. Peak RSS is VmHWM polled every 0.2 s by `run-unit.sh` (the driver is
single-threaded; the last value before exit; may under-read a final spike by up to one poll interval).

## Route deviation

The default route is Iso Lab EKS Spot. The account's monthly budget is exceeded
(`../quartic_dl1_cd_sqrt_20261113/launch/BUDGET-HOLD.md`) and the owner chose local runs: this is an L0 run on
the session container (4 vCPU, other lanes' units serialised by the lock). CPU seconds here are the primary
metric at identical counted work and are subject to the L0 caveat: shared host, no isolation, no replay or
matched-rho admission. It supports a within-session paired comparison only.

## Metrics

* **Primary:** `relations.cpu` of the DL record (the relations phase is computed once and shared by D, DL1,
  DL; summed over attempt records), ratio ARM/REF per (q, seed) in the same pass, then the mean of the two
  passes.
* Secondary: total CPU of the DL record over all phases (`fb relations matrix linear_algebra log_recovery
  descent verify`) ratio; peak RSS ratio, reported at q = 4194271 and every size.
* Counted M-eq is not compared (identical by the gate; per-run `ratios` timings are not used).

## Gate (all must hold, else the unit set is invalid for the claim)

Every counted field (all keys except those ending in `wall`/`cpu`, `load1`, `s.controls*`) of every `ic`
record, in order, identical between REF, HASH, COMBO, REF2 and between the two passes, for every (q, seed);
a full-line compare, not keyed by arm only. Every record `ok` and `k_found == k`; every exit code 0. A unit
that fails is rerun once (recorded) and the failure reported.

## Decision rule (fixed now)

A/A floor = max over the A/A pairs (4 single-pass, 2 pass-means) of |REF2/REF - 1| on `relations.cpu`.
For ARM in {HASH, COMBO}: **SUPPORTED** iff at all 15 (q, seed) the pass-mean ratio and both single-pass
ratios are below 1 - floor; "below 1 but not beyond the A/A floor" if they are below 1 only; otherwise
NOT SUPPORTED. The headline reports the range of the pass-mean ratios and the geomean per size. Total CPU
and RSS are reported without a threshold (a total CPU above 1 or an RSS ratio far above 1 is reported as a
cost).

## Exclusions

None planned. A unit whose rc != 0 or that was disturbed (reported by the operator) is rerun once; both
runs are kept and reported. No data are dropped for being unfavourable.

## Predictions (before data)

HASH relations.cpu 0.60-0.80 of REF; COMBO 0.45-0.70; total CPU ratio closer to 1 at small q (the matrix
dominates); RSS ratio of HASH above 1 (open-addressing tables sized 8 slots per key) by at most about 2 at
q = 4194271.

## Analysis

`analyze.py RESULTS_DIR` writes `analysis.json` and `tables.md`; `plot.py` and `REPORT.typ` render them.
