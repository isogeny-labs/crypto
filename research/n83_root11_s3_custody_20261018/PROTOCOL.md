# Frozen custody of two N83 root11 factor bases

The [plan](plan.json) pins the two exact public point-defined factor-base
dumps from the four-stratum [Tier 1 provider run
39061060775](https://github.com/aburan28/crypto/actions/runs/39061060775).
This is the degree-83 **a=1 diagnostic curve with a 53-bit subgroup**;
its bases and timings are not interchangeable with the primary a=0 N83
subgroup in the K600 SAT sweep. Preserve both K2816 and K3072 neutrally.
The subsequent online and cold selections are separate questions and do not
change these object identities.

Before writing to S3, bind each of the four provider ZIP files by its frozen
SHA-256 and require all 32 byte copies of each base dump to agree with the
plan. Check the complete JSON object, curve parameters, point count and
coordinate bounds. Deterministically gzip one exact copy per arm with an
empty filename and zero modification time. Address each object by BLAKE3 of
the compressed bytes under the approved
`s3://crypto-autoresearcher/factor-bases/icv1/etc/` prefix. Never overwrite an
existing object. Capture PUT and subsequent fresh GET receipts; independently
check compressed SHA-256/BLAKE3, raw SHA-256, sizes, metadata and single-part
ETag. An absent provider ZIP, mismatched copy, changed point row, failed PUT,
or failed fresh GET is a custody failure, not an alternate factor base.

This round measures **storage identity only**. It does not rerun relation
collection, promote an online selector, charge a new IC runtime, or choose a
minimum-total-runtime base. Keep `selected_best_total_runtime` null. The
provider holdout has its own independent arithmetic, scalar, rank, resource
and rho verification; this custody check binds the exact factor-base bytes
to that provider run without replacing those gates.
