#!/usr/bin/env bash
# Build the macOS CryptoCLI installer DMG and the update archive.
#
# The payload is:
#   /usr/local/bin/ic
#   /usr/local/bin/crax
#   /usr/local/share/cryptocli/version.json
#   /usr/local/share/cryptocli/ecc2k130/codegen/*.py
#
# The DMG contains a .pkg for first install. The .tar.gz is what
# `ic update` consumes. Both are signed ad-hoc unless MACOS_SIGN_IDENTITY is
# set; a Developer ID identity also permits optional notarization.
set -euo pipefail
export COPYFILE_DISABLE=1

usage() {
  cat <<'USAGE'
usage: build_dmg.sh --version TAG --arch ARCH --ic PATH --crax PATH [options]

  --version TAG           release tag, e.g. v0.1.6201 (required)
  --arch ARCH             aarch64, x86_64, or universal (required)
  --ic PATH               built ic executable (required)
  --crax PATH             built crax executable (required)
  --engine-dir PATH       ECC2K-130 Python engine (default: ecc2k130/codegen)
  --out DIR               output directory (default: dist)
  --sign-identity NAME    Developer ID Application identity; ad-hoc when unset
  --notary-profile NAME   notarytool keychain profile; notarize the DMG when set
  --keep-work             keep the staging directory for inspection

Environment:
  MACOS_SIGN_IDENTITY, MACOS_NOTARY_PROFILE
  CRYPTO_BUILD_GIT_COMMIT  source commit recorded in version.json
USAGE
}

VERSION=""
ARCH=""
IC=""
CRAX=""
ENGINE_DIR="ecc2k130/codegen"
OUT="dist"
SIGN_IDENTITY="${MACOS_SIGN_IDENTITY:-}"
NOTARY_PROFILE="${MACOS_NOTARY_PROFILE:-}"
KEEP_WORK=0

while [ $# -gt 0 ]; do
  case "$1" in
    --version) VERSION="${2:-}"; shift 2 ;;
    --arch) ARCH="${2:-}"; shift 2 ;;
    --ic) IC="${2:-}"; shift 2 ;;
    --crax) CRAX="${2:-}"; shift 2 ;;
    --engine-dir) ENGINE_DIR="${2:-}"; shift 2 ;;
    --out) OUT="${2:-}"; shift 2 ;;
    --sign-identity) SIGN_IDENTITY="${2:-}"; shift 2 ;;
    --notary-profile) NOTARY_PROFILE="${2:-}"; shift 2 ;;
    --keep-work) KEEP_WORK=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
  esac
done

[ -n "$VERSION" ] || { echo "--version is required" >&2; exit 2; }
[ -n "$ARCH" ] || { echo "--arch is required" >&2; exit 2; }
[ -x "$IC" ] || { echo "--ic is not executable: $IC" >&2; exit 2; }
[ -x "$CRAX" ] || { echo "--crax is not executable: $CRAX" >&2; exit 2; }
[ -d "$ENGINE_DIR" ] || { echo "engine directory not found: $ENGINE_DIR" >&2; exit 2; }
case "$ARCH" in aarch64|x86_64|universal) ;; *) echo "unsupported --arch: $ARCH" >&2; exit 2 ;; esac

case "$VERSION" in v*) TAG="$VERSION" ;; *) TAG="v$VERSION" ;; esac
PKG_VERSION="${TAG#v}"
PKG_VERSION="${PKG_VERSION%%-*}"

# Catch a mislabelled package before it is published. These are warnings
# because a developer may deliberately package a locally built version.
reported="$("$IC" --version 2>/dev/null | awk '{print $2}' || true)"
if [ -n "$reported" ] && [ "$reported" != "$PKG_VERSION" ]; then
  echo "warning: $IC reports $reported but the package version is $PKG_VERSION" >&2
fi
if [ "$ARCH" != universal ] && command -v lipo >/dev/null 2>&1; then
  for binary in "$IC" "$CRAX"; do
    if ! lipo -archs "$binary" 2>/dev/null | grep -qw "$ARCH"; then
      echo "warning: $binary does not contain the $ARCH slice" >&2
    fi
  done
fi
REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
GIT_COMMIT="${CRYPTO_BUILD_GIT_COMMIT:-}"
if [ -z "$GIT_COMMIT" ] && git -C "$REPO_ROOT" rev-parse --verify HEAD >/dev/null 2>&1; then
  GIT_COMMIT="$(git -C "$REPO_ROOT" rev-parse HEAD)"
fi
mkdir -p "$OUT"
OUT="$(cd "$OUT" && pwd)"

WORK="$(mktemp -d "${TMPDIR:-/tmp}/cryptocli-dmg.XXXXXX")"
cleanup() {
  if [ "$KEEP_WORK" -eq 1 ]; then
    echo "staging kept at: $WORK" >&2
  else
    rm -rf "$WORK"
  fi
}
trap cleanup EXIT

