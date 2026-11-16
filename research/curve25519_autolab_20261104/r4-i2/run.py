#!/usr/bin/env python3
"""Run msolve (F4, grevlex, reduced GB, 1 thread, -v 2, 2 GiB address space, 1200 s) on every system in
systems/, log to logs/<cell>.log with a trailing exit=/wall= line, and append the quotient dimension
(count of standard monomials of the reduced GB; msolve 0.10.1 -g 2 does not print it) to the log."""
import subprocess, sys, os, time, glob, re
from concurrent.futures import ThreadPoolExecutor
MS = "/tmp/claude-0/venv/lib/python3.13/site-packages/sage_wheels/bin/msolve"
HERE = os.path.dirname(os.path.abspath(__file__))
LOGDIR = os.environ.get("LOGDIR", "logs")
TL = int(os.environ.get("TLIMIT", "1200")); VL = int(os.environ.get("VLIMIT_KB", "2097152"))

def qdim(gb):
    """Number of standard monomials of the reduced GB (dimension of the quotient), or NA / inf?."""
    try: txt = open(gb).read()
    except OSError: return "NA"
    names = None
    for ln in txt.splitlines():
        if ln.startswith("#variable order:"): names = [x.strip() for x in ln.split(":", 1)[1].split(",")]
    if names is None: return "NA"
    idx = {n: i for i, n in enumerate(names)}
    body = txt.split("#---")[-1].strip().lstrip("[").rstrip(":").rstrip().rstrip("]")
    lead = []
    for poly in body.split(",\n"):
        poly = poly.strip()
        if not poly: continue
        term = re.split(r"(?<=[^\^])[+-](?=\d)", poly)[0]       # leading term
        mstr = term.split("*", 1)[1] if "*" in term else ""
        e = [0] * len(names)
        for f in mstr.split("*"):
            if f:
                n, _, ex = f.partition("^"); e[idx[n]] = int(ex or 1)
        lead.append(tuple(e))
    n = len(names); one_ = tuple([0] * n)
    def divisible(m): return any(all(m[i] >= l[i] for i in range(n)) for l in lead)
    if divisible(one_): return 0
    seen = {one_}; stack = [one_]; cnt = 0
    while stack:
        m = stack.pop(); cnt += 1
        if cnt > 200000: return "inf?"
        for i in range(n):
            mm = list(m); mm[i] += 1; mm = tuple(mm)
            if mm not in seen and not divisible(mm): seen.add(mm); stack.append(mm)
    return cnt

def one(f):
    b = f[:-3]; log = f"{HERE}/{LOGDIR}/{os.path.basename(b)}.log"
    if os.path.exists(log) and "exit=" in open(log).read(): return
    os.makedirs(f"{HERE}/{LOGDIR}", exist_ok=True); gb = f"{b}.gb"; t0 = time.time()
    cmd = f"ulimit -v {VL}; timeout {TL} {MS} -g 2 -t 1 -v 2 -f {f} -o {gb}"
    with open(log, "w") as lf:
        rc = subprocess.call(["bash", "-c", cmd], stdout=lf, stderr=subprocess.STDOUT)
    wall = time.time() - t0
    q = qdim(gb) if rc == 0 else "NA"
    with open(log, "a") as lf:
        lf.write(f"Dimension of quotient: {q}\nexit={rc} wall={wall:.2f}\n")
    if os.path.exists(gb): os.remove(gb)

if __name__ == "__main__":
    pat = sys.argv[1] if len(sys.argv) > 1 else "*"
    files = sorted(glob.glob(f"{HERE}/systems/{pat}.ms"), key=lambda s: (int(re.search(r"/d(\d+)_", s).group(1)), s))
    jobs = int(os.environ.get("JOBS", "3"))
    with ThreadPoolExecutor(jobs) as ex: list(ex.map(one, files))
