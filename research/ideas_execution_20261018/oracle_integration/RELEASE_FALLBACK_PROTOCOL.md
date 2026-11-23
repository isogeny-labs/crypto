# Prospective native release fallback for the factorial diagnostic

Frozen before plan/trial execution with this runtime. The current checkout's
ecbench CLI does not build because of its missing fbm import; that failure
and the prepared patch remain evidence. No existing file is edited here.

Use the repository owner's published Linux x86_64 release v0.1.2538, source
commit `b1bf5b8935f68257dfdd3fc542cb2fef41db741c`, released2026-10-17.
Downloaded archive `crypto-v0.1.2538-x86_64-unknown-linux-gnu.tar.gz` passes
the publisher's SHA256SUMS check. The ecbench binary SHA256 is
`62db5721a5c4ec528a8685aaccd96fbbfe74b61e06a022569f8b1a85b4f182f7`.
Asset list, release tag ref and publisher checksum file are retained in
`runs/`. The release binary resides under `/tmp/oracle-release-v0.1.2538/`.

The exact `factorial-spec.json` is unchanged: four sizes and eight targets
with seven arms. Launch the existing native harness with that binary;
retain its resolved plan, source identity, run records, native audit and
comparisons. The release source is different from the current checkout.
These results cannot be silently attributed to the current repaired build.
They remain an additional toy correctness/counts diagnostic with an
unpriced enumeration oracle, not a full-cost or deployed-curve claim.

The boundary, counters, all-arm pricing sensitivity and no-timing-claim rules
in FACTORIAL_PROTOCOL.md remain. If the release cannot represent the exact
spec, retain its refusal and do not weaken the spec. A trial or audit failure
is retained. Planned/current-checkout execution remains separately open.
