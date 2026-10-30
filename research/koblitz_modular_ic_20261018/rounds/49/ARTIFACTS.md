# R49 raw evidence custody
SHA256 `125720943b8436d3ecf555fba32b13f45cb06d1f0da3587baf35b8472e45f275`; 19300bytes,132regular files,60historical source pins.
Scientific source revision `9f1dc0c1fa4746401a3eafcf446922ad823e1d47`.
The archive retains all13attempts, native errors, four wallcaps, two failed
proof workflow attempts, accepted generic-prime data and all literal inputs. No
failed-attempt export counts as an accepted presentation or normalization.

Stored in setec-astronomy-artifacts/store. artifact-verification.json records
fresh direct GetObject, member hashes and deterministic byte-identical repack.
publication-validation.json checks59historical Git blobs,1archived donor
through the R34 committed manifest, and actual terminal
receipts, including the precompiler checkpoint failure. The original validator failed on that raw
donor/Git distinction; publication/ retains its failure and corrected checker. These are custody
checks, not independent scientific replay or semantic review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/49
cd research/koblitz_modular_ic_20261018/rounds/49
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Reuse accepted producers, never repeat them merely to resume publication.
Read CONTINUATION.md and PROOF_MAP.md before interpreting the result.
