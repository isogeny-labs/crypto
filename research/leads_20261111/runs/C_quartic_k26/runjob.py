#!/usr/bin/env python3
"""Run one job: stdout -> <out>.jsonl, stderr -> <out>.err, resources -> <out>.res.json.

usage: runjob.py <out-prefix> <timeout-s> <cmd> [args...]
Peak RSS (ru_maxrss, KiB) and user/sys CPU come from wait4; this container has no /usr/bin/time.
"""
import json
import os
import subprocess
import sys
import time

out, tmo, cmd = sys.argv[1], float(sys.argv[2]), sys.argv[3:]
env = dict(os.environ, RAYON_NUM_THREADS="1")
t0 = time.time()
with open(out + ".jsonl", "w") as fo, open(out + ".err", "w") as fe:
    p = subprocess.Popen(cmd, stdout=fo, stderr=fe, env=env)
    killed = False
    killed_rss = False
    cap = int(os.environ.get("RSS_CAP_KIB", "0"))  # optional hard memory cap (kill above it)
    while True:
        pid, status, ru = os.wait4(p.pid, os.WNOHANG)
        if pid:
            break
        if time.time() - t0 > tmo:
            p.kill()
            killed = True
        if cap:
            try:
                with open(f"/proc/{p.pid}/status") as st:
                    rss = int([l for l in st if l.startswith("VmRSS:")][0].split()[1])
                if rss > cap:
                    p.kill()
                    killed_rss = True
            except (OSError, IndexError, ValueError):
                pass
        time.sleep(0.25)
res = {
    "cmd": cmd,
    "rc": os.waitstatus_to_exitcode(status),
    "killed_timeout": killed,
    "killed_rss_cap": killed_rss,
    "rss_cap_kib": int(os.environ.get("RSS_CAP_KIB", "0")),
    "wall_s": round(time.time() - t0, 3),
    "user_s": ru.ru_utime,
    "sys_s": ru.ru_stime,
    "maxrss_kib": ru.ru_maxrss,
    "start_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(t0)),
    "load1_start": os.getloadavg()[0],
}
with open(out + ".res.json", "w") as f:
    json.dump(res, f)
print(json.dumps(res))
