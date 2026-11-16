# Equal-advice preprocessing rho vs compact-orbit IC at n = 73 and n = 83: protocol

Frozen 2026-10-18, before any n = 73 or n = 83 datum exists. The only runs made
before this file are the instrument checks in `README.md` (n = 61 narrow against
wide, and a harness smoke test at n = 41); they are not part of any result below.
Nothing here was run at n = 73 or n = 83.

Owner request 2026-10-18 after
`research/ic_leads_review_20261018/README.md` (section 2.1 item 1, section 3.2).
Everything is native Rust plus a shell script for process orchestration
(AGENTS.md, "Implementation language: no Python").

## Question

`RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md` section 2 calculates that, at
equal advice bytes, Bernstein-Lange (BL) preprocessing rho with Frobenius
folding needs thousands of times fewer online operations than the compact-orbit
IC extraction at n = 73 and n = 83, and that the `single_target_online` rows then
"have no content". Item 9 of `ic_next10_20261014` measured this at n = 41, 53
and 61 (BL 50 to 500 times faster online) but its instrument stops at n <= 63,
and the later n = 61 pairing
(`research/sat_factor_base_review_20260908/autolab/evidence/20261016-n61-bl-ic-pair-l1024/README.json`)
failed `claim_check` because the BL arm was replayed from an archive. This
protocol fixes one session that co-measures both arms at the two wide rungs.

**What exists already on main, and what this adds.** PR #1987
(`research/20261016-n83-precomputed-rho-control/`) ran a precomputed-table rho
against the N83 IC online claims, matched on setup wall time and on IC peak RSS,
on hosted runners, in separate processes, and found precomputed rho 3.7 to 395
times faster online. This protocol differs in four ways: (i) the BL table size is
the IC table's own byte budget, not a time or RSS match; (ii) n = 73 as well as
n = 83; (iii) both arms live in one process group under one resource envelope
with the BL table resident throughout, and the order is ABBA; (iv) the output is
one `vs_rho` claim report per (run, table level, target) with replay
certificates, checked by the native claim checker. Its result is a second,
independent reading of the same question, not a replacement.

## Instrument

- `examples/ic_equal_advice_wide.rs` (modes `targets`, `bl`, `selftest`): the
  wide twin of `examples/ic_next10_09_equal_advice.rs`. Same map, same labels,
  same streams (signed-Frobenius classes in a normal basis, 2,048 `G`-only jumps,
  4-step look-ahead, identical map offline and online, DP density `2^-t`,
  `t = round(log2 sqrt(N/T))`, `N = r/(2n)`). Field elements, class keys and
  coordinates are `u128`; scalars stay `u64` (`r < 2^64` on every admitted rung,
  asserted). Arithmetic is `Gf2_128`/`FastBinaryCurve128` for n > 63.
  The table entry is an 8-byte fingerprint of the canonical x (the key itself
  when n <= 64, a 64-bit fold above) plus an 8-byte log: 16 bytes, so
  `T = bytes / 16`. A fingerprint collision costs one failed `[d]G = Q`
  check, never a wrong answer.
- Additions over the narrow instrument: multi-threaded table walks (`BL_THREADS`;
  one thread reproduces the narrow loop exactly), the epoch tag saturates
  instead of overflowing, nested prefix tables by byte budget (`BL_PREFIX_BYTES`,
  zero-copy epoch cuts), a barrier protocol so the passes interleave with the IC
  processes (`BL_BARRIER_DIR`), the null control (`BL_NULL_TARGETS`), and three
  exclusive online phases (`walk`, `collision`, `recovery_check`) on one clock.
- `examples/ic_equal_advice_claims.rs`: replay, claims, checks, statistics.
- `run.sh`: orchestration only.
- Instrument check (README.md): wide at n = 61 against the narrow instrument on
  the frozen item-9 n = 61 fixture, same seeds; `cargo test --release --example
  ic_equal_advice_wide`.

## Rungs (frozen constants)

