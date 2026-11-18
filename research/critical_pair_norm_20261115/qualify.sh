#!/usr/bin/env bash
# Run a frozen copy, never the file being edited for future work.
set -euo pipefail
if [ "$#" -ne 2 ]; then echo 'usage: qualify.sh BUILD NEW_ROOT' >&2; exit 2; fi
critical_repo=$(git rev-parse --show-toplevel)
critical_study="$critical_repo/research/critical_pair_norm_20261115"
critical_build=$(realpath "$1")
critical_root=$(realpath -m "$2")
critical_fixtures="$critical_repo/research/macaulay_provenance_20261115/fixtures"
mkdir "$critical_root"
cp "$critical_build/build-source-binary.sha256" "$critical_root/build-source-binary.sha256"
cp "$critical_build/"{compiler.txt,openssl.txt} "$critical_root/"
sha256sum "$0" "$critical_study/"{qualify.sh,build.sh,produce.cpp} \
 "$critical_build/"{produce,verify,compare,auxiliary,replay} > "$critical_root/pre-call-source-binary-script.sha256"
git rev-parse HEAD > "$critical_root/source-commit.txt"
export PC_VERIFIER="$critical_build/verify"
printf 'case\tdegree\texit\tstatus\n' > "$critical_root/controls.tsv"
while read -r critical_case critical_degree; do
 critical_dir="$critical_root/$critical_case"
 mkdir "$critical_dir"
 cp "$critical_fixtures/$critical_case.ms" "$critical_dir/input.ms"
 if (ulimit -v 2097152 || exit 125; ulimit -t 20 || exit 125; exec timeout --signal=KILL 20 "$critical_build/produce" -t 1 -g 2 --degree "$critical_degree" -f "$critical_dir/input.ms" -o "$critical_dir/basis.gb") \
  > "$critical_dir/stdout.log" 2> "$critical_dir/stderr.log"; then critical_exit=0; else critical_exit=$?; fi
 printf '%s\n' "$critical_exit" > "$critical_dir/exit.txt"
 critical_status=unknown
 if [ "$critical_exit" = 0 ]; then
  critical_expected=$critical_case
  if [ "$critical_case" = norm-m3-t3-r0-6 ]; then critical_expected="$critical_fixtures/norm-m3-t3-r0-6-reference.gb"; fi
  "$critical_build/compare" "$critical_expected" "$critical_dir/input.ms" "$critical_dir/basis.gb" > "$critical_dir/compare.log" 2>&1
  critical_status=match
 elif [ "$critical_case" != norm-m3-t3-r0-6 ]; then
  printf '%s\t%s\t%s\tfailed_control\n' "$critical_case" "$critical_degree" "$critical_exit" >> "$critical_root/controls.tsv"
  exit 2
 fi
 printf '%s\t%s\t%s\t%s\n' "$critical_case" "$critical_degree" "$critical_exit" "$critical_status" >> "$critical_root/controls.tsv"
done <<'CASES'
grid 3
nonreduced 3
extension 3
unit 1
x 2
xy 2
norm-m3-t3-r0-6 4
CASES
export PC_DEGREE_CAP=6
if (ulimit -v 2097152 || exit 125; exec timeout --signal=KILL 1200 "$critical_build/auxiliary" --qualify-eliminated "$critical_build/produce" "$critical_root/models" "$critical_root/summary.tsv") \
 > "$critical_root/stdout.log" 2> "$critical_root/stderr.log"; then critical_exit=0; else critical_exit=$?; fi
printf '%s\n' "$critical_exit" > "$critical_root/exit.txt"
test "$critical_exit" = 0
"$critical_build/replay" "$critical_root/models" "$critical_root/summary.tsv" > "$critical_root/openssl-replay.txt" 2>&1
sha256sum -c "$critical_root/build-source-binary.sha256" > "$critical_root/post-call-build-binding.txt"
sha256sum -c "$critical_root/pre-call-source-binary-script.sha256" > "$critical_root/post-call-source-binary-script.txt"
cat "$critical_root/controls.tsv" "$critical_root/summary.tsv" "$critical_root/openssl-replay.txt"
