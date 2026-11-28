#!/usr/bin/env python3
"""Interleaved A/B runner for examples/koblitz_orbit_dlp_fast.rs switches.

Every repeat runs every configuration once, in an order rotated by the repeat
index, so slow drift of the shared guest hits all arms alike.  A configuration
is a name and a dict of environment switches; the binary is the same for all.

  run_ab.py scan  --binary B --spec construct:73:1:40 --states 40000 --repeats 5
  run_ab.py index --binary B --spec construct:73:1:300 --threads 4 --repeats 3
"""
import argparse, json, os, re, statistics, subprocess, sys, tempfile, time

# Since the commit that makes lazy canonicalisation and probe prefetch the
# defaults, the control arm switches both off explicitly.
OFF = {"KIC_LAZY_CANON": "0", "KIC_PREFETCH": "0"}
SCAN_CONFIGS = [
    ("control", dict(OFF)),
    ("lazy_canon", dict(OFF, KIC_LAZY_CANON="1")),
    ("blocked_bloom", dict(OFF, KIC_BLOCKED_BLOOM="1")),
    ("prefetch", dict(OFF, KIC_PREFETCH="1")),
    ("all_three", {"KIC_LAZY_CANON": "1", "KIC_BLOCKED_BLOOM": "1", "KIC_PREFETCH": "1"}),
    ("prefetch_blocked", dict(OFF, KIC_BLOCKED_BLOOM="1", KIC_PREFETCH="1")),
    ("lazy_prefetch", {"KIC_LAZY_CANON": "1", "KIC_PREFETCH": "1"}),
    ("defaults", {}),
]
INDEX_CONFIGS = [
    ("serial", {}),
    ("batch_inv_1thread", {"KIC_INDEX_BATCH_INV": "1"}),
    ("threads_only", {"KIC_INDEX_THREADS": "{T}"}),
    ("threads_batch_inv", {"KIC_INDEX_THREADS": "{T}", "KIC_INDEX_BATCH_INV": "1"}),
]


def run(binary, spec, env_extra, workdir, timeout):
    env = dict(os.environ)
    env.update(env_extra)
    empty = os.path.join(workdir, "empty.txt")
    open(empty, "w").close()
    out = os.path.join(workdir, "out.jsonl")
    started = time.time()
    proc = subprocess.run([binary, spec, empty, "1", out], env=env,
                          capture_output=True, text=True, timeout=timeout)
    wall = time.time() - started
    return proc, wall


def summarise(values):
    return {"n": len(values), "median": statistics.median(values),
            "min": min(values), "max": max(values), "all": values}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("mode", choices=["scan", "index"])
    ap.add_argument("--binary", required=True)
    ap.add_argument("--spec", required=True)
    ap.add_argument("--states", type=int, default=40000)
    ap.add_argument("--repeats", type=int, default=5)
    ap.add_argument("--threads", type=int, default=4)
    ap.add_argument("--only", default="")
    ap.add_argument("--extra-env", default="{}", help="JSON env added to every arm")
    ap.add_argument("--json-out", default="")
    ap.add_argument("--timeout", type=int, default=3600)
    a = ap.parse_args()
    base_env = json.loads(a.extra_env)
    configs = SCAN_CONFIGS if a.mode == "scan" else INDEX_CONFIGS
    if a.only:
        keep = a.only.split(",")
        configs = [c for c in configs if c[0] in keep]
    results = {name: [] for name, _ in configs}
    walls = {name: [] for name, _ in configs}
    with tempfile.TemporaryDirectory() as workdir:
        for rep in range(a.repeats):
            order = configs[rep % len(configs):] + configs[:rep % len(configs)]
            for name, env in order:
                env = {k: v.replace("{T}", str(a.threads)) for k, v in env.items()}
                env.update(base_env)
                if a.mode == "scan":
                    env["KIC_PROF_MAX_STATES"] = str(a.states)
                else:
                    env["KIC_INDEX_TIMING"] = "1"
                    env["KIC_PROF_MAX_STATES"] = "1"
                proc, wall = run(a.binary, a.spec, env, workdir, a.timeout)
                if proc.returncode != 0:
                    sys.exit(f"{name} failed: {proc.stderr[-2000:]}")
                walls[name].append(wall)
                if a.mode == "scan":
                    m = re.search(r"PROF .*probes=(\d+) hits=(\d+) relations=(\d+) index_states=(\d+) scan_s=([\d.]+) ns_per_probe=([\d.]+)", proc.stderr)
                    if not m:
                        sys.exit(f"{name}: no PROF line: {proc.stderr[-1000:]}")
                    probes, hits, rels, idx, scan_s, nspp = m.groups()
                    results[name].append({"probes": int(probes), "hits": int(hits), "relations": int(rels),
                                          "index_states": int(idx), "scan_s": float(scan_s),
                                          "ns_per_probe": float(nspp), "process_wall_s": wall})
                else:
                    m = re.search(r"INDEX_TIMING threads=(\d+) batch_inv=(\w+) states=(\d+) states_s=([\d.]+) keys_s=([\d.]+) fill_s=([\d.]+) total_s=([\d.]+)", proc.stderr)
                    if not m:
                        sys.exit(f"{name}: no INDEX_TIMING line: {proc.stderr[-1000:]}")
                    t, b, st, s1, s2, s3, tot = m.groups()
                    results[name].append({"states": int(st), "states_s": float(s1), "keys_s": float(s2),
                                          "fill_s": float(s3), "total_s": float(tot), "process_wall_s": wall})
                print(f"rep {rep} {name}: {results[name][-1]}", flush=True)
    key = "ns_per_probe" if a.mode == "scan" else "total_s"
    summary = {name: summarise([r[key] for r in rs]) for name, rs in results.items()}
    doc = {"mode": a.mode, "spec": a.spec, "repeats": a.repeats, "states_cap": a.states,
           "threads": a.threads, "metric": key, "summary": summary, "runs": results,
           "host": {"nproc": os.cpu_count(), "uname": os.uname().release}}
    if a.json_out:
        json.dump(doc, open(a.json_out, "w"), indent=1)
    base = summary[configs[0][0]]["median"]
    print(f"\n{key} (median [min-max]); ratio = median / {configs[0][0]} median")
    for name, _ in configs:
        s = summary[name]
        print(f"{name:20s} {s['median']:10.3f} [{s['min']:.3f}-{s['max']:.3f}]  x{s['median']/base:.3f}")


main()
