---
name: profile
description: Profile where a CPU or GPU workload spends its time, with `tools/profile.sh`. CPU sampling with `perf record` (hotspot table with binomial errors, flame graph) or Callgrind, top-down with `perf stat -M`, NVIDIA Nsight Systems (`nsys`) timelines and kernel coverage, Nsight Compute (`ncu`) per-kernel speed-of-light, occupancy, pipe use and warp stalls, and `nvidia-smi` clock/power traces. Each run leaves a manifest (host, tool versions, binary hash, commit), JSON, SVG graphs and a Typst report. Use before optimizing a rho walk, relation-collection loop, linear-algebra phase or CUDA kernel, when asked "where does the time go", "why is this kernel slow", or to run ncu/nsys/perf. A profile explains a cost. It never proves a speedup. Refuses to estimate counters the host cannot read.
---

# Profiling CPU and GPU workloads

`tools/profile.sh` wraps the profilers this repository uses behind one
contract. Each subcommand writes a directory holding:

- `manifest.json` (`profile/v1`): host, CPU features, GPUs, tool versions,
  binary path and SHA-256, commit and dirty count, the exact command, exit
  status, isolation, and the profiler settings;
- the profiler's raw output;
- a summary JSON;
- SVG graphs of the recorded values;
- `summary.md`, and `report.typ` for the PDF.

On a host that cannot produce a number, the manifest says
`"status": "unsupported"` with the reason, and the tool exits 3. It never
estimates. Read AGENTS.md §10 first. The isolation, identity and A/A rules
there apply to any number taken from a profile.

## 0. A profile explains; it does not measure a speedup

Every profiler perturbs what it observes:

- `ncu` replays each launch, locks clocks to base and flushes caches;
- `nsys` and `perf record` add tracing and sampling overhead;
- Callgrind runs on a simulated CPU 20–100× slower.

So:

- **Use a profile to find the cost and form a hypothesis.** "`gf2_mul` takes
  44% of samples, and the stall is the multiply chain."
- **Test the hypothesis with a measurement.** Use `perf-ipc` (cycles per
  unit), `ecbench-measure` (counted units against matched rho), or an
  interleaved A/B under `isolated_bench` with an A/A noise floor.
- **Never quote a profiled run's wall time, an ncu kernel duration, or an
  nsys total as a throughput or speedup figure.** The `wall_seconds` in a
  manifest is labelled "not a timing measurement" for this reason.

| question | subcommand |
| --- | --- |
| what can this host profile? | `probe` (add `--gpu-control` on a GPU host) |
| which functions burn CPU time? | `cpu` (perf sampling; Callgrind fallback) |
| which functions execute the instructions (deterministic)? | `cpu --mode callgrind` |
| front-end, bad speculation, back-end or retiring? | `topdown` (needs a hardware PMU) |
| cycles, instructions, IPC per unit of work | the `perf-ipc` skill (`tools/perf_ipc.sh`), not this one |
| is the GPU busy, or waiting on the host? | `nsys` |
| why is this kernel slow? | `ncu` on the dominant kernel from `nsys` |
| did the clock or power limit move during a throughput run? | `smi` |

## 1. Probe the host

```sh
tools/profile.sh probe --out research/<study>/profiles/probe-<host>
```

`capabilities` in `probe.json` decides what follows:

