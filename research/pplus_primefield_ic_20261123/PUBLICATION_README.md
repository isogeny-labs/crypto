# Portable source and evidence package

This subtree preserves the accepted selector qualification, the line-FFT
qualification, and the separate finite-evaluation identity records. Source,
protocol, manifest and evidence files were copied byte-for-byte. The new
`PUBLICATION_MANIFEST.json` identifies each copied file, its original relative
path, byte count and SHA-256. No research program was rerun while preparing
this package. Existing source and frozen recipes were not edited.

## Contents

* `selector/`: accepted attempt-02 source, compact native output, deliberate
  rejection controls, and custody records. Its normal output was emitted
  directly by Rust; Node in its original driver performs source/custody I/O
  only. No rejected Python harness is included.
* `fft/`: dependency-free Rust line FFT, derivation, frozen qualification
  protocol, commands, compact raw output and custody records.
* `fft/identity/`: unchanged production identity-certificate and SHA modules,
  native issuer/checker source, pinned Cargo manifests/lockfiles, dependency
  inventory, certificates and compact original receipts.
* `ATTEMPT01_CUSTODY.md`: explicit history of the rejected Python-orchestrated
  selector attempt and its oversized raw log. That rejected runner and log
  are deliberately not included here.

All scientific observations remain same-team and `[pcc:unverified]`.
Source hashes, separate checker processes, and this package manifest do not
constitute signed independent validation or proof of execution.

## Selector replay without full upstream snapshots

The copied `selector/production.rs` contains the complete selected dependency
closure needed by `selector/qualifier.rs`; it needs only Rust's standard
library. Full original production files are not needed to compile and run
those two files. From `selector/`, use a new output directory:

```sh
pplus_output=$(mktemp -d)
rustc --edition=2024 --crate-name pplus_selector_qualifier -C opt-level=2 qualifier.rs -o "$pplus_output/qualifier"
"$pplus_output/qualifier" > "$pplus_output/normal.json"
"$pplus_output/qualifier" --mutate-first-root
"$pplus_output/qualifier" --mutate-norm-one-lift
```

The first native call must succeed. The last two must fail through the
ordinary checking assertions. Preserve their exit codes and raw errors in a
fresh replay record. These instructions are provided for a future replay;
the packaging step did not execute them.

The archived `selector/run.sh` is the original frozen recipe. It deliberately
refuses to overwrite its fixed attempt-02 directory and its source-custody
utility expects full original snapshots at `../upstream/`. Those full
snapshots are not copied into this portable package. A replay of that exact
custody workflow therefore requires a clean staging copy and reconstruction
of the two original files from revision
`bd20b95dce44a91d7f74a1367853e724f280a3b6`:

| Repository source | Git blob | Expected snapshot name |
|---|---|---|
| `src/cryptanalysis/ic_framework/pkm_semaev.rs` | `6ed41bd81528ee2ddb9d471f1a0b4451511e5cf1` | `../upstream/pkm_semaev.rs.txt` |
| `src/cryptanalysis/ic_boundary.rs` | `2ef3958c2d83e1c6a40d2dbc345ab8a8c1032b89` | `../upstream/ic_boundary.rs.txt` |

`selector/source-manifest.json` binds their full SHA-256 values and every
selected byte range. The copied receipt records the original source-range
comparison. Compiling the selected module alone is a correctness replay, not
a fresh comparison against those omitted full upstream files.

## FFT and certificate replay

The FFT qualifier is standard-library-only and its recipe accepts a new
attempt identifier. From `fft/`, set `PPLUS_RUSTC` to the available compatible
compiler and run `bash qualify.sh attempt-02`. The original attempt used
Rust 1.100.0; the existing `attempt-01` evidence must remain untouched.

The identity issuer/checker additionally requires Cargo and the twelve
locked registry dependencies named in `fft/identity/dependencies.tsv` and
`Cargo.lock`. Its original recipe accepts `PPLUS_RUSTC`, `PPLUS_CARGO` and
`PPLUS_PARENT_CACHE`; choose a new attempt. The archive/index cache is omitted
from this package, so the offline recipe requires a separately prepared cache
containing those exact checksum-matched dependencies. Source and lockfiles
are included; a dependency-free or fully hermetic identity replay is not
claimed. The identity certificates cover fixed finite evaluations over
F_(2^61−1), not the complete FFT executable or a prime-field ECDLP solver.

## Custody and replay limits

Compiled executables, build targets and registry caches are excluded. Their
original hashes remain in receipts, but reproducing identical binary bytes
may depend on the original compiler, linker, system libraries and paths.
Some original receipts contain absolute execution paths; they are historical
records and were not rewritten to make paths appear portable. A future replay
must issue its own receipts, not overwrite or silently repair old ones.

The rejected attempt-01 raw selector log remains locally retained at 97,937
bytes. Its hash and original provenance are preserved in the custody note;
the object-store archive and descriptor have not been supplied by this
publication-staging subtask. That gap is separate from the compact accepted
selector evidence copied here. No oversized log was renamed or truncated.

This package supplies source and bounded correctness evidence. It contains
no measured speedup, useful relation rank, complete logarithm recovery,
full-library test pass, or independently signed validation.
