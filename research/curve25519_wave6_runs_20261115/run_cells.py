#!/usr/bin/env python3
"""Generic resumable cell runner for L1, L3, L4: one f4sub process at a time, cells in the given order, a chain
stops at its first censored cell. Records the binary sha256, limits, load and wall time per cell.
Usage: run_cells.py <lead_dir> <bin> <mem_kb> <cap_s> <N> <chain_spec>...
  chain_spec = "<miniature>:<kind>:<form>:<m>:<d1,d2,...>[:env=KEY=VAL]"   (seed = 20261108 + 1000*m + d)"""
import hashlib, json, os, subprocess, sys, time
lead, BIN, MEM_KB, CAP, N = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5])
OUT = os.path.join(lead, "cells"); os.makedirs(OUT, exist_ok=True)
sha = hashlib.sha256(open(BIN, "rb").read()).hexdigest()
print("binary", BIN, sha, "cap", CAP, "mem_kb", MEM_KB, flush=True)
def load1():
    return os.getloadavg()[0]
for spec in sys.argv[6:]:
    parts = spec.split(":"); mini, kind, form, m, ds = parts[0], parts[1], parts[2], int(parts[3]), [int(x) for x in parts[4].split(",")]
    env = dict(os.environ)
    for extra in parts[5:]:
        if extra.startswith("env="):
            k, v = extra[4:].split("=", 1); env[k] = v
    tag = mini.replace("=", "").replace(",", "_")
    for d in ds:
        name = f"{tag}_{kind}_m{m}_{form}_d{d}"
        path = os.path.join(OUT, name + ".json")
        if os.path.exists(path):
            j = json.load(open(path))
            if j.get("capped"):
                print(f"chain {spec}: already censored at d={d}; stop", flush=True); break
            continue
        cmd = f"ulimit -v {MEM_KB}; RAYON_NUM_THREADS=1 {BIN} f4sub {mini} {kind} {d} 0 mont {m} {form} {N} {20261108 + 1000 * m + d} {CAP}"
        l0, t0 = load1(), time.time()
        r = subprocess.run(["bash", "-c", cmd], capture_output=True, text=True, env=env)
        t1, l1 = time.time(), load1()
        try:
            j = json.loads(r.stdout.strip().splitlines()[-1])
        except Exception:
            j = {"miniature": mini, "kind": kind, "form": form, "m": m, "d_or_t": d, "capped": True,
                 "failure": "exit %d" % r.returncode, "stderr": r.stderr[-400:]}
        j["ext"] = {"study": "curve25519_wave6_runs_20261115/" + os.path.basename(lead.rstrip("/")), "cap_s": CAP, "mem_kb": MEM_KB,
                    "load_before": l0, "load_after": l1, "wall_s": round(t1 - t0, 1),
                    "started_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(t0)), "bin_sha256": sha,
                    "exit": r.returncode, "cmd": cmd, "env_extra": [p for p in parts[5:]]}
        tmp = path + ".tmp"; json.dump(j, open(tmp, "w"), indent=1); os.replace(tmp, path)
        sd = [t.get("solving_degree_max") for t in j.get("per_target", [])]
        print(time.strftime("%H:%M:%SZ", time.gmtime()), name, "capped" if j.get("capped") else f"T {j.get('mean_field_ops'):.4g} SD {sd}", f"{t1 - t0:.0f}s", flush=True)
        if j.get("capped"):
            print(f"chain {spec}: censored at d={d}; larger d not attempted", flush=True); break
print("ALL_DONE", flush=True)
