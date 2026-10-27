#!/usr/bin/env bash
# Reconstruct the predeclared doubled triangles; never regenerate zero sets.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_out=${1:?new output directory}
joint_saved="$joint_study/runs/uniform-zero-v2"
joint_frozen="$joint_study/runs/five-zero-v1"
test ! -e "$joint_out"
test "$(cat "$joint_frozen/metadata/native-tests.exit")" = 0
test "$(cat "$joint_frozen/metadata/compile-runner.exit")" = 0
sha256sum -c "$joint_frozen/metadata/runner.sha256"
mkdir -p "$joint_out/metadata"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
cp "$joint_study/SUPPLEMENT_ZERO_DOUBLING_IDENTITY.md" "$joint_out/metadata/frozen-protocol.md"
cp "$joint_frozen/metadata/runner.sha256" "$joint_out/metadata/reused-runner.sha256"
git rev-parse HEAD > "$joint_out/metadata/orchestration-commit.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
while true; do
  joint_ready=true
  for joint_p in 37 73 97 157 229 241 337; do
    for joint_s in 0 1 2; do
      test -f "$joint_saved/zero/p$joint_p-seed$joint_s/summary.json" || joint_ready=false
    done
  done
  if test "$joint_ready" = true; then break; fi
  if ! systemctl is-active --quiet twist-joint-uniform-zero-v2.service; then
    printf 'Original oracle owner is terminal with a missing zero summary; no oracle repeated.\n' > "$joint_out/metadata/missing-prerequisite.txt"
    exit 2
  fi
  sleep 10
done
for joint_p in 37 73 97 157 229 241 337; do
  for joint_s in 0 1 2; do
    sha256sum "$joint_saved/zero/p$joint_p-seed$joint_s/double-l0-active.json"
  done
done > "$joint_out/metadata/saved-zero-inputs.sha256"
export RAYON_NUM_THREADS=1
ulimit -c 0
joint_command=(prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_frozen/runner" --joint-zero-identity-audit "$joint_saved" "$joint_study/runs/algebra-v1/prepared/public" "$joint_out/verified")
printf '%q ' "${joint_command[@]}" > "$joint_out/metadata/check.command.txt"
set +e
python3 "$joint_repo/tools/isolated_bench.py" busy -- "${joint_command[@]}" > "$joint_out/metadata/check.stdout" 2> "$joint_out/metadata/check.stderr"
joint_rc=$?
set -e
printf '%s\n' "$joint_rc" > "$joint_out/metadata/check.exit"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
exit "$joint_rc"
