# Plan-only validation receipts

This directory records the prospective `ecbench plan` gate for all 104 frozen
projected-volcano cells. No `ecbench run`, solver execution, scientific cell,
EKS request, or measurement was started.

The executable was built from exact commit
`a6256b93b30f1f63f967823f04d91320df5eeffa` with Rust 1.99.0, `--locked
--release --no-default-features`, and `RUSTFLAGS=-C target-cpu=x86-64`. Its
SHA-256 was
`1174dea94f006838fb1fc96678b0126b247ca0caae2fec0597c40ec4a2ae86df`.

The compact receipt records these passing checks:

- all 104 cells used the exact four frozen arms in order;
- 31,232 workloads had unique per-cell workload IDs;
- 749,568 executions were planned;
- a complete second planning pass matched every first-pass plan byte-for-byte;
- protocol validation, deterministic spec regeneration, and all 105 frozen
  spec/manifest checksums passed.

The 104 raw JSON plans total 36,652,430 bytes. They were packed deterministically
as `plan-receipts.tar.xz`, 1,953,592 bytes, SHA-256
`088a8ecd4f1cab87703c425c4365cd6a2048866aedabb2033bf382f6758ae915`.
`xz -t` passed and a full extraction compared byte-for-byte with the source
tree. The canonical artifact-store push was attempted but failed closed on S3
`HeadObject` with HTTP 403 because the available identity could not establish
whether the immutable key was absent. Therefore the archive is local-only and
ignored, there is no `.artifacts.json`, and this record does not claim a remote
upload. Re-run the following only with the approved artifact-store identity:

```bash
PATH="/workspace/.venv/bin:$PATH" \
  python3 tools/artifacts/artifacts.py push \
  research/isogeny-volcano-projected-base_20261101/validation/plan-receipts/plan-receipts.tar.xz
```

After a successful push, require `artifacts.py verify --remote` and a fresh
download/hash comparison before changing the archive status in `receipts.json`.
