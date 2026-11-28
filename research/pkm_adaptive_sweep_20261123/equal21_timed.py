#!/usr/bin/env python3
"""Resume cold equal-21 interval/isogeny mirrors one invocation at a time."""

import argparse
import datetime
import hashlib
import json
import os
import resource
import statistics
import subprocess
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent

EXPECTED = "1ae5170f9b3362f75a636909e1f5d01fb0ead0fd4886627d6012e2b9de9d8d08"
INSTANCE, ORDER, SIZE = "icv1-fp14-tm43-158b6914", 16231, 21
ARMS = ("interval:1", "candidate:1", "candidate:2", "interval:2")
FACTOR_BASE = {"interval": "prime-interval:t=5,lo=118",
               "candidate": "prime-isogeny:t=5,seed=5,aux=ordinary"}
ORACLE = {"interval": "pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=118,mode=chain",
          "candidate": "pkm-semaev:m=2,engine=f4,degree=8,mode=chain"}


def append(path, row):
    with path.open("a", encoding="utf-8") as stream:
        stream.write(json.dumps(row, sort_keys=True) + "\n")
        stream.flush()
        os.fsync(stream.fileno())


def records(path):
    rows = {}
    if path.exists():
        for line in path.read_text(encoding="utf-8").splitlines():
            row = json.loads(line)
            if row["id"] in rows:
                raise ValueError(f"duplicate record {row['id']}")
            rows[row["id"]] = row
    return rows


def command(binary, seed, arm):
    return [str(binary), "bench", "--bits", "14", "--family", "generic",
            "--seed", str(seed), "--repeats", "16", "--targets", "walk",
            "--factor-base", FACTOR_BASE[arm], "--oracle", ORACLE[arm],
            "--max-trials", "1000", "--rho-runs", "1",
            "--solver-budget-seconds", "5", "--json"]


def quiet_receipt(exclude=()):
    """Capture process identity and CPU, without unrelated command arguments."""
    lines = subprocess.run(["ps", "-axo", "pid,pcpu,comm"], text=True,
                           capture_output=True, check=True).stdout.splitlines()[1:]
    competing = []
    for line in lines:
        parts = line.strip().split(None, 2)
        if len(parts) != 3:
            continue
        pid, cpu, comm = parts
        if int(pid) == os.getpid() or int(pid) in exclude:
            continue
        name = Path(comm).name
        if name in ("cargo", "rustc", "ic", "msolve") or (float(cpu) >= 25 and name != "ps"):
            competing.append({"pid": int(pid), "cpu_percent": float(cpu),
                              "comm": name[:48]})
    return {"time": datetime.datetime.now().astimezone().isoformat(),
            "loadavg": list(os.getloadavg()), "logical_cpus": os.cpu_count(),
            "competing_processes": competing}


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
            "process_exit_code": proc.returncode, "wall_s": time.perf_counter() - start,
            "child_cpu_s": (after.ru_utime + after.ru_stime) - (before.ru_utime + before.ru_stime),
            "result": result, "stdout_tail": stdout[-2000:] if result is None else None,
            "stderr_tail": stderr[-2000:], "load_contamination": contamination,
            "outer_timeout": timeout}


