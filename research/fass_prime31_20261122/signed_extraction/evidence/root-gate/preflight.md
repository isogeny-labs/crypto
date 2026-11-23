# Root-library gate preflight

Read-only source/dependency inspection and exact tracked-byte hydration at `02d296ba3c80034a4a8a806d627db10d596b96cb`. No Cargo build or test was run by this preflight. The files below were absent because of sparse materialization; each was copied with `git cat-file blob HEAD:<path>`, without changing its Git bytes.

## Build inputs and source scope

- The root manifest plus all nine transitive local path-dependency manifests and their library entry points are materialized. The optional `five-leads-study` dependency is included in this inventory.
- No root `build.rs`, `.cargo/config`, `.cargo/config.toml`, `rust-toolchain`, or `rust-toolchain.toml` is tracked at this head. Their absence is expected; no build script or Cargo configuration was invented or copied from an older checkout.
- No Cargo config was found in the worktree, every ancestor directory through the filesystem root, or the default Cargo home. No `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`, `RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`, `CARGO_TARGET_DIR`, `CARGO_BUILD_TARGET`, `RUSTUP_TOOLCHAIN`, or `CARGO_HOME` override was set during this inspection.
- The conservative library-source inventory starts from every tracked Rust file below the root `src` (excluding `src/bin`) and each local dependency library/source directory. It includes tests and conditional modules without asserting all are active under the default features. It then follows literal include macros, manifest-directory concatenations and explicit `#[path]` attributes. No runtime fixture closure is asserted.
- Initial narrow Rust-source superset: 1374 tracked files, all materialized. Expanded scan visited 1377 Rust files, including external generalized-rootfinder sources.
- A broader directory inventory also included 151 absent Rust files in dependency examples, tests, and distinct nested study crates; these are not all dependencies of `cargo test --lib` and were not blindly hydrated.

### Manifest roots

- `Cargo.toml`
- `crates/certified-relations/Cargo.toml`
- `crates/relation-replay-checker/Cargo.toml`
- `crates/hashed-ecdl-tensor/Cargo.toml`
- `crates/ecdl-tensor-ic/Cargo.toml`
- `research/pencil_large_prime_20261029/Cargo.toml`
- `tools/factor-base/Cargo.toml`
- `tools/volcano-cli/Cargo.toml`
- `isogeny_algos/Cargo.toml`
- `research/five_leads_e2e_20261019/native/Cargo.toml`

## Hydrated tracked inputs

70 files; exact-byte SHA-256 hashes below are custody metadata, not test results.

