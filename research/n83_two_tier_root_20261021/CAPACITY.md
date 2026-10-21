# N83 split-index capacity at the current 30-bit state-ID limit

**Calculated layout:** K5086 is the largest orbit-column count admitted by
the current 30-bit root-state ID. A K4549 primary split and K537 extension
fit the five-byte exact-root tables into 2^31 and 2^29 slots, respectively:
12.5 GiB of table storage. With separate `wide3` Bloom filters
(`KIC_INDEXED5_SPLIT_SHARED_BLOOM=0`), the two filters allocate 2.5 GiB
combined and the 11-byte root-state array allocates 11.00 GiB. These
arrays total 27,914,621,607 bytes (26.00 GiB). One shared union Bloom would
allocate 4 GiB and raise the sum to 27.50 GiB. Other buffers, base
construction, rank state, allocator overhead, and peak RSS are outside this
calculation. No K5086 index or target run is reported here.

The [Rust index builder](../../examples/koblitz_orbit_dlp_fast.rs) computes
the symmetric state count as

`S(K) = 83 K (K-1) / 2 + 42 K`.

It requests `q(S) = floor((5 S + 3)/4)` entries per load80 table, then
allocates the next power of two at or above `2 q(S)` five-byte slots. Each
`wide3` Bloom allocates the next power of two at or above `16 q(S)` bits;
the split can use one per-table filter or one full-base union filter. The
`RootOnly11` vector reserves 11 bytes for each state. The source
has the split routing and capacity assertions; the Lean module checks the
numeric implications of these formulas.

| Quantity | Exact value | Status |
| --- | ---: | --- |
| `S(5086)` | 1,073,499,477 | Lean-checked arithmetic |
| 30-bit state-ID bound | 1,073,741,824 | `S(5086)` below; `S(5087)` above |
| `S(4549)` primary | 858,778,416 | formula |
| extension states | 214,721,061 | formula |
| primary `q` | 1,073,473,020 | at most 2^30 requests |
| extension `q` | 268,401,327 | at most 2^28 requests |
| exact table slots | 2^31 + 2^29 | 13,421,772,800 bytes |
| separate `wide3` Blooms | 2^34 + 2^32 bits | 2,684,354,560 bytes |
| union `wide3` Bloom | 2^35 bits | 4,294,967,296 bytes |
| 11-byte state array | 1,073,499,477 entries | 11,808,494,247 bytes |
| separate-filter three-array sum | state + table + dual Bloom | 27,914,621,607 bytes (26.00 GiB) |
| union-filter three-array sum | state + table + shared Bloom | 29,525,234,343 bytes (27.50 GiB) |
| projected point count under 166-point orbit expansion | 166 × 5086 = 844,276 | base enumeration pending |

Among split points with a primary request cap of 2^30 and extension request
cap of 2^28, K4549 is unique: K4550 exceeds the primary cap and K4548
exceeds the extension cap, with monotonicity handling all points farther
away. The reversed orientation K2274 also gives a 2^29 + 2^31 slot sum, but
would place most states in the extension table, which the current lookup
probes second. Relative to splitting K5086 at K4300, K4549 saves 2.5 GiB
of five-byte table storage; relative to one monolithic 2^32-slot table, it
saves 7.5 GiB. The separate filters save 1.5 GiB against the union filter at
K5086 because that full-base request crosses a Bloom power-of-two boundary.
The K4405 shared-filter saving therefore does not extrapolate to K5086.
Lookup cost and natural-target yield have not been measured.

## Proof map and verification

The [Lean source](Capacity.lean) is pinned by [lean-toolchain](lean-toolchain)
to Lean 4.34.1. `lean Capacity.lean` passed; the exact version and all
`#print axioms` results are in [Capacity.check.txt](Capacity.check.txt).
`lean4checker --fresh` was unavailable on this host. The Lean statements use
natural-number arithmetic; `states_monotone` and `request_monotone` preserve
their `a ≤ b` hypotheses. No result uses `sorry`, `admit`, custom axioms,
kernel bypasses, or native evaluation. The kernel reports only standard
`propext`, `Classical.choice`, and `Quot.sound` for the structural proofs; the fixed numerical
lemmas require no axioms.

