# gpu/macaulay — batched row reduction over `F_p`

CUDA kernels for the one part of elliptic-curve index calculus that maps
onto a GPU without an argument: thousands of independent, identically
shaped Macaulay reductions over one small prime field.

**No kernel here has been run on a GPU.** The environment this was
written in had no NVIDIA device. What *is* verified is the arithmetic and
the serial reduction, against an independent Python oracle, on the CPU —
see [How this is tested](#how-this-is-tested).

## Why this shape, and why not RDMA

`src/cryptanalysis/gaudry_cubic.rs` solves one symmetrised-`S₄` system
per residual of an index-calculus run on `E(F_{p³})`.
`research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md` §11.6 measures that solve at `0.88 · 10⁶`
`F_p` multiplications, of which the Macaulay step is **57 %** (17 %
forward elimination, 40 % normal forms). Every residual produces the
same shape over the same `p`; there are thousands per run; no residual
depends on another.

That is a batched-LU problem. The measured shape at `p = 2083` is
**226 × 286 with rank 222**, which is what `make test` runs by default —
the `DEFICIT=4` in the Makefile is chosen to reproduce that rank, because
a real Macaulay matrix is always rank-deficient (its shifted rows carry
syzygies) and a full-rank test matrix would not exercise the same path.

It is also the answer to *"can we ship the decomposition to a CPU pool
over RDMA?"*, which is where this directory started. A residual is
`x_R` — three field words, **24 bytes** — and the matrix is *built* from
it, not shipped. At ~20 ms of work per residual that is about 1 KB/s of
inbound bandwidth per CPU core kept busy. There is nothing to move, so
there is nothing for RDMA to accelerate; the interesting question is
whether the reduction itself belongs on the GPU, and this is the kernel
that would answer it.

## Layout

| File | What it is |
|---|---|
| `mref.py` | Pure-Python oracle: generates `params.h`, the test vectors, and every expected answer |
| `fpmod.cuh` | Arithmetic mod a small odd prime — Montgomery, not `%` |
| `macaulay.cuh` | `mac_rref_serial` (host + device) and `mac_rref_block` (device only) |
| `test_cpu.cpp` | Verification harness — compiles the `.cuh` headers with g++ |
| `bench.cu` | Device driver: selftest, throughput sweep, occupancy report |

## Build and test

```bash
make test                       # needs only Python 3 and a C++17 compiler
make test P=271 ROWS=64 COLS=80 # a second shape
make test-small                 # both, in sequence
make bench                      # CUDA binary (needs nvcc)
make bench ARCH=sm_100          # datacenter Blackwell; sm_120 for RTX 50xx
```

## How this is tested

`make test` compiles `fpmod.cuh` and `macaulay.cuh` with a plain C++
compiler and checks five things against `mref.py`, which computes every
expected answer in plain Python:

| Check | What it covers |
|---|---|
| Montgomery round-trip, `mul`/`add`/`sub`/`inv` | 20,000 random products against `(a*b) % p`, every inverse against `1` |
| Five literal matrices | entry-by-entry equality of the reduced form, and the pivot column list |
| A rank-deficient case | the path a real Macaulay matrix always takes |
| The real shape | generator digest, reduced digest, spot entries, full pivot list |
| Batch independence | reducing eight instances together gives what reducing each alone gives, and the instances are checked to be distinct so the comparison is not vacuous |

The large case ships as a **seed**, not as a matrix: `mref.py` and
`macaulay.cuh` implement the same splitmix64 stream, so the header
carries a 64-bit digest of the expected reduced form plus four spot
entries. A wrong reduction fails the digest; a wrong generator fails the
spot entries and the pre-reduction digest.

### What the CPU harness cannot cover

`mac_rref_block` does not exist outside `__CUDACC__`, so no amount of
host testing touches it. That gap closes with

```bash
./bench selftest        # block kernel vs the verified serial reduction
```

on real hardware, which compares the two **on the device**, instance by
instance, including the pivot lists and the rank. Run it first.

## The two reductions, and which is right

`mac_rref_serial` and `mac_rref_block` both produce the **full reduced**
row echelon form. The CPU implementation in `gaudry_cubic` deliberately
does not: §11.5 replaced Gauss–Jordan with forward elimination plus
back-substitution memoised to the normal forms actually asked for, and
measured that as a `3.1×` cut in `C₃`. The reduced form does strictly
more arithmetic.

It is kept here anyway because the trade is the whole question. The
memoised version is cheaper in total work and almost entirely serial —
each normal form depends on the ones below it. The reduced form is more
work and all of it parallel. Which wins is throughput against work, and
nothing in this directory assumes an answer; `./bench throughput` is
where it would be measured, and it prints the CPU number to compare
against rather than a bare matrices-per-second.

## Known limits

- **The `u32` matrix does not fit in shared memory at this shape.**
  `226 × 286 × 4 B = 258 KB` against 228 KB on Hopper, so the global path
  is bandwidth-bound. At `p < 2^16` — which covers every size
  `gaudry_cubic` runs, and `p` grows only as `n^{1/3}` — the matrix is
  staged in shared memory as `u16` (129 KB + 3 KB scratch,
  `rref_batch_kernel_smem`, the same `mac_rref_block` with `uint16_t`
  storage). `bench.cu` takes that path when the device's opt-in shared
  memory per block holds it (sm_80: 163 KB, sm_90: 227 KB; not sm_86/89 at
  99 KB) and falls back to the global kernel otherwise; `MAC_SMEM=0`
  forces global, `MAC_SMEM_THREADS=256|512|1024` sets the block (default
  1024). Measured on an A100 80GB PCIe (2026-11-13), batch 8192: 718 ms
  global, 197 ms shared at 1024 threads, 177 ms with the shared path's 2D
  update mapping (`MAC_UPDATE_2D`, default there), with `selftest` passing
  on both paths and both shapes. Evidence:
  `research/ic_gpu_measure_20261112/REPORT.md`.
- **The pivot search and the inverse are serial**, one per column, 286
  of them. They bound how much a larger block can buy; `./bench
  occupancy` reports the launch configuration to tune against.
- **`p < 2^31`**, from the `u32` Montgomery form.
- **No `__launch_bounds__` tuning**, no multi-GPU, no streams.

## What this does not claim

Under `AGENTS.md`, a GPU is an **engineering** row: wall-clock is a
practicality note, never the metric, because operation counts survive
hardware. Moving this reduction to a GPU does not change `C₃`, does not
change the ratio to the floor, and does not move the crossover with
Pollard rho by a single bit — §11.7 puts that crossover past `2^{230}`
for the large-prime variant and shows `S` bottoming out around `200×`
rho near `2^{50}`.

What it would buy is the ability to run the *experiments* at larger `p`,
which is where the open questions in §11.6 and §11.8 actually live.
