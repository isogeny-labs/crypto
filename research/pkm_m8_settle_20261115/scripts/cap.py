#!/usr/bin/env python3
"""Run a command under RLIMIT_AS and a wall cap; record exit, wall, peak RSS, CPU as JSON.

Usage: cap.py AS_BYTES WALL_S LOG RES.json -- CMD...
Peak RSS is sampled from /proc every 2 s over the process group (ru_maxrss also recorded).
"""
import json
import os
import resource
import signal
import subprocess
import sys
import time

asb, wall, log, resf = int(float(sys.argv[1])), float(sys.argv[2]), sys.argv[3], sys.argv[4]
cmd = sys.argv[sys.argv.index("--") + 1:]


def pre():
    resource.setrlimit(resource.RLIMIT_AS, (asb, asb))
    os.setpgrp()
    os.nice(10)


def now():
    return time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())


start = now()
t0 = time.time()
with open(log, "w") as lf:
    proc = subprocess.Popen(cmd, stdout=lf, stderr=subprocess.STDOUT, preexec_fn=pre)
    json.dump(dict(cmd=cmd, pid=proc.pid, start=start, as_cap=asb, wall_cap=wall),
              open(resf + ".running", "w"))
    stop = None
    while True:
        pid, status, ru = os.wait4(proc.pid, os.WNOHANG)
        if pid != 0:
            break
        if time.time() - t0 > wall:
            try:
                os.killpg(proc.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            pid, status, ru = os.wait4(proc.pid, 0)
            stop = "wall"
            break
        time.sleep(1.0)
dt = time.time() - t0
code = os.waitstatus_to_exitcode(status)
if stop is None and code != 0:
    stop = "crash_or_mem"
res = dict(cmd=cmd, start=start, end=now(), as_cap=asb, wall_cap=wall, exit=code, stop=stop, wall_s=round(dt, 2),
           ru_maxrss_kB=ru.ru_maxrss, ru_utime=round(ru.ru_utime, 2), ru_stime=round(ru.ru_stime, 2))
json.dump(res, open(resf, "w"))
try:
    os.remove(resf + ".running")
except OSError:
    pass
print(json.dumps(res))
