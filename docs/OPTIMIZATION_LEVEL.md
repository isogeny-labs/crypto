# Optimization level: how close an implementation runs to its machine

`tools/optlevel/` measures how well optimized one implementation of a kernel
is on one machine. It compares the rate at which the kernel performs its own
counted operations with a roofline ceiling measured on the same host, and
explains the gap with hardware counters where the host has them.

Record schema: `optlevel.record/v1`
([`tools/optlevel/schema/optlevel.record.v1.schema.json`](../tools/optlevel/schema/optlevel.record.v1.schema.json)).
Skill: [`.claude/skills/optlevel/SKILL.md`](../.claude/skills/optlevel/SKILL.md).
Kernel registry across the three repositories:
[`tools/optlevel/registry.json`](../tools/optlevel/registry.json).
Validation on a host without a PMU or GPU:
[`research/optlevel_20261117/`](../research/optlevel_20261117/README.md).
The port from Python to Rust, with the kernel A/B and the equivalence runs:
[`research/optlevel_rs_20261117/`](../research/optlevel_rs_20261117/README.md).
Tests: `cargo test --release --manifest-path tools/optlevel/Cargo.toml`.

## 0. What it is not

- **Not a speed claim.** An attainment says how much of the machine a kernel
  uses for the operations it counts. It does not say the kernel is fast, and
  two attainments are not a speedup.
- **Not a comparison between algorithms.** A perfectly tuned bad algorithm
  scores high. Rho against index calculus is decided in counted units against
  the matched rho with the §5a band (AGENTS.md §5a, §12), never here.
- **Not a benchmark record, a `benchmarks/README.md` row or a leaderboard
  row.** A record has no replay certificate, no validator report and no matched
  rho reference, so it cannot pass the validation gate and must not be placed
  where a gated row belongs. Every record carries this statement in
  `not_a_speed_claim`.
- **Not portable across hosts.** The ceiling is this host's. An attainment of
  0.8 on a laptop and 0.4 on a server do not rank the code.

## 1. Terms

Every quantity is tagged *measured* (M), *derived* (D) from measured values, or
*declared* (X) by the kernel or the operator.

| term | unit | kind | definition |
|:--|:--|:--|:--|
| useful operation | 1 | X | one instance of the kernel's operation class, counted by the kernel itself (section 3) |
| operation class | name | X | which calibrated ceiling the operations are compared with (section 2) |
| achieved rate `R(k)` | ops/s | M | counted operations ÷ seconds of the timed region, at `k` threads; median over runs, with min and max |
| peak compute rate `P_c(k)` | ops/s | M | most operations of class `c` per second that `k` cores of this host sustained in the calibration kernels |
| bandwidth `B(k)` | bytes/s | M | most bytes per second `k` cores moved through arrays larger than the last-level cache |
| memory traffic `T` | bytes/op | M or X | bytes the kernel moves between DRAM and the cache hierarchy per operation |
| arithmetic intensity `I` | ops/byte | D | `1 / T` |
| compute roof | ops/s | D | `P_c(k)` |
| memory roof | ops/s | D | `I · B(k) = B(k) / T` |
| roofline bound `U(k)` | ops/s | D | `min(P_c(k), I · B(k))`; the roof that gives the minimum is the *binding* roof |
| attainment `A(k)` | ratio | D | `R(k) / U(k)` |
| raw parallel efficiency | ratio | D | `R(k) / (k · R(1))` |
| roofline-normalised parallel efficiency `E*` | ratio | D | `[R(k)/R(1)] / [U(k)/U(1)] = A(k)/A(1)` |
| machine scaling | ratio | D | `U(k) / (k · U(1))`: how far the machine itself is from linear at this intensity |
| noise floor | ratio | M | section 7 |
| score | ratio | D | section 6 |

`A_single = A(1)` and `A_full = A(k)` at the largest measured `k` are the two
core numbers. Both are reported with an interval: `low = R_min / U`,
`high = R_max / U_low`, where `U_low` is the bound built from the calibration
medians rather than its maxima.

