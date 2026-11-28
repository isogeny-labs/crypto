# rho-speed investigation (2026-11-10)

Two questions were asked of the walk-step kernel: what does it do in a real
chained walk, and what would it take to be fast there.

## Measured

| quantity | value | where |
|---|---:|---|
| chained walk, 65,536 walkers x 64 steps | **3.00 M updates/s**, 333 ns/update, `chain_exact: true` | `walk.py` on L4 |
| step benchmark (independent inputs) | 3.73 M steps/s | `bench.json`, v5 |
| production client (packed CUDA, G7) | **12.5 B updates/s** | `../README.md` |
| clmad.lo/hi.u64 via Triton, PRO 6000 | **46.9 G clmul64/s**, verified vs host | `clmul_test.py`, `modal_clmul.py` |

So the step is already rho-shaped — chaining costs only ~20% over the step
benchmark. The gap to the client is 4,170x and it is not batching, it is the
multiplier and the iteration.

## Why the client is 4,170x faster

1. **Carry-less multiply.** The client's product is `clmad.lo.u64` /
   `clmad.hi.u64`, 1 per 37.7 logic ops (measured in `benchmarks/clmad-price/
   probe.cu`). Our product is 17,161 bit-pair AND/XOR gathers. `clmad` is
   Blackwell only (sm_120) and needs a CUDA 13.3+ ptxas; PTX has no
   `pclmulqdq` at all, so on an L4/sm_89 the instruction does not exist.
2. **Basis.** The client stays in the polynomial basis, where sigma is a
   cheap operation on limbs. Our kernel stays in the normal basis, where the
   product is a 263-point convolution and sigma is a bit permutation — cheap
   for sigma, catastrophic for the product.
3. **Products per update.** The client runs the x-only doubling map with a
   sparse sigma^3 bridge: 4.81 products per update. Our affine step does 10
   (8 for the Itoh-Tsujii inversion).

## What a Triton port could reach, and why it is the wrong tool

