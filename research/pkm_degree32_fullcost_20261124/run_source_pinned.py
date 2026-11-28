#!/usr/bin/env python3
"""Run the source-pinned full-rank degree-32 panel under guarded quiet load."""
import argparse
import datetime
import hashlib
import json
import math
import os
import resource
import statistics
import subprocess
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
EXPECTED = "c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d"
INSTANCE = "icv1-fp13-tm137-9a89da76"
SEED = 20261276
REPEATS = 16
SIZE = 18
BASE = "prime-abscissa:below=32"
CAND = "prime-isogeny:t=5,seed=5,aux=ordinary"
BOR = "pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain"
COR = "pkm-semaev:m=2,engine=f4,degree=8,mode=chain"
ORDER = ("baseline:1", "candidate:1", "candidate:2", "baseline:2")


def append(path, item):
    with path.open("a") as handle:
        handle.write(json.dumps(item, sort_keys=True) + "\n")
        handle.flush()
        os.fsync(handle.fileno())
    print(item["id"], item.get("exit_code"), flush=True)


def load(path):
    out = {}
    if path.exists():
        for line in path.read_text().splitlines():
            item = json.loads(line)
            if item["id"] in out:
                raise ValueError(f"duplicate {item['id']}")
            out[item["id"]] = item
    return out


def command(binary, arm, preflight):
    fb, oracle = (BASE, BOR) if arm == "baseline" else (CAND, COR)
    cmd = [str(binary), "bench", "--bits", "13", "--family", "generic",
           "--seed", str(SEED), "--repeats", str(REPEATS), "--targets", "walk",
           "--factor-base", fb, "--oracle", "subtract" if preflight else oracle,
           "--max-trials", "1" if preflight else "20000",
           "--rho-runs", "0" if preflight else str(REPEATS), "--json"]
    if not preflight:
        cmd += ["--linalg", "incremental-gauss-full-rank-checked",
                "--solver-budget-seconds", "5"]
    return cmd


def quiet(exclude=()):
    ps = subprocess.run(["ps", "-axo", "pid,pcpu,comm,args"], capture_output=True,
                        text=True, check=True).stdout.splitlines()[1:]
    heavy = []
    for line in ps:
        parts = line.strip().split(None, 3)
        if len(parts) != 4:
            continue
        pid, cpu, comm, args = parts
        if int(pid) == os.getpid() or int(pid) in exclude:
            continue
        research_cpu = (float(cpu) >= 25 and not args.startswith(("/System/", "/Applications/",
                                                                  "/usr/", "/opt/homebrew/bin/ps ")))
        if Path(comm).name in ("cargo", "rustc", "ic", "msolve") or research_cpu:
            heavy.append(line)
    return {"time": datetime.datetime.now().astimezone().isoformat(),
            "loadavg": list(os.getloadavg()), "logical_cpus": os.cpu_count(),
            "competing_processes": heavy}


def run(command, cap, guarded=False):
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    started = time.perf_counter()
    proc = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    contamination = None
    timeout = False
    while True:
        try:
            stdout, stderr = proc.communicate(timeout=2 if guarded else cap)
            break
        except subprocess.TimeoutExpired:
            if guarded:
                snapshot = quiet(exclude=(proc.pid,))
                if snapshot["competing_processes"]:
                    contamination = snapshot
            if contamination or time.perf_counter()-started >= cap:
                timeout = not bool(contamination)
                proc.terminate()
                stdout, stderr = proc.communicate()
                break
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    try:
        result = json.loads(stdout)
    except json.JSONDecodeError:
        result = None
    return {"command": command, "exit_code": 125 if contamination else (124 if timeout else proc.returncode),
            "process_exit_code": proc.returncode, "wall_s": time.perf_counter()-started,
            "child_cpu_s": (after.ru_utime+after.ru_stime)-(before.ru_utime+before.ru_stime),
            "result": result, "stdout_tail": stdout[-2000:] if result is None else None,
            "stderr_tail": stderr[-2000:], "load_contamination": contamination, "outer_timeout": timeout}


def rows(item):
    return (item.get("result") or {}).get("rows") or []


def size(item):
    got = rows(item)
    values = {r.get("factor_base", {}).get("abscissae") for r in got}
    return values.pop() if len(got) == REPEATS and len(values) == 1 else None


def valid(item):
    result = item.get("result") or {}
    got = rows(item)
    if item["exit_code"] != 0 or result.get("instance") != INSTANCE or size(item) != SIZE:
        return False
    if result.get("status") != "complete" or not result.get("all_verified"):
        return False
    if result.get("cost_qualification") is not None:
        return False
    recovered = [r.get("recovered") for r in got]
    if any(x is None for x in recovered):
        return False
    for r in got:
        la = r.get("linear_algebra", {})
        if not r.get("verified") or r.get("exhausted"):
            return False
        if r["factor_base"].get("columns") != SIZE or la.get("rank") != SIZE+1:
            return False
        if la.get("cost", {}).get("native", {}).get("full_rank_reached") != 1:
            return False
    rho = result.get("rho_reference") or {}
    if not rho.get("all_verified") or rho.get("runs") != REPEATS:
        return False
    return [r.get("planted") for r in rho.get("per_run", [])] == recovered