| | n = 73 | n = 83 |
|---|---|---|
| curve | `a=0, b=1`, `x^73+x^4+x^3+x^2+1`, `EC1N73Ckb1h25afb0fafbdf` | `a=1, b=1`, low terms `[0,2,4,7]`, `EC1N83Ckb1a1haacb69ee9f37` |
| subgroup order r | 86,020,738,150,056,119 (2^56.3) | 8,569,786,107,849,059 (2^52.9) |
| N = r/(2n) | 5.89e14 (2^49.1) | 5.16e13 (2^45.5) |
| IC arm | `koblitz_orbit_dlp_fast construct:73:0:600`, rank seed 7, default `KIC_*` | `koblitz_orbit_dlp_fast construct:83:1:3240`, rank seed 29, root21 environment of `research/n83_root21_20261014/run.py` (`KIC_ROOT_ONLY21=1 KIC_INDEXED5_FIXED30=1 KIC_SYMMETRIC_S3=1 ...`) |
| IC candidate | `experiments/koblitz-single-target-n73-20261003/candidate-manifest.json` (`IC1N73Ckb1fb87600...h980b4cf20ad4`) | `research/n83_factor_base_panel_results_20261014/candidate_K3240_root21.json` (`IC1N83Ckb1a1fb537840...hff882a640d8e`) |
| IC states S | K^2 n = 26,280,000 | 435,652,020 (`83K(K-1)/2 + 42K`) |
| **advice, tier A** (IC-favourable) | **2,781,357,248 B**: the item-9 formula for K=600, n=73 (`scripts/ic_bytes.py`) | **14,517,401,540 B**: `21*435,652,020 + 5*2^30`, the Lean-checked K3240 root21 payload (13.520 GiB, `research/n83_root21_20261014/PROOF_MAP.md`) |
| **advice, tier B** (generous) | **5,630,296,064 B**: wait4 peak RSS of the frozen n = 73 K=600 IC run (`ic_execution.json`; `RESEARCH_ECC2K130_IC_FEASIBILITY.md` section 2: "26.3 M states, about 5.6 GB") | **14,954,881,024 B**: wait4 peak RSS of the K3240 root21 cloud run (`research/n83_root21_results_20261014/RESULT.md`, 13.928 GiB) |
| BL entries T (tier B / tier A) | 351,893,504 / 173,834,828 | 934,680,064 / 907,337,596 |
| predicted `dp_bits` (W = 2^t) | 10 (1,024) | 8 (256) |
| predicted BL precompute steps | about 3.6e11 | about 2.4e11 |
| predicted BL online steps (`N/(T W) + W`, times 2.2) | about 6e3 | about 1e3 |
| IC online (recorded, other hosts) | 249 to 544 ms (457,561 probes) | 14 to 60 ms |

Tier A is a lower bound on what the IC table needs (it leaves the hash maps and
allocator overhead out, like item 9), so a BL win at tier A holds at the IC's
real equal advice. Tier B is at least what the IC process held, so an IC win
at tier B holds at equal advice. The BL tables are built once at tier B; tier A
is the largest whole number of flushes of that table that fits the tier A bytes
(an epoch cut, so its bytes are at most the budget).

Two budget readings are used because "the IC table's byte budget" has no single
value: the recorded numbers are a formula (a lower bound) and a measured peak
RSS (an upper bound), and a conclusion should not depend on which a reader picks.
At n = 83 they differ by 3%.

## Targets and seeds

`T = 64` public targets per rung, `Q = [d]G` with `d = H("ic-next10-09-targets|n|a|equal-advice-wide-20261018-v1", i) mod r`
(the item-9 stream, new label). The scalars are written to a sidecar that no
producer reads and the claims tool does not read; recovered scalars are replayed
on the generic curve. BL seed `v1` (jumps, table walks, online starts); IC rank
seeds 7 (n = 73) and 29 (n = 83).

## Session (one rung)

One process group (`setsid`), one cpuset for every process (`taskset`), one
address-space cap (`prlimit --as`), `nice 0`, `RAYON_NUM_THREADS=1`, the same
envelope object recorded for both arms. Table walks and the IC rank stage use
`THREADS` threads (setup only). Every online interval is one thread.

1. The BL process starts, builds the table (tier B bytes) with the `G`-only walks,
   writes `barrier/bl_ready` and sleeps at its barrier with the table resident.
2. IC process 1 runs: base, index, rank stage, then the 64 targets, one clock per
   target.
3. BL pass 1 (`go_1`): tier B then tier A over the 64 targets, then 8 null-control
   targets.
4. BL pass 2 (`go_2`): the same.
5. IC process 2 runs as in 2.

