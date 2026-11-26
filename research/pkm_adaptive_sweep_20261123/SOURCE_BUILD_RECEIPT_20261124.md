# Source-pinned PKM replay build receipt — 2026-11-24

The replay binary was built from Git commit `a5503f4bb66ac3aa4eac3125fba01534d2574096` (`Bring in upstream IC build repair`). That commit contains the root-trace/radical repair needed to compile while preserving the three PKM workload sources byte-for-byte from evidence commit `ca7fd6139a56fded02768dec5e239e16b85ddf27`:

| Source | SHA-256 at both commits |
|:--|:--|
| `src/cryptanalysis/ic_framework/pkm_semaev.rs` | `f645d0c8231dd9d02e1b21ede8704612c53e4b452cc9b75b7e065bce19e74e16` |
| `src/cryptanalysis/ic_framework/prime_tower.rs` | `85609ee73f57eb819e921f713931772c162bada2d9c7fe0a6fb373ad02ef2133` |
| `src/bin/ic/bench.rs` | `1110989c0ebfb3f1d060f3fd15bfd05a77e1250659c2c1778539766488f6e6aa` |

The source was exported with `git archive` into an isolated temporary directory. The build used the locked dependency graph and an explicitly selected compiler because the default Homebrew Cargo otherwise invoked Rust 1.93.1:

```bash
RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
  /Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/cargo \
  build --release --locked --bin ic
```

Compiler: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. The build completed with warnings. The stripping helper could not load its bundled `libLLVM.dylib`, but Cargo retained a runnable optimized binary and returned success. Binary SHA-256: `c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`; size approximately 57 MiB; `ic --version` reports `ic 0.1.0`.

Two failed build attempts remain operational context: one used the default Rust 1.93.1 through Cargo and was rejected by dependencies; one reached optimized code generation but filled the system volume. Only explicitly created `/tmp/pkm-source-replay*` temporary artifacts were permanently removed, recovering approximately 9 GiB. Neither failure is mathematical evidence. The successful binary is retained at `/Volumes/SSD990-2/.codex/tmp/pkm-a550-target/release/ic` for the frozen replay.
