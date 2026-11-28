# Relation collection in the cold M31 solve, profiled (2026-11-28)

The solver is the frozen producer `prime31-graph-mitm-control` at `577d6c2c`,
the same build as `../m31-mst-cold-share-20261128/`, rebuilt here with
debug info (`CARGO_PROFILE_RELEASE_DEBUG=true`, otherwise `--locked
--release`). Its cold solve recovers `987654321` after 11 queries; relation
collection is 89 % of its 5,084 ms wall (4,516 ms).

There, `cold-solve-runs.json` has the wall split; collection per query is
about 410 ms. [pcc:unverified]

**Method.** Each query does the same collection work: one probe per pair
table entry, 915,304 probes. Each probe computes `r - left.sum` (one group
addition) and looks the result up in the pair table. Callgrind ran whole
solver processes with caps of 1 and 2 queries (`RUNS.txt`). Their
difference, 1.898 G instructions, is exactly one query's collection plus
its graph step. The graph step is 0.4 % here (`main` includes it).

Per-function and per-line tables:
- `exclusive-per-query.txt`
- `inclusive-per-query.txt`
- `arithmetic-lines-per-query.txt` (lines of `prime31_ic_20261115/native/lib.rs`)

They come from `callgrind_diff_units.sh`.

## One query: 1.898 G instructions, 915,304 probes (about 2,070 per probe)

| where | instructions | share |
|---|---:|---:|
| field multiplication: `(x as u128 * y as u128) % p` line 474.3 M + `__umodti3` 820.4 M | 1,294.7 M | 68.2 % |
| exponentiation loop control in `pow` (`while`, `if e & 1`, `e >>= 1`) | 118.1 M | 6.2 % |
| SipHash of the probe point for the table lookup (`DefaultHasher::write` 153.9 M, `hash_one` 97.0 M) | 250.9 M | 13.2 % |
| `sub` (`(x + p - y) % p`) 40.3 M, counters 30.2 M, `neg` 11.9 M, `add` control 28.2 M | 110.6 M | 5.8 % |
| probe loop in `main` (incl. hash-table probing and match handling) | 74.1 M | 3.9 % |
| below the 1 M-per-line threshold or otherwise unattributed | 50 M | 2.6 % |

Charged counts per query: 915,617 group additions, 915,561 field inversions
and 58.6 M field multiplications. That is 64 multiplications per addition,
at about 22 instructions each.

## Cause

`Arithmetic::add` makes one field inversion per affine addition.
`Arithmetic::inv` computes it as `x^(p-2)` by square-and-multiply, with
31 squarings and 30 multiplications for `p = 2^31 - 1`. So 61 of the 64
multiplications per probe are the inversion. Inversion, its multiplications
plus the loop control, is about 1.35 G of 1.898 G: **about 71 % of
collection**. Each multiplication reduces with a generic `u128 % u128`,
compiled to a `__umodti3` call, although `p` is the Mersenne prime `2^31 - 1`.
The pair-table build, 10 % of the cold solve, uses the same addition.

## Exact levers (not implemented; estimates are from these per-call costs, not measured)

1. **Batch inversion (Montgomery's trick).** Apply it across one query's
   915,304 probes, or across the table build. Every probe's denominator is
   independent, so the result is the same points. Inversion cost per probe
   falls from 61 multiplications plus loop control (about 1,480 instructions)
   to about 3 multiplications plus the amortized single inversion. Projected
   probe cost: about 660 instructions, about 3x fewer for collection, before
   extra memory traffic for the batch buffers.
2. **Mersenne reduction for `p = 2^31 - 1`.** Use `(t & p) + (t >> 31)` twice
   instead of `__umodti3`. This takes about 22 instructions per multiplication
   down to a few. It also helps any remaining multiplications, the table
   build and `sub`'s `% p`.
3. **Cheaper lookup hash.** SipHash on `Option<(u64, u64)>` is 13 % of
   collection. A non-cryptographic hash, an x-coordinate key or a sorted
   table removes most of it. The table holds public points, so this
   introduces no DoS concern.

Together these leave collection dominated by the probe loop and memory. Any
change must reproduce the solver's relation rows, recovered scalar and
counted group work. The frozen runs here give that check (`run1.json`
hashes in `../m31-mst-cold-share-20261128/SHA256SUMS`). It must also be
compared against matched rho before any speed claim.