## 2. Machine ceiling (calibration)

`optlevel calibrate` writes `optlevel.calibration/v1`, keyed by a host
fingerprint (CPU model, flag set, logical CPU count, cache sizes, memory,
hypervisor bit). The tool is a Rust crate, `tools/optlevel/`
(`tools/optlevel/run.sh` builds it on first use and runs it); nothing in it is
Python. The ceilings are measured by the kernels in
`tools/optlevel/src/kernels.rs`, which are part of the `optlevel` binary and
run as `optlevel kernel NAME --isa I`. The calibration records the hash of the
kernel source and of the binary.

**Builds.** One binary carries every variant and picks at run time, so a
binary built on one x86-64 machine calibrates another correctly: `avx512`
(512-bit loops, AVX-512 F/DQ/BW/VL/VPOPCNTDQ), `avx2` (256-bit loops) and
`scalar` (the same scalar instruction set, with vectorization blocked by an
empty assembly barrier on each element). A host without AVX2 gets `base` and
`scalar` from the target's baseline. A class's ceiling is the best of the
builds. The scalar and best-vector rates are both kept, so "the SIMD ceiling
is `h` times the scalar one" is a measured statement.

`calibrate --c-kernels` adds three C builds of the reference kernels the Rust
ones were ported from (`kernels/optk.c`, compiled with the system compiler as
`c-native`, `c-w512` and `c-scalar`) to the best-of. Use it once on a CPU
family nobody has calibrated before: a ceiling can only rise by it. On the
validation host the Rust kernels are above the C ones in seven of the nine
classes and level with them, inside the run-to-run spread, in `fma_f64` and
`modmuladd_p62` (`research/optlevel_rs_20261117/README.md`). The C kernels read the clock
after every pass over their 8–16 KiB arrays; the Rust kernels read it about
every 50 µs, which alone raises the in-cache SIMD ceilings by 1.4 to 1.9
times on that host (`modmuladd_p31` gains 1.07 from the timer and 1.7 from
better generated code). Calibrations taken before 2026-11-17 used the C
kernels, so their ceilings for those classes are low by 1.4 to 1.8 times and
attainments scored against them are high by the same factor.

**CPUs.** The scale points and the "all cores" ceiling use the CPUs this
process can really run on at once: the affinity mask, cut to the cgroup CPU
quota (cgroup v2 `cpu.max`, cgroup v1 `cpu.cfs_quota_us / cpu.cfs_period_us`,
at the process's cgroup and its ancestors; the tightest one, rounded down, at
least 1). `host` records all of them: `logical_cpus` (the machine),
`affinity_cpus`, `cpu_quota_cores` with `cpu_quota_source`, and `usable_cpus`
with its basis. A container that hides its allotment from both interfaces can
declare it with `OPTLEVEL_CPUS=N`. A `measure --scale` point above
`usable_cpus` is flagged `SCALE_EXCEEDS_USABLE_CPUS`.

**Operation classes** (each kernel runs over arrays of 8–16 KiB, inside L1):

| class | one operation |
|:--|:--|
| `muladd_u32`, `muladd_u64` | `a ← a·m + c` on 32- or 64-bit words (wrapping) |
| `mulwide_u64` | `a ← hi64(a·m) + c`: one 64×64→128 multiply |
| `modmuladd_p31` | `a ← a + m·b mod p`, `p < 2³¹`, Montgomery with 32×32→64 multiplies (the dense row operation) |
| `modmuladd_p62` | the same for `p < 2⁶²` with 64×64→128 multiplies (the `Fp64` rho arithmetic) |
| `xor_u64` | `a ← a ⊕ b` on 64-bit words (one word of an F₂ row addition) |
| `popcnt_u64` | `popcount(a ∧ b)` on 64-bit words |
| `fma_f64` | `a ← a·s + b` in binary64; the ceiling is the better of an array loop and 16 register-resident vector accumulators, because a blocked matrix product approaches the second |
| `ptr_hop` | one dependent pointer dereference (in-L1 chase) |

