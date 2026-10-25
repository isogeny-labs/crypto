# N83 K5500 relative-ID four-byte split index

The relative-ID split places the K5500 root index in **25.61 GiB of principal
allocations**, 4.25 GiB below the monolithic four-byte ID31 layout. At the
K4549 split, the primary table uses 2^31 four-byte slots and the extension
uses 2^30; the eleven-byte state array holds 1,255,377,750 states. Separate
`compact4` Bloom filters occupy 0.75 GiB together. These allocations sum to
27,499,363,506 bytes. The full process peak, index construction time, rank,
and one-target online time require measurement.

Each table encodes a state ID relative to its own starting state, a root
selector, and one fingerprint bit in a nonzero `u32`. Routing cuts the
deterministic symmetric state order at ordinal `S(4549)`: ordinals
`[0,858778416)` enter the primary table, and the remaining ordinals enter
the extension. The primary base is zero; the extension base is 858,778,416. Lookup
decodes an absolute state ID, reconstructs the complete canonical root, and
compares it exactly. The primary table is probed first, preserving the global
first-witness rule across the split. `KIC_INDEXED4_SPLIT30=1` and
`KIC_INDEXED4_SPLIT_AT=4549` select this opt-in mode with
`KIC_INDEXED_ROOT4=1`, `KIC_ROOT_ONLY11=1`,
`KIC_SYMMETRIC_S3=1`, and `KIC_PACKED_ROOT_TABLE=1`. The capacity gate
requires each relative ID to fit 30 bits, the primary load80 request to fit
2^30 entries, and the extension request to fit 2^29 entries.

The exact K5500 base has 913,000 subgroup-usable points and 5,500 folded
columns on `EC1N83Ckb1a1haacb69ee9f37`. Its ordered dump SHA-256 is
`823782912c75d782cdd7e6c9074c94282e28a686b1dd98ea978a367514d3acdd`
and its producer BLAKE3 base hash is
`0789049953a67b51dfe23f6fa2788d11e0bb4b35d52180fc038a3ffd2c0efb5f`.
The established independent base replay is described in
[`K5500_CURRENT_SOURCE.md`](../n83_highk_beyond4405_20261021/K5500_CURRENT_SOURCE.md).
Keep `candidate_id: null` until the exact full pipeline and its measurement
manifest are frozen.

## Capacity proof

The symmetric state formula is `S(K)=83 K(K-1)/2+42 K`. The first `S(4549)` states number
858,778,416 states, and its K5500 extension has 396,599,334. Both relative
counts fit 30 bits. The two-root upper loads are 79.98% and 73.87% of their
tables. With a 2^31-slot primary and this load80 request rule, K4549 is the
largest possible first state segment. This maximizes the number of states routed to
the first table within these fixed slot buckets.

| Result | Lean declaration | Exact hypotheses | Status |
| --- | --- | --- | --- |
| State and request monotonicity | `states_monotone`, `request_monotone` | Natural inputs with `a ≤ b` | Checked |
| Core, full, and extension counts | `core_states`, `full_states`, `extension_states` | `S(K)` formula at K4549 and K5500 | Checked |
| Relative ID and slot-bucket bounds | `relative_ids_fit`, `split_slot_buckets` | `S(K)` and `request(s)=(5s+3)/4` | Checked |
| Maximal first state segment under the primary bucket | `k4549_max_primary` | Natural K with `request(S(K)) ≤ 2^30` | Checked |
| Two-root load bound | `split_load80` | At most two roots per state; 2^31 and 2^30 table slots | Checked |
| Exact global ordinal recovery from either segment | `ordinal_split_roundtrip` | `cut ≤ total`, `global < total` | Checked |
| Exact slot decoding and unique witness | `relative_witness_roundtrip`, `relative_witness_injective` | Local ID below `2^30−1`; selector and fingerprint below 2 | Checked |
| Principal bytes and saving | `k5500_split_principal_bytes`, `split_saves_425_gib` | Four-byte slots, eleven-byte states, compact4 Bloom buckets | Checked |

The [Lean source](Capacity.lean) pins Lean 4.35.0 in
[`lean-toolchain`](lean-toolchain). Run `lean Capacity.lean`; the version,
build output and `#print axioms` audit are in [`lean-check.log`](lean-check.log).
No declaration uses `sorry`, `admit`, custom claim axioms, or native
evaluation. The structural lemmas use the standard foundations reported in
the log. `lean4checker --fresh` is unavailable on this host. The proof uses
the state and request formulas as explicit definitions; the correspondence
between those formulas and the compiled Rust allocation path is checked by
source review and the K128 receipt and remains an open Lean bridge.

## K128 implementation control

The release example suite passed all 26 tests. An index-only K128 control
built 680,000 symmetric states and 1,359,488 distinct root entries under
both the new four-byte ordinal split and the existing five-byte pair-label
split. Their base hash and complete eleven-byte state digest agree. The
four-byte control allocated 10,485,760 table bytes; the five-byte control
allocated 13,107,200. Their primary and extension entry counts differ
because the split routing policies differ, while their total count agrees.
The exact outputs, source and binary hashes are in
[`k128_control.json`](k128_control.json). A separate one-target K128
process reached its 180-second cap before emitting a result; its empty
output and cap are retained in that receipt. These local controls establish
index construction and sampled exact-root lookup behavior, with full K5500
peak RSS and target measurements as the next gates.

## Next measurement

Use `run_index_stage.py --layout split` from the
[`four-byte index study`](../n83_k5500_fourbyte_20261025/README.md) to
run a bounded full K5500 index build; use `--layout monolithic` on the same
source and host for its paired layout control. Check exact
base hash, all 1,255,377,750 states, table entry counts, representative root
lookups, digest receipts, memory peak, and index construction time. Then
complete rank and independently replayed one-target IC/rho pairs at Tier 1,
with the same point and resource envelope. Preserve both layouts as separate
candidate identities and retain all capped or incomplete runs.

The prepared RunPod diagnostic requests one Community Cloud A40 Pod with at
least 48 GB of provider-reported system RAM, four vCPUs, a 60 GB container
disk, and a maximum 2.5-hour provider lifetime. Its admitted quote is at most
$0.70/hour: up to $1.75 of compute plus a $0.50 storage and billing reserve,
for a $2.25 allocation ceiling. [`runpod_dispatch.py`](runpod_dispatch.py)
prints the exact request with `--dry-run`; live dispatch reads the API key
from the environment or a private local CLI config and runs the established
REST custody guard until provider absence is verified. The local guard
requests deletion ten minutes before the allocation deadline. Dispatch
requires the owner's separate cost approval and configured replacement key.
The Pod must report its actual cgroup memory and CPU limits before either
layout runs. Keep the split first and the monolithic layout second, with both
stage caps fitting the remaining provider deadline; retain each incomplete
stage and the provider deletion receipt.
