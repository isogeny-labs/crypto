# Incomplete root library check

`STATUS.md` and the frozen logs describe an incomplete run and a focused
missing-fixture failure. This is not a passing library suite or an exhaustive
diagnosis. PCC-20261115-806be3 remains unverified. [pcc:unverified]

The 282,285-byte raw log exceeds the data-store policy threshold. Local
`artifacts.py push` could not run because the AWS CLI is absent. The documented
same-repository `artifact-autofix` workflow must upload and replay it, then
commit its manifest and ignore pointer before the artifact gate passes.
The local frozen bytes and claim path remain unchanged.

When that pointer is present, hydrate before reading or checking claims:

```sh
python3 tools/artifacts/artifacts.py pull research/curve25519_torus_native_20261115/root-library-compatibility
python3 tools/pcc/pcc.py verify PCC-20261115-806be3
```

No artifact upload is claimed until that workflow's upload/readback succeeds.
