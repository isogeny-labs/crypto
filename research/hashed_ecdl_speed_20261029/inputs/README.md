# Frozen inputs for the hashed-ECDL A/B panel (2026-10-31)

These 85 files are the exact output of `hashed-ecdl-panel prepare` for the
frozen protocol in [../PROTOCOL.md](../PROTOCOL.md): 84 public problems
(60 development, seeds 1-10; 24 holdout, seeds 101-104; targets 1 and r-2 on
the three tiny curves) and `manifest.json`, which alone maps cases to expected
scalars and full cardinalities. Pass only `problems/<id>.json` to an arm.

| Item | Value |
| --- | --- |
| Hash list | [`SHA256SUMS`](SHA256SUMS), `sha256sum -c` format, 85 lines (manifest + 84 problems) |
| `SHA256SUMS` SHA-256 | `2fb0abc41597ba7c654019a0ef0e9a1f1ef6cfd3b23467f5a20666b7ae67667f` |
| `manifest.json` SHA-256 | `b28404c9a3b0d2d5b3db8d530f07746b4f0bfda3cc3090cb673e64930f3c1ea9` |
| Generator source | crypto `origin/main` 4df337e64d59bbe0145c204048ab1dc69a43c187 `src/bin/hashed-ecdl-panel.rs` (`prepare` unchanged on this branch) |
| Toolchain | cargo/rustc 1.97.0, `--release --locked` |
| Determinism | two independent `prepare` runs (unmodified origin/main binary and this branch's binary) produced byte-identical trees |
| S3 copy | `s3://setec-astronomy-artifacts/validation/hashed-ecdl-panel/20261031/` (86 objects: these 85 + `SHA256SUMS`) |
| S3 read-back | full download, `sha256sum -c SHA256SUMS` all OK, downloaded `SHA256SUMS` byte-identical to this one (2026-10-31) |

Verify locally: `cd research/hashed_ecdl_speed_20261029/inputs && sha256sum -c SHA256SUMS`.
The driver takes this file directly: `hashed-ecdl-panel run --panel <this dir>
--inputs-sha256 <this dir>/SHA256SUMS ...` and refuses any unpinned byte.
`hashed-ecdl-panel hash <dir>` regenerates the same list.

Preparation is not execution: no solve, timing or benchmark is attested here.
On 64-bit builds lambda <= 64, so root_u = lambda/131072 = 0 and these inputs
take the finite-size fallback; nothing exercises the recursive subexponential
kernel, and `ic.tensor_contraction_v1` is an exponential toy (protocol H1).