- **`cpu_sampling: "software timer (cpu-clock)"`.** The host has no hardware
  PMU: most cloud VMs, including Claude Code's cloud containers. `perf
  record -e cpu-clock` still samples where time goes, at the timer's
  resolution. There is no cycle, cache or branch attribution, and `topdown`
  and `perf-ipc` exit 3. This is enough for "which function". It is not
  enough for "why".
- **`cpu_hardware_counters: true`.** Sampling uses `cycles:u`, and
  `topdown` works.
- **No working perf.** On Ubuntu, `apt-get install linux-tools-common
  linux-tools-generic`. The tool finds the versioned
  `/usr/lib/linux-tools-*/perf`, which runs on other kernels. Set `PERF=` to
  choose a binary. If `perf_event_paranoid` > 2, see the `perf-ipc` skill;
  never profile as root to get around it.
- **GPU.** `probe --gpu-control` builds a one-block control kernel, checks
  its output, and profiles it with ncu. This separates "this host withholds
  counters" (`counter_permission_denied`) and "this profiler cannot prepare
  any kernel here" (`failed_to_prepare`) from "my kernel will not profile".
  On 2026-09-08 a Modal RTX PRO 6000 failed even this control
  (ecc2k130/TUNING.md); Modal runs gVisor.

GPU work does not run in a cloud container. Run it on a GPU host through the
ephemeral runner (AGENTS.md §9a; `--type g6.xlarge` and the like boot
NVIDIA's Deep Learning Base AMI), with the profile command inside the job,
and fetch the output directory. Do not launch instances by hand.

## 2. Build what you will profile

Profile the release build, with symbols that do not change code generation:

- **Rust.** `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only cargo build
  --release ...` adds file and line tables without changing codegen. `-C
  force-frame-pointers=yes` does change codegen (one register fewer). Use it
  only for `--call-graph fp`, and record that the profiled binary differs
  from the measured one. The manifest's SHA-256 shows which binary ran.
- **C/C++.** `-O2 -g`, optionally with `-fno-omit-frame-pointer`.
- **CUDA.** `nvcc -lineinfo`, never `-G`: `-G` disables optimization, so its
  profile describes a different kernel. Keep `-Xptxas -v` output beside the
  profile. Register counts and spill bytes are the static half of what ncu
  shows dynamically.

Fix the workload: the same seeds, inputs and thread count
(`RAYON_NUM_THREADS=1` unless threading is the question). Check that the
command's output (digests, counted units) matches an unprofiled run.

## 3. CPU: where the time goes

```sh
tools/profile.sh cpu --out research/<study>/profiles/<slug>-cpu --label "<what>" \
  --call-graph dwarf -- env RAYON_NUM_THREADS=1 target/release/examples/<bench> <args>
```

The run writes:

- `hotspots.json`: self samples per symbol, with `self_pct` and `se_pct`, the
  binomial standard error `sqrt(p(1-p)/N)`;
- `hotspots.svg`, with ±1 s.e. bars;
- `folded.txt` (folded stacks) and `flame.svg`. The flame graph comes from
  `inferno-flamegraph` when it is installed (`cargo install inferno`),
  otherwise from the built-in renderer;
- `report.txt`: `perf report` with call chains.

Options:

- `--freq` sets the sampling rate (default 999 Hz).
- `--event` overrides the event (default `cycles:u` with a PMU, `cpu-clock`
  without).
- `--isolate` runs under `isolated_bench`.
- `--mode callgrind` gives deterministic instruction counts (`Ir`) per
  function.

Read the results like this:

- **Sample size bounds what you can say.** Two shares closer than about
  2 s.e. are not ordered by the profile. Below about 1000 samples the
  summary says so. Lengthen the run or raise `--freq` (with `cpu-clock`,
  much above 4 kHz mostly adds overhead).
- **Frame-pointer stacks lose callers of leaf functions.** Measured on this
  repository's smoke binary (gcc -O2 `-fno-omit-frame-pointer`): with `fp`,
  every sample in the leaf `gf2_mul` was attributed to `main`, skipping its
  caller `sq_chain`. gcc emits no frame setup in leaves. `--call-graph dwarf`
  recovered `main;sq_chain;gf2_mul` for all of them. Use `dwarf` whenever
  caller attribution matters. Use `fp` (smaller `perf.data`) only for flat
  self-time tables.
- **Instruction share is not time share.** On the same binary, Callgrind
  gave `gf2_mul` 59.8% of instructions and `mix` 39.8%. Perf sampling gave
  `mix` 55% of time and `gf2_mul` 44% (±2.8 s.e.). `mix` is a chain of
  dependent 64-bit multiplies: fewer instructions, more cycles each. Use
  Callgrind to compare the work two builds do (deterministic, exact across
  runs). Use sampling to find where wall time goes. Do not substitute one
  for the other.
- **Inlining moves cost into callers.** A function missing from the table
  may have been inlined. `report.txt` and `perf annotate -i perf.data
  <symbol>` show the source lines.
- **Multi-threaded runs** sample every thread. The first frame of each
  folded stack is the thread's `comm`, so rayon workers appear as their own
  towers.
- **Kernel frames** (`[kernel.kallsyms]`) mean syscalls, page faults or
  scheduler time. A large share points at allocation or I/O, not
  arithmetic.

## 4. Top-down (hardware PMU only)

```sh
tools/profile.sh topdown --out .../<slug>-topdown [--level 2] -- <command>
```

This runs `perf stat -M TopdownL1` (or `--topdown` when that metric group
is absent) and gives the share of pipeline slots that are front-end bound,
bad speculation, back-end bound and retiring. It exits 3 without a PMU.
Multiplexed events are listed, and their values are scaled estimates. Rerun
with fewer events before relying on them. Top-down metric names and support
differ between Intel generations, AMD Zen, and Arm Neoverse. Compare only on
one host class.

## 5. GPU: nsys first, then ncu on what matters

**Is the GPU busy at all?**

```sh
tools/profile.sh nsys --out .../<slug>-nsys -- ./ecc2k130 --curve 131 --bench --steps 64 --launches 4 --verify 0
```

`summary.json` holds the per-kernel time share, the CUDA API time and memory
operations. `timeline.kernel_coverage` is the union of kernel intervals over
the span from the first kernel start to the last kernel end. Coverage well
below 1 means the GPU waits on the host (synchronization, launch gaps,
copies). No kernel tuning fixes that, so look at the API table
(`cudaDeviceSynchronize`, `cudaMemcpy`) first. Use `--cpu-sample` for host
call stacks and `--gpu-metrics` for SM activity over time; the latter needs
counter access.

**Why is the dominant kernel slow?**

```sh
tools/profile.sh ncu --out .../<slug>-ncu --kernel 'eccWalkKernel' --launch-skip 2 --launch-count 1 \
  -- ./ecc2k130 --curve 131 --bench --steps 64 --launches 4 --verify 0
