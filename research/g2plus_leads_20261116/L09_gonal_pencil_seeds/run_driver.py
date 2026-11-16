#!/usr/bin/env python3
"""Runs PROTOCOL section 9 commands unchanged, logging command, exit code, CPU seconds and sha256 of outputs."""
import subprocess, sys, hashlib, os, time, resource
os.chdir(os.path.dirname(os.path.abspath(__file__)))
B = "code/target/release/l09"
LOG = "raw/COMMANDS.log" if len(sys.argv) < 3 else sys.argv[2]
LIMIT = 600  # CPU seconds
def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()
def log(s):
    with open(LOG, "a") as f:
        f.write(s + "\n")
def run(argv, out):
    cmd = " ".join(argv) + " > " + out
    r0 = resource.getrusage(resource.RUSAGE_CHILDREN)
    t0 = time.time()
    with open(out, "w") as fo, open("raw/stderr.log", "a") as fe:
        try:
            pr = subprocess.run(argv, stdout=fo, stderr=fe, timeout=LIMIT * 1.5)
            rc = pr.returncode
        except subprocess.TimeoutExpired:
            rc = "TIMEOUT"
    r1 = resource.getrusage(resource.RUSAGE_CHILDREN)
    cpu = (r1.ru_utime - r0.ru_utime) + (r1.ru_stime - r0.ru_stime)
    log(f"CMD: {cmd}\n  exit={rc} cpu_s={cpu:.1f} wall_s={time.time()-t0:.1f} maxrss_kb={r1.ru_maxrss}\n  sha256={sha(out)}  {out}")
    if rc == "TIMEOUT" or cpu > LIMIT:
        with open("raw/overruns.txt", "a") as f:
            f.write(f"OVERRUN: {cmd} rc={rc} cpu_s={cpu:.1f}\n")
    return rc
which = sys.argv[1]
if which == "census":
    for p in (251, 1021, 4093, 16381):
        run([B, "census", "--p", str(p), "--seeds", "20"], f"raw/census_p{p}.jsonl")
elif which == "exact":
    for b in range(6):
        run([B, "exact", "--p", "251", "--beta-idx", str(b), "--seeds", "20", "--arms", "A,B,B3"], f"raw/exact_p251_b{b}.jsonl")
    for b in (3, 4, 5):
        run([B, "exact", "--p", "1021", "--beta-idx", str(b), "--seeds", "20", "--arms", "A,B,B3"], f"raw/exact_p1021_b{b}.jsonl")
elif which == "cell":
    for p in (251, 1021, 4093, 16381):
        for b in range(6):
            run([B, "cell", "--p", str(p), "--beta-idx", str(b), "--seeds", "20", "--arms", "A,A2,B,PC1,PC4"], f"raw/cell_p{p}_b{b}.jsonl")
    run([B, "cell", "--p", "1021", "--beta-idx", "2", "--seeds", "2", "--arms", "A,A2,B,PC1,PC4"], "raw/g6_rerun.jsonl")
