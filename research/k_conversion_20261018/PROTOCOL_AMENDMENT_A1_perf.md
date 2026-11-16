# Amendment A1: K at degree 4 with hardware counters (frozen 2026-10-18)

Frozen before any degree-4 perf datum exists. PROTOCOL.md stays in force except
where this file says otherwise. The plumbing (`examples/k_conversion_perf.rs`,
`perf/run_perf.sh`, `perf/analyze.py`) was tested on this container only with
software events (`KPERF_SOFTWARE=1`, labelled `"selftest":true`); no hardware
counter has been read yet.

Classification: a **measurement** (a ratio of two instruction counts), as in
PROTOCOL.md. K stays accounting: it changes the unit of a priced row and
lowers no cost. It is not an improvement and not a beat-rho claim.

## 1. Question

1. What is K = (instructions per dense-closure column) / (instructions per rho
   step) when the closure is built at **degree 4** on the N = 34 to 44 cells
   that Callgrind could not reach?
2. How does `I_col`, and so K, grow with `columns` from degree 3
   (4861 to 6585 columns, Callgrind K 4.0 to 15.6 at the n = 83 step) to degree 4
   (tens of thousands of columns), and from the N = 26 degree-4 Callgrind cell
   (17,902 columns, K = 82.9)?

The note's `w = 1` prices one touch per column independent of size. The
degree-3 study already showed `I_col` is not size independent; this
amendment measures the trend at the degree the ECC2K-130 m = 4 rows use.

## 2. Instrument

- `perf_event_open(2)` interval counters inside `k_conversion_perf`:
  `PERF_COUNT_HW_INSTRUCTIONS` (the unit) and `PERF_COUNT_HW_CPU_CYCLES`
  (reported, **never used in K**), user space only (`exclude_kernel`,
  `exclude_hv`), `inherit=1` (rayon threads are counted), `disabled=1` and
  enabled by `ioctl` immediately before the call and disabled immediately
  after. `read_format` carries `time_enabled` and `time_running`; a counter
  with `time_running` more than 0.1% short of `time_enabled` is multiplexed
  and the run is void.
- The interval is the one the Callgrind toggle used:
  - closure: exactly `ReducedBasis::from_system_closed_full(&sys.equations,
    sys.n_vars, D, 1_000_000)` (dense full-MutantXL closure, matrix build
    included). Parsing, the system rebuild (`find_irreducible_sparse`,
    `FieldStructure`, `build_decomposition_system`, `n_vars` assert) and the
    plain XL basis are outside. Callgrind collected the inlined callee
    `from_system_closed_impl`; the same interval, checked by control 5.2.
  - rho: exactly `StrongRhoG::solve` through the `#[inline(never)]` wrappers
    copied from `k_conversion_rho.rs` (n = 83 capped at 10^6 steps, `walk_steps`
    = 10^6; n = 61 complete, `walk_steps` from the outcome, answer verified).
- Engine counters (`columns`, `closure_word_ops`, `closure_products`) come from the
  engine as before; `I_col = instructions / columns`, `I_step = instructions /
  walk_steps`, `K = I_col / I_step`. `I_step` is the median over rho seeds 1 to 3
  at the same field (n = 83 primary, n = 61 secondary), measured by the same
  tool on the same host in the same session.
- One binary for all runs (SHA-256 in `host.txt`); `RAYON_NUM_THREADS=1`;
  `taskset` to one CPU per job when available. `PAR` (concurrent jobs, each on its
  own CPU) is recorded in every record; sequential (`PAR=1`) is the reportable
  default, `PAR>1` only on a host with separate physical cores.

Counters never estimated: on a host that cannot count, every mode prints
`"status":"unsupported"` with the errno and exits 3.

## 3. Cells and repeats

The four committed-draw cells of PROTOCOL.md section 3, files and draws unchanged
(same as `run_k.py`):

