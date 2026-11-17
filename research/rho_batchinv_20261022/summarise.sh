#!/usr/bin/env bash
# Round 4. Reduce runs/ to summary.json: Callgrind instructions per walk step for each
# arm at n41 and n53 (with an output digest per arm), the identity check at n41/53/61, and native ns per walk step per curve
# and arm from the uncontended A/A and A/B runs.  Medians, extremes and their
# quotients only.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
runs="$here/runs"
collected() { grep -o 'Collected : [0-9]*' "$1" | grep -o '[0-9]*$'; }
digest() { jq -c 'del(.peak_rss_bytes) | with_entries(select(.key | endswith("_ms") | not))' "$1" | sha256sum | cut -c1-16; }

ir=$(for f in "$runs"/callgrind/fixture-n*-4-*.jsonl; do
    f=${f%.jsonl}
    jq -nc --arg file "$(basename "$f")" --argjson ir "$(collected "$f.valgrind.log")" \
        --argjson steps "$(jq -s 'map(.walk_steps) | add' "$f.jsonl")" --arg id "$(digest "$f.jsonl")" \
        '{n: ($file | capture("n(?<n>[0-9]+)").n | tonumber), arm: ($file | capture("-(?<a>baseline|candidate)$").a),
          ir: $ir, walk_steps: $steps, ir_per_step: ($ir / $steps), output_digest: $id}'
done | jq -sc 'group_by(.n) | map({n: .[0].n, arms: ., identical: (map(.output_digest) | unique | length == 1),
    baseline_over_candidate: ((map(select(.arm == "baseline"))[0].ir) / (map(select(.arm == "candidate"))[0].ir))})')

identity=$(for f in "$runs"/identity/*.jsonl; do
    jq -nc --arg file "$(basename "$f")" --arg id "$(digest "$f")" \
        --argjson rows "$(wc -l < "$f")" --argjson recovered "$(jq -s 'map(.recovered_fixture_scalar == .published_fixture_scalar) | all' "$f")" \
        '{n: ($file | capture("n(?<n>[0-9]+)").n | tonumber), arm: ($file | capture("-(?<a>baseline|candidate)\\.").a),
          rows: $rows, all_recovered: $recovered, output_digest: $id}'
done | jq -sc 'group_by(.n) | map({n: .[0].n, rows: .[0].rows, identical: (map(.output_digest) | unique | length == 1),
    all_recovered: (map(.all_recovered) | all), digest: .[0].output_digest})')

time=null
if ls "$runs"/time/*/*.jsonl > /dev/null 2>&1; then
    contended=$(cat "$runs"/time/*/isolation.jsonl | jq -sc 'map(select(.run.contended == true) | .label)')
    time=$(for f in "$runs"/time/*/*.jsonl; do
        jq -sc --arg file "$(basename "$f")" --arg phase "$(basename "$(dirname "$f")")" --argjson contended "$contended" '
            {phase: $phase, file: $file,
             contended: (("time/" + $phase + "/" + ($file | sub("\\.jsonl$"; ""))) as $l | $contended | index($l) != null),
             arm: ($file | capture("-(?<a>baseline|candidate)-r").a),
             slug: ($file | capture("^(?<s>icv1-[^-]+-[^-]+-[0-9a-f]{8})").s),
             ns_per_step: ((map(.walk_ms) | add) * 1e6 / (map(.walk_steps) | add)),
             identity: (map(del(.peak_rss_bytes) | with_entries(select(.key | endswith("_ms") | not))) | tostring)}' "$f"
    done | jq -sc '
        def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;
        (map(.identity) | group_by(.) | length) as $ids |
        map(select(.contended | not)) | group_by(.slug) | map({slug: .[0].slug,
            aa_spread: (map(select(.phase == "aa") | .ns_per_step) | (max / min - 1)),
            baseline_ns: (map(select(.phase == "ab" and .arm == "baseline") | .ns_per_step) | {median: med, min: min, max: max, n: length}),
            candidate_ns: (map(select(.phase == "ab" and .arm == "candidate") | .ns_per_step) | {median: med, min: min, max: max, n: length}),
            identical: (map(.identity) | unique | length == 1)}
          | .ratio = (.baseline_ns.median / .candidate_ns.median))')
    iso=$(cat "$runs"/time/*/isolation.jsonl | jq -sc '{runs: length,
        contended: map(select(.run.contended == true) | .label), nonzero_exit: (map(select(.run.exit_status != 0)) | length)}')
    time=$(jq -nc --argjson t "$time" --argjson iso "$iso" '{cells: $t, isolation: $iso}')
fi

jq -n --argjson ir "$ir" --argjson identity "$identity" --argjson time "$time" '{instructions: $ir, identity: $identity, native_time: $time}' > "$here/summary.json"
echo "wrote $here/summary.json"
