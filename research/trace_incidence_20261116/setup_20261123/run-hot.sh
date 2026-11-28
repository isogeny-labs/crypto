#!/usr/bin/env bash
set -euo pipefail
build=$(realpath "${1:?qualified build directory}");public=$(realpath "${2:?fresh public targets}");current=$(realpath "${3:?current rho binary}");native=$(realpath "${4:?native rho binary}");out=${5:?new output directory}
test ! -e "$out";mkdir "$out";here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
cp "$here/PROTOCOL_HOT.json" "$out/protocol.json"
sources=("$here/target-hot.cpp" "$here/canonical-blocked.hpp" "$here/progression-reserved.hpp" "$here/progression-sized.hpp" "$here/canonical-field.hpp" "$here/current-rho-scaling.rs" "$here/../../../src/cryptanalysis/prime_fast.rs" "$here/PROTOCOL_HOT.json" "$here/run-hot.sh" "$here/build-hot.sh" "$here/build-current-rho-native.sh" "$build/target-hot-baseline" "$build/target-hot-reserve" "$build/target-hot-blocked" "$build/target-hot-combined" "$build/target-hot-baseline-native" "$build/target-hot-combined-native" "$current" "$native")
sha256sum "${sources[@]}" > "$out/pre-call.sha256"
for bin in "$build/target-hot-baseline" "$build/target-hot-reserve" "$build/target-hot-blocked" "$build/target-hot-combined" "$build/target-hot-baseline-native" "$build/target-hot-combined-native" "$current" "$native";do ldd "$bin";done > "$out/ldd.txt"
mapfile -t libs < <(awk '/=> \//{print $3} /^\//{print $1}' "$out/ldd.txt" | sort -u);sha256sum "${libs[@]}" > "$out/dependencies-pre.sha256"
lscpu > "$out/host.txt";cat /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max > "$out/cgroup.txt"
cpu=$(awk '/Cpus_allowed_list/{print $2}' /proc/self/status | cut -d, -f1 | cut -d- -f1);echo "$cpu" > "$out/cpu.txt"
trap 'code=$?; printf "{\"exit\":%s}\n" "$code" > "$out/exit.json"' EXIT
arms=(baseline-balanced reserve-balanced blocked-balanced combined-balanced baseline-native-balanced combined-native-balanced baseline-quarter combined-native-quarter combined-native-balanced-aa generic-tuned4 generic-tuned8 generic-tuned16 generic-tuned32 generic-tuned64 generic-tuned128 native-tuned4 native-tuned8 native-tuned16 native-tuned32 native-tuned64 native-tuned128);index=0
fixtures=("$public"/*.json);test "${#fixtures[@]}" -eq 192
for round in 0 1 2 3 4 5;do
 date -u +%FT%TZ >> "$out/progress.txt"
 for fixture in "${fixtures[@]}";do
  name=$(basename "$fixture");seed=$(jq -r '.public_seed' "$fixture")
  for ((j=0;j<21;j++));do
   arm=${arms[$(((j+round)%21))]};code=0;index=$((index+1))
   mode=${arm##*-};variant=${arm%-$mode}
   if [[ $arm == combined-native-balanced-aa ]];then mode=balanced;variant=combined-native;fi
   command=(env TRACE_PAIR_KERNEL=barrett "$build/target-hot-$variant" "$mode" "$fixture" "$seed")
   if [[ $arm == generic-* || $arm == native-* ]];then
    policy=${arm#*-};rho="$current";if [[ $arm == native-* ]];then rho="$native";fi
    command=(env TRACE_RHO_CONFIG="$policy" TRACE_INCIDENCE_SOURCE_REVISION="$(git -C "$here" rev-parse HEAD)" "$rho" "$fixture" "$seed" 4 8 0 10000000)
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
