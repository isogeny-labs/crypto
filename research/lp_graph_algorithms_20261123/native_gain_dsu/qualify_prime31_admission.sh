#!/usr/bin/env bash
# Fixed development controls for the ORIGINAL Prime31 admission example.
# No collector, mathematical implementation, or performance measurement here.
# Protocol: ../PRIME31_GROUP_ADMISSION_NATIVE_20261123.md
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../../.." && pwd)"
cd "$root"
artifact_dir="${PRIME31_QUALIFICATION_ARTIFACT_DIR:?set the qualification artifact directory}"
mkdir -p "$artifact_dir"
artifact_dir="$(cd "$artifact_dir" && pwd)"

# Capture each command without a pipeline that could hide its exit status.
run_capture() {
  local label="$1" status=0
  shift
  printf '%q ' "$@" >"$artifact_dir/$label.command"
  printf '\n' >>"$artifact_dir/$label.command"
  "$@" >"$artifact_dir/$label.stdout" 2>"$artifact_dir/$label.stderr" || status=$?
  printf '%s\n' "$status" >"$artifact_dir/$label.exit"
  printf '%s: exit %s\n' "$label" "$status"
  return "$status"
}

expect_rejection() {
  local label="$1" status=0
  shift
  run_capture "$label" "$@" || status=$?
  # Result-returning Rust main uses exit 1 for an ordinary input rejection.
  # A signal, timeout, or missing executable is not an accepted rejection.
  if [[ "$status" != 1 || -s "$artifact_dir/$label.stdout" ]]; then
    printf '%s: expected exit 1 and empty stdout, observed exit %s\n' "$label" "$status" >&2
    return 1
  fi
}

cli_checks() {
  local adapter="$artifact_dir/executables/prime31_lp_group_admission"
  local evaluator="$artifact_dir/executables/lp_graph_eval"
  local controls="$artifact_dir/controls"

  run_capture valid "$adapter" "$controls/valid.json"
  run_capture valid-byte-comparison cmp "$controls/expected.jsonl" "$artifact_dir/valid.stdout"
  run_capture negative-target "$adapter" "$controls/negative-target.json"
  run_capture negative-target-byte-comparison cmp "$controls/negative-target-expected.jsonl" "$artifact_dir/negative-target.stdout"
  expect_rejection invalid-final-row "$adapter" "$controls/late-invalid.json"
  expect_rejection malformed-json "$adapter" "$controls/malformed.json"
  expect_rejection oversized-input "$adapter" "$controls/oversized.json"
  expect_rejection missing-argument "$adapter"
  expect_rejection extra-argument "$adapter" "$controls/valid.json" unexpected

  run_capture graph-evaluation "$evaluator" \
    --input "$artifact_dir/valid.stdout" --modulus 2147446831 \
    --label planted-prime31-admission-correctness-only \
    --exact-forest --forest-max-rows 4096 --out "$artifact_dir/graph-evaluation.json"
  # Assert serialized native answers. All group/rank computation stays in Rust.
  # None of the serialized timing or performance fields is read below.
  run_capture graph-wire-assertions jq -e '
    .schema == "lp-graph-eval/v1" and .modulus == 2147446831 and .rows == 5
    and .forest_skip_reason == null
    and ([.forest_filter, .pivot_filter] | all(
      . != null and .refused == false and .replay_failures == 0
      and .exact_projected_rank.useful_small_rank == 2
      and .exact_projected_rank.augmented_rank == 2))
    and (.mst_filter != null and (
      (.mst_filter.refused == true and .mst_filter.exact_projected_rank == null)
      or (.mst_filter.refused == false and .mst_filter.replay_failures == 0
          and .mst_filter.exact_projected_rank.useful_small_rank == 2
          and .mst_filter.exact_projected_rank.augmented_rank == 2)))
  ' "$artifact_dir/graph-evaluation.json"
  if jq -e '.mst_filter.refused == true' "$artifact_dir/graph-evaluation.json" >/dev/null; then
    printf '%s\n' 'MST explicitly refused this control; no rank accepted for that arm.' >"$artifact_dir/mst-disposition.txt"
  else
    printf '%s\n' 'MST accepted: native useful/augmented rank 2 and zero source replay failures.' >"$artifact_dir/mst-disposition.txt"
  fi
  cat "$artifact_dir/mst-disposition.txt"
  printf '%s\n' 'Fixed CLI qualification completed; evaluator timing fields were not interpreted.'
}