Measured primitive rate 46.9 G clmul64/s on one PRO 6000. A 263-bit product
is 25 clmul64 schoolbook (vs 6 with the client's 2-way Karatsuba).

- product in the same normal-basis kernel: 25 clmul + pack/unpack between the
  bit-array and limb forms. The conversions are ~1,300 logic-lane-ops per row
  against 25 clmuls = 15 SM-clocks, i.e. conversions cost more than the
  multiplication: ~15x over today, ~45 M updates/s.
- polynomial basis, no conversions, still 10 products/update: ~2.9 G
  updates/s, within ~4x of the client.
- the last 4x is Karatsuba and the 4.81-product iteration, i.e. the client.

Triton 3.4 also refuses to drive a CUDA 13.3 ptxas outright
(`ptx_get_version` has no CUDA 13 entry); 3.5.1 works. That is why the
clmul probe runs in its own app (`modal_clmul.py`, CUDA 13.3.1 image,
`TRITON_PTXAS_PATH=/usr/local/cuda/bin/ptxas`).

## Conclusion

For rho work the packed CUDA client remains the fast path. A Triton kernel
built on `clmad` could plausibly reach a few G updates/s only after moving to
the polynomial basis, which is a rewrite of the field layer, not a tuning pass.

## Update: clmad works from Triton, and what it buys (2026-11-10)

Primitive verified on an RTX PRO 6000 (sm_120) with a CUDA 13.3 ptxas:

```
clmad.lo.u64 / clmad.hi.u64 via tl.inline_asm_elementwise   46.9 G clmul64/s
263-bit product (lift + 5x5 clmad + fold)     0.0136 ms / 65,536 rows
bit-pair convolution (v5)                     0.4300 ms / 65,536 rows   (31.7x slower)
bits->limbs pack (two of them)                0.1600 ms
limbs->bits unpack                            0.0440 ms
----------------------------------------------- chain 0.218 ms = 1.98x
```

Whole step, 65,536 rows, same card, both exact:

| step | ms | updates/s |
|---|---:|---:|
| v5, bit-pair convolution | 5.049 | 13.0 M |
| v6, clmad product | 2.582 | **25.4 M** |
| production client (G7) | — | 12,499 M |

So clmad doubles the walk rate, and the step is now ~80% representation
conversion: 10 muls x (2 packs + unpack) = 2.04 ms of the 2.58 ms. The
multiply itself is only 0.14 ms of the step.

Two bugs worth recording, both caught by the exactness gate:
  * `>>` on int64 is arithmetic in Triton and sign-extends product limbs —
    the product must run in uint64 (logical shifts);
  * the row index must include `program_id(0) * BLOCK`, not `arange(0, BLOCK)`.

Next lever: keep the state in packed limbs so sigma costs no conversion.
That means the polynomial basis, where sigma is squaring (shift + reduce)
rather than a bit permutation over a byte array — the client's design.

## Update: batching the per-row kernels (2026-11-10, later)

Every per-row kernel was one row per program: a 256-lane tile with three
quarters of the lanes masked off. Batched to R=8 rows per program:

| kernel | before | after | |
|---|---:|---:|---|
| pack bits->limbs | 0.092 ms | 0.028 ms | 3.3x |
| unpack limbs->bits | 0.051 ms | 0.025 ms | 2.0x |
| sigma / xor / copy / frob / pack-out | same shape | batched | |

Step, 65,536 rows, RTX PRO 6000, `step_exact: true` at each rung:

| step | ms | updates/s | vs client |
|---|---:|---:|---:|
| v5 convolution (L4) | 17.6 (scaled) | 3.0 M | 4,167x |
| v5 convolution (PRO 6000) | 5.049 | 13.0 M | 961x |
| clmad + scalar conversions | 2.582 | 25.4 M | 492x |
| clmad + batched conversions | 1.296 | 50.6 M | 247x |
| v6, all per-row kernels batched | **1.125** | **58.3 M** | **214x** |
| production client (G7) | — | 12,499 M | 1x |

Bugs found by the exactness gate (each one cost a run, all now fixed):
  * `>>` on int64 is arithmetic — product limbs must be uint64;
  * row index needs `program_id(0) * BLOCK`, not `arange(BLOCK)`;
  * a tuple-of-tuples loop is not tracers-able — unroll it;
  * scratch left dirty by an earlier benchmark pass invalidated a "false"
    exactness result; always re-unpack before the check.

## Remaining distance, honestly

At 58.3 M/s the client is 214x away. The remaining levers, in order:

  * conversions (pack/unpack round every multiply) — the bit-per-byte layout
    suits sigma; the polynomial basis removes them and makes sigma a square;
    est. 2-4x;
  * Karatsuba: 25 clmul per product -> 6; est. 3-4x on the product term;
  * the client's iteration: 4.81 products/update (x-only doubling + sigma^3
    bridge, inversion batched over 16 slots) vs our 10; est. 2x.

Those compound to roughly 20x, i.e. ~1 G updates/s — still 12x short of the
client, which sits on the clmad+ALU floor (34 SM-clocks/update: 29 clmad and
~1,100 ALU ops). Every further 2x is now a redesign, not a tuning pass.

## Update: one representation (2026-11-10, later still)

v7 — every kernel that writes a slot also emits its packed limbs (three masked
sums on a tile it already holds), so the multiply no longer needs pack kernels:

    step 1.125 ms -> 0.863 ms   (76.0 M updates/s, exact)

v8 — sigma reads limbs directly. Output bit t comes from source bit
p = table[k][t], i.e. limb p//64 at shift p%64, so a 3-way select and a shift
replace the byte gather; the same masked sums rebuild the limbs. The byte
array disappears, and with it every unpack:

    step 0.863 ms -> 0.829 ms   (79.0 M updates/s, exact)

Only 4% for v8: the unpacks were not the bottleneck. The step is now 10 clmad
products (0.136 ms) plus twelve 131-bit permutations, each costing a 3-way
select, a shift and three 256-lane reductions (about 0.69 ms). That is the
shape of the remaining cost, and it is why the polynomial basis is next: there
sigma is a square (spread + reduce), not a permutation plus a repack.

## The full ladder

