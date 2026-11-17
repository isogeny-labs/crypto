# Curated private musl qualification view

This directory copies producer evidence byte-for-byte for review/publication preparation. It includes report/PDF/SVG, frozen plan, build and smoke scripts/receipts/logs, official source archive/signature/public key, download receipts, signature logs and installed-file manifest. No source was regenerated and no build was repeated.

custody-map.json maps every original top-level input/output to its included bytes or an explicit omission reason. Extracted source/build/object trees, installed prefix and GPG home are excluded. Source tarball and detached signature are pinned artifact inputs retained here for later content-addressed publication; no artifact upload was performed. producer-manifest.json is the original manifest, whose paths continue to describe the producer directory, not this curated subset.

download-verify-replay.py is a newly written retrospective replay convenience script, not an original execution receipt. Original download/verification commands are retained in their native receipts. ECBENCH-BUILD.md is the unexecuted next command, awaiting the actual final committed source pin and root's software envelope. Current runtime/image identities remain unqualified.
