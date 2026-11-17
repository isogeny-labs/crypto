#!/usr/bin/env python3
"""Run the extension grid (PROTOCOL.md): sequential, one process at a time, resumable.
Output: cells/<name>.json in the parent's format plus load/wall-time fields."""
import hashlib, json, os, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.join(HERE, "..")
PARENT = os.path.join(ROOT, "..", "curve25519_exponent_study_20261108", "cells")
OUT = os.path.join(ROOT, "cells")
BIN = os.environ.get("BIN", "/home/user/bin-archive/curve25519_family-dc2c30a4")
SEED = 20261108
CAP = 10800
MEM_KB = 12000000
N = 6
# (k, m, [d ascending, extension cells only]); the chain rule reads parent cells too.
CHAINS = [
    (20, 3, [12, 14, 18]),
    (20, 4, [12, 14, 18]),
    (20, 3, [28, 36, 38, 42, 54, 57, 63, 73, 133]),
    (20, 4, [19, 21, 27, 28, 36, 38, 42, 54, 57]),
    (22, 3, [55, 75, 93]),
]

def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for b in iter(lambda: f.read(1 << 20), b""):
            h.update(b)
    return h.hexdigest()

def load1():
    try:
        return os.getloadavg()
    except OSError:
        return None

def cell_state(k, m, d):
    """'complete' | 'censored' | None, reading extension then parent cells."""
    name = f"k{k}_m{m}_sym_d{d}_g0.json"
    for base, tag in ((OUT, "ext"), (PARENT, "parent")):
        p = os.path.join(base, name)
        if os.path.exists(p):
            j = json.load(open(p))
            if j.get("capped"):
                # a parent-censored cell is rerun by the extension; an extension-censored cell ends the chain
                return ("censored" if tag == "ext" else None)
            return "complete"
    return None

def run_cell(k, m, d):
    name = f"k{k}_m{m}_sym_d{d}_g0"
    path = os.path.join(OUT, name + ".json")
    cmd = (f"ulimit -v {MEM_KB}; RAYON_NUM_THREADS=1 {BIN} f4sub {k} sub {d} 0 mont {m} sym {N} "
           f"{SEED + 1000 * m + d} {CAP}")
    l0, t0 = load1(), time.time()
    r = subprocess.run(["bash", "-c", cmd], capture_output=True, text=True)
    t1, l1 = time.time(), load1()
    try:
        j = json.loads(r.stdout.strip().splitlines()[-1])
    except Exception:
        j = {"k": k, "m": m, "form": "sym", "d_or_t": d, "gidx": 0, "capped": True,
             "failure": "exit %d" % r.returncode, "stderr": r.stderr[-400:]}
    j["ext"] = {"study": "curve25519_exponent_study_ext_20261110", "cap_s": CAP, "mem_kb": MEM_KB,
                "load_before": l0, "load_after": l1, "wall_s": round(t1 - t0, 1),
                "started_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(t0)),
                "bin_sha256": BIN_SHA, "exit": r.returncode}
    tmp = path + ".tmp"
    json.dump(j, open(tmp, "w"), indent=1)
    os.replace(tmp, path)
    print(time.strftime("%H:%M:%SZ", time.gmtime()), name,
          "capped" if j.get("capped") else j.get("mean_field_ops"), f"{t1 - t0:.0f}s", flush=True)
    return not j.get("capped")

if __name__ == "__main__":
    os.makedirs(OUT, exist_ok=True)
    BIN_SHA = sha256(BIN)
    want = [ln.split()[0] for ln in open(os.path.join(ROOT, "data", "frozen.sha256"))
            if ln.strip().endswith("curve25519_family")][0]
    if BIN_SHA != want:
        sys.exit(f"binary sha256 {BIN_SHA} != frozen {want}")
    print("binary ok", BIN_SHA[:16], "cap", CAP, "mem_kb", MEM_KB, flush=True)
    for k, m, ds in CHAINS:
        for d in ds:
            st = cell_state(k, m, d)
            if st == "complete":
                continue
            if st == "censored":
                print(f"chain k{k} m{m}: censored at d={d}; larger d not attempted", flush=True)
                break
            if not run_cell(k, m, d):
                print(f"chain k{k} m{m}: censored at d={d}; larger d not attempted", flush=True)
                break
    print("ALL_DONE", flush=True)
