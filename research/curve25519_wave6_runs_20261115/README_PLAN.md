# Wave-6 leads 1 to 4, run in parallel (2026-11-15)

Owner: "1-4 all parallel". Four frozen protocols, one per lead, each in its own directory. One new build of the
repository's binaries serves all four (its sha256 is recorded at each launch); the exponent studies' frozen binary
dc2c30a4 stays the reference for chain cells, and L3 re-runs two chain reference cells with the new binary to check
that field_ops are unchanged. Host: the shared 4-core, 15 GB container; one core per lead; the F4 leads run under
`ulimit -v 6000000` (6 GB each) so that two can run at once. No EKS, Conductor, S3 checkpoint, benchrec record or
ecdb lookup is reachable from this runtime (disclosed, as in the exponent studies). Every number is [pcc:unverified].
