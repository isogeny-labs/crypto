# Genus3Covers source and output importer

This directory pins the MIT-licensed [Genus3Covers](https://github.com/AndrewVSutherland/Genus3Covers)
repository at commit `221e74b7097a386527d3ebed68aa181b68954fd0`.
The upstream `README.md`, five Magma source files, license, and `P2example.txt`
are copied byte for byte in [`upstream/`](upstream/). The required default
number-field catalog is stored as `upstream/lmfdb_prym_nf.txt.gz` (gzip `-n`);
its **uncompressed** SHA-256 is
`8e10b5bf16ce52d6a0aaa2a236d295004ae96bf395ded902b34b71280c05c39a`.
The [ingestion record](../../research/genus3covers_upstream_20261018/README.md)
lists the archived result data and its exact digests.

The upstream computation requires Magma 2.29-5 or later. To run it, restore
the catalog in the same directory as `prym.m` and invoke Magma from there:

```sh
gzip -dc tools/genus3covers/upstream/lmfdb_prym_nf.txt.gz > tools/genus3covers/upstream/lmfdb_prym_nf.txt
cd tools/genus3covers/upstream
magma -b P:=2 prym.m > P2-new.txt 2> P2-new.log
```

`P2-new.txt` and the decompressed catalog are generated files; do not commit
them. The search is nondeterministic, so row count and order can differ from
the pinned example. Magma was unavailable for this integration, and the
vendored source has not been executed here.

The dependency-free Rust importer checks the nine-field row syntax, nested
integer coefficient arrays, curve shape, elliptic model, field polynomial,
and elapsed-time syntax. It emits one JSON object per input row with integer
coefficients represented as decimal strings, retaining arbitrary precision.
It parses the entire input before writing output, so a malformed row yields
no partial JSONL. It does **not** certify that a row's curve is smooth, that
the elliptic conductor or field discriminant is correct, or that a Prym
variety has the stated properties.

```sh
cargo test --release --offline --manifest-path tools/genus3covers/Cargo.toml
cargo run --release --offline --manifest-path tools/genus3covers/Cargo.toml -- \
  import tools/genus3covers/upstream/P2example.txt > /tmp/genus3covers-example.jsonl
```

Use `import -` to read from standard input, including a stream from `gzip -dc`.
The importer exits with code 2 on invalid input and identifies the first bad
line. `source_line` in JSONL is one-based. Each imported row is a candidate
record; connecting it to a repository curve identity, a checked map, Prym
arithmetic, or an ECDLP transfer requires separate validation.
