# N83 root4 RunPod result: independently replayed first stratum

The first eight-point RunPod stratum passed an independent local replay against
all 1,384 bytes-bound raw result files. Its Tier 1 topology, resource,
calibration and A/A gates passed. The 333,661,543-byte original raw archive
has SHA-256 `d06daffcbdb35d4ba3fc74c94d6eb3c4fe206d5b428f53a3673fce9f46866aca`.
`raw_s0_dedup.tar.gz` preserves those files with three distinct factor-base
dumps stored once; `verify_archive.py OUTPUT_DIR` restores and hashes every
path. The original archive, provider logs, build receipts, source and Pod
allocation receipts, local replay stdout, empty stderr and exit status are
bound by `raw_s0_dedup_manifest.json`, `raw_s0_dedup_receipt.json` and
`status.json`.

The second RunPod producer and its on-Pod verifier completed all eight points
with exit status 0 and an admitted Tier 1 analysis. Its raw archive manifest
has SHA-256 `facd358a5081e9f9d2e1bdd8dacf6136918e2269a4a359e205f4f70c37facf72`;
the first 48 MiB chunk was transferred and hash-checked. Subsequent SSH
gateway sessions failed before command execution when the provider container
runtime could not allocate a PTY (`reopen ptmx ... no space left on device`).
The Pod reported 26 GiB free in its overlay filesystem before that gateway
failure. `support_receipts_s1.tar.gz` preserves the producer and provider
verifier logs, exits, launch command and archive digest. A complete local
replay of stratum 1 awaits the remaining raw bytes.

The aggregate status is `INFRASTRUCTURE_TRANSFER_INCOMPLETE`; online and cold
selected candidate IDs remain null. The separately dispatched corrected
[GitHub reserve run 39171995000](https://github.com/aburan28/crypto/actions/runs/39171995000)
provides a second two-host route to the same frozen high-K comparison.
