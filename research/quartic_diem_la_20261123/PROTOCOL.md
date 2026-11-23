# Protocol (DRAFT, NOT FROZEN): cheaper linear algebra for Diem 2LP IC on prime-order plane quartics

Status: **DRAFT. Not frozen. No measured data exists.** Freezing (writing `frozen.sha256` over
this file, `analyze.py` and `run_*.sh`) is blocked until the library compiles again (main's
`src/cryptanalysis/prime_root_trace.rs` has a bad merge, 24d422e6ee7; owner decision pending)
and the development step of section 4 has fixed the three parameters marked TBD. Every number
this study produces is `[pcc:unverified]`.

Parent: `research/quartic_diem_ladder_20261121/` (arm `2lp-sfb`: `la_add` is 85% of all native
operations at p = 131101 and drives IC/rho = 7.75 at w = 1).

## 1. Question

On the same instances and solver seeds, how much does replacing the linear algebra of `2lp-sfb`
(scalar Wiedemann on `AᵀDA` of the whole peeled core) by trim + structured Gaussian elimination +
non-symmetric coordinate Wiedemann on a square subsystem (`2lp-sfb-sge`) change the counted cost,
and where does each arm sit against the ideal negation rho at w = 0, 0.1, 1?

## 2. Breakdown that motivates the change (existing raw rows, no new run)

`breakdown_from_raw.py` -> `breakdown_131101.json` (structural model of the counted code fitted to
the 10 ladder `2lp-sfb` rows at 131101; graph/DSU and cycle-row terms given generous upper bounds):
Wiedemann sparse products on `AᵀDA` >= 98.2% of `la_add`; Berlekamp-Massey 0.5%; Horner and
projection 0.4%; graph/DSU <= 0.3%; cycle-row building <= 0.6%; peel and t-scan 0.03%. Implied
core: about 1075 x 744 with about 49,500 non-zeros (about 46 per row, about 2% not +-1). Three of
ten runs needed a second attempt, which doubles their linear algebra. The exact split is
re-measured by the instrumented binary in the run (fields `la_<source>_{add,mul}`), which must
also reproduce the ladder's rows exactly (check C0).

## 3. Arms (one binary, `genus_ladder.qla`, source hashes in `source.sha256` at freeze)

- `2lp-sfb` (`Q_diem_2lp_sfb`): unchanged ladder arm. Its `la` is now reported split by source
  (`QLaSplit`); the split sums to the same `la_mul`/`la_add` (test `wiedemann_split_equals_reference`).
- `2lp-sfb-sge` (`Q_diem_2lp_sfb_sge`): identical relation phase and attempt schedule. Per attempt
  after the peel and the t-scan: (a) trim the core to `cols + EXCESS` rows by dropping the heaviest
  rows, never a row with a `t` entry or nonzero right-hand side and never the last row of a column;
  (b) `genus_ladder::sge_reduce` (`w_max` W, fill bound FILL); (c) a square subsystem chosen by the
  same rule; (d) coordinate Wiedemann for `t` only (`2N + 2` products by `A'`, Berlekamp-Massey, one
  dot product); (e) certificate `[t]G = T` (counted group operations). If (d)/(e) fail, the attempt
  falls back, fully counted, to `2lp-sfb`'s own solve of the whole core with the very same draws, so
  the arm never certifies less often. Row selection is charged one addition per entry visited,
  one per row, and `m ceil(log2 m)` additions for the ordering; SGE is charged as in `sge_reduce`.
- Parameters W, FILL, EXCESS: **TBD by section 4**, then written here before freezing.

## 4. Development (before freezing; not in any fit)

Curves of the ladder at p = 32771 (curve seeds 32, 163, 217, 221, 229; that prime is not in the
measured grid) and the ladder's dev curves at 4093 and 8191, seeds 1 and 2. Grid: W in {4, 8},
FILL in {1.5, 3}, EXCESS in {8, 16, 64}. Choose the setting with the lowest geometric-mean
`la_add + la_mul` at 32771 among settings with zero `la_fallbacks`; ties to the smaller EXCESS.
Record the dev rows in `dev/`. Tests `cargo test --release --lib quartic_ladder` must pass,
including `sfb_sge_same_logs_as_sfb`, `coord_square_and_row_selection`,
`wiedemann_split_equals_reference`.

## 5. Grid and commands

Primes 4099, 16411, 65537, 131101; the 5 curves per prime recorded in the ladder's `raw/`
(4099: 35, 205, 220, 257, 295; 16411: 93, 158, 252, 366, 382; 65537: 13, 57, 156, 166, 369;
131101: 53, 177, 193, 228, 288), passed as `--quartic-seeds` (same admission path, no search);
solver seeds 1 and 2; both arms on every (curve, seed), `2lp-sfb` first. 80 runs. At most 2
threads. Commands: `run_main.sh` (4099, 16411, 65537; 2 threads), then `run_131101.sh` (1 thread).
No measured rho is run; the reference is ideal.

## 6. Analysis and decision rule (`analyze.py`)

- Reference: ideal negation rho `R = sqrt(pi r / 4)` group additions, nothing unpriced charged to
  rho. GAE_w = `group_ops` + w x (all native operations: `rel_mul + rel_add + la_mul + la_add +
  idx_lookups + idx_inserts`), w in {0, 0.1, 1} (AGENTS.md section 5a).
- Paired ratio per (curve, seed): GAE_w(sge) / GAE_w(sfb); curve level = geometric mean over seeds;
  per prime = geometric mean over curves with a cluster bootstrap over curves (4000 replicates,
  seed 20261123); pooled = equal weight per prime, curves resampled within prime; slope in ln p.
  Also paired `la_add`, `la_mul`, `la` total, all-native.
- IC/rho per arm and prime at each w with the ladder's per-prime verdict (BELOW-RHO CANDIDATE only
  if all three upper bounds < 1; ABOVE RHO if all lower bounds > 1; else UNRESOLVED).
- Integrity: every row certified, `recovered == t_secret` recorded in the row, both arms recover the
  same log, every pair complete, and C0 (all `2lp-sfb` rows identical to the ladder's on the
  compared fields). Failure -> VOID.
- Decision: **IMPROVEMENT** if the pooled paired ratio has upper 95% < 1 at w = 1 and at w = 0.1
  and upper 95% <= 1.01 at w = 0 (the change does not touch group work; the tolerance is for
  divergent attempt schedules); **KILL** if the pooled w = 1 lower 95% >= 0.90; otherwise
  **INCONCLUSIVE**. An IMPROVEMENT is a component speed-up of the index-calculus baseline, not a
  rho claim; any per-prime BELOW-RHO CANDIDATE is a candidate only, pending the validation gate.

## 7. Outputs

`raw/`, `data.sha256`, `analysis.json`, `RESULTS.md`, `fig_la.svg` and `fig_la.png` (< 50 KB),
a short PDF in `reports/`.
