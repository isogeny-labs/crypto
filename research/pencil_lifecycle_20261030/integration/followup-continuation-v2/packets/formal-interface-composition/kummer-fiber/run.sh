#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
out=${1:?fresh receipt directory required}
mkdir "$out"
cp -r source "$out/source"
rm -f "$out"/source/*.olean
lean=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean
export LEAN_SYSROOT=/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain
export LEAN_PATH="$out/source:/var/tmp/pencil-comparator-word-20261101/snapshot:/var/tmp/independent-product-review-20261101/snapshot:/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union"
{ printf '%s\n' "$out"/source/*.lean check.rs run.sh "$lean"; find /var/tmp/pencil-comparator-word-20261101/snapshot /var/tmp/independent-product-review-20261101/snapshot -maxdepth 1 -type f \( -name '*.lean' -o -name '*.olean' \); } | sort -u | xargs sha256sum > "$out/before.sha256"
"$lean" --version > "$out/lean-version.txt"
for m in Comparator Repair SignedPair Fiber ExpectedFailure; do
 set +e
 timeout -k 5s 120s "$lean" -j1 -M4096 -o "$out/source/$m.olean" "$out/source/$m.lean" > "$out/$m.stdout" 2> "$out/$m.stderr"
 code=$?
 set -e
 printf '%s\n' "$code" > "$out/$m.exit"
 if [ "$m" != ExpectedFailure ] && [ "$code" != 0 ]; then exit "$code"; fi
done
rustc --edition=2024 check.rs -o "$out/check-native"
"$out/check-native" "$out" > "$out/acceptance.json"
sha256sum -c "$out/before.sha256" > "$out/bindings.txt"
