# Supplemental portable locations

This supplement leaves the rendered V1 report, artifact index and source manifest unchanged. `portable-locations.json` maps every indexed artifact and source to either a repository-relative path or an explicitly private evidence-bundle-relative path. It also includes the exact Rust/Lean/source entries from law-source-hashes.json, so readers can resolve the mathematical source references without relying on the original host layout.

Both local checkouts identify origin https://github.com/isogeny-labs/crypto.git. H was observed at commit `157f7a0d82f73fef3490c84bcc9a3a659fa9a05d`; C at `1befe970928bc83257caa4cfc35432ca72385ea4`. These are local HEAD observations, not claims that either commit is remotely published, that a branch is clean, or that every listed file is committed.

All 39 indexed local files matched their recorded SHA256. Of those entries,21 were tracked and their current bytes matched the blob in the observed HEAD. For those entries a reviewer can resolve `repository_relative_path` at that exact commit and verify the listed SHA256. Twelve repository-located entries were not established as available from that HEAD; the JSON records index/HEAD presence explicitly. Six entries live only in the private review bundle and require distribution of that bundle under its declared root label `c1-takeover-20261030`. Do not substitute current main or assume checkout alone supplies the local-only entries.

For review, first choose H or C from `repository_alias`, resolve the relative path, and verify `expected_sha256`. Treat missing or different bytes as unavailable evidence. For private entries resolve `bundle_relative_path` under the supplied evidence bundle and apply the same hash check. This supplement does not publish those files, repair missing provenance, prove scientific outputs, or qualify local dirty source as a committed implementation.

Checks were read-only and bounded to listed paths: git ls-files, exact HEAD blob lookup/read, local file hashing and origin/HEAD metadata. No network, checkout, build, cloud job or scientific execution occurred.
