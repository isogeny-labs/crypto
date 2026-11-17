# Explicit-input review convenience bundle

The archive contains the 140 existing four-lead inventory artifacts plus explicitly selected report V1, exact referenced sources, source manifests and pointer provenance:182 source files and two generated internal mapping/checksum files. Inputs total48,840,410 bytes; compressed archive6,170,254 bytes. All input and extracted hashes passed. A second deterministic tar/gzip generation produced the same SHA256. Tar metadata fixes owner/group, modes, timestamp and PAX header names; gzip suppresses timestamp/name metadata.

Archive SHA256: `b68d2325649313314aa632a33dcaa5646900fac8fc33a237b55beb6088b34037`.

Extract into an empty directory and run `sha256sum -c SHA256SUMS` there. SOURCE-MAP.json preserves original paths, hashes and bytes, with H/, C/ and private/ prefixes. Only explicit allowlisted files are included, with no recursive checkout, global logs or credential directories. The build script rejects symlink inputs/ancestors, conflicting hashes and paths outside the three declared roots. No scientific output was recomputed.

Six historical local-untracked PDF/CSV entries have matching locally tracked artifacts.manifest/v1 pointer entries. These entries record hashes and sizes, without explicit version IDs. Their exact pointer manifests and local custody snapshot are included. Local untracked status does not mean unpublished. The six new integration receipts have no matching ancestor pointer in this snapshot. No remote availability claim follows from this builder's checks; root is separately checking historical remote objects. This bundle is a convenience copy and does not replace original custody.

Report V1, source indexes and historical evidence are unchanged. No upload or cloud access was performed by this builder. BUNDLE-RECEIPT.json records local packaging checks; it is not scientific validation or reviewer acceptance.
