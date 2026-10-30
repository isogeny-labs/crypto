#!/usr/bin/env bash
# Exercise counter availability and exit statuses without requiring a PMU.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
cat > "$scratch/perf" <<'EOF'
#!/usr/bin/env bash
set -eu
if [ "$1" = --version ]; then echo 'perf version fixture'; exit 0; fi
out=""
while [ $# -gt 0 ]; do
  if [ "$1" = -o ]; then out=$2; shift; fi
  shift
done
if [ -z "$out" ]; then
  case $PERF_CASE in
    probe_unavailable) printf '<not counted>,,cycles:u,0,100\n<not counted>,,instructions:u,0,100\n' >&2 ;;
    *) printf '100,,cycles:u,100,100\n200,,instructions:u,100,100\n' >&2 ;;
  esac
  exit 0
fi
case $PERF_CASE in
  late_unavailable) printf '<not counted>,,cycles:u,0,100\n<not counted>,,instructions:u,0,100\n' > "$out" ;;
  software) printf '1.5,msec,task-clock,100,100\n' > "$out" ;;
  multiplexed) printf '100,,cycles:u,100,50\n200,,instructions:u,100,50\n' > "$out" ;;
  *) printf '100,,cycles:u,100,100\n200,,instructions:u,100,100\n' > "$out" ;;
esac
[ "$PERF_CASE" != failed_child ] || exit 7
EOF
chmod +x "$scratch/perf"
check() {
  local name=$1 expected=$2 predicate=$3
  shift 3
  local rc=0
  PERF_CASE="$name" PERF="$scratch/perf" bash "$here/perf_ipc.sh" \
    --repeat 1 --out "$scratch/$name.jsonl" "$@" -- true \
    > "$scratch/$name.stdout" 2> "$scratch/$name.stderr" || rc=$?
  if [ "$rc" != "$expected" ]; then
    cat "$scratch/$name.stderr" >&2
    echo "$name: expected exit $expected, got $rc" >&2
    exit 1
  fi
  jq -es "$predicate" "$scratch/$name.jsonl" > /dev/null
  echo "PASS $name"
}
check probe_unavailable 3 'length == 1 and .[0].kind == "unsupported"'
check late_unavailable 3 'length == 2 and .[0].kind == "run" and .[0].ipc == null and .[1].kind == "unsupported"'
check counted 0 '.[-1].kind == "summary" and .[-1].ipc.n == 1 and .[-1].ipc.median == 2'
check multiplexed 5 '.[-1].multiplexed_runs == 1'
check failed_child 4 '.[-1].failed_runs == 1 and .[0].exit_status == 7'
check software 0 '.[-1].ipc.n == 0 and .[-1].task_clock_ms.n == 1' --selftest
