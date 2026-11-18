#!/usr/bin/env bash
# Copy with prepare-qualifier.sh; execute only that immutable copy on RunPod.
set -euo pipefail
if [ "$#" -ne 3 ]; then
  echo 'usage: qualifier.sh REPOSITORY_ROOT EXPECTED_BINARY_SHA256 EXPECTED_RUNPOD_ID' >&2
  exit 2
fi
root="$(realpath "$1")"
expected_binary=$2
expected_pod=$3
case "$expected_binary" in *[!a-f0-9]*|'') echo 'invalid binary digest' >&2; exit 2;; esac
[ "${#expected_binary}" -eq 64 ] || exit 2
[ "${RUNPOD_POD_ID:-}" = "$expected_pod" ] && [ -n "$expected_pod" ] || {
  echo 'requires the specifically allocated RunPod pod' >&2; exit 2;
}
case "$expected_pod" in *[!a-z0-9]*) exit 2;; esac
receipt="$(cd "$(dirname "$0")" && pwd)"
(cd "$receipt" && sha256sum --check qualifier.sha256)
cd "$root"
study=research/p256_genus_dlp_20261115/minimal_transport
binary="$root/$study/native/target/release/p256-minimal-transport"
printf '%s  %s\n' "$expected_binary" "$binary" | sha256sum --check --status
sha256sum --check "$study/checks/build-001.sha256" > "$receipt/build-binding.txt"
test ! -e "$receipt/pre-call.json"  # never replace a pre-call record
commit="$(git rev-parse HEAD)"
printf '{"stage":"pre-call","source_commit":"%s","binary_sha256":"%s","runpod_id":"%s","native_calls_started":false}\n' \
  "$commit" "$expected_binary" "$expected_pod" > "$receipt/pre-call.json"
cp "$study/RESUME.md" .resume/RESUME.md
bash scripts/checkpoint.sh gate > "$receipt/checkpoint-gate.txt" 2>&1
uname -a > "$receipt/host.txt"
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$receipt/start.txt"
trap 'rc=$?; printf "%s\n" "$rc" > "$receipt/exit-code.txt"' EXIT
timeout 120 "$binary" qualify > "$receipt/qualification.json" 2> "$receipt/stderr.txt"
timeout 120 "$binary" qualify > "$receipt/qualification-repeat.json" 2> "$receipt/stderr-repeat.txt"
cmp "$receipt/qualification.json" "$receipt/qualification-repeat.json"
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$receipt/end.txt"
(cd "$receipt" && sha256sum pre-call.json qualification.json qualification-repeat.json host.txt > outputs.sha256)
printf '%s\n' 'producer qualification replay complete; no timing or net-speed claim'
