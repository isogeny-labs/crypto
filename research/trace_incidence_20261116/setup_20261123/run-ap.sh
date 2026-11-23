#!/usr/bin/env bash
set -euo pipefail
build=$(realpath "${1:?qualified build directory}");parent=$(realpath "${2:?parent study}");out=${3:?new output directory}
test ! -e "$out";mkdir "$out";here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
cp "$here/PROTOCOL_AP.json" "$out/protocol.json";cp /workspace/crypto_pr_context/runpod-c0176f942/extracted/rho.qualifier "$out/rho.qualifier";chmod 0555 "$out/rho.qualifier"
echo "a22eea11abeb41aea88a60f99860a5f9d6ba2dd5f367e6b6e66001c0ef6d09e8  $out/rho.qualifier" > "$out/rho-expected.sha256";sha256sum -c "$out/rho-expected.sha256" > "$out/rho-check.txt"
sources=("$here/target-ap.cpp" "$here/progression.hpp" "$here/PROTOCOL_AP.json" "$here/run-ap.sh" "$here/accelerated.cpp" "$here/benchmark.cpp" "$build/target-ap" "$out/rho.qualifier")
sha256sum "${sources[@]}" > "$out/pre-call.sha256"
for bin in "$build/target-ap" "$out/rho.qualifier";do ldd "$bin";done > "$out/ldd.txt"
mapfile -t libs < <(awk '/=> \//{print $3} /^\//{print $1}' "$out/ldd.txt" | sort -u);sha256sum "${libs[@]}" > "$out/dependencies-pre.sha256"
lscpu > "$out/host.txt";cat /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max > "$out/cgroup.txt"
cpu=$(awk '/Cpus_allowed_list/{print $2}' /proc/self/status | cut -d, -f1 | cut -d- -f1);echo "$cpu" > "$out/cpu.txt"
trap 'code=$?; printf "{\"exit\":%s}\n" "$code" > "$out/exit.json"' EXIT
arms=(reuse asym4 ap1 ap2 ap4 ap8 rho-3-1 rho-3-8 rho-4-1 rho-4-8 rho-5-1 rho-5-8 rho-6-1 rho-6-8);index=0
fixtures=("$parent"/historical/prime_fixtures/b???_c?_natural_s?.json);test "${#fixtures[@]}" -eq 60
for round in 0 1 2 3 4 5;do
 date -u +%FT%TZ >> "$out/progress.txt"
 for fixture in "${fixtures[@]}";do
  name=$(basename "$fixture");seed=$(jq -r '.public_seed' "$fixture")
  for ((j=0;j<14;j++));do
   arm=${arms[$(((j+round)%14))]};code=0;index=$((index+1));binary="$build/target-ap"
   command=("$binary" "$arm" "$fixture" "$seed")
   if [[ $arm == rho-* ]];then IFS=- read -r tag bits walkers <<< "$arm";command=(env TRACE_INCIDENCE_SOURCE_REVISION=c0176f942c03a549a98cb680b816e864b8820333 "$out/rho.qualifier" "$fixture" "$seed" "$bits" "$walkers" 0 10000000);fi
   /workspace/native-deps/time/usr/bin/time -q -f '{"wall_s":%e,"user_s":%U,"system_s":%S,"peak_rss_kib":%M}' -o "$out/current.time" timeout -k 2 45 taskset -c "$cpu" "${command[@]}" > "$out/current.json" 2>> "$out/stderr.log" || code=$?
   if ! jq -e 'type=="object"' "$out/current.json" >/dev/null 2>&1;then printf '{"status":"failed"}\n' > "$out/current.json";fi
   jq -c --slurpfile process "$out/current.time" --arg arm "$arm" --arg fixture "$name" --argjson round "$round" --argjson exit "$code" --argjson index "$index" '.+{arm_label:$arm,source_fixture:$fixture,round:$round,wrapper_exit:$exit,row_index:$index,process_peak_rss_kib:$process[0].peak_rss_kib}' "$out/current.json" >> "$out/rows.jsonl"
   jq -c --argjson index "$index" '.+{row_index:$index}' "$out/current.time" >> "$out/process.jsonl"
   if ((code));then exit "$code";fi
  done
 done
done
sha256sum "${sources[@]}" > "$out/post-call.sha256";cmp "$out/pre-call.sha256" "$out/post-call.sha256"
sha256sum "${libs[@]}" > "$out/dependencies-post.sha256";cmp "$out/dependencies-pre.sha256" "$out/dependencies-post.sha256"
