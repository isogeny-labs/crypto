#!/usr/bin/env python3
"""Run the PROTOCOL.md grid. One worker per (k, m, form) chain, d ascending;
stop a chain at the first capped / failed cell. Output: cells/<name>.json."""
import json, os, subprocess, sys, concurrent.futures as cf

BIN = os.environ.get("BIN", "/home/user/ctarget-f1/release/curve25519_family")
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "cells")
SEED = 20261108
CAP = 1200
N = 6
D20 = [3, 7, 9, 19, 21, 27, 57, 63, 73, 133, 171, 189, 219, 399, 511, 513]
CH = []  # (k, m, form, [d...], gidx)
for m in (2, 3, 4):
    CH.append((20, m, "sym", D20, 0))
for m in (2, 3):
    CH.append((20, m, "plain", [d for d in D20 if d <= 63], 0))
CH.append((20, 2, "sym", [27, 63], 1))
CH.append((20, 2, "sym", [27, 63], 2))
CH.append((22, 2, "sym", [3, 5, 11, 15, 25, 31, 33, 41, 55, 75, 93], 0))
CH.append((22, 3, "sym", [3, 5, 11, 15, 25, 31, 33, 41, 55, 75, 93], 0))
CH.append((24, 2, "sym", [3, 23, 69, 89, 267], 0))
CH.append((18, 2, "sym", [13, 71, 923], 0))

def run_chain(ch):
    k, m, form, ds, g = ch
    for d in ds:
        name = f"k{k}_m{m}_{form}_d{d}_g{g}"
        path = os.path.join(OUT, name + ".json")
        if os.path.exists(path):
            if not json.load(open(path)).get("capped"):
                continue
            return name + " (already censored)"
        cmd = f"ulimit -v 3500000; RAYON_NUM_THREADS=1 {BIN} f4sub {k} sub {d} {g} mont {m} {form} {N} {SEED + 1000*m + d} {CAP}"
        r = subprocess.run(["bash", "-c", cmd], capture_output=True, text=True)
        try:
            j = json.loads(r.stdout.strip().splitlines()[-1])
        except Exception:
            j = {"k": k, "m": m, "form": form, "d_or_t": d, "gidx": g, "capped": True,
                 "failure": "exit %d" % r.returncode, "stderr": r.stderr[-300:]}
        json.dump(j, open(path, "w"), indent=1)
        print(name, "capped" if j.get("capped") else j.get("mean_field_ops"), flush=True)
        if j.get("capped"):
            return name + " censored; larger d not attempted"
    return "done"

if __name__ == "__main__":
    os.makedirs(OUT, exist_ok=True)
    only = sys.argv[1:]  # optional substring filter
    chs = [c for c in CH if not only or any(o in f"k{c[0]}_m{c[1]}_{c[2]}_g{c[4]}" for o in only)]
    with cf.ThreadPoolExecutor(4) as ex:
        for res in ex.map(run_chain, chs):
            print("chain:", res, flush=True)
