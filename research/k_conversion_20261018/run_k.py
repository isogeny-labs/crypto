#!/usr/bin/env python3
"""Run the frozen K-conversion Callgrind jobs (PROTOCOL.md); write raw.jsonl."""
import json, re, subprocess, sys, os, concurrent.futures as cf
REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))
B = f"{REPO}/target/release/examples"
OUT = "/tmp/claude-0/cg"
CL = "*ReducedBasis>::from_system_closed_impl*"
RH = "*k_rho_solve*"
D = "research/dreg_closure_driver_20261016/draws/"
S = "research/dreg_closure_scale_20261016/draws/"
P = "research/dreg_surplus_control_20260925/runs/"
jobs = []
for cell, f, draws in [("m4-11-5", D+"m4-11-5.jsonl", [8,67,107,143]),
                       ("m4-12-3", S+"m4-12-3.jsonl", [1,2,3,4]),
                       ("m4-12-5", D+"m4-12-5.jsonl", [24,26,47,52]),
                       ("m3-16-6", S+"m3-16-6.jsonl", [0,1,2,3])]:
    for d in draws:
        jobs.append(dict(kind="closure", cell=cell, file=f, draw=d, to=3, toggle=True, rep=0))
jobs.append(dict(kind="closure", cell="m3-16-6", file=S+"m3-16-6.jsonl", draw=0, to=3, toggle=True, rep=1))  # A/A
for cell, f, d in [("m4-11-5", D+"m4-11-5.jsonl", 8), ("m4-12-3", S+"m4-12-3.jsonl", 1),
                   ("m4-12-5", D+"m4-12-5.jsonl", 24), ("m3-16-6", S+"m3-16-6.jsonl", 0)]:
    jobs.append(dict(kind="closure", cell=cell, file=f, draw=d, to=3, toggle=False, rep=0))  # total
for u, d in [("u3", 56), ("u0", 10)]:
    jobs.append(dict(kind="closure", cell="m3-11-5-deg4", file=P+f"cell-11-5-6.{u}.jsonl", draw=d, to=4, toggle=True, rep=0))
for n in ("n61", "n83"):
    for s in (1, 2, 3):
        jobs.append(dict(kind="rho", cell=n, seed=s, toggle=True, rep=0))
jobs.append(dict(kind="rho", cell="n61", seed=1, toggle=True, rep=1))  # A/A

def run(j):
    tag = f"{j['kind']}-{j['cell']}-{j.get('draw', j.get('seed'))}-t{int(j['toggle'])}-r{j['rep']}"
    cmd = ["valgrind", "--tool=callgrind", "--cache-sim=no", "--branch-sim=no", "--dump-instr=no",
           f"--callgrind-out-file={OUT}/{tag}.out"]
    if j["toggle"]:
        cmd.append(f"--toggle-collect={CL if j['kind']=='closure' else RH}")
    if j["kind"] == "closure":
        cmd += [f"{B}/dreg_closure", "--max-cols", "70000", "--to", str(j["to"]), "--draw", str(j["draw"]), j["file"]]
    else:
        cmd += [f"{B}/k_conversion_rho", j["cell"], str(j["seed"])]
    r = subprocess.run(cmd, capture_output=True, text=True, cwd=REPO)
    ir = int(re.search(r"Collected : (\d+)", r.stderr).group(1))
    rec = dict(j, tag=tag, ir=ir)
    for line in r.stdout.splitlines():
        if line.startswith("{"):
            o = json.loads(line)
            if j["kind"] == "closure":
                s = o["scan"][-1]
                rec.update(columns=s["columns"], word_ops=s["closure_word_ops"], products=s["closure_products"],
                           degree=s["degree"], n_vars=o["n_vars"], closure_refuted=s["closure_refuted"])
            else:
                rec.update(o)
    return rec

if __name__ == "__main__":
    with cf.ThreadPoolExecutor(3) as ex, open(sys.argv[1], "w") as out:
        for rec in ex.map(run, jobs):
            out.write(json.dumps(rec) + "\n"); out.flush()
