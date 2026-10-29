# R33 verified raw artifact

SHA256 `1ed0aaa793daa21db667ba6c1a4b6907d3fde67a1829484e79fa77d737ffb4e9`, 19511752bytes,247regular files.
The content-addressed object is in `setec-astronomy-artifacts/store`, indexed
by `.artifacts.json`. Fresh GetObject, all member hashes and deterministic
GNU-tar repack passed; see artifact-verification.json. This is byte integrity,
not independent mathematical replay. Earlier archives are unchanged.

The seal observed scientific commit `2988334ec59acd03896ce019297fbc7ff87af534` after its
commit operation completed. All75 source pins match:74tracked paths in that
commit and the one archived proof-cache path record. Original raw-meta.json
states the local sealing phase; subsequent remote verification is recorded
separately rather than rewriting that receipt. No source-revision race.

The archive contains all20 attempted runs, every native/Lean input and log,
failed/capped output, all donor inputs, explicit68 descended functions,
196/324 product tables, ten actual degree83 evaluation kernels, both closed
padding points and their actual deck actions, all2150 twist quadrics and
full padding Jacobians. The capped quotient's three completed prefix
families are retained, with omission4 explicitly unfinished. No full producer
acceptance or downstream specialization is inferred from that prefix.

Restore using:

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/33
cd research/koblitz_modular_ic_20261018/rounds/33
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

The proof-cache path record documents this host's pinned cache roots; a new
host must provide equivalent Lean4.34.1/mathlibd13f23b dependencies and map
those local paths appropriately. No credentials are contained in the bundle.
Do not repeat accepted native work merely to restore artifacts. Use a fresh
attempt directory for newly authorized computations and retain prior failures.
