#!/usr/bin/env python3
"""Record the exposed CPU and memory envelope before an N83 Tier 1 run."""

import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import platform


CGROUP = Path("/sys/fs/cgroup")
GIB = 1024 ** 3


def read(path):
    return path.read_text().strip()


def mem_total_bytes():
    for line in Path("/proc/meminfo").read_text().splitlines():
        if line.startswith("MemTotal:"):
            return int(line.split()[1]) * 1024
    raise ValueError("MemTotal is absent")


def cpu_model():
    for line in Path("/proc/cpuinfo").read_text().splitlines():
        if line.startswith("model name"):
            return line.split(":", 1)[1].strip()
    return None


def proc_cpu_counters():
    fields = Path("/proc/stat").read_text().splitlines()[0].split()
    if fields[0] != "cpu" or len(fields) < 9:
        raise ValueError("aggregate CPU counters differ")
    return {"raw": " ".join(fields), "steal_ticks": int(fields[8])}


def cgroup_snapshot(root=CGROUP):
    if (root / "cgroup.controllers").exists():
        cpu_max = read(root / "cpu.max")
        memory_max = read(root / "memory.max")
        return {"interface": "v2", "cpu_max": cpu_max,
                "memory_max": memory_max,
                "cpu_stat": read(root / "cpu.stat"),
                "memory_events": read(root / "memory.events"),
                "memory_current": read(root / "memory.current"),
                "cpu_pressure": read(root / "cpu.pressure") if
                (root / "cpu.pressure").exists() else None,
                "memory_pressure": read(root / "memory.pressure") if
                (root / "memory.pressure").exists() else None,
                "cpuset_cpus_effective": read(root / "cpuset.cpus.effective")}
    cpu = root / "cpu"
    memory = root / "memory"
    return {"interface": "v1",
            "cpu_max": (read(cpu / "cpu.cfs_quota_us") + " " +
                        read(cpu / "cpu.cfs_period_us")),
            "memory_max": read(memory / "memory.limit_in_bytes"),
            "cpu_stat": read(cpu / "cpu.stat"),
            "memory_events": ("failcnt " + read(memory / "memory.failcnt") +
                              "\noom_control " +
                              read(memory / "memory.oom_control")),
            "memory_current": read(memory / "memory.usage_in_bytes"),
            "cpuset_cpus_effective": read(root / "cpuset/cpuset.cpus")}


def gate(snapshot, physical_memory, affinity):
    quota, period = snapshot["cpu_max"].split()
    if not period.isdigit() or int(period) <= 0:
        raise ValueError("CPU quota period differs")
    cpu_count = (len(affinity) if quota in ("max", "-1") else
                 int(quota) / int(period))
    memory_max = snapshot["memory_max"]
    memory_limit = (physical_memory if memory_max in ("max", "-1") else
                    min(physical_memory, int(memory_max)))
    return {"status": "PASS_CAPACITY_PREFLIGHT" if
            len(affinity) >= 4 and cpu_count >= 4 and
            memory_limit >= 48 * GIB else "REJECTED_HOST_CAPACITY",
            "effective_cpu_count": cpu_count,
            "effective_memory_bytes": memory_limit,
            "frozen_online_affinity": affinity[:4] if len(affinity) >= 4 else []}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--allocation", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    allocation = json.loads(args.allocation.read_text())
    if allocation.get("schema") != "n83.maxk-runpod-allocation/v1":
        raise ValueError("allocation identity differs")
    if os.environ.get("RUNPOD_POD_ID") not in (None, allocation["pod_id"]):
        raise ValueError("Pod environment differs from allocation")
    affinity = sorted(os.sched_getaffinity(0))
    snapshot = cgroup_snapshot()
    physical_memory = mem_total_bytes()
    result = {"schema": "n83.maxk-pod-probe/v1",
              "at_utc": datetime.now(timezone.utc).isoformat(),
              "pod_id": allocation["pod_id"],
              "machine_id": allocation["machine_id"],
              "provider_rate_usd_hr": allocation["cost_per_hr_usd"],
              "os": platform.platform(),
              "architecture": platform.machine(),
              "cpu_model": cpu_model(),
              "proc_cpu_counters": proc_cpu_counters(),
              "visible_affinity": affinity,
              "physical_memory_bytes": physical_memory,
              "cgroup": snapshot,
              **gate(snapshot, physical_memory, affinity)}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open("x") as output:
        json.dump(result, output, indent=2, sort_keys=True)
        output.write("\n")
    print(json.dumps({"status": result["status"],
                      "effective_cpu_count": result["effective_cpu_count"],
                      "effective_memory_bytes": result["effective_memory_bytes"]}))
    return 0 if result["status"] == "PASS_CAPACITY_PREFLIGHT" else 2


if __name__ == "__main__":
    raise SystemExit(main())
