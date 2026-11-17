# L12 protocol: F4 trace replay on the T10/T12 winner (k = 20, isogeny factor base)

Wave 4, lead L12. Written 2026-11-04 from existing data only, before any L12 data.
**Status: design (preregistered). Nothing below changes after the first run; deviations go to a
dated addendum.** No git in this lead (the user opens the PR after validation).

**Class.** (1) Exploratory engine measurement in counted units (F4 multiply-adds from
`curve25519_family f4cost`); (2) conditionally, an end-to-end ecbench component measurement of
the algebraic PKM arm. Neither is a leaderboard row. Rho remains ahead on every track; the
distance remaining is stated, not gated.

Parents: T10 (`research/curve25519_leads_wave3_20261103/t10_seed_and_guess`, winner
t = 5, G = 4, seed 1307) and T12 (`research/curve25519_t10_winner_ecbench_20261104`, end-to-end:
S(winner)/S(t6,G4,seed 0) = 0.0578 [0.047, 0.072] at w ≥ 0.1; 6.06·10⁴× behind rho-neg at
w = 0.1, 17.1× at w = 0).

## 1. Question

`pkm-semaev` has an F4 trace mode (`trace=1`, `src/cryptanalysis/ic_framework/pkm_semaev.rs`):
at prepare it learns **one refutation trace per branch system** (2^G branches; plain F4 with
recording on fixed, target-independent x_R values, at most 8 tries per branch; every try's
mul-adds go to the setup ledger `pkm_trace_learn_muladds`), then **replays that trace per target**
with x_R specialised. Only a refutation is accepted; when the replay does not refute (a target
that decomposes, or a pivot that vanishes for this x_R) it falls back to plain F4 and charges
both. The per-target ledger `solver.ops` = replay mul-adds + fallback mul-adds.

The winner's work is 16 refutations per target (1,600 F4 calls, 1,600 refutations, 0 hits on
100 random targets at coverage 210/131,203 = 0.16%). A refutation certificate replays only the
rows that reach the contradiction, not the whole critical-pair exploration, so the per-target
mul-adds should fall by a constant factor at the same verdicts.

**Q. By what factor does trace replay cut the F4 mul-adds per target at the winner cell, with
the verdicts unchanged, and does the cut survive end to end in S across the §5a band?**

## 2. Inputs computed now from existing data (no new measurement)

