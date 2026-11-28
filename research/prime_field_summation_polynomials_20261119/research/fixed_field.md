# Fixed-field reduction experiment

Date: 2026-11-19 UTC

## Requirement and status

| Requirement | Status | Evidence |
| --- | --- | --- |
| Remove software 128-bit remainder from targeted hot multiplication paths | Implemented and assembly-verified | `results/raw/fixed_field_disassembly.txt` |
| Preserve arbitrary-prime support | Implemented | Explicit generic fallback and `--reduction generic` |
| Check arithmetic equivalence | Recorded local self-test pass for stated campaign `[pcc:unverified]` | 2,268,486 matched products and 1,225 inverse identities |
| Measure root-stage impact | Provisional branch measurement | `results/fixed_field_comparison.csv`; final serial integration run remains required |

## Reducers and exact ranges

`Field` selects one of three paths at construction.

### Mersenne reducer for `p = 2^31 - 1`

For reduced `a,b < p`, put `x=a*b < 2^62`. Since `2^31 = 1 (mod p)`,

```text
r = (x & p) + (x >> 31)
```

is congruent to `x` modulo `p`. The stated input bound makes `r < 2p`, so one
conditional subtraction produces the canonical residue.

### Reciprocal Barrett reducer for other `p < 2^32`

Precompute `mu=floor(2^64/p)`. For reduced inputs, `x=a*b < p^2 < 2^64` and

```text
q = high64(x*mu)
r = x - q*p
```

Here `q` is either `floor(x/p)` or one less: replacing `1/p` by `mu/2^64`
introduces an error strictly smaller than `x/2^64 < 1`. Therefore `0 <= r < 2p`
and one conditional subtraction is sufficient. The randomized campaign also
includes `p=4294967291`, close to the upper range boundary.

### Generic fallback

Other accepted primes retain `(u128(a)*b) % p`. This maintains the original
`p < 2^63` CLI scope; the test includes the prime `2^61-1` on this path.
`--reduction generic` forces the same path for comparison.

## Correctness evidence

Command:

```bash
make -j5 all
./build/semaev_lab self-test
```

Observed:

```text
PASS self-test s3=900 s4=900 root_methods=900 discriminants=900
  reducer_pairs=2268486 reducer_inverses=1225
```

The reducer checks comprise:

- all `1009^2 = 1,018,081` pairs at `p=1009`;
- 250,000 seeded random pairs plus Cartesian edge cases at each of
  `p=1009,1000003,2147483647,4294967291,2^61-1`;
- inverse identities sampled during those runs; and
- the pre-existing Semaev/root/resultant tests, now executed through the automatic
  reducer paths.

Every product is compared to the original 128-bit remainder implementation. This
is strong finite evidence and an exhaustive check only for `p=1009`; the range
argument above is the justification for all supported 32-bit moduli.

## Assembly evidence

The disassembly of `Field::mul` shows three branches. Only the generic branch
calls `__umodti3`. The Mersenne branch uses multiply, shift, mask, add, and
conditional move. The Barrett branch uses `mulq`, multiply/subtract, compare, and
conditional move. See `results/raw/fixed_field_disassembly.txt`.

The binary still imports `__umodti3` because arbitrary-prime fallback, primality
testing, and forced-generic benchmarking remain compiled in. The claim is limited
to execution of the two specialized `Field::mul` branches.

## Provisional matched timing

Commands for each campaign prime:

```bash
./build/semaev_lab benchmark --prime P --a 2 --b 3 \
  --seed 6000285944033488934 --min-arity 3 --max-arity 9 \
  --bench-cases 200 --bench-repeats 20 --reduction auto \
  --output results/fixed_field_auto_pP.csv

./build/semaev_lab benchmark --prime P --a 2 --b 3 \
  --seed 6000285944033488934 --min-arity 3 --max-arity 9 \
  --bench-cases 200 --bench-repeats 20 --reduction generic \
  --output results/fixed_field_generic_pP.csv
```

Degree-128 root-stage observations:

| `p` | Automatic reducer | Forced generic | Observed ratio |
| ---: | ---: | ---: | ---: |
| 1009 | 10.581 us | 11.512 us | 1.088x |
| 1,000,003 | 10.019 us | 18.036 us | 1.800x |
| 2,147,483,647 | 12.343 us | 18.053 us | 1.463x |

These values include root generation plus sort/dedup, start with lifted points,
and exclude point sampling and polynomial construction exactly as in the baseline
benchmark. All 200 cases per degree were equality-validated before timing.

Other parallel optimization tracks were active on the same five-CPU host during
this run, so these numbers are **provisional**. They preserve the commands and
observations but do not establish final ratios. The integrated branch must rerun
the matched workloads serially before making a performance claim.

## Reproduction

```bash
make test
./scripts/compare_reducers.py results
objdump -d -C build/semaev_lab | \
  awk '/<Field::mul\\(unsigned long, unsigned long\\) const>:$/{show=1} show{print} show && /^$/{exit}'
```

## Limits and next steps

- The runtime reducer tag adds two predictable comparisons per multiply. A
  compile-time fixed-field instantiation could remove them, but was not measured.
- No constant-time side-channel claim is made for the surrounding algorithms.
- SIMD lane arithmetic and 64-bit-prime Montgomery reduction are not implemented
  in this track.
- The branch timings are not a substitute for the required serial integrated
  benchmark.
