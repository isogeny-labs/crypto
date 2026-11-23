# Continued-run raw evidence, 2026-10-26

**Verified:** immutable upload, native index registration, fresh-cache download,
remote verification and all **8,729** archived/loose member hashes. The user-selected
destination is **CTEC Astronomy Artifacts: `setec-astronomy-artifacts`**. This
uses the study-local store, not the shared repository destination. No IAM or
bucket policy changed.

- Name: `store:evidence/twist_routes_1_3_20261021/continue-20261026/runs.tar.xz`
- Remote: `s3://setec-astronomy-artifacts/store`
- Bytes: **16,546,704**
- SHA-256: `f0b73ee2cd1ab71cec520098a6c4e4868a0a0846daaf64c207be6fcf0839be7a`
- Partition: **8,668** archived files and **61** individually linked loose files.

[Publication manifest](RAW_ARTIFACTS.json), [all members](RAW_MEMBERS.json),
[loose metadata](LOOSE_METADATA.json), [upload/download receipt](../publication-20261026-v1/RECEIPT.json)
and [downloaded-member check](../member-verification-20261026-v1/ARCHIVE_CHECK.json).
Credential values were never persisted. The inherited native uploader's source
revision is not established; its binary hash and transport exits are separate
from the pinned scientific worker sources.

The archive covers sweep, cold decomposition, given-map transfer, 600-box
frontier, exact-row controls, box costs and opt-in linear-root costs, retaining
all incomplete/capped attempts. Main summaries, record lists, CAS transport
records and top-level logs stay loose where individual replay paths are named.
Other capsules, worker outputs, profiles, finite cases and scripts are archived.
Analysis uses the extracted tree and independent native audits.

## Pull, extract and verify

Build native `ecbench` with Rust 1.90.0 if unavailable. Use normal configured
AWS credentials without placing values in command arguments. From repo root:

```sh
twist_v3_study=research/twist_routes_1_3_20261021
CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store \
CRYPTO_STORE_INDEX="$PWD/$twist_v3_study/store/index.json" \
  target/release/ecbench store pull \
  store:evidence/twist_routes_1_3_20261021/continue-20261026/runs.tar.xz \
  --out "$twist_v3_study/execution-v3/continue-20261026/runs.tar.xz"
(cd "$twist_v3_study/execution-v3/continue-20261026" && sha256sum -c runs.tar.xz.sha256)
tar -xJf "$twist_v3_study/execution-v3/continue-20261026/runs.tar.xz" \
  -C "$twist_v3_study/execution-v3"
node "$twist_v3_study/verify-continue-archive.mjs" \
  "$twist_v3_study/execution-v3/continue-20261026/runs.tar.xz" /absolute/fresh/member-check
```

The checker extracts separately, restores loose metadata from the checkout
and checks every path/size/hash. Scientific native/PARI audits are separate.
The older E26 archive supplies frozen expected witnesses, bases and complete
basis certificates: `store:evidence/twist_routes_1_3_20261021/block-20261025/runs.tar.xz`.
Incoming commit e621366c1 moved some historical loose metadata/PDFs; their
`.artifacts.json` records identify them. Original bytes remain reachable in
02e233a645e090f374aad0c7260ee7a91e46c4cb. Restore exact historical dependencies
in a separate replay checkout when required. Neither earlier archive is
repacked or rewritten by this wave.

## Deterministic repack

After extraction, run in `execution-v3`:

```sh
tar --sort=name --mtime='2026-10-26 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu --no-recursion \
  --null -T continue-20261026/ARCHIVE_FILES.nul -cf - \
  | xz -6 -T1 > /absolute/fresh/repacked-runs.tar.xz
```

The digest must match. Before upload the packer extracted/checksummed every
archived member and repacked to identical bytes. Fresh remote download/member
verification is additional. Archive/caches are ignored; native study index and
loose manifests accompany the result.

## Inspected reports in the same bucket

A separate immutable publication copy keeps all eight inspected PDFs, editable figures, 150-dpi PNGs, source data and render sources in `store:evidence/twist_routes_1_3_20261021/continue-20261026/reports.tar.xz`. It has **77 files**, **2,542,272 bytes**, SHA-256 `1d780b238cdfde2af4a311f235cfcd931e53c0028d9ffe34e9496ee9c0f55832`. Conditional upload, native index, fresh-cache pull, remote verification and every member hash pass. [Report manifest](REPORT_ARTIFACTS.json) and [member check](../report-member-verification-20261026-v1/ARCHIVE_CHECK.json). Git report sources remain committed loose; this is a publication copy, not a history repack. Pull with the study-local remote/index above, then run `node research/twist_routes_1_3_20261021/verify-continue-reports.mjs ARCHIVE FRESH_CHECK_DIRECTORY`.