def summary(call):
    rows = (call.get("result") or {}).get("rows") or []
    gae = [r.get("total_gae") for r in rows]
    rho = [r.get("s_over_rho") for r in rows]
    return {"wall_s": call["wall_s"], "child_cpu_s": call["child_cpu_s"],
            "total_gae": sum(gae) if len(gae) == 16 and all(isinstance(v, (int, float)) for v in gae) else None,
            "median_s_over_rho": statistics.median(rho) if len(rho) == 16 and all(isinstance(v, (int, float)) for v in rho) else None,
            "verified": sum(bool(r.get("verified")) for r in rows),
            "row_count": len(rows), "exit_code": call["exit_code"]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--target-seed", type=int, choices=range(20261301, 20261304), required=True)
    parser.add_argument("--through-arm", type=int, choices=range(1, 5), default=4)
    parser.add_argument("--preflight", type=Path, default=HERE / "equal21_preflight.jsonl")
    parser.add_argument("--store", type=Path, default=HERE / "equal21_timed.jsonl")
    args = parser.parse_args()
    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED:
        parser.error("binary SHA-256 differs from frozen protocol")
    pre = records(args.preflight)
    if not pre["gate"]["passed"] or pre["header"]["binary_sha256"] != digest:
        raise ValueError("equal-21 preflight is not source-pinned and passing")
    rec = records(args.store)
    if "header" not in rec:
        item = {"id": "header", "schema": "pkm.equal21-timed/v1", "binary_sha256": digest,
                "protocol": "PROTOCOL_F4_DEGREE32_EQUAL21_14BIT_20261125.md",
                "preflight_sha256": hashlib.sha256(args.preflight.read_bytes()).hexdigest(),
                "arms": list(ARMS), "seeds": [20261301, 20261302, 20261303]}
        append(args.store, item)
        rec["header"] = item
    if rec["header"]["binary_sha256"] != digest or rec["header"]["preflight_sha256"] != hashlib.sha256(args.preflight.read_bytes()).hexdigest():
        raise ValueError("timed store does not match binary or preflight")
    for arm_rep in ARMS[:args.through_arm]:
        key = f"timed:{args.target_seed}:{arm_rep}"
        if key in rec:
            if rec[key]["exit_code"] != 0:
                raise RuntimeError(f"{key} is censored; preserve this store and use a new one for qualifying replay")
            continue
        prefix = f"load-check:{args.target_seed}:{arm_rep}:"
        receipt = quiet_receipt()
        receipt["id"] = prefix + str(1 + sum(key.startswith(prefix) for key in rec))
        append(args.store, receipt)
        rec[receipt["id"]] = receipt
        if receipt["competing_processes"]:
            append(args.store, {"id": f"aborted-prelaunch:{args.target_seed}:{arm_rep}:{receipt['id'].split(':')[-1]}",
                                "load_check": receipt["id"], "timed_child_started": False})
            raise RuntimeError("competing build or benchmark process is active")
        arm = arm_rep.split(":")[0]
        call = guarded_run(command(args.bin, args.target_seed, arm), 900)
        item = {"id": key, "target_seed": args.target_seed, "arm": arm, **call}
        append(args.store, item)
        rec[key] = item
        print(json.dumps({"id": key, **summary(item)}, sort_keys=True), flush=True)
        if item["load_contamination"] or item["outer_timeout"] or item["exit_code"] != 0:
            raise RuntimeError("timed invocation censored or failed; preserve this store")
    keys = [f"timed:{args.target_seed}:{arm_rep}" for arm_rep in ARMS]
    if all(key in rec for key in keys) and f"verdict:{args.target_seed}" not in rec:
        calls = [rec[key] for key in keys]
        rowsets = [call["result"]["rows"] for call in calls]
        complete = (all(call["exit_code"] == 0 and call["result"]["instance"] == INSTANCE and
                        len(rows) == 16 for call, rows in zip(calls, rowsets)) and
                    all(all(r.get("verified") and r.get("r") == ORDER and
                                (r.get("factor_base") or {}).get("abscissae") == SIZE for r in rows)
                        for rows in rowsets) and
                    all(len({rows[i].get("recovered") for rows in rowsets}) == 1 for i in range(16)))
        item = {"id": f"verdict:{args.target_seed}", "complete": complete,
                "runs": {arm_rep: summary(call) for arm_rep, call in zip(ARMS, calls)}}
        if complete:
            a, b, c, d = [summary(call) for call in calls]
            item["pairs"] = [{"gae": b["total_gae"] / a["total_gae"],
                              "wall": b["wall_s"] / a["wall_s"],
                              "cpu": b["child_cpu_s"] / a["child_cpu_s"]},
                             {"gae": c["total_gae"] / d["total_gae"],
                              "wall": c["wall_s"] / d["wall_s"],
                              "cpu": c["child_cpu_s"] / d["child_cpu_s"]}]
        append(args.store, item)
        print(json.dumps(item, sort_keys=True))


if __name__ == "__main__":
    main()
