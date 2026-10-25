# PQC-Assay: native Rust reduction proofs

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
