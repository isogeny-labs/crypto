# Reusable-workspace and SIMD investigation

Date: 2026-11-19 UTC

## Scope and status

| Requested lead | Status | Evidence |
| --- | --- | --- |
| Reuse scratch allocation | Implemented and validated | `MitmWorkspace`, 900 randomized fixtures plus forced degeneracies |
| Remove per-root quotient/remainder | Implemented | reverse traversal uses nested `li`/`ri` loops |
| Separate generation from sort/dedup | Implemented and measured | `workspace_generate_ns`, `workspace_distinct_ns`, `sort_dedup_ns` columns |
| Fast common nondegenerate path | Implemented with exact fallback | common-path count retained per benchmark row |
| SIMD | Investigated, not implemented | compiler vectorization report records blockers |

The reusable API is `roots_mitm_workspace_into`. It retains half tables,
denominators, numerators, prefixes, modes, and output capacity across calls. The
original allocation-owning API remains unchanged. CLI callers can select the
new implementation with `--method workspace`.

## Correctness gates

`make test` passed all existing controls across nine field/curve combinations:

```text
PASS self-test s3=899 s4=899 root_methods=900 discriminants=900
```

Each randomized root check now compares naive, Gray, original MITM, and workspace
MITM multisets and infinity counts. A deterministic `P,P` input is also checked
on every trial; it forces both doubling and a signed sum at infinity through the
fallback path. Every timed case compares the workspace distinct-root set with
both original MITM and coefficient-only factoring before timing.

## Provisional measurements

Commands:

```bash
for p in 1009 1000003 2147483647; do
  ./build/semaev_lab benchmark --prime "$p" --a 2 --b 3 \
    --seed 6000285944033488934 --min-arity 3 --max-arity 9 \
    --bench-cases 200 --bench-repeats 20 \
    --output "results/optimization/workspace_p${p}.csv"
done

./build/semaev_lab benchmark --prime 1000003 --a 2 --b 3 \
  --seed 6000285944033488934 --min-arity 9 --max-arity 9 \
  --bench-cases 200 --bench-repeats 100 \
  --output results/optimization/workspace_refined_p1000003.csv
```

Other parallel research processes were using the five available CPUs, so these
numbers are explicitly provisional and must be rerun serially after integration.
The refined degree-128 `p=1000003` diagnostic observed:

| Stage | Mean ns/polynomial |
| --- | ---: |
| Original MITM + distinct output | 11,515.858 |
| Workspace root generation only | 7,915.192 |
| Workspace + distinct output | 11,298.318 |
| Reused-copy sort/dedup only | 2,914.130 |

The scoped observed ratio for matched distinct output was 1.019x. This does not
establish a stable gain. It does show that sorting/dedup is material at degree
128 and that eliminating allocation alone is not the dominant remaining cost.
At small degrees the retained workspace reduced fixed allocation overhead more,
but those measurements are also provisional.

## SIMD investigation

The exact compiler command and full diagnostic are retained in
`results/optimization/vectorization.log`:

```bash
g++ -O3 -march=native -std=c++20 -Wall -Wextra -Wpedantic -Wconversion \
  -fopt-info-vec-optimized -fopt-info-vec-missed src/semaev_lab.cpp \
  -o build/semaev_lab_vec
```

GCC did not vectorize the root hot loops. Reported blockers include control flow,
non-affine memory evolution, throwing calls, and the prefix-product dependency.
The binary still imports `__umodti3`, so modular multiplication also contains an
opaque software-remainder operation. Explicit SIMD was not added because it
would duplicate the concurrent fixed-field-reduction work and would not safely
vectorize the current generic 64-bit modulus path.

The next SIMD gate is therefore: first integrate division-free fixed-field
reduction, then move the common path to structure-of-arrays storage and vectorize
the independent denominator/numerator and final-x stages. Prefix products remain
serial within a batch unless a parallel prefix scheme is used.

## Limits

- The workspace object is caller-owned and not shared-thread safe; independent
  threads should use independent workspaces.
- The common path scans the small half tables for infinity and detects zero
  denominators while filling arrays. Exact degeneracies take the preserved
  fallback.
- Generation-only timing does not include sorting, y-coordinate recovery, input
  sampling, polynomial construction, or output serialization.
- No explicit SIMD speedup is claimed or measured `[pcc:unverified]`.
