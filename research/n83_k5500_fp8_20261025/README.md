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

The capped [ABBA runner](run_abba.py) records exact source, binary, fixture,
flags, host, per-process status, output digests, and complete stage summaries.
For example, from this repository run:

```sh
python3 research/n83_k5500_fp8_20261025/run_abba.py \
  --binary /absolute/path/to/koblitz_orbit_dlp_fast \
  --output /absolute/path/to/new-output-directory \
  --k 1024 --split-at 512 --cap-seconds 300
```

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

The [K128](k128_abba.json) and [K1024](k1024_abba.json) receipts use the
same source and binary hashes and the same ABBA order. All four K1024 runs
agree on the base hash, 43,516,416 regular states, 87,028,692 root entries,
and complete state, primary-table, and overflow-table digests. The sidecar
adds 167,772,160 bytes. K1024 baseline builds took 26.890 and 26.419
seconds; filtered builds took 17.623 and 17.809 seconds, a geometric-mean
ratio of 1.505. K128 likewise preserved exact digests and measured a 1.831
ratio at 680,000 states. All three local sizes ran on a shared Darwin ARM64
host with fourteen exposed logical CPUs; they are index-stage diagnostics.

The [K2048 control](k2048_abba.json) extends exact digest parity to
174,064,640 regular states and 348,120,365 root entries. Its sidecars add
671,088,640 bytes. The two filtered builds took 78.218 and 78.878 seconds;
baseline builds took 117.335 and 166.582 seconds. The closing baseline was
42% slower than the opening baseline while other host jobs were active.
Keep both baselines and the resulting timing spread in the receipt; this
comparison needs a calibrated Tier 1 repeat before using its timing ratio.

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

The [bounded Pod runner](pod_run.py) uses the approved one-A40 allocation
contract in the split-index controller: at most $0.70/hour, 9,000 seconds,
and $2.25 including storage reserve. It verifies the allocation, source
bundle, Rust source and public fixture before building. It requires at least
40 GiB effective memory and four effective CPUs, then pins each index child
to the same four CPUs. After a twenty-minute build cap it attempts baseline,
filtered, filtered, baseline, each with at most forty minutes and a
twenty-minute provider deadline margin. A shorter admissible pair remains
explicitly partial. Every arm retains its raw output, cgroup and process CPU
counters, cap, status and parsed stage summary. Exact state and table digest
parity is checked before returning a passing pair or ABBA status.

```sh
python3 -m research.n83_k5500_fp8_20261025.pod_run \
  --allocation /workspace/n83_k5500/allocation.json \
  --bundle /workspace/n83_k5500/source.tar.gz \
  --bundle-sha256 EXPECTED_SOURCE_BUNDLE_SHA256 \
  --output /workspace/n83_k5500/fp8_results \
  --archive /workspace/n83_k5500/fp8_results.tar.gz
```

Copy the raw archive and SHA-256 sidecar back before the independent Pod
controller verifies deletion. A configured RunPod API key is required for
dispatch; the local source and index controls above do not use one.
