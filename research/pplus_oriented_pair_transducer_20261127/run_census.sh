#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
study="$repo_root/research/pplus_oriented_pair_transducer_20261127"
manifest="$study/SOURCE_MANIFEST_2.json"
crate="$study/native/Cargo.toml"
evidence="$study/evidence"
source_commit="$(jq -er '.source_commit' "$manifest")"

cd "$repo_root"

if [[ -e "$evidence" ]]; then
  echo "run-census: refusing to overwrite $evidence" >&2
  exit 2
fi

if ! git merge-base --is-ancestor "$source_commit" HEAD; then
  echo "run-census: source commit $source_commit is not an ancestor of HEAD" >&2
  exit 2
fi

scripts/checkpoint.sh gate

jq -r '.files | to_entries[] | "\(.value)  \(.key)"' "$manifest" \
  | sha256sum --check --strict

cargo build --locked --release --manifest-path "$crate"

candidate="$study/native/target/release/pair-transducer"
verifier="$study/native/target/release/pair-transducer-verify"
analyzer="$study/native/target/release/pair-transducer-analyze"
for binary in "$candidate" "$verifier" "$analyzer"; do
  if [[ ! -x "$binary" ]]; then
    echo "run-census: missing release binary $binary" >&2
    exit 2
  fi
done

temporary="$(mktemp -d "$study/.census.XXXXXXXX")"
trap 'status=$?; if [[ $status -ne 0 ]]; then echo "run-census: failed evidence retained at '"$temporary"'" >&2; fi' EXIT

{
  echo "schema=isogeny-labs/pplus-oriented-pair-transducer-run/v1"
  echo "started_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "head=$(git rev-parse HEAD)"
  echo "source_commit=$source_commit"
  echo "source_manifest_sha256=$(sha256sum "$manifest" | cut -d' ' -f1)"
  echo "rustc=$(rustc --version)"
  echo "cargo=$(cargo --version)"
  echo "kernel=$(uname -srm)"
} > "$temporary/run-metadata.txt"

/usr/bin/time -f 'elapsed_seconds=%e\nmax_rss_kib=%M\nexit_status=%x' \
  -o "$temporary/candidate-time.txt" \
  "$candidate" > "$temporary/candidate.txt" 2> "$temporary/candidate.stderr"

/usr/bin/time -f 'elapsed_seconds=%e\nmax_rss_kib=%M\nexit_status=%x' \
  -o "$temporary/verifier-time.txt" \
  "$verifier" "$temporary/candidate.txt" \
  > "$temporary/verifier.txt" 2> "$temporary/verifier.stderr"

"$analyzer" "$temporary/candidate.txt" "$temporary/verifier.txt" \
  > "$temporary/analysis.json"
jq -e '.schema == "isogeny-labs/pplus-oriented-pair-transducer-analysis/v1"' \
  "$temporary/analysis.json" >/dev/null

echo "finished_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  >> "$temporary/run-metadata.txt"
(
  cd "$temporary"
  sha256sum \
    analysis.json \
    candidate-time.txt \
    candidate.stderr \
    candidate.txt \
    run-metadata.txt \
    verifier-time.txt \
    verifier.stderr \
    verifier.txt \
    > SHA256SUMS
  sha256sum --check --strict SHA256SUMS
)

mv "$temporary" "$evidence"
trap - EXIT
echo "run-census: evidence committed locally at $evidence"
