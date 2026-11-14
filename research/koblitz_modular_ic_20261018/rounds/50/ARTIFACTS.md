# R50 raw custody

SHA256 `9f3290b96fd51f5f5ea2087c72c0cf87fbab7c6fc2fff786ef9344a2c971b1f3`;2198668bytes,360regular files,136historical source pins.
Scientific source `10a8d54517d95fc2af47df77ec317fa8c95efd31`.
All30 attempts,including10rejections,are retained in setec-astronomy-artifacts/store.
Fresh direct GetObject,all member hashes and byte-identical deterministic repack pass.
Historical Git source pins and actual receipts are checked separately from scientific replay.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/50
cd research/koblitz_modular_ic_20261018/rounds/50
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Use the documented prefix loader for the old PID comment header. Its actual coefficients
and the full serialized multiplication table pass a fresh reconstruction check.
Never repeat accepted producers to resume publication;read CONTINUATION.md.
