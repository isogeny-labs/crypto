#!/usr/bin/env bash
set -euo pipefail
build=$(realpath "${1:?qualified build directory}")
parent=$(realpath "${2:?parent study}")
minimal=$(realpath "${3:?minimal public fixtures}")
out=${4:?new output directory}
test ! -e "$out";mkdir "$out"
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
cp "$here/PROTOCOL.json" "$out/protocol.json"
cp /workspace/crypto_pr_context/runpod-c0176f942/extracted/rho.qualifier "$out/rho.qualifier"
chmod 0555 "$out/rho.qualifier"
echo "a22eea11abeb41aea88a60f99860a5f9d6ba2dd5f367e6b6e66001c0ef6d09e8  $out/rho.qualifier" > "$out/rho-expected.sha256"
sha256sum -c "$out/rho-expected.sha256" > "$out/rho-check.txt"
sources=("$here/accelerated.cpp" "$here/benchmark.cpp" "$here/run.sh" "$here/PROTOCOL.json" "$build/prior" "$build/optimized" "$out/rho.qualifier")
sha256sum "${sources[@]}" > "$out/pre-call.sha256"
for bin in "$build/prior" "$build/optimized" "$out/rho.qualifier";do ldd "$bin";done > "$out/ldd.txt"
mapfile -t libs < <(awk '/=> \//{print $3} /^\//{print $1}' "$out/ldd.txt" | sort -u)
sha256sum "${libs[@]}" > "$out/dependencies-pre.sha256"
lscpu > "$out/host.txt";cat /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max > "$out/cgroup.txt"
cpu=$(awk '/Cpus_allowed_list/{print $2}' /proc/self/status | cut -d, -f1 | cut -d- -f1)
echo "$cpu" > "$out/cpu.txt"
trap 'code=$?; printf "{\"exit\":%s}\n" "$code" > "$out/exit.json"' EXIT
arms=(prior/full optimized/full prior/minimal optimized/minimal pair-control/minimal rho/full rho/minimal)
index=0
one(){
 local round=$1 label=$2 fixture=$3 attempt=$4;shift 4
 local code=0;index=$((index+1))
 /workspace/native-deps/time/usr/bin/time -q -f '{"wall_s":%e,"user_s":%U,"system_s":%S,"peak_rss_kib":%M}' -o "$out/current.time" timeout -k 2 45 taskset -c "$cpu" "$@" > "$out/current.json" 2>> "$out/stderr.log" || code=$?
 if ! jq -e 'type=="object"' "$out/current.json" >/dev/null 2>&1;then printf '{"status":"failed"}\n' > "$out/current.json";fi
 jq -c --arg label "$label" --arg fixture "$fixture" --argjson round "$round" --argjson attempt "$attempt" --argjson exit "$code" --argjson index "$index" '.+{arm_label:$label,source_fixture:$fixture,round:$round,attempt:$attempt,wrapper_exit:$exit,row_index:$index}' "$out/current.json" >> "$out/rows.jsonl"
 jq -c --argjson index "$index" '.+{row_index:$index}' "$out/current.time" >> "$out/process.jsonl"
 LAST_STATUS=$(jq -r '.status' "$out/current.json")
 if ((code));then return "$code";fi
}
fixtures=("$parent"/historical/prime_fixtures/b???_c?_natural_s?.json);test "${#fixtures[@]}" -eq 60
for round in 0 1 2 3 4 5;do
 date -u +%FT%TZ >> "$out/progress.txt"
 for full in "${fixtures[@]}";do
  name=$(basename "$full");base_seed=$(jq -r '.public_seed' "$minimal/$name")
  for ((j=0;j<7;j++));do
   label=${arms[$(((j+round)%7))]};file=$full
   if [[ $label == */minimal ]];then file="$minimal/$name";fi
   if [[ $label == rho/* ]];then
    one "$round" "$label" "$name" 0 env TRACE_INCIDENCE_SOURCE_REVISION=c0176f942c03a549a98cb680b816e864b8820333 "$out/rho.qualifier" "$file" "$base_seed" 4 8 0 10000000
   else
    binary=prior;norm=first-bloom
    if [[ $label == optimized/* ]];then binary=optimized;fi
    if [[ $label == pair-control/* ]];then norm=frozen-pairs;fi
    for attempt in 0 1 2 3;do
     seed=$((base_seed+attempt*2654435761))
     one "$round" "$label" "$name" "$attempt" "$build/$binary" window "$norm" "$file" "$seed"
     if [[ $LAST_STATUS == relation ]];then break;fi
    done
   fi
  done
 done
done
sha256sum "${sources[@]}" > "$out/post-call.sha256";cmp "$out/pre-call.sha256" "$out/post-call.sha256"
sha256sum "${libs[@]}" > "$out/dependencies-post.sha256";cmp "$out/dependencies-pre.sha256" "$out/dependencies-post.sha256"
