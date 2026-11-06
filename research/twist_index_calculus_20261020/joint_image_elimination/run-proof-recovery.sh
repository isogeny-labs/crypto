#!/usr/bin/env bash
# Check one unchanged standalone theorem file. Force one Lean worker.
set -euo pipefail
joint_repo=$(git rev-parse --show-toplevel)
joint_source=${1:?repository-relative Lean source}
joint_out=${2:?new receipt directory}
joint_limit=${3:-900}
test ! -e "$joint_out"
mkdir -p "$joint_out/bin" "$joint_out/metadata"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
sha256sum "$joint_repo/$joint_source" > "$joint_out/metadata/source.sha256"
cp "$joint_repo/$joint_source" "$joint_out/metadata/theorem-source.lean.txt"
git rev-parse HEAD > "$joint_out/metadata/source-commit.txt"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
joint_lean=$(command -v lean)
printf '#!/usr/bin/env bash\nexec %q -j1 "$@"\n' "$joint_lean" > "$joint_out/bin/lean"
chmod 700 "$joint_out/bin/lean"
export PATH="$joint_out/bin:$PATH" ELAN_TOOLCHAIN=leanprover/lean4:v4.34.1
ulimit -c 0
joint_command=(prlimit --as=8589934592 -- timeout --kill-after=10s "${joint_limit}s" bash "$joint_repo/tools/lean/proof-gate.sh" --out "$joint_out/receipts" "$joint_source")
printf '%q ' "${joint_command[@]}" > "$joint_out/metadata/command.txt"
set +e
python3 "$joint_repo/tools/isolated_bench.py" busy -- "${joint_command[@]}" > "$joint_out/stdout" 2> "$joint_out/stderr"
joint_rc=$?
set -e
printf '%s\n' "$joint_rc" > "$joint_out/exit"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
exit "$joint_rc"
