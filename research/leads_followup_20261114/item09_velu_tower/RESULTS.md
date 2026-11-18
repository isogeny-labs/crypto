# Item 9 RESULTS: exact last fall degree of the Velu 2-isogeny tower vs the Kummer chain (2026-11-14)

All numbers [pcc:unverified] (assertions from this session, no pcc claim filed). No leaderboard or benchmarks row.
Engine language: **numpy research prototype** (Python 3.13, numpy 2.5.3, exact mod p via blocked float64 BLAS with
k-chunks of 4096, p < 2^21), not landed Rust code. Run with OMP/OPENBLAS threads = 2, `ulimit -v 6000000`.
Repos: ml-cryptanalysis `27a67b9`, crypto `44d35033fa`, cryptanalysis `fb3f88a0` (read-only; nothing committed).
Protocol: `PROTOCOL.md` (frozen before any Velu closure; Amendment for the t = 9 extension was written after the t = 4..8 data).

## Engine and validation gates (all passed)
- `tower_engine.py`: generic tower closure. Columns = squarefree monomials in 2t variables, weight-descending order; shifts =
  sparse reduced products by each variable (Kummer: carries; Velu: x_j^2 = x_j x_{j+1} - xi_j x_{j+1} + xi_j x_j - tau_j,
  x_t = c); closure/processing/`verify_closed` logic as in `ml-cryptanalysis/scripts/ic_bounds_growth_lib.py`.
- **G1 Kummer parity** (`kummer_gate.log`, N8 main instance 12289/4021/9917/4408, full closure, no early exit):
  dim V_3(k) = 9, 11, 13, 15, 17, 19 = 2k+1 (k = 4..9); dim V_4(k) = 163, 386, 605, 829, 1061, 1301 (k = 4..9), profile
  (0,0,2,111,605) etc. = N8-growth.md 6.2; `verify_bad = 0` on every "1 not in V" cell. PASS.
- **G2 rewrite rules** (`g2_g3_check.log`): t = 3..6, every variable x all monomials x all 2^t points of V: 192, 1024, 5120,
  24576 products, 0 mismatches. PASS.
- **G3 tower build**: Q of order 2^t, 2R' not in <Q>, |V| = 2^t distinct, chain reaches the same c with x_j != xi_j,
  images of Q_j, R'_j on the codomain curves: asserted for every instance in `towers.build_tower/velu_data/check_tower`
  (run_cells aborts otherwise; none aborted). PASS.
- Memory fix: the t = 9 first attempt died with a numpy allocation error at rank 11,558 of 12,616 under the 6 GB limit
  (`run_t9_ext_attempt1_OOM.log`). `merge` was rewritten to update the basis in place and in row chunks (same arithmetic;
  `tower_engine_v1_pre_memfix.py` is the version used for t = 4..8). G1 re-run with the new engine gives identical dims
  (`kummer_gate_v2_memfix.log`); t = 9 used the new engine. The t = 4..8 grid was not re-run with it.

## Setup (as frozen)
p = 1048583 (smallest prime >= 2^20 with p = 3 mod 4; v_2(p-1) = 1). t = 4..8, two instances per t
(`random.Random(1000 t + 10 i)`); per cell E', Q, R', xi_j, tau_j, c and the unit instance (a, b, target) are in
`instance_t{t}_i{i}.json`. Every instance: #E' divisible by 2^t (aux curves tried 4..68 per cell), **all 14 instances were unit on
the first draw** (`nonunit_rejected = 0`), unit by enumeration of the 4^t pairs of V x V. The Kummer reference uses the same
(p, a, b, target). Raw rows: `cells_t4_6.jsonl`, `cells_t7_8.jsonl`, `cells_t9_ext.jsonl`; logs `run_*.log`.

## Result: LFD (first D with 1 in V_D), both instances at each t
| t | Kummer LFD (law floor((t+14)/4)) | Velu LFD, instance 1 | Velu LFD, instance 2 |
|--:|--:|--:|--:|
| 4 | 4 (4) | 5 | 5 |
| 5 | 4 (4) | 5 | 5 |
| 6 | 5 (5) | 5 | 5 |
| 7 | 5 (5) | 5 | 5 |
| 8 | 5 (5) | 5 | 5 |
| 9 (exploratory, no Kummer arm) | law 5 | 5 | not run |

