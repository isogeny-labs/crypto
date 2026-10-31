# Offline runtime archive qualification

The locally readable prior archive is `replacement-runtime.tar.gz`, SHA256 `72b227c00c3e3781b5b351aae133ffd2ece5c43313a4476b19c6d322bbffdaec`. Its full internal checksum manifest passed. The subsequent historical permission correction was applied to the image, not a separate archive: its receipt explicitly preserves this archive hash. Archive headers already specify progress0755 and controller0775. The later image had progress0700 after extraction/copy and was corrected using `COPY --chmod=0755`.

This new private archive retains that exact member layout and order, all file/directory modes and types, and all CRIU, tar and library bytes. Only controller/supervisor, progress, BUILD.json and SHA256SUMS changed. The replacement binary bytes match the adjacent final static qualification exactly. BUILD.json nests the previous build record and binds the new binaries to the exact qualification/source-manifest hashes. Before/after file hashes and mode inventories are retained.

Packaging used GNU tar PAX, root numeric ownership and epoch mtime with the original member list, then gzip with no timestamp/name. A fresh extraction passed the new checksum manifest; both replacement binaries matched their qualified sources. All nine unchanged dependency files matched byte-for-byte. No archive-level reproducibility assertion is made for PAX ancillary headers.

The fresh-owner binary belongs to the base task image (`Dockerfile.ecbench` copies it to `/usr/local/bin/isolab-fresh-owner`); it is intentionally absent from this runtime overlay.

No old snapshot permissions were changed. No new build, binary execution, image assembly, cloud operation or publication occurred. The historical image-only mode correction is an explicit input for future image packaging review: this archive alone does not establish permissions after a future extraction/COPY under a restrictive umask. Prior publication approvals identify other hashes and do not cover this archive.
