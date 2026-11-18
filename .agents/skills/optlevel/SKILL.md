---
name: optlevel
description: Measure how well optimized an implementation is on its machine, with the Rust tool in `tools/optlevel/` (`tools/optlevel/run.sh`). Calibrates the host's roofline (peak rate per operation class for integer, modular, XOR/popcount and float multiply-add work, memory bandwidth and latency, per core and all cores), measures a kernel's own counted operations per second (under `perf stat` when a hardware PMU exists, otherwise wall time and counters only), and writes an `optlevel.record/v1` with attainment against the roofline, parallel efficiency, an A/A noise floor, a confidence tier (T-pmu, T-timing, T-gpu-ncu, T-gpu-timing), a named limiting factor and a roofline SVG. Use when asked "how optimized is this kernel", "how much headroom is left on this machine", "is it compute or memory bound", or before deciding whether a rho walk, relation-collection loop, linear-algebra phase, closure engine or CUDA kernel is worth tuning. An optimization level is not a speed claim, not a comparison between algorithms and not a benchmark or leaderboard row.
---

# Optimization level of a kernel

`tools/optlevel/run.sh` (a Rust binary, built on first use) answers one question: of what this machine can do
for the operations this kernel counts, how much does the kernel achieve, and
what holds it back? Definitions, formulas and failure modes:
[`docs/OPTIMIZATION_LEVEL.md`](../../../docs/OPTIMIZATION_LEVEL.md). Read its
section 0 first. Read AGENTS.md §10 as well: isolation, identical outputs and
A/A apply here too.

## 0. What a record is not

- **Not a speed claim.** Speed is cycles per unit (`perf-ipc`) or counted
  units against matched rho (`ecbench-measure`).
- **Not an algorithm comparison.** A tuned bad algorithm scores high. The
  algorithmic axis is separate (`score --best-known-ops`), and cross-method
  questions go through §5a.
- **Not a `bench.record/v1`, a `benchmarks/README.md` row or a leaderboard
  row.** It has no replay certificate and no matched rho reference.
- **Not portable.** The ceiling belongs to one host fingerprint.

| question | command |
| --- | --- |
| what can this host measure? | `probe` |
| what is this machine's ceiling? | `calibrate` |
| how fast does the kernel run its own operations? | `measure -- CMD` |
| how close is that to the ceiling, and why not closer? | `score` |
| show it on the roofline | `plot` |
| does the metric behave on this host? | `controls` |
| where does the time go inside the kernel? | the `profile` skill, not this one |
| cycles and instructions per unit, for a result you will report | the `perf-ipc` skill, not this one |

## 1. Probe and calibrate

The tool is a Rust crate (`tools/optlevel/`, binary `optlevel`); none of it is
Python. `tools/optlevel/run.sh` builds it with `cargo build --release` when the
binary is missing or older than its sources, then runs it. On a host without
cargo, copy the binary there and call it directly (or set `OPTLEVEL_BIN`): one
x86-64 binary is correct on any x86-64 host, because the calibration kernels
select their instruction set at run time.

```sh
tools/optlevel/run.sh probe
tools/optlevel/run.sh calibrate --out research/<study>/optlevel/calibration.json
```

- `cpu_tier: "T-timing"` means no hardware PMU (most cloud VMs, Claude Code's
  containers). Records then carry rates, attainment, scaling and static
  vector share. **IPC, miss rates, top-down and measured memory traffic are
  null; do not estimate them.**
- Calibration takes two to five minutes and uses every usable CPU and several
  GiB of memory. Its kernels are inside the binary (`src/kernels.rs`: 512-bit,
  256-bit and vectorization-free variants, chosen at run time). On a CPU
  family not calibrated before, add `--c-kernels` once: it also builds the C
  reference kernels (`kernels/optk.c`) with the system compiler and takes the
  best of both, so a ceiling can only rise. Run nothing else meanwhile. Flags `CONTENDED` or `CLOCK_UNSTABLE` in its output mean the
  all-core ceilings are unreliable: recalibrate on a quiet host.
- `probe` prints `host.usable_cpus` and why: the affinity mask cut to the
  cgroup CPU quota (v2 `cpu.max`, v1 `cpu.cfs_quota_us`). In a container
  allotted 6 vCPUs on a 48-core host it says 6, and the scale points follow.
  If the allotment shows in neither interface, set `OPTLEVEL_CPUS=N`.
- A calibration taken before 2026-11-17 used the C kernels, which read the
  clock after every pass; its in-cache SIMD ceilings are low by 1.4 to 1.8
  times (`research/optlevel_rs_20261117/README.md`). Recalibrate rather than
  compare attainments across that date.
- Calibrate and measure in the same session, on the same host, under the
  same CPU and NUMA binding. A record scored against another host's
  calibration is flagged `CALIBRATION_HOST_MISMATCH`.

## 2. Make the kernel count its operations

The tool never infers a count from time. In order of preference:

