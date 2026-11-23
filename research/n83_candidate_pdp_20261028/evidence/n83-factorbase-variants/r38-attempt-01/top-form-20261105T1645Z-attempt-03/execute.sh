#!/usr/bin/env bash
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
EXPECTED_PDP=b8e00f73512977828497c4bb30f0ae08d319350688a90b15310aa4143308a3ca
EXPECTED_TOP=4749b49ba7e98cd916c9d36d4b14c0396041fd6ff31e3708e17e7a91749fe53a
if [[ -f "$HERE/build-target/release/pdp" && -f "$HERE/build-target/release/top" ]]; then
  printf '%s  %s\n' "$EXPECTED_PDP" "$HERE/build-target/release/pdp" | sha256sum -c -
  printf '%s  %s\n' "$EXPECTED_TOP" "$HERE/build-target/release/top" | sha256sum -c -
else
  bash "$HERE/build-exact.sh"
fi
bash "$HERE/run-spike38-topform.sh"
