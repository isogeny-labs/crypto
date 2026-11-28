#!/usr/bin/env python3
"""Single-target round of PROTOCOL.md (local container): python3 run.py main|smoke  (cwd = the payload directory:
bin/, instances/).  -> results/<phase>/runs.jsonl, host.json, DONE.
T = 1: every execution plants one target from its seed. One process at a time, pinned to one CPU ($PIN or the middle
allowed CPU); all executions in a fixed-seed shuffled order. Every execution runs with MTF_HP=1 and MTF_Z=1."""
import json, subprocess, sys, os, hashlib, itertools, random, time, platform
HERE = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(HERE, "bin/mt_fieldops"); INST = os.path.join(HERE, "instances")
_ALLOWED = sorted(os.sched_getaffinity(0)); PIN = os.environ.get("PIN", str(_ALLOWED[len(_ALLOWED) // 2]))
SIZES = [(32, range(4, 8)), (36, range(4)), (40, range(4))]
SEEDS = range(1, 33)
IC_MU = [1.5, 2.0, 2.5, 3.5]
LANES = [16, 32, 64]; OFF = [-1, 0, 1]
def keys(phase):
    sizes = [(32, [4]), (36, [0]), (40, [0])] if phase == "smoke" else SIZES
    seeds = [1] if phase == "smoke" else SEEDS
    for (pb, idxs), seed in itertools.product(sizes, seeds):
        for idx in idxs:
            base = dict(pb=pb, idx=idx, T=1, seed=seed)
            for mu in IC_MU:
                yield dict(base, key=f"ic:mu{mu}", arm="ic5b", param=mu,
                           extra=["4", "1.2", "flatf", "128", "bi", "rel,ib,blk"])
            for lanes in LANES:
                for off in OFF:
                    yield dict(base, key=f"rho:st:L{lanes}:{off}", arm="rhodpb", param=0.0, extra=[str(off), str(lanes), "st"])
            yield dict(base, key="rho:serial", arm="rhodpup", param=0.0, extra=["0"])
def argv(k):
    return ["taskset", "-c", PIN, BIN, k["arm"], f"{INST}/W{k['pb']}-{k['idx']}.json", "1", str(k["seed"]),
            str(k["param"])] + k["extra"]
def execute(cmd):
    env = dict(os.environ, MTF_HP="1", MTF_Z="1")
    t0 = time.perf_counter(); r = subprocess.run(cmd, capture_output=True, text=True, timeout=7200, env=env)
    wall = time.perf_counter() - t0
    if r.returncode != 0: return dict(error=r.stderr[-400:], wall_s=wall)
    o = json.loads(r.stdout.strip().splitlines()[-1]); o["wall_s"] = wall; return o
if __name__ == "__main__":
    phase = sys.argv[1]
    out = os.path.join(HERE, "results", phase); os.makedirs(out, exist_ok=True)
    sha = hashlib.sha256(open(BIN, "rb").read()).hexdigest()
    host = dict(cpu=next((l.split(":", 1)[1].strip() for l in open("/proc/cpuinfo") if l.startswith("model name")), ""),
                nproc=os.cpu_count(), allowed_cpus=_ALLOWED, kernel=platform.release(), pin=PIN, bin_sha256=sha,
                started=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
                thp=open("/sys/kernel/mm/transparent_hugepage/enabled").read().strip())
    json.dump(host, open(os.path.join(out, "host.json"), "w"), indent=1)
    ks = list(keys(phase)); order = list(range(len(ks))); random.Random(20261124).shuffle(order)
    n_err = 0
    with open(os.path.join(out, "runs.jsonl"), "w") as fh:
        for seq, i in enumerate(order):
            k = ks[i]; o = execute(argv(k))
            o.update({kk: vv for kk, vv in k.items() if kk != "extra"}, seq=seq, keyidx=i, loadavg=os.getloadavg()[0], bin_sha256=sha)
            fh.write(json.dumps(o) + "\n"); fh.flush()
            n_err += "error" in o or o.get("ok") != o.get("targets")
    open(os.path.join(out, "DONE"), "w").write(f"{len(order)} executions; {n_err} errors or unverified\n")
    print(phase, len(order), "executions;", n_err, "errors or unverified")
