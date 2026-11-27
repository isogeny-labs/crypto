#!/usr/bin/env python3
"""Resume a size-matched, cold PKM isogeny-vs-interval F4 search."""
import argparse
import hashlib
import json
import math
import os
import resource
import subprocess
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
SCREEN = range(20261123, 20261126)
CONFIRM = range(20261126, 20261150)
INTERVAL = "prime-abscissa:below=8"
INTERVAL_ORACLE = "pkm-semaev:m=2,engine=f4,degree=8,bits=3,lo=0,mode=chain"
ISOGENY_ORACLE = "pkm-semaev:m=2,engine=f4,degree=8,mode=chain"


def append(path, row):
    with path.open("a") as f:
        f.write(json.dumps(row, sort_keys=True) + "\n")
        f.flush()
        os.fsync(f.fileno())
    print(row["id"], row.get("exit_code"), row.get("decision"), flush=True)


def run(command, limit):
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    start = time.perf_counter()
    try:
        proc = subprocess.run(command, capture_output=True, text=True, timeout=limit)
        code, stdout, stderr = proc.returncode, proc.stdout, proc.stderr
    except subprocess.TimeoutExpired as exc:
        code = 124
        stdout = exc.stdout.decode(errors="replace") if isinstance(exc.stdout, bytes) else (exc.stdout or "")
        stderr = exc.stderr.decode(errors="replace") if isinstance(exc.stderr, bytes) else (exc.stderr or "")
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    try:
        result = json.loads(stdout)
    except json.JSONDecodeError:
        result = None
    return {"command": command, "exit_code": code, "wall_s": time.perf_counter() - start,
            "child_cpu_s": (after.ru_utime + after.ru_stime) - (before.ru_utime + before.ru_stime),
            "stderr_tail": stderr[-1000:], "result": result,
            "stdout_tail": stdout[-1000:] if result is None else None}


def command(binary, fb, oracle, target_seed, preflight=False):
    cmd = [str(binary), "bench", "--bits", "10", "--family", "generic",
           "--factor-base", fb, "--oracle", "subtract" if preflight else oracle,
           "--max-trials", "1" if preflight else "1000", "--rho-runs", "0" if preflight else "1",
           "--repeats", "1", "--seed", str(target_seed), "--targets", "walk", "--json"]
    if not preflight:
        cmd += ["--solver-budget-seconds", "5"]
    return cmd


def row(result):
    return ((result or {}).get("rows") or [{}])[0]


def verified_pair(base, candidate):
    a, b = row(base["result"]), row(candidate["result"])
    if not (base["exit_code"] == candidate["exit_code"] == 0):
        return False
    if not (a.get("verified") and b.get("verified")):
        return False
    if not (a.get("recovered") is not None and a.get("recovered") == b.get("recovered") and
            base["result"].get("instance") == candidate["result"].get("instance")):
        return False
    return a["factor_base"]["abscissae"] == b["factor_base"]["abscissae"] == 4


