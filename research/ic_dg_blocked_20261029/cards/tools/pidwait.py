# Usage: python3 -I pidwait.py SECONDS PIDFILE [PIDFILE ...]
# A PIDFILE named .../pid-<tag> belongs to the process whose command line holds
# both "launch-remote" and "request-<tag>.json". Prints one line per PIDFILE and
# a final PAIR_DONE (all exited) or PAIR_RUNNING (some alive after SECONDS).
# Polls every 15 s. Completion is read from /proc/<pid> only, never from pgrep.
import os, sys, time

def alive(pidfile):
    tag = os.path.basename(pidfile)[len("pid-"):]
    want = ("request-%s.json" % tag).encode()
    pid = open(pidfile).read().strip()
    if not pid.isdigit():
        return None
    try:
        cmd = open("/proc/%s/cmdline" % pid, "rb").read()
        stat = open("/proc/%s/stat" % pid).read()
    except (FileNotFoundError, ProcessLookupError):
        return False
    if b"launch-remote" not in cmd or want not in cmd:
        return False
    return stat[stat.rindex(")") + 2] != "Z"

limit = int(sys.argv[1])
files = sys.argv[2:]
t0 = time.time()
while True:
    st = {f: alive(f) for f in files}
    if all(v is False for v in st.values()) or time.time() - t0 >= limit:
        break
    time.sleep(15)
for f in files:
    print("PID_STATE", os.path.basename(f), {True: "alive", False: "exited", None: "BAD_PIDFILE"}[st[f]])
print("PAIR_DONE" if all(v is False for v in st.values()) else "PAIR_RUNNING")