| step | ms | updates/s | vs client |
|---|---:|---:|---:|
| v5 convolution, L4 | — | 3.0 M | 4,167x |
| v5 convolution, PRO 6000 | 5.049 | 13.0 M | 961x |
| clmad, scalar conversions | 2.582 | 25.4 M | 492x |
| clmad, batched conversions | 1.296 | 50.6 M | 247x |
| v6, every per-row kernel batched | 1.125 | 58.3 M | 214x |
| v7, limbs emitted by the producer | 0.863 | 76.0 M | 164x |
| v8, single representation | 0.829 | 79.0 M | 158x |
| production client (G7) | — | 12,499 M | 1x |

26x over where this file started, all bit-exact against include/ref.h through
walk_step_problem.Model.

## Update: launch shapes (2026-11-10, tuning pass)

Sigma and frob were ~9x off their ALU bound, which said occupancy rather than
arithmetic. Sweeping the tile width and warp count (tune.py):

| kernel | best | ms | previous | |
|---|---|---:|---:|---|
| sigma | R=16, 2 warps | 0.0386 | 0.0682 (R=8, 4 warps) | 1.77x |
| frob | R=8, 2 warps | 0.0725 | 0.1267 | 1.75x |
| mul | B=128, 4 warps | 0.0163 | 0.0163 | — |

Fewer warps is better because the tiles are small: four warps leave most of a
2048-lane tile idle, two fill it.

    step 0.829 ms -> 0.435 ms   150.5 M updates/s   (buffers reused, exact)

As a drop-in `kernel_function` (v9.py) that also allocates and zeroes the
14-slot limb arena and copies the output each call, the same step measures
0.558 ms / 117.4 M updates/s, exact at 65,536 rows and at an odd 40,001-row
batch (which exercises the padding path).

## Ladder to date

| step | ms | updates/s |
|---|---:|---:|
| v5 convolution, L4 | — | 3.0 M |
| v5 convolution, PRO 6000 | 5.049 | 13.0 M |
| clmad, scalar conversions | 2.582 | 25.4 M |
| clmad, batched conversions | 1.296 | 50.6 M |
| v6, all per-row kernels batched | 1.125 | 58.3 M |
| v7, limbs emitted by the producer | 0.863 | 76.0 M |
| v8, single representation | 0.829 | 79.0 M |
| **v9, tuned launch shapes** | **0.435** | **150.5 M** |
| v9 as a drop-in function | 0.558 | 117.4 M |
| production client (G7) | — | 12,499 M |

50x over where this started (39x as a drop-in call). The client is now 83x
ahead, and its advantage is the algorithm: 4.81 products/update with the
inversion batched over 16 walkers, against our 10 products and 127 sigma
applications per step.

## Update: shapes swept on the step, and a negative result (2026-11-10)

Per-kernel optima do not compose: sigma likes one warp, the small xor/copy
kernels are worse at one warp because each block then owns only 32 threads.
Sweeping the shapes on the step itself (step_sweep.py):

| sigma | frob | step | updates/s |
|---|---|---:|---:|
| R16, 1 warp | R8, 1 warp | 0.292 ms | 224.5 M |
| R8, 1 warp | R8, 1 warp | 0.296 ms | 221.4 M |
| R16, 2 warps | R8, 2 warps | 0.443 ms | 147.9 M |
| R8, 2 warps | R8, 1 warp | 0.439 ms | 149.2 M |

v9.py now carries R16/1 warp for sigma and R8/1 warp for frob. Measured on its
own, one call at a time (cold clocks): 0.428 ms drop-in / 153.2 M updates/s,
0.420 ms with a reused arena / 156.1 M. Under the sustained load of the sweep
the same step ran 0.292 ms / 224.5 M, so the figure is clock-dependent; quote
150-225 M and say which.

**Fusing the whole step into one kernel does not help (v10.py).** The step is
row-local, so it can live in registers in a single launch, and that was the
obvious move once an empty kernel measured ~13 us on the same grid. It is
exact at every shape tried, but slower: 1.03 ms at one warp, and the time
scales *linearly* with warp count (1.03 / 2.01 / 4.00 ms for 1/2/4 warps),
which is register spilling, not arithmetic. Fourteen live slots plus the
transient (R,256) sigma tile do not fit. The 13 us "launch overhead" was an
artefact of timing an empty kernel with a CUDA-event pair, so the step was
never launch-bound in the first place.

