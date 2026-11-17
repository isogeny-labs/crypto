#!/usr/bin/env bash
# Independent-host replay of the frozen auditor, never the chart producer.
set -euo pipefail
audit_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_dir="$(git -C "$audit_dir" rev-parse --show-toplevel)"
custody_dir="${AUDIT11_CUSTODY_DIR:?Set a fresh absolute custody directory}"
mkdir "$custody_dir"
date -u +%FT%TZ > "$custody_dir/actual-start.txt"
git -C "$repo_dir" rev-parse HEAD > "$custody_dir/actual-source-head.txt"
archive="$repo_dir/research/chained_jacobian_transfers_20261110/native-richelot-chart-boundary-10/evidence/first-frozen-unit-02/entire-first-chart10-native-source-compiler-qualifier-and-traces.tar.xz"
printf '%s  %s\n' e2aa430c16d062296e1f8f7fe3c3a150938c2b77fe24996a88c3f9791a244fb2 "$archive" > "$custody_dir/frozen-archive.sha256"
printf '%s  %s\n' 791dffb1f6f1ae7aff4ef5f41a2e3f87c26324ca360d044bd5bd06979b416e53 "$audit_dir/src/main.rs" > "$custody_dir/frozen-source.sha256"
printf '%s  %s\n' 8809526e6a4f7dee113835e3cd34844355743474569436cdd808254938269096 "$audit_dir/fixtures-11.csv" >> "$custody_dir/frozen-source.sha256"
record() {
  local phase="$1" want="$2"
  shift 2
  set +e
  "$@" > "$custody_dir/$phase.entire-stdout.txt" 2> "$custody_dir/$phase.entire-stderr.txt"
  local actual=$?
  set -e
  printf '%s\n' "$actual" > "$custody_dir/$phase.exit"
  printf '%s %s expected%s\n' "$phase" "$actual" "$want" >> "$custody_dir/actual-phases.txt"
  if [[ "$actual" != "$want" ]]; then
    printf 'Unexpected %s exit%s expected%s\n' "$phase" "$actual" "$want" >&2
    exit 1
  fi
}
record frozen-archive-guard 0 sha256sum -c "$custody_dir/frozen-archive.sha256"
record frozen-source-guard 0 sha256sum -c "$custody_dir/frozen-source.sha256"
mkdir "$custody_dir/entire-original-producer-hold"
record entire-producer-archive-extraction 0 tar -xJf "$archive" -C "$custody_dir/entire-original-producer-hold"
input="$custody_dir/entire-original-producer-hold/all-four-native-units.entire-stdout.txt"
printf '%s  %s\n' bb655a03fd8cc3ab9d162a5cefa3da732a48d9cb80f7853697cad1eb4ccae692 "$input" > "$custody_dir/frozen-input.sha256"
record frozen-input-guard 0 sha256sum -c "$custody_dir/frozen-input.sha256"
cp -a "$audit_dir" "$custody_dir/frozen-auditor-template"
manifest="$custody_dir/frozen-auditor-template/Cargo.toml"
record compiler-version 0 timeout 120 rustc +1.99.1 --version --verbose
record cargo-version 0 timeout 120 cargo +1.99.1 --version
record compile-only 0 timeout 120 cargo +1.99.1 build --manifest-path "$manifest" --release --offline --locked -j1 --bin audit_trace
qualifier="$custody_dir/immutable-audit-qualifier.bin"
cp "$custody_dir/frozen-auditor-template/target/release/audit_trace" "$qualifier"
chmod 555 "$qualifier"
sha256sum "$custody_dir/frozen-auditor-template/target/release/audit_trace" "$qualifier" > "$custody_dir/frozen-qualifier.sha256"
stat -c '%a %s %n' "$qualifier" > "$custody_dir/qualifier-mode-and-size-before.txt"
record original-positive 0 timeout 120 "$qualifier" audit "$input"
for control in forward-coordinate drop-projective duplicate-forward sign-relation summary-coefficient lost-infinity timing-only; do
  record "generate-$control" 0 timeout 120 "$qualifier" make-control "$control" "$input" "$custody_dir/$control.entire-transcript.txt"
  want=1
  if [[ "$control" == timing-only ]]; then want=0; fi
  record "audit-$control" "$want" timeout 120 "$qualifier" audit "$custody_dir/$control.entire-transcript.txt"
done
record elapsed-only-output-equality 0 cmp "$custody_dir/original-positive.entire-stdout.txt" "$custody_dir/audit-timing-only.entire-stdout.txt"
record strict-clippy 0 timeout 120 cargo +1.99.1 clippy --manifest-path "$manifest" --release --offline --locked -j1 --all-targets --no-deps -- -D warnings
record format 0 timeout 120 cargo +1.99.1 fmt --manifest-path "$manifest" --all -- --check
record post-source-guard 0 sha256sum -c "$custody_dir/frozen-source.sha256"
record post-input-guard 0 sha256sum -c "$custody_dir/frozen-input.sha256"
record post-qualifier-guard 0 sha256sum -c "$custody_dir/frozen-qualifier.sha256"
date -u +%FT%TZ > "$custody_dir/actual-end.txt"
cat "$custody_dir/actual-phases.txt"
