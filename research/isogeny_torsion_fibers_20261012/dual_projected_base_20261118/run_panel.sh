#!/usr/bin/env bash
set -euo pipefail

# Thin orchestration only; the native runner and checker produce every result.
study=research/isogeny_torsion_fibers_20261012/dual_projected_base_20261118
run="$study/RUN_V1"
prior=research/isogeny_torsion_fibers_20261012/chi_polynomial_20261015/different_j_resultant_20261016/FOUR_TARGET_PUBLIC_V1.json
full_base=research/isogeny_torsion_fibers_20261012/factor_base_grid_20261016/RUN_V2/BASE_PUBLIC_V1.json
dual_base=research/isogeny_torsion_fibers_20261012/dual_projected_base_20261115/RUN_V1/DUAL_BASE_PUBLIC_V1.json
public="$study/BATCH_PUBLIC_V1.json"
checker_input="$study/BATCH_CHECKER_V1.json"
runner_build=target/release/examples/isogeny_fiber_dual_projected
checker_build=target/release/examples/isogeny_fiber_dual_projected_check

test "$(git rev-parse --show-toplevel)" = "$PWD"
test ! -e "$run"
test -f "$public" && test -f "$checker_input"
test -x "$runner_build" && test -x "$checker_build"
mkdir -p "$run/children"
cp examples/isogeny_fiber_dual_projected.rs "$run/RUNNER_SOURCE_V1.rs.txt"
cp examples/isogeny_fiber_dual_projected_check.rs "$run/CHECKER_SOURCE_V1.rs.txt"
cp "$runner_build" "$run/RUNNER_BINARY_V1"
cp "$checker_build" "$run/CHECKER_BINARY_V1"
cp "$public" "$run/BATCH_PUBLIC_V1.json"
cp "$checker_input" "$run/BATCH_CHECKER_V1.json"
runner="$run/RUNNER_BINARY_V1"
checker="$run/CHECKER_BINARY_V1"

{
    date -u '+started_utc=%Y-%m-%dT%H:%M:%SZ'
    printf 'git_head=%s\n' "$(git rev-parse HEAD)"
    printf 'isolation=L0 shared Darwin; no enforced address-space cap\n'
    uname -a
    sw_vers
    sysctl -n machdep.cpu.brand_string
    /Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc --version
} > "$run/HOST.txt"
sha256sum "$prior" "$full_base" "$dual_base" "$public" "$checker_input" \
    "$run/RUNNER_SOURCE_V1.rs.txt" "$run/CHECKER_SOURCE_V1.rs.txt" \
    "$runner" "$checker" > "$run/FROZEN_INPUTS.sha256"
printf '%s\n' "$runner freeze-batch-targets $prior $public $checker_input" \
    "$checker check-batch BASE $full_base $prior $public $checker_input RUN" \
    "$checker audit-batch $run/children" \
    "$checker summarize-batch $run $run/SUMMARY_V1.json $run/S_GROUP_GRAPH_V1.svg" \
    > "$run/COMMANDS.txt"

arms=(direct-dual direct-inverse destination-direct source-bsgs rho-negation rho-negation-aa)
failures=0
for round in 0 1 2; do
    for index in 0 1 2 3 4 5; do
        arm="${arms[$(((index + round) % 6))]}"
        base="$full_base"
        if [[ "$arm" == direct-dual ]]; then base="$dual_base"; fi
        prefix="$run/children/r${round}-${arm}"
        printf '%s\n' "gtimeout 300 $runner run-batch $base $public $round $arm" \
            >> "$run/COMMANDS.txt"
        if gtimeout 300 "$runner" run-batch "$base" "$public" "$round" "$arm" \
            > "$prefix.jsonl" 2> "$prefix.stderr"; then
            rc=0
        else
            rc=$?
        fi
        printf '%s\n' "$rc" > "$prefix.exit"
        if (( rc != 0 )); then
            failures=$((failures + 1))
            printf 'child_failed=%s exit=%s\n' "$prefix" "$rc" >&2
            continue
        fi
        if "$checker" check-batch "$base" "$full_base" "$prior" \
            "$public" "$checker_input" "$prefix.jsonl" \
            > "$prefix.replay.json" 2> "$prefix.replay.stderr"; then
            replay_rc=0
        else
            replay_rc=$?
        fi
        printf '%s\n' "$replay_rc" > "$prefix.replay.exit"
        if (( replay_rc != 0 )); then
            failures=$((failures + 1))
            printf 'replay_failed=%s exit=%s\n' "$prefix" "$replay_rc" >&2
        fi
    done
done
if (( failures == 0 )); then
    if "$checker" audit-batch "$run/children" > "$run/PAIR_AUDIT_V1.json" \
        2> "$run/PAIR_AUDIT_V1.stderr"; then
        audit_rc=0
    else
        audit_rc=$?
        failures=$((failures + 1))
    fi
    printf '%s\n' "$audit_rc" > "$run/PAIR_AUDIT_V1.exit"
    if (( audit_rc == 0 )); then
        if "$checker" summarize-batch "$run" "$run/SUMMARY_V1.json" \
            "$run/S_GROUP_GRAPH_V1.svg" > "$run/SUMMARY_V1.receipt.json" \
            2> "$run/SUMMARY_V1.stderr"; then
            summary_rc=0
        else
            summary_rc=$?
            failures=$((failures + 1))
        fi
        printf '%s\n' "$summary_rc" > "$run/SUMMARY_V1.exit"
    fi
fi
printf 'failures=%s\n' "$failures" > "$run/RUN_STATUS.txt"
date -u '+finished_utc=%Y-%m-%dT%H:%M:%SZ' >> "$run/HOST.txt"
(
    cd "$run"
    find . -type f ! -name ARTIFACT_FILES.sha256 -print0 | sort -z | \
        xargs -0 sha256sum > ARTIFACT_FILES.sha256
)
(( failures == 0 ))