1. The command prints an `optlevel.ops/v1` line on stdout:
   `{"schema":"optlevel.ops/v1","ops":N,"op_class":"modmuladd_p62","kernel_seconds":T,"bytes_declared":B,"working_set_bytes":W}`.
   Only `ops` is required. Without `kernel_seconds` the process wall time is
   used and start-up is charged to the kernel.
2. `--ops-regex 'walk_steps=(\d+)'` sums every match in stdout and stderr.
3. `--ops N` for a fixed workload whose count follows from its inputs.

Choose the unit so the class's ceiling is a fair comparison: field
multiplications, 64-bit word XORs, float multiply-adds. A walk step or a
relation is many operations; scoring it against one class gives a lower
bound, and the registry entry must say so. Classes: `muladd_u32`,
`muladd_u64`, `mulwide_u64`, `modmuladd_p31`, `modmuladd_p62`, `xor_u64`,
`popcnt_u64`, `fma_f64`, `ptr_hop`. There is no carry-less-multiply class yet.

Never edit a sibling repository to add a counter. Wrap it from your own
directory, as `research/optlevel_20261117/real_kernel/closure_wrapper.py` (a wrapper in
the language of the kernel it wraps, there a Python closure; wrap native code natively)
does, and say what the wrapper does not count.

`tools/optlevel/registry.json` lists the hot kernels of crypto, cryptanalysis
and lfd-summation-systems with how each counts operations, or that it does
not. Add an entry when you measure a new kernel.

## 3. Measure

```sh
tools/optlevel/run.sh measure --label "<what>" --repeat 5 \
  --scale env:RAYON_NUM_THREADS:1,4 --out research/<study>/optlevel/<slug>.measurement.json \
  -- target/release/examples/<bench> <args>
```

- `--repeat 5` gives ten single-thread runs in interleaved A and B slots (the
  A/A noise floor) and five runs at each scaled point. Fewer than five pairs
  is flagged.
- `--scale copies:1,4` runs pinned concurrent copies (independent walks,
  relation workers). `--scale env:VAR:1,2,4` sets a thread-count variable
  (`RAYON_NUM_THREADS`, `OPENBLAS_NUM_THREADS`).
- Pass `--bytes-per-op` and `--working-set-bytes` when the command does not
  declare them. With neither, the record is `INTENSITY_UNKNOWN` and its
  attainment is a lower bound.
- The timed region must last at least 0.5 s, or the record has no grade.
- With a PMU the single-thread runs are counted under `perf stat` in two
  event groups (core, memory), one per slot, so nothing is multiplexed.
- The tool records contention (load, PSI, CPU used by others, steal) but does
  not isolate. For anything you report, also follow §10: nothing else on the
  host, or the isolab route.

## 4. Score and plot

```sh
tools/optlevel/run.sh score --calibration CAL.json --measurement M.json --out R.json \
  [--topdown topdown.json] [--vector-share 0.8] [--best-known-ops N --best-known-source "..."]
tools/optlevel/run.sh plot --calibration CAL.json --out roofline.svg R.json [R2.json ...]
tools/optlevel/run.sh validate R.json
```

- `--topdown` takes `topdown.json` from `tools/profile.sh topdown` (T-pmu).
- `--vector-share` takes the value printed by
  `optlevel vector-share BINARY SYMBOL...` (static, from `objdump`).
- Render a PNG beside the SVG from the same file, at 150 dpi.

## 5. Read the record

Read in this order.

1. **Tier**: what could be measured at all.
2. **Flags**: `CEILING_EXCEEDED`, `TOO_SHORT`, `OPS_MISSING` void the grade.
   `CONTENDED`, `CALIBRATION_CONTENDED`, `FULL_POINT_CONTENDED`,
   `INTENSITY_UNKNOWN`, `WORKING_SET_IN_LLC`, `WALL_INCLUDES_STARTUP` weaken it.
3. **Binding roof**: compute or memory. This says which resource bounds the
   kernel on this host, before any tuning.
4. **`A_single` and `A_full`** with their intervals, and the
   roofline-normalised parallel efficiency `A_full / A_single`.
5. **Limiting factor and its basis.** "timing inference" is a hypothesis; "pmu
   top-down" is a measurement.
6. **Noise floor.** Two attainments closer than the larger floor are equal.

Then use the `profile` skill to find the cost inside the kernel, change the
code, and establish the gain with `perf-ipc` or `ecbench-measure`. Re-score
afterwards; the attainment should move the way the cycles per unit did.

Things that are not defects of the code:

- attainment near 1 at the memory roof: the kernel is as fast as the machine
  moves bytes; only higher arithmetic intensity or another algorithm helps;
- raw parallel efficiency under 1 with normalised efficiency near 1: the
  machine saturates, not the kernel;
- a low attainment from a counter that covers a small part of the timed
  region: the counted operations are not the bottleneck. Count the rest.

## 6. GPU