**Ceiling rule.** `P_c(k)` is the maximum over repeats of the *summed* rate of
`k` concurrent copies, copy `i` pinned to its own CPU. The median is kept as
the low end. `k` runs over 1, the powers of two below the CPU count, and the
CPU count. A `k` between two measured points is interpolated linearly and
flagged; a `k` above the largest is clamped to it and flagged
`CEILING_EXTRAPOLATED`.

**Bandwidth.** Three streaming kernels over arrays of
`max(64 MiB, 1.5 × LLC)` each (capped at 40% of available memory, flagged
`TRIAD_ARRAY_NOT_ABOVE_LLC` if the cap brings an array under the LLC): the
STREAM triad, a 64-bit xor stream and a popcount stream. Bytes are the bytes
the loop really moves: loads, stores and write-allocate reads, so the triad
counts 32 bytes per element (STREAM reports 24), the xor 24, the popcount 16.
`B(k)` is the best of the three.

**Latency.** Nanoseconds per dependent load over a random single cycle of
64-byte nodes as large as one triad array. It is used only for diagnosis.

**Effective clock.** A dependent chain of one-cycle additions gives operations
per second = core clock in Hz, recorded with its spread (`CLOCK_UNSTABLE` above
3%).

A ceiling is only as good as its microbenchmark. That is why the tool never
clamps: an attainment above `1 + 0.02 + noise floor` raises
`CEILING_EXCEEDED` and the record has no grade.

**GPU.** `optlevel gpu calibrate` builds `kernels/gpu_calib.cu` with `nvcc`
at first use (the source is inside the binary) and measures, with every SM
occupied, the device rate of `muladd_u32`, `muladd_u64`, `xor_u64`,
`popcnt_u64`, `fma_f64` and a device-memory triad. The triad gives the device
bandwidth and also a `triad` operation class for memory-bound kernels: its
compute roof is the `fma_f64` ceiling (one multiply-add per element) and its
memory roof is bandwidth over the 24 bytes per element the kernel declares.
`gpu measure` takes a kernel's declared bytes from its `optlevel.ops/v1`
lines, as `measure` does on CPU, and `--bytes-per-op` overrides them; a class
named `triad` scored against an older calibration without that class uses the
`fma_f64` ceiling and is flagged `CEILING_CLASS_ALIAS_FMA_F64`.
`kernels/gpu_calib.cu` takes one optional argument, a class name or `triad`,
to run that class alone as a control. `optlevel gpu probe` lists the devices
and tools, names the helper programs `tools/profile.sh` needs and does not
find (`gawk`, `jq`), and runs the control kernel once under `ncu`: when the
driver answers `ERR_NVGPUCTRPERM` it reports "counters withheld by the
driver" and the tier `T-gpu-timing`. The
calibration has one scale point, `"1"`, meaning one whole device. These
ceilings are cross-checked, not replaced, by `ncu`'s
`sm__throughput.avg.pct_of_peak_sustained_elapsed` (section 5). Per-pipe
ceilings finer than a class (ALU, FMA and carry-less pipes on one SM) already
exist for the ECC2K-130 walk: `ecc2k130/roofline.py`,
`ecc2k130/benchmarks/roofline/pipes.cu`, and the sibling repository's `gpueff`
(section 9). Use those for that kernel.

## 3. Operation-count protocol

The kernel counts; the tool never infers an operation count from time.

1. **`optlevel.ops/v1` line on stdout** (preferred). One JSON object per line;
   several lines from one process are summed:

   ```json
   {"schema": "optlevel.ops/v1", "ops": 123456789, "op_class": "modmuladd_p62",
    "kernel_seconds": 1.0023, "bytes_declared": 2962962936, "working_set_bytes": 16384}
   ```

   `ops` is required. `kernel_seconds` is the wall time of the timed region;
   without it the whole process wall time is used and the record is flagged
   `WALL_INCLUDES_STARTUP`. `bytes_declared` is the bytes the counted
   operations move (loads + stores + write-allocate reads);
   `working_set_bytes` is what the kernel touches repeatedly.