| Cell | N | draws | file |
|---|--:|---|---|
| C: m3-16-6 | 34 | 0, 1, 2, 3 | `research/dreg_closure_scale_20261016/draws/m3-16-6.jsonl` |
| B: m4-12-3 | 36 | 1, 2, 3, 4 | `research/dreg_closure_scale_20261016/draws/m4-12-3.jsonl` |
| A: m4-11-5 | 42 | 8, 67, 107, 143 | `research/dreg_closure_driver_20261016/draws/m4-11-5.jsonl` |
| B': m4-12-5 | 44 | 24, 26, 47, 52 | `research/dreg_closure_driver_20261016/draws/m4-12-5.jsonl` |

- **Degree 4** on all 16 draws (the new datum).
- **Degree 3** on the same 16 draws (positive control against the committed
  Callgrind `Ir` in `raw.jsonl`).
- **Rho** n = 83 seeds 1, 2, 3 and n = 61 seeds 1, 2, 3.
- **Every job is repeated 3 times** (A/A); repeats are the outermost loop so
  slow drift shows. Degree-4 columns are whatever the engine builds
  (`closed.columns()`); the draw-time `n_vars` assert guards the system.

Columns and cost at degree 4 (arithmetic in section 7): `C(N, <=4)` is an upper
bound on `columns`; at degree 3 the engine built 0.43 to 0.84 of `C(N, <=3)`.

## 4. Host requirement (hard)

A Linux host with a **hardware PMU** and `/proc/sys/kernel/perf_event_paranoid
<= 2`, i.e. the owner's own machine: bare metal, or a VM that exposes a vPMU.
Not valid hosts:

- **RunPod containers**: probe 2026-10-18 found the `cpu` event source present
  but `perf_event_paranoid = 4`, so `perf_event_open` returns EPERM.
- **This container** (Claude Code cloud): no PMU at all (`perf_event_open`
  returns ENOENT for hardware events).

`run_perf.sh` starts with `k_conversion_perf probe`; on exit 3 it writes an
`unsupported` host record and stops. Recorded per run: `lscpu`, `uname`,
paranoid level, governor and turbo state, rustc, commit and dirty-file count,
binary SHA-256, `PAR`, load average. Isolation tier is whatever the owner's host
provides (record it by hand if `isolated_bench` is used); instruction counts do
not depend on contention, cycles do, so cycles carry no claim here.

## 5. Controls (all must pass or the run is not reported as K)

1. **A/A.** Each job's three repeats of `instructions` agree within 0.5%
   (max-min over median), for closures and rho alike. Engine counters
   (`columns`, `closure_word_ops`) must be identical across repeats.
