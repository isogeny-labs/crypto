#!/usr/bin/env python3
"""Route 3: m=2 vs m=3 panel on a public 13-bit instance.

Frozen shape copied from the m=2 evidence scripts (order interval, candidate,
candidate, interval; identical seed, instance and base size), but m, trial cap,
solver budget and repeats are explicit so the m=3 arm can be run at whatever
protocol is actually feasible.  Every call is appended to an append-only JSONL
store; a store is never rewritten.

Read-only import of f4_height4 (repo file) for run()/append() helpers.
"""
import argparse
import hashlib
import json
import statistics
import sys
from pathlib import Path

REPO = Path("/Volumes/SSD990-2/.codex/worktrees/crypto-pkm-map-sweep-20261123")
RESEARCH = REPO / "research/pkm_adaptive_sweep_20261123"
sys.path.insert(0, str(RESEARCH))
import f4_height4 as f4  # noqa: E402

EXPECTED_SHA = "1ae5170f9b3362f75a636909e1f5d01fb0ead0fd4886627d6012e2b9de9d8d08"


def command(args, fb, oracle, preflight):
    cmd = [str(args.bin), "bench", "--bits", str(args.bits), "--family", "generic",
           "--seed", str(args.seed), "--repeats", str(1 if preflight else args.repeats),
           "--targets", "walk", "--factor-base", fb,
           "--oracle", "subtract" if preflight else oracle,
           "--max-trials", "1" if preflight else str(args.max_trials),
           "--rho-runs", "0" if preflight else "1", "--json"]
    if not preflight:
        cmd += ["--solver-budget-seconds", str(args.budget)]
    return cmd


def size(item):
    rows = (item.get("result") or {}).get("rows") or []
    values = {r.get("factor_base", {}).get("abscissae") for r in rows}
    return values.pop() if len(rows) == 1 and len(values) == 1 else None


