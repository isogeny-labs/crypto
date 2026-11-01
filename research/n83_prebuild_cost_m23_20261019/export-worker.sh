#!/bin/sh
set -eu
output=$1
printf 'memory.max=' > "$output/cgroup.txt"
cat /sys/fs/cgroup/memory.max >> "$output/cgroup.txt"
printf 'memory.swap.max=' >> "$output/cgroup.txt"
cat /sys/fs/cgroup/memory.swap.max >> "$output/cgroup.txt"
printf 'cpu.max=' >> "$output/cgroup.txt"
cat /sys/fs/cgroup/cpu.max >> "$output/cgroup.txt"
exec /home/ec2-user/n83-d18-paired-20261019/target/release/examples/n83_d18_subgroup_orbit_export export "$output/generated"