2. **Positive control.** Degree-3 `instructions` within +-5% of the committed
   Callgrind `Ir` on each of the 16 draws, with `columns` and `closure_word_ops`
   equal to the committed values. Outside +-5% means the interval differs from
   the Callgrind toggle (or the binary's codegen differs materially): the run
   is **not reported as K**, the discrepancy is reported instead. The ratio is
   reported per draw whatever the outcome. (Same binary is not required; the
   ratio is the check.)
3. **Null control.** The interval is a strict subset of the process: the m3-16-6
   draw 0 closure at degree 3 and at degree 4 is also run once whole under
   `tools/perf_ipc.sh` (`instructions:u`), and interval < whole is required
   (reported as a share). This needs a working `perf` binary on the host.
4. **No multiplexing** in any record (`multiplexed:false`), and no job with exit
   != 0 or `status != ok`.
5. Rho: every n = 61 solve verified, n = 83 `capped:true`; `walk_steps` from the
   engine, not estimated. The Callgrind-versus-perf ratio of `I_step` is
   reported for information.

`perf/analyze.py` evaluates these and sets `reportable` in `RESULT.json`.

## 6. Reporting rule

- Per draw: `columns`, `instructions`, `I_col`, `K` (n = 83 step, n = 61 step as
  second). Per (cell, degree): median with min and max over the 4 draws (a
  range, not an interval). The A/A spread, the degree-3 perf/Callgrind ratio
  per draw and the null-control share are reported with it. `I_col` against
  `columns` (degree 4 cells in order of columns, plus the degree-3 and N = 26
  Callgrind points) shows the growth.
- Converted rows (`row + log2 K`, PROTOCOL.md section 5) are **derived
  accounting**, stated with the K used and the range. K measured at tens of thousands of columns
  still understates `I_col` at about 2^49 columns, so converted rows stay lower
  bounds on the oracle side; the n = 83 denominator remains the n = 131
  proxy. The classification stays accounting; it is not an improvement and
  moves neither row below rho. The distance to rho is reported as the distance
  remaining.
- A control failure, a void run, an unsupported host, or a cell that did not fit
  is reported as such, per cell; nothing is imputed from degree 3 or from
  Callgrind. Cycles and IPC may be listed from the same records, labelled
  cycles, with no K derived from them.

## 7. Size and time estimate (upper bound: `columns = C(N, <=4)`)

`C(N,<=4) = 1 + N + C(N,2) + C(N,3) + C(N,4)`.

| Cell | N | columns | memory, 3 c^2/8 x 0.6 | memory, measured ratio | time/draw (cubic) | 4 draws |
|---|--:|--:|--:|--:|--:|--:|
| C | 34 | 52,956 | 0.63 GB | 1.2 GB | 144 s | 0.16 h |
| B | 36 | 66,712 | 1.00 GB | 1.9 GB | 287 s | 0.32 h |
| A | 42 | 124,314 | 3.48 GB | 6.6 GB | 1,859 s | 2.07 h |
| B' | 44 | 149,986 | 5.06 GB | 9.6 GB | 3,266 s | 3.63 h |

- Arithmetic: `c^2/8` bytes is one dense bit matrix; the requested figure is
  3 x 0.6 of it. A calibration run here (N = 26, degree 4, draw 56 of
  `cell-11-5-6.u3`, 17,902 columns) had `VmHWM` 141.8 MB = 3.4 x `c^2/8` + 7 MB
  baseline, so the "measured ratio" column uses 3.4 x `c^2/8` and is the
  safer sizing: **allow about 10 GB for B' (16 GB host recommended)**.
- Time: that same run took 5.553 s; the dense closure is cubic in `columns`
  (`time = 5.553 s x (c / 17,902)^3`), which gives 144 s at N = 34, in line with
  the "90 s or more" of PROTOCOL.md section 3. One calibration point, so
  treat the table as +-2x; B (`m4-12-3`) is the denser system and cost about 3.5x
  per Ir at degree 3, so it may take up to 3.5x its row (0.32 h to about 1.1 h).
- Sum over the 16 draws: 6.17 h per repeat; **x 3 repeats = 18.5 h**
  (20.9 h with B at 3.5x; up to about 37 h if the cubic estimate is low by 2x).
  If the engine builds fewer than `C(N,<=4)` columns, as at degree 3 (0.43 to
  0.84 of `C(N,<=3)`), the cubic scaling shrinks this to as little as about 2 h;
  the true figure is known after the first C draw. Plan for one day.
- Degree 3 (16 draws x 3, seconds each), rho (n = 83: about 7 x 10^9
  instructions, a few seconds per run; n = 61 under a second) and the whole-process
  null controls (about 3 min) add under 15 minutes.
- Parallelism: `PAR=4` on four free physical cores cuts the wall time to about
  5 to 6 h but needs the memory of the four largest concurrent jobs (up to about 38 GB
  for four B' jobs) and shared L3/DRAM may perturb cycles (not instructions).
  The reportable default is `PAR=1`.

## 8. Commands

```sh
cargo build --release --example k_conversion_perf
research/k_conversion_20261018/perf/run_perf.sh            # PAR=1 REPS=3, writes perf/runs/<host>-<UTC>/
python3 research/k_conversion_20261018/perf/analyze.py research/k_conversion_20261018/perf/runs/<host>-<UTC>
```

`run_perf.sh --dry` exercises the plumbing with software events (selftest).

## 9. What this does not say

Nothing about n = 131 rho step cost, wall-clock time, attainability of `w = 1`,
the closure degree up to N near 394, or beating rho. K at N = 34 to 44 is not K
at N = 394.
