# Radical root-finder field arithmetic: local screening (2026-11-17 PT)

## Scope

The changes are in `src/cryptanalysis/prime_radical_root.rs`, not the GPU
`gpu/rootfind` kernel. This is a public-input radical polynomial root finder
over prime fields `5 <= p <= 2^61-1`, including Cardano/Ferrari and
Adleman-Manders-Miller in `Fp` and `Fp2`.

Candidates retained:

1. `Fp::add`: replace a 128-bit remainder with u64 addition and a single
   conditional subtraction for canonical inputs. Preserve the old wide
   remainder for noncanonical inputs.
2. `Fp::sub`: replace a 128-bit remainder with a conditional modular
   correction. Preserve the old path for noncanonical inputs.
3. `Fp::mul`: for **p = 2^61-1** and canonical inputs, reduce the
   122-bit product by two Mersenne folds, otherwise retain the original
   wide remainder.
4. `Fp::inv`: replace Fermat exponentiation with ordinary signed
   extended Euclid. All field moduli are prime and at most 2^61-1; coefficient
   intermediates fit i64. This method is **variable-time**, strictly for
   public-input root finding, and must not handle secrets.
5. `Fq2::mul`: for the common quadratic nonresidue `n=2` or `n=3`,
   replace one base-field multiplication by one or two modular additions,
   respectively. Other n take the existing path.

## Local experiment (preliminary, NOT actual Rust timings)

This container cannot resolve GitHub DNS, install Rust, or install the
`perf` executable. A standalone, noinline C++17 implementation mirrors the
old and proposed modular arithmetic **including canonical-input guards**.
It is **not** a compiled checkout of the Rust source. Each candidate was
checked against the original wide-remainder arithmetic over nine primes
(including the p=2^61-1 boundary), 50,000 random pairs per prime,
inversions and `Fp2` products. All tests passed.

Measurements: pinned AMD EPYC 9V74 virtual CPU, Linux 6.18.57, Debian 13,
GCC and Clang `-O2`, 8192 deterministic operands, alternating
before/after arms, median of 13 paired passes. Linux
`perf_event_open(PERF_TYPE_SOFTWARE, PERF_COUNT_SW_TASK_CLOCK)` counters
were available and agree with wall times. Hardware cycles returned
`ENOENT`; the kernel exposes no CPU PMU. Measurements include the
standalone harness's function-call overhead. They are **not** representative
of compiled Rust call-site inlining or whole-rootfinding throughput.

### Representative GCC -O2 task-clock medians

| Function | Field | Before ns | After ns | Speedup |
|---|---|---:|---:|---:|
| Guarded modular addition | p=2^61-1 | 4.382 | 3.057 | 1.433x |
| Guarded modular subtraction | p=2^61-1 | 4.172 | 2.947 | 1.416x |
| Guarded Mersenne multiplication | p=2^61-1 | 4.683 | 2.981 | 1.571x |
| Extended Euclid inverse vs Fermat | p=2^61-1 | 636.466 | 122.190 | 5.209x |
| Fp2 multiplication, all arithmetic optimizations | p=2^61-1, n=3 | 35.428 | 19.741 | 1.795x |
| Guarded modular addition | p=1,000,000,007 | 5.106 | 2.710 | 1.884x |
| Guarded modular subtraction | p=1,000,000,007 | 4.681 | 2.509 | 1.865x |
| Generic multiplication control | p=1,000,000,007 | 4.059 | 5.308 | 0.765x |
| Extended Euclid inverse vs Fermat | p=1,000,000,007 | 178.502 | 62.519 | 2.855x |
| Fp2 multiplication, all arithmetic optimizations | p=1,000,000,007, n=3 | 34.760 | 25.066 | 1.387x |

The generic multiplication guard caused inconsistent measurements in
separate processes: 0.765x in the first process versus 0.978x, 0.940x
and 1.005x in three later independent processes. **Do not claim it has
no regression** before the actual Rust benchmark is run. Run it with
realistic modulus mixes; an alternate design would specialize the context
once instead of checking the modulus on each multiply.

The same C++ experiment with Clang -O2 gave for p=2^61-1:
guarded add 2.485x, guarded sub 2.426x, guarded multiplication 1.398x,
inverse 4.358x and Fp2 overall 1.959x. This confirms the broad trend
but illustrates compiler sensitivity.

## Verification gates

Run the actual Rust source (not just this C++ mirror):

```sh
cargo test --release --lib optimized_field_arithmetic_matches_wide_reference
cargo test --release --lib cube_root_exhaustive_fp_and_fp2_both_classes
cargo test --release --lib matches_prime_root_trace_on_random_large_instances
```

Then run a dedicated before/after **actual Rust** benchmark pinned to a
CPU with the same compiler, field, inputs and timing window. Use
`perf stat -r 7 -e cycles,instructions,branches,branch-misses,cache-misses -- ...`
on a self-hosted PMU-enabled machine. Check low-degree roots/sec and total
solver time across p=1e9+7, p=2^61-1, quadratic/cubic/quartic classes
(including fallback), and both `Fp` and `Fp2` cube-root paths.
No ECDLP speedup or GPU improvement is claimed.

This PR remains draft until the actual Rust tests and benchmark gate pass.
