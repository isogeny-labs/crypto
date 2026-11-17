"""Independent recompute of results.json's headline from the raw records.

Shares no code with summarize.py: reads every records.jsonl itself, builds the paired
ratios from dictionaries keyed by (session, target, round) and takes geometric means as
exp of a plain average of logs.  Writes validator-report.json (validation-gate/v1
validator report).  Standard library only.
"""
import hashlib
import json
import math
import os
import pathlib

HERE = pathlib.Path(__file__).parent
REPO = HERE.parent.parent
EVIDENCE = REPO / "benchmarks" / "20261028-ic-wide-batching"


def main():
    claimed = {h["name"]: h["claimed"] for h in json.loads((HERE / "results.json").read_text())["headline"]}
    raw, by = [], {}
    for path in sorted((HERE / "sessions").glob("*/records.jsonl")):
        bits = path.parent.name.split("-")[1]
        raw.append({"path": os.path.relpath(path, EVIDENCE), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()})
        for line in path.read_text().splitlines():
            r = json.loads(line)
            if r["warmup"] or r["method"]["id"] != "ic.pipeline":
                continue
            rel = [p["wall_ns"] for p in r["phases"] if p["name"] == "relations"][0]
            k = (bits, r["session_id"], r["workload"]["target_index"], r["round"])
            by.setdefault((bits, r["arm"]), {})[k] = (rel, r["time"]["solve_wall_ns"], r["cost"]["s"])
    headline = []
    for (bits, arm), recs in sorted(by.items()):
        if arm == "walk":
            continue
        base = by[(bits, "walk")]
        for idx, label in ((0, "relations_wall_ratio"), (1, "solve_wall_ratio"), (2, "s_ratio")):
            logs = [math.log(v[idx] / base[k][idx]) for k, v in recs.items() if k in base]
            headline.append({"name": f"{bits}/{arm}/{label}", "claimed": claimed.get(f"{bits}/{arm}/{label}"),
                             "recomputed": math.exp(sum(logs) / len(logs)), "rel_tol": 1e-9})
    ok = len(headline) == len(claimed) and all(
        h["claimed"] is not None and abs(h["claimed"] - h["recomputed"]) <= 1e-9 * abs(h["recomputed"]) for h in headline)
    report = {"schema": "validation-gate/v1 validator report", "status": "pass" if ok else "fail", "independent": True,
              "method": "validator.py: paired ratios from per-key dictionaries, geometric means as exp of averaged logs, over raw records; no shared code with summarize.py",
              "raw_data": raw, "headline": headline}
    (HERE / "validator-report.json").write_text(json.dumps(report, indent=1, sort_keys=True) + "\n")
    print(report["status"], len(headline), "headline numbers")


if __name__ == "__main__":
    main()
