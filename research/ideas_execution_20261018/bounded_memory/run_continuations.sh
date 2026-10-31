#!/usr/bin/env bash
set -u
cd /workspace/crypto
study=research/ideas_execution_20261018/bounded_memory
previous="$study/runs/run-20261018-native-v2"
run_dir="$study/runs/run-20261018-large-continuation-v1"
if [ -e "$run_dir" ]; then echo 'append-only run directory already exists' >&2; exit 2; fi
mkdir "$run_dir"
exec > >(tee "$run_dir/driver.log") 2>&1
date -u +%Y-%m-%dT%H:%M:%SZ > "$run_dir/started-utc.txt"
sha256sum "$study/protocol-large-continuation.json" "$study/run_continuations.sh" /tmp/ideas-disk-macaulay /tmp/ideas-resource-probe > "$run_dir/frozen-sha256.txt"
printf 'case\texit\n' > "$run_dir/attempts.tsv"
trial() {
 local name="$1"; shift
 printf '%q ' "$@" > "$run_dir/$name.command.txt"; printf '\n' >> "$run_dir/$name.command.txt"
 /tmp/ideas-resource-probe "$run_dir/$name.resources.json" "$@" > "$run_dir/$name.stdout" 2> "$run_dir/$name.stderr"
 local code=$?
 printf '%s\t%s\n' "$name" "$code" >> "$run_dir/attempts.tsv"
 echo "$name exit=$code"
}
for stem in n15-m4-ell5 n17-m4-ell6; do
 for policy in first min-weight; do
  name="$stem-$policy"
  mkdir "$run_dir/$name"
  cp "$previous/$name/checkpoint.txt" "$previous/$name/pivots.bin" "$run_dir/$name/"
  sha256sum "$run_dir/$name/checkpoint.txt" "$run_dir/$name/pivots.bin" > "$run_dir/$name.inherited-sha256.txt"
  trial "$name" /tmp/ideas-disk-macaulay reduce "$study/inputs/$stem.rows" "$run_dir/$name" 1048576 "$policy" 0 120 0 100 resume
 done
done
mkdir "$run_dir/identity-control"
cp "$previous/n5-resume/checkpoint.txt" "$previous/n5-resume/pivots.bin" "$run_dir/identity-control/"
trial restart-identity /tmp/ideas-disk-macaulay reduce "$study/inputs/n9-m4-ell3.rows" "$run_dir/identity-control" 1048576 first 0 120 0 100 resume
date -u +%Y-%m-%dT%H:%M:%SZ > "$run_dir/finished-utc.txt"
