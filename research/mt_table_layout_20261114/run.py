#!/usr/bin/env python3
"""Round of PROTOCOL.md on the pod: python3 run.py holdout|smoke  (cwd = the payload directory: bin/, instances/).
-> results/<phase>/runs.jsonl, host.json, DONE.  One process at a time, pinned to one CPU ($PIN or the middle allowed CPU);
all keys' repeats interleaved in a fixed-seed shuffled order (as ../mt_timed_runpod_20261112/run.py)."""
import json, subprocess, sys, os, hashlib, itertools, random, time, platform
HERE = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(HERE, "bin/mt_fieldops"); INST = os.path.join(HERE, "instances")
_ALLOWED = sorted(os.sched_getaffinity(0)); PIN = os.environ.get("PIN", str(_ALLOWED[len(_ALLOWED) // 2]))
IC5B = [("A", "std", "1"), ("Bs", "std", "32"), ("Bf1", "flat", "1"), ("Bf8", "flat", "8"), ("B", "flat", "32")]
KS_K = [0.05, 0.1, 0.25, 0.5]; DPU_K = [0.0, 0.25]; DPU_OFF = [-1, 0, 1]
def keys(phase):
    cells = [(24, 1024)] if phase == "smoke" else [(pb, T) for pb in (24, 28, 32) for T in (1024, 4096, 16384)]
    idxs = [0] if phase == "smoke" else range(4)
    for (pb, T), idx in itertools.product(cells, idxs):
        base = dict(pb=pb, idx=idx, T=T, seed=43)
        for tag, mode, b in IC5B: yield dict(base, key=tag, arm="ic5b", param=1.0, extra=["4", "1.2", mode, b])
        for kp in KS_K: yield dict(base, key=f"rho2xks:{kp}", arm="rho2xks", param=kp, extra=[])
        for kp in DPU_K:
            for off in DPU_OFF: yield dict(base, key=f"rhodpu:{kp}:{off}", arm="rhodpu", param=kp, extra=[str(off)])
def argv(k):
    return ["taskset", "-c", PIN, BIN, k["arm"], f"{INST}/W{k['pb']}-{k['idx']}.json", str(k["T"]), str(k["seed"]),
            str(k["param"])] + k["extra"]
def execute(cmd):
    t0 = time.perf_counter(); r = subprocess.run(cmd, capture_output=True, text=True, timeout=7200)
    wall = time.perf_counter() - t0
    if r.returncode != 0: return dict(error=r.stderr[-400:], wall_s=wall)
    o = json.loads(r.stdout.strip().splitlines()[-1]); o["wall_s"] = wall; return o
if __name__ == "__main__":
    phase = sys.argv[1]; repeats = int(os.environ.get("REPEATS", "3"))
    out = os.path.join(HERE, "results", phase); os.makedirs(out, exist_ok=True)
    sha = hashlib.sha256(open(BIN, "rb").read()).hexdigest()
    host = dict(cpu=next((l.split(":", 1)[1].strip() for l in open("/proc/cpuinfo") if l.startswith("model name")), ""),
                nproc=os.cpu_count(), allowed_cpus=_ALLOWED, kernel=platform.release(), pin=PIN, bin_sha256=sha,
                started=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()))
    json.dump(host, open(os.path.join(out, "host.json"), "w"), indent=1)
    ks = list(keys(phase)); rnd = random.Random(20261114); order = []
    for rep in range(repeats):
        o = list(range(len(ks))); rnd.shuffle(o); order += [(rep, i) for i in o]
    n_err = 0
    with open(os.path.join(out, "runs.jsonl"), "w") as fh:
        for seq, (rep, i) in enumerate(order):
            k = ks[i]; o = execute(argv(k))
            o.update({kk: vv for kk, vv in k.items() if kk != "extra"}, rep=rep, seq=seq, keyidx=i,
                     loadavg=os.getloadavg()[0], bin_sha256=sha)
            fh.write(json.dumps(o) + "\n"); fh.flush()
            n_err += "error" in o or o.get("ok") != o.get("targets")
    open(os.path.join(out, "DONE"), "w").write(f"{len(order)} executions; {n_err} errors or unverified\n")
    print(phase, len(order), "executions;", n_err, "errors or unverified")
