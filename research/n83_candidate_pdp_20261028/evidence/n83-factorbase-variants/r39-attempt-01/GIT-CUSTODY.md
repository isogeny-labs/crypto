# Spike39 capsule Git custody map

The complete, immutable spike39 generation capsule remains at:

`s3://isolab-results-104186811093-us-east-2/runs/n83-ffd-lfd-factorbase-panel-20261105/v2/spike39/attempt-01/capsule-v1/`

That S3 prefix contains 16 files totaling 2,755,007 bytes and was previously downloaded and matched byte-for-byte. The tracked `CAPSULE-INDEX.json` is the manifest for that complete S3 capsule.

The Git projection omits only `first-fall-20261105T2013Z-attempt-01/instruments/pdp`, a 1,290,088-byte executable that the repository artifact policy places outside Git. Its S3 object remains at `capsule-v1/instruments/pdp`, with SHA-256 `b8e00f73512977828497c4bb30f0ae08d319350688a90b15310aa4143308a3ca`. The source, protocols, generation binding, factor-base indexes, command record, layout, and capsule index remain tracked.

To hydrate the executable into a fresh checkout before running `prepare-inputs.sh`:

```sh
CAPSULE=research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r39-attempt-01/first-fall-20261105T2013Z-attempt-01
aws s3 cp s3://isolab-results-104186811093-us-east-2/runs/n83-ffd-lfd-factorbase-panel-20261105/v2/spike39/attempt-01/capsule-v1/instruments/pdp "$CAPSULE/instruments/pdp" --region us-east-2
sha256sum "$CAPSULE/instruments/pdp"
```

The expected digest is the value above; verify it before execution. `instruments/.gitignore` excludes the hydrated runtime copy so it stays in S3 and does not re-enter Git. This capsule is prepared input-generation material; it contains no spike39 generated inputs or FFD/LFD results.
