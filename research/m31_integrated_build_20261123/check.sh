#!/usr/bin/env bash
# Bounded local development gates only; this does not launch a research panel.
set -uo pipefail

repo_root=$(cd -- "$(dirname -- "$0")/../.." && pwd)
cd -- "$repo_root" || exit 2
output_dir=${1:-"research/m31_integrated_build_20261123/evidence/check-$(date -u +%Y%m%dT%H%M%SZ)"}
cargo_bin=${CARGO:-cargo}
gate_seconds=${BUILD_TIMEOUT_SECONDS:-600}
case "$gate_seconds" in ''|*[!0-9]*) printf 'BUILD_TIMEOUT_SECONDS must be a positive integer\n' >&2; exit 2;; esac
if ((gate_seconds == 0)); then exit 2; fi
if ! mkdir -- "$output_dir"; then
    printf 'Evidence directory must be new: %s\n' "$output_dir" >&2
    exit 2
fi
output_dir=$(cd -- "$output_dir" && pwd)

{
    git rev-parse HEAD
    git status --short --untracked-files=normal
    "$cargo_bin" --version
    "${RUSTC:-rustc}" --version
    uname -srm
    printf 'profile=release jobs=1 test_threads=1 timeout_seconds=%s\n' "$gate_seconds"
} > "$output_dir/environment.txt" 2>&1

offline_args=()
if [[ ${OFFLINE:-0} == 1 ]]; then offline_args+=(--offline); fi
overall=0
run_gate() {
    local name=$1 manifest=$2 gate_status
    shift 2
    local command=("$cargo_bin" test "${offline_args[@]}" --locked --release
        --manifest-path "$manifest" "$@" -j1 -- --test-threads=1)
    printf '%q ' timeout "$gate_seconds" "${command[@]}" > "$output_dir/$name.command"
    printf '\n' >> "$output_dir/$name.command"
    timeout "$gate_seconds" "${command[@]}" > "$output_dir/$name.log" 2>&1
    gate_status=$?
    printf '%s\n' "$gate_status" > "$output_dir/$name.exit"
    printf '%s exit=%s evidence=%s\n' "$name" "$gate_status" "$output_dir/$name.log"
    if ((gate_status != 0)); then overall=1; fi
}

# A failed gate is retained; later planned components still run.
# The standalone graph crate qualifies its integration target; inherited root
# unit tests require the production telemetry interface from the full crate.
run_gate graph research/m31_integrated_build_20261123/graph/Cargo.toml --test graph_suite
run_gate structured research/m31_integrated_build_20261123/structured/Cargo.toml --all-targets
run_gate rr5 research/prime31_four_summand_20261122/native/Cargo.toml --all-targets
printf '%s\n' "$overall" > "$output_dir/overall.exit"
exit "$overall"
