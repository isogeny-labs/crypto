#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
study="$repo_root/research/pplus_dickson_norm_gcd_20261127"
manifest="$study/SOURCE_MANIFEST.json"
crate="$study/native/Cargo.toml"
evidence="$study/evidence"

cd "$repo_root"

if [[ ! -f "$manifest" ]]; then
  echo "run-census: missing active source manifest $manifest" >&2
  exit 2
fi
if [[ -e "$evidence" ]]; then
  echo "run-census: refusing to overwrite $evidence" >&2
  exit 2
fi

source_commit="$(jq -er '.source_commit' "$manifest")"
if ! git merge-base --is-ancestor "$source_commit" HEAD; then
  echo "run-census: source commit $source_commit is not an ancestor of HEAD" >&2
  exit 2
fi
if [[ -n "$(git status --porcelain --untracked-files=all)" ]]; then
  echo "run-census: pushed-Git checkpoint requires a clean worktree and index" >&2
  exit 2
fi
branch="$(git symbolic-ref --quiet --short HEAD)" || {
  echo "run-census: pushed-Git checkpoint requires a named branch" >&2
  exit 2
}
local_head="$(git rev-parse HEAD)"
remote_head="$(git ls-remote --exit-code --heads origin "$branch" | awk 'NR == 1 { print $1 }')"
if [[ -z "$remote_head" || "$local_head" != "$remote_head" ]]; then
  echo "run-census: local $local_head is not exactly pushed as origin/$branch" >&2
  exit 2
fi

check_sources() {
  jq -r '.files | to_entries[] | "\(.value)  \(.key)"' "$manifest" \
    | sha256sum --check --strict
}

check_sources
cargo build --locked --release --bins --manifest-path "$crate"
check_sources

candidate="$study/native/target/release/dickson-norm-gcd"
verifier="$study/native/target/release/dickson-norm-gcd-verify"
analyzer="$study/native/target/release/dickson-norm-gcd-analyze"
for binary in "$candidate" "$verifier" "$analyzer"; do
  if [[ ! -x "$binary" ]]; then
    echo "run-census: missing release binary $binary" >&2
    exit 2
  fi
done

temporary="$(mktemp -d "$study/.census.XXXXXXXX")"
trap 'status=$?; if [[ $status -ne 0 ]]; then echo "run-census: failed evidence retained at '"$temporary"'" >&2; fi' EXIT

run_with_diagnostics() {
  local label="$1"
  local stdout_path="$2"
  local stderr_path="$3"
  shift 3
  local started_ns finished_ns status
  started_ns="$(date +%s%N)"
  set +e
  "$@" > "$stdout_path" 2> "$stderr_path"
  status=$?
  set -e
  finished_ns="$(date +%s%N)"
  {
    echo "elapsed_nanoseconds=$((finished_ns - started_ns))"
    echo "max_rss_kib=unavailable"
    echo "exit_status=$status"
  } > "$temporary/$label-time.txt"
  return "$status"
}

{
  echo "schema=isogeny-labs/pplus-dickson-norm-gcd-run/v1"
  echo "started_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "head=$local_head"
  echo "branch=$branch"
  echo "source_commit=$source_commit"
  echo "checkpoint_mode=pushed-git"
  echo "source_manifest_sha256=$(sha256sum "$manifest" | cut -d' ' -f1)"
  echo "seed=none"
  echo "rustc=$(rustc --version)"
  echo "cargo=$(cargo --version)"
  echo "kernel=$(uname -srm)"
} > "$temporary/run-metadata.txt"

run_with_diagnostics candidate \
  "$temporary/candidate.txt" "$temporary/candidate.stderr" \
  env PPLUS_NORM_GCD_PRODUCTION=1 "$candidate" --max-depth 6

run_with_diagnostics verifier \
  "$temporary/verifier.txt" "$temporary/verifier.stderr" \
  "$verifier" --candidate "$temporary/candidate.txt"

run_with_diagnostics analyzer \
  "$temporary/analysis.json" "$temporary/analyzer.stderr" \
  "$analyzer" --candidate "$temporary/candidate.txt" \
    --verifier "$temporary/verifier.txt"

jq -e '
  .schema == "isogeny-labs/pplus-dickson-norm-gcd-analysis/v1" and
  .correctness_pass == true and
  .complete_ladder == true and
  (.depths | length) == 6 and
  .composition_status == "absent-two-leaf-primitive-only"
' "$temporary/analysis.json" >/dev/null

echo "finished_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  >> "$temporary/run-metadata.txt"
(
  cd "$temporary"
  sha256sum \
    analysis.json \
    analyzer-time.txt \
    analyzer.stderr \
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
