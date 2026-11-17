"""Independent recompute of results.json's headline from the raw data.

Shares no code with summarize.py: reads every records.jsonl and every
`ic price` JSON itself, takes medians by sorting, and checks each claimed
ratio.  Writes validator-report.json (validation-gate/v1 validator report).
Standard library only.
"""
import hashlib
import json
import os
import pathlib

HERE = pathlib.Path(__file__).parent
REPO = HERE.parent.parent
# validation-gate/v1 resolves raw_data against the row's evidence directory.
EVIDENCE = REPO / "benchmarks" / "20261022-algorithm-perf-round2"


def med(xs):
    xs = sorted(xs)
    n = len(xs)
    return xs[n // 2] if n % 2 else (xs[n // 2 - 1] + xs[n // 2]) / 2


def digest(path):
    return {"path": os.path.relpath(path, EVIDENCE), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def main():
    claimed = {h["name"]: h["claimed"] for h in json.loads((HERE / "results.json").read_text())["headline"]}
    raw, solve = [], {}
    for path in sorted((HERE / "sessions").glob("*/records.jsonl")):
        spec, variant, _ = path.parent.name.rsplit("-", 2)
        raw.append(digest(path))
        for line in path.read_text().splitlines():
            r = json.loads(line)
            if not r["warmup"]:
                solve.setdefault((spec, r["arm"], variant), []).append(r["time"]["solve_wall_ns"])
    setup = {}
    for path in sorted((HERE / "koblitz-ic").glob("*-*.json")):
        curve, tag = path.stem.split("-", 1)
        variant = "base" if tag.startswith("base") else "new"
        raw.append(digest(path))
        for rep in json.loads(path.read_text())["repetitions"]:
            setup.setdefault((curve, variant), []).append(rep["setup_ns"])
    headline = []
    for (spec, arm, variant), xs in sorted(solve.items()):
        if variant == "new":
            headline.append({"name": f"{spec}/{arm}/solve_ratio", "claimed": claimed.get(f"{spec}/{arm}/solve_ratio"),
                             "recomputed": med(xs) / med(solve[(spec, arm, "main")]), "rel_tol": 1e-9})
    for (curve, variant), xs in sorted(setup.items()):
        if variant == "new":
            headline.append({"name": f"koblitz/{curve}/setup_ratio", "claimed": claimed.get(f"koblitz/{curve}/setup_ratio"),
                             "recomputed": med(xs) / med(setup[(curve, "base")]), "rel_tol": 1e-9})
    ok = len(headline) == len(claimed) and all(
        h["claimed"] is not None and abs(h["claimed"] - h["recomputed"]) <= 1e-9 * abs(h["recomputed"]) for h in headline)
    report = {"schema": "validation-gate/v1 validator report", "status": "pass" if ok else "fail", "independent": True,
              "method": "validator.py: medians by sort over raw records and ic price outputs, no shared code with summarize.py",
              "raw_data": raw, "headline": headline}
    (HERE / "validator-report.json").write_text(json.dumps(report, indent=1, sort_keys=True) + "\n")
    print(report["status"], len(headline), "headline numbers")


if __name__ == "__main__":
    main()
