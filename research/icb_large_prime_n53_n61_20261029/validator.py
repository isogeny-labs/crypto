"""Independent recompute of results.json's headline from the raw records.

Shares no code with summarize.py: reads every records.jsonl itself, takes means by summing and
builds the paired S ratios from dictionaries keyed by (session, target, round).  Writes
validator-report.json (validation-gate/v1 validator report).  Standard library only.
"""
import hashlib
import json
import math
import os
import pathlib

HERE = pathlib.Path(__file__).parent
REPO = HERE.parent.parent
EVIDENCE = REPO / "benchmarks" / "20261029-icb-large-prime"


def main():
    claimed = {h["name"]: h["claimed"] for h in json.loads((HERE / "results.json").read_text())["headline"]}
    raw, by = [], {}
    for path in sorted((HERE / "sessions").glob("*/records.jsonl")):
        raw.append({"path": os.path.relpath(path, EVIDENCE), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()})
        for line in path.read_text().splitlines():
            r = json.loads(line)
            if r["warmup"] or r["outcome"]["status"] != "verified":
                continue
            grp = "n53" if path.parent.name.startswith("n53") else ("n61a" if path.parent.name == "n61-s1" else "n61b")
            by.setdefault((grp, r["arm"]), {})[(path.parent.name, r["workload"]["target_index"], r["round"])] = r["cost"]["s"]
    headline = []
    for name, value in sorted(claimed.items()):
        group, _, name_rest = name.partition("/")
        if name.endswith("/mean_s"):
            recs = by[(group, name_rest[: -len("/mean_s")])]
            rec = sum(recs.values()) / len(recs)
        else:
            arm, _, rest = name_rest[: -len("/paired_ratio")].partition("/vs/")
            other = rest.split(":", 1)[1] if rest.startswith("best-plain:") else rest
            a, b = by[(group, arm)], by[(group, other)]
            logs = [math.log(v / b[k]) for k, v in a.items() if k in b]
            rec = math.exp(sum(logs) / len(logs))
        headline.append({"name": name, "claimed": value, "recomputed": rec, "rel_tol": 1e-9})
    ok = bool(headline) and all(abs(h["claimed"] - h["recomputed"]) <= 1e-9 * abs(h["recomputed"]) for h in headline)
    report = {"schema": "validation-gate/v1 validator report", "status": "pass" if ok else "fail", "independent": True,
              "method": "validator.py: means by summing and paired ratios from per-key dictionaries over raw records; no shared code with summarize.py",
              "raw_data": raw, "headline": headline}
    (HERE / "validator-report.json").write_text(json.dumps(report, indent=1, sort_keys=True) + "\n")
    print(report["status"], len(headline), "headline numbers")


if __name__ == "__main__":
    main()
