"""Independent recompute of results.json's headline from the raw records.

Shares no code with summarize.py: reads every records.jsonl itself, builds paired ratios from
dictionaries keyed by (rep, arm, target, round) and takes geometric means as exp of an average of
logs.  Writes validator-report.json (validation-gate/v1 validator report).  Standard library only.
"""
import hashlib
import json
import math
import os
import pathlib

HERE = pathlib.Path(__file__).parent
REPO = HERE.parent.parent
EVIDENCE = REPO / "benchmarks" / "20261029-ic-table-floor"


def phase(r, name):
    for p in r["phases"]:
        if p["name"] == name:
            return p["wall_ns"]
    return None


def gm(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def main():
    claimed = {h["name"]: h["claimed"] for h in json.loads((HERE / "results.json").read_text())["headline"]}
    raw, sessions = [], {}
    for path in sorted((HERE / "sessions").glob("*/records.jsonl")):
        raw.append({"path": os.path.relpath(path, EVIDENCE), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()})
        sessions[path.parent.name] = [json.loads(l) for l in path.read_text().splitlines() if l.strip()]
    headline = []
    for bits in (24, 28, 32):
        ratios = {}
        solve = {}
        for rep in (1, 2, 3):
            old = {(r["arm"], r["workload"]["target_index"], r["round"]): r for r in sessions.get(f"ab-old-{bits}-{rep}", []) if not r["warmup"]}
            new = {(r["arm"], r["workload"]["target_index"], r["round"]): r for r in sessions.get(f"ab-new-{bits}-{rep}", []) if not r["warmup"]}
            for k, o in old.items():
                if k[0] == "rho-neg" or k not in new:
                    continue
                n = new[k]
                solve.setdefault(k[0], []).append(n["time"]["solve_wall_ns"] / o["time"]["solve_wall_ns"])
                for p in o["phases"]:
                    a, b = phase(n, p["name"]), p["wall_ns"]
                    if a and b:
                        ratios.setdefault((k[0], p["name"]), []).append(a / b)
        for (arm, name), xs in sorted(ratios.items()):
            if len(xs) > 1 and f"{bits}/{arm}/{name}" in claimed:
                headline.append({"name": f"{bits}/{arm}/{name}", "claimed": claimed[f"{bits}/{arm}/{name}"], "recomputed": gm(xs), "rel_tol": 1e-9})
        for arm, xs in sorted(solve.items()):
            headline.append({"name": f"{bits}/{arm}/solve", "claimed": claimed.get(f"{bits}/{arm}/solve"), "recomputed": gm(xs), "rel_tol": 1e-9})
    names = {h["name"] for h in headline}
    extra = [n for n in claimed if n not in names]
    ok = bool(headline) and not extra and all(h["claimed"] is not None and abs(h["claimed"] - h["recomputed"]) <= 1e-9 * abs(h["recomputed"]) for h in headline)
    report = {"schema": "validation-gate/v1 validator report", "status": "pass" if ok else "fail", "independent": True,
              "method": "validator.py: paired ratios from per-key dictionaries, geometric means as exp of averaged logs, over raw records; no shared code with summarize.py",
              "raw_data": raw, "headline": headline, "unmatched_claims": extra}
    (HERE / "validator-report.json").write_text(json.dumps(report, indent=1, sort_keys=True) + "\n")
    print(report["status"], len(headline), "headline numbers", "extra claims:", extra[:3])


if __name__ == "__main__":
    main()
