# N83 K5500 exact four-byte root index

The opt-in `KIC_INDEXED4_FIXED32=1` mode stores an exact root witness in one
`u32` slot: 31 bits for the symmetric S3 state index plus one root selector.
Zero marks an empty slot. Lookup reconstructs and compares the complete
canonical 83-bit root, so discarding the two-bit slot fingerprint affects
probe cost, not the set of accepted roots or the first-witness rule. The mode
requires `KIC_INDEXED_ROOT4=1`, `KIC_ROOT_ONLY11=1`,
`KIC_SYMMETRIC_S3=1`, and `KIC_PACKED_ROOT_TABLE=1`. It rejects a state bound
that does not fit 31 bits or whose two-root upper occupancy exceeds 75% of
2^32 slots. Existing four- and five-byte layouts retain their encodings.

The independently replayed current-source K5500 base has 913,000 distinct
subgroup-usable points in 5,500 folded signed-Frobenius columns on
`EC1N83Ckb1a1haacb69ee9f37`; see
[`K5500_CURRENT_SOURCE.md`](../n83_highk_beyond4405_20261021/K5500_CURRENT_SOURCE.md).
Its ordered dump SHA-256 is
`823782912c75d782cdd7e6c9074c94282e28a686b1dd98ea978a367514d3acdd`,
and its producer BLAKE3 base hash is
`0789049953a67b51dfe23f6fa2788d11e0bb4b35d52180fc038a3ffd2c0efb5f`.
The new index mode has not yet built the full index, solved its log matrix,
or produced a same-point one-target IC/rho pair. Keep `candidate_id: null` and
measured online and cold costs unknown until the complete pipeline and
independent replay are sealed and run.

For K5500, the symmetric state bound is 1,255,377,750. Two roots per state
would occupy at most 58.4581% of 2^32 slots. The four-byte table reserves
17,179,869,184 bytes, the eleven-byte state array has a 13,809,155,250-byte
bound, and the `KIC_ROOT_BLOOM_PROFILE=compact4` Bloom allocation is
1,073,741,824 bytes. These
three principal allocations total 32,062,766,258 bytes (29.86 GiB).
Allocator overhead, factor-base storage, rank, process code and host memory
need a measured peak before a 32 GiB Pod can be admitted. This is a capacity
calculation, not a full-process RSS measurement or a speed result.

## Proof map

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| Symmetric K5500 state count | `k5500_state_count` | N83 symmetric S3 formula, K=5500 | Checked |
| State ID fits 31 bits | `k5500_id31` | Same formula and K | Checked |
| Two-root load below 75% | `k5500_load75` | 2^32 slots; at most two keys per state | Checked |
| Nonzero, bounded, exact slot decoding | `witness_word_exact` | state index below K5500 bound; selector below 2 | Checked |
| Unique encoded witness | `witness_word_injective` | both selectors below 2 | Checked |
| Principal allocation arithmetic | `k5500_principal_bytes` | four-byte slots, eleven-byte states, compact4 Bloom | Checked |

`lean-toolchain` pins Lean 4.35.0. Run `cd
research/n83_k5500_fourbyte_20261025 && lean Capacity.lean`; the full output
is in `lean-check.log`. The declarations contain no `sorry`, custom claim
axioms or native-evaluation shortcuts. `#print axioms` reports no axioms for
the four arithmetic results and `propext`, `Classical.choice`, `Quot.sound`
for the two structural decoding results. `lean4checker --fresh` was not
available on this host.

The release example test suite passes all 25 tests. This includes exact
first-witness behavior at the highest encodable ID and comparison with the
existing packed table on actual N83 S3 roots. The test command is:

```sh
CARGO_TARGET_DIR=/path/to/build-cache cargo test --offline --locked --release \
  --example koblitz_orbit_dlp_fast
```

`run_index_stage.py` is the bounded full K5500 index diagnostic. It freezes
the source SHA-256, public fixture, flag set, 5,500-column base hash, expected
state count, slot layout and allocation size; it writes launch and completion
receipts even for timeout or process failure. Build the example on the target
host and pass its binary, a new output directory and a cap that fits the
approved host budget:

```sh
CARGO_TARGET_DIR=/path/to/build-cache cargo build --offline --locked --release \
  --example koblitz_orbit_dlp_fast
python3 -m research.n83_k5500_fourbyte_20261025.run_index_stage \
  --binary /path/to/build-cache/release/examples/koblitz_orbit_dlp_fast \
  --output /path/to/new-output --cap-seconds 5400
```

The index receipt records peak child RSS and hashes the exact state and slot
arrays. A complete index-stage run and independent root replay are still
needed before this capacity option can be promoted.

After full base, rank, relation and scalar replay, freeze the new candidate
manifest and compare it on fresh same-point single-target Tier 1 workloads
against the admitted nearby and K4405–K4549 finalists. Record every capped,
OOM, noisy and incomplete row. A Pod allocation requires an approved cost
envelope and independent deletion receipts.
