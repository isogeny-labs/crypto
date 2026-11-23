# Evidence and reproduction

This directory contains unsigned development evidence for PR #4928. It carries no signed PCC standing, no admitted performance result, and no complete ECDLP recovery. [pcc:unverified]

## Frozen source

The new component is `research/prime31_ic_20261115/native/examples/prime31_dickson_quotient.rs`, SHA-256 `b76cd5a1efeff77bc065554d89acc9b53be051e6b17811cca589c2dc93a018b1`, Git blob `750eb4ed1ea511cc70b0e7a4af413a8bf7ebeeab`. Reused library files and their original Cargo manifest and lock are unchanged. `source-manifest.json` records their byte identities. These hashes identify bytes; they are not execution proofs.

## Receipts

| File | Scope |
| --- | --- |
| `new-example-tests-final.stdout` / `.stderr` | Five final component tests, including 25 original-curve cases |
| `source-library-tests.stdout` / `.stderr` | 41 original library tests under the cached-dependency adapter |
| `correctness-cases-final.stdout` / `.stderr` | Twelve final instrumented correctness cases and operation counts |
| `commands-final.json` | Exact commands and exit codes for the final component receipts |
| `commands.json` and earlier outputs | Earlier development iteration, retained rather than overwritten |
| `independent_review.md`, `independent-gate-test.log` | Separate agent's source audit and five-test replay |
| `original-lock-build.stdout` / `.stderr` | Exact-original-lock attempt, exit 101: unavailable `either 1.19.0` in offline cache |
| `dependency-differences.json` | Eight cached dependency version deviations |
| `development-adapter-original.toml` | Exact manifest layout used by the recorded local commands |
| `certificate-adapter-original.toml`, `certificate-adapter-original-lib.rs` | Original native certificate adapter layout |

The counts and pass reports above are unverified producer assertions until an authorized independent verifier records signed standing. [pcc:unverified]

The final diagnostic JSON SHA-256 is `2f491f3250b71bca1597668098f0feecd9a40d9c76714986b482b2c8b8fc6efb`. At N=64 and B=31, the recurrence uses 17,923 counted field multiplications and the sparse-product control uses 8,816. Neither count includes base setup, root extraction, relation validation or a complete logarithm. Both Horner controls include denominator-clearing rescaling that a normalized gcd-only oracle can omit. [pcc:unverified]

## Reproduce development checks

From the repository root, with the adapter's exact locked packages available:

```sh
cargo test --offline --locked --release --manifest-path research/prime31_summation_compression_20261122/development-adapter/Cargo.toml --lib
cargo test --offline --locked --release --manifest-path research/prime31_summation_compression_20261122/development-adapter/Cargo.toml --example prime31_dickson_quotient
cargo run --offline --locked --release --manifest-path research/prime31_summation_compression_20261122/development-adapter/Cargo.toml --example prime31_dickson_quotient
```

The committed adapter only changes the original scratch adapter's relative source paths. It uses the original Rust source files with a separate cached dependency lock. These commands were run in the original scratch layout, as recorded; the relocated manifest is provided for reproduction and is not falsely labeled as a second completed run. The upstream lock remains untouched. A CI environment with the original pinned packages must also run the original manifest.

## Identity certificates

`../identity-certificates/` contains four actual `identity.certificate/v1` records, their native SHA-256 replay receipt, and a separately seeded root replay. Every certificate has `proof:null`, 32 evaluation points over 2^61-1, and an additional 64-point replay. The issuer also performs checked-i128 integer polynomial coefficient normalization. This software calculation does not constitute Lean-kernel or PCC verification. The function-field degree theorem is outside these certificates' scope. [pcc:unverified]

The native issuer is `examples/identity_certificate_dickson_s3.rs`. Reproduce with the ordinary repository manifest when its dependencies are available, or with the explicitly qualified certificate adapter:

```sh
cargo run --offline --locked --release --manifest-path research/prime31_summation_compression_20261122/certificate-adapter/Cargo.toml --bin identity_certificate_dickson_s3 -- --check research/prime31_summation_compression_20261122/identity-certificates 2026112300234919
```

The root replay command was executed against the previously built scratch adapter binary, exited zero, and is retained as `../identity-certificates/root-replay.json`. The relocated certificate adapter refers directly to canonical repository source paths; it does not vendor or modify the certificate checker.

## Accounting and next gate

The native example is a development correctness and operation-count diagnostic. It is not an isolated benchmark. `../PROTOCOL.md` describes prospective matched, complete per-query accounting. No EKS job or paid compute was launched. No failed query, preprocessing, extraction, or membership work may be removed from a later performance comparison.

PCC claims pin the source, receipts and written arguments as unsigned attestations. The local workspace is a partial materialization through the GitHub connector, without a complete Git object graph; a zero producer commit in these claims means no local Git HEAD was available. Remote tracking commit `087744300ac8d91335f2099b72fd311743fe7d32` and the frozen source hashes supply the separate source provenance. Neither a fabricated Git object nor a substitute formal proof is used.
