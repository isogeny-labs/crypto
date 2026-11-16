# Externalized base-ledger snapshot

Bot commit2c8ee48b531695e5ffa9f85d205e3fe9512f1006 externalized the353423-byte base-ledger.jsonl snapshot. The committed .artifacts.json pins SHA25666e038b39a73d6f1dc7dbd1e8da8dc929b3ae89feb77f61d6a636326186f6c7b. The committed store configuration/key rule derives:

`s3://setec-astronomy-artifacts/store/sha256/66/66e038b39a73d6f1dc7dbd1e8da8dc929b3ae89feb77f61d6a636326186f6c7b`

This is a derived locator, not an independently read remote object. The exact local backup was restored after a fast-forward, without an agent S3 GET. The original snapshot is also retained in Git commit9202e538647748ced578ab2919192f3f0fe9a20f at this same path. Hydrate through the repository artifact path, or extract that original blob, then run sha256sum -c manifest.sha256 in this directory. This is metadata custody, not independent scientific verification. [pcc:unverified]
