"""Independent recompute of results.json's headline from the raw records.

Shares no code with summarize.py: reads every records.jsonl itself, takes
medians by sorting and means by summing, and checks each claimed ratio
against the reference arm.  Writes validator-report.json
(validation-gate/v1 validator report).  Standard library only.
"""
import hashlib
import json
import os
import pathlib

HERE = pathlib.Path(__file__).parent
REPO = HERE.parent.parent
# validation-gate/v1 resolves raw_data against the row's evidence directory.
EVIDENCE = REPO / "benchmarks" / "20261023-rho-negation-lanes"


def med(xs):
    xs = sorted(xs)
    n = len(xs)
    return xs[n // 2] if n % 2 else (xs[n // 2 - 1] + xs[n // 2]) / 2


def main():
    claimed = {h["name"]: h["claimed"] for h in json.loads((HERE / "results.json").read_text())["headline"]}
    raw, wall, s = [], {}, {}
    for path in sorted((HERE / "sessions").glob("*/records.jsonl")):
        bits = path.parent.name.split("-")[1]
        raw.append({"path": os.path.relpath(path, EVIDENCE), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()})
        for line in path.read_text().splitlines():
            r = json.loads(line)
            if r["warmup"]:
                continue
            wall.setdefault((bits, r["arm"]), []).append(r["time"]["solve_wall_ns"])
            s.setdefault((bits, r["arm"]), []).append(r["cost"]["s"])
    headline = []
    for (bits, arm), xs in sorted(wall.items()):
        if arm == "rho-neg":
            continue
        ref_w, ref_s = wall[(bits, "rho-neg")], s[(bits, "rho-neg")]
        for kind, value in (("wall_ratio", med(xs) / med(ref_w)),
                            ("s_ratio", (sum(s[(bits, arm)]) / len(s[(bits, arm)])) / (sum(ref_s) / len(ref_s)))):
            name = f"{bits}/{arm}/{kind}"
            headline.append({"name": name, "claimed": claimed.get(name), "recomputed": value, "rel_tol": 1e-9})
    ok = len(headline) == len(claimed) and all(
        h["claimed"] is not None and abs(h["claimed"] - h["recomputed"]) <= 1e-9 * abs(h["recomputed"]) for h in headline)
    report = {"schema": "validation-gate/v1 validator report", "status": "pass" if ok else "fail", "independent": True,
              "method": "validator.py: medians by sort and means by sum over raw records, no shared code with summarize.py",
              "raw_data": raw, "headline": headline}
    (HERE / "validator-report.json").write_text(json.dumps(report, indent=1, sort_keys=True) + "\n")
    print(report["status"], len(headline), "headline numbers")


if __name__ == "__main__":
    main()
