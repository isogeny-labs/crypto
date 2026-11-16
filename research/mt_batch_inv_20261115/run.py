#!/usr/bin/env python3
"""Batched-inversion round of PROTOCOL.md on the pod: python3 run.py main|smoke  (cwd = the payload directory: bin/,
instances/).  -> results/<phase>/runs.jsonl, host.json, DONE.
One process at a time, pinned to one CPU ($PIN or the middle allowed CPU); every key's repeats interleaved in a
fixed-seed shuffled order (as ../mt_rho_rematch_20261115/run.py)."""
import json, subprocess, sys, os, hashlib, itertools, random, time, platform
HERE = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(HERE, "bin/mt_fieldops"); INST = os.path.join(HERE, "instances")
_ALLOWED = sorted(os.sched_getaffinity(0)); PIN = os.environ.get("PIN", str(_ALLOWED[len(_ALLOWED) // 2]))
KS_K = [0.05, 0.1, 0.25, 0.5]; DPU_K = [0.0, 0.25]; DPU_OFF = [-1, 0, 1]
TS = [1024, 4096, 16384]
SIZES = [(32, range(4, 8)), (36, range(4))]  # W32 from ../mt_fieldops_v5_20261112, W36 from ../mt_ic_scale_20261115
def ic(base, inv):
    return dict(base, key=f"ic:{inv}", arm="ic5b", param=0.85, extra=["4", "1.2", "flat", "32", inv])
def rho(base):
    for kp in KS_K: yield dict(base, key=f"rho2xksp:{kp}", arm="rho2xksp", param=kp, extra=[])
    for kp in DPU_K:
        for off in DPU_OFF:
            yield dict(base, key=f"rhodpup:{kp}:{off}", arm="rhodpup", param=kp, extra=[str(off)])
            yield dict(base, key=f"rhodpb:{kp}:{off}", arm="rhodpb", param=kp, extra=[str(off), "32"])
def keys(phase):
    if phase == "smoke":
        for pb in (32, 36):
            base = dict(pb=pb, idx=(4 if pb == 32 else 0), T=1024, seed=43)
            yield ic(base, "plain"); yield ic(base, "bi")
            yield dict(base, key="rhodpb:0.0:0", arm="rhodpb", param=0.0, extra=["0", "32"])
            yield dict(base, key="rhodpb:0.25:0", arm="rhodpb", param=0.25, extra=["0", "32"])
        return
    for (pb, idxs), T in itertools.product(SIZES, TS):
        for idx in idxs:
            base = dict(pb=pb, idx=idx, T=T, seed=43)
            yield ic(base, "plain"); yield ic(base, "bi")
            yield from rho(base)
def argv(k):
    return ["taskset", "-c", PIN, BIN, k["arm"], f"{INST}/W{k['pb']}-{k['idx']}.json", str(k["T"]), str(k["seed"]),
            str(k["param"])] + k["extra"]
def execute(cmd):
    t0 = time.perf_counter(); r = subprocess.run(cmd, capture_output=True, text=True, timeout=7200)
    wall = time.perf_counter() - t0
    if r.returncode != 0: return dict(error=r.stderr[-400:], wall_s=wall)
    o = json.loads(r.stdout.strip().splitlines()[-1]); o["wall_s"] = wall; return o
if __name__ == "__main__":
    phase = sys.argv[1]; repeats = int(os.environ.get("REPEATS", "1" if phase == "smoke" else "3"))
    out = os.path.join(HERE, "results", phase); os.makedirs(out, exist_ok=True)
    sha = hashlib.sha256(open(BIN, "rb").read()).hexdigest()
    host = dict(cpu=next((l.split(":", 1)[1].strip() for l in open("/proc/cpuinfo") if l.startswith("model name")), ""),
                nproc=os.cpu_count(), allowed_cpus=_ALLOWED, kernel=platform.release(), pin=PIN, bin_sha256=sha,
                started=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()))
    json.dump(host, open(os.path.join(out, "host.json"), "w"), indent=1)
    ks = list(keys(phase)); rnd = random.Random(20261118); order = []
    for rep in range(repeats):
        o = list(range(len(ks))); rnd.shuffle(o); order += [(rep, i) for i in o]
    n_err = 0
    with open(os.path.join(out, "runs.jsonl"), "w") as fh:
        for seq, (rep, i) in enumerate(order):
            k = ks[i]; o = execute(argv(k))
            o.update({kk: vv for kk, vv in k.items() if kk != "extra"}, rep=rep, seq=seq, keyidx=i, loadavg=os.getloadavg()[0], bin_sha256=sha)
            fh.write(json.dumps(o) + "\n"); fh.flush()
            n_err += "error" in o or o.get("ok") != o.get("targets")
    open(os.path.join(out, "DONE"), "w").write(f"{len(order)} executions; {n_err} errors or unverified\n")
    print(phase, len(order), "executions;", n_err, "errors or unverified")
