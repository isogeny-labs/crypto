#!/bin/sh
# Amendment 15: Amendment 14's cells (R256-A0, R256-A11), checkpointed.
#
#   run_amendment15.sh CELL BINARY OUT_DIR
#
# CELL is R256-A0 (A0, m = 4, t = 4) or R256-A11 (A11, m = 4, t = 5). BINARY is a release
# build of examples/fb_designs_slope.rs; FB_DESIGNS_COMMIT names the commit it
# was built from. Amendment 14's script with three changes: the run passes
# `--checkpoint-every` (FB_CKPT_EVERY seconds, default 600), so a host that
# dies loses at most a step and a re-run of this script on a new host, with
# OUT_DIR restored, resumes the system; a cell that already has its row is
# skipped; and the pre-flight and OOM records are appended, one section per
# invocation (a segment), not overwritten. The OOM record is the job cgroup's
# counter (memory.events) before and after the invocation plus the kernel log
# when it is readable (it is, as root on EC2).
set -eu

cell=$1
bin=$2
out=$3
: "${FB_DESIGNS_COMMIT:?set FB_DESIGNS_COMMIT to the commit the binary was built from}"

case $cell in
R256-A0) arm=A0 t=4 ;;
R256-A11) arm=A11 t=5 ;;
*) echo "unknown cell $cell (R256-A0 or R256-A11)" >&2; exit 2 ;;
esac
name="R256-$arm-m4-t$t"
mkdir -p "$out"
pre="$out/$name.preflight.txt"
# A cell has two targets. Rows already written are kept; the run continues from
# the first target without a row. (The first version of this script skipped a
# cell as soon as it had any row, so a host that died during the second target
# never resumed it: see Amendment 15, erratum.)
targets=2
rows=0
if [ -f "$out/$name.jsonl" ]; then
    rows=$(grep -c . "$out/$name.jsonl" || true)
fi
if [ "$rows" -ge "$targets" ]; then
    echo "$name already has its $rows rows; skipping"
    exit 0
fi

# Pre-flight: MemTotal, the shell's memory cgroup limit and nproc.
mem_kb=$(awk '/^MemTotal:/ {print $2}' /proc/meminfo)
cg_path=$(awk -F: '$1 == "0" || $2 ~ /(^|,)memory(,|$)/ {print $3; exit}' /proc/self/cgroup)
cg_kb=none
cg_dir=
for d in "/sys/fs/cgroup$cg_path" "/sys/fs/cgroup"; do
    if [ -r "$d/memory.max" ]; then
        v=$(cat "$d/memory.max")
        case $v in
        max) cg_kb=unlimited ;;
        *) cg_kb=$((v / 1024)) ;;
        esac
        cg_dir=$d
        break
    fi
done
if [ "$cg_kb" = none ] && [ -r "/sys/fs/cgroup/memory$cg_path/memory.limit_in_bytes" ]; then
    cg_kb=$(($(cat "/sys/fs/cgroup/memory$cg_path/memory.limit_in_bytes") / 1024))
    cg_dir="/sys/fs/cgroup/memory$cg_path"
fi
if [ "$cg_kb" = none ]; then
    echo "pre-flight: no readable memory cgroup limit under $cg_path; not running" >&2
    exit 3
fi
if [ "$cg_kb" != unlimited ] && [ "$cg_kb" -ge "$mem_kb" ]; then
    cg_kb=unlimited
fi

# V = min(cgroup_kB - 524288, MemTotal_kB - 524288) (Amendments 6, 12, 13, 14 and 15).
v_kb=$((mem_kb - 524288))
if [ "$cg_kb" != unlimited ] && [ $((cg_kb - 524288)) -lt "$v_kb" ]; then
    v_kb=$((cg_kb - 524288))
fi

oom_kills() {
    if [ -r "$cg_dir/memory.events" ]; then
        awk '$1 == "oom_kill" {print $2}' "$cg_dir/memory.events"
    elif [ -r "$cg_dir/memory.oom_control" ]; then
        awk '$1 == "oom_kill" {print $2}' "$cg_dir/memory.oom_control"
    else
        echo unreadable
    fi
}
oom_before=$(oom_kills)

# Read a baseline without clearing the shared node kernel ring buffer.
kernel_tmp=$(mktemp -d)
trap 'rm -rf "$kernel_tmp"' EXIT
kernel_baseline_readable=no
if dmesg > "$kernel_tmp/before" 2>/dev/null; then
    kernel_baseline_readable=yes
fi

{
    echo "== segment start: cell $name"
    echo "date $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "commit $FB_DESIGNS_COMMIT"
    echo "binary_sha256 $(sha256sum "$bin" | cut -d' ' -f1)"
    echo "rustc $(rustc --version 2>/dev/null || echo unknown)"
    echo "cargo $(cargo --version 2>/dev/null || echo unknown)"
    echo "runner_image ${FB_RUNNER_IMAGE:-unknown}"
    echo "runner_name ${RUNNER_NAME:-none}"
    echo "MemTotal_kB $mem_kb"
    echo "cgroup_path $cg_path"
    echo "cgroup_dir $cg_dir"
    echo "cgroup_limit_kB $cg_kb"
    echo "cgroup_oom_kill_before $oom_before"
    echo "nproc $(nproc)"
    echo "V_kB $v_kb"
    echo "kernel $(uname -r)"
    if [ -r /sys/devices/virtual/dmi/id/product_name ]; then
        echo "product $(cat /sys/devices/virtual/dmi/id/product_name)"
    fi
    grep -m1 '^model name' /proc/cpuinfo
} | tee -a "$pre"

# Do not clear the node-wide kernel ring buffer; use a read-only delta below.
"$bin" run --arms "$arm" --m 4 --t-min "$t" --t-max "$t" \
    --budget 14400 --grace 600 --vmem-kb "$v_kb" \
    --checkpoint-every "${FB_CKPT_EVERY:-600}" \
    --first-target "$rows" --targets "$((targets - rows))" \
    --file "R256-{arm}-m4-t$t" --out-dir "$out"

# Attribution: the job cgroup's OOM-kill counter, and the kernel log if
# this host lets the job read it.
{
    echo "== segment end: cell $name"
    echo "cgroup_oom_kill_before $oom_before"
    echo "cgroup_oom_kill_after $(oom_kills)"
    if [ "$kernel_baseline_readable" = yes ] && dmesg > "$kernel_tmp/after" 2>/dev/null; then
        oom=$(awk 'FILENAME == ARGV[1] { baseline[$0]++; next } { if (baseline[$0] > 0) baseline[$0]--; else print }' \
            "$kernel_tmp/before" "$kernel_tmp/after" |
            grep -E 'invoked oom-killer|memory: usage|Memory cgroup out of memory|Out of memory' || true)
        if [ -n "$oom" ]; then
            printf '%s\n' "$oom"
        else
            echo "no new kernel OOM lines"
        fi
    else
        echo "kernel log unreadable or baseline unavailable"
    fi
    echo "checked $(date -u +%Y-%m-%dT%H:%M:%SZ)"
} >> "$out/$name.oom.txt"