```sh
tools/optlevel/run.sh gpu probe
tools/optlevel/run.sh gpu calibrate --out CAL-gpu.json
tools/optlevel/run.sh gpu measure --op-class muladd_u64 --ops-regex '...' --out M.json -- ./bench
tools/optlevel/run.sh score --calibration CAL-gpu.json --measurement M.json --out R.json \
  --ncu <profile dir>/kernels.json --nsys <profile dir>/summary.json
```

- `gpu probe` says up front what is missing: helper programs
  `tools/profile.sh` needs (`gawk`, `jq`; without them its `ncu` and `nsys`
  subcommands exit 2), and whether `ncu` can read the counters. On
  `ERR_NVGPUCTRPERM` it reports "counters withheld by the driver" and tier
  `T-gpu-timing`: nothing inside a container changes that, so do not spend
  time on `ncu` there. `--no-ncu-check` skips the `ncu` run.
- A memory-bound GPU kernel is scored like a CPU one: print `bytes_declared`
  in the `optlevel.ops/v1` line or pass `--bytes-per-op`. The class `triad`
  (one multiply-add per element, streamed) is in every new GPU calibration;
  `gpu_calib.cu` run with one class name or `triad` is the matching control.
- Every `gpu` subcommand exits 3 on a host where `nvidia-smi` lists no GPU.
  GPU work runs on a GPU host through the lab's runner (AGENTS.md §9, §9a),
  never by launching instances by hand.
- Without `--ncu` the record is `T-gpu-timing`: rates and the `nvidia-smi`
  trace only. With it, `T-gpu-ncu`: SM and memory throughput, occupancy and
  warp stall reasons from `tools/profile.sh ncu`, and the host idle share from
  `tools/profile.sh nsys`.
- `kernels/gpu_calib.cu` first ran on 2026-11-17 on a rented GPU whose driver
  withheld the counters (`research/hky_pod_runs_20261117/`), so its ceilings
  are still unchecked against `ncu`'s speed-of-light figure; calibrations
  carry `GPU_CEILING_UNVALIDATED_BY_AUTHOR` until that comparison exists.
- For the ECC2K-130 walk use `ecc2k130/roofline.py` and the sibling
  repository's `gpueff`: they model the pipes that bound that kernel.

## 7. Record it

- Keep `calibration.json`, every `*.measurement.json` and `*.record.json`, the
  SVG and its PNG in the study directory, with the exact commands in its
  README. State the tier, the flags, the load during the runs and the noise
  floor beside every attainment you quote.
- Cite an attainment as an unverified assertion (`[pcc:unverified]`) unless a
  pcc claim covers it.
- An attainment does not go into `benchmarks/README.md` or the leaderboard.
  If a study opens its own `bench.record/v1`, `optlevel benchrec R.json`
  prints the values in benchrec's units with `--reference-kind none`.

## 8. Validate on a new host class

```sh
tools/optlevel/run.sh controls --out research/<study>/optlevel/controls [--c-kernels]
cargo test --release --manifest-path tools/optlevel/Cargo.toml    # 64 tests; no PMU, GPU or compiler needed
```

`optlevel summary R.json` prints one JSON line per record (tier, attainment,
grade, limiting factor, flags) for job logs.

Expected, from `research/optlevel_20261117/` and
`research/optlevel_rs_20261117/`: the vectorized in-cache control
near 1.0, the same loop with vectorization disabled near the class's
scalar/SIMD ratio, the Horner chain far below, the streaming control near 1.0
on the memory roof, the pointer chase a few percent with latency attainment
near 1. On a PMU host also check that the streaming control's measured
traffic matches its declared 24 bytes per operation
(`intensity.declared_over_measured`), which is the test of the `llc-miss`
source.

## Continuous loop: learn, adapt, act on your own

AGENTS.md "Continuous learning loop"; ledger `tools/lessons.py`.

1. **Before acting**: `python3 tools/lessons.py brief --scope optlevel`. Apply the
   adopted lessons; treat candidates as hypotheses this run can test.
2. **While acting**: resolve a blocker yourself within your authority
   (diagnose the cause, try the documented remedy, then a materially different
   approach) before escalating; AGENTS.md "Persistent scientific research loop".
   Persistence grants no extra execution or spending authority.
3. **After acting**: record the outcome with evidence,
   `python3 tools/lessons.py outcome --scope optlevel --task "..." --result success|partial|fail|blocked --evidence <receipt|commit|job> [--applied L-...]`,
   and `lessons.py test --lesson L-... --outcome confirm|refute --evidence ...`
   for each lesson you applied. A surprise that would change what the next run
   does becomes `lessons.py add --scope optlevel --trigger "..." --action "..." --evidence ...`.
4. **Adapt**: when a lesson scoped to `optlevel` is adopted (two distinct
   confirmations), propose the edit to this file in a PR that cites the lesson
   id; when one is refuted twice more than confirmed, it retires itself.
