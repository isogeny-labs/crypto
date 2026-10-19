# Genus3Covers ingestion record

Review follow-up: [source/parser audit](REVIEW-20261019.md),
[candidate-count and validation-boundary figure](review-20261019.svg),
and [PDF](review-20261019.pdf). Original upstream artifacts are unchanged.

## Scope and provenance

Upstream: [AndrewVSutherland/Genus3Covers](https://github.com/AndrewVSutherland/Genus3Covers),
commit `221e74b7097a386527d3ebed68aa181b68954fd0`, MIT license. It is
research code for genus-3 double covers of elliptic curves over number fields
and associated Prym surfaces. The existing `crypto` cover checker constructs
different, fixed finite-field covers; the two paths must not be equated.

| Requirement | Status | Evidence or remaining work |
| --- | --- | --- |
| Pin usable upstream Magma sources | Implemented, source hashes checked | `tools/genus3covers/upstream/`, revision above; Magma is not installed here. |
| Make its default number-field catalog available | Implemented, unexecuted | Pinned `lmfdb_prym_nf.txt.gz`; restore to its original filename before running Magma. |
| Import the published example output natively | Verified as syntax and shape | All 654 `P2example.txt` rows pass the Rust importer and tests. |
| Validate the mathematical cover and Prym claims independently | Open | Requires exact curve/map checks and a Magma or independent algebra run. |
| Connect a record to ECDLP transfer and cost accounting | Open | Requires an explicit map on the selected subgroup, kernel/order checks, Jacobian/Prym solver, and full charged comparison. |

## Source and data inventory

The small source snapshot and example are vendored. The large historical
output archives below stay in the upstream repository; their hashes make a
later import precise without silently treating them as validated results.

| Upstream file | SHA-256 | Here |
| --- | --- | --- |
| `LICENSE` | `c6735cbf3c3ef1c558d173323b41389c4225cc7a9759a356fb3ec9acfd818a1f` | vendored |
| `README.md` | `372889d0d155e5fa7fd95e96f8f269c32a4bc80be069c8d74b362d9a0743364d` | vendored |
| `bielliptic.m` | `9a45c707dfe54a576a3ad75798289faddbd62304c9ffc772d7ced775ed0c5fbe` | vendored |
| `g3cover.m` | `87523c923f2e9e7b963e01b0620e17498199fef89dde342dc41f73299e7b0ea4` | vendored |
| `genus3.m` | `ce8c4845525c7d58124a3f16be4f94c7a4490d2a16e687024d0e98c7f8c9a04f` | vendored |
| `prym.m` | `1b6d94d21f9592f98b353ab899e47bc5716a44148a57984d6e359df8ce5dd25d` | vendored |
| `utils.m` | `b2fc9b2c6452864546a539d37a82d2c9a73384f34f82feaba4afebda4a833fdd` | vendored |
| `P2example.txt` | `512e74ee354fcfa42c5e1e1ca30afd2871fb809e95f700437f24ce8a8d592325` | vendored |
| `lmfdb_prym_nf.txt` | `8e10b5bf16ce52d6a0aaa2a236d295004ae96bf395ded902b34b71280c05c39a` | compressed here as `lmfdb_prym_nf.txt.gz` (`54e274f2d79fb545d5c81d7f7114fe3bab49e16564392b5781db604b22de1e02`) |
| `P2.txt` | `a2cc585e825922c1708aed6fc59d84d71569008ecb2738da134fed1d0f446ed9` | upstream only |
| `P2.log` | `0db17d51ab8a28b33b725f4f1cbe0e9e34cbe80739b5b51d58c88febf29e3ffe` | upstream only |
| `hyp_prym_output.txt.gz` | `549943cd1ef281330092c9986fdb13aa623a7bc52b74b80e0b0149d05ff1a826` | upstream only |
| `hyp_prym_usp4_output.txt.gz` | `fa05391d32154fceba6a03666d487d45642dca582b46f5888c27d9a3d8be2616` | upstream only |
| `spq_prym_output.txt.gz` | `faa4512dab1a05765c82e22f05f230c66d727bb773b58b33e8e0669554e6590b` | upstream only |
| `spq_prym_usp4_output.txt.gz` | `50091ca49486a2f6c01523ea91072aa6b34b5dc1b81e182b1d11f1fd9dcea8f5` | upstream only |

## Validation boundary

The importer checks representation only. Upstream's `n` is a cover-type code,
not field degree: type 1 is a plane quartic, type 2 is rationally
hyperelliptic, and type 3 is a rationally hyperelliptic twist. Its `g=2`
rows describe a genus-2 Jacobian isogenous to the Prym. `E` is five
Weierstrass coefficients over the rationals; it is not the finite-field EC1
identity in this repository. The example is not an independently reproduced
search. No timing, speedup, or ECDLP result is inferred from these records.

## Integration checks

On the macOS checkout, `cargo test --release --offline --manifest-path
tools/genus3covers/Cargo.toml` passed 3/3 tests. The full importer command in
the tool README returned 654 JSONL rows; `jq` parsed all rows, with type/genus
counts `1:2=40`, `1:3=382`, `2:2=92`, `2:3=126`, `3:2=4`, and `3:3=10`.
`cargo clippy --release --offline --manifest-path tools/genus3covers/Cargo.toml
-- -D warnings` passed. The decompressed catalog and all vendored plaintext
files matched the SHA-256 values above.

The first `cargo test --release --lib` run inside the sandbox returned exit
101: 4,381 passed, nine existing `pollard_collab` local-socket tests failed
with `Operation not permitted`, and 118 were ignored. Rerunning the required
release library suite with local-socket access (`cargo test --release --lib
--quiet`) returned exit 0: 4,390 passed, zero failed, 118 ignored.
