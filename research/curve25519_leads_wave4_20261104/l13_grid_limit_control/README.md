# L13: the grid-limit control (k = 20, isogeny base t = 5, seed 1307; results, 2026-11-04)

**Class.** Control measurement in counted units (ecbench GAE plus unpriced native work at
w = 0, 0.1, 1) on the k = 20 Curve25519 miniature. Not a leaderboard row, not a `benchmarks/`
row. Rho remains ahead.

Protocol: [PROTOCOL.md](PROTOCOL.md), frozen with `scripts/` at `5a800bb3` before any L13 data.
Instrument: `/home/user/ctarget/release/ecbench`, sha256 `54737834…9641b` (= T12 and L12), checked
by `scripts/run.sh` at each launch. Decisions: `analysis/decide.py` → `analysis/verdicts.json`,
strictly by PROTOCOL.md §4. Every number below is read from `data/ecbench/{band_pooled.txt,
pooled_compare.jsonl, identity_check.json, verify/rc.txt}`.

## Headline

- **The F4 winner sits 42× above its own grid limit at w ≥ 0.1** (at w = 0 the two are identical). On the same 22-column isogeny base, the
  exhaustive grid oracle makes the same decisions (identical trials, relations and GAE in all 32
  pairs) for 0.0238 of the F4 arm's S at w = 0.1 and 0.0236 at w = 1 (point prediction 0.0245).
- **The generic pair table sits a further 76× below the grid at w = 0.1** (0.0131) and 356× at
  w = 1 (0.0028). At w = 0 it costs 4.7 % more: its 506 table additions, net of about 44 lift
  additions per run that the grid and F4 arms pay and the table does not.
