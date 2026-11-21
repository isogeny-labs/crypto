# Index-calculus GPU code: speed and memory audit (2026-11-12)

Scope: `gpu/explicit-relations`, `gpu/macaulay`, `gpu/rootfind`,
`gpu/semaev`, `gpu/spmv`, `gpu/fes` (CUDA, plus the Metal FES worker).
Baseline revision `e9d5dbdccd` (origin/main); the four code commits are
listed in [Changes](#changes). Writer: Claude (cloud session, no GPU).

**Everything here is untested on a GPU.** The host has no NVIDIA device and
no `nvcc`. Every number below is either a static compiler report
(`ptxas -v`), a host-oracle result, or an estimate. Estimates carry
`[pcc:unverified]`; none of them is a measured speedup. The lab cards in
[GPU validation cards](#gpu-validation-cards) are what would turn them into
measurements.

## How the numbers were produced

- Toolchain: Ubuntu clang 18.1.3 (CUDA front end, device-only PTX, `-O3
  -std=c++17`, PTX ISA 8.3) and NVIDIA `ptxas` 12.9.86 (`-O3 -v`), with the
  CUDA 12.9 runtime/CCCL headers. `ptxas`, `libdevice` and the headers were
  taken from NVIDIA's pip wheels (`nvidia-cuda-nvcc-cu12==12.9.86`,
  `nvidia-cuda-runtime-cu12==12.9.79`, `nvidia-cuda-cccl-cu12==12.9.27`),
  the same fetch as `cryptanalysis:scripts/build_cuda_kernel.sh --fetch`
  (56 MB assembled; `libnvvm.so` dropped). A stub `cuda.h` carries only
  `CUDA_VERSION 12090` and the `cuuint32_t`/`cuuint64_t` typedefs clang's
  wrapper needs.
- Targets: `sm_70`, `sm_80`, `sm_89`, `sm_90`, one PTX per arch, flags as in
  each Makefile (`-O3 -std=c++17` plus the variant `-D`s). Generated headers
  used the Makefile defaults: macaulay `P=2083 ROWS=226 COLS=286`, semaev
  `L=5`, fes `N=18 M=20 SEED=1`; rootfind `RF_BLOCK=128 RF_MIN_BLOCKS=0`.
  explicit-relations was built in 9 `service.cpp` variants (`ER_CUDA`,
  `+QUERY_CACHED`, `+COMPACT_ROWS`, dense+CUB, persistent, filter, affine,
  full-cache) plus `benchmark.cu`.
- **Caveat: clang is not nvcc.** NVIDIA's front end (cicc/NVVM) generates
  different PTX, so register counts and stack frames from an `nvcc` build
  will differ. The before/after deltas are like for like (same toolchain);
  the absolute values are indicative only. The lab cards re-read them with
  `nvcc -Xptxas -v`.
- Host-pass syntax: every changed `.cu` file also passes
  `clang++ -x cuda --cuda-host-only -fsyntax-only -Wall -Wextra`. Nothing
  was linked (no `libcudart` here).

Cell format in the tables: `registers / spill-store:spill-load bytes /
stack-frame bytes / static shared bytes`, per thread except shared memory.
Dynamic shared memory is not in `ptxas -v`; it is listed per change.

## Baseline (`e9d5dbdccd`)

### macaulay, rootfind, semaev, spmv, fes

| target | kernel | sm_70 | sm_80 | sm_89 | sm_90 |
|---|---|---|---|---|---|
| spmv | `spmv_kernel` | 28 / 0:0 / 0 / 0 | 28 / 0:0 / 0 / 0 | 28 / 0:0 / 0 / 0 | 30 / 0:0 / 0 / 0 |
| fes | `fes_kernel` | 30 / 0:0 / 832 / 0 | 28 / 0:0 / 832 / 0 | 28 / 0:0 / 832 / 0 | 29 / 0:0 / 832 / 0 |
| semaev (default, `GF2N_CLMAD=1` on sm_80+) | `sweep_kernel_flat` | 96 / 0:0 / 312 / 0 | ptxas fails | ptxas fails | ptxas fails |
| semaev (default) | `sweep_kernel` | 96 / 0:0 / 312 / 0 | ptxas fails | ptxas fails | ptxas fails |
| semaev `-DGF2N_CLMAD=0` | `sweep_kernel_flat` | 96 / 0:0 / 312 / 0 | 63 / 0:0 / 312 / 0 | 63 / 0:0 / 312 / 0 | 64 / 0:0 / 312 / 0 |
| semaev `-DGF2N_CLMAD=0` | `sweep_kernel` | 96 / 0:0 / 312 / 0 | 79 / 0:0 / 312 / 0 | 72 / 0:0 / 312 / 0 | 80 / 0:0 / 312 / 0 |
| macaulay | `from_mont_kernel` | 18 / 0:0 / 0 / 0 | 18 / 0:0 / 0 / 0 | 18 / 0:0 / 0 / 0 | 18 / 0:0 / 0 / 0 |
| macaulay | `to_mont_kernel` | 18 / 0:0 / 0 / 0 | 18 / 0:0 / 0 / 0 | 18 / 0:0 / 0 / 0 | 18 / 0:0 / 0 / 0 |
| macaulay | `rref_batch_kernel` | 52 / 0:0 / 0 / 0 | 32 / 0:0 / 0 / 0 | 34 / 0:0 / 0 / 0 | 38 / 0:0 / 0 / 0 |
| rootfind | `k_decompose_global_hash<P256,C64>` | 128 / 0:0 / 4368 / 0 | 128 / 0:0 / 4368 / 0 | 128 / 0:0 / 4368 / 0 | 128 / 0:0 / 4368 / 0 |
| rootfind | `k_decompose_shared_hash<P256,C64>` | 144 / 0:0 / 4368 / 49152 | 136 / 0:0 / 4368 / 49152 | 144 / 0:0 / 4368 / 49152 | 128 / 0:0 / 4368 / 49152 |
| rootfind | `k_decompose_sorted<P256,C64>` | 128 / 0:0 / 4368 / 0 | 126 / 0:0 / 4368 / 0 | 126 / 0:0 / 4368 / 0 | 128 / 0:0 / 4368 / 0 |
| rootfind | `k_decompose_global_hash<P256,C32>` | 128 / 0:0 / 2320 / 0 | 128 / 0:0 / 2320 / 0 | 128 / 0:0 / 2320 / 0 | 128 / 0:0 / 2320 / 0 |
| rootfind | `k_decompose_shared_hash<P256,C32>` | 144 / 0:0 / 2320 / 49152 | 136 / 0:0 / 2320 / 49152 | 144 / 0:0 / 2320 / 49152 | 128 / 0:0 / 2320 / 49152 |
| rootfind | `k_decompose_sorted<P256,C32>` | 128 / 0:0 / 2320 / 0 | 126 / 0:0 / 2320 / 0 | 128 / 0:0 / 2320 / 0 | 126 / 0:0 / 2320 / 0 |
| rootfind | `k_decompose_global_hash<P256,C16>` | 128 / 0:0 / 1296 / 0 | 128 / 0:0 / 1296 / 0 | 128 / 0:0 / 1296 / 0 | 128 / 0:0 / 1296 / 0 |
| rootfind | `k_decompose_shared_hash<P256,C16>` | 144 / 0:0 / 1296 / 49152 | 136 / 0:0 / 1296 / 49152 | 142 / 0:0 / 1296 / 49152 | 128 / 0:0 / 1296 / 49152 |
| rootfind | `k_decompose_sorted<P256,C16>` | 128 / 0:0 / 1296 / 0 | 126 / 0:0 / 1296 / 0 | 128 / 0:0 / 1296 / 0 | 126 / 0:0 / 1296 / 0 |
| rootfind | `k_decompose_global_hash<Fp64,C64>` | 204 / 0:0 / 624 / 0 | 204 / 0:0 / 624 / 0 | 204 / 0:0 / 624 / 0 | 206 / 0:0 / 624 / 0 |
| rootfind | `k_decompose_shared_hash<Fp64,C64>` | 214 / 0:0 / 624 / 49152 | 204 / 0:0 / 624 / 49152 | 214 / 0:0 / 624 / 49152 | 206 / 0:0 / 624 / 49152 |
| rootfind | `k_decompose_sorted<Fp64,C64>` | 204 / 0:0 / 624 / 0 | 204 / 0:0 / 624 / 0 | 204 / 0:0 / 624 / 0 | 206 / 0:0 / 624 / 0 |
| rootfind | `k_decompose_global_hash<Fp64,C32>` | 128 / 0:0 / 368 / 0 | 127 / 0:0 / 368 / 0 | 125 / 0:0 / 368 / 0 | 128 / 4:0 / 376 / 0 |
| rootfind | `k_decompose_shared_hash<Fp64,C32>` | 138 / 0:0 / 368 / 49152 | 128 / 0:0 / 368 / 49152 | 140 / 0:0 / 368 / 49152 | 128 / 4:0 / 376 / 49152 |
| rootfind | `k_decompose_sorted<Fp64,C32>` | 128 / 0:0 / 368 / 0 | 127 / 0:0 / 368 / 0 | 125 / 0:0 / 368 / 0 | 128 / 4:0 / 376 / 0 |
| rootfind | `k_decompose_global_hash<Fp64,C16>` | 90 / 0:0 / 240 / 0 | 90 / 0:0 / 240 / 0 | 80 / 0:0 / 240 / 0 | 91 / 0:0 / 240 / 0 |
| rootfind | `k_decompose_shared_hash<Fp64,C16>` | 102 / 0:0 / 240 / 49152 | 98 / 0:0 / 240 / 49152 | 100 / 0:0 / 240 / 49152 | 97 / 0:0 / 240 / 49152 |
| rootfind | `k_decompose_sorted<Fp64,C16>` | 90 / 0:0 / 240 / 0 | 90 / 0:0 / 240 / 0 | 80 / 0:0 / 240 / 0 | 91 / 0:0 / 240 / 0 |

Generated from the raw `ptxas -v` logs (`g-ptxas.sh` + `tables.py`, kept
with the session scratch, not committed).

### explicit-relations (project kernels; CUB/Thrust kernels omitted)

Cell: registers (range over the variants that contain the kernel) / max
spill-store bytes / stack bytes / static shared bytes. The number after the
kernel name is how many of the 10 builds contain it.

| kernel (builds) | sm_70 | sm_80 | sm_89 | sm_90 |
|---|---|---|---|---|
| `search_right<unpacked,Row>` (3) | 42 / 0 / 120 / 4096 | 40 / 0 / 120 / 4096 | 40 / 0 / 120 / 4096 | 48 / 0 / 120 / 4096 |
| `search_right<packed,Row>` (3) | 42 / 0 / 120 / 4096 | 40 / 0 / 120 / 4096 | 40 / 0 / 120 / 4096 | 48 / 0 / 120 / 4096 |
| `search_right<*,CompactRow>` (1) | 42 / 0 / 120 / 4096 | 40 / 0 / 120 / 4096 | 42 / 0 / 120 / 4096 | 48 / 0 / 120 / 4096 |
| `search_right<*,DenseRow>` (1) | 42 / 0 / 120 / 4096 | 40 / 0 / 120 / 4096 | 42 / 0 / 120 / 4096 | 48 / 0 / 120 / 4096 |
| `search_right_persistent<DenseRow>` (4) | 72 / 0 / 120 / 4104 | 62–64 / 0 / 120 / 4104 | 50–52 / 0 / 120 / 4104 | 48 / 0 / 120 / 4104 |
| `search_right_fullcache<DenseRow>` (1) | 72 / 0 / 56 / 4104 | 54 / 0 / 56 / 4104 | 44 / 0 / 56 / 4104 | 34 / 0 / 56 / 4104 |
| `resolve_persistent_winner<DenseRow>` (4) | 42 / 0 / 120 / 4096 | 40 / 0 / 120 / 4096 | 40 / 0 / 120 / 4096 | 48 / 0 / 120 / 4096 |
| `build_left<Row/CompactRow/DenseRow>` (9) | 42 / 0 / 120 / 4096 | 42 / 0 / 120 / 4096 | 40 / 0 / 120 / 4096 | 48 / 0 / 120 / 4096 |
| `build_partial` (9) | 42 / 0 / 120 / 4096 | 42 / 0 / 120 / 4096 | 40 / 0 / 120 / 4096 | 48 / 0 / 120 / 4096 |
| `build_right_cache` (3) | 42 / 0 / 120 / 4096 | 42 / 0 / 120 / 4096 | 40 / 0 / 120 / 4096 | 48 / 0 / 120 / 4096 |
| `build_prefix<Row>` (3) | 20 / 0 / 0 / 0 | 20 / 0 / 0 / 0 | 20 / 0 / 0 / 0 | 20 / 0 / 0 / 0 |
| `build_prefix<CompactRow>` (1) | 18 / 0 / 0 / 0 | 18 / 0 / 0 / 0 | 18 / 0 / 0 / 0 | 18 / 0 / 0 / 0 |
| `build_prefix<DenseRow>` (5) | 20 / 0 / 0 / 0 | 20 / 0 / 0 / 0 | 20 / 0 / 0 / 0 | 22 / 0 / 0 / 0 |
| `build_negative_filter<DenseRow>` (3) | 14 / 0 / 0 / 0 | 14 / 0 / 0 / 0 | 14 / 0 / 0 / 0 | 14 / 0 / 0 / 0 |
| `lift_x` (9) | 39 / 0 / 0 / 0 | 39 / 0 / 0 / 0 | 38 / 0 / 0 / 0 | 44 / 0 / 0 / 0 |
| `hash_base<Fp64>` (9) | 40 / 0 / 0 / 0 | 40 / 0 / 0 / 0 | 38 / 0 / 0 / 0 | 40 / 0 / 0 / 0 |
| `hash_base_wide` (9) | 40 / 0 / 0 / 0 | 40 / 0 / 0 / 0 | 38 / 0 / 0 / 0 | 40 / 0 / 0 / 0 |
| `normalize_fixture` (`benchmark.cu`) | 32 / 0 / 56 / 4096 | 32 / 0 / 56 / 4096 | 40 / 0 / 56 / 4096 | 32 / 0 / 56 / 4096 |

The 4096-byte static shared allocation is `normalize_block`'s product tree
(`2 * BLOCK` words); the 120-byte stack is the ABI frame of the
`__noinline__` `fp64_inv` call (its `Fp64Ctx&` argument is spilled to an
addressable slot). No explicit-relations kernel spills.

## Findings, ranked by expected device time × confidence

Ranks weigh how much device time the item plausibly controls in that
directory's own workload against how sure the static evidence is. Rows
marked *changed* are fixed in this branch; the others are recommendations.

| # | dir | finding | evidence | expected effect | status |
|---|---|---|---|---|---|
| 1 | fes | Worker mode without `--low-bits` (which is how `src/cryptanalysis/fes_gpu.rs` calls it) set `low_bits = n`: **one thread** walked the whole `2^n` cube. | `fes.cu` main; `fes_gpu.rs:run_worker` passes only `--in`. | Up to 2^16 threads instead of 1: device time bounded by the slowest sub-cube, roughly `2^(n-16)` of the old walk for n ≥ 24 `[pcc:unverified]`. | changed |
| 2 | fes | Correctness: the kernel kept zeros in `uint64_t local[64]` and dropped every further zero of a sub-cube, count included; with the single-thread default, at most 64 zeros were ever returned. The I/O contract allows `n ≤ 62` but `df[40]` is fixed: `n > 40` wrote past the array (CUDA and Metal). | Host run, n=16 m=3: 8384 zeros by brute force, the old single-thread path would list 64. | Complete solution lists; no out-of-bounds writes. | changed |
| 3 | spmv | Device product by add-and-double: up to 63 dependent compare-subtract steps per nonzero per lane, plus a 64-bit `%` per nonzero per lane. | `spmv.cu`; PTX: 0 → 4 `mul.hi.u64`, in-loop `rem.u64` removed. | Static estimate ~10× fewer instructions per product (≈500 → ≈50 SASS) `[pcc:unverified]`; the worker is one launch inside a text-I/O process, so end-to-end is I/O-bound. | changed |
| 4 | rootfind | `k_decompose_shared_hash` reserved a static 48 KiB of shared memory per block whatever the table size. | 49152 B static smem, all 18 instantiations. | Residency limited by shared memory on sm_70/sm_89 (2 blocks/SM = 8 warps at 128 threads) and sm_80 (3 blocks); register-limited 4 blocks after (16 warps) for 128-register kernels with small tables `[pcc:unverified]`. | changed |
| 5 | rootfind | `d[]` (and `pre[]` for P-256) live in **local memory**, not registers as the header claims: the sweep indexes `d[w]` with a run-time `w`. Stack 240/368/624 B (Fp64 C16/32/64), 1296/2320/4368 B (P-256). Local-memory reservation is stack × resident threads: P-256 C64 on a 132-SM H100 ≈ 4368 B × 2048 × 132 ≈ 1.18 GB of device memory. | `ptxas -v` stack frames. | An unrolled sweep (`#pragma unroll` over `CHUNK` with a `w < cnt` guard) cut Fp64 C16 stack 240 → 112 B with no spill, but introduced 104–360 B spills in other instantiations. Mixed: needs hardware. | recommended (lab card R-B) |
| 6 | macaulay | Update loop computed `idx / width` (64-bit division) per element; pivot search did one shared `atomicMin` per non-zero entry (≈ every row of a random matrix) per column; pivot row re-read from global per element. | PTX: one `div/rem.s64` in the update loop → 0; atomics now ≤ THREADS/32 per column. | The update is the kernel's dominant loop (`rows × (cols − c)` per column); removing a 64-bit division per element is plausibly the larger part of its issue slots `[pcc:unverified]`. | changed |
| 7 | fes | Hot loop: triangular index + branch + global load per variable per step; 832 B stack. | `ptxas`. | Shared-memory broadcast row; stack 832 → 320 B. | changed |
| 8 | explicit-relations | `thrust::device_vector::resize` value-initialises: `rows.resize(p.left)`, `right_full.resize(...)`, `right_first/second.resize(...)` each launch a fill kernel (the `uninitialized_fill` / `construct1_via_allocator` kernels in the build) that writes the whole buffer, which `build_left` / `build_right_cache` / `build_partial` then overwrite completely. | CUB `for_each` fill kernels present in every variant; `prepare()`. | One redundant full write of the left table (12–32 B/row) and of the right cache per `prepare`. Small next to the sums themselves but pure waste. Fix with a default-init allocator or raw `cudaMalloc`. | recommended; not applied (see Decisions) |
| 9 | explicit-relations | `prepare()` `shrink_to_fit`s and reallocates every buffer per plan (cudaFree/cudaMalloc churn); the non-persistent `find` does a blocking D2H copy per chunk. | `device.cuh`. | Per-plan latency only; the persistent path already removes the per-chunk round trip. | recommended |
| 10 | semaev | Default build fails for sm_80+ with CUDA ≤ 12.9: `GF2N_CLMAD=1` emits `clmad.lo/hi`, which `ptxas` 12.9 rejects ("Not a name of any known instruction"). | `ptxas` log. | Build break on CUDA 12.x hosts; `-DGF2N_CLMAD=0` builds (63–80 regs). The `clmad` claim (PTX ISA 9.3) could not be checked here. | recommended (card) |
| 11 | semaev | 312 B stack in both sweep kernels (run-time-indexed `SemPoly::c[d]`, `wide[9]`), `n`/`irr_low` are run-time kernel arguments so no loop over `n` is specialised. | `ptxas`. | Unknown without a profile; host `bench_host` gives per-pair cost on CPU only. | recommended |
| 12 | rootfind | Per call: 8 `cudaMalloc` + 8 `cudaFree`, 2 events, all copies synchronous from caller-owned (pageable) memory. | `run_device`. | Fixed per-call latency (hundreds of µs) that matters only for small batches; pooling needs a context object in the C ABI. | recommended |
| 13 | macaulay | Pivot search and factor caching read a column (stride `cols` words): uncoalesced by layout. | `mac_rref_block`. | Inherent to row-major one-block-per-matrix; a column-major or shared-resident u16 variant (README) is the structural fix. | recommended |

## Changes

All four are untested on a GPU. Identity evidence classes:
**oracle-checked** — a host program compiles the same header the kernel uses
and compares it against the existing CPU reference;
**output-preserving by construction** — the change cannot alter results
(argued per item).

| commit | dir | change | identity evidence |
|---|---|---|---|
| `e1810bfafb` | spmv | Barrett `spmv_mul_mod` (4 mulhi + 5 mullo + 1 correction) replaces add-and-double; coefficients and `x` reduced once on the host; `__restrict__`; 64-bit task index. Math moved to `spmv_math.cuh`. | **oracle-checked**: `make -C gpu/spmv oracle-check` compares `spmv_mul_mod(a mod m, b mod m)` with the CPU worker's add-and-double on raw `b` for 218 moduli (edges 2, 2^62, 2^63−1, …) × 92² operand pairs = 1845152 checks, and the kernel's per-task body with the CPU worker's loop on 300 random CSR problems (28509 tasks): 0 mismatches, also under `-fsanitize=undefined`. The 64-bit task index only differs from the old 32-bit product when `grid × 256 > 2^32`, where the old kernel computed wrong task ids. |
| `9872924cfe` | fes | Default split `fes_default_low_bits(n)` (≤ 2^16 threads); zero-diagonal symmetric table in dynamic shared memory (`n²·8` B: 2.6 KB at n=18, 12.8 KB at n=40); zeros appended straight to the global list; reject `n > 40` in the CUDA and Metal workers. | **oracle-checked**: `make test` now checks `fes_walk_sym` against `fes_search_subcube` sub-cube by sub-cube (same zeros in the same order, same final derivatives) at 5 splits, and the default split against brute force; passed on 7 generated systems (n = 6…22, up to 8384 zeros). Output *changes* where the old kernel truncated (> 64 zeros in a sub-cube) or overflowed (n > 40): it now lists all zeros or refuses. Order of the list is unspecified in both; `fes_gpu.rs` sorts and dedups after re-verifying. The Metal edit is a host-side guard, not compiled here. |
| `ba14c7d9f0` | macaulay | Division-free update cursor; first-hit + warp-shuffle-min pivot search (one `atomicMin` per warp); pivot row in shared memory (dynamic smem `(512+4+cols)·4` B: 2064 → 3208 B at cols=286). | **oracle-checked (index logic)** + **by construction (values)**: `make test` checks the cursor against `idx / width` for every lane at block sizes 32…1024, widths 1…1100 (14451624 positions), and first-hit-then-min against the lowest non-zero row on 2000 random columns. Every element update computes the same `fp_sub(a, fp_mul(f, p))` from the same operands; the shared pivot row equals the global one because row `r` has factor 0 in step 5. `./bench selftest` closes the device gap. |
| `6b25b221ee` | rootfind | Shared hash staged in dynamic shared memory sized `cap·12` B (≤ 48 KiB) instead of a static 48 KiB array. | **by construction**: same keys and values at the same slots, same probe sequence; `make test` (host emulation vs CPU reference) passes. |

### After table (only rows that changed; same toolchain)

| target | kernel | arch | regs | spill st:ld (B) | stack (B) | static smem (B) |
|---|---|---|---|---|---|---|
| fes | `fes_kernel` | sm_70 | 30 → 31 | 0:0 | 832 → 320 | 0 (dyn n²·8) |
| fes | `fes_kernel` | sm_80 | 28 → 32 | 0:0 | 832 → 320 | 0 (dyn n²·8) |
| fes | `fes_kernel` | sm_89 | 28 → 40 | 0:0 | 832 → 320 | 0 (dyn n²·8) |
| fes | `fes_kernel` | sm_90 | 29 → 32 | 0:0 | 832 → 320 | 0 (dyn n²·8) |
| macaulay | `rref_batch_kernel` | sm_70 | 52 → 40 | 0:0 | 0 | 0 (dyn +cols·4) |
| macaulay | `rref_batch_kernel` | sm_80 | 32 | 0:0 | 0 | 0 (dyn +cols·4) |
| macaulay | `rref_batch_kernel` | sm_89 | 34 → 40 | 0:0 | 0 | 0 (dyn +cols·4) |
| macaulay | `rref_batch_kernel` | sm_90 | 38 → 32 | 0:0 | 0 | 0 (dyn +cols·4) |
| spmv | `spmv_kernel` | sm_70 | 28 → 40 | 0:0 | 0 | 0 |
| spmv | `spmv_kernel` | sm_80 | 28 → 42 | 0:0 | 0 | 0 |
| spmv | `spmv_kernel` | sm_89 | 28 → 42 | 0:0 | 0 | 0 |
| spmv | `spmv_kernel` | sm_90 | 30 → 44 | 0:0 | 0 | 0 |
| rootfind | `k_decompose_shared_hash<Fp64,C16>` | sm_70 / 80 / 89 / 90 | 102→80 / 98→90 / 100→80 / 97→90 | 0:0 | 240 | 49152 → 0 (dyn cap·12) |
| rootfind | `k_decompose_shared_hash<Fp64,C32>` | sm_70 / 80 / 89 / 90 | 138→128 / 128 / 140→128 / 128 | 0:0 (sm_90 4:0 both) | 368 (sm_90 376) | 49152 → 0 |
| rootfind | `k_decompose_shared_hash<Fp64,C64>` | sm_70 / 80 / 89 / 90 | 214→168 / 204 / 214→168 / 206 | sm_70 0:0→24:24, sm_89 0:0→24:20 | 624 → 648 (sm_70, sm_89) | 49152 → 0 |
| rootfind | `k_decompose_shared_hash<P256,C16/C32/C64>` | sm_70 / 80 / 89 / 90 | 144→128 / 136→128 / 142–144→128 / 128 | 0:0 | 1296 / 2320 / 4368 | 49152 → 0 |

Every other kernel in the six directories is byte-for-byte the same source
and kept its baseline row. The register increase in spmv (+12–14) lowers the
theoretical residency at 256 threads from 8 blocks (64 warps, sm_80) to 5
blocks (40 warps): the product costs fewer instructions but more live
registers, and card S measures which side wins. The rootfind Fp64 C64 spill on sm_70/sm_89 appears because ptxas
re-balances registers once the static 48 KiB cap on residency is gone; card
R-A measures it, and `RF_MIN_BLOCKS` / `-maxrregcount` can pin the old
allocation if it costs time.

## Rejected or deferred ideas

- **rootfind sweep unroll** (finding 5): tried in a scratch copy. Fp64 C16
  shared/sorted: stack 240 → 112 B, regs 98 → 120 / 90 → 117, no spill;
  but global_hash C16 dropped to 96 regs with 104 B spills, and C32/C64
  spilled 76–360 B. Not committed: the trade is per instantiation and per
  arch and needs measurement (card R-B).
- **Reordering rootfind's sweep into the back-substitution loop** (keeps
  `d[w]` in registers by consuming each inverse as it is produced): changes
  the per-thread emission order. The emitted set is unchanged and the host
  emulation could check it, but it is a result-producing restructure;
  deferred to a measured round.
