#!/usr/bin/env bash
set -euo pipefail

# Run as root after booting with isolcpus/nohz_full/rcu_nocbs/irqaffinity
# for the physical core with logical CPUs 23 and 47.
test "$(id -u)" -eq 0
test "$(cat /sys/devices/system/cpu/cpu23/topology/thread_siblings_list)" = 23,47
test "$(cat /sys/devices/system/cpu/cpu47/topology/thread_siblings_list)" = 23,47
test "$(systemd-detect-virt)" = none

housekeeping=0-22,24-46
benchmark=23,47
systemctl stop irqbalance.service || true
systemctl disable irqbalance.service || true
systemctl set-property --runtime system.slice AllowedCPUs="$housekeeping"
systemctl set-property --runtime user.slice AllowedCPUs="$housekeeping"
systemctl set-property --runtime init.scope AllowedCPUs="$housekeeping"

for irq in /proc/irq/[0-9]*; do
    test -f "$irq/effective_affinity_list" || continue
    printf '%s\n' "$housekeeping" > "$irq/smp_affinity_list" || true
done

for cpu in 23 47; do
    base=/sys/devices/system/cpu/cpu${cpu}/cpufreq
    printf '%s\n' performance > "$base/scaling_governor"
    high=$(cat "$base/scaling_max_freq")
    printf '%s\n' "$high" > "$base/scaling_min_freq"
done

root=/sys/fs/cgroup
if ! grep -qw cpuset "$root/cgroup.subtree_control"; then
    printf '%s\n' +cpuset > "$root/cgroup.subtree_control"
fi
mkdir -p "$root/rho-tier0"
printf '%s\n' 1 > "$root/rho-tier0/cpuset.mems"
printf '%s\n' "$benchmark" > "$root/rho-tier0/cpuset.cpus"
printf '%s\n' "$benchmark" > "$root/rho-tier0/cpuset.cpus.exclusive"
printf '%s\n' isolated > "$root/rho-tier0/cpuset.cpus.partition"

printf '%s\n' 'configured physical core 23,47 and NUMA node 1'
cat "$root/rho-tier0/cpuset.cpus.partition"
cat "$root/rho-tier0/cpuset.cpus.exclusive.effective"
cat "$root/cpuset.cpus.isolated"
