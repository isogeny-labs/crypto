#!/usr/bin/env bash
set -euo pipefail
study_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
run_dir="${1:-$study_dir/native/runs/run-$(date -u +%Y%m%dT%H%M%SZ)}"
if [[ -e "$run_dir" ]]; then
  echo "Refusing to overwrite $run_dir" >&2
  exit 2
fi
mkdir -p "$run_dir"
scratch_dir="$(mktemp -d "${TMPDIR:-/tmp}/twist-native.XXXXXX")"
finish() {
  status=$?
  printf '%s\n' "$status" > "$run_dir/exit-code"
  date -u '+Finished UTC: %Y-%m-%dT%H:%M:%SZ'
  rm -rf -- "$scratch_dir"
}
trap finish EXIT
exec > >(tee "$run_dir/run.log") 2>&1
date -u '+Started UTC: %Y-%m-%dT%H:%M:%SZ'
printf 'Command: bash run-native.sh FRESH_OUTPUT_DIRECTORY\n'
g++ --version | head -1
openssl version
uname -sm
git -C "$study_dir" rev-parse HEAD || true
(cd "$study_dir/historical" && sha256sum -c source-and-evidence.tar.xz.sha256)
tar -xJf "$study_dir/historical/source-and-evidence.tar.xz" -C "$scratch_dir"
cp "$study_dir/native/full.cpp" "$run_dir/source.cpp"
cp "$study_dir/native/json.hpp" "$run_dir/json.hpp"
cp "$study_dir/native/basis.cpp" "$run_dir/basis.cpp"
cp "$study_dir/native/full.cpp" "$run_dir/full.cpp"
cp "$study_dir/audit.cpp" "$run_dir/audit-source.cpp"
cp "$study_dir/run-native.sh" "$run_dir/runner-source.sh"
(cd "$run_dir" && sha256sum source.cpp full.cpp basis.cpp json.hpp audit-source.cpp runner-source.sh) > "$run_dir/inputs.sha256"
sha256sum "$scratch_dir/results/results.json" | cut -d' ' -f1 > "$run_dir/historical-results.sha256"
g++ -std=c++17 -O2 -Wall -Wextra -Werror "$run_dir/source.cpp" -lcrypto -o "$scratch_dir/full"
g++ -std=c++17 -O2 -Wall -Wextra -Werror "$run_dir/audit-source.cpp" -lcrypto -o "$scratch_dir/audit"
g++ -std=c++17 -O2 -Wall -Wextra -Werror "$run_dir/basis.cpp" -lcrypto -o "$scratch_dir/basis"
sha256sum "$scratch_dir/full" "$scratch_dir/audit" "$scratch_dir/basis" | sed "s|$scratch_dir/||" > "$run_dir/binaries.sha256"
"$scratch_dir/full" "$scratch_dir/results/results.json" "$run_dir/results.json" "$run_dir/parity.json"
"$scratch_dir/audit" "$scratch_dir" "$run_dir/independent-ic-audit.json" "$run_dir/results.json"
"$scratch_dir/basis" "$run_dir/basis.json"
(cd "$run_dir" && sha256sum -c inputs.sha256)
printf 'Native replay and independent certificate audit exit code: 0\n'
