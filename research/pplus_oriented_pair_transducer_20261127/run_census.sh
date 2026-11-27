#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
study="$repo_root/research/pplus_oriented_pair_transducer_20261127"
manifest="$study/SOURCE_MANIFEST_3.json"
crate="$study/native/Cargo.toml"
evidence="$study/evidence"
source_commit="$(jq -er '.source_commit' "$manifest")"
checkpoint_mode="${PPLUS_CHECKPOINT_MODE:-git}"

cd "$repo_root"

if [[ -e "$evidence" ]]; then
  echo "run-census: refusing to overwrite $evidence" >&2
  exit 2
fi

if ! git merge-base --is-ancestor "$source_commit" HEAD; then
  echo "run-census: source commit $source_commit is not an ancestor of HEAD" >&2
  exit 2
fi

case "$checkpoint_mode" in
  git)
    checkpoint_rel="research/pplus_oriented_pair_transducer_20261127/GIT_CHECKPOINT.md"
    checkpoint="$repo_root/$checkpoint_rel"
    if [[ ! -f "$checkpoint" ]] || ! git ls-files --error-unmatch "$checkpoint_rel" >/dev/null 2>&1; then
      echo "run-census: missing tracked Git checkpoint $checkpoint" >&2
      exit 2
    fi
    if [[ -n "$(git status --porcelain --untracked-files=all)" ]]; then
      echo "run-census: Git checkpoint requires a clean worktree and index" >&2
      exit 2
    fi
    branch="$(git symbolic-ref --quiet --short HEAD)" || {
      echo "run-census: Git checkpoint requires a named branch" >&2
      exit 2
    }
    local_head="$(git rev-parse HEAD)"
    remote_head="$(git ls-remote --exit-code --heads origin "$branch" | awk 'NR == 1 { print $1 }')"
    if [[ -z "$remote_head" ]] || [[ "$local_head" != "$remote_head" ]]; then
      echo "run-census: local $local_head is not exactly pushed as origin/$branch" >&2
      exit 2
    fi
    ;;
  s3)
    scripts/checkpoint.sh gate
    ;;
  *)
    echo "run-census: unknown PPLUS_CHECKPOINT_MODE=$checkpoint_mode" >&2
    exit 2
    ;;
esac

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

temporary="$(mktemp -d "$study/.census.XXXXXXXX")"
trap 'status=$?; if [[ $status -ne 0 ]]; then echo "run-census: failed evidence retained at '"$temporary"'" >&2; fi' EXIT

{
  echo "schema=isogeny-labs/pplus-oriented-pair-transducer-run/v1"
  echo "started_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "head=$(git rev-parse HEAD)"
  echo "source_commit=$source_commit"
  echo "checkpoint_mode=$checkpoint_mode"
  echo "source_manifest_sha256=$(sha256sum "$manifest" | cut -d' ' -f1)"
  echo "rustc=$(rustc --version)"
  echo "cargo=$(cargo --version)"
  echo "kernel=$(uname -srm)"
} > "$temporary/run-metadata.txt"

run_with_diagnostics candidate \
  "$temporary/candidate.txt" "$temporary/candidate.stderr" \
  "$candidate"

run_with_diagnostics verifier \
  "$temporary/verifier.txt" "$temporary/verifier.stderr" \
  "$verifier" "$temporary/candidate.txt"

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
