#!/bin/sh
# DRAFT Iso Lab command payload: interleaved F4 A/B/A-A for f4_fp_bench.
# Modelled on research/prime_leads_chase_20261113/norm-branch-census/launch/run.sh.
set -eu
[ "$#" -eq 2 ] || { echo 'usage: run.sh SOURCE_REVISION PROTOCOL_SHA256' >&2; exit 2; }
source_revision=$1 protocol_sha256=$2
output=${ISOLAB_OUTPUT_DIR:?} results_uri=${ISOLAB_RESULTS_URI:?}
mkdir -p "$output/rounds"
finish() {
  status=$?; trap - EXIT
  printf '{"schema":"f4.fp-ab-exit/v1","exit_status":%s}\n' "$status" > "$output/job-exit.json"
  ( cd "$output" && rm -f SHA256SUMS && find . -type f ! -name SHA256SUMS -print | sort | xargs sha256sum > SHA256SUMS )
  aws s3 cp "$output/" "$results_uri" --recursive --only-show-errors || { [ "$status" -ne 0 ] && exit "$status"; exit 91; }
  exit "$status"
}
trap finish EXIT; trap 'exit 143' TERM; trap 'exit 130' INT

cp /opt/isolab/build-info.json "$output/build-info.json"
cp /opt/isolab/f4ab/protocol.json "$output/protocol.json"
[ "$(sha256sum "$output/protocol.json" | cut -d' ' -f1)" = "$protocol_sha256" ]
bench_sha=$(sha256sum /usr/local/bin/f4_fp_bench | cut -d' ' -f1)
jq -e --arg r "$source_revision" --arg p "$protocol_sha256" --arg b "$bench_sha" \
  '.source_revision==$r and .f4ab_protocol_sha256==$p and .f4_fp_bench_sha256==$b' "$output/build-info.json" >/dev/null

# Host facts (no PMU assumed): CPU model and flags, cpuset, pressure, kernel cmdline.
{ grep -m1 'model name' /proc/cpuinfo; grep -m1 -o -w 'avx512f' /proc/cpuinfo || echo 'avx512f absent';
  grep Cpus_allowed_list /proc/self/status; nproc; cat /proc/cmdline; uname -a; } > "$output/host.txt" 2>&1 || true
cat /proc/pressure/cpu > "$output/psi-cpu-start.txt" 2>/dev/null || true

rounds=$(jq -r '.warmup_rounds + .measured_rounds' "$output/protocol.json")
warm=$(jq -r '.warmup_rounds' "$output/protocol.json")
narms=$(jq -r '.arms|length' "$output/protocol.json")
threads=$(jq -r '.threads[]' "$output/protocol.json")
r=0
while [ "$r" -lt "$rounds" ]; do
  for t in $threads; do
    k=0
    while [ "$k" -lt "$narms" ]; do
      i=$(( (k + r) % narms ))
      arm=$(jq -r ".arms[$i].name" "$output/protocol.json")
      # env -i: nothing inherited except the arm's toggles and pinned basics.
      set -- $(jq -r ".arms[$i].env | to_entries[] | \"\(.key)=\(.value)\"" "$output/protocol.json")
      out="$output/rounds/r$(printf %03d "$r")-t$t-$arm.jsonl"
      t0=$(date +%s%N)
      env -i PATH=/usr/bin:/bin HOME=/work LC_ALL=C RAYON_NUM_THREADS="$t" "$@" \
        /usr/local/bin/f4_fp_bench $(jq -r '.bench_argv|join(" ")' "$output/protocol.json") > "$out" 2> "$out.stderr"
      t1=$(date +%s%N)
      printf '{"round":%s,"warmup":%s,"threads":%s,"arm":"%s","start_ns":%s,"end_ns":%s}\n' \
        "$r" "$([ "$r" -lt "$warm" ] && echo true || echo false)" "$t" "$arm" "$t0" "$t1" >> "$output/schedule.jsonl"
      k=$((k + 1))
    done
  done
  r=$((r + 1))
done
cat /proc/pressure/cpu > "$output/psi-cpu-end.txt" 2>/dev/null || true

# Output identity (hard gate): every invocation of every arm and thread count
# reproduces the same output fields per case.
for f in "$output"/rounds/*.jsonl; do
  b=${f##*/}; b=${b%.jsonl}; arm=${b##*-}
  jq -c --arg arm "$arm" '. + {arm:$arm}' "$f"
done > "$output/all.jsonl"
jq -s --argjson f "$(jq -c .identity_required "$output/protocol.json")" '
  group_by(.case) | map({case: .[0].case,
    distinct: (map(with_entries(select(.key as $k | $f | index($k)))) | unique | length)})' \
  "$output/all.jsonl" > "$output/identity.json"
# Counters: constant within each arm (gate); differences between arms reported.
jq -s --argjson f "$(jq -c .counters_constant_within_arm "$output/protocol.json")" '
  group_by([.case, .arm]) | map({case: .[0].case, arm: .[0].arm,
    distinct: (map(with_entries(select(.key as $k | $f | index($k)))) | unique | length),
    counters: (.[0] | with_entries(select(.key as $k | $f | index($k))))})' \
  "$output/all.jsonl" > "$output/counters.json"
jq -e 'all(.[]; .distinct==1)' "$output/identity.json" >/dev/null
jq -e 'all(.[]; .distinct==1)' "$output/counters.json" >/dev/null
