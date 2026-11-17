# Item 5 results: last fall degree of balanced binary unit systems, k = 16..19

All results [pcc:unverified] (engine validated by its own authors only to k <= 10 against the older engine; CPU only; no independent replay).
Protocol: PROTOCOL.md (frozen before any k >= 16 closure; no amendment).

## Verdict
* k = 16 and 17 (3 instances, |Z| = 0): 1 in V_4, certificate degree 4, strict falls [2,3,4]: d_lf = 4 (both laws predicted 4).
* k = 18 (2 instances) and k = 19 (1 instance), |Z| = 0, e = 1: **1 is not in V_4**, V_4 is closed (certificate sweep, 0 repairs) with
  codim 5287 (k=18, both instances) and 10066 (k=19). Hence the last fall degree exceeds 4 there (d_lf >= 5; the degree-5 closure was not run).
* Reading: law B (floor rises to 5 at k + e >= 19, i.e. k >= 18 at e = 1) is **not refuted and predicted the transition exactly**; the post-hoc law
  (4) and A_Z (4) are **refuted at k = 18 and 19** on these instances. d_lf = 4 does not persist in the balanced regime beyond k = 17.
  Caveat: "d_lf >= 5" is read from "unit not reached at D = 4" (a unit |Z| = 0 instance has a fall at the first degree containing 1);
  the exact value 5 (rather than 6+) is not certified; degree 5 columns = C(36,<=5) = 443,704: out of reach on this host.

