# Verified publication to the user-selected S3 bucket

The user authorized `setec-astronomy-artifacts` as the upload destination.
The exact reconstructed primary K2048 hash-policy base and a 4,205-byte
public receipt bundle were uploaded in `us-east-2`, then downloaded anew.
Both PUTs used `If-None-Match: *` to prevent overwrites and AES-256 server-side
encryption. HEAD and GET lengths, ETags, SHA-256 metadata and downloaded
SHA-256 matched for both objects; archive BLAKE3 also matched. The
[round-trip receipt](publication.json) records every command and outcome.

The 10,313,647-byte archive is at:

`s3://setec-astronomy-artifacts/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/objects/82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851.jsonl.gz`

Its SHA-256 is
`aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`.
The [published receipt bundle](published-receipts.json) preserves the
original manifest, generic replay, reconstruction comparison and semantic
negative control unchanged; its new destination is recorded separately.
Historical references to the prior bucket remain historical evidence.
The receipt bundle's exact S3 URI and SHA-256 are in `publication.json`.

The restored Conductor check returned clear. Docker's client is now
installed, but the explicitly selected managed socket has no reachable
daemon, and no dockerd process was observed. The task UID is 1000; the
managed runtime guidance requires environment recovery for a non-root user
to obtain a working daemon. See [capability check](capability-check.json).
Thus this publication resolves storage access for the new destination;
guarded solver execution remains unavailable on this executor.

Validation: fresh download hashes, object metadata and receipt-bundle
integrity passed. Runtime source is unchanged from the
[previous complete validation](../cloud-input-reconstruction-20261018/VALIDATION.md):
4,091 release library, 19 exporter, 63 N83 Python and 46 boundary tests
passed. This change adds publication evidence and documentation only, so
those suites were not repeated. No worker binary or broad raw-source archive
was uploaded, and no new solver trial ran. Complete cold runtime and
`selected_best_total_runtime` remain unknown/null.
