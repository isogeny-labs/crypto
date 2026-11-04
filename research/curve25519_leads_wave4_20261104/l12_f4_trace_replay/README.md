# L12: F4 trace replay on the T10/T12 winner (Curve25519 k = 20, isogeny factor base)

**Class.** (1) Exploratory engine measurement in counted units (F4 multiply-adds from
`curve25519_family f4cost`), decided; (2) end-to-end chunked ecbench of the algebraic PKM arm with
and without trace replay, **launched by the frozen runner and still in flight when this report was
written** (see "Status of the end-to-end stage"). Neither is a leaderboard row. Rho remains ahead on
every track; the distance remaining is stated, not gated.

Protocol: [PROTOCOL.md](PROTOCOL.md), frozen with `scripts/` at `5a800bb3d06c69b32f32195c3aebccdfb3fee31e`
before any L12 data. Runner: `scripts/run.sh` (launched 2026-11-04T06:32:56Z, host `vm`, 3 cores,
load 1.76 at launch). Engine analysis: `scripts/analyze_engine.py` -> `data/engine/summary.json`,
`data/engine/GATE`. No git in this lead.

## Question

By what factor does F4 trace replay (`pkm-semaev` `trace=1`: one refutation trace learned per branch
system at prepare, replayed per target) cut the F4 mul-adds per target at the winner cell
(t = 5, G = 4, seed 1307), with verdicts unchanged, and does the cut survive end to end in S?

## Instruments (frozen hashes, checked by run.sh before every cell)

- `/home/user/ctarget/release/curve25519_family` sha256 `ba1e3c0518eae58ae39a9b08e1f757c7e9fe1eb02c86b158b883244bc6a5ecef` (= T10).
- `/home/user/ctarget/release/ecbench` sha256 `54737834d048b22e77399160fa9e159782b1124ea8190d84dba185309229641b` (= T12).
- Census `research/curve25519_isogeny_seed_20261103/data/k20.jsonl` sha256 `bc74d8ac5151713c59f6c6d5bf58df1c0bf5e3442c6b50c7d234163c4c807adc`.
- Engine command per cell: `curve25519_family f4cost 20 <form> <t> <G> 100 <2026111500+t> 3600 2 full 0 - <trace 0|1>`
  with `F4COST_PLANTED=16`. All 8 cells exited 0 on the first try; none capped; `undecided = 0`;
  100/100 targets and 16/16 planted targets in every cell.

## Engine results (verbatim from `data/engine/*_tr{0,1}.json`)

r = 131,203. mpt = `muladds_per_target` (100 random targets, planted block excluded).
E₀ = cols · r · mpt / covered (modelled F4 mul-adds per solve). learn = `trace_learn_muladds`
(one-time per prepare, the setup ledger `pkm_trace_learn_muladds`).

| cell | form | t | G | cols | covered | mpt plain | mpt trace | **ρ = trace/plain** | learn | learn/E₀ | E₀ plain | E₀ trace |
|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| **winner** | isogeny@1307 | 5 | 4 | 22 | 210 | 52,816.0 | 24,912.0 | **0.4717** | 52,816 | 7.3·10⁻⁵ | 7.260·10⁸ | 3.424·10⁸ |
| t5 G3 | isogeny@1307 | 5 | 3 | 22 | 210 | 177,112.0 | 100,208.0 | **0.5658** | 177,112 | 7.3·10⁻⁵ | 2.434·10⁹ | 1.377·10⁹ |
| t6 G4 s1308 | isogeny@1308 | 6 | 4 | 36 | 498 | 710,095.08 | 322,703.82 | 0.45445 | 710,096 | 1.05·10⁻⁴ | 6.735·10⁹ | 3.061·10⁹ |
| t6 G4 s0 | isogeny | 6 | 4 | 25 | 172 | 710,094.0 | 322,701.91 | 0.45445 | 710,096 | 5.2·10⁻⁵ | 1.354·10¹⁰ | 6.154·10⁹ |

Verdicts and trace counters, every cell, trace 1 against trace 0:

| cell | hits / verified / exact | undecided | refutations | planted done/hits/exact | replays | refuted | fallbacks | untraced | branches learned / 2^G | learn runs | selfcheck refuted |
|:--|:--|--:|--:|:--|--:|--:|--:|--:|:--|--:|--:|
| winner | 0 / 0 / 0 (both) | 0 (both) | 1,600 (both) | 16/16/16 (both) | 1,600 | 1,600 | 0 | 0 | 16 / 16 | 16 | 16 |
| t5 G3 | 0 / 0 / 0 (both) | 0 (both) | 800 (both) | 16/16/16 (both) | 800 | 800 | 0 | 0 | 8 / 8 | 8 | 8 |
| t6 G4 s1308 | 0 / 0 / 0 (both) | 0 (both) | 1,600 (both) | 16/16/16 (both) | 1,600 | 1,600 | 0 | 0 | 16 / 16 | 16 | 16 |
| t6 G4 s0 | 0 / 0 / 0 (both) | 0 (both) | 1,600 (both) | 16/16/16 (both) | 1,600 | 1,600 | 0 | 0 | 16 / 16 | 16 | 16 |

`columns`, `covered_points`, `capped = false`, `targets_done = 100` are identical within every pair
(`verdict_diffs = {}` in all four cells). F4 mul-add totals: winner 5,281,600 (plain) vs
2,491,200 (trace) over 1,600 calls; t6 G4 s1308 71,009,508 vs 32,270,382; t6 G4 s0 71,009,400 vs
32,270,191; t5 G3 17,711,200 vs 10,020,800. Matrix sizes shrink with the trace: winner
`max_rows_max` 42 -> 27, `max_nnz_max` 306 -> 181; t6 G4 105 -> 57 and 1,341 -> 662.

## Predictions (decided strictly by the frozen rules in `scripts/analyze_engine.py`; `data/engine/summary.json`)

| | prediction (frozen rule) | outcome |
|:--|:--|:--|
| P1 | verdict identity in all 4 cells (columns, covered, hits, hits_verified, hits_exact, undecided, capped, targets_done, refutations, planted block; planted_hits = planted_done > 0) | **held**: no differences in any cell; planted 16/16/16 everywhere |
| P2 | gate: ρ_win ≤ 0.5 at (t5, G4, seed 1307) | **held**: ρ_win = 24,912 / 52,816 = **0.47168** -> GATE = open |
| P3 | ρ ≤ 0.5 in the other three cells and \|ρ(t6,G4,1308) − ρ(t6,G4,0)\| ≤ 0.02 | **failed**: t5 G3 has ρ = 0.5658 > 0.5 (t6 cells 0.45445 and 0.45445, seed deviation 2.0·10⁻⁶ ≤ 0.02 holds) |
| P4 | branches_learned = 2^G, untraced = 0, replays > 0, fallbacks/replays ≤ 0.01 in every cell | **held**: 16/16 (8/8 at G = 3) learned, 0 untraced, 0 fallbacks over 1,600 (800) replays, all refuted |
| P5 | learn ≤ 8·mpt(plain) and ≤ 1% of E₀(plain) in every cell | **held**: learn = 1.000·mpt(plain) in every cell (winner 52,816 = exactly one try per branch, 16 × 3,301); learn/E₀ ≤ 1.05·10⁻⁴ |
| P6 | trace-0 mpt = T10 value within 10⁻⁶ relative; cols/covered = census | **held**: relative deviation 0.0 in all four cells (52,816.0; 177,112.0; 710,095.08; 710,094.0); census matched. `max_nnz = -` reproduces T10's `3·10⁷` counts exactly |
| P7–P10 | end-to-end (verification 16/16 + replay audit; S(trace)/S(plain) ≤ 0.6 at w = 0.1 and 1 with upper 95% < 1, also hand-charged; w = 0 identity; band ranks) | **pending**: the end-to-end ecbench chunks are still running; no result is claimed (below) |

Exploratory point guess, not a rule: ρ_win in [0.15, 0.35] -> **missed** (0.472). The refutation
trace replays about half of the plain F4 work, not a fifth; `kept_rows` is 768 of the winner's
critical-pair exploration (48 rows per branch), and the replayed matrix still has 27 rows against 42.

P3 fails on the threshold at G = 3 (deeper systems, solving degree 5, replay 0.566), not on seed
independence, which holds to 2·10⁻⁶. Nothing was retuned.

## Model of the end-to-end effect from the engine result (protocol §2 table, ρ_win = 0.4717)

