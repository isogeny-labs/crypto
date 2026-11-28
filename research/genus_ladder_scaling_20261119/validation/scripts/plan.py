#!/usr/bin/env python3
"""Draw the replay sample from the nonce (validation/out/nonce.txt) and write plan.json + jobs.txt.

One job = one (curve, target k) pair of one block, replayed with both solver seeds (1, 2) and
--threads 1.  Flags are those of run.sh for that block.  Genus-4 pairs also get a log-dump job
(val_po harness, genus 4 only); genus-5 pairs get an export job (genus_ladder.blpt with
GENUS_LADDER_EXPORT_CORE) whose exported coordinate is turned into the recovered log.
"""
import hashlib, json, os, sys

V = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
S = os.path.dirname(V)
OUT = os.path.join(V, "out")
nonce = open(os.path.join(OUT, "nonce.txt")).read().split()[0]

BLOCKS = {
    # block: (p, g, cc, [sge...], pairs to draw, log job)
    "b4_p54293_cc2.0": (54293, 4, "2.0", ["4,1.5"], 1, None),
    "b4_p54293_cc1.4": (54293, 4, "1.4", ["4,1.5"], 2, "valpo"),
    "b5_p2357_x5": (2357, 5, "2.0", ["4,1.5"], 1, "export"),
    "b5_p2357_x5star": (2357, 5, "2.8", ["8,2"], 2, "export"),
    "b5_p4099_x5": (4099, 5, "2.0", ["4,1.5"], 1, "export"),
    "b5_p4099_x5star": (4099, 5, "2.8", ["8,2"], 2, "export"),
    "tune5_cc2.8": (2357, 5, "2.8", ["4,1.5", "8,2"], 1, None),
    "tune5_cc1.0": (2357, 5, "1.0", ["4,1.5", "8,2"], 1, None),
}


def load(name):
    rows = [json.loads(l) for l in open(os.path.join(S, name + ".jsonl")) if l.strip()]
    inst = {r["curve_seed"]: r for r in rows if r["kind"] == "instance"}
    runs = [r for r in rows if r["kind"] == "run"]
    return inst, runs


plan, jobs = {"nonce": nonce, "blocks": {}}, []
for b, (p, g, cc, sges, npairs, logjob) in BLOCKS.items():
    inst, runs = load(b)
    pairs = sorted({(r["curve_seed"], (r["target_seed"] - r["curve_seed"]) // 1_000_003) for r in runs})
    for r in runs:  # target_seed = curve_seed + k * 1_000_003 (src/bin/genus_ladder.rs)
        assert (r["target_seed"] - r["curve_seed"]) % 1_000_003 == 0
    h = int(hashlib.sha256((nonce + "|" + b).encode()).hexdigest(), 16)
    chosen = []
    while len(chosen) < npairs:
        c = pairs[h % len(pairs)]
        if c not in chosen:
            chosen.append(c)
        h //= len(pairs) + 1
    plan["blocks"][b] = {"pairs_available": len(pairs), "chosen": chosen, "log_job": logjob}
    for cs, k in chosen:
        tag = f"{b}_{cs}_k{k}"
        args = ["run", "--primes", str(p), "--genera", str(g), "--max-h", "1", "--curves", "1",
                "--curve-seeds", f"{p}={cs}", "--targets", "1", "--target-from", str(k), "--seeds", "2",
                "--arms", "x", "--cc", cc]
        for s in sges:
            args += ["--sge", s]
        args += ["--threads", "1"]
        jobs.append({"tag": tag, "kind": "po", "args": args})
        ts = cs + k * 1_000_003
        if logjob == "valpo":
            I = inst[cs]
            w, fm = sges[0].split(",")
            jobs.append({"tag": tag + "_valpo", "kind": "valpo",
                         "stdin": f"inst {p} {cs} {ts} {I['order']} {I['h']} {I['r']} x {cc} {w} {fm} 1,2"})
        elif logjob == "export":
            jobs.append({"tag": tag + "_blpt", "kind": "blpt", "args": args})
json.dump(plan, open(os.path.join(OUT, "plan.json"), "w"), indent=1)
with open(os.path.join(OUT, "jobs.txt"), "w") as fh:
    for j in jobs:
        fh.write(json.dumps(j) + "\n")
print(json.dumps(plan, indent=1))
print(len(jobs), "jobs")
