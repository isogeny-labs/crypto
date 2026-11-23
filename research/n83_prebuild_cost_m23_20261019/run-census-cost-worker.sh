#!/bin/sh
set -eu
output=$1
binary=$2
cat /proc/self/cgroup > "$output/cgroup.txt"
cgroup=$(sed -n 's/^0:://p' "$output/cgroup.txt")
for setting in memory.max memory.swap.max cpu.max; do
    cat "/sys/fs/cgroup$cgroup/$setting" > "$output/$setting.txt"
done
exec "$binary" "$output/generated" 18
