# Exact shared-selector capacity checker

This Rust program evaluates 62 exact rational upper bounds for the M31
shared-chain/tagged-pool question. It uses an included arbitrary-precision
nonnegative-integer implementation and the standard library only. There is no
sampling, curve search, relation search, or performance comparison.

## What was checked

- Four tests passed: arithmetic versus independent `u128` edge cases; the
  Stirling counting identity and Vandermonde identity; exact dyadic rounding;
  exhaustive abstract finite-set partition events versus the bounds.
- The original source was copied to `frozen/exact_bounds.rs`, hashed, and made
  read-only before the pre-call manifest and the first executable call.
- `evidence/certificates.json` contains complete hexadecimal numerators and
  denominators. Every reported exponent is obtained by exact integer comparison,
  not floating-point logarithms.
- `evidence/bounds.tsv` contains the compact result table. Positive exponents
  are clamped to a probability bound of one and mean **inconclusive**.

The written proof is in `../notes/adaptive-chain-proof.md`. Executing the
program checks the arithmetic evaluation, not the general mathematical theorem.
Lean formalization remains open.

The original formula evaluation is single-threaded. The original unit-test
harness used its default test scheduling, so its intended one-thread cap was
not enforced; this is recorded in the post-call manifest. No timing claim is
made. The portable replay script explicitly sets `--test-threads=1`.

## Parameters and interpretation

The canonical native M31 fixture has `p=2147483647`, prime subgroup order
`r=2147446831`, and `M=(r-1)/2=1073723415` nonidentity negation classes.
The model draws `N=32768` distinct classes uniformly without replacement and
asks whether any chain of at most ten reduced quadratic rational maps over
`F_p`, with at most `t` output fibers chosen after seeing the pool, captures
at least `h=128` of those classes. All selection/adaptation of those chains
and output fibers is included in the family union bound.

The chain partition count is bounded by `A=sum(p^(2*j),j=0..10)` and each fiber
has at most `D=1024` points. A second bound also conditions the selected fiber
union to have at most `b` eligible subgroup classes in total. It does not
assume that the degree bound is the actual rational support size.

| Formula | Output fibers t | Total support cap b | Certified probability upper bound |
|---|---:|---:|---:|
| Stirling | 1 | unrestricted within degree bound | 2^-716 |
| Stirling | 4 | unrestricted within degree bound | 2^-405 |
| Stirling | 8 | unrestricted within degree bound | 2^-207 |
| Stirling | 12 | unrestricted within degree bound | 2^-66 |
| Stirling | 14 | unrestricted within degree bound | 2^-5 |
| Stirling | 15 | unrestricted within degree bound | inconclusive |
| Anchors | 16 | 128 | 2^-864 |
| Anchors | 32 | 128 | 2^-458 |
| Anchors | 32 | 1024 | 2^-7 |

An upper bound becoming inconclusive at `t=15` does not establish a construction
with 15 fibers. Correlated tag generation, maps on the elliptic curve rather
than the stated univariate rational-map family, greater depths, larger pools,
and different selector families require their own analysis. These are scope
boundaries, not universal ECDLP lower bounds.

The fixed-chain complete-fiber rows have a different, narrower hypothesis:
the selector is frozen independently of the uniform tag pool, while the
output fibers may be chosen afterward. Do not use those rows for freely
adaptive chain fitting without a corresponding family factor.

## Reproduce

From this directory, with an available Rust compiler:

```sh
RUSTC=/absolute/path/to/rustc ./replay.sh /absolute/path/to/new-replay-directory
```

The replay directory must not already exist. The script recompiles the frozen
source, runs the tests, and compares both the compact table and every full
rational certificate with the preserved evidence. It does not overwrite the
original pre-call, build, or post-call records.

Original compiler: `rustc 1.100.0 (0ef1bc386 2026-11-09)`,
`x86_64-unknown-linux-gnu`, LLVM 23.1.5. Original commands and binary hashes are
in `evidence/`. The source uses stable standard-library Rust and no external
crate downloads.
