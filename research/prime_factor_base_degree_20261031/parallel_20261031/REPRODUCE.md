# Reproduce and inspect the three-track result

The standalone instruments are native Rust; the F4 adapter is C. All crates
have locked dependencies and their own workspace. Use Rust 1.94.1 or a
compatible toolchain, plus GCC/OpenMP/GMP for the pinned neogb adapter.
`source-lock.json` records the exact source/spec and executed-binary hashes.
Building elsewhere need not reproduce machine-specific AVX2 binaries.

## Obtain the raw bytes

After artifact-autofix has migrated the archive, hydrate it with the repository
artifact tool before extraction:

```sh
python3 tools/artifacts/artifacts.py pull research/prime_factor_base_degree_20261031/parallel_20261031
cd research/prime_factor_base_degree_20261031/parallel_20261031
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

The Python artifact CLI transfers/checks stored bytes only. It performs no
research computation. The archive contains `runs/run-20261031T201044Z/` and a
separate `runs/replay-v2-20261031T201044Z/` companion. Original absolute scratch
paths are retained as historical provenance; the extracted paths are the
portable mapping. Never overwrite either original tree with a new run.

## Native build and development checks

From this study directory:

```sh
for crate in audit f4 ladder analysis reporting f4/replay_v2; do
  cargo test --release --locked --manifest-path "$crate/Cargo.toml"
  cargo clippy --locked --manifest-path "$crate/Cargo.toml" -- -D warnings
  cargo build --release --locked --manifest-path "$crate/Cargo.toml"
done
```

The adapter build is documented in [f4/README.md](f4/README.md). It requires
the exact external source revision; no upstream F4 source is patched.
The initial four crates and their specifications were frozen before data.
`reporting/` and `f4/replay_v2/` are separately versioned post-data analysis.

## Recompute the frozen primary analysis

```sh
analysis/target/release/secant-panel-analysis \
  runs/run-20261031T201044Z/audit \
  runs/run-20261031T201044Z/f4 \
  runs/run-20261031T201044Z/ladder \
  FRESH-primary-analysis.json
```

Compare its mathematical/count fields with the retained `analysis-all.json`.
Original source-record paths may differ after extraction. Preserve the source
hashes and the 26 unknown extraction cases. The tool refuses duplicate case
identifiers and incomplete four-branch targets do not receive degree credit.
Separate reporting and exact saved-basis replay commands are documented in
their own READMEs. Their outputs do not replace this primary result.

## Repeat the experiment

`runs/run-20261031T201044Z/exact-commands.json` retains the executed shell
commands, including CPU affinity, inherited address-space limits, all phase
caps, paths, redirections and the shared remaining F4 budget. The run plan,
per-track admission records, manifests, inputs, stdout/stderr, original failed
records and analysis are included. The specific local exception applies to
this recorded run; new experimental execution follows current backend and
checkpoint admission. Do not reuse the old receipts as new authorization.

## Deterministic archive

From this study directory, after extraction, the following reproduces the
archive bytes. Keep the original file and compare a fresh output by SHA-256.

```sh
tar --sort=name --mtime='2026-10-31 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > FRESH-runs.tar.xz
sha256sum FRESH-runs.tar.xz
```

The archive is transported by the repository's existing same-repository
`artifact-autofix` workflow, which uploads, downloads and hashes the complete
bytes before committing its `.artifacts.json` pointer. Publication is not
complete until that migration and required PR checks pass.

Raw `f4/development/*.stderr` retain upstream trailing whitespace, and
`f4/replay_v2/development/test.log` retains its trailing blank line. Their bytes
are evidence and are not rewritten to satisfy whitespace lint. Source/report
diff checks exclude only these raw logs; no mathematical or test check is waived.
