# Frozen S3 custody for the N83 bridge bases

The [plan](plan.json) freezes the exact K1107 and K2304 point-defined
factor bases used by the 32-point native bridge. The source is the first
completed stratum of [run 39096257170](https://github.com/aburan28/crypto/actions/runs/39096257170),
with its GitHub job and artifact metadata retained in [receipts](receipts/).
This is the degree-83 `a=1` diagnostic curve with a 53-bit subgroup. Its
factor bases and runtimes are separate from the primary `a=0` N83 sweep.

Before storage, require the downloaded provider ZIP to match the GitHub
artifact digest, size, run, source commit, branch, completed job, and pinned
candidate manifests. All eight point-specific byte copies of each factor-base
dump must have the same frozen SHA-256. Check the curve, subgroup, orbit and
point counts, coordinate bounds and uniqueness. Gzip one exact copy per arm
with an empty filename and zero modification time; name the S3 object by the
BLAKE3 of those compressed bytes. Store under the approved
`s3://crypto-autoresearcher/factor-bases/icv1/etc/` prefix with
`If-None-Match: *`. Preserve PUT responses, fresh GET bytes and metadata,
the prefix inventory and a changed-GET rejection control. A failed source
check, S3 PUT, GET, or independent replay is a custody failure.

The independent [verifier](verify.py) recomputes provider member hashes,
candidate-manifest identity, compressed SHA-256/BLAKE3, decompressed SHA-256,
S3 sizes, metadata and single-part ETags. Its storage result does not select
the faster base. The bridge's full matched panel, independent arithmetic
replay, resource admission and same-point rho results have their own gates.

After the four-stratum provider run completes, the independent
[all-copies auditor](audit_all_copies.py) compares all 32 byte copies per
base with the fresh S3 factor-base downloads. The four raw provider ZIPs are
also stored at content-addressed, write-once S3 keys under a separate
`provider-artifacts/` prefix. Their GitHub identities, complete fresh S3
downloads and four-object inventory are checked by the bridge's
[provider auditor](../n83_small_intermediate_bridge_20261018/audit_provider.py).
Keep `selected_best_total_runtime` null for the primary N83 objective.

Rebuild from the downloaded provider ZIP as `stratum0.zip` and replay fresh
S3 GET files using the frozen plan and pinned `blake3==1.0.8` dependency:

```sh
PYTHONPATH=/path/to/pinned-packages python3 research/n83_bridge_s3_custody_20261018/build.py \
  --provider-root /path/to/provider --output /new/output
PYTHONPATH=/path/to/pinned-packages python3 research/n83_bridge_s3_custody_20261018/verify.py \
  --provider-root /path/to/provider --downloads /path/to/fresh-gets \
  --build-receipt /new/output/build.json
```
