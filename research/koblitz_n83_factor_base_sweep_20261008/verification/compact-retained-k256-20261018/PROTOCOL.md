# Retained primary N83 K256 compact-root capacity probe

This rung extends the independently audited K64 run to the frozen `a=0`
K256 public-x-hash factor base at seed 2026100801. Its exact S3 compressed
object BLAKE3 is
`0b06b75e26cff57a62c28c2502c85f3fc3f1a5027e5620d90969ecae0947b045`.
Preparation requires the original manifest, replay and upload receipts plus
a fresh encrypted S3 GET and compressed-byte hash check. The worker then
imports and validates the point records independently.

Use the same static Linux worker binary and immutable Docker image as the
audited K64 rung. The supervisor requires a clean source commit and captures
its source snapshot. Run one public-point query, unordered pair mode, exactly
`83 * 256 * 257 / 2 = 2730368` candidate states, one CPU, a hard 4-GiB
memory cap with zero swap, no network, and a 1800-second wall cap. Retain raw
stdout, stderr, config, source, worker, outer receipt, and a live sanitized
container inspection. Independently audit both a genuine outcome and a
mutated copy.

This is an index-capacity and one-query correctness check. A miss does not
measure relation yield, useful rank, linear algebra, or complete cold IC
runtime. A cap remains censored. Charge build and preparation separately,
and charge the full search envelope for the attempted worker.