def summarise(rows):
    """Per-arm aggregates over the repeated rows."""
    sol = [r["decomposition"]["solver"] for r in rows]
    sysd = [r["decomposition"]["system"] for r in rows]
    dec = [r["decomposition"] for r in rows]
    censored = sum(1 for s in sol if s.get("budget_exceeded"))
    return {
        "rows": len(rows),
        "vars": sysd[0]["n_vars"], "equations": sysd[0]["n_equations"],
        "degrees": sysd[0]["degrees"],
        "solving_degree_max": max(s["solving_degree_max"] for s in sol),
        "solver_calls": sum(s["calls"] for s in sol),
        "solver_ops": sum(s["ops"] for s in sol),
        "solver_ops_median": statistics.median([s["ops"] for s in sol]),
        "censored_calls": censored,
        "budget_exceeded_rows": censored,
        "solver_wall_s": sum(s["wall_ns"] for s in sol) / 1e9,
        "max_cols": max((s.get("extra") or {}).get("max_cols_max", 0) for s in sol),
        "max_rows": max((s.get("extra") or {}).get("max_rows_max", 0) for s in sol),
        "max_nnz": max((s.get("extra") or {}).get("max_nnz_max", 0) for s in sol),
        "relations": sum(d["relations_found"] for d in dec),
        "trials": sum(d["targets_tried"] for d in dec),
        "relation_prob_per_trial": (sum(d["relations_found"] for d in dec) /
                                    max(1, sum(d["targets_tried"] for d in dec))),
        "total_gae": sum(r["total_gae"] for r in rows),
        "gae_median": statistics.median([r["total_gae"] for r in rows]),
        "s_over_rho_median": statistics.median([r["s_over_rho"] for r in rows
                                                if isinstance(r.get("s_over_rho"), (int, float))])
        if any(isinstance(r.get("s_over_rho"), (int, float)) for r in rows) else None,
        "verified_all": all(r.get("verified") for r in rows),
        "wall_s": sum(r["wall_seconds"] for r in rows),
    }


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--bits", type=int, default=13)
    ap.add_argument("--seed", type=int, default=20261185)
    ap.add_argument("--repeats", type=int, default=16)
    ap.add_argument("--max-trials", type=int, default=1000)
    ap.add_argument("--budget", type=int, default=5)
    ap.add_argument("--degree", type=int, default=8)
    ap.add_argument("--m", type=int, default=3)
    ap.add_argument("--interval", default="prime-abscissa:below=32")
    ap.add_argument("--candidate", default="prime-isogeny:t=5,seed=5,aux=ordinary")
    ap.add_argument("--order", default="interval,candidate,candidate,interval")
    ap.add_argument("--call-cap", type=float, default=100000)
    ap.add_argument("--store", type=Path, required=True)
    ap.add_argument("--group", required=True)
    args = ap.parse_args()

    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED_SHA:
        raise SystemExit(f"binary differs: {digest}")

    oracles = {
        "interval": f"pkm-semaev:m={args.m},engine=f4,degree={args.degree},"
                    f"bits=5,lo=0,mode=chain",
        "candidate": f"pkm-semaev:m={args.m},engine=f4,degree={args.degree},mode=chain",
    }
    fbs = {"interval": args.interval, "candidate": args.candidate}

    records = {}
    if args.store.exists():
        for line in args.store.read_text().splitlines():
            item = json.loads(line)
            if item["id"] in records:
                raise SystemExit(f"duplicate {item['id']}")
            records[item["id"]] = item

    # preflights: base sizes for the two arms
    for arm in ("interval", "candidate"):
        key = f"{args.group}:preflight:{arm}"
        if key not in records:
            item = {"id": key, "group": args.group, "arm": arm, "kind": "preflight",
                    **f4.run(command(args, fbs[arm], "", True), 60)}
            f4.append(args.store, item)
            records[key] = item
    a = records[f"{args.group}:preflight:interval"]
    b = records[f"{args.group}:preflight:candidate"]
    sa, sb = size(a), size(b)
    if a["exit_code"] not in (0, 1) or b["exit_code"] not in (0, 1):
        f4.append(args.store, {"id": f"{args.group}:verdict", "group": args.group,
                               "complete": False, "reason": "preflight-failed"})
        return
    if a["result"].get("instance") != b["result"].get("instance") or sa != sb:
        f4.append(args.store, {"id": f"{args.group}:verdict", "group": args.group,
                               "complete": False, "reason": "size-mismatch",
                               "sizes": [sa, sb]})
        return

    order = [x.strip() for x in args.order.split(",")]
    for i, arm in enumerate(order, 1):
        key = f"{args.group}:timed:{i}:{arm}"
        if key in records:
            continue
        item = {"id": key, "group": args.group, "arm": arm, "kind": "timed",
                "index": i, "oracle": oracles[arm], "factor_base": fbs[arm],
                **f4.run(command(args, fbs[arm], oracles[arm], False), args.call_cap)}
        f4.append(args.store, item)
        records[key] = item

    runs = [records[f"{args.group}:timed:{i}:{arm}"] for i, arm in enumerate(order, 1)]
    arms = {}
    for arm in ("interval", "candidate"):
        rows = [r for r in runs if r["arm"] == arm]
        for r in rows:
            if not r.get("result") or not (r["result"].get("rows") or []):
                raise SystemExit(f"no rows in {r['id']}")
        rows = [x for r in rows for x in r["result"]["rows"]]
        arms[arm] = summarise(rows)
        arms[arm]["base_size"] = sa

    verdict = {"id": f"{args.group}:verdict", "group": args.group,
               "instance": a["result"].get("instance"), "bits": args.bits,
               "seed": args.seed, "m": args.m, "repeats": args.repeats,
               "max_trials": args.max_trials, "solver_budget_s": args.budget,
               "degree": args.degree, "base_size": sa,
               "order": order, "arms": arms,
               "gae_ratio_int_over_cand": (arms["interval"]["total_gae"] /
                                           arms["candidate"]["total_gae"]
                                           if arms["candidate"]["total_gae"] else None),
               "censored_any": (arms["interval"]["censored_calls"] > 0 or
                                arms["candidate"]["censored_calls"] > 0),
               "complete": (all(r["exit_code"] == 0 for r in runs)
                            and all(arms[k]["verified_all"] and
                                    len(r["result"]["rows"]) == args.repeats for r in runs
                                    for k in (r["arm"],)))}
    f4.append(args.store, verdict)
    print(json.dumps(verdict, sort_keys=True))


if __name__ == "__main__":
    main()
