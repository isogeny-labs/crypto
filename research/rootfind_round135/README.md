# Root finding Round 135: guarded Atkin sqrt for primes p ≡ 5 (mod 8)

Date: 2026-11-23. Base: [Round 134](https://github.com/isogeny-labs/crypto/pull/5003).

## Change and proof obligation

`src/cryptanalysis/ecbench/isogeny.rs`, `Fp::sqrt_tonelli_shanks`:
for odd prime `p % 8 == 5`, once the binary Jacobi symbol confirms that
`a` is a nonzero quadratic residue, use the single modular exponent
`v = (2a)^((p-5)/8)`, followed by
`i = 2a v²`, `r = a v (i-1)`.
Because `(2a/p)=-1` in this case, `i²=-1` and `r²=a`.
The implementation checks `r²=a` before returning `Some(r)`,
and retains the existing Tonelli–Shanks implementation for all other
prime fields. It does not change field parameters or the public API.

An **initial candidate that skipped Jacobi rejection was rejected**:
local nonresidue square-root queries regressed to ~0.02–0.20x of
the baseline because modular exponentiation was performed for every
nonresidue. The final patch retains the Jacobi check, and this control
returned to near parity.

## Local Linux perf counters, before vs after

Intel Xeon Platinum 8573C, CPU-pinned to CPU 0, Linux 6.18.57;
GCC 14.2 and Clang 17, C++17 -O2 -DNDEBUG algorithmic reproductions.
Linux `perf_event_open` software `PERF_COUNT_SW_TASK_CLOCK`
worked; hardware counters and the standalone perf executable were
not available. **These are C++ reproductions, NOT the production Rust
timings.** Inputs generated before timing; 13 alternating A/B pairs,
first two warmup; table lists median task-clock nanoseconds per operation.
Reported speedup is median of paired before/after ratios. Complete
root-query checksums are accumulated to prevent elimination.

| Field p | Operation | GCC before → after | GCC speedup | Clang before → after | Clang speedup |
| ---: | --- | ---: | ---: | ---: | ---: |
| 65,213 | complete quadratic | 241.47 → 195.84 ns | **1.266x** | 276.21 → 197.12 ns | **1.330x** |
| 1,000,000,021 | complete quadratic | 572.94 → 348.12 ns | **1.729x** | 542.45 → 335.01 ns | **1.645x** |
| 4,294,967,029 | complete quadratic | 858.24 → 497.37 ns | **1.826x** | 1,075.61 → 434.18 ns | **2.092x** |
| 2,305,843,009,213,684,517 | complete quadratic | 1,491.06 → 818.32 ns | **1.815x** | 1,494.44 → 896.79 ns | **1.769x** |
| 1,000,000,021 | square root, residue | 526.91 → 263.34 ns | **1.901x** | 536.35 → 304.58 ns | **1.843x** |
| 4,294,967,029 | square root, residue | 748.70 → 340.88 ns | **2.177x** | 673.80 → 328.03 ns | **1.983x** |
| 1,000,000,021 | square root, nonresidue | 18.77 → 18.88 ns | **0.993x** | 17.96 → 17.90 ns | **1.004x** |

Full 40-row summary and both 260-row raw CSVs are included in the
automation's local artifact. Other workload controls: repeated-root
quadratics near parity; constructed cubics inconclusive and often near
parity. No broad isogeny/EC-index-calculus throughput gain is claimed.

## Correctness and safety

- Local C++ differential checks: exhaustively verified quadratic roots
  and square-root residuosity over 15 small primes (5 to 349) with
  `p ≡ 5 (mod 8)`; additional randomized square/root/cubic checks over
  65,213, 1,000,000,021, 4,294,967,029, 2,305,843,009,213,684,517.
- GCC AddressSanitizer and UndefinedBehaviorSanitizer checks passed.
- New source-exact native Rust workflow compares the **actual production
  module** against the unmodified Round 134 base. It appends identical
  benchmark/oracle helpers to both modules, verifies square-root results
  independently against Euler's criterion and brute-force small-field
  polynomial roots, then times complete quadratic/cubic/isogeny queries.
- This PR should remain draft pending native Rust A/B results and broader
  repository CI. All p ≡ 1 (mod 8) paths stay on the older code. No
  ECC2K relation-finding improvement has been measured.

## Local reproduction

Compile `round135_atkin.cpp` with `g++` / `clang++ -std=c++17 -O2
-DNDEBUG`; `--verify` performs small-prime exhaustive comparisons.
Run the binary without arguments for alternating before/after task-clock
CSV output. Raw CSVs, sanitizer logs, and the standalone source are
distributed as the automation's local results archive.
