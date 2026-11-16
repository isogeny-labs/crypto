# N83 full-fixture Linux release-library validation, attempt 1

Frozen before execution on 2026-11-16 UTC. This is a repository validation run for PR #4295, not an index-calculus timing, relation, rank, descent, or factor-base comparison.

## Question and reference

The sparse-checkout Linux run at `verification/k600-colima-linux-lib-20261116/` had 6,395 passed, 45 failed, and 138 ignored tests. Its failed tests cited missing files. Test whether a full materialization of the same PR source, plus exact restoration of the one cited archived `records.jsonl`, removes those failures. Preserve the previous run as the reference; do not combine its test counts with this run.

The run source is the committed HEAD of the full detached-derived checkout at `/Volumes/SSD990-2/tmp/n83-full-validation-20261116`, recorded by `launch.sh` in `source.commit`. It is based on PR head `4b0361fabaae0bce302204ea0207136943422d00` plus this versioned protocol and launcher. The checkout must have `core.sparseCheckout=false`, a clean tracked worktree, and the tracked `gpu/fes/fes_solve.cpp` and `research/ic_planner_validation_20261020/sessions/r24/records.jsonl` present. Any later PR merge resolution needs fresh validation at the new source SHA.

Archived fixture: `research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl`, manifest SHA-256 `8970667e2f18587eeb08d033123dab029b94042ac5c07c870b08988f1ee5b738`, 2,293,097 bytes. The object was read from `s3://setec-astronomy-artifacts/store/sha256/89/8970667e2f18587eeb08d033123dab029b94042ac5c07c870b08988f1ee5b738` using profile `ecc-challenges-new`, and the local copy's full SHA-256 and size were checked. The launcher rechecks both on host and inside the container. This ignored fixture is a read-only input, not a source modification.

## Execution and stop conditions

Run `bash research/koblitz_n83_factor_base_sweep_20261008/verification/k600-colima-full-fixture-20261116/launch.sh` after `bash -n` and ShellCheck pass. Use pinned image `sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573`, Linux arm64, 12 GiB cgroup memory, zero swap, four CPUs, 512 PIDs, a fresh Cargo target, locked dependencies, one Cargo job, and a 7,200-second outer cap. Source/Git/alternate mounts are read-only; no Docker socket or AWS credentials enter the container. Require at least 20 GiB free on the boot disk, 80 GiB on SSD, and 10 GiB on the Colima VM root before launch.

The child command is `cargo test --release --lib --locked -j 1` with default features. Preserve all stdout/stderr, child and outer receipts, source SHA, Docker inspect, exact test summary and failed names. Stop at timeout, OOM, source/input mismatch, missing disk headroom, or nonzero test exit. A passing suite is a validation result for that exact source and fixture set; a failure retains its names and error text. No result here selects a factor base or changes the cold one-target cost claim.
