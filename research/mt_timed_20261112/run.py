#!/usr/bin/env python3
"""Timed head-to-head (see PROTOCOL.md).  usage: run.py holdout|smoke  -> results/<phase>/{runs,opcost}.jsonl
Sequential, one process at a time, pinned with taskset to CPU $PIN (default 2). Every run key (configuration x arm x
parameter) is executed REPEATS times; the order of all executions is a seeded shuffle within each round, rounds in
sequence, so repeats of one key are spread over the whole session (interleaved A/A)."""
import json, subprocess, sys, os, hashlib, itertools, random, time, platform
HERE = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(HERE, "../../target/release/examples/mt_fieldops")
INST = os.path.join(HERE, "../mt_fieldops_v5_20261112/instances")
PIN = os.environ.get("PIN", "2")
IC_ARMS = ["ic5", "ic5ks", "ic5w", "ic5rw"]; KS_K = [0.05, 0.1, 0.25, 0.5]
DPU_K = [0.0, 0.25]; DPU_OFF = [-1, 0, 1]; BSGS_MF = [0.7, 1.0, 1.4]
def keys(phase):
    if phase == "smoke":
        cells = [(24, 1024)]; idxs = [0]; seeds = [43]
    else:
        cells = [(pb, T) for pb in (24, 28, 32) for T in (1024, 4096, 16384)]; idxs = range(0, 4); seeds = [43]
    for (pb, T), idx, seed in itertools.product(cells, idxs, seeds):
        base = dict(pb=pb, idx=idx, T=T, seed=seed)
        for a in IC_ARMS: yield dict(base, arm=a, param=1.0, extra=["4"])
        for kp in KS_K: yield dict(base, arm="rho2xks", param=kp, extra=[])
        for kp in DPU_K:
            for off in DPU_OFF: yield dict(base, arm="rhodpu", param=kp, extra=[str(off)], doff=off)
        for mf in BSGS_MF: yield dict(base, arm="bsgs2", param=mf, extra=[])
def argv(k):
    return ["taskset", "-c", PIN, BIN, k["arm"], f"{INST}/W{k['pb']}-{k['idx']}.json", str(k["T"]), str(k["seed"]),
            str(k["param"])] + k["extra"]
def execute(cmd):
    t0 = time.perf_counter(); r = subprocess.run(cmd, capture_output=True, text=True, timeout=3600)
    wall = time.perf_counter() - t0
    if r.returncode != 0: return dict(error=r.stderr[-400:], wall_s=wall)
    o = json.loads(r.stdout.strip().splitlines()[-1]); o["wall_s"] = wall; return o
if __name__ == "__main__":
    phase = sys.argv[1]; repeats = int(os.environ.get("REPEATS", "3"))
    out = os.path.join(HERE, "results", phase); os.makedirs(out, exist_ok=True)
    binsha = hashlib.sha256(open(BIN, "rb").read()).hexdigest()
    commit = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True, cwd=HERE).stdout.strip()
    host = dict(cpu=next((l.split(":", 1)[1].strip() for l in open("/proc/cpuinfo") if l.startswith("model name")), ""),
                nproc=os.cpu_count(), kernel=platform.release(), pin=PIN)
    ks = list(keys(phase)); rnd = random.Random(20261112)
    order = []
    for rep in range(repeats):
        o = list(range(len(ks))); rnd.shuffle(o); order += [(rep, i) for i in o]
    # opcost before and after the head-to-head, per size, on curve 0
    with open(os.path.join(out, "opcost.jsonl"), "w") as fo:
        def opcost(tag):
            for pb in sorted({k["pb"] for k in ks}):
                o = execute(["taskset", "-c", PIN, BIN, "opcost", f"{INST}/W{pb}-0.json", "1", "1", "0.25"])
                o.update(pb=pb, when=tag, bin_sha256=binsha, commit=commit, host=host); fo.write(json.dumps(o) + "\n"); fo.flush()
        opcost("before")
        n_err = 0
        with open(os.path.join(out, "runs.jsonl"), "w") as fh:
            for seq, (rep, i) in enumerate(order):
                k = ks[i]; o = execute(argv(k))
                o.update({kk: vv for kk, vv in k.items() if kk != "extra"}, rep=rep, seq=seq, key=i,
                         loadavg=os.getloadavg()[0], bin_sha256=binsha, commit=commit)
                fh.write(json.dumps(o) + "\n"); fh.flush()
                n_err += "error" in o or o.get("ok") != o.get("targets")
        opcost("after")
    print(phase, len(order), "executions;", n_err, "errors or unverified")
