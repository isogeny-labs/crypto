#!/bin/bash
set -euo pipefail
cg=$(awk -F: '$1=="0" {print $3}' /proc/self/cgroup)
path="/sys/fs/cgroup${cg}"
printf 'CGROUP=%s\nPATH=%s\n' "$cg" "$path"
if test -e "$path/cpu.max"; then printf 'CPU_MAX='; cat "$path/cpu.max"; else echo CPU_MAX=MISSING; fi
if test -e "$path/cpu.stat"; then echo CPU_STAT_BEGIN; cat "$path/cpu.stat"; echo CPU_STAT_END; else echo CPU_STAT=MISSING; fi
if test -e "$path/memory.events"; then echo MEMORY_EVENTS_BEGIN; cat "$path/memory.events"; echo MEMORY_EVENTS_END; else echo MEMORY_EVENTS=MISSING; fi
