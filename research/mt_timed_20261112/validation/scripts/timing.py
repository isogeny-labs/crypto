#!/usr/bin/env python3
"""VALIDATOR timing reproduction for research/mt_timed_20261112 (not the producer's run.py; same instrument and pinning).

Executions (all pinned with taskset -c $PIN, default 2, sequential, one process at a time):
  part A  every one of the 36 configurations x the producer's per-configuration fastest IC key, fastest rho key and
          fastest bsgs2 key (selected on the producer's medians, scored on the validator's new timings: no selection
          on the validator's data), REPEATS repeats;
  part B  one nonce-sampled configuration per cell (9) x all 17 run keys x REPEATS repeats (an independent best-of);
  part C  (optional, PRISTINE=<bin>) the validator's own source build for part A's IC and rho keys, 1 repeat, to check
          that the producer's binary times like a build of the frozen source.
Order: rounds of repeats; inside a round a nonce-seeded shuffle of all executions (interleaved A/A).  opcost (param
0.25, curve 0 of each size) runs before and after.  Each execution records loadavg and /proc/stat deltas (steal on
the pinned CPU, busy jiffies on the other CPUs) as interference indicators.
usage: timing.py <producer bin> <frozen root> <recompute.json> <out.jsonl> <nonce>"""
import json, sys, os, subprocess, time, random, hashlib, collections

BIN, ROOT, RC, OUT, NONCE = sys.argv[1:6]
PIN = os.environ.get("PIN", "2"); REPEATS = int(os.environ.get("REPEATS", "3")); PRISTINE = os.environ.get("PRISTINE")
INST = os.path.join(ROOT, "research/mt_fieldops_v5_20261112/instances")
IC = ("ic5", "ic5ks", "ic5w", "ic5rw")


def rng_for(tag):
    return random.Random(hashlib.sha256((NONCE + ":" + tag).encode()).digest())


def all_keys():
    ks = [(a, 1.0, None) for a in IC] + [("rho2xks", kp, None) for kp in (0.05, 0.1, 0.25, 0.5)]
    ks += [("rhodpu", kp, off) for kp in (0.0, 0.25) for off in (-1, 0, 1)] + [("bsgs2", mf, None) for mf in (0.7, 1.0, 1.4)]
    return ks


def argv(bin_, pb, idx, T, seed, k):
    a = ["taskset", "-c", PIN, bin_, k[0], os.path.join(INST, f"W{pb}-{idx}.json"), str(T), str(seed), str(k[1])]
    if k[0] in IC: a.append("4")
    if k[0] == "rhodpu": a.append(str(k[2]))
    return a


def stat():
    s = {}
    for l in open("/proc/stat"):
        if l.startswith("cpu") and l[3].isdigit():
            f = l.split(); s[f[0]] = [int(v) for v in f[1:]]
    return s


def delta(a, b):
    pin = "cpu" + PIN
    steal = b[pin][7] - a[pin][7]
    other_busy = sum((sum(b[c][:8]) - b[c][3] - b[c][4]) - (sum(a[c][:8]) - a[c][3] - a[c][4]) for c in a if c != pin)
    return steal, other_busy


rc = json.load(open(RC))
cells = sorted({(e["bits"], e["T"]) for e in rc["per_configuration"]})
rb = rng_for("partB")
sampled = {c: rb.randrange(4) for c in cells}
ex = []
for e in rc["per_configuration"]:
    c = (e["bits"], e["T"]); cfg = (e["bits"], e["T"], e["idx"], e["seed"])
    if sampled[c] == e["idx"]:
        for k in all_keys(): ex.append(dict(part="B", cfg=cfg, key=k, bin="producer"))
    else:
        for side in ("ic_key", "rho_key", "bsgs_key"):
            k = tuple(e["ms_arm"][side]); ex.append(dict(part="A", cfg=cfg, key=k, bin="producer"))
    if PRISTINE:
        for side in ("ic_key", "rho_key"):
            ex.append(dict(part="C", cfg=cfg, key=tuple(e["ms_arm"][side]), bin="pristine"))
ro = rng_for("order"); order = []
for rep in range(REPEATS):
    o = [i for i in range(len(ex)) if not (ex[i]["part"] == "C" and rep > 0)]
    ro.shuffle(o); order += [(rep, i) for i in o]
bins = dict(producer=BIN, pristine=PRISTINE)
shas = {k: hashlib.sha256(open(v, "rb").read()).hexdigest() for k, v in bins.items() if v}
fo = open(OUT, "w")
fo.write(json.dumps(dict(header=True, nonce=NONCE, sampled_idx={f"{c[0]}/{c[1]}": v for c, v in sampled.items()}, n_exec=len(order),
                         bin_sha256=shas, pin=PIN, repeats=REPEATS, started=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()))) + "\n")


def run(cmd):
    a = stat(); t0 = time.perf_counter(); p = subprocess.run(cmd, capture_output=True, text=True, timeout=3600)
    w = time.perf_counter() - t0; b = stat(); st, ob = delta(a, b)
    o = json.loads(p.stdout.strip().splitlines()[-1]) if p.returncode == 0 else dict(error=p.stderr[-300:])
    o.update(wall_s=w, steal_jiffies_pin=st, other_cpus_busy_jiffies=ob, loadavg=os.getloadavg()[0])
    return o


def opcost(tag):
    for pb in (24, 28, 32):
        o = run(["taskset", "-c", PIN, BIN, "opcost", os.path.join(INST, f"W{pb}-0.json"), "1", "1", "0.25"])
        o.update(kind="opcost", pb=pb, when=tag); fo.write(json.dumps(o) + "\n"); fo.flush()
    if tag == "before":  # scaling check: param 1.0 runs every loop 4x longer; per-op ns must not change (no dead code)
        o = run(["taskset", "-c", PIN, BIN, "opcost", os.path.join(INST, "W32-0.json"), "1", "1", "1.0"])
        o.update(kind="opcost", pb=32, when="scale1.0"); fo.write(json.dumps(o) + "\n"); fo.flush()


s0 = stat(); opcost("before")
for seq, (rep, i) in enumerate(order):
    x = ex[i]; pb, T, idx, seed = x["cfg"]
    o = run(argv(bins[x["bin"]], pb, idx, T, seed, x["key"]))
    o.update(kind="run", part=x["part"], binary=x["bin"], pb=pb, T=T, idx=idx, seed=seed, key=list(x["key"]), rep=rep, seq=seq)
    fo.write(json.dumps(o) + "\n"); fo.flush()
opcost("after"); s1 = stat()
tot = {c: [b - a for a, b in zip(s0[c], s1[c])] for c in s0}
fo.write(json.dumps(dict(footer=True, ended=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), proc_stat_delta=tot)) + "\n")
print("done", len(order))
