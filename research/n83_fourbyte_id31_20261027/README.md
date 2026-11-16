# Four-byte ID31 exact root slots for the N83 split index

The opt-in `KIC_INDEXED4_ID31=1` layout stores a 31-bit state ID and a
one-bit root selector in each four-byte slot. The producer reconstructs the
exact root key before accepting a hit. It extends the existing four-byte
root table beyond its 29-bit state-ID limit while retaining the `RootOnly11`
state array and load80 split sizing. For the K5500/K4549 split, the two
tables use **12 GiB** instead of **15 GiB**. The calculated state, table and
dual-`wide3`-Bloom array sum falls from 33,136,508,082 bytes (30.86 GiB)
to **29,915,282,610 bytes (27.86 GiB)**, a 3 GiB saving. Buffers,
allocator overhead, rank storage and peak RSS still require measurement.

The four-byte token is `2 × (state_index + 1) + root_selector`. Zero marks an
empty slot; the largest admitted state ID and selector encode as
`0xffffffff`. The five-byte ID31 control retains its eight-bit fingerprint.
The four-byte mode uses no fingerprint, so it reconstructs a candidate root
at every occupied probed slot. It is a memory option, with its build and
lookup costs to be measured before promotion into an IC candidate.

## Implementation and proof

The code is in
[`examples/koblitz_orbit_dlp_fast.rs`](../../examples/koblitz_orbit_dlp_fast.rs).
The mode requires symmetric N83 traversal, `RootOnly11`, packed exact root
slots, `KIC_INDEXED_ROOT4=1`, and `KIC_INDEXED4_ID31=1`. With
`KIC_INDEXED5_SPLIT_AT=<core>`, it constructs separate primary and overflow
four-byte tables using the same load80 request formula as the five-byte
split. Existing index modes remain opt-in and unchanged.

[`Capacity.lean`](Capacity.lean), pinned to Lean 4.34.1 by
[`lean-toolchain`](lean-toolchain), proves the slot round trip, nonzero
four-byte range, K5500 state-ID bound, split requests and exact byte sums.
[`Capacity.check.txt`](Capacity.check.txt) retains the command, successful
build and `#print axioms` output. The structural slot proofs use standard
`propext`, `Classical.choice` and `Quot.sound`; fixed arithmetic lemmas are
axiom-free. `lean4checker --fresh` was unavailable. The Rust-to-Lean bridge
is checked through source inspection and the executable tests below; it is
not a Lean theorem about Rust execution.

| Result | Lean declaration | Scope |
| --- | --- | --- |
| Token decode | `slot_roundtrip` | `state + 1 < 2^31`, selector `< 2` |
| Nonzero 32-bit token | `slot_nonzero_u32` | same hypotheses |
| K5500 state and split limits | `k5500_state_fits`, `k5500_core_request`, `k5500_extension_request` | symmetric state and load80 request formulas |
| Three-array capacity | `k5500_four_byte_table_bytes`, `k5500_table_saving`, `k5500_three_array_bytes` | two tables, dual `wide3` Bloom filters, 11-byte states |

The 25 example unit tests pass, including boundary encoding, collision
reconstruction and comparison with real N83 S3 roots. The build used Rust
1.93.1 on macOS ARM64. This is a local correctness and stage-cost control;
cross-platform correctness and Tier 1 timing are subsequent gates.
The measured source is pinned to commit `618fd6590995971981f0e12b4af74bdfda83168f`;
the later source change only combines identical condition branches for the
repository's Rust Clippy gate. The independent replay hashes the original
committed source through Git, preserving the executed version's identity.

## K128 split-index control

[`run.py`](run.py) ran three alternating five-byte/four-byte ID31 pairs on
the same committed [public point](target.jsonl), `K128`, split at `K64`,
four root threads and dual `wide3` Bloom filters. The exact source SHA-256
was `19095c16bfb40751a8c7690ebfde4093d1651d4cd402526bac4b9a9cc8d7c160`;
the executed release binary SHA-256 was
`afea030a9f44a698e13f34f4554276b5b7f27c225f621f5520b549bb14b0763c`.
The [raw stage receipts](runs/k128_split) record commands, flags, hashes,
process statuses and per-run outputs. Their selector is
`PASS_SPLIT_STAGE_CONTROL`: both layouts built the same 21,248 usable base
points, 680,000 states, 1,359,488 exact root entries and table split.

| Layout | Table bytes | Index-build ms, three repetitions | Median index-build ms |
| --- | ---: | --- | ---: |
| Five-byte ID31 | 13,107,200 | 386.868, 517.238, 502.690 | 502.690 |
| Four-byte ID31 | 10,485,760 | 878.344, 776.665, 1140.786 | 878.344 |

The four-byte slots used 20% less table storage in this control. Its
observed index-build median was 1.75 times the five-byte median; the small
local run is a diagnostic, not a K5500 cost estimate or Tier 1 ranking.

The [K128 full-process attempt](runs/k128_full) retained `TIMEOUT` at its
180-second cap, before a rank or target receipt.

## K1107 full-process correctness control

[`full_k1107.py`](full_k1107.py) ran the four-byte ID31 split on the
previously solved [public fixture zero](target_k1107.jsonl), with a `K553`
core, four root and rank threads, two target threads, and a 900-second cap.
The [raw receipt](runs/k1107_full) records the exact flags, executable,
source, base dump, rank and target. [`verify_k1107.py`](verify_k1107.py)
independently checked the binary and input hashes, exact previously replayed
K1107 base dump, rank, four-point relation, scalar multiplication and
exclusive target-phase sum. All 16 checks passed with
`PASS_INDEPENDENT_FULL_REPLAY`.

| Quantity | Measured value |
| --- | ---: |
| Usable base points / folded columns / final rank | 183,762 / 1,107 / 1,107 |
| Symmetric states | 50,856,687 |
| Four-byte root tables / dual Bloom filters | 671,088,640 / 167,772,160 bytes |
| Peak child RSS from wrapper | 1,459,683,328 bytes |
| Full local process wall | 674.839 seconds |
| Setup / rank | 458,658.794 / 214,841.231 ms |
| Exclusive target interval / probes | 1,027.186 ms / 940,621 |
| Independently replayed scalar | 3,640,978,018,804,724 |

The base dump SHA-256 is
`08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc`,
byte-identical to the previously replayed K1107 dump. This run establishes
full-process correctness of the four-byte ID31 split on that point. The
shared macOS host had substantial swap activity; its wall intervals are
diagnostics and do not rank this layout against the five-byte control.

## Next factor-base gate

The K5500 base has 913,000 independently replayed usable subgroup points.
A larger-memory worker must measure the four-byte K5500 root index and peak
RSS, complete rank and target recovery, and independently replay the answer.
Only then can this exact layout enter the sealed same-point Tier 1 IC/rho
comparison against the other N83 bases. A two-bit side fingerprint is a
possible next index experiment if its extra 0.75 GiB at K5500 reduces root
reconstructions enough to improve the full process.
