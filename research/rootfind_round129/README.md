# Round 129: monic factorization for rational isogeny root extraction

## Scope

Stacked on Round 128 PR #4971, base commit `b9041ccc17150f58cd0a823daf7ad9d575108e07`. Production module: `src/cryptanalysis/ecbench/isogeny.rs`. The code solves prime-field polynomial roots for rational 2- and 3-isogeny transfer; this is **not** the binary ECC2K point-decomposition or relation finder.

## Actual production optimizations

1. **Normalize the input polynomial once** to monic before `x^p mod f`, avoiding inversion of the original leading coefficient at every modular remainder.
2. **Normalize the final gcd once** before splitting; Euclidean gcd otherwise yields an arbitrary leading scalar even when the input is monic.
3. **Normalize recursive gcd factors** before exact quotient extraction and subsequent split exponentiations. Perform exact division by monic factors without any leading-coefficient inversions or cancelled-leading-term multiplications.
4. **Extract roots from monic linear factors** by negation alone rather than invoking a Fermat inversion.

Mathematically `f` and `c*f`, `c != 0` in `F_p`, define the same root set and quotient ideal. Scaling gcd factors by a nonzero field element preserves divisibility and degree, and the complementary quotient is correspondingly rescaled. Root ordering and duplicate elimination are unchanged.

## Native Rust: source-exact A/B, PASS

Workflow: [Round129 native Rust run](https://github.com/isogeny-labs/crypto/actions/runs/45125645100). Both modules are copied unmodified from the Round128 base commit and the candidate PR, with only benchmark entrypoints appended. `rustc 1.100.0`, optimized, 13 alternating A/B trials. Units: median ns per **complete query**; speedup median of paired ratios.

| p | Complete `find_isogeny(2)` | Complete `find_isogeny(3)` | psi3 polynomial roots | Constructed quartic (leading 3) |
| --- | ---: | ---: | ---: | ---: |
| 65,521 | 5,733 → 4,243 ns (1.349x) | 11,408 → 6,323 ns (1.805x) | 6,969 → 4,161 ns (1.678x) | 35,603 → 17,786 ns (2.000x) |
| 1,000,000,007 | 7,723 → 6,120 ns (1.262x) | 21,198 → 9,109 ns (2.328x) | 19,827 → 8,678 ns (2.280x) | 76,942 → 29,192 ns (2.637x) |
| 4,294,967,291 | 13,445 → 9,059 ns (1.486x) | 30,905 → 11,851 ns (2.606x) | 33,704 → 12,699 ns (2.653x) | 113,018 → 38,899 ns (2.908x) |
| 2^61 - 1 | 24,533 → 16,672 ns (1.471x) | 60,950 → 22,621 ns (2.699x) | 74,205 → 25,581 ns (2.900x) | 291,775 → 83,690 ns (3.490x) |

Independent oracle PASS on both exact Rust modules for 5 prime fields, using known-root cubic/quartic checks (monic and nonmonic), evaluation of discovered psi3 roots, and exhaustive psi3 checks for p=101. All native A/B checksums match. No full-crate build claim.


### Independent native Rust replication

The successful [re-run of the source-exact job](https://github.com/isogeny-labs/crypto/actions/runs/45125645100) used the same exact Rust sources and benchmark cases, with a new CI execution. Median paired speedups remained positive:

| p | find_isogeny(2) run 1 / run 2 | find_isogeny(3) run 1 / run 2 | psi3 roots run 1 / run 2 |
| --- | ---: | ---: | ---: |
| 65,521 | 1.349x / 1.457x | 1.805x / 1.990x | 1.678x / 1.763x |
| 1,000,000,007 | 1.262x / 1.253x | 2.328x / 2.496x | 2.280x / 2.543x |
| 4,294,967,291 | 1.486x / 1.499x | 2.606x / 2.721x | 2.653x / 2.845x |
| 2^61 - 1 | 1.471x / 1.465x | 2.699x / 2.646x | 2.900x / 2.827x |

The two native executions passed their independent correctness oracles and paired checksums. Absolute timing changed between CI attempts, so report paired ratios rather than claiming clock-stable nanoseconds.

## Local Linux perf task-clock: PASS

AMD EPYC 9V74, CPU 0 affinity, GCC 14.2 and Clang 17, `-O3 -march=native`, `perf_event_open(PERF_TYPE_SOFTWARE, PERF_COUNT_SW_TASK_CLOCK)`. 9 alternating A/B trials, *two different-seed runs*; below is the median paired **speedup range across two runs** for the C++ reproduction of the production algorithm.

| Benchmark | GCC 1e9+7 | Clang 1e9+7 | GCC (2^32-5) | Clang (2^32-5) |
| --- | ---: | ---: | ---: | ---: |
| ψ3 quartic root queries | 2.733–2.924x | 2.751–2.858x | 3.409–3.580x | 3.091–3.253x |
| Known-root quartic, leading 3 | 3.318–3.544x | 3.168–3.206x | 3.775–4.088x | 3.459–3.546x |
| Known-root monic quartic | 2.513–2.702x | 2.452–2.480x | 2.553–2.910x | 2.478–2.547x |

These are **C++ arithmetic reproductions**, not Rust timings. Linux task-clock software counters succeeded; hardware cycles/instructions/cache counters and the standalone `perf` binary were unavailable. Both compilations were CPU pinned. Absolute before/after medians, paired ratios and environment details are in the attached Round 129 artifact.

## Correctness

Local C++ differential oracle: **1,106,600** paired polynomial/arithmetic comparisons across 5 fields and 11 candidate variants (including known roots and psi3); independent brute-force checks for p=101. GCC ASan/UBSan passed on a separate reduced workload with no reported errors. The production Rust tests added in this PR cover nonmonic scaling, three/four rational roots and exhaustive small-field 3-division polynomials. They still require regular crate-level testing.

## Evaluated and rejected

- Symmetric polynomial squaring reduces unique products but the C++ GCC complete-root benchmark often regressed; Clang improved only modestly, so not retained.
- Skipping the first identity `mulmod` in exponentiation gave noisy and inconsistent complete-query performance.
- Sparse convolution checks slowed dense cases; not retained.

The retained speedup primarily comes from removing *repeated modular inversions* in `rem`/GCD/split. Optimization of arithmetic kernels alone was weaker.

## Remaining gates

- Full repository CI and broad crate tests, including the separate malformed `prime_root_trace.rs` baseline issue #4959.
- Diverse real curve parameters, full `find_isogeny` correctness/homomorphism, and mixed-degree downstream workload profiling.
- Actual ECC2K index-calculus relations/sec remains **unmeasured**. Do not infer a binary-field cryptanalytic speedup.

Keep this PR draft until those integration gates are resolved.