Instruments (frozen; `run.sh` refuses to run on any other hash):
- `/home/user/ctarget/release/curve25519_family`, sha256
  `ba1e3c0518eae58ae39a9b08e1f757c7e9fe1eb02c86b158b883244bc6a5ecef` (= T10's). Its `f4cost`
  takes `trace 0|1` as the 13th positional argument (`src/bin/curve25519_family.rs`, "Position 12:
  max_nnz; position 13: F4 trace"); its JSON already carries `trace`, `trace_learn_muladds`,
  `amortised_muladds_per_target`, `native.pkm_trace_*` (replays, refuted, fallbacks, untraced
  branch runs, failure reasons) and `setup_native.pkm_trace_*` (branches, branches_learned,
  learn_runs, learn_muladds, kept_rows, selfcheck_refuted).
- `/home/user/ctarget/release/ecbench`, sha256
  `54737834d048b22e77399160fa9e159782b1124ea8190d84dba185309229641b` (= T12's).
  `ecbench plan --spec ecbench/spec.json` accepts the L12 spec (design check,
  `ecbench/plan_design_check.txt`: 64 executions, 8 workloads, the trace arm resolved as
  `pkm-semaev:m=2,mode=full,guess=4,trace=1`). `plan` resolves methods but does not run the
  oracle's parameter parser; a rejection of `trace=1` can still surface at run time (section 5).
- Census `research/curve25519_isogeny_seed_20261103/data/k20.jsonl`, sha256
  `bc74d8ac5151713c59f6c6d5bf58df1c0bf5e3442c6b50c7d234163c4c807adc`.

Plain-F4 cells from T10 (`data/cells/*.json`, 100 targets, 0 hits, 0 undecided, none capped):

| cell | form | t | G | cols | covered | mpt (plain) | F4 calls/target | per branch system | E₀ = cols·r·mpt/covered |
|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| **winner** | isogeny@1307 | 5 | 4 | 22 | 210 | 52,816.0 | 16 | 3,301 | 7.260·10⁸ |
| t5 G3 | isogeny@1307 | 5 | 3 | 22 | 210 | 177,112.0 | 8 | 22,139 | 2.434·10⁹ |
| t6 G4 s1308 | isogeny@1308 | 6 | 4 | 36 | 498 | 710,095.08 | 16 | 44,381 | 6.735·10⁹ |
| t6 G4 s0 | isogeny | 6 | 4 | 25 | 172 | 710,094.0 | 16 | 44,381 | 1.354·10¹⁰ |

r = 131,203, √r = 362.22. T10 used `max_nnz` 3·10⁷; no cell came near it (max_nnz_max ≤ 1,341),
so L12's `-` (no cap) must reproduce the same counts (P6).

Learning-cost bound: ≤ 8 tries × 2^G branches × (mpt/2^G) = 8·mpt per prepare: 422,528 at the
winner (= 8 targets' plain work, 5.8·10⁻⁴ of E₀), 1.42·10⁶ (t5 G3), 5.68·10⁶ (t6 G4). Expected:
about one try per branch, ≈ 16 × 3,301 ≈ 5.3·10⁴ at the winner.

End-to-end scale from T12 (chunk c00, round 0, winner arm): `solver.ops` 6.13·10⁸ for 12,116
trials and 13 relations; other unpriced natives (`pkm_system_fp_mults` 2.6·10⁶, grid 1.6·10⁴,
...) are 0.4% of the F4 term. Mean S over 32 runs: 26.404 (w = 0), 100,277.2 (w = 0.1),
1,002,534 (w = 1); rho-neg 1.654 and proj-a 13.625 at w = 0.1.

Model, ρ = mpt(trace)/mpt(plain) at the winner cell (group operations unchanged, F4 term scaled
by ρ, learning added at w per solve):

| ρ | S_trace(w = 0.1) | S_trace/S_plain (w = 0.1) | distance to rho-neg at w = 0.1 |
|--:|--:|--:|--:|
| 0.1 | 10,449 | 0.104 | 6.3·10³× |
| 0.2 | 20,470 | 0.204 | 1.24·10⁴× |
| 0.3 | 30,492 | 0.304 | 1.84·10⁴× |
| 0.5 | 50,535 | 0.504 | 3.06·10⁴× |
| 0.6 | 60,556 | 0.604 | 3.66·10⁴× |

At w = 0 the ratio is 1 by construction (same walk, same verdicts, same relations).

**Accounting fact found in the source, stated before data.** The band prices every record
counter whose name ends in `_uncharged` (`unpriced_of`) plus the solver term resolved to
`solver.ops`. `pkm_trace_learn_muladds` has no `_uncharged` suffix, so **ecbench's band and
compare do not price the learning cost**; `pkm_trace_learn_spec_fp_mults_uncharged` is priced.
L12 therefore adds the learning mul-adds **by hand** at every w (`scripts/learn_charge.py`,
S_charged = (GAE + w·(unpriced + learn))/√r per run) and reports both columns. By the bound above
this is ≤ 0.06% of a solve, so it cannot flip a verdict, but it is reported, not assumed away. A
missing charge is an accounting gap, not an improvement.

## 3. Design

### 3.1 Engine stage (always run): 8 f4cost cells, strictly sequential, one process at a time

For each cell, trace 0 then trace 1, with `F4COST_PLANTED=16` (16 planted sums of two base
points, run after the 100 random targets; their F4 work is reported apart from `mpt`, so they
exercise the fallback path and verdict identity on decomposable targets without touching the
ratio):

```
/home/user/ctarget/release/curve25519_family f4cost 20 isogeny@1307 5 4 100 2026111505 3600 2 full 0 - 0   # winner, plain
/home/user/ctarget/release/curve25519_family f4cost 20 isogeny@1307 5 4 100 2026111505 3600 2 full 0 - 1   # winner, trace
/home/user/ctarget/release/curve25519_family f4cost 20 isogeny@1307 5 3 100 2026111505 3600 2 full 0 - {0,1}
/home/user/ctarget/release/curve25519_family f4cost 20 isogeny@1308 6 4 100 2026111506 3600 2 full 0 - {0,1}
/home/user/ctarget/release/curve25519_family f4cost 20 isogeny      6 4 100 2026111506 3600 2 full 0 - {0,1}
```

Target seed 2026111500 + t is T10's convention (2026111505 at the winner, as the brief
specifies), so every trace-0 cell is an exact replication of a T10 cell. Outputs:
`data/engine/<cell>_tr{0,1}.{json,stderr,status}`; a non-zero exit keeps the attempt
(`.try1.*`) and reruns once (F2). `scripts/analyze_engine.py` writes `data/engine/summary.json`
and `data/engine/GATE`.

### 3.2 End-to-end stage (only if GATE = open, i.e. P2 holds): chunked ecbench

Frozen spec `ecbench/spec.json` (copied to `data/ecbench/spec.frozen.json` at launch): k = 20
curve of T12; arms `rho-neg`, `proj-a`, `pkm-iso-t5-g4-s1307`
(`pkm-semaev:m=2,mode=full,guess=4`) and `pkm-iso-t5-g4-s1307-trace` (`...,trace=1`), same
`prime-isogeny:t=5,seed=1307` base; 2 rounds, `alternate_measured`, timeout 1,800 s; 8 public
targets, `target_seed` base 2026110421, measurement seed base 2026110422.

Run through `scripts/chunked_run.py` (verbatim copy of T12's, sha256
`e8edaa81515b400bcb29e402f6ff0d8548001498aef662c23f47daf3d19aabde`): 8 one-target chunks, chunk c
at target_seed 2026110421 + c and measurement seed 2026110422 + c; each a complete audit-valid
session; complete chunks skipped on relaunch, killed ones moved to `interrupted/`; `--wait
--allow-busy` behind `/tmp/crypto-bench.lock` (T11 may hold it). 16 runs per arm.

Analysis on the pooled chunks: `ecbench verify --replay 1` per chunk; `ecbench band --dir
<8 chunks> --ranks` at w = 0, 0.1, 1; paired ratios by `scripts/pooled_compare.py` (verbatim copy
of T12's, sha256 `061b11b0eec670ee6160b60d1b821bf52a7af277d2d4adf342d6afee57d4fdf9`; reproduces
`ecbench compare` to 6 decimals on T9's single complete session) for trace ÷ plain, trace ÷
proj-a, trace ÷ rho-neg at each w (K = 8 workloads, df = 7); `scripts/learn_charge.py` for the
hand-charged learning column, the per-pair w = 0 identity and the summed `pkm_trace_*` counters.

`scripts/run.sh` executes exactly this, in this order, and is re-launchable after a container
restart (engine cells with `rc=0` are skipped; the driver skips complete chunks; the analysis and
`ALL_DONE` are written only when all 8 chunks are complete).

## 4. Predictions and decision rules (decided on the files named above; nothing is retuned)

Engine stage (P1–P6), each decided per cell from `data/engine/summary.json`:

- **P1 (verdict identity).** In each of the 4 cells, trace = 1 and trace = 0 agree exactly on
  `columns`, `covered_points`, `hits`, `hits_verified`, `hits_exact`, `undecided`, `capped`
  (= false), `targets_done` (= 100), `solver_extra.refutations`, and on the planted block
  `planted_done`, `planted_hits`, `planted_exact`, with `planted_hits = planted_done > 0`. Any
  inequality fails P1.
- **P2 (gate; winner cell).** ρ_win = mpt(trace)/mpt(plain) at (t = 5, G = 4, seed 1307) ≤ 0.5.
  Exploratory point guess, not a rule: ρ_win in [0.15, 0.35].
- **P3 (other cells, seed independence).** ρ ≤ 0.5 in each of the other three cells, and
  |ρ(t6, G4, seed 1308) − ρ(t6, G4, seed 0)| ≤ 0.02 (mpt is seed-independent to 10⁻⁵ in T10).
- **P4 (replay reliability).** In every cell `pkm_trace_branches_learned` = 2^G,
  `pkm_trace_untraced_branch_runs` = 0, `pkm_trace_replays` > 0 and
  `pkm_trace_fallbacks / pkm_trace_replays` ≤ 0.01.
- **P5 (learning cost bounded).** In every cell `trace_learn_muladds` ≤ 8·mpt(plain) and
  ≤ 1% of E₀(plain); hence the hand-charged learning changes no verdict.
- **P6 (trace-0 replication of T10).** Each trace-0 cell's mpt equals the T10 value in the table
  of section 2 to within 10⁻⁶ relative, and `columns`/`covered_points` equal the census.

End-to-end stage (P7–P10), only if P2 holds; decided on the 8 pooled chunks:

- **P7 (verification).** Both PKM arms verify 16/16 and every chunk's `verify --replay 1` exits 0.
- **P8 (ratio).** S(trace)/S(plain) ≤ 0.6 at w = 0.1 and at w = 1, with the 95% upper bound < 1
  (pooled_compare, K = 8); the same holds for the hand-charged ratio of `learn_charge.py`.
- **P9 (w = 0 identity).** For every paired run (workload, round), `total_gae` and the recovered
  scalar are identical between the two PKM arms, so the w = 0 ratio is exactly 1.000. If the
  plan does not pair algorithm seeds across arms (`seeds_paired = false` in `learn_charge.json`),
  P9 is declared not testable as stated and replaced, before looking at the ratio, by: w = 0
  ratio in [0.9, 1.1] with the 95% interval covering 1.
- **P10 (band ranks).** Both PKM arms stay behind `proj-a` and `rho-neg` at every weight; the
  trace arm's distance to rho-neg at w = 0.1 is reported as a number (model: 6.06·10⁴ × ρ_win).

## 5. Failure handling

- A failed prediction is reported as failed, never retuned; the numbers stay in the report.
- P2 fails (ρ_win > 0.5): the gate stays closed, no ecbench is run, the engine result is the
  result of the lead.
- A trace cell exits non-zero twice, or its JSON lacks `trace`/`trace_learn_muladds`: recorded as
  "instrument does not support trace here"; the lead stops at whatever cells completed.
- ecbench rejects `trace=1` for this oracle (`plan` error, or an error outcome mentioning
  `trace` on the trace arm in chunk c00): `data/ecbench/TRACE_REJECTED` is written, the
  rejection text is quoted in the report, and the lead stops at the engine result.
- Container restarts (every 40–60 min): relaunch `run.sh`; no killed chunk is cited; interrupted
  chunks stay in `interrupted/`.
- P1 fails because trace and plain disagree on a verdict: this is a correctness finding about
  the trace engine and is reported as such; no ratio from that cell is quoted as a gain.
- The host is shared (3–4 cores, T11's ecbench session in flight): S and mul-adds are counted
  units; no wall-time claim; `wall_s` is recorded but not interpreted.

## 6. Scope and distance remaining

- Trace replay is a solver change that performs fewer mul-adds for the same verdicts; it is a
  component speed-up of the algebraic arm, priced at w = 0, 0.1 and 1 like every other native
  operation, with the learning cost charged by hand because the band does not price it.
  Relabelling nothing: the walk, base, relations and linear algebra are identical.
- At w = 0 nothing changes: the winner stays 17.1× behind rho-neg (T12). At w = 0.1 the winner
  is 6.06·10⁴× behind rho-neg; if ρ_win = 0.5 the trace arm would be ≈ 3.1·10⁴× behind, if
  ρ_win = 0.2 ≈ 1.2·10⁴× behind. Rho remains ahead on every track; this is not a leaderboard
  claim and no row is proposed.
- Not covered here: trace replay at other (t, G), on the bits/t0 forms, at k ≠ 20, or combined
  with per-seed selection at k = 22/24; a hit on P2 makes those follow-ups, not this lead.
- Counted units only: F4 mul-adds (engine), GAE + w·native (ecbench). No timing claims.

## 7. Files

- `scripts/run.sh` (the planned runs, exactly), `scripts/analyze_engine.py` (P1–P6, GATE),
  `scripts/chunked_run.py` and `scripts/pooled_compare.py` (verbatim from T12),
  `scripts/learn_charge.py` (hand-charged learning, P9 identity, trace counters).
- `ecbench/spec.json` (frozen), `ecbench/plan_design_check.txt` (design-time `plan` output).
- `data/engine/`, `data/ecbench/` (chunks, verify, band, pooled_compare.jsonl, learn_charge.json),
  `data/env.txt`, `data/run.log`, `data/SHA256SUMS`, `data/ALL_DONE`.
