"""Independent recompute of results.json's headline from the raw records.

Shares no code with summarize.py: reads every records.jsonl itself, builds paired ratios from
dictionaries keyed by (session, target, round) and takes geometric means as exp of an average
of logs, means by summing.  Writes validator-report.json (validation-gate/v1 validator report).
Standard library only.
"""
import hashlib
import json
import math
import os
import pathlib

HERE = pathlib.Path(__file__).parent
REPO = HERE.parent.parent
EVIDENCE = REPO / "benchmarks" / "20261028-ic-wide-ext"


def rel(r):
    return [p["wall_ns"] for p in r["phases"] if p["name"] == "relations"][0]


def gm(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def main():
    claimed = {h["name"]: h["claimed"] for h in json.loads((HERE / "results.json").read_text())["headline"]}
    raw, headline = [], []
    sessions = {}
    for path in sorted((HERE / "sessions").glob("*/records.jsonl")):
        raw.append({"path": os.path.relpath(path, EVIDENCE), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()})
        sessions[path.parent.name] = [json.loads(l) for l in path.read_text().splitlines() if l.strip()]
    # old/new binary: pair ab-old-i with ab-new-i on (arm, target, round)
    logs = {}
    for i in (1, 2, 3):
        old = {(r["arm"], r["workload"]["target_index"], r["round"]): r for r in sessions[f"ab-old-{i}"] if not r["warmup"]}
        new = {(r["arm"], r["workload"]["target_index"], r["round"]): r for r in sessions[f"ab-new-{i}"] if not r["warmup"]}
        for k, o in old.items():
            if o["method"]["id"] == "ic.pipeline":
                logs.setdefault(k[0], []).append(rel(new[k]) / rel(o))
    for arm, xs in sorted(logs.items()):
        headline.append({"name": f"old_new/{arm}/relations", "claimed": claimed.get(f"old_new/{arm}/relations"),
                         "recomputed": gm(xs), "rel_tol": 1e-9})
    # grid: pool the sessions of a size; ratios on (session, target, round)
    for bits in (28, 32):
        pooled = {}
        for name, recs in sessions.items():
            if name.startswith(f"grid-{bits}-"):
                for r in recs:
                    if not r["warmup"]:
                        pooled.setdefault(r["arm"], {})[(name, r["workload"]["target_index"], r["round"])] = r
        for arm, recs in sorted(pooled.items()):
            if arm == "rho-neg":
                continue
            mean_s = sum(r["cost"]["s"] for r in recs.values()) / len(recs)
            headline.append({"name": f"grid-{bits}/{arm}/mean_s", "claimed": claimed.get(f"grid-{bits}/{arm}/mean_s"),
                             "recomputed": mean_s, "rel_tol": 1e-9})
            for ref, label in (("b1024-W", "vs_incumbent_walk"), ("b1024-L16", "vs_round3_default")):
                base = pooled[ref]
                xs = [r["time"]["solve_wall_ns"] / base[k]["time"]["solve_wall_ns"] for k, r in recs.items()]
                headline.append({"name": f"grid-{bits}/{arm}/{label}", "claimed": claimed.get(f"grid-{bits}/{arm}/{label}"),
                                 "recomputed": gm(xs), "rel_tol": 1e-9})
    # the claimed set must be exactly the recomputed set; the grid S of the reference arm is not claimed
    names = {h["name"] for h in headline}
    extra = [n for n in claimed if n not in names]
    ok = not extra and all(h["claimed"] is not None and abs(h["claimed"] - h["recomputed"]) <= 1e-9 * abs(h["recomputed"]) for h in headline)
    report = {"schema": "validation-gate/v1 validator report", "status": "pass" if ok else "fail", "independent": True,
              "method": "validator.py: means by summing, paired ratios from per-key dictionaries, geometric means as exp of averaged logs, over raw records; no shared code with summarize.py",
              "raw_data": raw, "headline": headline, "unmatched_claims": extra}
    (HERE / "validator-report.json").write_text(json.dumps(report, indent=1, sort_keys=True) + "\n")
    print(report["status"], len(headline), "headline numbers", "extra claims:", extra[:3])


if __name__ == "__main__":
    main()