Group operations unchanged, F4 term scaled by ρ, learning (52,816 mul-adds per prepare, 5.3·10⁻⁴ of
one solve's F4 work at w = 1) added by hand: S_trace(w = 0.1) ≈ 0.4717 × 100,277.2 + 26.4 × (1 − 0.4717) = 47,314.6
-> S_trace/S_plain = 47,314.6 / 100,277.2 = 0.4718 (≈ 0.472) at w = 0.1 and w = 1, 1.000 at w = 0
(same walk, relations and verdicts). This README now uses 0.472 from this displayed formula; the
earlier 0.476 came from the PROTOCOL §2 table model (427.5 + 100,215 ρ = 47,696.4, ratio 0.4756),
whose S_plain at ρ = 1 is 100,642, 0.36 % above the measured T12 S_plain 100,277.2. Both are
projections; see the errata section. Modelled distance to rho-neg at w = 0.1: 6.06·10⁴ × 0.4717 ≈ **2.9·10⁴×** (2.86·10⁴–2.87·10⁴ under either model). This is a
projection; the measured value comes from the ecbench stage.

## Status of the end-to-end stage (written 2026-11-04T06:35Z)

- `data/engine/GATE` = `open`, so `run.sh` proceeded as frozen: `ecbench plan --spec ecbench/spec.json`
  accepted the spec (`data/ecbench/plan.txt`, 64 executions, trace arm resolved as
  `pkm-semaev:m=2,mode=full,guess=4,trace=1`; `TRACE_REJECTED` not written), the spec was frozen to
  `data/ecbench/spec.frozen.json`, and `scripts/chunked_run.py` (verbatim T12 copy, sha256
  `e8edaa81515b400bcb29e402f6ff0d8548001498aef662c23f47daf3d19aabde`) started chunk c00 at
  06:33:16Z (`data/ecbench/chunks/driver.log`), behind `/tmp/crypto-bench.lock` which it shares with
  T11's session (T11 chunk c03 was waiting for the lock at the time).
- When this README was written, c00 had been executing for about 75 s and had no `records.jsonl` yet;
  this report had to be returned before the 8 chunks (8 PKM solves per chunk, tens of seconds each
  in T12, plus T11's lock turns) could complete. **P7–P10 are therefore undecided here, not held and
  not failed.** No end-to-end number is quoted.
- The frozen runner finishes the stage unattended: `verify --replay 1` per chunk,
  `band --ranks` at w = 0, 0.1, 1, `pooled_compare.py` (trace ÷ plain, ÷ proj-a, ÷ rho-neg at each w),
  `learn_charge.py` (hand-charged learning, P9 identity), then `data/SHA256SUMS` and `data/ALL_DONE`.
  After a container restart, relaunch `nohup scripts/run.sh > data/run.out 2>&1 &` from the lead
  directory: engine cells with rc=0 are skipped, complete chunks are skipped, killed chunks go to
  `data/ecbench/chunks/interrupted/` and are never cited. P7–P10 are then decided by the frozen rules
  from `data/ecbench/pooled_compare.jsonl`, `band_pooled.json`, `verify/rc.txt` and `learn_charge.json`,
  and this README must be extended with those verbatim numbers (a dated addendum, no retuning).
- Accounting fact (protocol §2): ecbench's band does not price `pkm_trace_learn_muladds` (no
  `_uncharged` suffix). The engine result bounds it at 52,816 mul-adds per prepare, 7.3·10⁻⁵ of E₀,
  so it cannot flip a ratio verdict; `learn_charge.py` reports it by hand regardless.

## Costs (counted units)

- Engine stage: 8 f4cost cells, 800 random + 128 planted targets. F4 mul-adds, random targets:
  5,281,600 + 2,491,200 + 17,711,200 + 10,020,800 + 71,009,508 + 32,270,382 + 71,009,400 + 32,270,191
  = 2.42·10⁸; planted blocks 1.40·10⁷; trace learning 52,816 + 177,112 + 710,096 + 710,096 = 1.65·10⁶.
  `pkm_system_fp_mults_uncharged` per cell: 22,400 (t5 G4), 14,400 (t5 G3), 28,800 (t6 G4), plus
  224/144/288 learn-specialisation F_p mults with trace. No new seed search; no group operations
  (`setup_group_ops = 0`). `wall_s` 1.1–3.7 s per cell is recorded, not interpreted.
- Ecbench stage: 8 chunks × 8 runs = 64 solves (16 per arm), in flight.

## Distance remaining to rho-neg

- Measured (T12, w = 0.1): the winner without trace is **6.06·10⁴× behind rho-neg** and 7.36·10³×
  behind proj-a; 17.1× behind at w = 0. These remain the measured distances until the L12 chunks land.
- With the engine-measured ρ_win = 0.4717 the trace arm is modelled at ≈ 2.9·10⁴× behind rho-neg at
  w = 0.1 and unchanged (17.1×) at w = 0. A halving of the F4 term is a component speed-up of the
  algebraic arm, not a change of track. **Rho remains ahead; nothing here is a leaderboard claim and
  no row is proposed.**

## Exploratory observations (not preregistered)

- Learning cost is exactly one plain target's work in every cell (one try per branch, no retries),
  below the ≤ 8·mpt bound by 8×.
- ρ is nearly the same at G = t − 2 for t = 6 (0.454) and at the winner (0.472), but 0.566 at the
  deeper G = 3 system of t = 5 (solving degree 5): the replayed share grows with the solving degree.
- Planted (decomposable) targets: the trace run spent fewer planted mul-adds than plain (winner
  175,740 vs 255,468; t6 s1308 2,090,891 vs 3,002,999) while finding all 16 exactly, and
  `pkm_trace_fallbacks` stayed 0 in the random-target ledger; the planted block's trace counters are
  not broken out by the instrument, so the fallback path's cost on decomposable targets is not
  separately measured here.
- `pkm_system_specialisations` rises from 1,600 to 1,616 with trace at t = 6 (the 16 learning
  specialisations), consistent with one learn run per branch.

## Deviations

- **End-to-end stage not complete at report time** (above): the orchestration required the summary
  before the 8 chunks could finish; the frozen runner was left executing and is re-launchable. P7–P10
  are reported as not yet decided, not as held or failed.
- `data/SHA256SUMS` was written by hand (same command as `run.sh`'s `finish`) over the data present at
  report time; the runner rewrites it when ALL_DONE is reached.
- Route: run locally on the shared 3-core host as the wave brief directs, not through `isolab-eks`;
  counted units do not depend on the host. Resume checkpoint, Conductor check, session log and visual
  report were not produced: writes are confined to this lead directory by the brief, and the result is
  exploratory, not a benchmark row.
- The t5 G3 cell failing P3 is reported as failed; its ratio (0.566) is not quoted as a gain.

## Files

- `data/engine/<cell>_tr{0,1}.{json,stderr,status}` (8 cells), `data/engine/summary.json`,
  `data/engine/GATE`, `data/engine/analyze.out`.
- `data/ecbench/plan.txt`, `data/ecbench/spec.frozen.json`, `data/ecbench/chunks/` (in flight),
  later `verify/`, `band_pooled.{txt,json}`, `pooled_compare.jsonl`, `learn_charge.json`.
- `data/run.log`, `data/run.out`, `data/env.txt`, `data/family.sha256`, `data/ecbench.sha256`, `data/SHA256SUMS`.

## Validator errata (applied)

Source: `validator-report.json` (pass_with_errata, 2026-11-04T06:50:00Z). Engine stage P1-P6 validated; end-to-end stage P7-P10 not validated (insufficient evidence). Only this README was edited. Prediction verdicts are unchanged (P1, P2, P4, P5, P6 held; P3 failed; point guess missed).

- **Erratum 1 (model formula 0.472 vs 0.476), applied.** Two derivations: (a) the displayed formula 0.4717 x 100,277.2 + 26.4 x (1 - 0.4717) = 47,314.6, ratio 0.4718 (~0.472); (b) the PROTOCOL §2 table model 427.5 + 100,215 rho = 47,696.4, ratio 0.4756 (~0.476), which gives 100,642 rather than the measured S_plain 100,277.2 at rho = 1 (0.36 % high). The README now uses 0.472 (a). The modelled distance 2.9e4x is unaffected (2.86e4-2.87e4). Projection only.
- **Erratum 2 (load), applied.** "load 1.74 at launch" corrected to 1.76 (data/env.txt, data/run.log).
- **Erratum 3 (frozen ecbench files not in commit 5a800bb3), disclosed, not fixable here.** ecbench/spec.json and ecbench/plan_design_check.txt, README.md and data/ are in no commit (git-excluded); their freeze rests on mtime 06:07Z and the identical sha256 of data/ecbench/spec.frozen.json (ed9ad013725655601b05e6d824a84bc65b23dc8a479b9ee218345e04bd0fb2a5).
- **Erratum 4 (P7-P10), applied (status statement).** P7-P10 are pending: the end-to-end chunks are still running (c00 was queued behind /tmp/crypto-bench.lock at validation time). They are neither held nor failed and no end-to-end number is claimed. data/SHA256SUMS was hand-written over the partial tree and will be rewritten by the runner; this README must get a dated addendum when the chunks land.
- **Erratum 5 (process deviations), disclosed, not fixable here.** No isolab-eks route, resume checkpoint, Conductor check, session log or visual report (already listed under Deviations); they do not affect the counted-unit engine measurement.
