# macOS packaging

Two artifacts are produced from one staged payload:

| Artifact | Purpose |
| --- | --- |
| `crypto-cli-<tag>-macos-<arch>.dmg` | First install. Contains `CryptoCLI Installer.pkg` and `INSTALL.txt`. |
| `crypto-cli-<tag>-macos-<arch>.tar.gz` | What `ic update` consumes. Contains `bin/ic`, `bin/crax`, `share/cryptocli/`. |
| `SHA256SUMS` | Checksums for both. The release workflow also checksums the Linux assets. |

The payload installs:

```text
/usr/local/bin/ic
/usr/local/bin/crax
/usr/local/share/cryptocli/version.json
/usr/local/share/cryptocli/ecc2k130/codegen/*.py
```

`ic fixed` and `ic ecc2k130` locate the Python engine relative to the
executable (`<prefix>/share/cryptocli/...`), with the source checkout as a
developer fallback. The archive carries the whole `ecc2k130/codegen` tree; the
standalone binary does not embed it.

## Local build

```sh
cargo build --release --bin ic
CARGO_TARGET_DIR=/path/to/crax-target \
  cargo build --release --manifest-path /path/to/cryptanalysis/suite/Cargo.toml --bin crax

packaging/macos/build_dmg.sh \
  --version v0.1.6201 \
  --arch aarch64 \
  --ic target/release/ic \
  --crax /path/to/crax-target/release/crax \
  --out dist
```

`--arch` is a label for the asset name. Build the matching architecture
(or a universal binary with `lipo`) before invoking the script.

## Signing and notarization

Unsigned builds are ad-hoc signed:

```sh
packaging/macos/build_dmg.sh ...            # codesign --sign -
```

With a Developer ID:

```sh
export MACOS_SIGN_IDENTITY="Developer ID Application: Your Name (TEAMID)"
export MACOS_NOTARY_PROFILE="notary-profile"
packaging/macos/build_dmg.sh ...
```

The script signs the binaries and the `.pkg`, then notarizes and staples the
DMG when `MACOS_NOTARY_PROFILE` is set. The current release job does not have
these credentials, so published DMGs are unsigned/ad-hoc until they are
provided.

## Release job

`.github/workflows/release.yml` has a `macos` job that:

1. builds `ic` on `macos-14` (arm64 only for now);
2. clones the public `isogeny-labs/cryptanalysis` repository and builds
   `crax` from `suite/`;
3. runs `build_dmg.sh` and uploads the DMG, tarball, and checksums.

The release job merges those assets with the existing Linux archives. The
update command only needs the tarball and `SHA256SUMS`.

## Update command

`ic update` reads `https://api.github.com/repos/isogeny-labs/crypto/releases/latest`
(or `--repo OWNER/REPO`), selects `crypto-cli-*-macos-{arch,universal}.tar.gz`,
downloads `SHA256SUMS`, verifies SHA-256, unpacks the archive, and atomically
replaces `ic`, `crax`, and the engine. It refuses to write to a directory it
cannot write; with the default `/usr/local/bin` install, run `sudo ic update`.

`GH_TOKEN` or `GITHUB_TOKEN` is sent as a bearer token when present, which
raises the GitHub API rate limit and supports a private mirror.

Tests use `--from-tarball` so no network is required:

```sh
ic update --from-tarball dist/crypto-cli-v0.1.6201-macos-aarch64.tar.gz \
  --version v0.1.6201 --install-dir /tmp/cryptocli-test --yes
```
