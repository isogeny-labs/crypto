# N83 exact byte-fingerprint filter for the split root index

The optional `KIC_INDEXED4_FINGERPRINT8=1` mode adds one fingerprint byte per
slot to each four-byte relative-ID root table. A candidate slot is rejected
when its stored byte differs from the query byte. Every surviving slot still
reconstructs and compares the complete 83-bit root. Insertion order, probing,
and the first exact witness therefore follow the existing split-index path.
The mode requires `KIC_INDEXED4_SPLIT30=1`; the unsplit four-byte layout keeps
its previous representation.

The byte is the low eight bits of the fingerprint already computed while
building or querying the table. The packed word retains its fingerprint bit.
Empty slots remain identified by their zero packed word, so a stored zero
fingerprint byte is valid. The test
`indexed_four_byte_byte_fingerprint_filters_without_changing_exact_keys`
checks exact-key and first-witness parity, high absolute state IDs, and a
collision with the same slot and packed fingerprint bit but a different byte.
The collision avoids full-root reconstruction; lookup still returns both
exact witnesses.

## Local index controls

The [K512 ABBA receipt](k512_abba.json) uses the formatted source SHA-256
`05b64717b10e17c8cd09e8a6823e6efec600ac4b53c8a0e8e0476771c702cb7b`
and release binary SHA-256
`a768b71f8059f0cf9049213ebc5bd2ed7ff9eccc3ab4947cff822093b58ef706`.
It runs baseline, byte-filter, byte-filter, baseline in separate processes
with four root-construction threads and the same K512 base, 10,879,232
regular states, and 21,756,405 root entries. All four complete state,
primary-table, and overflow-table digests match. The filter adds 41,943,040
bytes of sidecar storage at this size. Index-build intervals were 10.792 and
6.072 seconds for baseline, versus 7.443 and 4.853 seconds with the filter:
the ratio of geometric means is 1.347. These local shared-host timings are
exploratory; no Tier 1 noise or same-target gate was applied.

The [K5500 split launch](k5500_split_launch.json) and
[completion receipt](k5500_split_receipt.json) document the immediately
preceding baseline build on the source branch. It completed 1,255,377,750
states and 2,510,698,617 root entries within a 2,400-second cap, with a
19,145,818,112-byte peak child RSS and a 2,297-second index interval. That
run used one root-construction thread and experienced host swapping, so its
time is a capacity diagnostic. The receipt fixes the full base and table
digests for the upcoming K5500 sidecar comparison.

For K5500, the filter's two sidecars add 3,221,225,472 bytes to the
27,499,363,506-byte split-index principal allocation. The resulting
30,720,588,978-byte principal total is a calculated allocation requirement;
full process peak memory and build time remain to be measured on the Pod.

## Next measurement

Run baseline and filtered K5500 builds on the same Tier 1 Pod with identical
source, base, CPU allocation, and caps. Preserve cgroup CPU and memory
receipts, exact table digests, failed or capped runs, and the provider deletion
receipt. Then include the winning index in one-target IC and same-point rho
comparisons with target-independent setup recorded separately.
