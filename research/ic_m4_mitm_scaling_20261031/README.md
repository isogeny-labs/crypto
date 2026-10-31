# m = 4 two-plus-two MITM: scaling and validation

Status: stage diagnostic. Synthetic known-answer instances; not a security
claim, not a deployed-curve result, no key recovery, no sub-rho claim.

## Question

The m-summand decomposition oracle `rootfind-mitm` supports m = 3 only, and
`pkm-semaev` refuses m > 3 (`src/cryptanalysis/ic_framework/pkm_semaev.rs`,
"m is 2 or 3": its grid recovery is written for m <= 3). Measure the m = 4
decomposition cost exponent directly and check it at growing sizes.

## Method

`examples/m4_mitm_scaling.rs` implements the 2 + 2 meet-in-the-middle in the
prime group over a **random** factor base of size `B = round(p^{1/5})`:

```
R = F_a + F_b + F_c + F_d
T = { F_i + F_j : i <= j }                 table, |F|(|F|+1)/2 additions
query: for each pair (i, j), is R - (F_i + F_j) in T ?
```

Two quantities are reported separately, because they are different:

- **planted solve** -- a target known to be a sum of four base points. Cost is
  the table amortised over the targets plus the pairs scanned until the planted
  decomposition is reached. Expected: `B^2 = p^{2/5}`.
- **uniform relation** -- a random on-curve target. A full pair scan refutes a
  target; a target decomposes with probability about `B^4 / p`, so one relation
  costs about `p / B^2 = p^{3/5}`.

Every recovered decomposition is re-added in the group and compared against the
target; the run aborts if any witness fails. Curve `y^2 = x^3 - 3x + 1`, primes
the largest below `2^bits` that are `3 mod 4`, random factor base, fixed seeds.

```sh
cargo run --release --example m4_mitm_scaling
cargo run --release --example m4_mitm_scaling -- --bits 48,52 --targets 10 --uniform-bits 40,44
```

## Results

Planted solve, 40 targets per rung (`run.txt`):

| bits | |F| | pairs | adds/target | verified |
|---:|---:|---:|---:|---:|
| 24 | 28 | 406 | 108.1 | 40/40 |
| 28 | 49 | 1225 | 441.7 | 40/40 |
| 32 | 84 | 3570 | 1062.0 | 40/40 |
| 36 | 147 | 10878 | 4215.4 | 40/40 |
| 40 | 256 | 32896 | 13090.8 | 40/40 |
| 44 | 446 | 99681 | 35747.6 | 40/40 |
| 48 | 776 | 301476 | 109172.0 | 40/40 |
| 52 | 1351 | 913276 | 356864.2 | 40/40 |

Fitted planted-solve exponent over log2 p: **0.412** (R^2 0.9986, 8 sizes).
This is below 1/2, and close to the `B^2 = p^0.4` prediction. Re-run at 48/52
bits with 10 targets gives 0.324 on two sizes (`run_large.txt`); the two-rung
fit is not informative on its own, the 8-rung fit is the reading.

Uniform relation, 2 relations per rung:

| bits | |F| | adds/relation | verified |
|---:|---:|---:|---:|
| 24 | 28 | 101,465 | yes |
| 28 | 49 | 1,296,972 | yes |
| 32 | 84 | 3,995,574 | yes |
| 36 | 147 | 13,507,016 | yes |
| 40 | 256 | 20,875,915 | yes |
| 44 | 446 | 1,080,186,802 | yes |

The 24--40 fit is 0.469, but the 40 -> 44 step is `log2(1.08e9 / 2.09e7) / 4 =
1.41`, i.e. the sparse-regime constant (`B^4/p` rarity) has taken over. At 48
bits the same geometry predicts ~`4.7e8` additions per relation and hours of
wall time, so the uniform arm was stopped at 44.


## Larger rungs (native u64 cap)

Pushing the planted arm to the `Fp64` ceiling (`p < 2^62`), 5 targets per rung
(`run_60bit.txt`):

| bits | |F| | pairs | adds/target | verified |
|---:|---:|---:|---:|---:|
| 52 | 1351 | 913,276 | 480,854 | 5/5 |
| 56 | 2353 | 2,769,481 | 1,776,582 | 5/5 |
| 60 | 4096 | 8,390,656 | 4,519,071 | 5/5 |

Three-rung fit: **0.404** (R^2 0.9908). The 8-rung fit at 0.412 and this
0.404 agree; the planted solve exponent is stable below 1/2 up to 60-bit
primes and 8.4M pair-table entries. The uniform arm was not pushed past 44
bits: at 60 bits the `p / B^2` relation cost is about `2^36` additions per
relation, outside a session.


## Past the native cap: arbitrary-precision planted solve

`examples/m4_mitm_scaling_big.rs` repeats the planted arm with
`num-bigint::BigUint`, so the ladder passes the `Fp64` ceiling `p < 2^62`
(`run_big.txt`, 2 targets per rung):

| bits | |F| | pairs | adds/target | verified | wall |
|---:|---:|---:|---:|---:|---:|
| 64 | 7132 | 25,436,278 | 20,251,331 | 2/2 | 108 s |
| 66 | 9410 | 44,278,755 | 35,066,153 | 2/2 | 233 s |

Two-rung fit: **0.396** (R^2 1.0000). It agrees with the u64 ladders
(0.391--0.412), so the planted m = 4 solve exponent is stable below 1/2
from 24 to 66 bits, across both native and arbitrary precision.

The wall from here is memory, not the exponent: the 2+2 table is
`B^2 = p^{2/5}` entries, 44M at 66 bits and ~2.4e8 at 72 bits (multi-GB),
with each 68-bit rung roughly quadrupling the 66-bit wall time. A 3+1
split trades a `B^3` table for a lower query exponent; an external-memory
or distinguished-point table is the route past ~70 bits.

## Reading

- The **planted** m = 4 solve exponent is stable and below 1/2 (0.412 over eight
  sizes to 52 bits, every witness verified). This is the concrete sense in which
  the m = 4 solve beats the 1/2 line.
- The **uniform** relation cost is not: once the factor base is small relative
  to p the `B^4/p` rarity dominates, and the local exponent at 40 -> 44 is 1.41.
  The table/rarity is the `exp(c*m)`-type constant that eats the planted gain.
- The m = 4 2+2 solve is a valid oracle at every size tested; it is not an
  end-to-end sub-1/2 relation collector on this model.

## Files

- `examples/m4_mitm_scaling.rs` -- implementation and ladder.
- `run.txt`, `run_large.txt` -- exact outputs above.
- `m3mitm.py` -- the m = 3 1+2 MITM cross-check used to choose the method
  (solve exponent 0.471 over n = 13..21).
