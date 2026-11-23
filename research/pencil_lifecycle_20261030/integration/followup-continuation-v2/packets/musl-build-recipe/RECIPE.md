# Read-only intended ecbench image-build recipe

The root's ongoing host `cargo build --release --bin ecbench` is software validation. It is not the required x86_64-unknown-linux-musl image executable. No build was started here.

## Observed prerequisites

- Rust1.99.1 is installed at /home/ec2-user/.rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu; rustup lists both GNU and musl targets installed. The musl libstd rlib/rmeta are present.
- Default registry cache is /home/ec2-user/.cargo/registry/cache (two index directories). All259 registry packages named by the current Cargo.lock have a corresponding .crate archive. This is a file-presence inventory, not an offline compilation proof or vendored/native-source qualification.
- Reusable partial musl target cache exists at /tmp/pencil-musl-target. Its rustc metadata records1.99.1; it contains host build scripts and musl dependency outputs. No release/ecbench exists in it. The repository target directory contains host debug/release outputs only.
- The retained /tmp/pencil-musl-build-20261029.log failed in ring0.17.14 because x86_64-linux-musl-gcc was unavailable. Current PATH resolves cc but not musl-gcc, x86_64-linux-musl-gcc, clang or zig. No alternate compiler was found in the inspected /opt, /usr/bin, /usr/local/bin, ~/.local or obvious musl/zig directories. This bounded inspection does not prove no compiler exists anywhere on disk.

Therefore the native musl C compiler/sysroot is the concrete prerequisite to qualify before repeating cargo. Rust's installed musl std target alone did not close the prior ring build. No compiler install or substitution is authorized by this recipe.

## Minimal intended command after prerequisite resolution

First freeze the exact final source manifest and reconcile the existing host build. Use an actual clean immutable release commit for publication; if performing only a private dirty-source software build, record dirty=true and do not represent it as that release. Set PENCIL_SOURCE_REVISION and PENCIL_SOURCE_DIRTY from that frozen source (not stale shell metadata). Set PENCIL_MUSL_CC only to the reviewed installed musl compiler, with its identity/sysroot recorded; this value is currently unset.

```sh
cd /home/ec2-user/crypto-pencil-signed-graph-20261029
env -u CARGO_ENCODED_RUSTFLAGS -u CARGO_BUILD_RUSTFLAGS \
  CARGO_TARGET_DIR=/tmp/pencil-musl-target \
  CARGO_BUILD_JOBS=2 \
  RUSTFLAGS='-C target-cpu=x86-64' \
  CC_x86_64_unknown_linux_musl="$PENCIL_MUSL_CC" \
  CRYPTO_BUILD_GIT_COMMIT="$PENCIL_SOURCE_REVISION" \
  CRYPTO_BUILD_GIT_DIRTY="$PENCIL_SOURCE_DIRTY" \
  cargo +1.99.1 build --locked --offline --release --no-default-features \
  --target x86_64-unknown-linux-musl --bin ecbench
```

This preserves the intended release/musl/no-default-features recipe and reuses only the dedicated partial musl cache. A controller-selected build timeout/log/exit receipt should wrap the command. Reconcile cache ownership immediately before use. Do not run it now with empty placeholders; do not substitute host cc merely to bypass the missing compiler. Cross-linker overrides should be added only if required by the qualified toolchain, not guessed here.

Metadata behavior was read directly in src/cryptanalysis/ecbench/host.rs:432–449. CRYPTO_BUILD_GIT_COMMIT overrides fallback GITHUB_SHA; CRYPTO_BUILD_GIT_DIRTY accepts0/1 or true/false. Both must match the source actually compiled. The publication build-info.json must additionally bind the final binary hash, actual source revision, dirty/source manifest record, rustc1.99.1 -Vv, targetx86_64-unknown-linux-musl, architecturelinux/amd64, default_features=false, exact flags and separate retained fresh-owner hash/receipt.

After a successful build only: inspect ELF machine, absence of INTERP/dynamic NEEDED and hash the executable at /tmp/pencil-musl-target/x86_64-unknown-linux-musl/release/ecbench; install it0755 into a fresh explicit bundle. Reuse qualified fresh owner f6f6016e… rather than rerunning build-bundle and rebuilding it. Keep image digest and runtime qualification unset until actual image assembly/qualification. A successful static build alone is not image qualification.