| Path | Bytes | SHA-256 |
| --- | ---: | --- |
| `docs/curves/cover-catalog.json` | 123499 | `cfee6d447e7c377fd24ef3dd3b7e01eabab392aba503227fffefc58f70dba77c` |
| `docs/curves/factor-hints.txt` | 88555 | `17e6113e25a0ce173d85b141225a0d4785ae54863370428ee90efa9674373a21` |
| `docs/curves/ic/curves.yaml` | 71557 | `0bbdf920814f59463d0395429d37abb5a0833c1d89cbd3926c6d942bcc68bcc6` |
| `docs/curves/level-certificates.json` | 360256 | `3d6fa4bd11b68e5a9fe68904a5e9de98addfbbebf47cc6a0cfeed566faef6fbd` |
| `docs/curves/registry.json` | 22548033 | `b7046bad4a8e0b9e261a851b20e54affc035bd3dda12bc893796c38db5dc6183` |
| `docs/curves/standards/parameters.json` | 405178 | `587e8e12872df19e64d94a8302bd24082ca981e432d536544707fe13a9c3ea59` |
| `docs/curves/trait-sources.json` | 614 | `0f1e908940bad5cff04ca4b0f5509b6ec44f0c5cadec860c04c368c112b0791c` |
| `docs/curves/traits.json` | 14208652 | `c5591e3482b4522257f7e68dce054cb04be4296b47552bab2a048ca611dab65f` |
| `docs/ecbench/schema.sql` | 28724 | `d5e9605f4f11b10bb0d0b118d8d8870f62eab5e3d514ee81eb641cb670a9d0c4` |
| `docs/ic/archive-comparison-v1.json` | 3142 | `496d12d9f96583befb26118a144c01bd9f40870d58366a9e5cf74c45badf6420` |
| `docs/ic/archive-comparison-v1.json.sha256` | 101 | `2609d57993209515adbb8886d62bca3ddbf7c036f4cc6c903f3f03003b8bb792` |
| `docs/ic/archive-technique-comparison-v2.json` | 5675 | `4bac23b98b5006fdcba42b45aeb469a4096c2f99f3a0bea8547750c1a5bbca64` |
| `docs/ic/archive-technique-comparison-v2.json.sha256` | 111 | `b009a1961cc57fb2a6137c0e04aa87f1fa9459fbbb91c5143e5a7b7f7a6c4092` |
| `docs/ic/boundary_targets.json` | 1188295 | `468757a3b495e13e0f5909ec47ef110a2dbeb59ecd5b558141c379535d5ce3eb` |
| `docs/ic/calibration.json` | 9531 | `2184306e93b5ed848ed63de5830770639e463fa1a60b9a29bb2550e9f311f1b0` |
| `docs/ic/groebner-field-profile.json` | 419 | `8ff3ef52cba9fc2bf3c50a7707dfe2518d63ce4f8a6a4ab118af58af16133717` |
| `docs/identity-certificates/schoenhage-10mult.json` | 34875 | `84a61d6ccc237b724695e8f1416e9ad48f24c899ae316512bf0b952e46edb293` |
| `docs/units/bitop-v1.vectors.json` | 40375 | `150b4eca50847bc3541beff6f7ad3d0c67a2e2c76a4df58fe9cdd2942ce5c069` |
| `docs/units/bitop-v2.vectors.json` | 16699 | `c3b089170a1186442b5b5b6da14a591c573b0a347f5767dacde8d89b5d297400` |
| `gpu/certicom/arithmetic.h` | 8631 | `ee589d3c4d8de809f597d20d7d7162bb9333d16eb8b131c67da6ca3aeeface3a` |
| `gpu/certicom/emulator.cpp` | 788 | `710be5d077320fa7ae1d5d0137a3bc595f660ff53c9ec77c86479b7ce2e6abc2` |
| `gpu/certicom/wire.hpp` | 4004 | `34657d272b4c3ffe9e09faba46b9a9f56b67d73ca1cf3b011c6de70c72080339` |
| `research/aps_measurement_20261029/spec-controls.json` | 4174 | `c658760495904e069409dfd12c004a37d0b2ec4b7431037e4213d9b6233aba56` |
| `research/aps_measurement_20261029/spec-orbit-controls.json` | 2523 | `c7b8b73eeba09ec42933c42d64ea421efce9f2627b8ed04c410b5595030d133f` |
| `research/chained_transfer_next_20261115/select_only.tsv` | 605 | `f8ed37874a617d0c6caeef7bf5877a53556b6fbfb26179986914e09d95c7c1cb` |
| `research/curve25519_algebraic_ecbench_20261102/fixtures/pkm_iso_t6_g4_target3.record.json` | 10861 | `4e79f40a9e3a0611bf79fa78b0a1e411f1d5452726830f1512510d83aa755b56` |
| `research/curve25519_f1_targetwalk_20261108/data/band-k20-s0.json` | 2468 | `9169e209c39f3ab10fd4158dc1b82ef3f70ab3b67805efadff68b835f90e0b89` |
| `research/curve25519_index_calculus_20261102/model.json` | 2375 | `6815c56b927d2e189a904bd9cf5fcac6b902e6bb1e6042a0d75c37b3fef20ad7` |
| `research/curve25519_index_calculus_20261102/montgomery-translation.identity.json` | 5781 | `8b0f2ca98afc3e384d8c970e50f9d1c8b56bc3ded89709f060ab736981dfb4db` |
| `research/ecbench_factor_bases_20261013/sessions/prime20/host.json` | 1881 | `62d874b0c781780afe3553ceb62db2f9edf5af9addafc6f87c9fc2a5c75dc144` |
| `research/ecbench_factor_bases_20261013/sessions/prime20/plan.json` | 6721 | `20aaa4180f64be95ec0654bac9e960772e6bd819260145c0812af38ecc1ed253` |
| `research/ecbench_factor_bases_20261013/sessions/prime20/records.jsonl` | 103446 | `319d06f1e4af9680da6b40c65e7760c9091c24f94547adee7e014e5b07cb43d9` |
| `research/ecbench_factor_bases_20261013/sessions/prime20/session.json` | 3341 | `6e89ac785a9119e09755c2549922f58ee23924ef91c5b13f65c34fe422c37fa6` |
| `research/ecbench_factor_bases_20261013/sessions/prime20/spec.json` | 1261 | `09840560939b09e7189ba6d4c5042e62d8675e88a83637bd633c921c0920e3f6` |
| `research/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0/records.jsonl` | 7610 | `da674814594dd9f64d4e1b2d893e93e0102fe0ac42290ab44d0542c132345fe9` |
| `research/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0/session.json` | 2804 | `44e0b8a280c3a02595733aa8cba9299547345fe9bb6ac7d2c2b81895437c64af` |
| `research/ecbench_phased_panel_20261117/panel.json` | 26694 | `a5fccecb86e749aab879920e2100168266f838a3049838d9d85b5517c9302626` |
| `research/ecbench_rootfind_parity_20261013/sessions/claim-n13-v2/comparisons/rho-strong__ic-c2-frob-counted-m2.json` | 4519 | `718f8971362e6f40ddb1e2ad2fc301f009cd15e06f77130e643cc1c36bf7b2f2` |
| `research/efd_coordinate_charts_20261108/Charts.lean` | 13478 | `35d4f80d58ba4d5cc3e99f90e7d8716354b672f462860941a30f2be59fe2ecad` |
| `research/f4_linear_algebra_20260914/contract.json` | 3971 | `6182adce91e94c8577ba6bab5c04a0132a723292c03a35aff98d77937c01d622` |
| `research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/host.json` | 3466 | `a584565459bda5950744ad22ca38df150de0641e05e7e2b1f787b6f98137f07f` |
| `research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/plan.json` | 14633 | `738e92edc4885776f114ba88dac114966f684024f670f74d0d80527a5153ea3c` |
| `research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/records.jsonl` | 16147 | `d4eea9cbdd10c69a3cf24e3706239bbe0e2b087e94b9291f48bbfe02652fcf70` |
| `research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/session.json` | 3074 | `a2bb73941a6146b70abd23151ef8b02239f2062a7c3a3ddbfebdf81abe2ae510` |
| `research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/spec.json` | 1587 | `575028e68a81a714c67744c0fd51fbcd84ca99e1f35bf12498f005204b840662` |
| `research/genus_ladder_20261114/analysis.json` | 529582 | `dafec2ef7c6645e067a632b8fba4fdf592e16fceeb47dc4fb7565d08f82c2b1b` |
| `research/genus_ladder_scale_20261115/g3_analysis.json` | 31209 | `aaf231e1d355014f025186843c2fefa549743e4484672d40337044de8b000f1c` |
| `research/ghs_cubic_genus3_build_20261012/results/run-20261013T0320Z.jsonl` | 6969 | `d66aa9a65b87969fb289272724792dac47c8f500d3d96c79474c3fa6a06998a9` |
| `research/ic_candidate_tournament_20260915/goal_20260924/prepared-ic-state-v1/sat-preparation.json` | 21881 | `1b24a4d507fb1c211bb562b557a3eae6f96f6fdd1aa496f6f63480297a613150` |
| `research/ic_candidate_tournament_20260915/goal_20260924/prepared-report-contract-v1/sat-source-check/instance.anf` | 28877 | `e5d49feddef960a0ea8a74836f517563a4f63f987abfc45defbe36a337c83057` |
| `research/ic_candidate_tournament_20260915/goal_20260924/prepared-report-contract-v1/sat-source-check/instance.xor.cnf` | 39608 | `d46c9d1edea83cb1ea2d09ce6cab4f42ff15bce99263b676430e61f4930ab3a0` |
| `research/ic_candidate_tournament_20260915/goal_20260924/prepared-report-contract-v1/sat-source-check/manifest.json` | 4034 | `9f516b3b6e1600a63ae42bd6e06342c3a8539c8f84d5bccc86b62d11c500822b` |
| `research/ic_candidate_tournament_20260915/goal_20260924/prepared-report-contract-v1/sat-source-check/result.json` | 7170 | `d7c1ba4287da129f28567f5852048bfb78d8670c28786ca8f51da830e1496a43` |
| `research/isogeny_walk_20261004/runs/p224-ell61-radius2/isogeny_routes.json` | 544860 | `1c84fe54c11bff6d84d623b5d213f8f8b96d17b2795610c1d803a58c5249e523` |
| `research/isogeny_walk_20261004/runs/p256-ell31-radius2/isogeny_routes.json` | 429669 | `9241deccb95787d8a03a3beb8579dc34a15adb52881a1e2418228cd65879cc9c` |
| `research/koblitz_isogeny_descent_37_results_20260925/raw.json.gz` | 40311 | `eb4773d556886672e8b80735fc486c16b73765fd5069ff81133100bfc3eafa90` |
| `research/ml_dataset_cli_generator_preservation_20261101/fixtures/root-systems.json` | 67562 | `4921edde15b222fe087374ba6fa30e480d3b483778f28aeba53bfdb6f9e76c76` |
| `research/notes/ecc2k130/disjoint_cold_v2_20261001/FROZEN.json` | 48749 | `da958a3f1117dd1b88703fed2055c5c9f64255516e1e920f35d193bf53c6a88d` |
| `research/notes/ecc2k130/disjoint_cold_v2_20261001/fixtures/n37_L1024_b00.points.jsonl` | 27002 | `187ec04fe50326bbb2f17dadf76056841f04af37a8abb8ff7f502fcd531711ad` |
| `research/notes/ecc2k130/n37_native_basis_bridge_20261002/NATIVE42.json` | 11423 | `bd4bd8af982bcc65234ae1fae1ad52eb0ed41a1c203bf4d2556fd8e87f788a4c` |
| `research/prime_conductor_gap80_20261031/wide-harness-20261118/source-vectors.json` | 13679 | `0db8fbfc32efec7e71e748f1c7118aed572d8bd65dcbf0f2c6cf0e1643b9a144` |
| `research/quartic_pic_canonical_20261018/results/v4_dump_q11.jsonl` | 37445 | `ea6f7e8df29940a1e30d32968f70adaadd2fdbb8d7dc65bf29bcdf509e617b9a` |
| `research/quartic_pic_canonical_20261018/results/v4_dump_q5.jsonl` | 4371 | `be21ac07f7c5372b0ccb184df8a7fbced64d61791330e74db7eae9325c059344` |
| `research/quartic_pic_canonical_20261018/results/v4_dump_q7.jsonl` | 8833 | `3f3367ccc9eb63bc0cda0758ab640bbad2755bb32b5109dee67a843a8aba31f5` |
| `tests/fixtures/ecbench_manifest_small.json` | 568 | `235f49a6d07a5f7afb18a37f574ebc7ee1ae0b69dc9d4a33cd0b8ff15556b5b7` |
| `tools/factor-base/Cargo.lock` | 6224 | `73b5d91d6569e9550327d112f23a886da3770dcc01c99bf26a559aabfaace1bb` |
| `tools/factor-base/examples/prime-structured.json` | 1543 | `8599469c70cd7b9c2d96ae56cbc39dbd72374562c218d3ddb85966faad267309` |
| `tools/generalized-rootfinder/src/field.rs` | 16075 | `e030b77e1d2fa3e98cfbad95b13f6124c00c7f746d31d9a755d8b08f1079d8cb` |
| `tools/generalized-rootfinder/src/poly.rs` | 5437 | `184ce151b85a0c59d1162b12957e97d7916c0360b80bc601b5b159c64afa1c33` |
| `tools/generalized-rootfinder/src/solver.rs` | 47028 | `6aeaa092c3e6479ec88dff53ab71fa0ba9571e078978cb505ecbec1a7ad03ae0` |

