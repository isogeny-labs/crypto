# Executed development checks (not experiments or benchmarks)

Base source: bec16151f92b5764e94579bf10b6680557d663ff, plus source bytes in
source-sha256.txt. Working source changes are committed with this audit.

- `make -C gpu/nist test BEST2=0` and `BEST2=1`, with the default C++ flags
  plus `-I/tmp/best2-toolchain/usr/include` for extracted Boost headers: exit 0.
- `make -C gpu/nist test-best2 BEST2=0` and `BEST2=1`: exit 0. The `openssl-b*.jsonl`
  files are direct executable stdout, without Make wrappers.
- Both OpenSSL tests also built with `g++ -O2 -std=c++17 -Wall -Wextra -Werror
  -Wno-unknown-pragmas -DRHO_BEST2=B gpu/nist/test_best2.cpp -lcrypto`; stdout
  exactly matches the retained records. Repeating BEST2=1 gives `cmp` exit 0.
- `g++ -O2 -std=c++17 -Wall -Wextra -Werror gpu/nist/best2_stats.cpp` succeeds.
  `stats_fixture.cpp` emits `stats-fixture.jsonl`; summary stdout is retained.
  Two concatenated copies are rejected (exit 1), stderr retained. The records
  and expected 1.1 ratio are synthetic validator fixtures, not research samples.
- Rust 1.75: `rustc --edition 2021 --test
  research/nist_best2_small_batch_20261027/model_unit.rs` then execution: exit 0.
  It imports the real model source with a minimal counted-group shim. It does
  not test the root registry or establish a root integration build.
- Rustfmt 1.75 ran on the new model, touched methods and shim source.
- `cargo test --release --lib`: exit 101 before compilation (lockfile v4
  unsupported by available Cargo 1.75). Exact stderr in cargo-test.log.
- `cargo run --release --bin backlog -- validate XP-20261027-nist-best2-seeded-validation`:
  exit 101 on the same toolchain/parser issue; native backlog validation pending.
- `bash -n gpu/nist/best2_ab.sh` and `git diff --check`: exit 0.
- PDF rendered with pdftoppm and both pages visually inspected.

No EKS/GPU job was submitted, no DP statistical sample was run, no root
integration success or independent replay certificate is asserted. Check files
stay loose because this document and SHA256SUMS name their exact paths.
