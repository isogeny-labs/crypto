#!/usr/bin/env python3
"""Does this container expose hardware counters? (research/ic_leads_rented_20261018, item 5)

Reads /proc/sys/kernel/perf_event_paranoid, lists the PMU event sources, and
opens PERF_COUNT_HW_CPU_CYCLES and PERF_COUNT_HW_INSTRUCTIONS (user space only,
this process) with perf_event_open(2) through ctypes, counts a fixed loop, and
prints one JSON object. Exit 0 if both counters opened and counted, 3 if not.
Plumbing only: no measurement here is a result about any algorithm.
"""
import ctypes, json, os, platform, struct, sys, time

NR = {"x86_64": 298, "aarch64": 241}.get(platform.machine())
libc = ctypes.CDLL(None, use_errno=True)
libc.syscall.restype = ctypes.c_long
libc.ioctl.argtypes = [ctypes.c_int, ctypes.c_ulong, ctypes.c_ulong]
ENABLE, DISABLE, RESET = 0x2400, 0x2401, 0x2403

def read(path):
    try:
        with open(path) as f:
            return f.read().strip()
    except OSError as e:
        return f"unreadable: {e.strerror}"

def open_hw(config, kind=0):
    """One counter (PERF_TYPE_HARDWARE unless kind says otherwise), disabled, user space only, this thread."""
    attr = bytearray(128)
    struct.pack_into("IIQ", attr, 0, kind, 128, config)       # type, size, config
    struct.pack_into("Q", attr, 32, 1 | 2)                  # read_format: time enabled, running
    struct.pack_into("Q", attr, 40, 1 | 1 << 5 | 1 << 6)    # disabled, exclude_kernel, exclude_hv
    buf = ctypes.create_string_buffer(bytes(attr), 128)
    fd = libc.syscall(ctypes.c_long(NR), buf, ctypes.c_int(0), ctypes.c_int(-1), ctypes.c_int(-1), ctypes.c_ulong(0))
    err = ctypes.get_errno()
    return int(fd), (os.strerror(err) if fd < 0 else None)

def main():
    rec = {
        "schema": "pmu-probe/v1",
        "date": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "host": platform.node(), "kernel": platform.release(), "machine": platform.machine(),
        "pod": os.environ.get("RUNPOD_POD_ID"),
        "cpu_model": next((l.split(":", 1)[1].strip() for l in read("/proc/cpuinfo").splitlines() if l.startswith("model name")), None),
        "nproc": os.cpu_count(),
        "perf_event_paranoid": read("/proc/sys/kernel/perf_event_paranoid"),
        "event_sources": sorted(os.listdir("/sys/bus/event_source/devices")) if os.path.isdir("/sys/bus/event_source/devices") else None,
        "cgroup_memory_max": read("/sys/fs/cgroup/memory.max"),
        "counters": {},
    }
    ok = NR is not None
    fds = {}
    # --selftest: the same code path on two software events (task-clock,
    # page-faults), which every Linux host has; a check of this script only.
    events = ((("task_clock", 1, 1), ("page_faults", 2, 1)) if "--selftest" in sys.argv
              else (("cycles", 0, 0), ("instructions", 1, 0)))
    rec["selftest"] = "--selftest" in sys.argv
    for name, cfg, kind in events:
        fd, err = open_hw(cfg, kind) if NR else (-1, "unknown syscall number")
        rec["counters"][name] = {"opened": fd >= 0, "error": err}
        ok &= fd >= 0
        if fd >= 0:
            fds[name] = fd
    for fd in fds.values():
        libc.ioctl(fd, RESET, 0); libc.ioctl(fd, ENABLE, 0)
    x = 0
    for i in range(2_000_000):   # a fixed user-space loop
        x = (x * 6364136223846793005 + i) & 0xFFFFFFFFFFFFFFFF
    for fd in fds.values():
        libc.ioctl(fd, DISABLE, 0)
    for name, fd in fds.items():
        value, enabled, running = struct.unpack("QQQ", os.read(fd, 24))
        rec["counters"][name].update(value=value, time_enabled_ns=enabled, time_running_ns=running,
                                     multiplexed=running < enabled)
        ok &= value > 0
        os.close(fd)
    c, i = rec["counters"].get("cycles", {}), rec["counters"].get("instructions", {})
    if rec["selftest"]:
        ok &= all(v.get("value", 0) > 0 for v in rec["counters"].values())
    rec["ipc_of_probe_loop"] = (i["value"] / c["value"]) if c.get("value") and i.get("value") else None
    rec["status"] = ("selftest_ok" if ok else "selftest_failed") if rec["selftest"] else ("counters_available" if ok else "unsupported")
    print(json.dumps(rec, indent=1))
    return 0 if ok else 3

if __name__ == "__main__":
    sys.exit(main())