ROOT="$WORK/root"
BIN_DIR="$ROOT/usr/local/bin"
SHARE_DIR="$ROOT/usr/local/share/cryptocli"
ENGINE_OUT="$SHARE_DIR/ecc2k130/codegen"
DOC_DIR="$ROOT/usr/local/share/doc/cryptocli"
mkdir -p "$BIN_DIR" "$ENGINE_OUT" "$DOC_DIR"
cp "$IC" "$BIN_DIR/ic"
cp "$CRAX" "$BIN_DIR/crax"
chmod 0755 "$BIN_DIR/ic" "$BIN_DIR/crax"
# Preserve the module layout the Python engine imports at run time.
(cd "$ENGINE_DIR" && find . -type f -name '*.py' -print0 | while IFS= read -r -d '' file; do
    mkdir -p "$ENGINE_OUT/$(dirname "$file")"
    cp "$file" "$ENGINE_OUT/$file"
  done)
PARAMS_DIR="$SHARE_DIR/docs/ic/params"
mkdir -p "$PARAMS_DIR"
cp "$REPO_ROOT/docs/ic/params/ecc2k130-fixed.json" "$PARAMS_DIR/"
cp "$REPO_ROOT/packaging/macos/INSTALL.txt" "$DOC_DIR/INSTALL.txt"
VERSION_JSON="$SHARE_DIR/version.json"
python3 - "$VERSION_JSON" "$TAG" "$ARCH" "$GIT_COMMIT" <<'PY'
import json, sys
path, version, arch, commit = sys.argv[1:]
document = {
    'schema_version': 1,
    'operation': 'cryptocli-install',
    'version': version,
    'arch': arch,
    'binaries': ['ic', 'crax'],
    'engine': 'ecc2k130/codegen',
}
if commit:
    document['git_commit'] = commit
with open(path, 'w', encoding='utf-8') as stream:
    json.dump(document, stream, sort_keys=True, indent=2)
    stream.write('\n')
PY

sign_binary() {
  local binary="$1"
  if ! file "$binary" | grep -q 'Mach-O'; then
    return 0
  fi
  if [ -n "$SIGN_IDENTITY" ]; then
    codesign --force --options runtime --timestamp --sign "$SIGN_IDENTITY" "$binary"
  else
    codesign --force --sign - "$binary"
  fi
  codesign --verify --strict "$binary"
}
sign_binary "$BIN_DIR/ic"
sign_binary "$BIN_DIR/crax"

PKG="$WORK/CryptoCLI.pkg"
# AppleDouble `._*` files are packager noise, not payload.
find "$ROOT" -name '._*' -delete
xattr -cr "$ROOT" 2>/dev/null || true
pkgbuild --root "$ROOT" --identifier com.isogenylabs.cryptocli \
  --version "$PKG_VERSION" --install-location / --ownership recommended "$PKG"
if [ -n "$SIGN_IDENTITY" ]; then
  product-sign() { productsign --sign "$SIGN_IDENTITY" "$1" "$2"; }
  SIGNED_PKG="$WORK/CryptoCLI-signed.pkg"
  product-sign "$PKG" "$SIGNED_PKG"
  mv "$SIGNED_PKG" "$PKG"
fi

DMG_NAME="crypto-cli-$TAG-macos-$ARCH.dmg"
DMG_ROOT="$WORK/dmg"
mkdir -p "$DMG_ROOT"
cp "$PKG" "$DMG_ROOT/CryptoCLI Installer.pkg"
cp "$REPO_ROOT/packaging/macos/INSTALL.txt" "$DMG_ROOT/INSTALL.txt"
hdiutil create -volname "CryptoCLI $TAG" -srcfolder "$DMG_ROOT" \
  -ov -format UDZO "$OUT/$DMG_NAME" >/dev/null
if [ -n "$NOTARY_PROFILE" ]; then
  xcrun notarytool submit "$OUT/$DMG_NAME" --keychain-profile "$NOTARY_PROFILE" --wait
  xcrun stapler staple "$OUT/$DMG_NAME"
fi

STAGE="$WORK/stage"
TAR_NAME="crypto-cli-$TAG-macos-$ARCH.tar.gz"
TAR_ROOT="$STAGE/crypto-cli-$TAG-macos-$ARCH"
mkdir -p "$TAR_ROOT/bin" "$TAR_ROOT/share/cryptocli"
cp "$BIN_DIR/ic" "$BIN_DIR/crax" "$TAR_ROOT/bin/"
cp -R "$SHARE_DIR/." "$TAR_ROOT/share/cryptocli/"
cp "$REPO_ROOT/packaging/macos/INSTALL.txt" "$TAR_ROOT/INSTALL.txt"
tar -czf "$OUT/$TAR_NAME" -C "$STAGE" "$(basename "$TAR_ROOT")"

if command -v shasum >/dev/null 2>&1; then
  (cd "$OUT" && shasum -a 256 "$DMG_NAME" "$TAR_NAME" > SHA256SUMS)
else
  (cd "$OUT" && sha256sum "$DMG_NAME" "$TAR_NAME" > SHA256SUMS)
fi

echo "wrote $OUT/$DMG_NAME"
echo "wrote $OUT/$TAR_NAME"
echo "wrote $OUT/SHA256SUMS"