Order IC#1, BL#1, BL#2, IC#2 (ABBA). Run 1 pairs (IC#1, BL#1), run 2 pairs
(IC#2, BL#2). The idle arm is asleep with its advice resident. Both arms keep
their setup and precompute out of the online interval and report it apart:
IC `cold_phase_ms`/`timing_ms.setup_total`, BL `bl_precompute`. Process launch,
input loading and target generation are outside both intervals.

**Online intervals.** IC: `target_query_begin` to `recovery_check_end`, five
exclusive phases (the producer's). BL: from before the first `[c]G` of the first
walk (`target_walk_start`) to after the `[d]G = Q` check (`recovery_check_end`),
three exclusive phases `walk`, `collision`, `recovery_check`; the claim reports
`walk + collision` as `walk_and_collision_ms` and `recovery_check_ms`, as the
ledger declares. The BL interval includes no subgroup check (the targets are
generated in the subgroup and verified by the replay); the IC interval does
not either.

## Memory, time and hosts

| | n = 73 | n = 83 |
|---|---|---|
| resident at once | IC about 5.6 GB + BL about 5.8 GB (16 B x 351.9 M x 1.031) | IC 13.9 GiB + BL 15.4 GB (16 B x 934.7 M x 1.031) |
| minimum RAM (preflight rejects below) | 24 GiB | 48 GiB |
| address-space cap per process | 16 GiB | 32 GiB |
| IC setup, recorded | rank stage 18,398 s single-thread (5.1 h), 5.63 GB | 580 to 694 s with 4 rank threads, 13.93 GiB peak RSS |
| BL table walks, projected | 3.6e11 steps; 0.4 to 1.0 us per wide step gives 40 to 100 core-hours | 2.4e11 steps; 27 to 67 core-hours |
| session wall, projected, 32 threads | 3 to 5 h | 2 to 3.5 h |

These are projections; the wide step cost has not been measured (the n = 61
check measures only the narrow backend, 0.2 to 0.3 us). Disk: 2 GiB free. The
host must be otherwise idle during the online passes; the script does not
establish CPU isolation (`isolation_levels` is `L0` unless `ISOLATION_LEVEL` is
set by an operator who holds an isolab receipt).

Command (from the repository root, on a large host; `bash >= 4.4`, GNU `time`,
`taskset`, `prlimit`, `setsid`, `util-linux`):

```
CARGO_TARGET_DIR=target THREADS=32 research/equal_advice_wide_20261018/run.sh n73 n83
```

`OUT_ROOT` defaults to `research/equal_advice_wide_20261018/runs/<UTC>/`; each
rung writes `<rung>/assembled/decision.json`, `claims/`, `certs/`, `rows.json`.
`run.sh n41smoke` exercises the harness on a 44 MB table in seconds; its output
is not a result.

## Verification, replay and the claim

- Both arms' recovered scalars are replayed by `KoblitzCurve::mul` on the generic
  `F2mElement` curve (independent of both producers' field arithmetic). One
  replay certificate per arm, run and level; its SHA-256 (of the canonical JSON)
  is `ic_replay_certificate_sha256` / `rho_replay_certificate_sha256`.
  A target either arm fails to solve stays in the record and the rung verdict is
  `INVALID_UNVERIFIED_OR_MISSING`; it never counts as a win.
- One `vs_rho` report per (run, level, target) with the BL arm in the rho slot:
  `target_count = 1`, `timing_class = single_target_online_wall`, matching
  target hashes, five IC and two BL exclusive phases summing to the walls,
  `online_interval`, `rho_policy` (`worker_count`, `distinguished_point_memory_bytes`,
  `walk_policy`, `collision_policy`), `operation_accounting` with native u64
  counters, the identical `ic_resource_envelope`/`rho_resource_envelope`,
  `same_resource_envelope`, `isolation_levels`, `record_class`. Each is checked
  in-process by `ecbench::claim::check_vs_rho`; `run.sh` also runs
  `ecbench claim check` when that binary is built.
- **Fields this session cannot establish, left false or absent (so the checker
  lists them):** `independent_validation` and `independent_replay_pointer` (a
  second host must replay; `ecbench claim attach`), any isolation level above
  `L0` (needs an isolab receipt), and, for the IC1 identity, the following
  limit: the native identity minting (`claim::build`) admits Koblitz n <= 61, so
  wide candidates reuse the frozen `IC1` manifests above (re-hashed and checked
  here) and the workload manifest is minted natively with wide coordinates as
  decimal strings (`identity.workload_manifest` otherwise). Expected outcome:
  `claim_check: FAIL` on those fields, with `record_class =
  verified_answer_exploratory_wall`. The decision below does not depend on them;
  a promotion to the ledger does, and is not requested.

## Controls (a failed control makes the rung verdict `UNRESOLVED_CONTROL_FAILED`)

1. **A/A.** The median per-target ratio IC#1/IC#2 and BL#1/BL#2 (each level) lies
   in [1/1.25, 1.25]; IC probe counts are identical across the two IC processes
   (determinism); the IC `base_hash` equals the frozen one where one is recorded
   (n = 83: `54847e372e3d68a2509bb16227aef70e09ceb9948d9e73149b6abc21355a2c0e`).
2. **Positive (BL instrument law).** Mean matched-map BL online steps at tier B
   lie within [0.25, 8] x `N/(T W) + W`.
3. **Null (identical map).** With the online jump map different from the
   table's (8 targets), mean steps are at least 5 x the matched-map mean (the
   table helps only through the identical map; item 9 saw 52 x).

## Statistic and decision rule (fixed now)

Per (run, level): `ratio_i = BL_online_ms_i / IC_online_ms_i` over the 64
targets; median with a 10,000-resample target bootstrap 95% interval (seed
`20261018 + 100 run + level`); the ratio of means and its interval are reported
beside it. Wall time only, on one host, so the unit is milliseconds; the section
5a band is reported in addition (below).

- **Tier A verdict** `BL_FASTER_AT_TIER_A` iff, in both runs, the upper end of
  the median-ratio interval is below 1; otherwise `NOT_BL_FASTER_AT_TIER_A`.
- **Tier B verdict** `IC_FASTER_AT_TIER_B` iff, in both runs, the lower end of the
  interval is above 1; otherwise `NOT_IC_FASTER_AT_TIER_B`.
- **Rung verdict.** `BL_WINS_AT_EQUAL_ADVICE` if tier A is `BL_FASTER_AT_TIER_A`
  (the online IC "win" over plain rho does not survive rho given the IC's own
  advice; the coordinator edits the leaderboard, this item recommends only);
  else `IC_WINS_AT_EQUAL_ADVICE` if tier B is `IC_FASTER_AT_TIER_B`; else
  `UNRESOLVED`. Controls failed or a replay failure override both.
- The two rungs are judged separately; no pooling across rungs or runs.

The prediction recorded before the run (a derivation, not a measurement): BL
online is about 2 to 4 ms at n = 73 and about 1 to 2 ms at n = 83 against the IC
medians above, so `BL_WINS_AT_EQUAL_ADVICE` on both rungs, with median ratios
near 0.01 to 0.1. If the wide step turns out far slower than the narrow one, the
ratio rises; the rule does not change.

**Section 5a pricing (AGENTS.md).** Per arm, priced work is group additions only
(IC: `relation_checks`, each at least one point addition; BL: online steps plus
one addition per walk start). Unpriced native work is IC root calls plus probes,
and BL canonicalisations (one per step), `[c]G` scalar multiplications (one per
walk) and table lookups. The report prices each unpriced operation at 0, 0.1 and
1 GAE and ranks the arms only where the order holds across the band
(`section_5a_order_holds_across_band`); IC counts are lower bounds. The anchor
note's `log2(S T^2) - log2(N)` is recorded for both arms on every row
(`log2_S_T2_minus_log2_N`, with `S` = IC states or BL entries and `T` = mean IC
probes or BL steps). Distance to rho is reported as these ratios, not as a
pass/fail gate; precomputation (IC setup, BL table walks) is quoted next to them
and is not part of the claim.

## Limits and non-claims

Public synthetic points on two Koblitz curves; no key recovery, asymptotic or
deployed-curve claim. This is the online interval at matched advice, not the
end-to-end cold `S`: BL's precompute is 10^11 to 10^12 group-step equivalents and
IC's rank stage is hours, both reported apart. Process-level ABBA, not
per-target interleaving, because the IC producers run all targets in one pass.
Not an improvement to the IC baseline: it audits a reference, and a result here
is classed accounting/audit, not advance or engineering.