def sign_tail(wins, losses):
    n = wins + losses
    return sum(math.comb(n, k) for k in range(wins, n + 1)) / 2**n if n else 1.0


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--through-aux-seed", type=int, required=True)
    ap.add_argument("--max-wall-seconds", type=float, default=120)
    ap.add_argument("--store", type=Path, default=HERE / "matched_f4.jsonl")
    args = ap.parse_args()
    binary_hash = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    rows = {}
    if args.store.exists():
        for line in args.store.read_text().splitlines():
            item = json.loads(line)
            if item["id"] in rows:
                raise ValueError(f"duplicate row {item['id']}")
            rows[item["id"]] = item
    if "header" not in rows:
        item = {"id": "header", "schema": "pkm.matched-f4/v1", "binary_sha256": binary_hash,
                "protocol": "PROTOCOL_F4_SWEEPER_20261123.md",
                "screen_seeds": list(SCREEN), "confirm_seeds": list(CONFIRM)}
        append(args.store, item); rows[item["id"]] = item
    if rows["header"]["binary_sha256"] != binary_hash:
        raise ValueError("binary changed; use a new store")
    if any(r.get("decision") == "confirmation-lead" for r in rows.values()):
        print("confirmation lead already recorded", flush=True); return
    deadline = time.monotonic() + args.max_wall_seconds
    for aux in range(args.through_aux_seed + 1):
        if f"decision:{aux}" in rows:
            continue
        if deadline - time.monotonic() < 10:
            break
        preid = f"preflight:{aux}"
        if preid not in rows:
            fb = f"prime-isogeny:t=3,seed={aux},aux=ordinary"
            item = {"id": preid, **run(command(args.bin, fb, "", 20261123, True), 10)}
            append(args.store, item); rows[preid] = item
        pre = rows[preid]
        size = row(pre.get("result"))["factor_base"]["abscissae"] if row(pre.get("result")).get("factor_base") else None
        if size != 4:
            dec = f"decision:{aux}"
            if dec not in rows:
                item = {"id": dec, "decision": "size-mismatch", "abscissae": size}
                append(args.store, item); rows[dec] = item
            continue
        for phase, seeds in (("screen", SCREEN), ("confirm", CONFIRM)):
            if phase == "confirm":
                prior = rows.get(f"decision:{aux}")
                if prior and prior.get("decision") == "screen-reject":
                    break
            for target in seeds:
                for arm in ("baseline", "candidate"):
                    key = f"{arm}:{aux}:{target}" if arm == "candidate" else f"baseline:{target}"
                    if key in rows:
                        continue
                    if deadline - time.monotonic() < 10:
                        return
                    fb = INTERVAL if arm == "baseline" else f"prime-isogeny:t=3,seed={aux},aux=ordinary"
                    oracle = INTERVAL_ORACLE if arm == "baseline" else ISOGENY_ORACLE
                    item = {"id": key, "arm": arm, "aux_seed": aux if arm == "candidate" else None,
                            "target_seed": target, **run(command(args.bin, fb, oracle, target), 10)}
                    append(args.store, item); rows[key] = item
            pairs = [(rows[f"baseline:{target}"], rows[f"candidate:{aux}:{target}"]) for target in seeds]
            complete = all(verified_pair(a, b) for a, b in pairs)
            ratios = [row(b["result"])["total_gae"] / row(a["result"])["total_gae"]
                      for a, b in pairs] if complete else []
            walls = [b["wall_s"] / a["wall_s"] for a, b in pairs] if complete else []
            cpus = [b["child_cpu_s"] / a["child_cpu_s"] for a, b in pairs] if complete else []
            if phase == "screen":
                advance = complete and all(x < 1 for x in ratios) and all(x < 1 for x in walls)
                if not advance:
                    item = {"id": f"decision:{aux}", "decision": "screen-reject",
                            "complete": complete, "cost_ratios": ratios, "wall_ratios": walls}
                    append(args.store, item); rows[item["id"]] = item
                    break
            else:
                wins = sum(x < 1 for x in ratios)
                losses = sum(x > 1 for x in ratios)
                pval = sign_tail(wins, losses) if complete else 1.0
                alpha = .01 / ((aux + 1) * (aux + 2))
                lead = (complete and pval < alpha and
                        math.exp(sum(math.log(x) for x in ratios) / len(ratios)) < .9 and
                        sorted(walls)[len(walls)//2] < 1 and sorted(cpus)[len(cpus)//2] < 1)
                item = {"id": f"decision:{aux}", "decision": "confirmation-lead" if lead else "confirmation-reject",
                        "complete": complete, "wins": wins, "losses": losses, "paired_p": pval,
                        "alpha": alpha, "cost_ratios": ratios, "wall_ratios": walls,
                        "cpu_ratios": cpus}
                append(args.store, item); rows[item["id"]] = item
                if lead:
                    return


if __name__ == "__main__":
    main()
