#!/usr/bin/env python3
"""Run the three frozen cells of PROTOCOL.md; writes cells/<name>.json in the exponent studies' format."""
import hashlib, json, os, subprocess, sys, time
H = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(H, "cells"); os.makedirs(OUT, exist_ok=True)
BIN = "/home/user/bin-archive/curve25519_family-dc2c30a4"; K, M, N, CAP, MEM_KB = 22, 2, 2, 1800, 12000000
sha = hashlib.sha256(open(BIN, "rb").read()).hexdigest()
assert sha.startswith("dc2c30a4242d8028"), sha
print("binary ok", sha[:16], flush=True)
for d in (341, 825, 1271):
    name = f"k{K}_m{M}_sym_d{d}_g0"; path = os.path.join(OUT, name + ".json")
    if os.path.exists(path): continue
    cmd = f"ulimit -v {MEM_KB}; RAYON_NUM_THREADS=1 {BIN} f4sub {K} sub {d} 0 mont {M} sym {N} {20261108 + 1000 * M + d} {CAP}"
    t0 = time.time(); r = subprocess.run(["bash", "-c", cmd], capture_output=True, text=True); t1 = time.time()
    try:
        j = json.loads(r.stdout.strip().splitlines()[-1])
    except Exception:
        j = {"k": K, "m": M, "form": "sym", "d_or_t": d, "gidx": 0, "capped": True, "failure": "exit %d" % r.returncode, "stderr": r.stderr[-400:]}
    j["ext"] = {"study": "curve25519_leads_wave6_20261114/sdrule", "cap_s": CAP, "mem_kb": MEM_KB, "wall_s": round(t1 - t0, 1),
                "started_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(t0)), "bin_sha256": sha, "exit": r.returncode}
    json.dump(j, open(path, "w"), indent=1)
    sd = [t.get("solving_degree_max") for t in j.get("per_target", [])]
    print(time.strftime("%H:%M:%SZ", time.gmtime()), name, "capped" if j.get("capped") else f"SD {sd} T {j.get('mean_field_ops'):.3g}", f"{t1 - t0:.0f}s", flush=True)
print("ALL_DONE", flush=True)