2. **`--ops-regex RE`**: one capture group; every match in stdout and stderr is
   summed. For binaries that already print a counter (`walk_steps=…`).
3. **`--ops N`**: a constant, for a fixed workload whose count is known from
   its inputs.

The record names which source was used (`kernel.ops_source`). Choose the
operation so that the class's ceiling is a fair comparison: a walk step that
costs one inversion share, five multiplications and a table lookup is not one
`modmuladd_p62`; report it as the field multiplications it performs (most
engines here already count those) or score it against a ceiling of its own.
When the counted unit is coarser than a class, say so in the registry entry
and treat the attainment as a lower bound.

## 4. Arithmetic intensity

Sources of `T`, best first; the record says which one it used
(`intensity.source`) and keeps the declared and measured values side by side
with their ratio:

| source | tier | how |
|:--|:--|:--|
| `imc` | T-pmu | uncore memory-controller CAS counts × 64 bytes, system-wide (specified; not implemented in v1, section 10) |
| `llc-miss` | T-pmu | user-space `cache-misses` × 64 bytes ÷ operations. A lower bound: the last-level prefetcher can hide traffic |
| `declared` | any | `bytes_declared ÷ ops` or `--bytes-per-op`. If `working_set_bytes` is at or below the LLC size, the bytes stay in cache and `T = 0` (compute roof binds). Between L2 and the LLC the record is flagged `WORKING_SET_IN_LLC`: cache-level bandwidths are not calibrated, so the compute roof is an upper bound on the true roof |
| `none` | any | flagged `INTENSITY_UNKNOWN`. The bound is the compute roof (`binding: "compute (traffic unknown)"`). Since the true bound can only be lower, the attainment is a **lower bound** |

GPU: `ncu`'s `dram__throughput` and `gpu__compute_memory_throughput`
percentages are attached as diagnosis; `T` itself is declared.

## 5. Diagnosis components

None of these enters the score. They explain it.

| component | tier | definition |
|:--|:--|:--|
| `ipc`, `cycles_per_op`, `instructions_per_op`, `branch_miss_rate` | T-pmu | user-space `perf stat` counters of the single-thread runs, per counted operation |
| `llc_misses_per_op`, `l1d_misses_per_op`, `dtlb_misses_per_op` | T-pmu | second event group, on the alternate runs, so no group is multiplexed |
| `topdown` | T-pmu | level 1 (frontend bound, bad speculation, backend bound, retiring) and level 2 memory against core bound, ingested from `tools/profile.sh topdown` (`score --topdown topdown.json`) |
| `vector_share_static` | any | share of instructions in named symbols that use packed-vector registers, from `objdump` (`optlevel vector-share`). Static: instructions in the text, not instructions executed |
| `vector_share_sampled` | T-pmu | share of `perf record` samples on packed-vector instructions (specified; not implemented in v1) |
| `scalar_ceiling_ratio`, `simd_headroom` | any | `R(1) ÷` the class's scalar ceiling, and the class's vector ÷ scalar ceiling |
| `latency_attainment` | any | `R(1) × latency`: about 1 means one dependent miss outstanding at a time; reported only when the memory roof binds |
| `startup_share` | any | `1 − kernel_seconds / wall` |
| scaling curve | any | `R(k)` and raw efficiency at every measured `k` (`parallel.curve`) |
| `gpu.nvidia_smi` | T-gpu-* | utilisation, memory utilisation, power, temperature and throttle-reason share sampled every 200 ms |
| `gpu.host_cpu_utilisation` | T-gpu-* | host CPU-seconds per second of the run |
| `gpu.ncu` | T-gpu-ncu | SM and memory throughput (% of peak), achieved and theoretical occupancy, issue-active share, warp stall reasons, registers per thread, from `tools/profile.sh ncu` (`score --ncu kernels.json`) |
| `gpu.host_idle_share` | T-gpu-ncu | `1 − timeline.kernel_coverage` from `tools/profile.sh nsys` (`score --nsys summary.json`) |

