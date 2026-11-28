#!/usr/bin/env bash
set -euo pipefail
set -C
umask 022
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
for required in jq timeout sha256sum; do command -v "$required" >/dev/null; done
rustc_path=$(command -v -- "${RUSTC:-rustc}")
[[ -x "$rustc_path" ]] || { printf '%s\n' 'Rust compiler is not executable' >&2; exit 127; }
attempt=${1:-attempt-001}
[[ "$attempt" =~ ^attempt-[0-9]{3}$ ]] || { echo "Expected attempt-NNN" >&2; exit 2; }
run_dir="results/$attempt"
mkdir -- "$run_dir"
mkdir -- "$run_dir/frozen" "$run_dir/build"
cp -R -- src sources "$run_dir/frozen/"
cp -- QUALIFIER.md run.sh "$run_dir/frozen/"
(
    cd -- "$run_dir/frozen"
    sha256sum QUALIFIER.md run.sh src/*.rs sources/* > source-manifest.sha256
    chmod a-w QUALIFIER.md run.sh source-manifest.sha256 src/* sources/*
)
frozen_hash=$(sha256sum "$run_dir/frozen/source-manifest.sha256" | cut -d ' ' -f 1)
started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
jq -n --arg attempt "$attempt" --arg created "$started" --arg source_manifest_sha256 "$frozen_hash" --arg compiler "$rustc_path" \
    '{schema:"isogeny.shared-terminal-build/v1",attempt:$attempt,created_utc:$created,source_manifest_sha256:$source_manifest_sha256,qualifier:"frozen/QUALIFIER.md",compiler:$compiler,compile_flags:["--edition=2021","-O","-C","codegen-units=1"],test_flag:"--test",resource_caps:{virtual_memory_kib:2097152,per_call_wall_seconds:45,kill_grace_seconds:2,test_threads:1},scope:"deterministic archived-fixture correctness replay; no new relation search or timing claim",fixture:{p:2147483647,r:2147446831,a:847514846,b:2087444724,G:[625037715,2100627251],Q:[1477799919,824532935]},historical_source_commit:"90a0314f5be63d5ca56465a884091f4be0f114fd",seed:{hex:"4150535f50523331",standing:"inherited historical replay seed; not newly preregistered"},historical_indices:[437,1247,879],historical_signs:[-1,1,-1],relation_search_calls:0}' > "$run_dir/build-manifest.json"
stage=build
post_manifest() {
    code=$?
    trap - EXIT
    set +e
    finished=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    (cd -- "$run_dir" && sha256sum build-manifest.json compiler.sha256 toolchain.txt frozen/source-manifest.sha256 > retained-output-hashes.sha256)
    for f in pre-execution-manifest.json compile-main.stdout compile-main.stderr compile-tests.stdout compile-tests.stderr tests.stdout tests.stderr result.json gate.stderr exit-code.txt; do
        if [[ -f "$run_dir/$f" ]]; then (cd -- "$run_dir" && sha256sum "$f") >> "$run_dir/retained-output-hashes.sha256"; fi
    done
    jq -n --arg attempt "$attempt" --arg finished "$finished" --arg stage "$stage" --argjson exit_code "$code" \
        '{schema:"isogeny.shared-terminal-post/v1",attempt:$attempt,finished_utc:$finished,last_stage:$stage,wrapper_exit_code:$exit_code,status:(if $exit_code==0 then "pass" else "fail-retained" end),retained_hashes:"retained-output-hashes.sha256",performance_claim:false}' > "$run_dir/post-execution-manifest.json"
    exit "$code"
}
trap post_manifest EXIT
"$rustc_path" --version --verbose > "$run_dir/toolchain.txt"
sha256sum "$rustc_path" > "$run_dir/compiler.sha256"
ulimit -v 2097152
timeout -k 2s 45s "$rustc_path" --edition=2021 -O -C codegen-units=1 "$run_dir/frozen/src/main.rs" -o "$run_dir/build/shared_terminal_gate" > "$run_dir/compile-main.stdout" 2> "$run_dir/compile-main.stderr"
timeout -k 2s 45s "$rustc_path" --edition=2021 -O -C codegen-units=1 --test "$run_dir/frozen/src/main.rs" -o "$run_dir/build/shared_terminal_tests" > "$run_dir/compile-tests.stdout" 2> "$run_dir/compile-tests.stderr"
sha256sum "$run_dir/build/shared_terminal_gate" "$run_dir/build/shared_terminal_tests" > "$run_dir/binaries.sha256"
pre_created=$(date -u +%Y-%m-%dT%H:%M:%SZ)
jq -n --arg attempt "$attempt" --arg created "$pre_created" --arg source_manifest_sha256 "$frozen_hash" \
    '{schema:"isogeny.shared-terminal-pre-execution/v1",attempt:$attempt,created_utc:$created,source_manifest_sha256:$source_manifest_sha256,qualifier:"frozen/QUALIFIER.md",binaries:"binaries.sha256",commands:["timeout -k 2s 45s build/shared_terminal_tests --test-threads=1","timeout -k 2s 45s build/shared_terminal_gate"],input_policy:"fixed historical public Q, historical source seed, indices and signs; expected scalar used only after reconstructed relation",relation_search_calls:0,mutations:"enumerated in the frozen qualifier",stop_on_first_failed_call:true,all_prior_failures_retained:true,performance_claim:false}' > "$run_dir/pre-execution-manifest.json"
chmod a-w "$run_dir/build-manifest.json" "$run_dir/pre-execution-manifest.json" "$run_dir/binaries.sha256"
stage=tests
timeout -k 2s 45s "$run_dir/build/shared_terminal_tests" --test-threads=1 > "$run_dir/tests.stdout" 2> "$run_dir/tests.stderr"
stage=gate
timeout -k 2s 45s "$run_dir/build/shared_terminal_gate" > "$run_dir/result.json" 2> "$run_dir/gate.stderr"
stage=json-validation
jq -e '.status=="pass" and .all_checks_pass==true and (.checks|all(.pass==true)) and .archival_source_replay.relation_search_calls==0' "$run_dir/result.json" > "$run_dir/json-validation.stdout"
printf '%s\n' 0 > "$run_dir/exit-code.txt"
stage=complete