```

- **Launch selection.** `--launch-skip` passes over warm-up and
  initialization launches. Profile a steady-state launch, and use the same
  launch in every build you compare.
- **Default sections.** SpeedOfLight, LaunchStats, Occupancy,
  ComputeWorkloadAnalysis, MemoryWorkloadAnalysis, SchedulerStats,
  WarpStateStats, InstructionStats and SourceCounters, plus a fixed metric
  list, so every summary has the same fields. `--set full` collects more
  sections at the price of more replays. `--source` imports CUDA source; it
  needs `-lineinfo`.
- **Outputs.** `kernels.json` (per launch), `sol.svg` (throughput,
  occupancy and pipe utilization on a fixed 0–100 axis), `stalls.svg`, and
  `details.txt` (ncu's own section text with its rule hints).

Read `kernels.json` like this. The causes in the right-hand column are
hypotheses to test, not conclusions:

| signal | likely meaning |
| --- | --- |
| SM throughput high, memory low; `math_pipe_throttle` top stall; one pipe (`alu`, `fma`, `fmaheavy`) near its peak | compute-bound on that pipe. GF(2^m) walks (LOP3, SHF, IMAD) live here: the gain is fewer instructions on the saturated pipe, or moving work to an idle one (ecc2k130/ROOFLINE.md) |
| `long_scoreboard` top stall | waiting on global or local memory. Check `local_memory_sectors`: non-zero means register spills or local arrays (compare `ptxas -v` spill bytes) |
| `wait` or `short_scoreboard` high | fixed-latency dependency chains or shared-memory/MIO latency. More independent work per warp (ILP) or more resident warps helps |
| achieved ≪ theoretical occupancy | tail effects, imbalance or a small grid. Theoretical occupancy capped by `occupancy_limits.registers` means registers per thread decide residency |
| `no_instruction` stalls | instruction-cache misses. Heavily unrolled or inlined kernels (the ecc2k130 packed walk) hit this. Compare against an out-of-line build |
| `not_selected` high | many eligible warps. Usually healthy; the scheduler is the limit |
| issue slots busy near 100% | the kernel issues as fast as it can. Only fewer instructions help |

ncu settings and failures:

- **Clocks.** `--clock-control base` (ncu's default, recorded in the
  manifest) locks clocks below boost, so ncu durations run longer than
  production. Ratios and stall mixes still compare across builds. Durations
  do not equal throughput. `--clock-control none` profiles at the clocks the
  GPU chooses.
- **Deterministic counters.** `instructions` (`smsp__inst_executed.sum`) is
  exact for a given launch and input. Compare it across builds like
  Callgrind `Ir`. Throughput percentages and durations vary between replays
  and need repeats.
- **Version.** Blackwell (compute capability 10.x and 12.x) needs Nsight
  Compute 2025.1 or newer. Ubuntu's unversioned `nsight-compute` is 2022.4
  and fails with "Failed to prepare kernel". The tool refuses that pairing
  (`--force-version` overrides it). It looks for
  `/opt/nvidia/nsight-compute/<version>/ncu` after `PATH`; set `NCU=` to pin
  one. ecc2k130's profiler image pins `nsight-compute-2025.3.1`.
- **`ERR_NVGPUCTRPERM`.** The driver withholds counters from non-admin
  users. On a host you administer, set `NVreg_RestrictProfilingToAdminUsers=0`.
  In a container, the runtime must grant profiling. Elsewhere, ask the
  provider. It is reported as `unsupported`, never worked around.

**Did the clock hold during a real run?**

```sh
tools/profile.sh smi --out .../<slug>-smi --gpu-index 0 --interval-ms 100 -- <unprofiled throughput command>
```

This samples SM clock, memory clock, power, temperature, utilization and the
clock-limit reason mask during the command. `--gpu-index` selects one physical
NVIDIA GPU index (default 0, independent of `CUDA_VISIBLE_DEVICES`). The manifest,
summary and graph identify that device; samples from other devices are excluded.
An empty sample stream is a profiler failure. `smi.svg` plots clock and power
over time, with limited samples marked red. A GPU rate measured while the
power or thermal limit was active is a measurement of the limit, not the
kernel. Report it beside every GPU throughput number.

## 6. Comparing builds with profiles

Each `--out` directory must be new, including for `probe`. Existing output is
rejected before the command runs, preserving earlier evidence. Use `report DIR`
to regenerate a report from an existing run.

1. **Same settings on both sides.** Profile both builds with identical
   flags, inputs, launch selection and clock control. Run the two arms
   back-to-back on the same host.
2. **Exact counters first.** Compare Callgrind `Ir` or ncu `instructions`:
   any difference is real work removed or added.
3. **Sampled shares second.** Compare them with their s.e.: a hotspot moving
   from 44% ± 2.8 to 41% ± 2.8 has not moved.
4. **Throughput from a measurement.** Settle the throughput question with a
   measurement (§0), not the profile. The profile's job is to say why the
   measured difference happened, and to rule out "it decided something
   differently".

## 7. Record it

- **Where.** Put the directory under the study, as
  `research/<study>/profiles/<slug>-<subcommand>/`.
- **Commit.** Commit `manifest.json`, `summary.md`, the summary JSON
  (`hotspots.json`, `kernels.json`, `summary.json`, `topdown.json`,
  `smi-summary.json`), the SVGs, `report.typ` and its PDF, and the small
  CSV/text outputs. Unsupported and failed runs are results too: commit
  their manifests.
- **Raw output.** `perf.data`, `*.ncu-rep`, `*.nsys-rep` and `callgrind.out`
  are large, and `perf.data` embeds host paths. Push them with `crypto store
  push` (AGENTS.md §13) when a reader needs them (an `.ncu-rep` opens in the
  Nsight Compute GUI). Otherwise leave them out. Never commit them loose.
- **PDF.** Build it from `report.typ` following the `research-visuals` skill
  ("Rendering"), for example `typst compile report.typ report.pdf`. Inspect
  every page, as that skill requires.
- **Re-rendering.** `tools/profile.sh report DIR` rebuilds `summary.md`, the
  SVGs and `report.typ` from the JSON without re-running anything.
- **`benchmarks/`.** A profile alone is not a performance result. It goes to
  `benchmarks/` only with the measurement it explains (AGENTS.md
  "Performance results also land in `benchmarks/`").
- **Claims.** A profile never moves a leaderboard row, and it never counts as
  an improvement under the success criterion. The measurement it led to
  does, if it holds.

## 8. Under Claude Code

With the host probed and the command, flags and output directory fixed, the
main session may dispatch `experiment-director` to write a task card of the
`tools/profile.sh` invocations, relay it verbatim to `executor-mechanical`,
and dispatch the director again to assess the run against the files on disk
(AGENTS.md "Model tiers"). Reading the profile (sections 3–6) stays with the
reasoning tier.

## Exit status

| code | meaning |
| --- | --- |
| 0 | profiled |
| 2 | usage error |
| 3 | unsupported on this host (manifest says why) |
| 4 | the profiled command failed (profile kept; outputs may be partial) |
| 6 | the profiler failed or profiled nothing (wrong `--kernel`, failed preparation) |

`tools/test_profile.sh` checks `tools/profile.sh`; CI runs it
(`.github/workflows/profile-tool.yml`). It runs the GPU parsers,
graphs and refusal paths against the test doubles in
`tools/testdata/profile/bin`: synthetic output in the ncu, nsys and
nvidia-smi formats with invented values, so no GPU is needed. It also runs
the CPU paths on the smoke workload when the host has perf, valgrind and a C
compiler. Run it after changing `tools/profile.sh`. The doubles fix the
formats as of Nsight 2025.x. When a newer Nsight renames a column or report,
the first real run shows nulls or `profiler_failed`. Then update the double
from that run's actual output and the parser together.