**Limiting factor.** `grade.limiting_factor` is chosen by this rule, in order,
and `grade.limiting_basis` names the evidence:

1. `A_single ≥ 0.70` and (`E*` unmeasured or `≥ 0.70`): at the memory roof,
   "only higher arithmetic intensity or a different algorithm helps";
   otherwise "near the roofline; what remains is the algorithmic axis".
2. `E* < A_single`: parallel scaling.
3. Memory roof binds: memory latency bound if `0.5 ≤ latency_attainment ≤ 2`;
   else TLB (T-pmu, more than 0.05 dTLB misses per operation); else memory
   bound below the bandwidth roof.
4. Compute roof binds, T-pmu with a top-down profile: the largest non-retiring
   level-1 share, split by level 2; if retiring dominates, instructions per
   operation.
5. Compute roof binds, no top-down: "not vectorized" when the static vector
   share is below 0.2 and the class's SIMD headroom exceeds 1.5; "not
   vectorized or a serial dependency (inferred from timing)" when the rate is
   at most 1.15 × the scalar ceiling; otherwise "core bound below the compute
   roof (timing tier cannot split these)".

## 6. Summary grade

```text
score = sqrt(A_single · A_full)          when both points were measured   (scope "single+parallel")
score = A_single                         single-thread only                (scope "single-thread only")
letter: A ≥ 0.70,  B ≥ 0.40,  C ≥ 0.20,  D ≥ 0.08,  E below
```

`sqrt(A_single · A_full) = A_single · sqrt(E*)`. This replaces the first
proposal, the geometric mean of attainment and raw parallel efficiency, for a
measured reason: raw efficiency charges the kernel for the machine's own
saturation. On a host whose bandwidth does not grow with the core count, a
streaming kernel at the bandwidth roof on one core and on all cores would have
raw efficiency well under 1 and yet nothing to fix; `E*` is 1 there. Raw
efficiency and machine scaling are still reported.

**The full point needs free cores.** When everything else on the host used
more than 0.25 CPU-seconds per second during the scaled runs, the record is
flagged `FULL_POINT_CONTENDED`: `A_full` and the parallel block are still
reported, but the score falls back to `A_single` (scope "single-thread only")
and the limiting factor ignores scaling. This rule was added on 2026-11-17
after the first control run on a shared host, where the same kernel's `A_full`
read 1.23 and then 0.66 (research/optlevel_20261117).

`letter_range` gives the letters reachable inside the noise floor (`"B/C"`).
Scores with different scopes are never compared. A record with
`CEILING_EXCEEDED`, `TOO_SHORT` or `OPS_MISSING` has no score; the reasons are
in `grade.invalid`.

**Algorithmic efficiency is a separate axis.** `algorithmic` holds the counted
operations, the best known count for the task and their ratio when the
operator supplies them (`score --best-known-ops N --best-known-source …`). It
never enters the score.

## 7. Noise floor

`measure --repeat R` runs the single-thread point `2R` times in interleaved
slots A and B of the same command, with the scaled points between them.

```text
aa_ratio_deviation = | median(R_A) / median(R_B) − 1 |
rel_range          = (max − min) / median over the 2R runs
noise floor        = max of the two
```

Every graded record carries it; `R < 5` is flagged
`NOISE_FLOOR_UNDER_5_PAIRS`. A difference between two attainments inside the
larger of their noise floors is not a difference.

## 8. Confidence tiers

| tier | host | what the record may contain |
|:--|:--|:--|
| `T-pmu` | hardware PMU readable (`perf stat -e cycles:u` counts) | everything in sections 4 and 5 |
| `T-timing` | no PMU | wall time and operation counters only: rates, attainment against declared or unknown traffic, scaling, static vector share. **IPC, miss rates, top-down and measured traffic are null**, with the reason under `diagnosis.unavailable` |
| `T-gpu-ncu` | GPU with Nsight Compute counters | rates, `nvidia-smi` trace, `ncu` and `nsys` components |
| `T-gpu-timing` | GPU without counter access (no `ncu`, or `ERR_NVGPUCTRPERM`: counters withheld by the driver) | rates and `nvidia-smi` trace only |

