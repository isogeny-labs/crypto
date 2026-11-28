# Rust evaluator implemented and executed

The actual ecbench command is `isogeny-evaluate`. Scheduling, jets, prepared norms, coverage, coefficient reconstruction and twist transport are Rust; existing Rust Montgomery/Jacobian code and FLINT/GMP primitives are explicit dependencies. It does not call the C++ constructor. [pcc:unverified]

Final build:7 tests,18 reference controls/470 points,8 rejections and scoped port clippy pass. Full33555419-degree retained map and normalized dual pass all 33 independent point/composition checks on original subgroup 5629830654216971. This is the same26-bit prime End-conductor crossing, not the original 80-bit goal. [pcc:unverified]

The exact original 80-bit input ran through the final Rust build and hit its 60second cap without quotient/images. The earlier unsigned-codec refusal is retained separately. Existing Rust ECDLP/scalar results are preserved baseline, not re-measured by this port. [pcc:unverified]

[Two-page report](reports/report.pdf), [protocol](PROTOCOL.md), [signed input correction](SIGNED_INPUT_AMENDMENT.md), [native summary](summary.json), [raw archive](raw-results.tar.xz), [inventory](archive-info.json). Whole raw archive: 12164 bytes, SHA256 7591f3dfad2e846a0a3e5edb1fc1770f41a7e4229cff92340ba122f75679210e. Extraction/repack checks pass; no S3 upload/readback is claimed. [pcc:unverified]

Build argv and exact source/library/binary identities are in build-a through build-e and the pre-call manifests. Failed module-path/build-role/first smoke attempts remain. Current protocol has one post-call evidence label only; exact pre-call text is archived and mapped in protocol-annotation.json. Larger-family40-bit successor remains a separately proposed next experiment. Original80-bit and full 32-target ECDLP remain open. [pcc:unverified]
