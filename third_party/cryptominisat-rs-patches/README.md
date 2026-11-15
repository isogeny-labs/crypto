# cryptominisat-rs patches

Patch applied by [`prepare.sh`](prepare.sh) on top of the crates.io tarball of
`cryptominisat 5.8.0-rust1` (Rust bindings by Robert Grosse, MIT; bundling
CryptoMiniSat 5.8 by Mate Soos, MIT). The prepared tree lands in
`third_party/cryptominisat-rs/` (gitignored) and is consumed through
`[patch.crates-io]` by `tools/b0-ext-solvers/`. crates.io is the one source the
lab's network policy allows; GitHub is not reachable from the cloud container
(checked 2026-11-15).

| patch | what it changes |
|:--|:--|
| `0001-stats-limits-nosqlite-cstdint.patch` | C shim and Rust API: `set_max_confl`, `sum_conflicts`, `sum_propagations`, `sum_decisions` (the C++ `SATSolver` already has them). `build.rs`: `NOSQLITE=ON`, because the tarball omits `scripts/xxd-alike.py` and `cmsat_tablestructure.sql`, which the CMake build reaches whenever the host has sqlite3; and `-include cstdint`, because the 2020 sources lack the include under current GCC. |

Reproduce: `third_party/cryptominisat-rs-patches/prepare.sh`, then
`cargo build --release` in `tools/b0-ext-solvers`.
