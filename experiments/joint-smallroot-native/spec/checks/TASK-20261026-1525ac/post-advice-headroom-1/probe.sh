#!/bin/sh
# Readback/empty-cgroup probe only. No supervisor, worker, allocator or advice.
set -eu
record=/records
snapshot() {
  level=$1
  output=$2
  mkdir "$output"
  printf '%s\n' "$level" > "$output/mapping"
  for key in memory.max memory.current memory.swap.max memory.swap.current memory.events memory.peak memory.stat cpu.max cpu.stat pids.max pids.current cgroup.procs cgroup.events cgroup.controllers cgroup.subtree_control; do
    cat "$level/$key" > "$output/$key" 2> "$output/$key.read-error" || true
  done
}
date -u +%Y-%m-%dT%H:%M:%SZ > "$record/started"
cat /proc/1/stat > "$record/init.proc-stat"
cat /proc/1/cgroup > "$record/init.cgroup"
sed -n '/^Uid:/p; /^Gid:/p; /^Cap/p; /^Seccomp/p; /^NoNewPrivs/p' /proc/1/status > "$record/init.identity"
cat /proc/meminfo > "$record/meminfo-before"
snapshot /sys/fs/cgroup "$record/container-before"
mkdir /sys/fs/cgroup/qualifier
printf '%s\n' "$$" > /sys/fs/cgroup/qualifier/cgroup.procs
printf '%s\n' '+memory +cpu +pids' > /sys/fs/cgroup/cgroup.subtree_control
mkdir /sys/fs/cgroup/delegated
printf '%s\n' '+memory +cpu +pids' > /sys/fs/cgroup/delegated/cgroup.subtree_control
snapshot /sys/fs/cgroup "$record/container-after-bootstrap"
snapshot /sys/fs/cgroup/delegated "$record/delegated-enclosing"
mkdir /sys/fs/cgroup/delegated/worker-cap-probe
printf '%s\n' 8589934592 > /sys/fs/cgroup/delegated/worker-cap-probe/memory.max
snapshot /sys/fs/cgroup/delegated/worker-cap-probe "$record/empty-worker"
rmdir /sys/fs/cgroup/delegated/worker-cap-probe
rmdir /sys/fs/cgroup/delegated
cat /proc/meminfo > "$record/meminfo-after"
snapshot /sys/fs/cgroup "$record/container-after-probe"
printf '%s\n' 'readbacks_completed; empty8GiBgroup_removed; worker_launches=0; allocator_launches=0; science_calls=0; hidden_ancestors=unobserved' > "$record/result"
chmod -R a+rX /records
chown -R 1000:1000 /records
