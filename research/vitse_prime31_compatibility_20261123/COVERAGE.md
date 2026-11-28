# Pinned technique inventory

This is a bounded source audit for the supplied Vitse DLP 2014 talk. It records implementation coverage, not new execution or a proof of repository-wide absence. Current native execution is limited to the Prime31 compatibility study. [pcc:unverified]

## Snapshot boundaries

| Repository | Inspected revision |
|---|---|
| isogeny-labs/crypto | 7d61e3b45de6e93d235345a0061dae08db1f8b44 |
| isogeny-labs/ml-cryptanalysis | 3fbeeaa8734bb1b99779b2c2dd700b3f5017670f |
| isogeny-labs/cryptanalysis | f9c7b49e770212c24e29ca5c87e202e2aa6e8493 |
| isogeny-labs/crypto-autoresearcher | d46b06b5327b82a8a0b802df52923d18c637faf1 |

The original local crypto checkout was at 5926c60afd2d2b3d63b373c99059634faef8ceb6. It was not used to establish current coverage. Live connector source reads and the later separate pinned checkout supplied current evidence.

## Core implementations

Paths in this table are relative to crypto at the revision above. Status is source-inspected unless a historical study is explicitly named. [pcc:unverified]

| Source | What is actually present | Boundary |
|---|---|---|
| [gaudry_quartic_d4.rs](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/gaudry_quartic_d4.rs) | Adapted Y coordinate for rational 2-torsion, even coordinate-sign changes and permutations, elementary invariants of Y^2 plus the product, interpolation, Weil equations, solve/lift/decomposition path. | The D4 here is the Weyl-type even-sign action, not merely the order-eight dihedral group. |
| [gaudry_quartic_bin.rs](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/gaudry_quartic_bin.rs) | Characteristic-two t coordinate, t -> t+1, w=t^2+t and sum invariant, interpolation, Weil system and BinD4Oracle. | Binary-field construction; not an odd-prime x transformation. |
| [klein_v4.rs](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/klein_v4.rs) | Full rational E[2] coordinate action and invariant-field variables built from z^2+z^-2, products of z-1/z and z+1/z. | Requires the adapted coordinate and its torsion/field prerequisites. |
| [quartic_folds.rs](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/quartic_folds.rs) | Klein instance/base generation, precomputation, decomposition, relation collection and solve pipeline. | Historical batch accounting is not automatically complete cost; no new ratio is asserted. |
| [jv_quintic_edwards.rs](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/jv_quintic_edwards.rs) and [joux_vitse_edwards.rs](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/joux_vitse_edwards.rs) | Edwards extension-field symmetry, trace replay/fallback and signed-row recovery. | Symbolic preprocessing and full cost need their own accounting. |
| [ecbench/methods.rs](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/ecbench/methods.rs) | Registered quartic prime-extension and binary-extension routes. | Historical summaries that call these only disconnected formula experiments understate current source. |
| [symmetrized_semaev.rs](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/symmetrized_semaev.rs) and [semaev_higher.rs](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/semaev_higher.rs) | Lower-arity symmetric builders and specialized univariate higher-Semaev work through S6. | This does not establish a general exact P_phi,8 compiler. |
| coordinate_search.rs, coordinate_quotients.rs, coordinate_descent.rs | Particular coordinate searches, quotient/descent experiments, and a fixed-target stabilizer formulation. | Bounded experiments do not constitute arbitrary-coordinate-map support. |
| [torsion_orbit_20261013](https://github.com/isogeny-labs/crypto/tree/7d61e3b45de6e93d235345a0061dae08db1f8b44/research/isogeny_torsion_fibers_20261012/torsion_orbit_20261013) | Concrete order-three equivariant coordinate on y^2=x^3+1, with finite-prime checks. | Polynomial oracle, relation collection/linear algebra and scalar recovery were not completed in that study. |
| [semaev_selector.rs](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/semaev_selector.rs) | Split-selector pair extraction, polynomial norm/support checks and signed lifting. | Coordinate candidates are separate from signed relations. |
| [pkm_smooth_tower.rs](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/pkm_smooth_tower.rs) and [ic_framework/pkm_semaev.rs](https://github.com/isogeny-labs/crypto/blob/7d61e3b45de6e93d235345a0061dae08db1f8b44/src/cryptanalysis/ic_framework/pkm_semaev.rs) | Kummer, auxiliary-isogeny and Dickson selector machinery; invariant T0 quotient route where target torsion exists. | Auxiliary group action is not automatically a target-curve action. |

The historical studies gaudry_quartic_d4_20261013 and joux_vitse_n1_20261015 include actual small complete pipeline evidence and component comparisons. This audit does not rerun them, promote their component ratios to total gains, or erase unresolved/degenerate trials. [pcc:unverified]

## Sibling repositories

- ml-cryptanalysis: [csd/fp3_sym.py](https://github.com/isogeny-labs/ml-cryptanalysis/blob/3fbeeaa8734bb1b99779b2c2dd700b3f5017670f/csd/fp3_sym.py), fp3_sym_ic.py, and the native csd/fp3_native/src/pipeline.rs contain symmetry-based extension-field construction, extraction, signed recovery and public-target verification paths. tests/test_fp3_sym.py provides tests. This source exists even though new research in the present lane is Rust-only. [pcc:unverified]
- ml-cryptanalysis: experiments/ic-fp3-goal-sym2-20261117/klein_ic.py and its derive-invariant companion implement the full Klein route on a concrete model. Exceptional/repeated cases and historical runner independence must retain their stated limits. The later extension study supplies additional same-instance baseline evidence; it is incorrect to repeat an older missing-baseline claim without checking that follow-up. docs/records/POST334_INDEX.md is the consolidated pointer. [pcc:unverified]
- cryptanalysis: [ecc2k130/runner/codegen/nagaoannihilator.py](https://github.com/isogeny-labs/cryptanalysis/blob/f9c7b49e770212c24e29ca5c87e202e2aa6e8493/ecc2k130/runner/codegen/nagaoannihilator.py) contains a bounded coefficient search for the Nagao/Riemann-Roch construction and cites Vitse. It is not a universal arbitrary-coordinate-map compiler. [pcc:unverified]
- crypto-autoresearcher: searched inventory and research references did not add a complete Cartier or generic higher-torsion implementation to this audit. That is a bounded search outcome. [pcc:unverified]

## Gaps and search limits

Targeted reads and repository searches covered Vitse, summation/Semaev, symmetric coordinates, equivariance, Klein/V4, D4, Cartier, higher torsion, coordinate maps and P_phi,8, followed by the linked source and historical studies. The result distinguishes code paths from prose mentions and primary implementations from duplicated inventory. It is not an exhaustive proof about every branch, binary, untracked file or inaccessible repository. [pcc:unverified]

The open items are a reusable arbitrary-phi construction with inverse/exception handling, exact high-arity P_phi,8 beyond the located specialized support, and a complete generic Cartier/higher-torsion extraction pipeline. Their usefulness on the ordinary Prime31 workload remains a separate applicability question. No blanket claim that all literature techniques are implemented, or that none can help, follows from this audit. [pcc:unverified]
