#!/usr/bin/env bash
# Native fixed-degree diagnostic only; preserves every exit status.
set -u
cd /workspace/crypto
study=research/ideas_execution_20261018/bounded_memory
run_dir="$study/runs/run-20261018-native-v1"
if [ -e "$run_dir" ]; then echo 'append-only run directory already exists' >&2; exit 2; fi
mkdir "$run_dir"
exec > >(tee "$run_dir/driver.log") 2>&1
date -u +%Y-%m-%dT%H:%M:%SZ > "$run_dir/started-utc.txt"
uname -a > "$run_dir/host.txt"
cat /sys/fs/cgroup/memory.max > "$run_dir/cgroup-memory-bytes.txt"
g++ --version > "$run_dir/compiler.txt"
sha256sum "$study/protocol.json" "$study/src/disk_macaulay.cpp" examples/ideas_bounded_memory_export.rs /tmp/ideas-disk-macaulay /tmp/ideas-bounded-memory-export > "$run_dir/frozen-sha256.txt"
printf '%s\n' 'Source base e063d98475589d9aba7ae265a8cba830927e3156 plus root library repair working tree, all direct dependencies hashed separately.' > "$run_dir/revision.txt"
sha256sum src/cryptanalysis/koblitz_groebner.rs src/cryptanalysis/koblitz_index_calculus.rs src/binary_ecc/f2m.rs target/release/deps/libcrypto_lib-3e0b38c24aa5646f.rlib > "$run_dir/dependency-sha256.txt"
printf 'case\texit\n' > "$run_dir/attempts.tsv"
trial() {
 local name="$1"; shift
 printf '%q ' "$@" > "$run_dir/$name.command.txt"; printf '\n' >> "$run_dir/$name.command.txt"
 (ulimit -v 524288; /usr/bin/time -v "$@") > "$run_dir/$name.stdout" 2> "$run_dir/$name.stderr"
 local code=$?
 printf '%s\t%s\n' "$name" "$code" >> "$run_dir/attempts.tsv"
 echo "$name exit=$code"
}
trial selftest /tmp/ideas-disk-macaulay selftest
for cell in '5 2' '9 3' '15 5' '17 6'; do
 read -r n ell <<< "$cell"
 stem="n${n}-m4-ell${ell}"
 trial "$stem-export" /tmp/ideas-bounded-memory-export "$study/inputs/$stem.anf" "$n" "$ell" 4 17
 if [ -f "$study/inputs/$stem.anf" ]; then
  trial "$stem-generate" /tmp/ideas-disk-macaulay generate "$study/inputs/$stem.anf" 4 "$study/inputs/$stem.rows" 1073741824
  if [ -f "$study/inputs/$stem.rows" ]; then
   for policy in first min-weight; do
    trial "$stem-$policy" /tmp/ideas-disk-macaulay reduce "$study/inputs/$stem.rows" "$run_dir/$stem-$policy" 1048576 "$policy" 50000 120 0 100
   done
  fi
 fi
done
trial n5-zero-cache /tmp/ideas-disk-macaulay reduce "$study/inputs/n5-m4-ell2.rows" "$run_dir/n5-zero-cache" 0 first 50000 120 0 100
trial n5-checkpoint /tmp/ideas-disk-macaulay reduce "$study/inputs/n5-m4-ell2.rows" "$run_dir/n5-resume" 1048576 first 50000 120 31 10
trial n5-restart /tmp/ideas-disk-macaulay reduce "$study/inputs/n5-m4-ell2.rows" "$run_dir/n5-resume" 1048576 first 50000 120 0 10 resume
trial ell7-representation-limit /tmp/ideas-bounded-memory-export "$study/inputs/n19-m4-ell7.anf" 19 7 4 17
cp "$study/inputs/n5-m4-ell2.rows" "$run_dir/corrupt.rows"
truncate -s 27 "$run_dir/corrupt.rows"
trial malformed /tmp/ideas-disk-macaulay reduce "$run_dir/corrupt.rows" "$run_dir/malformed" 0 first 50000 120 0 10
trial disk-budget /tmp/ideas-disk-macaulay generate "$study/inputs/n5-m4-ell2.anf" 4 "$run_dir/budget.rows" 24
sha256sum "$study"/inputs/* > "$run_dir/input-sha256.txt"
date -u +%Y-%m-%dT%H:%M:%SZ > "$run_dir/finished-utc.txt"
