"""Independent recompute of results.json's headline from the raw records.

Shares no code with summarize.py: reads every records.jsonl itself, takes means by summing and
builds the paired S ratios from dictionaries keyed by (session, target, round).  Only verified,
non-warm-up runs count, as in the summary.  Writes validator-report.json (validation-gate/v1
validator report).  Standard library only.
"""
import hashlib
import json
import math
import os
import pathlib

HERE = pathlib.Path(__file__).parent
REPO = HERE.parent.parent
EVIDENCE = REPO / "benchmarks" / "20261028-ic-sweep-ext"


def main():
    claimed = {h["name"]: h["claimed"] for h in json.loads((HERE / "results.json").read_text())["headline"]}
    raw, by = [], {}
    for path in sorted((HERE / "sessions").glob("*/records.jsonl")):
        bits = path.parent.name.split("-")[1]
        raw.append({"path": os.path.relpath(path, EVIDENCE), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()})
        for line in path.read_text().splitlines():
            r = json.loads(line)
            if r["warmup"] or r["outcome"]["status"] != "verified":
                continue
            k = (r["session_id"], r["workload"]["target_index"], r["round"])
            by.setdefault((bits, r["arm"]), {})[k] = r["cost"]["s"]
    headline = []
    for (bits, arm), recs in sorted(by.items()):
        headline.append({"name": f"{bits}/{arm}/mean_s", "claimed": claimed.get(f"{bits}/{arm}/mean_s"),
                         "recomputed": sum(recs.values()) / len(recs), "rel_tol": 1e-9})
        name = f"{bits}/{arm}/paired_s_vs_incumbent"
        if name in claimed:
            inc = next(v for (b, a), v in by.items() if b == bits and a.startswith("pair-walk-b"))
            logs = [math.log(v / inc[k]) for k, v in recs.items() if k in inc]
            headline.append({"name": name, "claimed": claimed[name], "recomputed": math.exp(sum(logs) / len(logs)), "rel_tol": 1e-9})
    ok = len(headline) == len(claimed) and all(
        h["claimed"] is not None and abs(h["claimed"] - h["recomputed"]) <= 1e-9 * abs(h["recomputed"]) for h in headline)
    report = {"schema": "validation-gate/v1 validator report", "status": "pass" if ok else "fail", "independent": True,
              "method": "validator.py: means by summing and paired ratios from per-key dictionaries over raw records; no shared code with summarize.py",
              "raw_data": raw, "headline": headline}
    (HERE / "validator-report.json").write_text(json.dumps(report, indent=1, sort_keys=True) + "\n")
    print(report["status"], len(headline), "headline numbers")


if __name__ == "__main__":
    main()