- **libfes O(1)-amortised second Gray code** for FES: a different
  enumeration algorithm; out of scope without a preregistered round.
- **explicit-relations allocator / merge-sort → radix sort**: the directory
  carries exact-replay validation tied to source hashes (README, v11/v15
  panels); even output-preserving edits change the replayed source, so they
  are recommended, not applied (finding 8–9).
- **semaev `n`-specialised kernels**: output-preserving, but the selftest's
  host and device paths would then differ in how they are instantiated;
  left for after a hardware profile shows the field loop dominates.
- **Pinned host buffers** in spmv/fes workers: each process makes one
  transfer of a text-parsed input; `cudaHostAlloc` would add a host copy and
  allocation cost for one DMA. Not worth it at these sizes.
- **clang-format**: none of the six directories has a `.clang-format` or a
  formatting commit in history; edits match the surrounding style.

## GPU validation cards

Common to every card. Do not run them from an agent session without the
owner's authorization; this document did not submit anything.

- **Host** (AGENTS.md §9): an AWS EC2 GPU instance launched with
  `--key-name meow34`; SSH with `meow34.pem` (mode 0600). Cover the arch
  classes the change claims: at least one of sm_80 (A100, p4d), sm_89 (L4 /
  L40S, g6 / g6e) or sm_90 (H100, p5); name the class in the record and do
  not extend a result to other classes. Record `nvidia-smi -q`, driver,
  `nvcc --version`, CPU model, and lock nothing (default boost) unless the
  card says so.