## Ladder, final

| step | ms | updates/s |
|---|---:|---:|
| v5 convolution, L4 (start) | — | 3.0 M |
| v5 convolution, PRO 6000 | 5.049 | 13.0 M |
| clmad, scalar conversions | 2.582 | 25.4 M |
| clmad, batched conversions | 1.296 | 50.6 M |
| v6, all per-row kernels batched | 1.125 | 58.3 M |
| v7, limbs emitted by the producer | 0.863 | 76.0 M |
| v8, single representation | 0.829 | 79.0 M |
| v9, shapes tuned on the step | 0.428 | 153 M |
| v9 under sustained load | 0.292 | 225 M |
| production client (G7) | — | 12,499 M |

50x as a single call, 75x sustained, over where this file started.

## Update: sigma loses its lane waste (2026-11-10, v11)

Every sigma still built a 256-lane tile for 131 useful bits. Same fix the
multiply got in v5: a 128-lane main chunk (bits 0..127 -> limbs 0 and 1) plus
a 4-lane tail (bits 128..130 -> limb 2), so ~99% of the lanes are useful.

    v9   single call 0.428 ms   153.2 M updates/s
    v11  single call 0.303 ms   216.5 M updates/s     (1.41x)
    v11  reused arena 0.297 ms  221.0 M
    v11  sustained     0.289 ms 226.5 M

Exact at 65,536 rows and at an odd 40,001-row batch. A shape re-sweep on the
new kernel confirms sigma R16/1 warp and frob R8/1 warp (R32 is within noise,
R64 is 1.7x worse).

## Ladder

| step | ms | updates/s |
|---|---:|---:|
| v5 convolution, L4 (start) | — | 3.0 M |
| v5 convolution, PRO 6000 | 5.049 | 13.0 M |
| clmad, batched conversions | 1.296 | 50.6 M |
| v7, limbs emitted by the producer | 0.863 | 76.0 M |
| v8, single representation | 0.829 | 79.0 M |
| v9, shapes tuned on the step | 0.428 | 153 M |
| **v11, sigma without lane waste** | **0.303** | **216 M** |
| production client (G7) | — | 12,499 M |

72x over where this file started. The client is 58x ahead, and its advantage
is now entirely algorithmic: it amortises one inversion over 16 walkers
(4.81 products and ~8 sigma applications per update) where this step pays 10
products and 127 sigma applications for every walker, because the benchmark
problem *is* one affine addition with its own inversion.

## Update: CUDA graph (2026-11-10, v12)

Each sigma moves ~3 MB and does ~8.6M lane-ops but measured ~12 us, so a large
part of the step was per-launch submission across ~30 launches. Fusing them
into one kernel is not the fix (v10: register spilling), so v12 captures the
whole launch sequence in a CUDA graph once per row count and replays it,
copying the inputs into place first.

    v11  single call   0.303 ms   216.5 M updates/s
    v12  graph replay  0.208 ms   314.5 M updates/s   (1.45x)
    v12  re-swept      0.214 ms   306.0 M  (best shapes still sigma R16/frob R8, 1 warp)

Exact on the first input, on a second different input (the graph replays, the
copies change), and on an odd 40,001-row batch (its own graph).

Caveat: a graph pins the row count and the buffer addresses, so there is one
graph per shape; that is fine for a benchmark and for a real walk, which has a
fixed geometry, but it is not a general-purpose function any more.

## Ladder

| step | ms | updates/s |
|---|---:|---:|
| v5 convolution, L4 (start) | — | 3.0 M |
| v5 convolution, PRO 6000 | 5.049 | 13.0 M |
| clmad, batched conversions | 1.296 | 50.6 M |
| v8, single representation | 0.829 | 79.0 M |
| v9, shapes tuned on the step | 0.428 | 153 M |
| v11, sigma without lane waste | 0.303 | 216 M |
| **v12, CUDA graph replay** | **0.208** | **314 M** |
| production client (G7) | — | 12,499 M |

102x over where this file started. The client is 40x ahead.