- **Even the cheapest oracle on this base loses to proj-a at every weight.** With the oracle held
  fixed (pair table vs proj-a's table), S(mitm on the isogeny base)/S(proj-a) = 2.39 to 3.32. The
  22-column unprojected base, not the solver, is what keeps the algebraic track behind the
  generic baseline at w = 0.
- 6 of 8 predictions held. **P4 failed** on its per-pair offset clause, and **P8 failed** on its
  bookkeeping rule, which used the wrong denominator. Every ratio rule in both held.

## Band (pooled, 16 chunks × 2 rounds = 32 runs per arm, √r = 362.22)

| arm | verified | S(w = 0) | S(w = 0.1) | S(w = 1) |
|:--|--:|--:|--:|--:|
| pkm-iso-t5-g4-s1307 (F4 winner) | 32/32 | 27.04 | 103,353.4 | 1,033,290.5 |
| grid-iso-t5-s1307 (exhaustive grid) | 32/32 | 27.04 | 2,462.7 | 24,383.6 |
| mitm-iso-t5-s1307 (pair table) | 32/32 | 28.31 | 32.33 | 68.50 |
| proj-a | 32/32 | 11.84 | 12.72 | 20.65 |
| rho-neg | 32/32 | 1.580 | 1.694 | 2.724 |

## Predictions (PROTOCOL.md §4)

Ratios are pooled paired ratios B ÷ A with 95 % intervals (K = 16 workloads, 32 pairs).

| | rule | outcome |
|:--|:--|:--|
| P1 | 32/32 verified per arm; 16 replays rc 0 | **held**: 32/32 in all five arms; c00–c15 rc = 0 |
| P2 | grid ÷ F4 ≤ 0.1 at w = 0.1 and 1, upper < 1 (point 0.0245) | **held**: **0.02383 [0.02382, 0.02384]** at w = 0.1, **0.02360 [0.02359, 0.02360]** at w = 1, 16/16 workloads; inside the exploratory [0.020, 0.030] |
| P3 | same-base arms identical trials, relations; GAE(grid) = GAE(F4) | **held**: seeds paired; 32/32 identical trials, 32/32 identical relations, 32/32 equal GAE; w = 0 ratio 1.000 [1.000, 1.000] |
| P4 | mitm ÷ grid ≤ 0.05 (w = 0.1), ≤ 0.01 (w = 1), upper < 1; in [1.02, 1.10] at w = 0; every pair's GAE(mitm) − GAE(grid) in 506 ± 2 × relations; band unresolved | **failed** on the per-pair clause: 0/32 pairs in the band, the differences are 437–489. The ratio clauses held: 0.01313 [0.01275, 0.01352] at w = 0.1, 0.00281 [0.00277, 0.00285] at w = 1, 1.0471 [1.0426, 1.0517] at w = 0. The band verdict was "unresolved inside the band", as predicted |
| P5 | grid ÷ proj-a ≥ 1.5 at w = 0 (lower > 1), ≥ 50 at w = 0.1, ≥ 300 at w = 1; proj-a ahead in the band | **held**: 2.284 [2.034, 2.565], 193.6 [167.6, 223.8], 1,181 [1,011, 1,380]; "proj-a < grid" |
| P6 | mitm ÷ proj-a ≥ 1.5 at every w, lower > 1; proj-a ahead | **held**: 2.392 [2.138, 2.676], 2.542 [2.259, 2.861], 3.318 [2.874, 3.831]; "proj-a < mitm" |
| P7 | rho-neg ahead of every IC arm at every w; distances reported | **held**: rho-neg first in every pair. Distances below |
| P8 | every grid run: solver.ops = 0 and `pkm_grid_fp_mults_uncharged` = 1,248 × trials | **failed**: solver.ops = 0 in 32/32, but the multiplication count equals 1,248 × trials in 0/32 |

### Why P4 and P8 failed (no retuning; the verdicts stand)

- **P8.** In all 32 grid runs `pkm_grid_fp_mults_uncharged` is exactly 1,248 × `solver_calls`.
  `solver_calls` is trials minus `repeated_targets_skipped`. For example, c00 round 0 has
  10,147,488 = 1,248 × 8,131, with 8,415 trials and 284 skipped. The invariant is per oracle call.
  PROTOCOL.md §1 says "1,248 per consistent system" and §4 uses trials. The design-stage validator
  flagged the §4 denominator at 06:55Z, before any data; the rule was left frozen and is reported
  as failed. The grid's work is still priced exactly once by the
  band, which is what P8 was meant to check.
- **P4.** The table arm's `oracle_setup` phase is 506 GAE as modelled. Its `relations` phase,
  however, charges 17–69 GAE fewer than the grid arm's on the same trials. That puts mitm minus
  grid at 437–489 instead of 506 ± 2 × relations. Mechanism (validator, from source): after the
  grid or F4 step returns abscissae, `lift_abscissae` re-adds sign combinations on the counted
  ledger, about 5 charged additions per relation, while the pair-table probe returns the signed
  pair and charges 1. The predicted net is 4.0 per relation, and 3.97 ± 0.12 was measured
  (1,422 additions over 358 relations). The same cost is in the F4 arm, which is why
  GAE(grid) = GAE(F4) exactly. The ratio clauses, which carry P4's content, held.

## Distance remaining (pooled ratio arm ÷ rho-neg, 95 % interval)

| arm | w = 0 | w = 0.1 | w = 1 |
|:--|--:|--:|--:|
| F4 winner | 17.1× [13.9, 21.0] | 6.10·10⁴× [4.86·10⁴, 7.66·10⁴] | 3.79·10⁵× [2.96·10⁵, 4.86·10⁵] |
| grid | 17.1× [13.9, 21.0] | 1.45·10³× [1.16·10³, 1.82·10³] | 8.95·10³× [6.99·10³, 1.15·10⁴] |
| pair table | 17.9× [14.6, 22.0] | 19.1× [15.5, 23.5] | 25.1× [19.8, 31.9] |

The F4 winner's re-measured distance at w = 0.1 (6.10·10⁴×) reproduces T12's 6.06·10⁴×. The grid
distance matches the frozen point of 1.49·10³×. proj-a is 8.1·10³× ahead of the F4 winner at
w = 0.1 and 2.28× ahead at w = 0.

## Reading

- The solver is not the main problem at w = 0, and it is the whole problem at w > 0. At w = 0 the
  F4, grid and table arms cost the same group operations, and all three are 2.3–2.4× behind
  proj-a. The preregistered model attributes that gap to the base, with 22 columns and coverage
  210/r, against proj-a's projected 32-column base. This run does not isolate the cause; the
  measured trials per relation are 650.4 against proj-a's 64.4, 10.1× against the model's 9.3×.
  At w ≥ 0.1, F4 costs 42× its own grid, about 3,200× the table at w = 0.1 and 15,100× at w = 1.
- So none of the three m = 2 oracles measured here on this base (t = 5, seed 1307, k = 20)
  beats proj-a, and an oracle cannot go below the w = 0 row, already 2.3× behind. An improvement
  on the algebraic track has to change the base's coverage per column, not only the solver.
  Cross-lead arithmetic: L12's trace replay (0.474×, its own session) closes about one fifth of
  the decades between F4 and its grid limit.
- The `[0.02382, 0.02384]` interval is conditional on the fixed 16 targets. The grid and F4 arms
  share trials, relations and GAE and differ only by the solver, so the ratio is an arithmetic
  stability check, not a method-level sampling interval.

## Run record and deviations

- Launched 06:37:02Z. The run waited behind `/tmp/crypto-bench.lock` (T11, L12) and was
  relaunched after container restarts (`data/env.txt`, two launches). Chunks c00–c07 were
  complete by 09:17:53Z. `driver.log` has no entry between then and the 15:29:54Z relaunch, and
  no c08 session directory existed at the relaunch, so `interrupted/` is empty. The driver was
  presumably killed by a container restart while c08 was starting; the logs do not establish
  this. c08 ran from scratch after the relaunch, and no partial chunk is cited. All 16 chunks were complete at 16:12:55Z, and `ALL_DONE` was written at
  16:14:50Z.
- L13, T13 and L12 sessions were serialised by `/tmp/crypto-bench.lock`; no two sessions
  overlapped (T13 validator, E4). S is in counted units; no wall-time claim is made.
- `analysis/decide.py` was written after the freeze, outside `scripts/`, before any data
  existed. It encodes §4 verbatim; its P8 test reproduces the frozen rule's defect on purpose.
- Conductor and the AWS CLI are absent in this container: no S3 resume checkpoint was pushed,
  and the run was local rather than on EKS.

## Files

`PROTOCOL.md` (frozen). `scripts/` (frozen: `run.sh`, `chunked_run.py`, `pooled_compare.py`,
`identity_check.py`, `cells_from_t12.py`). `design/` (pre-freeze design numbers). `analysis/`
(`decide.py`, `verdicts.json`). `data/ecbench/chunks/c00…c15` (sessions), `band_pooled.{txt,json}`,
`pooled_compare.jsonl`, `identity_check.json`, `verify/cNN.{json,txt}`, `verify/rc.txt`;
`data/SHA256SUMS`; `data/run.log`, `data/env.txt`.

## Validation (2026-11-04)

`validator-report.json`: **pass_with_errata**, disposition validated at `fd6a650cb2`. Band and all
27 pooled ratio rows recomputed with independent code (agreement to 10⁻¹⁴); P1–P8 decided
independently, with the same verdicts; 160/160 certificates; extra replays identical (c01 grid
and mitm in both rounds; c15 all 10 runs, including both F4 runs). Errata applied above: E2 (P8
history), E3 (P4 mechanism), E4 (scope of the proj-a claim; trials per relation 10.1×), E5
(cross-lead arithmetic), E6 (15,100× at w = 1), E7 (net table cost at w = 0), E8 (42× at
w ≥ 0.1), E9 (run-record gap). Disclosed here:
- E1: `data/SHA256SUMS` verifies 158 of 161. `env.txt`, `run.log` and `run.out` each had a final
  line appended after the sums were written and match without it; `ALL_DONE` is not listed.
- E10: `ecbench/spec.json` was first committed at 07:13:56Z, after the freeze commit, but its
  sha256 is pinned in the frozen PROTOCOL.md and matches. The `records.jsonl` files (about 96 KB)
  are committed in git, as T12's are. Sessions record `git_commit` null, so the instrument is
  bound by sha256 only. Only the final scalar is certified; relation-level certificates are not
  exported.