- **Revisions**: `BASE=e9d5dbdccd` (baseline) and `CAND=6b25b221ee`
  (candidate; contains all four GPU commits, and one unrelated Rust commit).

  ```sh
  git -C ~/crypto fetch origin && cd ~
  git -C crypto worktree add ~/base $BASE
  git -C crypto worktree add ~/cand $CAND
  ```
- **Static half**: for each directory and both trees, rebuild with `nvcc
  -Xptxas -v` (commands below) and diff the register/stack/spill lines
  against the clang table here.
- **Records**: `python3 tools/benchrec.py init --slug <slug> --problem
  ic-gpu-<dir>`; wrap each timed launcher in `benchrec.py run RECORD --`;
  `add-raw` the logs; `metric` the medians; `close`. Fill
  `.resume/RESUME.md` (`scripts/checkpoint.sh init`) first.
- **Timing discipline** (§10): A/A first (baseline vs a second copy of the
  baseline binary) to get the spread; then ABAB with ≥ 5 rounds, alternating
  binaries, reporting median and minimum; a difference inside the A/A spread
  is not a result. Identity checks run on every round.
- **Profiles** (profile skill; explain, never prove): `tools/profile.sh
  nsys --out research/ic-gpu-audit-20261112/profiles/<dir>-<rev>-nsys --
  <cmd>` then `tools/profile.sh ncu --out .../<dir>-<rev>-ncu --kernel
  '<kernel>' --launch-skip 1 --launch-count 1 -- <cmd>` on both revisions.

