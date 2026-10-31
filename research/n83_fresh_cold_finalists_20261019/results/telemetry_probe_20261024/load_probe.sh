#!/bin/bash
set -euo pipefail
cg=$(awk -F: '$1=="0" {print $3}' /proc/self/cgroup)
path="/sys/fs/cgroup${cg}"
echo "CGROUP=$cg"
echo "CPU_MAX=$(cat "$path/cpu.max")"
before_nr=$(awk '$1=="nr_throttled" {print $2}' "$path/cpu.stat")
before_steal=$(awk '/^cpu / {print $9}' /proc/stat)
python3 - <<'PYWORK'
import multiprocessing,time
deadline=time.monotonic()+60

def burn():
    x=1
    while time.monotonic()<deadline:
        x=(1664525*x+1013904223)&0xffffffff

workers=[multiprocessing.Process(target=burn) for _ in range(2)]
for worker in workers: worker.start()
for worker in workers: worker.join()
assert all(worker.exitcode==0 for worker in workers)
print('TWO_PROCESS_LOAD_DONE')
PYWORK
after_nr=$(awk '$1=="nr_throttled" {print $2}' "$path/cpu.stat")
after_steal=$(awk '/^cpu / {print $9}' /proc/stat)
echo "NR_THROTTLED_DELTA=$((after_nr-before_nr))"
echo "STEAL_JIFFIES_DELTA=$((after_steal-before_steal))"
echo CPU_STAT_AFTER_BEGIN
cat "$path/cpu.stat"
echo CPU_STAT_AFTER_END
echo MEMORY_EVENTS_AFTER_BEGIN
cat "$path/memory.events"
echo MEMORY_EVENTS_AFTER_END
