# Raw evidence archives

`ARCHIVES.json` records SHA-256 and byte count for each archive; `SOURCES.json`
maps every run to its exact measured source and protocol. Each raw run has
one archive, including unsuccessful/interrupted samples. Inputs and frozen
worker binaries have separate archives. Compact derived tables stay in
`../results/final/`; they do not replace the raw evidence.

On this same-repository PR, GitHub's trusted `artifact-autofix` workflow
uploads policy-selected archives, replays their complete remote bytes and
replaces them with `.artifacts.json` pointers. Until that workflow succeeds,
the archives are available as Git blobs on the PR branch. No local-only path
is used as a durable evidence location. After migration, durable objects use
`s3://setec-astronomy-artifacts/store/sha256/<first-two-hash-digits>/<sha256>`;
the generated `.artifacts.json` is authoritative. Upload/replay CI status
must be checked before treating remote availability as established.

From a full repository checkout:

```sh
python3 tools/artifacts/artifacts.py pull research/relation_degree_trace_20261115/artifacts
cd research/relation_degree_trace_20261115
sha256sum -c artifacts/SHA256SUMS
for archive in artifacts/*.tar.xz; do tar -xJf "$archive"; done
cargo run --release --locked --bin report -- . /tmp/relation-report-new
```

`recorded-inputs/{cubic,projection,confirmation}` contains the exact fixtures
and CAS scripts. `recorded-workers/{compare,campaign}` are byte copies used
for confirmation (source 2471e4c); hashes are in
`confirmation-run-20261115/binaries.sha256` and `../validation/`.
Do not overwrite the archived runs when replaying: choose fresh output paths.
The source files in the PR may have subsequent formatting or reporting-only
changes; the measured worker source remains the pinned commit.

Original native command shapes:

```sh
recorded-workers/campaign --fixtures recorded-inputs/confirmation \
  --out /tmp/relation-fresh-replay --compare-bin recorded-workers/compare \
  --backend local --candidate-arm projection-criteria
target/release/external --fixtures recorded-inputs/confirmation \
  --native-results confirmation-run-20261115 --out /tmp/relation-cas-replay \
  --singular /path/to/Singular --msolve /path/to/msolve --cpu 0
```

Timing replays require a suitable CPU affinity and a separate frozen protocol;
these command shapes are not a claim of signed independent validation.