A minimal ABAB driver (shell; per round prints the binaries' own timing):

```sh
abab() {  # abab ROUNDS CMD_A CMD_B   (commands quoted)
  for i in $(seq 1 "$1"); do
    echo "round $i A"; eval "$2"; echo "round $i B"; eval "$3"
  done
}
```

### Card S — spmv

```sh
for t in base cand; do make -C ~/$t/gpu/spmv cuda-worker host-worker && \
  nvcc -O3 -std=c++17 -arch=sm_80 -Xptxas -v -c ~/$t/gpu/spmv/spmv.cu -o /dev/null; done
make -C ~/cand/gpu/spmv oracle-check                       # host oracle, must print "0 mismatches"
# input: 2^16 rows, 32 nnz/row, 32 lanes, modulus 2^61-1 (values < 2^52, exact in awk doubles)
awk 'BEGIN{srand(1); r=65536; c=65536; l=32; m="2305843009213693951"; z=32;
  printf "SPMV1 %d %d %d %s %d\n", r, c, l, m, r*z;
  for(i=0;i<=r;i++) printf "%d ", i*z; print "";
  for(i=0;i<r*z;i++) printf "%d ", int(rand()*c); print "";
  for(i=0;i<r*z;i++) printf "%.0f ", int(rand()*2^52); print "";
  for(i=0;i<c*l;i++) printf "%.0f ", int(rand()*2^52); print ""}' > /tmp/spmv.in
~/base/gpu/spmv/spmv_solve --in /tmp/spmv.in | sha256sum   # CPU reference digest
for t in base cand; do ~/$t/gpu/spmv/spmv_cuda --in /tmp/spmv.in | sha256sum; done  # all three equal
abab 5 "nsys profile -o /tmp/s-base-\$i --stats=true ~/base/gpu/spmv/spmv_cuda --in /tmp/spmv.in >/dev/null" \
       "nsys profile -o /tmp/s-cand-\$i --stats=true ~/cand/gpu/spmv/spmv_cuda --in /tmp/spmv.in >/dev/null"
```

Metric: `spmv_kernel` duration from the nsys kernel summary (the process
wall time is dominated by text I/O and is not the metric). A/A: the same
loop with base vs base. Also run `IC_SPMV_WORKER=~/cand/gpu/spmv/spmv_cuda
cargo test --lib koblitz_sparse_la` (the Rust caller re-checks every
product).

### Card F — fes

```sh
for t in base cand; do (cd ~/$t/gpu/fes && make cuda host-worker && make nvcc-check); done
(cd ~/cand/gpu/fes && make test && make test N=16 M=3 SEED=11)  # host oracle
for t in base cand; do (cd ~/$t/gpu/fes && ./fes_cuda); done    # compiled-in selftest: MATCH
# contract input, m <= 30 so awk values are exact: n=26 for A/B timing, n=16 m=3 for the cap
gen() { awk -v n=$1 -v m=$2 -v s=$3 'BEGIN{srand(s); printf "%d %d\n%d\n", n, m, int(rand()*2^m);
  for(i=0;i<n;i++) printf "%d ", int(rand()*2^m); print "";
  for(i=0;i<n*(n+1)/2;i++) printf "%d ", int(rand()*2^m); print ""}'; }
gen 26 28 1 > /tmp/fes26.in; gen 16 3 11 > /tmp/fes16.in
for f in /tmp/fes26.in /tmp/fes16.in; do
  ~/base/gpu/fes/fes_solve --in $f | tail -n +2 | sort -n | sha256sum     # CPU reference
  for t in base cand; do ~/$t/gpu/fes/fes_cuda --in $f | tail -n +2 | sort -n | sha256sum; done
done
# expected: cand == CPU on both; base == CPU on fes26 (if the single thread finishes)
#           and base lists at most 64 zeros on fes16 (the cap; a correctness delta, not a timing row)
abab 5 "/usr/bin/time -f 'A %e s' ~/base/gpu/fes/fes_cuda --in /tmp/fes26.in >/dev/null" \
       "/usr/bin/time -f 'B %e s' ~/cand/gpu/fes/fes_cuda --in /tmp/fes26.in >/dev/null"
# split-only control (isolates the shared-memory table): base with the candidate's split
abab 5 "~/base/gpu/fes/fes_cuda --in /tmp/fes26.in --low-bits 10 >/dev/null" \
       "~/cand/gpu/fes/fes_cuda --in /tmp/fes26.in --low-bits 10 >/dev/null"
```

Also: `cargo run --release --bin icx -- fes` with the GPU backend and
`ICX_FES_WORKER=~/cand/gpu/fes/fes_cuda` (see `icx fes --help` for the n/m
flags). ncu kernel: `fes_kernel`.

### Card M — macaulay

```sh
for t in base cand; do (cd ~/$t/gpu/macaulay && make test && make bench ARCH=sm_80 \
  NVCCFLAGS="-O3 -std=c++17 -arch=sm_80 -lineinfo -Xptxas -v"); done
for t in base cand; do (cd ~/$t/gpu/macaulay && ./bench selftest 64 && ./bench occupancy); done
abab 5 "(cd ~/base/gpu/macaulay && ./bench throughput)" "(cd ~/cand/gpu/macaulay && ./bench throughput)"
(cd ~/cand/gpu/macaulay && make test-small)
```

Metric: ms per batch at batch 1024 and 8192 from `throughput` (kernel-only
events). Identity: `selftest` (digest, rank and pivots against the host
serial reduction) must pass on both revisions. ncu kernel:
`rref_batch_kernel` (compare issue-slot use and `smsp__inst_executed`).

### Card R — rootfind

```sh
for t in base cand; do (cd ~/$t/gpu/rootfind && make test && make bench ARCH=sm_80 && make ptxas-info ARCH=sm_80); done
for t in base cand; do (cd ~/$t/gpu/rootfind && ./bench selftest && ./bench p256); done   # vs host emulation
abab 5 "(cd ~/base/gpu/rootfind && ./bench throughput)" "(cd ~/cand/gpu/rootfind && ./bench throughput)"
```

- **R-A** (this change): rows with lookup `hash` and `|F| ≤ 2048` exercise
  `k_decompose_shared_hash`; report achieved occupancy (ncu
  `sm__warps_active.avg.pct_of_peak_sustained_active`) and relations/s for
  CHUNK 16/32/64, both fields. Watch Fp64 CHUNK=64 on sm_89 (new 24 B
  spill); if it regresses, rebuild with `RF_MIN_BLOCKS=2` and report both.
- **R-B** (not committed): the sweep unroll from [Rejected](#rejected-or-deferred-ideas)
  as a third arm, built from a patch on CAND; only with the same identity
  check.

### Card E — explicit-relations (no code change; profile only)

```sh
cd ~/base/gpu/explicit-relations && make test && make test-cuda ARCH=sm_80
make build/explicit-relations-fullcache ARCH=sm_80
tools/profile.sh nsys --out research/ic-gpu-audit-20261112/profiles/er-nsys -- \
  build/paper-driver build/explicit-relations-fullcache --self-test
```

Purpose: size finding 8 (time of the `uninitialized_fill` /
`construct1_via_allocator` kernels against `build_left` + sort in the nsys
kernel table) and finding 9 (`cudaMalloc`/`cudaFree` API time per
`prepare`). Any code change that follows goes through the directory's own
replay validation (`VALIDATION.md`).

### Card Z — semaev

```sh
for t in base cand; do (cd ~/$t/gpu/semaev && make test && make ladder); done
cd ~/base/gpu/semaev && make bench2 ARCH=sm_80 || echo "expected on CUDA <= 12.9: clmad"
make clean && make bench2 ARCH=sm_80 NVCCFLAGS="-O3 -std=c++17 -arch=sm_80 -lineinfo -DGF2N_CLMAD=0"
./bench2 selftest && ./bench2 throughput && ./bench2 occupancy
```

Record which CUDA version (if any) accepts `clmad`, and the clmad vs
portable throughput ratio there, before trusting the 22.4 % figure the
header cites from another client.
