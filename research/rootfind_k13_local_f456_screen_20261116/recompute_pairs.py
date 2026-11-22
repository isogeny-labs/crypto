#!/usr/bin/env python3
"""Producer-authored arithmetic check from raw ecbench rows, separate from analyze.py."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SESSION = ROOT / "sessions/local-custody-repeat-202611160511"
AUDIT = ROOT / "audits/repeat/replay-all.json"
ANALYSIS = ROOT / "reports/repeat/analysis.json"
OUT = ROOT / "reports/repeat"
BASELINES = ("matrix-f4", "matrix-f5", "inherited-f4", "f6-ic", "rootfind-m3", "batch-m3-aa", "rho-strong")


def file_binding(path):
    return {"path": str(path.relative_to(ROOT.parent.parent)), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def write_json(path, obj):
    path.write_text(json.dumps(obj, indent=2, sort_keys=True) + "\n")


def main():
    records_path = SESSION / "records.jsonl"
    rows = [json.loads(line) for line in records_path.read_text().splitlines() if line]
    audit = json.loads(AUDIT.read_text())
    analysis = json.loads(ANALYSIS.read_text())
    assert len(rows) == 48 and audit["records"] == 48 and audit["verified_records"] == 48
    assert audit["ok"] is True and len(audit["replays"]) == 32
    assert all(x["reproduced"] for x in audit["replays"])
    measured = [r for r in rows if r["warmup"] is False]
    assert len(measured) == 32
    by_key = {}
    for r in measured:
        key = (r["arm"], r["workload"]["workload_id"], r["round"])
        assert key not in by_key, key
        assert r["outcome"]["status"] == "verified" and r["outcome"]["matches_target"] is True
        assert r["isolation"]["level"] == "L0"
        by_key[key] = r
    candidate = [r for r in measured if r["arm"] == "batch-m3"]
    assert len(candidate) == 4 and len({r["workload"]["workload_id"] for r in candidate}) == 2
    pairs = {}
    headline = []
    for baseline in BASELINES:
        candidates_ns, baselines_ns = [], []
        for c in candidate:
            b = by_key[(baseline, c["workload"]["workload_id"], c["round"])]
            assert c["workload"]["workload_sha256"] == b["workload"]["workload_sha256"]
            assert c["factor_base"]["fb_sha256"] == b["factor_base"]["fb_sha256"] if baseline != "rho-strong" else True
            candidates_ns.append(c["time"]["process_wall_ns"])
            baselines_ns.append(b["time"]["process_wall_ns"])
        ratio = sum(candidates_ns) / sum(baselines_ns)
        claimed = analysis["comparisons"][f"batch-m3/{baseline}"]["sum_process_wall_ratio_valid"]
        assert abs(ratio - claimed) <= 1e-12 * max(1, abs(claimed))
        pairs[baseline] = {"pair_count": 4, "candidate_total_ns": sum(candidates_ns), "baseline_total_ns": sum(baselines_ns), "ratio": ratio}
        headline.append({"name": f"batch-m3/{baseline} summed complete-cold wall ratio", "claimed": claimed, "recomputed": ratio, "rel_tol": 1e-12})
    binding = [file_binding(x) for x in (records_path, AUDIT, ANALYSIS)]
    write_json(OUT / "pcc-recompute-wrapper.json", {
        "schema": "pcc.local-unsigned-recompute-wrapper/v1", "status": "pass", "note": "Producer-authored arithmetic check; no independent signature.",
        "session_id": audit["session_id"], "pair_count_per_comparison": 4, "pairs": pairs, "headline": headline, "raw_data": binding,
    })
    write_json(OUT / "pcc-replay-wrapper.json", {
        "schema": "pcc.local-unsigned-replay-wrapper/v1", "status": "pass", "note": "Producer-authored wrapper of ecbench replay-all; no independent signature.",
        "session_id": audit["session_id"], "scalars_verified": True,
        "runs": [{"record_id": x["record_id"], "seq": x["seq"]} for x in audit["replays"]],
        "source_audit": file_binding(AUDIT),
    })
    print(json.dumps(pairs, indent=2))


if __name__ == "__main__":
    main()
