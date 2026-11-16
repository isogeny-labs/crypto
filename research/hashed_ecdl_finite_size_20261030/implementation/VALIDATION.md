# Final native development checks

2026-10-30. All commands below are development correctness/static checks.
No scientific profile, benchmark or matched A/B panel was launched.

| Check | Outcome | Transcript |
| --- | --- | --- |
| Crypto release package suite | 55 passed: 39 library, 3 panel, 9 finite/CLI, 4 width | `crypto-tests.txt.gz` |
| Cryptanalysis release package suite | 81 passed: 51 library, 5 runner, 3 panel, 6 certified-hash, 9 finite/CLI, 3 strict-JSON, 4 width | `cryptanalysis-tests.txt.gz` |
| Strict clippy, both packages, all targets | Exit zero, `-D warnings` | `crypto-clippy.txt`, `cryptanalysis-clippy.txt` |
| Rust 1.78 locked all-targets checks | Both exit zero; crypto prints a pre-existing parent-manifest `unexpected_cfgs.check-cfg` warning | `crypto-msrv.txt`, `cryptanalysis-msrv.txt` |
| Formatting, shared-addition identity, scoped whitespace and hashes | Pass | `static-checks.txt`, source manifests |
| Native binary fixture identity check | Both a=0 and a=1 pass; exact generator/eigenvalue metadata retained | `binary-identities.txt.gz` |
| Graphviz/pdfLaTeX/Ghostscript report build | Exit zero; both current PDF pages rendered and visually inspected | sources and current SVG/PDF |

Commands (crypto root; substitute the binding workspace/package for its mirror):

```sh
cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml
cargo clippy --locked --all-targets --manifest-path crates/hashed-ecdl-tensor/Cargo.toml -- -D warnings
cargo +1.78.0 check --locked --all-targets --manifest-path crates/hashed-ecdl-tensor/Cargo.toml
cargo fmt --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --check
cargo test --locked --release --manifest-path bindings/rust/Cargo.toml -p hashed-ecdl-tensor
cargo clippy --locked --all-targets --manifest-path bindings/rust/Cargo.toml -p hashed-ecdl-tensor -- -D warnings
cargo +1.78.0 check --locked --all-targets --manifest-path bindings/rust/Cargo.toml -p hashed-ecdl-tensor
```

The installed toolchains were selected explicitly from
`/home/agent/.rustup/toolchains/{stable,1.78.0}-x86_64-unknown-linux-gnu/bin`
with the existing `/home/agent/.cargo` cache. Stable rustc/cargo are 1.99.1.
The initial PATH-only lookup missed this existing installation; no compiler
was downloaded and no credential was inspected to restore development access.

Source manifests are checked from their respective repository roots:
`crypto-native-sources.sha256` and `cryptanalysis-native-sources.sha256`.
The four new modules, lib/encoding additions, two new integration suites and
FINITE_SIZE.md match byte for byte across repositories. Existing hash-proof,
functional-pruning, reader/strict-JSON and point changes differ between the
integration bases and were preserved rather than overwritten. Relevant shared
function additions were applied as patches, not wholesale core-file copies.

The first compile rejected an ambiguous BigUint shift; it was made explicit.
The first strict lint run rejected three unnecessary test vectors; slices
replaced them. Both failed transcripts are retained as `check-initial.txt`
and `clippy-initial.txt`. A first mirror patch correctly refused the runner
because its bounded-reader imports differ; applied shared hunks separately
and preserved the mirror's runner protections. The new worktree initially
omitted other workspace manifests; sparse paths were materialized before
building. These failures were not converted to mathematical evidence.

Crypto main was merged locally after Copilot's merge request went unanswered
for about 25 minutes, with a public explanation on PR #2930. The only merge
conflict was the shared session log. Union resolution preserved all lines of
both versions: 216 insertions/0 deletions relative to the prior branch log,
22 insertions/0 deletions relative to the incoming log. The combined native
code passed the release/static checks above. Main's unrelated committed
LaTeX auxiliary/log whitespace was preserved; the scoped diff is clean.
No frozen evidence or protocol conflict was resolved by choosing a side.

Conductor remains unavailable and its documented warn/allow fallback applies;
no reservation is claimed. The native root backlog validator is not installed;
the card remains draft/zero-funded and canonical cross-field checking is
pending CI. No check, review or branch protection is bypassed. PR merge gates
are separate from these local development receipts.

The parent planning report is unchanged. A smaller-packet proof, authenticated
native execution, profiling, matched performance, both challenge scalars,
coverage analysis for changed bases and the independent full audit remain open.

Three successful raw transcripts contain an extra final blank line. Git's
whitespace check rejected those bytes as new text. They are retained in
deterministic gzip instead of edited; `raw-transcripts.sha256` binds the
original uncompressed bytes, and independent decompression/readback passed.
