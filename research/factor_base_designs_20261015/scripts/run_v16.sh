#!/bin/sh
# Amendment 16 validation run V16-A0: A0, m = 4, t = 4, target 0 (the system of
# R256-A0 target 0), with snapshots inside steps.
#
#   run_v16.sh BINARY OUT_DIR
#
# One system, `--checkpoint-every 600 --checkpoint-in-step-every 900`. The
# row is V16-A0-m4-t4.jsonl. The caller may kill the process and call this
# script again: it resumes (or skips when the row exists).
set -eu

bin=$1
out=$2
: "${FB_DESIGNS_COMMIT:?set FB_DESIGNS_COMMIT to the commit the binary was built from}"
name=V16-A0-m4-t4
mkdir -p "$out"
if [ -s "$out/$name.jsonl" ]; then
    echo "$name already has its row; skipping"
    exit 0
fi
mem_kb=$(awk '/^MemTotal:/ {print $2}' /proc/meminfo)
v_kb=$((mem_kb - 524288))
{
    echo "== segment start: $name"
    echo "date $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "commit $FB_DESIGNS_COMMIT"
    echo "binary_sha256 $(sha256sum "$bin" | cut -d' ' -f1)"
    echo "MemTotal_kB $mem_kb"
    echo "nproc $(nproc)"
    echo "V_kB $v_kb"
    echo "kernel $(uname -r)"
    grep -m1 '^model name' /proc/cpuinfo
    df -h / | tail -1 | sed 's/^/disk /'
} | tee -a "$out/$name.preflight.txt"
"$bin" run --arms A0 --m 4 --t-min 4 --t-max 4 --targets 1 \
    --budget 14400 --grace 600 --vmem-kb "$v_kb" \
    --checkpoint-every 600 --checkpoint-in-step-every 900 \
    --file "V16-{arm}-m4-t4" --out-dir "$out"
