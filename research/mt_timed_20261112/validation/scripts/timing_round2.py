#!/usr/bin/env python3
"""VALIDATOR timing reproduction, round 2 (added AFTER round 1 showed 3 of 9 cell decisions not reproduced; not
pre-registered, reported separately and pooled).  Every one of the 36 configurations x the producer's per-configuration
fastest IC key and fastest rho key (selected on the producer's medians), REPEATS (default 5) repeats, rounds in
sequence with a nonce-seeded shuffle inside each round, pinned with taskset -c $PIN (default 2), sequential.
Records loadavg and /proc/stat deltas per execution (steal on the pinned CPU, busy jiffies on the other CPUs).
usage: timing_round2.py <producer bin> <frozen root> <recompute.json> <out.jsonl> <nonce>"""
import json, sys, os, subprocess, time, random, hashlib

BIN, ROOT, RC, OUT, NONCE = sys.argv[1:6]
PIN = os.environ.get("PIN", "2"); REPEATS = int(os.environ.get("REPEATS", "5"))
INST = os.path.join(ROOT, "research/mt_fieldops_v5_20261112/instances")
IC = ("ic5", "ic5ks", "ic5w", "ic5rw")


def stat():
    s = {}
    for l in open("/proc/stat"):
        if l.startswith("cpu") and l[3].isdigit():
            f = l.split(); s[f[0]] = [int(v) for v in f[1:]]
    return s


def run(cmd):
    a = stat(); t0 = time.perf_counter(); p = subprocess.run(cmd, capture_output=True, text=True, timeout=3600)
    w = time.perf_counter() - t0; b = stat(); pin = "cpu" + PIN
    o = json.loads(p.stdout.strip().splitlines()[-1]) if p.returncode == 0 else dict(error=p.stderr[-300:])
    o.update(wall_s=w, steal_jiffies_pin=b[pin][7] - a[pin][7], loadavg=os.getloadavg()[0],
             other_cpus_busy_jiffies=sum((sum(b[c][:8]) - b[c][3] - b[c][4]) - (sum(a[c][:8]) - a[c][3] - a[c][4]) for c in a if c != pin))
    return o


rc = json.load(open(RC))
ex = []
for e in rc["per_configuration"]:
    for side in ("ic_key", "rho_key"):
        ex.append(dict(part="R2", cfg=(e["bits"], e["T"], e["idx"], e["seed"]), key=tuple(e["ms_arm"][side])))
ro = random.Random(hashlib.sha256((NONCE + ":round2").encode()).digest()); order = []
for rep in range(REPEATS):
    o = list(range(len(ex))); ro.shuffle(o); order += [(rep, i) for i in o]
fo = open(OUT, "w")
fo.write(json.dumps(dict(header=True, nonce=NONCE, n_exec=len(order), bin_sha256=hashlib.sha256(open(BIN, "rb").read()).hexdigest(), pin=PIN,
                         repeats=REPEATS, started=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()))) + "\n")
s0 = stat()
for seq, (rep, i) in enumerate(order):
    x = ex[i]; pb, T, idx, seed = x["cfg"]; k = x["key"]
    a = ["taskset", "-c", PIN, BIN, k[0], os.path.join(INST, f"W{pb}-{idx}.json"), str(T), str(seed), str(k[1])]
    if k[0] in IC: a.append("4")
    if k[0] == "rhodpu": a.append(str(k[2]))
    o = run(a)
    o.update(kind="run", part="R2", binary="producer", pb=pb, T=T, idx=idx, seed=seed, key=list(k), rep=rep, seq=seq)
    fo.write(json.dumps(o) + "\n"); fo.flush()
s1 = stat()
fo.write(json.dumps(dict(footer=True, ended=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
                         proc_stat_delta={c: [b - a for a, b in zip(s0[c], s1[c])] for c in s0})) + "\n")
print("done", len(order))