def totals(item):
    got = rows(item)
    rho = item["result"]["rho_reference"]
    rho_gae = REPEATS * rho["mean_s"] * math.sqrt(got[0]["r"])
    gae = sum(r["total_gae"] for r in got)
    return {"total_gae": gae, "rho_gae": rho_gae, "s_over_rho": gae/rho_gae,
            "median_s_over_rho": statistics.median(r["s_over_rho"] for r in got),
            "wall_s": item["wall_s"], "cpu_s": item["child_cpu_s"],
            "relations": sum(r["decomposition"]["relations_found"] for r in got),
            "trials": sum(r["decomposition"]["targets_tried"] for r in got),
            "f4_ops": sum(r["decomposition"]["solver"]["ops"] for r in got),
            "la_row_ops": sum(r["linear_algebra"]["work"] for r in got),
            "ranks": [r["linear_algebra"]["rank"] for r in got],
            "recovered": [r["recovered"] for r in got]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--store", type=Path, default=HERE/"source_pinned_fullrank_quiet_20261125.jsonl")
    parser.add_argument("--preflight-only", action="store_true")
    args = parser.parse_args()
    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED:
        raise ValueError("binary hash mismatch")
    records = load(args.store)
    if "header" not in records:
        item = {"id": "header", "schema": "pkm.degree32-fullrank-source-pinned/v1",
                "binary_sha256": digest, "source_commit": "a5503f4bb66ac3aa4eac3125fba01534d2574096",
                "protocol": "PROTOCOL_SOURCE_PINNED_20261125.md", "seed": SEED, "repeats": REPEATS}
        append(args.store, item)
        records["header"] = item
    if records["header"]["binary_sha256"] != digest:
        raise ValueError("store binary mismatch")
    if "verdict" in records:
        print(json.dumps(records["verdict"], sort_keys=True))
        return
    for arm in ("baseline", "candidate"):
        key = "preflight:"+arm
        if key not in records:
            item = {"id": key, "arm": arm, **run(command(args.bin, arm, True), 30)}
            append(args.store, item)
            records[key] = item
    a, b = records["preflight:baseline"], records["preflight:candidate"]
    if not (a["exit_code"] in (0, 1) and b["exit_code"] in (0, 1)
            and (a.get("result") or {}).get("instance") == INSTANCE
            and (b.get("result") or {}).get("instance") == INSTANCE
            and size(a) == size(b) == SIZE):
        item = {"id": "verdict", "complete": False, "reason": "preflight-mismatch",
                "sizes": [size(a), size(b)]}
        append(args.store, item)
        return
    if args.preflight_only:
        print(json.dumps({"preflight": "passed", "size": SIZE, "instance": INSTANCE}, sort_keys=True))
        return
    for arm_rep in ORDER:
        key = "timed:"+arm_rep
        if key in records:
            continue
        receipt = quiet()
        prefix = "pre-call:"+arm_rep+":"
        receipt["id"] = prefix+str(1+sum(key.startswith(prefix) for key in records))
        append(args.store, receipt)
        records[receipt["id"]] = receipt
        if receipt["competing_processes"]:
            raise RuntimeError("host is busy; timed call not started")
        arm = arm_rep.split(":")[0]
        item = {"id": key, "arm": arm, **run(command(args.bin, arm, False), 900, guarded=True)}
        append(args.store, item)
        records[key] = item
        if item["load_contamination"]:
            raise RuntimeError("timed call censored by competing process")
    calls = [records["timed:"+name] for name in ORDER]
    complete = all(valid(x) for x in calls)
    item = {"id": "verdict", "complete": complete,
            "reason": "complete" if complete else "failed-or-censored-invocation"}
    if complete:
        t = [totals(x) for x in calls]
        same = all(x["recovered"] == t[0]["recovered"] for x in t[1:])
        pairs = [{"gae_ratio": t[c]["total_gae"]/t[b]["total_gae"],
                  "wall_ratio": t[c]["wall_s"]/t[b]["wall_s"],
                  "cpu_ratio": t[c]["cpu_s"]/t[b]["cpu_s"],
                  "candidate_s_over_rho": t[c]["s_over_rho"]}
                 for b, c in ((0, 1), (3, 2))]
        item.update({"same_scalars": same, "runs": dict(zip(ORDER, t)), "pairs": pairs,
                     "combined_gae_ratio": (t[1]["total_gae"]+t[2]["total_gae"])/
                                           (t[0]["total_gae"]+t[3]["total_gae"]),
                     "component_gate": same and all(p["gae_ratio"] < .9 and
                                                    p["wall_ratio"] < 1 and p["cpu_ratio"] < 1
                                                    for p in pairs),
                     "cold_rho_crossover": same and all(t[c]["s_over_rho"] < 1 for c in (1, 2))})
    else:
        item.update({"component_gate": False, "cold_rho_crossover": False})
    append(args.store, item)
    print(json.dumps(item, sort_keys=True))


if __name__ == "__main__":
    main()
