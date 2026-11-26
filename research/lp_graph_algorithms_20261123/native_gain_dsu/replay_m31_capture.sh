#!/usr/bin/env bash
# Replay one frozen, authentic M31 capture through the original native consumers.
# Thin orchestration only: no collector, mathematical implementation, or timing
# interpretation. Protocol: ../M31_CAPTURE_REPLAY_20261124.md
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../../.." && pwd)"
cd "$root"
qualification_dir="${PRIME31_QUALIFICATION_ARTIFACT_DIR:?set the preceding qualification artifact directory}"
qualification_dir="$(cd "$qualification_dir" && pwd)"
artifact_dir="${M31_CAPTURE_REPLAY_ARTIFACT_DIR:?set the M31 replay artifact directory}"
mkdir -p "$artifact_dir"
artifact_dir="$(cd "$artifact_dir" && pwd)"
[[ $# == 0 ]] || { printf '%s\n' 'unexpected replay arguments' >&2; exit 2; }

ulimit -v 4194304
export CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1
producer_commit=577d6c2c1c4a527604d965e6fb62c44df7902f36
producer_study=research/prime31_graph_ecdlp_20261123
producer_evidence="$producer_study/evidence/mitm-b677-control-01"
native_dir=research/prime31_ic_20261115/native
graph_dir=research/lp_graph_algorithms_20261123/native_gain_dsu
fixture_path=research/prime31_four_summand_20261122/native/b677_fixture.rs
csv_path=research/prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv
input="$artifact_dir/producer/$producer_evidence/ten-query.json"
source_paths=()
input_paths=()
toolchain_paths=()
lock_paths=()
executable_paths=()
derived_paths=()
prefix_paths=()
mutation_paths=()

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

snapshot() {
  local name="$1"
  shift
  [[ $# -gt 0 ]]
  run_capture "$name" sha256sum "$@" || return $?
  install -m 444 "$artifact_dir/$name.stdout" "$artifact_dir/$name.sha256"
}

expect_rejection() {
  local label="$1" status=0
  shift
  run_capture "$label" timeout --signal=TERM --kill-after=5s 60s "$@" || status=$?
  # Result-returning Rust main rejects ordinary invalid input with exit 1.
  # A timeout, missing executable, or signal is not a successful rejection.
  if [[ "$status" != 1 || -s "$artifact_dir/$label.stdout" ]]; then
    printf '%s: expected exit 1 and empty capture stdout, observed exit %s\n' "$label" "$status" >&2
    return 1
  fi
}

finish() {
  local status=$? name variable
  trap - EXIT TERM INT
  set +e
  for name in source input toolchain lock executable derived prefix mutation; do
    if [[ -f "$artifact_dir/$name-before.sha256" ]]; then
      variable="${name}_paths"
      local -n paths="$variable"
      if ! snapshot "$name-after" "${paths[@]}"; then
        status=1
      fi
      if ! cmp -s "$artifact_dir/$name-before.sha256" "$artifact_dir/$name-after.sha256"; then
        printf '%s identity changed or could not be rechecked.\n' "$name" >&2
        status=1
      fi
    fi
  done
  printf '%s\n' "$status" >"$artifact_dir/script.exit"
  exit "$status"
}
trap finish EXIT
trap 'exit 143' TERM
trap 'exit 130' INT

# Never overwrite identities or native output from an earlier attempt.
[[ ! -e "$artifact_dir/source-before.sha256" && ! -e "$artifact_dir/executables" ]]
mkdir "$artifact_dir/executables" "$artifact_dir/locks" "$artifact_dir/derived" \
  "$artifact_dir/prefixes" "$artifact_dir/mutations"
mkdir -p "$artifact_dir/producer/$producer_evidence" "$artifact_dir/producer/$producer_study/native"
run_capture checkout git rev-parse HEAD
install -m 444 "$artifact_dir/checkout.stdout" "$artifact_dir/checkout.txt"
printf '%s\n' "$producer_commit" >"$artifact_dir/producer-commit.txt"

# Read only this exact producer revision. No checkout, submodule traversal, or
# invocation of its collector occurs. Preserve only the four named files.
run_capture producer-fetch git -c submodule.recurse=false fetch \
  --no-tags --no-recurse-submodules --depth=1 origin "$producer_commit"
run_capture producer-fetched-commit git rev-parse FETCH_HEAD
[[ "$(cat "$artifact_dir/producer-fetched-commit.stdout")" == "$producer_commit" ]]
for file in ten-query.json ten-query.stderr sources.sha256; do
  run_capture "producer-$file" git show "$producer_commit:$producer_evidence/$file"
  install -m 444 "$artifact_dir/producer-$file.stdout" "$artifact_dir/producer/$producer_evidence/$file"
done
run_capture producer-mitm-source git show "$producer_commit:$producer_study/native/mitm_four.rs"
install -m 444 "$artifact_dir/producer-mitm-source.stdout" "$artifact_dir/producer/$producer_study/native/mitm_four.rs"
run_capture producer-input-blob git rev-parse "$producer_commit:$producer_evidence/ten-query.json"
[[ "$(cat "$artifact_dir/producer-input-blob.stdout")" == 7fd27b9888058399d8814cef373d373243e6a1e5 ]]
run_capture producer-input-size wc -c "$input"
read -r input_size input_filename <"$artifact_dir/producer-input-size.stdout"
[[ "$input_size" == 165511 ]]

printf '%s  %s\n' \
  e2407b695d4df7a7c9d71eae00895a643aebe3afab50d1c338e58778884dabab "$input" \
  6b97389197bcebc85ace5b47096dbc590475a1fd1021590715e66aa88f13fc12 "$artifact_dir/producer/$producer_study/native/mitm_four.rs" \
  3f491defbc7aa1643192db4d0c7144afcec1f84a41f03489355a866459329624 "$fixture_path" \
  fc484615394ee423190747d05001cea526580fb0df0279fbcf789ab6cafc0c46 "$csv_path" \
  52b30e41478b0306603db7d09bcb17377cae64efcfd78d4c34db7bf91325a867 "$native_dir/lib.rs" \
  >"$artifact_dir/frozen-expected.sha256"
chmod a-w "$artifact_dir/frozen-expected.sha256"
run_capture frozen-identities sha256sum -c "$artifact_dir/frozen-expected.sha256"
cat >"$artifact_dir/expected-producer-sources.sha256" <<'HASHES'
6b97389197bcebc85ace5b47096dbc590475a1fd1021590715e66aa88f13fc12  research/prime31_graph_ecdlp_20261123/native/mitm_four.rs
3f491defbc7aa1643192db4d0c7144afcec1f84a41f03489355a866459329624  research/prime31_four_summand_20261122/native/b677_fixture.rs
52b30e41478b0306603db7d09bcb17377cae64efcfd78d4c34db7bf91325a867  research/prime31_ic_20261115/native/lib.rs
HASHES
run_capture producer-source-list cmp "$artifact_dir/expected-producer-sources.sha256" \
  "$artifact_dir/producer/$producer_evidence/sources.sha256"
input_paths=("$input" "$artifact_dir/producer/$producer_evidence/ten-query.stderr" \
  "$artifact_dir/producer/$producer_evidence/sources.sha256" \
  "$artifact_dir/producer/$producer_study/native/mitm_four.rs")
snapshot input-before "${input_paths[@]}"

# The inherited qualification must have passed, and its source and copied
# binaries must still match their original identities before reuse.
[[ "$(cat "$qualification_dir/qualification.exit")" == 0 ]]
[[ "$(cat "$qualification_dir/script.exit")" == 0 ]]
run_capture qualified-source-stable cmp "$qualification_dir/source-before.sha256" "$qualification_dir/source-after.sha256"
run_capture qualified-binaries-stable cmp "$qualification_dir/executables-before.sha256" "$qualification_dir/executables-after.sha256"
run_capture qualified-current-source sha256sum -c "$qualification_dir/source-before.sha256"
run_capture qualified-current-binaries bash -c 'cd "$1" && sha256sum -c executables-before.sha256' bash "$qualification_dir"
run_capture qualified-toolchain-stable cmp "$qualification_dir/toolchain-before.sha256" "$qualification_dir/toolchain-after.sha256"
run_capture qualified-current-toolchain sha256sum -c "$qualification_dir/toolchain-before.sha256"
run_capture qualified-graph-lock-stable cmp "$qualification_dir/generated-graph-lock-before.sha256" "$qualification_dir/generated-graph-lock-after.sha256"
run_capture qualified-current-graph-lock sha256sum -c "$qualification_dir/generated-graph-lock-before.sha256"
run_capture qualified-checkout cmp "$qualification_dir/checkout.txt" "$artifact_dir/checkout.txt"

source_paths=(
  "$native_dir/Cargo.toml" "$native_dir/Cargo.lock" "$native_dir/lib.rs"
  "$native_dir/examples/prime31_b677_capture.rs"
  "$native_dir/examples/prime31_lp_group_admission.rs"
  "$fixture_path" "$csv_path"
  src/cryptanalysis/f4_fp_tower.rs src/cryptanalysis/sig_fp_tower.rs
  "$graph_dir/Cargo.toml" "$graph_dir/src/lib.rs" "$graph_dir/src/cryptanalysis.rs"
  "$graph_dir/run.sh" "$graph_dir/qualify_prime31_admission.sh" "$graph_dir/replay_m31_capture.sh"
  research/lp_graph_algorithms_20261123/M31_CAPTURE_REPLAY_20261124.md
  examples/lp_graph_eval.rs src/cryptanalysis/large_prime_graph.rs
  src/cryptanalysis/large_prime_filter.rs src/cryptanalysis/large_prime_filter_mst.rs
  src/cryptanalysis/jv_large_prime_rank_bound.rs .github/workflows/graph-ic-gain-dsu-native.yml
)
run_capture graph-source-paths git ls-files -- src/cryptanalysis/large_prime_graph/ src/cryptanalysis/large_prime_filter/
mapfile -t graph_module_paths <"$artifact_dir/graph-source-paths.stdout"
[[ ${#graph_module_paths[@]} -gt 0 ]]
source_paths+=("${graph_module_paths[@]}")
snapshot source-before "${source_paths[@]}"
run_capture rustc-version rustc --version --verbose
run_capture cargo-version cargo --version
run_capture rustc-path rustup which rustc
run_capture cargo-path rustup which cargo
toolchain_paths=("$(cat "$artifact_dir/rustc-path.stdout")" "$(cat "$artifact_dir/cargo-path.stdout")")
snapshot toolchain-before "${toolchain_paths[@]}"
install -m 444 "$native_dir/Cargo.lock" "$artifact_dir/locks/prime31-committed-Cargo.lock"
install -m 444 "$graph_dir/Cargo.lock" "$artifact_dir/locks/graph-generated-Cargo.lock"
printf '%s\n' 'graph-generated-Cargo.lock: generated by preceding graph CI steps; not committed or frozen before those steps.' >"$artifact_dir/locks/PROVENANCE.txt"
lock_paths=("$native_dir/Cargo.lock" "$graph_dir/Cargo.lock" \
  "$artifact_dir/locks/prime31-committed-Cargo.lock" "$artifact_dir/locks/graph-generated-Cargo.lock")
snapshot lock-before "${lock_paths[@]}"

run_capture converter-build cargo build --locked --release --jobs 1 \
  --manifest-path "$native_dir/Cargo.toml" --example prime31_b677_capture --message-format=json
run_capture converter-artifact jq -ser '
  [.[] | select(.reason == "compiler-artifact" and .target.name == "prime31_b677_capture"
                and .profile.test == false and .executable != null)]
  | if length == 1 then .[0].executable else error("expected exactly one converter executable") end
' "$artifact_dir/converter-build.stdout"
converter_path="$(cat "$artifact_dir/converter-artifact.stdout")"
printf '%s\n' "$converter_path" >"$artifact_dir/converter-executable-origin.txt"
printf '%s\n' "$qualification_dir/executables/prime31_lp_group_admission" >"$artifact_dir/admission-executable-origin.txt"
printf '%s\n' "$qualification_dir/executables/lp_graph_eval" >"$artifact_dir/graph-executable-origin.txt"
install -m 555 "$converter_path" "$artifact_dir/executables/prime31_b677_capture"
install -m 555 "$qualification_dir/executables/prime31_lp_group_admission" "$artifact_dir/executables/prime31_lp_group_admission"
install -m 555 "$qualification_dir/executables/lp_graph_eval" "$artifact_dir/executables/lp_graph_eval"
executable_paths=("$artifact_dir/executables/prime31_b677_capture" \
  "$artifact_dir/executables/prime31_lp_group_admission" "$artifact_dir/executables/lp_graph_eval" \
  "$qualification_dir/executables/prime31_lp_group_admission" "$qualification_dir/executables/lp_graph_eval")
snapshot executable-before "${executable_paths[@]}"
run_capture reused-admission-identical cmp "$qualification_dir/executables/prime31_lp_group_admission" "$artifact_dir/executables/prime31_lp_group_admission"
run_capture reused-evaluator-identical cmp "$qualification_dir/executables/lp_graph_eval" "$artifact_dir/executables/lp_graph_eval"

converter="$artifact_dir/executables/prime31_b677_capture"
adapter="$artifact_dir/executables/prime31_lp_group_admission"
evaluator="$artifact_dir/executables/lp_graph_eval"
audit="$artifact_dir/derived/conversion-audit.json"
run_capture conversion timeout --signal=TERM --kill-after=5s 60s "$converter" "$input" --receipt "$audit"
install -m 444 "$artifact_dir/conversion.stdout" "$artifact_dir/derived/capture.json"
chmod a-w "$audit"
# These compare serialized native answers with preregistered literals. All
# elliptic-curve, arity classification, and rank computation remains in Rust.
run_capture conversion-audit-wire jq -e '
  .schema == "prime31-b677-capture-audit/v1"
  and .counts.queries == 10 and .counts.source_rows == 645
  and .counts.replayed_rows == 645 and .counts.kept_rows == 613
  and .counts.excluded_rows == 32 and .counts.source_points == 677
  and .counts.small_points == 509 and .counts.large_points == 168
  and .counts.classes_by_base_terms == [2,30,138,276,199]
  and .counts.kept_by_lp_arity == [199,276,138]
  and ([.query_joins[].kept_rows] == [50,51,70,61,68,68,62,54,66,63])
  and (.excluded_rows | length) == 32
  and (.point_tables.small_points | length) == 509
  and (.point_tables.large_points | length) == 168
  and .arithmetic.counters_are_complete_cold_cost == false
' "$audit"
run_capture converted-capture-wire jq -e '
  .schema == "prime31-lp-capture/v1" and .target == [976979610,1228979761]
  and (.small_points | length) == 509 and (.large_points | length) == 168
  and (.rows | length) == 613
' "$artifact_dir/derived/capture.json"
run_capture point-tables jq '.point_tables' "$audit"
install -m 444 "$artifact_dir/point-tables.stdout" "$artifact_dir/derived/point-tables.json"
run_capture admission timeout --signal=TERM --kill-after=5s 60s "$adapter" "$artifact_dir/derived/capture.json"
install -m 444 "$artifact_dir/admission.stdout" "$artifact_dir/derived/admitted.jsonl"
run_capture admitted-wire jq -se 'length == 613 and all(.[];
  has("id") and has("small") and has("large") and has("rhs"))' "$artifact_dir/derived/admitted.jsonl"
derived_paths=("$artifact_dir/derived/capture.json" "$audit" \
  "$artifact_dir/derived/point-tables.json" "$artifact_dir/derived/admitted.jsonl")
snapshot derived-before "${derived_paths[@]}"

# Preserve each fixed source-order prefix and hash it before any evaluator call.
for rows in 256 512 613; do
  run_capture "prefix-$rows-create" head -n "$rows" "$artifact_dir/derived/admitted.jsonl"
  install -m 444 "$artifact_dir/prefix-$rows-create.stdout" "$artifact_dir/prefixes/first-$rows.jsonl"
  prefix_paths+=("$artifact_dir/prefixes/first-$rows.jsonl")
done
run_capture full-prefix-identical cmp "$artifact_dir/derived/admitted.jsonl" "$artifact_dir/prefixes/first-613.jsonl"
snapshot prefix-before "${prefix_paths[@]}"
for rows in 256 512 613; do
  report="$artifact_dir/graph-$rows.json"
  run_capture "graph-$rows" timeout --signal=TERM --kill-after=5s 60s "$evaluator" \
    --input "$artifact_dir/prefixes/first-$rows.jsonl" --modulus 2147446831 \
    --label "m31-ten-query-retained-first-$rows" --exact-forest --forest-max-rows 4096 --out "$report"
  # Validate schema and replay/refusal disposition only. Do not prescribe rank
  # or read any timing field; retain the native rank/nonzero counters verbatim.
  run_capture "graph-$rows-wire" jq -e --argjson expected_rows "$rows" '
    .schema == "lp-graph-eval/v1" and .modulus == 2147446831 and .rows == $expected_rows
    and .forest_skip_reason == null and .rank.exact_for_stream == true
    and .rank.dense_matches == true
    and ([.forest_filter, .pivot_filter, .mst_filter] | all(
      . != null and ((.refused == true and .exact_projected_rank == null)
      or (.refused == false and .replay_failures == 0 and .exact_projected_rank != null))))
    and .pivot_filter.refused == false
  ' "$report"
  run_capture "graph-$rows-counts" jq '{
    schema,modulus,label,rows,lp_columns,incidences,arity_histogram,
    rank:{two_entry_rows:.rank.two_entry_rows,exact_for_stream:.rank.exact_for_stream,
          gain_dsu:.rank.gain_dsu,dense_rank:.rank.dense_rank,dense_matches:.rank.dense_matches}
  }' "$report"
  run_capture "graph-$rows-arms" jq '[
    {name:"forest",arm:.forest_filter},{name:"pivot",arm:.pivot_filter},{name:"mst",arm:.mst_filter}
    | {name,refused:.arm.refused,lp_rank:.arm.lp_rank,lp_left_nullity:.arm.lp_left_nullity,
       replayed:.arm.replayed,replay_failures:.arm.replay_failures,
       residual_columns:.arm.residual_columns,residual_nonzeros:.arm.residual_nonzeros,
       source_terms:.arm.source_terms,max_row_weight:.arm.max_row_weight,
       exact_projected_rank:.arm.exact_projected_rank,report:.arm.report,mst_stats:.arm.mst_stats}
  ]' "$report"
  if jq -e '.mst_filter.refused == true' "$report" >/dev/null; then
    printf '%s\n' 'MST explicitly refused this prefix; no MST rank accepted.' >"$artifact_dir/graph-$rows-mst-disposition.txt"
  else
    printf '%s\n' 'MST accepted this prefix with zero source replay failures; native ranks retained in the report.' >"$artifact_dir/graph-$rows-mst-disposition.txt"
  fi
done

# Three literal corruptions of this same M31 capture, never a new small-field
# workload. The converter must refuse them before emitting capture stdout.
run_capture mutation-field-create jq '.curve.p = 103' "$input"
run_capture mutation-query-create jq '.queries[0].a = 732241240' "$input"
run_capture mutation-sign-create jq '.relation_rows[0].terms[0][1] = 1' "$input"
for mutation in field query sign; do
  install -m 444 "$artifact_dir/mutation-$mutation-create.stdout" "$artifact_dir/mutations/$mutation.json"
  mutation_paths+=("$artifact_dir/mutations/$mutation.json")
done
snapshot mutation-before "${mutation_paths[@]}"
for mutation in field query sign; do
  expect_rejection "reject-$mutation" "$converter" "$artifact_dir/mutations/$mutation.json" \
    --receipt "$artifact_dir/mutations/$mutation-audit.json"
done
printf '%s\n' 'M31 retained-input replay completed; evaluator timing fields were not interpreted.'
