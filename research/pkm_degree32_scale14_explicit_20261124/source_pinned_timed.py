#!/usr/bin/env python3
"""Run one selected explicit-curve mirror at a time under quiet host load."""
import argparse
import datetime
import hashlib
import json
import os
import resource
import statistics
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "pkm_adaptive_sweep_20261123"))
import f4_height4 as f4
import run as old

EXPECTED = "c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d"
ORDER = ("baseline:1", "candidate:1", "candidate:2", "baseline:2")


def quiet_receipt(exclude=()):
    lines = subprocess.run(["ps", "-axo", "pid,pcpu,comm,args"], text=True,
                           capture_output=True, check=True).stdout.splitlines()[1:]
    heavy = []
    for line in lines:
        parts = line.strip().split(None, 3)
        if len(parts) != 4:
            continue
        pid, cpu, comm, args = parts
        if int(pid) == os.getpid() or int(pid) in exclude:
            continue
        name = Path(comm).name
        if name in ("cargo", "rustc", "ic") or ("pkm_degree32_fullcost" in args and
                                                   "source_pinned_timed.py" not in args):
            heavy.append(line)
    return {"time": datetime.datetime.now().astimezone().isoformat(),
            "loadavg": list(os.getloadavg()), "logical_cpus": os.cpu_count(),
            "competing_processes": heavy}


def guarded_run(command, cap):
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    start = time.perf_counter()
    proc = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    contamination = None
    timeout = False
    while True:
        try:
            stdout, stderr = proc.communicate(timeout=2)
            break
        except subprocess.TimeoutExpired:
            snapshot = quiet_receipt(exclude=(proc.pid,))
            if snapshot["competing_processes"]:
                contamination = snapshot
                proc.terminate()
                stdout, stderr = proc.communicate()
                break
            if time.perf_counter() - start >= cap:
                timeout = True
                proc.terminate()
                stdout, stderr = proc.communicate()
                break
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    try:
        result = json.loads(stdout)
    except json.JSONDecodeError:
        result = None
    return {"command": command, "exit_code": 125 if contamination else (124 if timeout else proc.returncode),
            "process_exit_code": proc.returncode, "wall_s": time.perf_counter()-start,
            "child_cpu_s": (after.ru_utime+after.ru_stime)-(before.ru_utime+before.ru_stime),
            "result": result, "stdout_tail": stdout[-2000:] if result is None else None,
            "stderr_tail": stderr[-2000:], "load_contamination": contamination,
            "outer_timeout": timeout}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--curve-index", type=int, required=True)
    parser.add_argument("--store", type=Path, default=HERE / "source_pinned_explicit14_20261125.jsonl")
    args = parser.parse_args()
    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED:
        raise ValueError("binary hash mismatch")
    rec = old.load(args.store)
    if rec["header"]["binary_sha256"] != digest:
        raise ValueError("store binary mismatch")
    selection = rec["amended_selection"]
    if not selection["complete"]:
        raise ValueError("three-curve selection incomplete")
    chosen = {x["curve_index"]: x for x in selection["first_three"]}
    if args.curve_index not in chosen:
        raise ValueError("curve is not selected")
    i = args.curve_index
    if f"verdict:{i}" in rec:
        print(json.dumps(rec[f"verdict:{i}"], sort_keys=True))
        return
    curve = next((c for c in old.CURVES if c["index"] == i), None)
    if curve is None:
        curve = rec[f"size-check:{i}"]["curve"]
    receipt = quiet_receipt()
    receipt["id"] = f"load-check:{i}:{sum(k.startswith('load-check:'+str(i)+':') for k in rec)+1}"
    f4.append(args.store, receipt)
    if receipt["competing_processes"]:
        raise RuntimeError("competing build or benchmark process is active")
    for arm_rep in ORDER:
        key = f"timed:{i}:{arm_rep}"
        if key in rec:
            continue
        pre_call = quiet_receipt()
        prefix = f"pre-call:{i}:{arm_rep}:"
        pre_call["id"] = prefix + str(1 + sum(k.startswith(prefix) for k in rec))
        f4.append(args.store, pre_call)
        rec[pre_call["id"]] = pre_call
        if pre_call["competing_processes"]:
            raise RuntimeError("competing process appeared before timed call")
        arm = arm_rep.split(":")[0]
        fb = old.BASE if arm == "baseline" else old.CAND
        oracle = old.BOR if arm == "baseline" else old.COR
        item = {"id": key, "curve_index": i, "arm": arm,
                **guarded_run(old.cmd(args.bin, curve, fb, oracle, False), 900)}
        f4.append(args.store, item)
        rec[key] = item
        if item["load_contamination"]:
            raise RuntimeError("timed call censored by competing load; use a fresh store for qualifying replay")
    calls = [rec[f"timed:{i}:{name}"] for name in ORDER]
    rowsets = [(x.get("result") or {}).get("rows") or [] for x in calls]
    complete = (all(x["exit_code"] == 0 and x["result"].get("instance") == chosen[i]["instance"]
                    and old.size(x) == chosen[i]["base_size"] for x in calls)
                and all(len(rows) == old.REPEATS and all(r.get("verified") and r.get("r") == curve["r"]
                                                    for r in rows) for rows in rowsets)
                and all(len({rows[j].get("recovered") for rows in rowsets}) == 1
                        for j in range(old.REPEATS)))
    item = {"id": f"verdict:{i}", "curve_index": i, "complete": complete,
            "runs": {name: old.summarize(x) for name, x in zip(ORDER, calls)}}
    if complete:
        ss = [item["runs"][name] for name in ORDER]
        item["pairs"] = [{"gae_ratio": ss[c]["gae"]/ss[b]["gae"],
                          "wall_ratio": ss[c]["wall_s"]/ss[b]["wall_s"],
                          "cpu_ratio": ss[c]["cpu_s"]/ss[b]["cpu_s"]}
                         for b, c in ((0, 1), (3, 2))]
        item["combined_gae_ratio"] = (ss[1]["gae"]+ss[2]["gae"])/(ss[0]["gae"]+ss[3]["gae"])
        item["component_gate_pass"] = (item["combined_gae_ratio"] < .9 and
                                       all(p["gae_ratio"] < .9 and p["wall_ratio"] < 1 and
                                           p["cpu_ratio"] < 1 for p in item["pairs"]))
    else:
        item["component_gate_pass"] = False
    f4.append(args.store, item)
    print(json.dumps(item, sort_keys=True))


if __name__ == "__main__":
    main()
