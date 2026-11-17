# PR #2117 current-head CLI and S3 acceptance

These receipts were produced with the release binaries built from crypto
`2025935afae24b1d07645a1d66a4feb19a3e52ac` on 2026-10-25. The
subsequent evidence-only commit does not change those binaries. All S3 writes
used unique prefixes under
`s3://setec-astronomy-artifacts/ic/v1/acceptance/`; each prefix was checked
empty with S3 ListObjectsV2 after exact-prefix cleanup.

| Receipt | Checked behavior |
| --- | --- |
| [Artifact store](PR2117_EXACT_HEAD_ARTIFACT_S3_ACCEPTANCE_20261025.json) | Upload, remote existence and downloaded bytes, fresh-cache pull, byte equality, cleanup. |
| [Prime IC route](PR2117_EXACT_HEAD_PRIME_ROUTE_S3_ACCEPTANCE_20261025.json) | Fetch target and certified two-edge route from S3 on demand, verify scalar 190, publish and retrieve report, cleanup. |
| [Cubic IC route](PR2117_EXACT_HEAD_CUBIC_ROUTE_S3_ACCEPTANCE_20261025.json) | Fetch target and certified one-edge route from S3 on demand, verify scalar 5, publish and retrieve report, cleanup. |
| [Isogeny walk](PR2117_EXACT_HEAD_WALK_S3_ACCEPTANCE_20261025.json) | Publish walk; `exists --check-objects` checks three referenced files; fetched files match; independent walk verify checks three curves and two edges; after cleanup `exists` is false. |

The target and route SHA-256 values and byte counts in both route receipts
match their committed source fixtures.

The [16-bit](../ic/fixtures/hash_joux_reduced_sha256_16bit_20261025.json)
and [9-bit](../ic/fixtures/hash_joux_reduced_sha256_9bit_20261025.json)
SHA-256 Joux certificates replay with the separate
[Python verifier](../ic/fixtures/verify_joux_sha256_certificate.py):

```sh
python3 docs/ic/fixtures/verify_joux_sha256_certificate.py \
  docs/ic/fixtures/hash_joux_reduced_sha256_16bit_20261025.json
python3 docs/ic/fixtures/verify_joux_sha256_certificate.py \
  docs/ic/fixtures/hash_joux_reduced_sha256_9bit_20261025.json
```

The verifier first checks its SHA-256 compression routine against Python's
`hashlib` on `abc`, then replays both blocks at every reduced-state link.
A one-block change to the 9-bit certificate was rejected at link zero. These
certificates concern an 8..24-bit compression state truncated after every
block; they do not certify a full-width SHA-256 collision.
