# Two-bit side fingerprint for the four-byte N83 root index

The opt-in `KIC_INDEXED4_FP2=1` variant stores a two-bit fingerprint for each
four-byte ID31 root slot in a packed side array. It skips most exact-root
reconstructions during collision probing, while a matching fingerprint still
requires full root comparison. It keeps the 31-bit state ID and one-bit root
selector in the four-byte slot. The flag requires `KIC_INDEXED_ROOT4=1` and
`KIC_INDEXED4_ID31=1`; the plain four-byte layout remains available.

At the K5500/K4549 split, the side array contains 805,306,368 bytes for
`2^31 + 2^30` slots. The calculated state, root-table, side-array and dual
`wide3` Bloom sum is 30,720,588,978 bytes (28.61 GiB), versus
29,915,282,610 bytes (27.86 GiB) for plain four-byte ID31 and
33,136,508,082 bytes (30.86 GiB) for five-byte ID31. The FP2 layout still
saves 2.25 GiB against five-byte ID31. These are allocated-array sums;
full peak RSS and pipeline cost require a larger-memory run.

## Proof map

[`Capacity.lean`](Capacity.lean) uses pinned Lean 4.34.1. `packed_fits_byte`
and `packed_slot0` through `packed_slot3` prove the four two-bit positions
decode independently for values below four. `k5500_sidecar_bytes`,
`k5500_fp2_three_array_bytes` and `k5500_saving_vs_five_byte` prove the
exact capacity arithmetic. [`Capacity.check.txt`](Capacity.check.txt)
retains the successful build and `#print axioms` audit. The packing proofs
use standard `propext` and `Quot.sound`; the fixed arithmetic proofs are
axiom-free. Rust source inspection and executable tests connect these
lemmas to the implemented bit operations.

## Paired K128 index stage

[`run.py`](run.py) ran 12 plain/FP2 split-index pairs in alternating AB/BA
order on the same earlier public [point](target.jsonl), K128/K64, four root
threads, and dual `wide3` Bloom filters. The source SHA-256 was
`a99c5a554af48949fe253c28a440ce03095ecdc14eadc8e6087dab8df509e049`;
the executed macOS ARM64 release binary SHA-256 was
`4de2b556aa4963f125ac0fb86b58467d71cb7bcdaf2d74b2e8d99880ea0b6449`.
The [raw receipts](runs/k128_balanced12) passed `PASS_FP2_STAGE_CONTROL`: all
24 runs had 21,248 usable base points, 680,000 states, 1,359,488 root
entries, and identical state and exact-table digests.

| Layout | Table plus side bytes | Median index-build ms, 12 repetitions | Mean ms |
| --- | ---: | ---: | ---: |
| Plain four-byte ID31 | 10,485,760 | 498.456 | 500.345 |
| Four-byte ID31 with FP2 | 11,141,120 | 327.356 | 324.429 |

FP2 reduced the observed median index-build time by 34.3%. It was faster
in all 12 pairs; the median paired plain/FP2 ratio was 1.526, with a
descriptive paired bootstrap 95% interval of 1.474–1.672 (20,000 resamples,
fixed seed 8312). An earlier [three-pair pilot](runs/k128_split3) on the same
source had FP2 faster in two pairs and slower in one; those rows are retained.
This is an adaptive K128 construction diagnostic under shared-host conditions.
Rank, target lookup, peak RSS and Tier 1 comparison remain open. The variant
has `candidate_id: null` until its complete pipeline is sealed and measured.
