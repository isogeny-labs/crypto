# Native validation scope

Exact native commands from the repository root (offline caches already
present; the library command ran with permitted localhost fixture access):

```sh
CARGO_TARGET_DIR=/home/ec2-user/scratch/n83-sat-boundary-20261019/worker-target \
RAYON_NUM_THREADS=1 cargo test --offline --locked --release --lib \
  --example koblitz_n83_factor_base_export \
  --example koblitz_n83_fixed_witness_control \
  --example koblitz_n83_selector_witness_control -j2 -- --test-threads=1
CARGO_TARGET_DIR=/home/ec2-user/scratch/n83-sat-boundary-20261019/worker-target \
cargo test --offline --locked --release \
  --example koblitz_n83_free_selector_export_control -- --test-threads=1
CARGO_TARGET_DIR=/home/ec2-user/scratch/n83-selector-export-20261019/runner-target \
cargo test --offline --locked --release --manifest-path tools/n83-sweep-runner/Cargo.toml
```

Full permitted serial release suite at 5e9ee5d4b1aa6ca22368213e5bc05b343e00bfb4:
4397 library tests passed,118 ignored,0 failed (936.16 s test time);
exporter 19, original fixed witness0, pinned selector 1. Command wall including
compilation1607.61 s. Localhost fixtures were permitted; no assertion or
timeout was relaxed. Logs and exit0 are retained under validation/.

Final source cbe8e44da5ac0a792922d746368882a3b8f40170 adds an example,
standalone runner changes and protocol only. The diff from5e9ee5d4 under
src/, Cargo.toml and Cargo.lock is empty. Full validation applies to
byte-identical library sources. The new decoder example1test and release
build pass. Final native runner 8 unit +7 card integration tests pass,
including exact float-byte preservation. Original runner 14 tests and
transport correction15 tests are separately preserved. Original-group/domain
replay and coherent phase-forgery rejection pass. The original native relay
failure is transport evidence, not a library failure. No research Python
harness/test suite was run; Python only plots, renders and inspects PDFs.
