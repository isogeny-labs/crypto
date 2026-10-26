# PQC-Assay

The complete [PQC-Assay](https://github.com/amarshat/pqc-assay) proof suite is
imported as the standalone Git submodule [upstream/](upstream/), pinned to
`6b0096f4a2bda9cf07c7347094affc85d9eb24ff`. Its Makefile, reference targets,
models, proofs, assumptions and licenses remain unchanged. The native Rust
adaptation below is an additional local check.

## Standalone suite

From the `crypto` repository root:

```sh
verification/pqc-assay/standalone.sh check
verification/pqc-assay/standalone.sh setup
verification/pqc-assay/standalone.sh make verify
```

The runner initializes only `verification/pqc-assay/upstream`, verifies both
its Git link and checkout against `toolchain.env`, and rejects tracked source
changes. Upstream's private `paper/` submodule is not initialized; the proof
targets do not require it. No private repository access is needed.

`check` needs Git, make and Python 3. It checks the vendored target identity,
upstream claim lint and generated HQC proof inputs. These integrity checks do
not execute the theorem provers. `setup` delegates to upstream's pinned
toolchain and Isabelle-support installers. Upstream develops and runs CI on
Apple Silicon macOS; its Linux setup path is experimental. The separate
native Rust bootstrap below supports Linux x86_64.

Every upstream make target remains available through `standalone.sh make`:

| Command after `standalone.sh make` | Scope |
| --- | --- |
| `verify` | upstream's gated SAW/Isabelle pipeline, including its mutation controls |
| `saw` | reference ML-DSA C-to-model checks and signing composition |
| `mlkem-reduce mlkem-ntt` | reference ML-KEM reductions and transforms |
| `hqc hqc-avx hqc-isabelle` | HQC reference, AVX2 and model-to-mathematics checks |
| `qseal-tbs qseal-ref` | Q-SEAL model and reference serialization checks |

Additional projects, including Cap-V1/Kani, AutoCorres2 and ProVerif, retain
their own prerequisites and entry points in [upstream/README.md](upstream/README.md)
and the project-specific READMEs. `make verify` is upstream's defined pipeline;
it does not run every auxiliary target. `SKIP_ISABELLE=1` selects a SAW-only
bootstrap; `SKIP_PROVERIF=1` omits the independent protocol tool.
Compiler/tool overrides can be passed as make variables, for example
`standalone.sh make mlkem-reduce SAW=/path/to/saw CLANG=/path/to/clang`.
Logs go to the ignored `build/standalone-*.txt` files.

CI runs the standalone integrity checks on every relevant change. The manual
`standalone_verify` workflow input runs the full upstream `make verify` target
on macOS 15 and uploads its logs. This import does not claim that the full
pipeline has been rerun locally or that upstream's reference-C proofs cover
this repository's Rust implementation. Consult [VALIDATION.md](VALIDATION.md)
for actual execution results and [upstream/docs/ASSUMPTIONS.md](upstream/docs/ASSUMPTIONS.md)
for the upstream trust and composition boundaries.

To update the import, deliberately change the submodule commit and
`ASSAY_UPSTREAM_REV` together, review the upstream diff and licenses, and rerun
the affected proof and integrity targets. The native model cache is pinned
to the same revision.

## Native Rust reduction proofs

This ports the implementation-to-model leg of
[PQC-Assay](https://github.com/amarshat/pqc-assay) to this repository's
`src/pqc/fast/ml_dsa.rs`. Upstream is pinned to
`6b0096f4a2bda9cf07c7347094affc85d9eb24ff`; SAW is 1.5.1 and rustc is 1.90.0.
The selected functions use the same modulus and reduction algorithms as
upstream's ML-DSA-44 target. This repository implements ML-DSA-65. Sharing
these scalar primitives does not transfer upstream's NTT or signing proofs.

## Checked scope

Each row is one SAW proof over every input satisfying its stated precondition,
with Rust overflow checks enabled. The output is bit-exact equality to the
unchanged upstream `MLDSA_NTT.cry` model, not an equality to another Rust copy.

| Production function | Signed input domain | Cryptol model |
| --- | --- | --- |
| `montgomery_reduce` | `-2^31*q < a < 2^31*q`, `q = 8380417` | `montgomery_reduce` |
| `reduce32` | `i32::MIN <= a <= 2143289343` | `reduce32` |
| `caddq` | `-q < a < q` | `caddq` |
| `canon` | same domain as `reduce32` | `freeze` |

Every row has an output-bit mutation that must produce a solver
counterexample. `mutations.sh` additionally compiles a copy of production
`caddq` that subtracts q; the unchanged proof must reject it. Neither control
edits the production source. Positive and negative obligations share a symbol
and spec constructor. The runner checks the counterexample count, so a parse
error or missing function cannot stand in for a rejected mutant.

## Reproduce

Linux x86_64, Rust 1.90.0, git, curl, tar and sha256sum:

```sh
rustup toolchain install 1.90.0 --profile minimal
rustup default 1.90.0
verification/pqc-assay/setup.sh
verification/pqc-assay/verify.sh
verification/pqc-assay/mutations.sh
```

`setup.sh` checks the published SAW archive digest and fetches the exact
upstream commit. `sources.sha256` binds all production modules compiled by
the harness. A source change fails the identity check; update the hashes
deliberately and rerun both proofs and mutations. Do not update only the
hashes. `verify.sh` concatenates the complete current fast ML-DSA file and
four ABI wrappers into an ignored build file, then compiles it. The other
modules are loaded directly from their production paths. No function body is
extracted, rewritten, or replaced, and no dependency is stubbed.

`ASSAY_SAW_HOME` can select an installed SAW 1.5.1; `ASSAY_CACHE` selects the
download cache. Logs, compiler details and input/bitcode hashes are written
to `build/verify.txt`. GitHub Actions runs the native checks on changes to
any input and uploads the logs even on failure.

## Mathematical bridge and assumptions

The imported model is the same file that upstream lifts to Isabelle.
Upstream's `spec/isabelle/Assay_Equivalence.thy` contains
`montgomery_reduce_correct`, `reduce32_correct`, `caddq_correct`, and
`freeze_correct`. Their mathematical contracts and hypotheses are separate
from the native equalities above. To recheck that leg on an upstream-supported
machine:

```sh
verification/pqc-assay/verify-math.sh
```

The workflow's manual `math_reference` input runs it on macOS 15. It rebuilds
the pinned Isabelle/AFP/Cryptol sessions and regenerates and compares every
upstream lift. A cold run can take hours. This leg has not been rerun in the
initial port's local validation; upstream's reported results are not new
local proof results. See [VALIDATION.md](VALIDATION.md) for exact execution.

Trusted: rustc's Rust-to-LLVM translation, SAW's LLVM semantics, Cryptol,
the bundled z3 solver, and the proof-only ABI adapters. The mathematical
reference additionally trusts cryptol-to-isabelle and Isabelle and retains
upstream's recorded assumptions. The proof does not establish behavior on
inputs outside the table, reachability of these domains, NTT composition,
sampling, signatures, timing behavior, or other build optimizations. The
Keccak and reference signer modules are compiled for real dependencies, but
none of their functions is verified by these four obligations.

The companion C port is
[cryptanalysis/verification/pqc-assay](https://github.com/aburan28/cryptanalysis/tree/main/verification/pqc-assay).
Extending coverage requires a new local function contract and negative control;
upstream's proof table must never be copied as local coverage.

## Attribution

The model, proof design and model-to-Isabelle bridge come from
[amarshat/pqc-assay](https://github.com/amarshat/pqc-assay/tree/6b0096f4a2bda9cf07c7347094affc85d9eb24ff).
Its original project license is preserved in `UPSTREAM-LICENSE`. Vendored
third-party targets stay upstream and retain their separate licenses.