## Table (engine ic_bounds_bin_big, cpu, 2 threads, shared host; seconds = whole ladder D <= 4 wall time, load average about 5 on 4 vCPU, so times carry contention)
| instance | k | n | e | seed | \|Z\| | cols C(2k,<=4) | rank V_4 | codim V_4 | 1 in V_4 | falls | status | repairs | seconds | law B | post-hoc | reading |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| C3b_..._ran_1 (control, chain's own) | 12 | 23 | 1 | chain | 0 | 12951 | 12951 | 0 | yes | 2,3,4 | exact-unit | 0 | 29.7 (43.6 first run) | 4 | 4 | d_lf=4, equals chain record [0,0,46,1334,12951] |
| C3f_..._ran_0 (chain's instance) | 16 | 31 | 1 | chain | 0 | 41449 | 41449 | 0 | yes | 2,3,4 | exact-unit | 0 | 621 | 4 | 4 | d_lf=4 |
| B5_k17_0 | 17 | 33 | 1 | 20261115 | 0 | 52956 | 52956 | 0 | yes | 2,3,4 | exact-unit | 0 | 1217 | 4 | 4 | d_lf=4; control below threshold, both laws hit |
| B5_k17_1 | 17 | 33 | 1 | 20261116 | 0 | 52956 | 52956 | 0 | yes | 2,3,4 | exact-unit | 0 | 1809 | 4 | 4 | d_lf=4; both laws hit |
| B5_k18_0 | 18 | 35 | 1 | 20261115 | 0 | 66712 | 61425 | 5287 | **no** | 2,3,4 | capped | 0 | 3066 | 5 | 4 | d_lf>=5; law B hit, post-hoc refuted |
| B5_k18_1 | 18 | 35 | 1 | 20261117 | 0 | 66712 | 61425 | 5287 | **no** | 2,3,4 | capped | 0 | 2515 | 5 | 4 | d_lf>=5; law B hit, post-hoc refuted |
| B5_k19_0 | 19 | 37 | 1 | 20261115 | 0 | 82993 | 72927 | 10066 | **no** | 2,3,4 | capped | 0 | 4019 | 5 | 4 | d_lf>=5; law B hit, post-hoc refuted |
Quotient ranks (V_4 cap M_<=j, j=0..4) for the unit/non-unit instances: k17 [0,0,66,2739,52956]; k18 [0,0,70,3080,61425]; k19 [0,0,74,3441,72927]; k16 [0,0,62,2418,41449].
Degree-3 and below behave identically across k (falls at 2,3,4 and nothing else); the change at k = 18 is only that degree 4 stops being enough.
The k=18 pair has identical profiles and codim although the instances differ (system hashes differ): the failure of unit-at-4 looks structural, not an instance accident.
Measured vs derived: ranks, codim, contains_one, closed, repairs, seconds, RSS, flops are engine output (`out/*.json`, `out/*.D4.json`); "d_lf >= 5" and the law readings are derived.

## Not run (dropped, with reason)
* B5_k19_1 (built, |Z| = 0, seed 20261117): est. 4000-4700 s; dropped because the decision between the laws was already made at k = 18 (2 of 2) and 19 (1 of 1) and the session budget.
* B5_k20_0, B5_k20_1 (built, |Z| = 0): est. 7500 s (N^3 from the k=19 run; 2.1 h on a loaded host, near the 2.5 h limit); dropped.
  Next command: `OMP_NUM_THREADS=2 python3 scripts/ic_bounds_bin_big_run.py ladder --instance .../instances/B5_k20_0.json --max-degree 4 --device cpu --batch 128 --work-gb 0.5 --tile-mb 32 --col-block 4096 --out out/k20_0.json` (peak RSS about 1.5 GB).
* Degree-5 closure at k = 18 (N = C(36,<=5) = 443,704) to certify d_lf = 5 exactly: not feasible on CPU here (basis ~N^2/8 bytes, N^3 cost); GPU (deep-chain pod) job.
* No engine support for stop-on-one inside a closure (engine read-only); the ladder tests unit after each completed degree.
* Independent re-run of k=18 with different tile/batch settings (hash-equality check): not run (about 50 min each). The flag independence of the canonical hash was checked on the k=12 control only (`ctl_k12` vs `ctl_k12_tuned`: equal rref_sha256 and quotient ranks).
* Local premise check: the fetched chain snapshot has no k = 13..16 or C6b record (unfinished snapshot at 886 s: results through C3b/C2b only). So "d_lf = 4 on C3a..C3f and 5 for C6b" is not reproducible from the repository files; the k = 11, 12 values do match, and k = 16 (C3f) was re-derived here (d_lf = 4, 621 s).

## Commands and provenance
* Repos: ml-cryptanalysis 27a67b9 (engine csd/ic_bounds_bin_big.py sha256 3ba6e423cbe0...; driver scripts/ic_bounds_bin_big_run.py sha256 f5b433ec6891...), cryptanalysis fb3f88a0 (not used), crypto 44d35033fa.
* Selftest: `ic_bounds_bin_big_run.py selftest --device cpu` -> ok true, 7 cases (`out/selftest-cpu.json`).
* Build: `python3 tools/build_balanced.py K 2 instances` for K = 17..20 (log `logs/build.log`; rejected draws had |Z| = 2, 4, 6; instance sha256 in `logs/instances.sha256`).
* Run: `tools/run_chain.sh` (`OMP_NUM_THREADS=2`, `ulimit -v 6000000`, `ladder --max-degree 4 --device cpu --batch 128 --work-gb 0.5 --tile-mb 32 --col-block 4096`); status `logs/chain.status`; per-run logs `logs/*.log`; k19_0 python PID 9419 (chain script stopped after k18_1 to drop k19_1; k19_0 finished normally, rc recorded in out file). No process of this item remains running.
* Output hashes: `logs/out.sha256`.
* Tuning note: flags were tuned on the k = 12 control (defaults 42 s, tuned 19-30 s); `plan` at defaults predicted 12-22 GB at k = 17..20 (`work_gb`), so `--work-gb 0.5` was used; measured peak RSS 1.2-1.3 GB.
* Cost: 0.28 N^3 model flops vs measured per_degree_flops at D = 4: k=18 5.8e13, k=19 1.1e14 (about 14-28 GFLOP/s effective on a loaded host).

## Sources checked (KB step not run: the research-kb MCP server is not reachable from this subagent; local files read instead)
ml-cryptanalysis: experiments/ic-bounds-binary-deep/{PLAN.md,cells.json,make_cells.py,fetched/chain-status.json,fetched/results.jsonl}; experiments/ic-bounds-binary-law/laws.md (laws B and A as quoted in the task and make_cells.py).