### Include origins

- `docs/curves/cover-catalog.json` from `src/cryptanalysis/curve_traits/query.rs:159 (literal include_str)`.
- `docs/curves/factor-hints.txt` from `src/cryptanalysis/curve_traits/query.rs:168 (literal include_str)`.
- `docs/curves/ic/curves.yaml` from `src/cryptanalysis/curve_traits/query.rs:178 (literal include_str)`.
- `docs/curves/level-certificates.json` from `src/cryptanalysis/curve_traits/query.rs:179 (literal include_str)`.
- `docs/curves/registry.json` from `src/cryptanalysis/curve_traits/mod.rs:2313 (literal include_str)`.
- `docs/curves/standards/parameters.json` from `src/cryptanalysis/secg_standards.rs:20 (literal include_str)`.
- `docs/curves/trait-sources.json` from `src/cryptanalysis/curve_traits/query.rs:169 (literal include_str)`.
- `docs/curves/traits.json` from `src/cryptanalysis/curve_traits/query.rs:150 (literal include_str)`.
- `docs/ecbench/schema.sql` from `src/cryptanalysis/ecbench/db.rs:42 (literal include_str)`.
- `docs/ic/archive-comparison-v1.json` from `src/cryptanalysis/ic_framework/archive_tests.rs:658 (literal include_bytes)`.
- `docs/ic/archive-comparison-v1.json.sha256` from `src/cryptanalysis/ic_framework/archive_tests.rs:659 (literal include_str)`.
- `docs/ic/archive-technique-comparison-v2.json` from `src/cryptanalysis/ic_framework/archive_tests.rs:672 (literal include_bytes)`.
- `docs/ic/archive-technique-comparison-v2.json.sha256` from `src/cryptanalysis/ic_framework/archive_tests.rs:673 (literal include_str)`.
- `docs/ic/boundary_targets.json` from `src/cryptanalysis/ecbench/claim.rs:39 (literal include_str)`.
- `docs/ic/calibration.json` from `src/cryptanalysis/curve_id.rs:584 (literal include_str)`.
- `docs/ic/groebner-field-profile.json` from `src/cryptanalysis/semaev_field_profile.rs:18 (literal include_str)`.
- `docs/identity-certificates/schoenhage-10mult.json` from `src/cryptanalysis/identity_certificate.rs:832 (literal include_str)`.
- `docs/units/bitop-v1.vectors.json` from `src/cryptanalysis/bitops.rs:598 (literal include_str)`.
- `docs/units/bitop-v2.vectors.json` from `src/cryptanalysis/bitops.rs:607 (literal include_str)`.
- `gpu/certicom/arithmetic.h` from `src/cryptanalysis/certicom_dlp/device_tests.rs:38 (literal include_bytes)`.
- `gpu/certicom/emulator.cpp` from `src/cryptanalysis/certicom_dlp/device_tests.rs:33 (literal include_bytes)`.
- `gpu/certicom/wire.hpp` from `src/cryptanalysis/certicom_dlp/device_tests.rs:35 (literal include_bytes)`.
- `research/aps_measurement_20261029/spec-controls.json` from `src/cryptanalysis/ecbench/aps/tests.rs:427 (literal include_str)`.
- `research/aps_measurement_20261029/spec-orbit-controls.json` from `src/cryptanalysis/ecbench/aps/tests.rs:428 (literal include_str)`.
- `research/chained_transfer_next_20261115/select_only.tsv` from `src/cryptanalysis/chained_transfer/quartic_ic.rs:2539 (literal include_str)`.
- `research/curve25519_algebraic_ecbench_20261102/fixtures/pkm_iso_t6_g4_target3.record.json` from `src/cryptanalysis/ecbench/band.rs:1156 (literal include_str)`.
- `research/curve25519_f1_targetwalk_20261108/data/band-k20-s0.json` from `src/cryptanalysis/ecbench/band.rs:1234 (literal include_str)`.
- `research/curve25519_index_calculus_20261102/model.json` from `src/cryptanalysis/curve25519_ic.rs:472 (literal include_str)`.
- `research/curve25519_index_calculus_20261102/montgomery-translation.identity.json` from `src/cryptanalysis/curve25519_ic.rs:459 (literal include_str)`.
- `research/ecbench_factor_bases_20261013/sessions/prime20/host.json` from `src/cryptanalysis/ecbench/audit.rs:2420 (literal include_str)`.
- `research/ecbench_factor_bases_20261013/sessions/prime20/plan.json` from `src/cryptanalysis/ecbench/audit.rs:2414 (literal include_str)`.
- `research/ecbench_factor_bases_20261013/sessions/prime20/records.jsonl` from `src/cryptanalysis/ecbench/audit.rs:2426 (literal include_str)`.
- `research/ecbench_factor_bases_20261013/sessions/prime20/session.json` from `src/cryptanalysis/ecbench/audit.rs:2402 (literal include_str)`.
- `research/ecbench_factor_bases_20261013/sessions/prime20/spec.json` from `src/cryptanalysis/ecbench/audit.rs:2408 (literal include_str)`.
- `research/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0/records.jsonl` from `src/cryptanalysis/ecbench/claim.rs:2218 (literal include_str)`.
- `research/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0/session.json` from `src/cryptanalysis/ecbench/compare.rs:2193 (literal include_str)`.
- `research/ecbench_phased_panel_20261117/panel.json` from `src/cryptanalysis/ecbench/panel.rs:21 (literal include_str)`.
- `research/ecbench_rootfind_parity_20261013/sessions/claim-n13-v2/comparisons/rho-strong__ic-c2-frob-counted-m2.json` from `src/cryptanalysis/ecbench/compare.rs:2710 (literal include_str)`.
- `research/efd_coordinate_charts_20261108/Charts.lean` from `src/prime_ecc/efd/tests.rs:690 (literal include_str)`.
- `research/f4_linear_algebra_20260914/contract.json` from `src/cryptanalysis/f4_rref_tests.rs:814 (literal include_str)`.
- `research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/host.json` from `src/cryptanalysis/ecbench/audit.rs:2239 (literal include_str)`.
- `research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/plan.json` from `src/cryptanalysis/ecbench/audit.rs:2235 (literal include_str)`.
- `research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/records.jsonl` from `src/cryptanalysis/ecbench/audit.rs:2243 (literal include_str)`.
- `research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/session.json` from `src/cryptanalysis/ecbench/audit.rs:2227 (literal include_str)`.
- `research/f6_ic_ecbench_ladder_20261005/sessions/n31-killed-02/spec.json` from `src/cryptanalysis/ecbench/audit.rs:2231 (literal include_str)`.
- `research/genus_ladder_20261114/analysis.json` from `src/cryptanalysis/genus_ladder.rs:4235 (literal include_str)`.
- `research/genus_ladder_scale_20261115/g3_analysis.json` from `src/cryptanalysis/genus_ladder.rs:4246 (literal include_str)`.
- `research/ghs_cubic_genus3_build_20261012/results/run-20261013T0320Z.jsonl` from `src/cryptanalysis/plane_quartic_pic.rs:2251 (literal include_str)`.
- `research/ic_candidate_tournament_20260915/goal_20260924/prepared-ic-state-v1/sat-preparation.json` from `src/cryptanalysis/prepared_sat_control.rs:620 (literal include_str)`.
- `research/ic_candidate_tournament_20260915/goal_20260924/prepared-report-contract-v1/sat-source-check/instance.anf` from `src/cryptanalysis/prepared_sat_control.rs:622 (literal include_str)`.
- `research/ic_candidate_tournament_20260915/goal_20260924/prepared-report-contract-v1/sat-source-check/instance.xor.cnf` from `src/cryptanalysis/prepared_sat_control.rs:623 (literal include_str)`.
- `research/ic_candidate_tournament_20260915/goal_20260924/prepared-report-contract-v1/sat-source-check/manifest.json` from `src/cryptanalysis/prepared_sat_control.rs:621 (literal include_str)`.
- `research/ic_candidate_tournament_20260915/goal_20260924/prepared-report-contract-v1/sat-source-check/result.json` from `src/cryptanalysis/prepared_sat_control.rs:624 (literal include_str)`.
- `research/isogeny_walk_20261004/runs/p224-ell61-radius2/isogeny_routes.json` from `src/cryptanalysis/curve_traits/query.rs:176 (literal include_str)`.
- `research/isogeny_walk_20261004/runs/p256-ell31-radius2/isogeny_routes.json` from `src/cryptanalysis/curve_traits/query.rs:172 (literal include_str)`.
- `research/koblitz_isogeny_descent_37_results_20260925/raw.json.gz` from `src/cryptanalysis/binary_velu.rs:1635 (manifest concat include)`.
- `research/ml_dataset_cli_generator_preservation_20261101/fixtures/root-systems.json` from `src/ml_dataset/prime_chain_prefix.rs:1121 (literal include_str)`.
- `research/notes/ecc2k130/disjoint_cold_v2_20261001/FROZEN.json` from `src/cryptanalysis/binary_field_basis.rs:281 (manifest concat include)`.
- `research/notes/ecc2k130/disjoint_cold_v2_20261001/fixtures/n37_L1024_b00.points.jsonl` from `src/cryptanalysis/binary_field_basis.rs:349 (manifest concat include)`.
- `research/notes/ecc2k130/n37_native_basis_bridge_20261002/NATIVE42.json` from `src/cryptanalysis/binary_velu.rs:1911 (manifest concat include)`.
- `research/prime_conductor_gap80_20261031/wide-harness-20261118/source-vectors.json` from `src/cryptanalysis/ecbench/prime_mont4.rs:429 (literal include_str)`.
- `research/quartic_pic_canonical_20261018/results/v4_dump_q11.jsonl` from `src/cryptanalysis/plane_quartic_pic.rs:3507 (literal include_str)`.
- `research/quartic_pic_canonical_20261018/results/v4_dump_q5.jsonl` from `src/cryptanalysis/plane_quartic_pic.rs:3495 (literal include_str)`.
- `research/quartic_pic_canonical_20261018/results/v4_dump_q7.jsonl` from `src/cryptanalysis/plane_quartic_pic.rs:3501 (literal include_str)`.
- `tests/fixtures/ecbench_manifest_small.json` from `src/cryptanalysis/ecbench_large_prime.rs:2011 (literal include_bytes)`.
- `tools/factor-base/Cargo.lock` from `tools/factor-base/src/store.rs:34 (literal include_str)`.
- `tools/factor-base/examples/prime-structured.json` from `tools/factor-base/src/tests.rs:263 (literal include_str)`.
- `tools/generalized-rootfinder/src/field.rs` from `tools/generalized-rootfinder/src/solver.rs:2 (path attribute)`.
- `tools/generalized-rootfinder/src/poly.rs` from `tools/generalized-rootfinder/src/solver.rs:4 (path attribute)`.
- `tools/generalized-rootfinder/src/solver.rs` from `src/cryptanalysis/ic_framework/genroot.rs:38 (path attribute)`.

