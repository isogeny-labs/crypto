#!/usr/bin/env bash
set -euo pipefail
build=$(realpath "${1:?qualified build directory}");public=$(realpath "${2:?fresh public targets}");current=$(realpath "${3:?current rho binary}");out=${4:?new output directory}
test ! -e "$out";mkdir "$out";here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
cp "$here/PROTOCOL_FLAT.json" "$out/protocol.json"
sources=("$here/target-flat.cpp" "$here/flat.hpp" "$here/canonical-flat.hpp" "$here/progression-flat.hpp" "$here/canonical.hpp" "$here/progression.hpp" "$here/current-rho.rs" "$here/../../../src/cryptanalysis/prime_fast.rs" "$here/PROTOCOL_FLAT.json" "$here/run-flat.sh" "$build/target-kernel" "$build/target-flat-full" "$build/target-flat-gen" "$current")
sha256sum "${sources[@]}" > "$out/pre-call.sha256"
for bin in "$build/target-kernel" "$build/target-flat-full" "$build/target-flat-gen" "$current";do ldd "$bin";done > "$out/ldd.txt"
mapfile -t libs < <(awk '/=> \//{print $3} /^\//{print $1}' "$out/ldd.txt" | sort -u);sha256sum "${libs[@]}" > "$out/dependencies-pre.sha256"
lscpu > "$out/host.txt";cat /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max > "$out/cgroup.txt"
cpu=$(awk '/Cpus_allowed_list/{print $2}' /proc/self/status | cut -d, -f1 | cut -d- -f1);echo "$cpu" > "$out/cpu.txt"
trap 'code=$?; printf "{\"exit\":%s}\n" "$code" > "$out/exit.json"' EXIT
arms=(tree-ap2 tree-ap4 tree-ap8 flat-gen-ap4 flat-ap2 flat-ap4 flat-ap8 flat-ap4-aa current-tuned8 current-tuned16 current-tuned);index=0
fixtures=("$public"/*.json);test "${#fixtures[@]}" -eq 480
for round in 0 1 2 3 4 5;do
 date -u +%FT%TZ >> "$out/progress.txt"
 for fixture in "${fixtures[@]}";do
  name=$(basename "$fixture");seed=$(jq -r '.public_seed' "$fixture")
  for ((j=0;j<11;j++));do
   arm=${arms[$(((j+round)%11))]};code=0;index=$((index+1))
   mode=${arm##*-};bin="$build/target-flat-full"
   if [[ $arm == tree-* ]];then bin="$build/target-kernel";fi
   if [[ $arm == flat-gen-* ]];then bin="$build/target-flat-gen";fi
   if [[ $arm == flat-ap4-aa ]];then mode=ap4;fi
   command=(env TRACE_PAIR_KERNEL=barrett "$bin" "$mode" "$fixture" "$seed")
   if [[ $arm == current-* ]];then
    policy=${arm#current-}
    command=(env TRACE_RHO_CONFIG="$policy" TRACE_INCIDENCE_SOURCE_REVISION="$(git -C "$here" rev-parse HEAD)" "$current" "$fixture" "$seed" 4 8 0 10000000)
   fi
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