- Kummer reference on this engine reproduces the law at every cell (4,4,5,5,5; both instances), at p ~ 2^20 with v_2(p-1) = 1.
- Velu: certified `1 not in V_4` at every t (V_4 = span{g}, dim 1, `verify_bad = 0`; as derived: generator weight 4 vs Kummer's 2),
  and `1 in V_5` at every t = 4..9 (V_5 = all of M_<=5: dim = columns = 219, 638, 1586, 3473, 6885, 12616 at t = 4..9; this
  is automatic once 1 is in V_5). Time of the t = 8 / t = 9 D = 5 closures: 195 s, 741 s (2 threads).
- Reading against the frozen hypotheses: L_V(t) = 5 for t = 4..8 on both instances, no step. By the frozen definition that is
  **H_flat** (no step in t = 4..8). H_same (a step between t = 5 and 6) and H_steep are falsified; the registered point
  prediction (L_V = 5 flat) holds. Registered probabilities were H_flat 0.50, H_same 0.20, H_steep 0.20.

## What this does and does not say (measured vs derived)
- Measured: the table above. Derived (and measured at D = 4): the Velu floor LFD >= 5 comes from the generator weight.
- Caveat on the reading: the Velu sequence sits at its floor (generator weight + 1 = 5) for the whole range, so the
  data show only that Velu's first step is at t >= 10 (not that the step pattern differs from Kummer's in position: Kummer's
  first step 4 -> 5 at t = 6 is hidden under the floor, and its second step 5 -> 6 is at t = 10). Velu is flat while Kummer
  rises by one; in absolute LFD Velu is at or above Kummer (+1 at t = 4, 5; equal at t = 6..9).
  Whether Velu steps 5 -> 6 at t = 10 (as Kummer) or later is the open cell: t = 10 needs D = 5 on 21,700 columns (dense basis
  about 3.8 GB, estimated several hours) and was not run. Next command (needs a >= 12 GB memory allowance and a sparse or
  GPU linear-algebra back end, so not on this host's 6 GB budget):
  `python3 run_cells.py 10 cells_t10.jsonl 5 velu 1`.
- Not tested here: the weight-matched map-image reference of the synthesis record, the supersingular E' (y^2 = x^3 + x) arm, and
  stage 2 (t = 14). The Q4 conclusion "(b) carried by the graded algebra vs presentation-free" is therefore NOT decided by
  this item; the data fit "LFD <= 5 flat through t = 9" for the Velu presentation, consistent with the F4 plateau D = 5.
- The dim V_5 = full space is not an independent finding (1 in V_5 implies it). No cost, speedup or P-192 claim.

## Files (sha256 prefix)
tower_engine.py 616f001ff961db73; tower_engine_v1_pre_memfix.py dfb39b6999bdbfb3; towers.py 6ec550387e3da767;
run_cells.py 64cd6c76a3ec6faf (CLI args for kinds/instances added after the t = 4..8 run; logic unchanged);
cells_t4_6.jsonl f9e7dc550022d8f8; cells_t7_8.jsonl 0e8a5669cc729b3c; cells_t9_ext.jsonl 8c339780c9c6e953;
kummer_gate.log 07fd2a342660e6c2; kummer_gate_v2_memfix.log f19478ca76d4567b; g2_g3_check.log 8a2331a0e16cd414; PROTOCOL.md ba651de3c40c126a.
Commands: `python3 kummer_gate.py`; `python3 g2_g3_check.py`; `python3 run_cells.py 4,5,6 cells_t4_6.jsonl 6`;
`python3 run_cells.py 7,8 cells_t7_8.jsonl 6` (PID 7925, finished); `python3 run_cells.py 9 cells_t9_ext.jsonl 5 velu 1` (PID 9121, finished; attempt 1 PID 8567 died of OOM).
Not done: the CLAUDE.md session-log entry (write scope restricted to this directory; the main session should log this item).