if [[ "${1:-}" == --cli ]]; then
  [[ $# == 1 ]] || { printf '%s\n' 'unexpected CLI-check arguments' >&2; exit 2; }
  cli_checks
  exit 0
fi
[[ $# == 0 ]] || { printf '%s\n' 'unexpected qualification arguments' >&2; exit 2; }

# The workflow gives the entire build + qualification at most 300 seconds.
# Preserve the same bounds when this script is called by a development shell.
ulimit -v 4194304
export CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1
native_dir=research/prime31_ic_20261115/native
graph_dir=research/lp_graph_algorithms_20261123/native_gain_dsu
source_paths=(
  "$native_dir/Cargo.toml"
  "$native_dir/Cargo.lock"
  "$native_dir/lib.rs"
  "$native_dir/examples/prime31_lp_group_admission.rs"
  src/cryptanalysis/f4_fp_tower.rs
  src/cryptanalysis/sig_fp_tower.rs
  "$graph_dir/Cargo.toml"
  "$graph_dir/src/lib.rs"
  "$graph_dir/run.sh"
  "$graph_dir/qualify_prime31_admission.sh"
  examples/lp_graph_eval.rs
  src/cryptanalysis/large_prime_graph.rs
  src/cryptanalysis/large_prime_filter.rs
  src/cryptanalysis/large_prime_filter_mst.rs
  src/cryptanalysis/jv_large_prime_rank_bound.rs
  .github/workflows/graph-ic-gain-dsu-native.yml
)
run_capture graph-source-paths git ls-files -- src/cryptanalysis/large_prime_graph/ src/cryptanalysis/large_prime_filter/
mapfile -t graph_module_paths <"$artifact_dir/graph-source-paths.stdout"
[[ ${#graph_module_paths[@]} -gt 0 ]]
source_paths+=("${graph_module_paths[@]}")
toolchain_paths=()

finish() {
  local status=$?
  trap - EXIT TERM INT
  set +e
  if [[ -f "$artifact_dir/source-before.sha256" ]]; then
    sha256sum "${source_paths[@]}" >"$artifact_dir/source-after.sha256" 2>"$artifact_dir/source-after.stderr"
    if ! cmp -s "$artifact_dir/source-before.sha256" "$artifact_dir/source-after.sha256"; then
      printf '%s\n' 'Source/committed-lock identity changed or could not be rechecked.' >&2
      status=1
    fi
  fi
  if [[ -f "$artifact_dir/generated-graph-lock-before.sha256" ]]; then
    sha256sum "$graph_dir/Cargo.lock" >"$artifact_dir/generated-graph-lock-after.sha256" 2>"$artifact_dir/generated-graph-lock-after.stderr"
    if ! cmp -s "$artifact_dir/generated-graph-lock-before.sha256" "$artifact_dir/generated-graph-lock-after.sha256"; then
      printf '%s\n' 'Generated graph lock identity changed or could not be rechecked.' >&2
      status=1
    fi
  fi
  if [[ -f "$artifact_dir/toolchain-before.sha256" ]]; then
    sha256sum "${toolchain_paths[@]}" >"$artifact_dir/toolchain-after.sha256" 2>"$artifact_dir/toolchain-after.stderr"
    if ! cmp -s "$artifact_dir/toolchain-before.sha256" "$artifact_dir/toolchain-after.sha256"; then
      printf '%s\n' 'Native toolchain identity changed or could not be rechecked.' >&2
      status=1
    fi
  fi
  if [[ -f "$artifact_dir/executables-before.sha256" ]]; then
    (cd "$artifact_dir" && sha256sum executables/*) >"$artifact_dir/executables-after.sha256" 2>"$artifact_dir/executables-after.stderr"
    if ! cmp -s "$artifact_dir/executables-before.sha256" "$artifact_dir/executables-after.sha256"; then
      printf '%s\n' 'Copied executable identity changed or could not be rechecked.' >&2
      status=1
    fi
  fi
  if [[ -f "$artifact_dir/controls-before.sha256" ]]; then
    (cd "$artifact_dir" && sha256sum controls/*) >"$artifact_dir/controls-after.sha256" 2>"$artifact_dir/controls-after.stderr"
    if ! cmp -s "$artifact_dir/controls-before.sha256" "$artifact_dir/controls-after.sha256"; then
      printf '%s\n' 'Fixed control identity changed or could not be rechecked.' >&2
      status=1
    fi
  fi
  printf '%s\n' "$status" >"$artifact_dir/script.exit"
  exit "$status"
}
trap finish EXIT
trap 'exit 143' TERM
trap 'exit 130' INT

# Never overwrite a pre-call identity record from an earlier attempt.
[[ ! -e "$artifact_dir/source-before.sha256" && ! -e "$artifact_dir/executables-before.sha256" ]]
mkdir "$artifact_dir/executables" "$artifact_dir/controls" "$artifact_dir/locks"
git rev-parse HEAD >"$artifact_dir/checkout.txt"
run_capture rustc-version rustc --version --verbose
run_capture cargo-version cargo --version
run_capture rustc-path rustup which rustc
run_capture cargo-path rustup which cargo
toolchain_paths=("$(cat "$artifact_dir/rustc-path.stdout")" "$(cat "$artifact_dir/cargo-path.stdout")")
sha256sum "${toolchain_paths[@]}" >"$artifact_dir/toolchain-before.sha256"
chmod a-w "$artifact_dir/toolchain-before.sha256"
sha256sum "${source_paths[@]}" >"$artifact_dir/source-before.sha256"
chmod a-w "$artifact_dir/source-before.sha256"
install -m 444 "$native_dir/Cargo.lock" "$artifact_dir/locks/prime31-committed-Cargo.lock"
# This lock was generated by the existing graph test step. It is not frozen
# in the source tree and must never be presented as a committed pre-run lock.
test -f "$graph_dir/Cargo.lock"
install -m 444 "$graph_dir/Cargo.lock" "$artifact_dir/locks/graph-generated-Cargo.lock"
printf '%s\n' 'graph-generated-Cargo.lock: generated by preceding existing graph CI steps; not committed/frozen.' >"$artifact_dir/locks/PROVENANCE.txt"
sha256sum "$graph_dir/Cargo.lock" >"$artifact_dir/generated-graph-lock-before.sha256"
chmod a-w "$artifact_dir/generated-graph-lock-before.sha256"

run_capture admission-build cargo build --locked --release --jobs 1 \
  --manifest-path "$native_dir/Cargo.toml" --example prime31_lp_group_admission --message-format=json
adapter_path="$(jq -ser '
  [.[] | select(.reason == "compiler-artifact" and .target.name == "prime31_lp_group_admission"
                and .profile.test == false and .executable != null)]
  | if length == 1 then .[0].executable else error("expected exactly one admission executable") end
' "$artifact_dir/admission-build.stdout")"
run_capture admission-test-build cargo test --locked --release --jobs 1 \
  --manifest-path "$native_dir/Cargo.toml" --example prime31_lp_group_admission --no-run --message-format=json
test_path="$(jq -ser '
  [.[] | select(.reason == "compiler-artifact" and .target.name == "prime31_lp_group_admission"
                and .profile.test == true and .executable != null)]
  | if length == 1 then .[0].executable else error("expected exactly one admission test executable") end
' "$artifact_dir/admission-test-build.stdout")"
run_capture graph-metadata cargo metadata --no-deps --format-version 1 --manifest-path "$graph_dir/Cargo.toml"
graph_target="$(jq -er '.target_directory | select(type == "string" and length > 0)' "$artifact_dir/graph-metadata.stdout")"
graph_path="$graph_target/release/examples/lp_graph_eval"
test -x "$graph_path"
printf '%s\n' "$adapter_path" >"$artifact_dir/admission-executable-origin.txt"
printf '%s\n' "$test_path" >"$artifact_dir/test-executable-origin.txt"
printf '%s\n' "$graph_path" >"$artifact_dir/graph-executable-origin.txt"
install -m 555 "$adapter_path" "$artifact_dir/executables/prime31_lp_group_admission"
install -m 555 "$test_path" "$artifact_dir/executables/prime31_lp_group_admission-tests"
install -m 555 "$graph_path" "$artifact_dir/executables/lp_graph_eval"
(cd "$artifact_dir" && sha256sum executables/*) >"$artifact_dir/executables-before.sha256"
chmod a-w "$artifact_dir/executables-before.sha256"

run_capture native-tests env PRIME31_ADMISSION_CONTROL_DIR="$artifact_dir/controls" \
  timeout --signal=TERM --kill-after=5s 60s "$artifact_dir/executables/prime31_lp_group_admission-tests" --nocapture --test-threads=1
for control in valid.json expected.jsonl late-invalid.json negative-target.json negative-target-expected.jsonl; do
  test -s "$artifact_dir/controls/$control"
done
printf '{\n' >"$artifact_dir/controls/malformed.json"
head -c 8388609 /dev/zero >"$artifact_dir/controls/oversized.json"
chmod a-w "$artifact_dir/controls/"*
(cd "$artifact_dir" && sha256sum controls/*) >"$artifact_dir/controls-before.sha256"
chmod a-w "$artifact_dir/controls-before.sha256"
run_capture cli-suite timeout --signal=TERM --kill-after=5s 60s bash "$here/qualify_prime31_admission.sh" --cli
printf '%s\n' 'Native Prime31 group-admission development qualification completed.'
