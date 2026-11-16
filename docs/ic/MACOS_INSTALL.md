# macOS install and update

The macOS release bundle installs `ic`, `crax`, and the ECC2K-130 Python
engine from one signed-or-ad-hoc disk image.

## First install

Download `crypto-cli-v*-macos-aarch64.dmg` from the latest
[`isogeny-labs/crypto` release](https://github.com/isogeny-labs/crypto/releases/latest),
open it, and run `CryptoCLI Installer.pkg`. Or from a terminal:

```sh
hdiutil attach ~/Downloads/crypto-cli-v0.1.6201-macos-aarch64.dmg
sudo installer -pkg "/Volumes/CryptoCLI v0.1.6201/CryptoCLI Installer.pkg" -target /
hdiutil detach "/Volumes/CryptoCLI v0.1.6201"
```

This installs:

```text
/usr/local/bin/ic
/usr/local/bin/crax
/usr/local/share/cryptocli/version.json
/usr/local/share/cryptocli/docs/ic/params/ecc2k130-fixed.json
/usr/local/share/cryptocli/ecc2k130/codegen/*.py
```

`ic fixed` and `ic ecc2k130` find the engine and default ECC2K-130 profile
relative to `/usr/local/bin/ic`, so they do not need a source checkout.

Published DMGs are currently unsigned and ad-hoc signed. If Gatekeeper blocks
the installer, right-click the `.pkg` and choose Open, or remove the quarantine
attribute from the mounted volume:

```sh
xattr -dr com.apple.quarantine "/Volumes/CryptoCLI v0.1.6201/CryptoCLI Installer.pkg"
```

## Update

```sh
ic update --check          # report only
sudo ic update             # install the latest release
sudo ic update --version v0.1.6201
sudo ic update --dry-run   # verify the archive without replacing files
```

`ic update` queries the latest `isogeny-labs/crypto` GitHub release, selects
`crypto-cli-*-macos-aarch64.tar.gz` (or a universal archive when present),
downloads and checks `SHA256SUMS`, then atomically replaces `ic`, `crax`, and
the `share/cryptocli` tree. The running `ic` process is replaced by an atomic
rename and exits normally.

`GH_TOKEN` or `GITHUB_TOKEN` raises the GitHub API rate limit when set. Use
`--repo OWNER/REPO` for a mirror and `--install-dir PATH` for a non-standard
prefix.

Tests and offline installs can use a local archive:

```sh
ic update --from-tarball crypto-cli-v0.1.6201-macos-aarch64.tar.gz \
  --version v0.1.6201 --install-dir /tmp/cryptocli --yes
```

## Building the bundle

See [`packaging/macos/README.md`](../../packaging/macos/README.md). The release
workflow builds the Apple Silicon DMG on `macos-14`, clones the public
`isogeny-labs/cryptanalysis` repository for `crax`, and uploads the DMG, the
update archive, and `SHA256SUMS`.

The default release asset is Apple Silicon (`aarch64`). Intel/universal
packaging works through the same script once a matching `crax` build is
available; pass `--arch universal` after `lipo`-combining both slices.

## Signing

Without an Apple Developer ID, the DMG is not notarized. To sign and notarize:

```sh
export MACOS_SIGN_IDENTITY="Developer ID Application: Your Name (TEAMID)"
export MACOS_NOTARY_PROFILE="notarytool-profile"
packaging/macos/build_dmg.sh ...
```

The script signs the binaries and the package with hardened runtime, then
notarizes and staples the DMG. The GitHub release job needs those credentials
as repository secrets before a fully Gatekeeper-clean DMG can be published.
