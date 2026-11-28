# Third-party notices

The repository's own code is under the MIT licence in [`LICENSE`](LICENSE)
(action plan 2026-11-21, row F3; the owners confirm the copyright holder).
The files below carry, or are derived from, code under their own terms.
Their notices stay where they are and are **not** replaced by the root
`LICENSE`. Where a directory has its own licence file, that file governs the
directory.

| Path | Upstream | Licence | Note |
|:--|:--|:--|:--|
| `third_party/m4gb-patches/` | [cr-marcstevens/m4gb](https://github.com/cr-marcstevens/m4gb) | **GPL-3.0** | The patches modify GPL-3.0 code and are published under GPL-3.0 (their `README.md`). M4GB is built separately and run only as a subprocess; no M4GB source is linked into the crate. The root MIT licence does not cover these files. |
| `third_party/cryptominisat-rs-patches/` | `cryptominisat 5.8.0-rust1` (Robert Grosse), bundling CryptoMiniSat 5.8 (Mate Soos) | MIT | Patch applied to the crates.io tarball (its `README.md`). |
| `eigen_seta/LICENSE.castryck-decru` | Castryck–Decru attack implementation, © 2023 Giacomo Pope, Rémy Oudompheng, Lorenz Panny | MIT | Covers the Richelot, gluing and splitting formulas in `eigen_seta` (`eigen_seta/README.md`). |
| `research/eigen_seta_prototype_20261011/LICENSE.richelot_aux` | same authors | MIT | Prototype copy. |
| `docs/curves/standards/LICENSE.std-curves` | std-curves database, © 2018–2020 Jan Jancar, Vladimir Sedlacek | MIT | Source of `docs/curves/standards/parameters.json`. |
| `taskq/deploy/eks-karpenter/vendor/LICENSE.karpenter` | Karpenter | Apache-2.0 | Vendored deployment manifests. |
| `crates/hashed-ecdl-tensor/LICENSE`, `research/btn_ecdl_20261029/hashed-ecdl-tensor-fast/LICENSE`, `research/jacobian_smoothness_audit_20261109/native-census-v1/src/vendor/LICENSE`, `research/photo_volcano_restart_20261017/field_backend/LICENSE`, `research/photo_volcano_restart_20261017/source/native_snapshot/LICENSE` | sibling repository `cryptanalysis` ("The cryptanalysis contributors") | MIT | Copied from the sibling repository. |
| `crates/ecdl-tensor-ic/LICENSE` | this organisation ("isogeny-labs") | MIT | Same terms as the root licence. |
| `research/ideas_execution_20261018/targets_prime/verification_v2/vendor/LICENSE.MIT` | nlohmann/json, © 2013–2025 Niels Lohmann | MIT | Header-only JSON library. |
| `research/notes/ecc2k130/n13_oaware_unsat_proof_20260925/checker/LICENSE`, `research/notes/ecc2k130/symbolic_dag_dimacs_gate_20260925/third_party/LICENSE` | drat-trim, © 2014–2016 Marijn Heule, Nathan Wetzler (University of Texas at Austin) | MIT-style | Proof checker. |
| `research/chained_jacobian_transfers_20261110/evidence/native-cubic-cofactor-primary-review-01/arkworks-v0.5.0/` | arkworks v0.5.0 | MIT or Apache-2.0 | Both licence files kept. |

## How this list was made, and its limits

On 2026-11-22 every path in the tree at `main` (455,541 paths) was searched
for files named `LICENSE*`, `LICENCE*`, `COPYING*` and `NOTICE*`, and the
`third_party/` READMEs were read. Source files that carry a third-party
header but no separate licence file would not show up this way. A full scan
of file contents (for example with ScanCode) is owed before the owners treat
this list as complete.
