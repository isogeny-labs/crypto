# Coefficient materialization: subproduct-tree experiment

## Requirement and status

| Requirement | Status | Evidence |
| --- | --- | --- |
| Preserve root multiplicities | Recorded local self-test pass `[pcc:unverified]` | Direct vector equality against the sequential builder, including forced repeated roots |
| Handle odd and non-power-of-two counts | Recorded local self-test pass `[pcc:unverified]` | Self-test sizes 3, 7, 31, 127 and benchmark implementation |
| Check across campaign fields | Recorded local self-test pass `[pcc:unverified]` | 135 self-test comparisons over all nine field/curve fixtures |
| Benchmark through degree 4096 | Implemented; timings provisional | Five validated cases per degree/field, three timed repetitions; parallel worktree activity shared five CPUs |
| Select a crossover automatically | Implemented | Conservative degree-256 threshold; quadratic below, product tree at and above |
| Fast NTT/FFT multiplication | Not implemented | Karatsuba avoids field-root-of-unity and external dependency requirements |

## Implementation

The baseline inserts one root at a time:

```text
F <- 1
for r in roots: F <- F * (X-r)
```

Its arithmetic cost is quadratic in the number of roots. The new path creates
all linear leaves `(X-r)` and multiplies adjacent pairs in a balanced tree. Each
node uses schoolbook multiplication for small or strongly unbalanced operands and
Karatsuba above 32 coefficients. The asymptotic multiplication work is
`O(n^log2(3))` with the current implementation.

No roots are sorted or deduplicated. Repeated leaves therefore preserve the exact
multiplicity semantics of the sequential builder. Empty input returns the monic
constant polynomial `1`.

## Correctness evidence

`make test` reported:

```text
PASS self-test s3=900 s4=897 root_methods=900 discriminants=900 coefficient_methods=135
```

The 135 new checks are 15 root counts over nine field/curve pairs. Counts are
`0,1,2,3,4,7,8,16,31,32,64,127,128,256,512`. Every nonempty case forces a zero
root, every size at least two forces a duplicate, and every size at least three
includes `p-1`.

The benchmark independently compared the complete coefficient vectors before
timing at every power-of-two degree from 2 through 4096. That adds 180 validated
vectors: 12 degrees times five cases times three fields.

A separate integration control generated 200 complete JSONL records at arities 3
through 10 with both forced methods and compared them byte-for-byte. They matched;
the command receipt is `results/raw/product_tree_validation.log`.

## Provisional measurements

These measurements ran while other parallel research worktrees shared the same
five CPUs. They establish the observed crossover and scaling trend for this run,
but final integration must rerun them serially before making stable performance
claims.

Conditions: g++ 14.2.0, `-O3 -march=native`, seed
`6000285944033488934`, five polynomials per degree, three timed repetitions.
Random-root generation and validation are outside the timed interval; allocation
of the output polynomial and internal tree nodes is included.

| Field | Degree | Sequential | Product tree | Sequential / tree |
| ---: | ---: | ---: | ---: | ---: |
| 1009 | 256 | 267.57 us | 245.12 us | 1.092x |
| 1009 | 4096 | 62.91 ms | 21.23 ms | 2.963x |
| 1,000,003 | 256 | 275.14 us | 247.66 us | 1.111x |
| 1,000,003 | 4096 | 64.56 ms | 18.85 ms | 3.424x |
| 2,147,483,647 | 256 | 274.17 us | 252.12 us | 1.087x |
| 2,147,483,647 | 4096 | 66.41 ms | 20.76 ms | 3.199x |

Across all three fields, degree 128 was not a stable universal win: the product
tree was slower on two fields in this run. Degree 256 was the first measured point
where it won on every field, which is why `auto` uses 256. Field- or machine-
specific calibration could choose a different cutoff.

Raw CSVs are `results/optimization/product_tree_p*.csv`; command output is in
`results/raw/product_tree_p*.log`.

## Reproduction

```bash
make test
for prime in 1009 1000003 2147483647; do
  ./build/semaev_lab coeff-benchmark \
    --prime "$prime" --a 2 --b 3 --seed 6000285944033488934 \
    --min-degree 2 --max-degree 4096 \
    --bench-cases 5 --bench-repeats 3 \
    --output "results/optimization/product_tree_p${prime}.csv" \
    2>&1 | tee "results/raw/product_tree_p${prime}.log"
done
```

An initial attempt to wrap the command with `/usr/bin/time -v` failed because
that executable is absent in this environment. The benchmark itself subsequently
ran normally; the failed command and error are retained in
`results/raw/product_tree_attempts.log`.

## Limits and next steps

- This improves coefficient construction only `[pcc:unverified]`. It does not speed root discovery
  and should be skipped entirely when callers only need roots.
- The tree allocates many temporary vectors. Arena allocation or reusable level
  buffers may lower the crossover.
- NTT-friendly fields, multimodular convolution, or a mature polynomial library
  may improve high-degree multiplication further `[pcc:unverified]`; none was measured here.
- The automatic degree-256 cutoff is an implementation default, not a theorem or
  a global optimum.
