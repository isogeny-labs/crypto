# Local solver toolchain receipt

The registered external arms reuse `tools/b0-ext-solvers`, which wraps
CryptoMiniSat 5.8.0-rust1 and cadical-sys 0.8.0 in one single-threaded Rust
binary.  Its existing patch adds conflict limits and counters.

The local host advertises an incomplete OpenMPI package whose imported CMake
target references the absent directory `/usr/lib/x86_64-linux-gnu/openmpi/include`.
The first build therefore stopped during optional MPI discovery.  The scoped
`0002-disable-mpi-discovery.patch` disables only that optional distributed
mode; the registered arms are single-process.  No solver input was read before
or during either build.

Reproduction:

1. Download crates.io `cryptominisat/5.8.0` and verify the crate SHA-256 in
   `build-receipt.json`.
2. Extract it to the gitignored `third_party/cryptominisat-rs/`.
3. Apply repository patch
   `third_party/cryptominisat-rs-patches/0001-stats-limits-nosqlite-cstdint.patch`,
   then this directory's `0002-disable-mpi-discovery.patch`.
4. Copy `b0-ext-solvers.Cargo.lock` to `tools/b0-ext-solvers/Cargo.lock` and
   run `cargo build --release --locked --manifest-path tools/b0-ext-solvers/Cargo.toml`
   with the recorded Rust/CMake/C++ versions.

The binary itself and build directory are reproducible local artifacts and
are not committed.  Their digest is committed.  All evidence remains
`[pcc:unverified]`.

The native arms are built from the research-scoped `native-solver` crate.  It
includes the repository's exact `src/cryptanalysis/sat.rs` by path rather than
copying it, supports the same ordinary-CNF and CNF-XOR files, records native
counters, and emits a complete model for independent replay.  Build it with:

```text
cargo build --release --locked --manifest-path \
  research/prime31_shared_coefficient_membership_20261118/native-solver/Cargo.toml
```

The common-support RR9 arm is built from `rr9-f4`.  Its build script copies the
repository's exact `src/cryptanalysis/f4_fp.rs`, `reuse.rs`, and
`pair_certificate.rs` into Cargo's output directory.  The narrow bridge
provides only the interfaces required by that source and independently checks
tower equations, monic divisions, Hermite decoding, and affine EC replay:

```text
cargo build --release --locked --manifest-path \
  research/prime31_shared_coefficient_membership_20261118/rr9-f4/Cargo.toml
```

The registered launchers verify every release-binary digest they use from
`build-receipt.json`.  The receipt also binds all four launcher sources, both
protocols, the selector and RR9 candidate sources, both isolated graders, the
repository SAT and F4 sources, and every standalone lockfile.
