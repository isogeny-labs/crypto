# SAT search integration source review

The terminal-report and dedicated qualification proposals passed independent source review and compose against the tested original-owner source. Their combined Rust source parses. No typecheck, native execution, qualification result or CLI activation is claimed by this packet.

Qualification fixes preserve the original 128 MiB trace ceiling: reserve 100 MiB for bounded journals/logs/metadata before producer capture, leaving at most 28 MiB for capture. Borrowed serialization avoids full map/model clones. Recursive inventory includes nested originals. Candidate verification outcomes are checkpointed before canonical model hashing; hash failures remain separate. All four fixed genuine binary/Koblitz both-twist candidates remain required, with no fallback.

The producer terminal prepays its bounded publication through the original owner. Actual acknowledgment survives post-commit failure. Stopped owners retain their local report and earlier partial capture; unavailable durable terminal custody is explicit. A checked relation or acknowledged terminal alone is not export authority.

The S3 archive preserves the original proposals, V1/V2/V3 qualification revisions, findings and corrections, reviewed composition and root-only compile-command probe. Its receipt confirms fresh download and contained hashes. Download the receipt URI, verify byte count and SHA-256, extract into a fresh directory and run `sha256sum -c SHA256SUMS`.

Original-owner implementation checkpoint `7ec4f39d082c` remains local pending the running full release-library gate. The subsequent proposals are external and unapplied to that running checkout. Central search integration and explicit CLI/report/projection routing remain separate ongoing work; corrected source review cannot replace genuine runtime, modified-capture and post-ack failure controls.
