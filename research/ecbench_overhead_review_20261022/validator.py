"""Independent recompute of results.json's headline from the raw records.

Shares no code with summarize.py: it reads each records.jsonl itself,
takes medians by sorting, and checks every claimed ratio. Writes
validator-report.json (validation-gate/v1 validator report) and prints
the verdict. Standard library only.
"""
import hashlib
import os
import json
import pathlib

HERE = pathlib.Path(__file__).parent
REPO = HERE.parent.parent
# validation-gate/v1 resolves raw_data against the row's evidence directory.
EVIDENCE = REPO / "benchmarks" / "20261022-ecbench-harness-overhead"


def med(xs):
    xs = sorted(xs)
    n = len(xs)
    return xs[n // 2] if n % 2 else (xs[n // 2 - 1] + xs[n // 2]) / 2


def main():
    claimed = {h["name"]: h["claimed"] for h in json.loads((HERE / "results.json").read_text())["headline"]}
    raw, times = [], {}
    for path in sorted((HERE / "sessions").glob("*/records.jsonl")):
        spec, variant, rep = path.parent.name.rsplit("-", 2)
        raw.append({"path": os.path.relpath(path, EVIDENCE), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()})
        for line in path.read_text().splitlines():
            r = json.loads(line)
            if r["warmup"]:
                continue
            t = times.setdefault((spec, r["arm"], variant), ([], []))
            t[0].append(r["time"]["solve_wall_ns"])
            t[1].append(r["time"]["process_wall_ns"])
    headline = []
    for (spec, arm, variant), (solve, proc) in sorted(times.items()):
        if variant != "final":
            continue
        old_solve, old_proc = times[(spec, arm, "main")]
        for kind, new, old in (("solve_ratio", solve, old_solve), ("process_ratio", proc, old_proc)):
            name = f"{spec}/{arm}/{kind}"
            headline.append({"name": name, "claimed": claimed.get(name), "recomputed": med(new) / med(old), "rel_tol": 1e-9})
    ok = all(h["claimed"] is not None and abs(h["claimed"] - h["recomputed"]) <= 1e-9 * abs(h["recomputed"])
             for h in headline) and len(headline) == len(claimed)
    report = {"schema": "validation-gate/v1 validator report", "status": "pass" if ok else "fail",
              "independent": True, "method": "validator.py: medians by sort over raw records, no shared code with summarize.py",
              "raw_data": raw, "headline": headline}
    (HERE / "validator-report.json").write_text(json.dumps(report, indent=1, sort_keys=True) + "\n")
    print(report["status"], len(headline), "headline numbers")


if __name__ == "__main__":
    main()