| Result | Lean declaration | Exact scope |
| --- | --- | --- |
| State count is monotone | `N83SplitCapacity.states_monotone` | natural `a ≤ b` |
| ID31 slot has a unique state/selector/fingerprint decoding and fits five bytes away from the zero sentinel | `id31_slot_roundtrip`, `id31_slot_nonzero_five_bytes` | `state + 1 < 2^31`, `selector < 2`, `fingerprint < 2^8` for byte bound |
| K5086 fits and every `K ≥ 5087` crosses the 30-bit bound | `k5086_state_fits`, `k5087_state_crosses`, `k5086_max` | this `S(K)` formula |
| K4549 is the only split under the oriented request caps | `k5086_only_core_split` plus boundary lemmas | `K < 5086`, primary `q ≤ 2^30`, extension `q ≤ 2^28` |
| Table, Bloom, state, and point-count arithmetic | `k5086_table_bytes`, `k5086_wide3_bloom_bucket`, `k5086_primary_wide3_bloom_bucket`, `k5086_extension_wide3_bloom_bucket`, `k5086_state_bytes`, `k5086_three_array_bytes`, `k5086_dual_bloom_bytes`, `k5086_dual_three_array_bytes`, `k5086_point_count_formula` | layout formulas and 166-point expansion assumption |

The formalization of the arithmetic and oriented split implication is
complete. The bridge from `S(K)`, `q(S)`, and the power-of-two sizing rules
to all Rust execution paths is checked by source inspection and K128 tests,
but has not been formalized in Lean; K5086 construction and subgroup point
enumeration are open execution checks. The next run is an exact K5086/K4549
index construction with base hash, state count, table/Bloom allocation, and
full relation/scalar replay before comparing online time. Until then this is
a capacity proposal with `candidate_id: null`.

## Opt-in 31-bit state IDs and K5500 capacity

The new `KIC_INDEXED5_ID31=1` slot format uses 31 bits for a state ID, one
bit for the root selector, and eight fingerprint bits in the same five-byte
slot. Exact root reconstruction still decides collisions. It is mutually
exclusive with `KIC_INDEXED5_ID30=1`; the latter remains the current K5086
protocol format. `RootOnly11`, symmetric S3 traversal, indexed five-byte
load75/load80 sizing, and the root-index flag are required. The slot
encoding and lookup were checked by the dedicated Rust test, including the
largest encodable ID and an equal-slot/equal-fingerprint collision. The
ID31 implementation source SHA-256 is
`6876cfa98aac31b007a91437c2be9afc0d7c3532d23f0374acd8a780cbce5b13`.
The [K128 stage receipt](runs/k128_id31/result.json) passed monolithic and
split construction with four root threads and the same base/state counts as
the ID30 control; it does not measure K5500 or a target solve.

For the same split-table and separate-`wide3`-Bloom sizing rule, K5500 with
primary K4549 has the following **calculated** capacity. It is a new
proposal, not a completed factor-base comparison or an amendment to the
sealed K5086 screen.

| Quantity | Exact value | Status |
| --- | ---: | --- |
| `S(5500)` | 1,255,377,750 | Lean-checked arithmetic |
| ID31 limit | `S(7193) < 2^31 ≤ S(7194)` | Lean-checked formula bound |
| K4549 primary, K951 extension | 858,778,416 and 396,599,334 states | formula |
| load80 requests | 1,073,473,020 and 495,749,168 | formula |
| exact table slots | 2^31 + 2^30 | 15 GiB of five-byte slots |
| separate `wide3` Blooms | 2^34 + 2^33 bits | 3 GiB |
| 11-byte state array | 1,255,377,750 entries | 13,809,155,250 bytes |
| three-array sum | state + two tables + two Blooms | 33,136,508,082 bytes (30.86 GiB) |
| projected point count under 166-point expansion | 913,000 | actual enumeration pending |

The 31-bit format permits K up to 7193 by its state-ID bound alone.
Memory, index-construction cost, rank completion, natural-target yield,
and online lookup cost can limit the useful base much earlier. The K5500
three-array sum leaves about 9.14 GiB within the frozen 40 GiB sampled
child-RSS cap, before other buffers and allocator overhead; it requires an
actual peak-memory measurement. K5800 already needs 38.30 GiB for these
arrays, leaving too little headroom for that pilot envelope. The K5500
proposal retains `candidate_id: null` and measured costs `null` until its
base is enumerated, subgroup checked, hashed, and paired on the sealed
same-point workload.

The same pinned Lean module proves the ID31 slot decode and five-byte
nonzero bound, and `k7193_id31_max` from monotonicity,
`k5500_core_request`, `k5500_extension_request`, the K5500 root bound,
point-count multiplication, and byte sum. `Capacity.check.txt` records
the successful Lean 4.34.1 build and `#print axioms` output: the fixed
numeric lemmas are axiom-free; the structural slot lemmas use the standard
`propext`, `Classical.choice`, and `Quot.sound` foundations. The Rust-to-Lean
source correspondence remains open.
