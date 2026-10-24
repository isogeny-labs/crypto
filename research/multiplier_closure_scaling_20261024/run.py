#!/usr/bin/env python3
"""Frozen run script for PROTOCOL.md. Usage: run.py REGIME(bits|char2) FAMILY SIZE SEED OUT.jsonl
Builds one sweep file (all grid cells on one instance), runs `ic bench --sweep`, appends one JSONL row per cell."""
import resource, json, sys, os, subprocess, hashlib, tempfile, math, time
IC = os.environ.get("IC", "/Volumes/SSD990-2/cargo-target-ic-followthrough-20261024/release/ic")
ORACLE = "mitm:negation_folded=1,m=3"
GLV = {24:[50,100,200,400],28:[100,200,400,800],32:[200,400,800,1600],36:[400,800,1600,3200],40:[800,1500,3000,6000]}
SUB = {20:range(6,10),22:range(7,11),24:range(8,12),26:range(9,13),28:range(9,13)}

def cells(kind, size):
    inc, mul = [], []
    if kind == "prime":
        inc = [("glv-orbit", f"glv-orbit:size={s}") for s in GLV[size]]
        for K in (4,8,16,32,64,256,512):
            for N in (8,16,32,64,128,256):
                if 256 <= K*N <= 8192: mul.append(("cm-multiplier", f"cm-multiplier:size={K},norm={N}"))
    else:
        inc = [("binary-subspace", f"binary-subspace:dimension={d}") for d in SUB[size]]
        inc += [("binary-plain", f"binary-multiplier:size={K},mult=1") for K in (64,128,256,384,512,1024)]
        for K in (2,3,4,8,16,32,256,512):
            for M in (2,4,8,16,32,64):
                if 64 <= K*M <= 4096: mul.append(("binary-multiplier", f"binary-multiplier:size={K},mult={M}"))
    return [("incumbent",)+c for c in inc] + [("multiplier",)+c for c in mul]

def main():
    kind_flag, family, size, seed, out = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
    kind = "prime" if kind_flag == "bits" else "binary"
    cs = cells(kind, size)
    sha = hashlib.sha256(open(IC, "rb").read()).hexdigest()
    sweep = {"instance": {"regime": "prime" if kind=="prime" else "char2", "degree": size,
                          **({"family": family} if kind=="prime" else {})},
             "repeats": 4, "seed": seed,
             "configurations": [{"factor_base": c[2], "oracle": ORACLE} for c in cs]}
    d = tempfile.mkdtemp(); sp = os.path.join(d, "sweep.json"); rp = os.path.join(d, "report.json")
    json.dump(sweep, open(sp, "w"))
    cmd = [IC, "bench", "--sweep", sp, "--json", "--out", rp, "--rho-runs", "4"]
    t0 = time.time()
    meta = {"kind": kind, "family": family if kind=="prime" else "random", "size": size, "seed": seed, "ic_sha256": sha}
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, timeout=14400,
                           preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CPU, (1200, 1200)))  # CPU-time cap, not wall (load 60-100)
        status = f"rc={p.returncode}" + (" (CPU cap 1200s hit)" if p.returncode < 0 else ""); err = p.stderr[-300:]
    except subprocess.TimeoutExpired:
        status, err = "timeout_wallguard_4h", ""
    rows = []
    if os.path.exists(rp):
        rep = json.load(open(rp))
        rows = rep.get("rows", [])
        sk = rep.get("configurations_skipped")
        if sk:
            with open(out, "a") as f:
                f.write(json.dumps({**meta, "status": status, "role": "skipped", "skipped": sk}) + "\n")
    with open(out, "a") as f:
        if not rows:
            f.write(json.dumps({**meta, "status": status, "stderr_tail": err, "row": None}) + "\n")
        for r in rows:
            spec = r["spec"]
            fbp = spec["factor_base_params"]
            fbname = spec["factor_base"]
            role = next((c[0] for c in cs if c[2].split(":")[0] == fbname and
                         all(f"{k}={v}" in c[2] for k, v in fbp.items())), "?")
            label = next((c[1] for c in cs if c[2].split(":")[0] == fbname and
                          all(f"{k}={v}" in c[2] for k, v in fbp.items())), "?")
            f.write(json.dumps({**meta, "status": status, "role": role, "base": label,
                "fb": r["factor_base"]["name"], "fb_params": fbp,
                "instance": r["instance"], "r": r["r"], "log2_r": r["log2_r"],
                "S": r["s"], "total_gae": r["total_gae"],
                "setup_gae": r["decomposition"]["setup"]["gae"],
                "fb_gae": r["factor_base"]["cost"]["gae"],
                "oracle_gae": r["decomposition"]["cost"]["gae"],
                "cols": r["factor_base"]["columns"], "points": r["factor_base"]["signed_points"],
                "rows_needed": r["linear_algebra"]["rows"], "rank": r["linear_algebra"]["rank"],
                "relations": r["decomposition"]["relations_found"],
                "trials": r["decomposition"]["targets_tried"],
                "verified": r["verified"], "rho_s": r["rho_s"], "elapsed_s": round(time.time()-t0,1)}) + "\n")
    print(f"{kind} {family} {size} seed{seed}: {status} rows={len(rows)}/{len(cs)} {time.time()-t0:.0f}s")
main()