`score::enforce_tier` (`src/score.rs`) rejects a record whose components exceed its tier, and
`optlevel validate` runs it. The PMU test is the one `tools/perf_ipc.sh`
uses; the tool never derives cycles from wall time and a nominal clock.

## 9. Relation to the existing tools

| tool | what it gives | relation |
|:--|:--|:--|
| `tools/perf_ipc.sh` (`perf-ipc`) | cycles, instructions and IPC per unit of work, under `isolated_bench` | The speed measure is cycles per unit there. optlevel uses the same PMU probe and event names, counts its own two event groups so a record is self-contained, and adds the ceiling. For a number you will report, measure cycles per unit with `perf_ipc.sh --isolate`; optlevel does not isolate (section 10) |
| `tools/profile.sh` (`profile`) | hotspots, top-down, `ncu`, `nsys`, `nvidia-smi` traces | optlevel ingests `topdown.json`, the `ncu` `kernels.json` and the `nsys` `summary.json`; it does not re-run those profilers. A profile explains a cost; an optlevel record says how large the gap to the machine is and names the first place to look |
| `hwcost` (`docs/hwcost/README.md`) | a *predictive* roofline: time and dollars from operation counts and a fitted profile | Opposite direction. hwcost predicts a time from counts; optlevel divides a measured rate by a ceiling. Their calibrations overlap (per-operation cycles, bandwidth per cache level, scaling). A `hwcost.hardware.v1` profile could supply optlevel's ceilings; that adapter is not written |
| `ecc2k130/roofline.py`, `ROOFLINE.md` | per-pipe SASS demand of one CUDA kernel against measured lane rates | Finer than an operation class. For the ECC2K-130 walk that analysis is the authority; an optlevel GPU record of it is coarser |
| `cryptanalysis/gpueff` | roofline efficiency and a 0–100 composite from DCGM and Prometheus metrics, N roofs from a demand profile | Same idea, fleet-side, for long-running GPU work. optlevel takes its N-roof reading (the lowest roof binds; the next roof is `next_roof_ratio`). One difference is deliberate: gpueff's composite weights SM activity, occupancy and clock beside the roofline efficiency that already contains them as factors; optlevel keeps such components out of the score |
| `tools/benchrec.py` | `bench.record/v1`, raw data in S3, the progress report | An optlevel record is not a benchmark record. `optlevel benchrec R.json` prints its values in benchrec's vocabulary (`ops_per_s`, `ratio`, `ipc`) with `--reference-kind none`, for a study that wants them in a record it opens itself. Attainment is `better: neutral`: it is not a series to maximise against a rho reference |
| AGENTS.md §5a | prices unpriced native work at 0, 0.1 and 1 GAE per operation | §5a prices *work* across methods. An attainment prices nothing. It can tell you that a counter §5a prices is executed at 5% of the machine, which is a reason to tune it, not a reason to reprice it |
| AGENTS.md §10 | isolation, A/A, interleaving, identical outputs | Section 7 follows it. optlevel records contention instead of refusing to run; a `CONTENDED` record is exploratory |

## 10. Failure modes and what the tool does about each

