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


## Follow-up: AMM and Tonelli exponent reuse (2026-11-18 UTC)

Three additional call-site optimizations are committed to the same source module:

1. \`Cube::root\` (AMM for \`Fp\` and \`Fp2\`): compute \`g=a^t\`
   once for \`|K*|=t*3^s\`, reuse its \`3^(s-1)\`-th power for
   cube-residuosity testing, and use \`g\` or \`g^2\` as the Sylow
   correction. This replaces two long exponentiations by one plus
   a short sequence of cubings; a separate exponentiation for \`x0\`
   is still required. Correctness uses the identity
   \`3*e3-1 = t\` (t mod 3 = 2) or \`2t\` (t mod 3 = 1).
2. \`RadicalRootCtx::sqrt\` (Tonelli-Shanks for p = 1 mod 4):
   derive Euler's criterion from the already-needed \`a^t2\` by
   \`s2-1\` squarings; replace \`c^(2^k)\` with k explicit squarings.
   The p=3 mod 4 branch is unchanged.
3. \`quad_monic\`: replace the generic \`4*c\` multiplication with
   two modular doublings. Values are reduced by the existing guarded
   \`Fp::add\` so noncanonical coefficients also retain semantics.

### Local counter measurements (independent C++ arithmetic mirror)

The test harness uses **the same algorithms** before and after but
**is not the compiled Rust repository**. It is pinned to AMD EPYC 9V74
CPU 0, GCC/Clang -O2, 4,096 deterministic operands per square/cube
test (and 8,192 pairs for quadratic discriminants), thirteen paired
alternating trials, reported as independent medians. Root inputs
include actual square/cube residues and nonresidues. For AMM the
mixed corpus includes early noncube rejection, so a speedup for
that arm should not be confused with the pure-cube workload.

\`perf_event_open(PERF_TYPE_SOFTWARE, PERF_COUNT_SW_TASK_CLOCK)\` worked;
the hardware cycles counter returned **ENOENT** (no exposed PMU).
The \`perf\` command itself could not be installed in this sandbox
because its package repositories are unreachable. The table below
is therefore **software task-clock CPU time**, not hardware cycles.

| Kernel and prime | GCC old ns/op | GCC new ns/op | GCC speedup | Clang speedup |
|---|---:|---:|---:|---:|
| Tonelli sqrt (squares), p=1,000,000,009 | 432.92 | 291.17 | **1.487x** | 1.552x |
| Tonelli sqrt (squares), p=998,244,353 | 946.79 | 864.79 | **1.095x** | 1.142x |
| Tonelli sqrt (squares), p=2,013,265,921 | 1203.29 | 1110.43 | **1.084x** | 1.108x |
| AMM cube (cubes), p=1,000,000,009 | 474.78 | 322.99 | **1.470x** | 1.454x |
| AMM cube (mixed), p=1,000,000,009 | 258.97 | 213.70 | **1.212x** | 1.228x |
| AMM cube (cubes), p=2,013,265,921 | 409.67 | 293.10 | **1.398x** | 1.440x |
| AMM cube (mixed), p=2,013,265,921 | 222.07 | 182.65 | **1.216x** | 1.209x |
| Quadratic discriminant, p=1,000,000,009 | 7.800 | 4.637 | **1.682x** | 1.604x |
| Quadratic discriminant, p=998,244,353 | 7.301 | 4.597 | **1.588x** | 1.610x |

All **51,000** before/after square/cube outputs matched across
17 prime moduli, including high 2-adicity (p=998244353 and
p=2013265921), high 3-adicity (p=487, v3(p-1)=5), zero,
and the 2^61-1 boundary. Discriminants matched exactly for
8,192 operand pairs per measured prime.

### Actual Rust and CI status

To avoid accepting C++ mirror results as Rust results, the PR includes
\`radical_power_reuse_matches_frozen_reference\`, a frozen before/after
test for Fp, Fp2 and Tonelli, plus an **ignored paired Rust benchmark**
(\`bench_radical_power_reuse\`) against the actual module.
A dependency-free isolated test crate imports the **actual production
Rust source by path**, stubbing only unrelated prime-root trace and
primality APIs. The targeted arithmetic and AMM tests do not call
the stubs. The standard full-lib CI remains necessary for complete
polynomial-root and Frobenius fallback validation.

At the time this section was authored, the full-lib CI could not compile
because of unrelated F4/APS duplicate definitions and missing
Hilbert-diagnostic interfaces, before any radical correctness tests
could execute. The separate isolated test gate was added to provide
direct source-level validation despite that blocker; do not merge
until it passes. A separate workflow-parse check was also blocked by
an unrelated unavailable artifact
\`research/weak_curve_properties_lean_20261011/root_lib.stdout\`.

Neither the above task-clock speedups nor the new unit timings
establish an end-to-end rootfinder, GPU, or ECDLP improvement.
