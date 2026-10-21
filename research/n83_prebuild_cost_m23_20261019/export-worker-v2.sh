#!/bin/sh
set -eu
output=$1
cgroup_path=$(cut -d: -f3 /proc/self/cgroup)
cgroup_root="/sys/fs/cgroup$cgroup_path"
printf 'cgroup_path=%s\n' "$cgroup_path" > "$output/cgroup.txt"
for name in memory.max memory.swap.max cpu.max; do
    printf '%s=' "$name" >> "$output/cgroup.txt"
    cat "$cgroup_root/$name" >> "$output/cgroup.txt"
done
exec /home/ec2-user/n83-d18-paired-20261019/target/release/examples/n83_d18_subgroup_orbit_export export "$output/generated"