| failure mode | detection or record | effect |
|:--|:--|:--|
| frequency scaling, turbo | governor, `no_turbo` and `boost` recorded when readable; effective clock measured at calibration with its spread (`CLOCK_UNSTABLE`) | calibrate and measure in the same session; on bare metal fix the governor |
| AVX-512 or all-core frequency licence | all-core ceilings are measured, not `k ×` the single-core one; `machine_scaling` reports the drop | none needed |
| SMT | `smt/active` and sibling list recorded; `SMT_ACTIVE` flag | pinned copies may share a core; a `k` above the physical core count is a different machine |
| noisy neighbours | per run: load average, CPU pressure (PSI), CPU-seconds used by everything else (`other_cpu_cores`), steal time; `CONTENDED` above 0.25 cores; `FULL_POINT_CONTENDED` when the scaled runs were; `CALIBRATION_CONTENDED` when the ceiling was taken that way | exploratory only; the full point is not graded. The tool does not take the `isolated_bench` lock or evict other threads |
| NUMA | node count recorded; `NUMA_MULTI_NODE` flag | pin with `numactl` and calibrate under the same binding |
| thermal | `thermal_throttle` counts before and after; `THERMAL_THROTTLE`; GPU: throttle-reason share, `GPU_THROTTLED` | rerun |
| virtual machine | `hypervisor` CPU flag; `VIRTUALIZED` | caches and bandwidth are shared with unseen tenants; the reported LLC may be the host's |
| container allotted fewer CPUs than it can see | cgroup v2 `cpu.max` and v1 quota read beside the affinity mask; `host.usable_cpus` with its basis; `SCALE_EXCEEDS_USABLE_CPUS` | scale points and "all cores" follow the allotment; `OPTLEVEL_CPUS=N` when no interface shows it |
| container hiding the PMU | the `perf_ipc.sh` probe; tier falls to `T-timing` with the reason | PMU components are null, never estimated |
| counter multiplexing | `perf` running share under 100%: `MULTIPLEXED`, events listed | drop the multiplexed components |
| kernel too short | timed region under 0.5 s: `TOO_SHORT` | no grade |
| start-up inside the timed region | no `kernel_seconds`: `WALL_INCLUDES_STARTUP`; otherwise `startup_share` | attainment is a lower bound |
| operation counter counts the wrong thing | `CEILING_EXCEEDED` when it overcounts; `intensity.declared_over_measured` in T-pmu; `OPS_VARY` when counts differ between repeats of a fixed workload | an undercount cannot be detected: it lowers the attainment. Check the counter against a second one (instructions per op in T-pmu, Callgrind `Ir` per op anywhere) |
| ceiling is not a ceiling | `CEILING_EXCEEDED`; ceilings are best-of-builds and best-of-kernels | no grade; fix the calibration kernel |
| calibration from another host | fingerprint compared: `CALIBRATION_HOST_MISMATCH` | recalibrate |
| working set between L2 and LLC | `WORKING_SET_IN_LLC` | the compute roof overstates the bound |
| traffic unknown | `INTENSITY_UNKNOWN` | attainment is a lower bound |
| prefetch hides LLC misses | documented on `llc-miss`; declared and measured kept side by side | prefer IMC counts when implemented |

Not implemented in v1, and therefore not validated anywhere: IMC traffic,
sampled vector share, cache-level bandwidths, a `hwcost` profile adapter,
`isolated_bench` integration. The PMU path is implemented and tested on
synthetic `perf` output but has not run on a host that has a PMU;
`research/optlevel_20261117/README.md` gives the commands that validate it.
The GPU path ran once, in its first (Python) version, on a rented GPU whose
driver withheld the counters (`research/hky_pod_runs_20261117/`); the Rust
port of it is tested against stub `nvidia-smi`, `nvcc` and `ncu` programs
and against those committed files, and has not itself run on a GPU.

## 11. Reading a record

```text
closure m4 k2  [T-timing]  class fma_f64
  attainment single 0.31 [0.28, 0.34]  of 4.2e10 ops/s (compute roof, 1 thread(s))
  attainment full   0.12 [0.10, 0.13]  of 1.6e11 ops/s (compute roof, 4 thread(s))
  parallel   raw 0.38, roofline-normalised 0.39 at 4 threads
  grade      C/D (score 0.19, scope single+parallel)
  limiting   parallel scaling: roofline-normalised efficiency 0.39 at 4 threads …
```

Read it in this order: tier (what could be measured), flags (whether to trust
it), binding roof (which resource bounds this kernel on this host), the two
attainments with their intervals, `E*`, then the limiting factor and its
basis. Then ask the separate question: is this the right algorithm?
