#!/usr/bin/env python3
"""Timed head-to-head on a RunPod CPU pod, two builds (see PROTOCOL.md).  usage: run.py holdout|smoke
-> results/<phase>/{runs,opcost}.jsonl, host.json.  Runs inside the extracted payload directory (bin/, instances/).
Sequential, one process at a time, pinned with taskset to CPU $PIN (default 2). Every run key (build x configuration
x arm x parameter) runs REPEATS times; within each round all keys run in a fixed-seed shuffled order, so the two
builds and the repeats of a key are interleaved over the whole session."""
import json, subprocess, sys, os, hashlib, itertools, random, time, platform
HERE = os.path.dirname(os.path.abspath(__file__))
BINS = {"std": os.path.join(HERE, "bin/mt_fieldops_std"), "fast": os.path.join(HERE, "bin/mt_fieldops_fast")}
INST = os.path.join(HERE, "instances")
PIN = os.environ.get("PIN", "2")
IC_ARMS = ["ic5", "ic5ks", "ic5w", "ic5rw"]; KS_K = [0.05, 0.1, 0.25, 0.5]
DPU_K = [0.0, 0.25]; DPU_OFF = [-1, 0, 1]; BSGS_MF = [0.7, 1.0, 1.4]
def keys(phase):
    if phase == "smoke":
        cells = [(24, 1024)]; idxs = [0]
    else:
        cells = [(pb, T) for pb in (24, 28, 32) for T in (1024, 4096, 16384)]; idxs = range(0, 4)
    for build in ("std", "fast"):
        for (pb, T), idx in itertools.product(cells, idxs):
            base = dict(build=build, pb=pb, idx=idx, T=T, seed=43)
            for a in IC_ARMS: yield dict(base, arm=a, param=1.0, extra=["4"])
            for kp in KS_K: yield dict(base, arm="rho2xks", param=kp, extra=[])
            for kp in DPU_K:
                for off in DPU_OFF: yield dict(base, arm="rhodpu", param=kp, extra=[str(off)], doff=off)
            for mf in BSGS_MF: yield dict(base, arm="bsgs2", param=mf, extra=[])
def argv(k):
    return ["taskset", "-c", PIN, BINS[k["build"]], k["arm"], f"{INST}/W{k['pb']}-{k['idx']}.json", str(k["T"]),
            str(k["seed"]), str(k["param"])] + k["extra"]
def execute(cmd):
    t0 = time.perf_counter(); r = subprocess.run(cmd, capture_output=True, text=True, timeout=3600)
    wall = time.perf_counter() - t0
    if r.returncode != 0: return dict(error=r.stderr[-400:], wall_s=wall)
    o = json.loads(r.stdout.strip().splitlines()[-1]); o["wall_s"] = wall; return o
def steal():
    f = open("/proc/stat").readline().split(); return int(f[8]) if len(f) > 8 else None
if __name__ == "__main__":
    phase = sys.argv[1]; repeats = int(os.environ.get("REPEATS", "3"))
    out = os.path.join(HERE, "results", phase); os.makedirs(out, exist_ok=True)
    shas = {b: hashlib.sha256(open(p, "rb").read()).hexdigest() for b, p in BINS.items()}
    host = dict(cpu=next((l.split(":", 1)[1].strip() for l in open("/proc/cpuinfo") if l.startswith("model name")), ""),
                nproc=os.cpu_count(), kernel=platform.release(), pin=PIN, bin_sha256=shas,
                cpuset=open("/sys/fs/cgroup/cpuset.cpus.effective").read().strip() if os.path.exists("/sys/fs/cgroup/cpuset.cpus.effective") else None,
                cpu_max=open("/sys/fs/cgroup/cpu.max").read().strip() if os.path.exists("/sys/fs/cgroup/cpu.max") else None,
                started=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()))
    json.dump(host, open(os.path.join(out, "host.json"), "w"), indent=1)
    ks = list(keys(phase)); rnd = random.Random(20261112)
    order = []
    for rep in range(repeats):
        o = list(range(len(ks))); rnd.shuffle(o); order += [(rep, i) for i in o]
    with open(os.path.join(out, "opcost.jsonl"), "w") as fo:
        def opcost(tag):
            for pb in sorted({k["pb"] for k in ks}):
                o = execute(["taskset", "-c", PIN, BINS["std"], "opcost", f"{INST}/W{pb}-0.json", "1", "1", "0.25"])
                o.update(pb=pb, when=tag, host=host); fo.write(json.dumps(o) + "\n"); fo.flush()
        opcost("before")
        n_err = 0
        with open(os.path.join(out, "runs.jsonl"), "w") as fh:
            for seq, (rep, i) in enumerate(order):
                k = ks[i]; s0 = steal(); o = execute(argv(k)); s1 = steal()
                o.update({kk: vv for kk, vv in k.items() if kk != "extra"}, rep=rep, seq=seq, key=i,
                         loadavg=os.getloadavg()[0], steal_ticks=(s1 - s0) if s0 is not None else None,
                         bin_sha256=shas[k["build"]])
                fh.write(json.dumps(o) + "\n"); fh.flush()
                n_err += "error" in o or o.get("ok") != o.get("targets")
        opcost("after")
    open(os.path.join(out, "DONE"), "w").write(f"{len(order)} executions; {n_err} errors or unverified\n")
    print(phase, len(order), "executions;", n_err, "errors or unverified")