## Remaining limits

- No unresolved literal include or explicit path-attribute reference remained in the inspected source superset after hydration.
- The six root-library dynamic-looking macros found in the initial scan were `concat!(env!("CARGO_MANIFEST_DIR"), "<literal>")` references under `binary_field_basis.rs` and `binary_velu.rs`; all four distinct files were resolved and hydrated. One apparent dynamic macro under `src/bin/icprog/tournament_tests.rs` was a string used to test an include parser and is outside the root-library inventory.
- This textual inventory is not a complete Rust conditional-module interpreter, registry dependency-source attestation, or inventory of files opened at test runtime. Historical artifact-store inputs may still block runtime tests. Compiler dep-info can refine actual inputs only after a successful compile.
- Reusing a Cargo target cache is permitted only with Cargo fingerprinting intact; do not copy or rewrite old fingerprints to imply current-source validation. The root library is not a dependency of the standalone signed extractor.

Additional provenance/compiler/native-toolchain environment keys inspected (`CRYPTO_BUILD_GIT_COMMIT`, `CRYPTO_BUILD_GIT_DIRTY`, `GITHUB_SHA`, `RUSTC`, `CC`, `CXX`, `CFLAGS`, `CXXFLAGS`, `LDFLAGS`, `CARGO_INCREMENTAL`, target-specific Rust flags) were all unset.

## Recommended one-shot gate

Run from the new worktree after the parent records fresh pre-call hashes, using its fixed 600-second deadline, 6-GiB virtual-memory limit, one Cargo job and one test thread:

```sh
CARGO_TARGET_DIR=/workspace/scratch/457a488177fc/crypto-quotient/target \
CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1 \
cargo test --release --lib --locked --offline -j 1
```

The parent wrapper must impose the 600-second/6-GiB bounds and preserve exit status, stdout, stderr and post-call hashes. A timeout while compiling is incomplete, not a test pass. A failed runtime fixture is reported precisely; it is not repaired by changing frozen input bytes.
