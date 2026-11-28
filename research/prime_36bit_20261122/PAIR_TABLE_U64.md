# Pair-table build in word arithmetic

**Date:** 2026-11-22 (same day as the rung). **Status:** measured, `[pcc:unverified]`.
**Scope:** public synthetic known-answer fixture only; no deployed-curve,
`vs_rho`, or asymptotic claim.

## What changed

`build_pair_sum_table` built every one of the `B(B+1)/2` pair sums with
`Point::add`, the **BigUint** affine add — one modular exponentiation
(`a^(p−2)`) per pair — and only converted the resulting abscissa to a `u64`
for the hash key.  That is why the table stage had grown to 35–48% of the
36-bit rung and why its per-pair cost rose with the factor-base size: the
"u64 fast path" never touched the arithmetic.

The table is now built in `u64` for `p < 2^63`, using the same
`add_point_u64` the probe path already trusts (one extended-Euclid inversion
per pair, no heap allocation).  The BigUint path remains for wider fields.

## Equivalence gate

`examples/prime_pair_table_check` re-derives the table with `Point::add` and
asserts the two agree exactly — same abscissae, same index pairs, same count:

```text
{"bits":16,"fb_size":120,"identical":true,"pairs_word":7260,"pairs_biguint":7260}
{"bits":32,"fb_size":400,"identical":true,"pairs_word":80200,"pairs_biguint":80200}
{"bits":36,"fb_size":512,"identical":true,"pairs_word":131328,"pairs_biguint":131328}
```

A dropped key would lose relations silently, so this is the gate that
matters, and it passes at every rung.  A unit test
(`word_sized_pair_table_matches_biguint_build`) asserts the same thing in CI.

## 36-bit B probe after the change (seed 20261121, CPU time)

| B | IC CPU ms | table ms | relations ms | LA ms | trials |
|--:|--:|--:|--:|--:|--:|
| 2048 | 17880 | 846 | 25566 | 1216 | 13261 |
| 2560 | 16820 | 1022 | 18859 | 2150 | 9253 |
| **3072** | **16340** | 1684 | 15662 | 3577 | 7332 |
| 4096 | 19960 | 3233 | 9419 | 9767 | 5589 |
| 5120 | 27370 | 4780 | 10409 | 21137 | 5525 |
| 6144 | 48320 | 12605 | 11187 | 53101 | 6215 |

Two things move at once.  The table stage stops mattering (846–1684 ms
instead of 15 000–24 000 ms), so the optimal factor base **shifts from 2048
to 3072**.  And the wall beyond 3072 is now set by the **sparse F_n linear
algebra**, whose fill-in grows fast: `nnz_peak` is 9.5 M at B = 3072 against
~30 M at B = 6144, and LA goes 3.6 s → 9.8 s → 21 s → 53 s over
3072/4096/5120/6144.  LA is the next lever, not the table.

## Four seeds at the new optimum, before vs after

Same fixture, same seeds, same host, CPU seconds:

| seed | before: B=2048 CPU ms | table ms | after: B=3072 CPU ms | table ms | relations ms | LA ms |
|--:|--:|--:|--:|--:|--:|--:|
| 20261121 | 33120 | 19082 | **16090** | 1807 | 14752 | 3866 |
| 20261122 | 32700 | 24252 | **14280** | 1191 | 11028 | 2798 |
| 20261123 | 33040 | 19981 | **13770** | 1161 | 10512 | 2419 |
| 20261124 | 31440 | 15585 | **13780** | 1205 | 10327 | 2547 |

IC CPU drops from 31.4–33.1 s to 13.8–16.1 s, i.e. **~2.2×**; the table stage
alone drops **~13×**.  Every seed still recovers the planted scalar.

Against the tuned `cryptanalysis` rho on the identical targets (16–23 ms,
`a3_36bit_rho_paired.jsonl`) the ratio improves from ~1.5×10³ to **~6–9×10²**.
Still three orders of magnitude away; this is a constant-factor win on a
stage that was badly written, not a change of regime.

## Caveat

Wall time on this host moved 14 s → 20 s for identical CPU work across these
runs (load 70–150), so only CPU seconds and per-run stage *shares* are
comparable.  Nothing here is a hardware-independent constant.

## Artifacts

| file | contents |
|:--|:--|
| `examples/prime_pair_table_check.rs` | the equivalence gate above |
| `a3_36bit_u64_table_after.txt` | B probe after the change (CPU + JSON per B) |
| `a3_36bit_u64_seeds_b3072.jsonl` | four seeds at the new B |
| `a3_36bit_seeds_b3072_timing.txt` | `/usr/bin/time -p` per seed + load |
