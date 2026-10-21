#!/bin/sh
# Amendment 7: run one m = 4 cell (X1 or X2) on a large-memory host.
#
#   run_amendment7.sh CELL BINARY OUT_DIR
#
# CELL is X1 (A0, m = 4, t = 4) or X2 (A11, m = 4, t = 5). BINARY is a release
# build of examples/fb_designs_slope.rs; FB_DESIGNS_COMMIT names the commit it
# was built from. The script needs only sh, coreutils and dmesg: no checkout,
# no Rust toolchain. Run it as root on an otherwise idle host so dmesg is
# readable; it refuses to start if the pre-flight cannot read a memory limit.
set -eu

cell=$1
bin=$2
out=$3
: "${FB_DESIGNS_COMMIT:?set FB_DESIGNS_COMMIT to the commit the binary was built from}"

case $cell in
X1) arm=A0 t=4 ;;
X2) arm=A11 t=5 ;;
*) echo "unknown cell $cell (X1 or X2)" >&2; exit 2 ;;
esac
name="$cell-$arm-m4-t$t"
mkdir -p "$out"
pre="$out/$name.preflight.txt"

# Pre-flight (Amendment 6, kept): MemTotal, the shell's memory cgroup limit
# and nproc, recorded before the first row.
mem_kb=$(awk '/^MemTotal:/ {print $2}' /proc/meminfo)
cg_path=$(awk -F: '$1 == "0" || $2 ~ /(^|,)memory(,|$)/ {print $3; exit}' /proc/self/cgroup)
cg_kb=none
for f in "/sys/fs/cgroup$cg_path/memory.max" "/sys/fs/cgroup/memory$cg_path/memory.limit_in_bytes"; do
    if [ -r "$f" ]; then
        v=$(cat "$f")
        case $v in
        max) cg_kb=unlimited ;;
        *) cg_kb=$((v / 1024)) ;;
        esac
        cg_file=$f
        break
    fi
done
if [ "$cg_kb" = none ]; then
    echo "pre-flight: no readable memory cgroup limit under $cg_path; not running" >&2
    exit 3
fi
# A cgroup v1 "no limit" is a huge number; treat anything above MemTotal as none.
if [ "$cg_kb" != unlimited ] && [ "$cg_kb" -ge "$mem_kb" ]; then
    cg_kb=unlimited
fi

# V = min(cgroup_kB - 524288, MemTotal_kB - 524288) (Amendment 7).
v_kb=$((mem_kb - 524288))
if [ "$cg_kb" != unlimited ] && [ $((cg_kb - 524288)) -lt "$v_kb" ]; then
    v_kb=$((cg_kb - 524288))
fi

{
    echo "cell $name"
    echo "date $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "commit $FB_DESIGNS_COMMIT"
    echo "binary_sha256 $(sha256sum "$bin" | cut -d' ' -f1)"
    echo "MemTotal_kB $mem_kb"
    echo "cgroup_path $cg_path"
    echo "cgroup_file ${cg_file:-}"
    echo "cgroup_limit_kB $cg_kb"
    echo "nproc $(nproc)"
    echo "V_kB $v_kb"
    echo "kernel $(uname -r)"
    if [ -r /sys/devices/virtual/dmi/id/product_name ]; then
        echo "product $(cat /sys/devices/virtual/dmi/id/product_name)"
    fi
    grep -m1 '^model name' /proc/cpuinfo
} | tee "$pre"

dmesg -c > /dev/null 2>&1 || true
"$bin" run --arms "$arm" --m 4 --t-min "$t" --t-max "$t" \
    --budget 14400 --grace 600 --vmem-kb "$v_kb" \
    --file "$cell-{arm}-m4-t$t" --out-dir "$out"

# Attribution check (Amendment 6, kept): the kernel log's OOM lines, verbatim.
oom=$(dmesg 2>/dev/null | grep -E 'invoked oom-killer|memory: usage|Memory cgroup out of memory|Out of memory' || true)
if [ -n "$oom" ]; then
    printf '%s\n' "$oom" > "$out/$name.dmesg.txt"
else
    echo "no kernel OOM lines for $name ($(date -u +%Y-%m-%dT%H:%M:%SZ))" > "$out/$name.dmesg.txt"
fi
