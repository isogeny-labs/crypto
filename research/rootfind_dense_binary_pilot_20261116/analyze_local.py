#!/usr/bin/env python3
"""Descriptive, fixed-session K13 local diagnostic; no claim admission."""

import argparse
import hashlib
import json
import statistics
from collections import Counter, defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parent
SESSION = ROOT / "sessions/local-fresh-seed-202611160401"
WORK = ROOT / "reports/work-records.jsonl"
AUDIT = ROOT / "audits/replay-all.json"
OUT = ROOT / "reports/analysis.json"
SVG = ROOT / "reports/paired-ratios.svg"
ARMS = ("rho-strong", "f6-ic", "rootfind-m3", "batch-m3", "batch-m3-aa")


def read_jsonl(path):
    return [json.loads(line) for line in path.read_text().splitlines() if line]


def med(values):
    return statistics.median(values)


def main():
    session = json.loads((SESSION / "session.json").read_text())
    audit = json.loads(AUDIT.read_text())
    records_path = SESSION / "records.jsonl"
    records = read_jsonl(records_path)
    work = read_jsonl(WORK)
    assert session["session_id"] in {"ECBS1h3cf526729f36", "ECBS1h08f6d5475be8"}
    assert session["spec_id"] == "ECS1hb376719703c9"
    assert session["status"] == "complete" and len(records) == 200
    assert audit["ok"] and audit["session_id"] == session["session_id"]
    assert audit["replay_candidates"] == 160 and not audit["problems"]
    ids = [r["record_id"] for r in records]
    assert len(set(ids)) == len(ids)
    assert Counter(x["record_id"] for x in work) == Counter(ids)
    assert all(r["outcome"]["status"] == "verified" and r["outcome"]["matches_target"] for r in records)
    assert all(r["isolation"]["level"] == "L0" for r in records)
    assert all(r["workload"]["target_seed"] == 202611160401 for r in records)
    assert all(r["binary_sha256"] == session["binary_sha256"] for r in records)

    measured = [r for r in records if not r["warmup"]]
    by_arm = defaultdict(list)
    for r in measured:
        by_arm[r["arm"]].append(r)
    assert set(by_arm) == set(ARMS)
    assert all(len(by_arm[arm]) == 32 for arm in ARMS)
    keyed = {(r["arm"], r["workload"]["workload_id"], r["round"]): r for r in measured}
    assert len(keyed) == len(measured)
    workloads = sorted({r["workload"]["workload_id"] for r in measured})
    assert len(workloads) == 8
    work_by_id = {x["record_id"]: x for x in work}

    arms = {}
    for arm in ARMS:
        rows = by_arm[arm]
        phases = defaultdict(list)
        for r in rows:
            for phase in r["phases"]:
                phases[phase["name"]].append(phase["wall_ns"])
        work_rows = [work_by_id[r["record_id"]] for r in rows]
        walls = [r["time"]["process_wall_ns"] for r in rows]
        methods = [r["time"].get("method_call_wall_ns") for r in rows]
        bitops = [x["work"].get("Ok", {}).get("bitops") for x in work_rows]
        arms[arm] = {
            "n": len(rows),
            "process_wall_ns_median": med(walls),
            "process_wall_ns_min": min(walls),
            "process_wall_ns_max": max(walls),
            "method_call_wall_ns_median": med([x for x in methods if x is not None]) if any(x is not None for x in methods) else None,
            "phase_wall_ns_median": {
                name: med([value for value in values if value is not None])
                if any(value is not None for value in values) else None
                for name, values in phases.items()
            },
            "work_rows": len(work_rows),
            "work_complete": sum(x["work"].get("Ok", {}).get("complete", False) for x in work_rows),
            "bitop_v1_median": med([int(x) for x in bitops if x is not None]) if any(x is not None for x in bitops) else None,
        }

    comparisons = {}
    for baseline in ("f6-ic", "rootfind-m3", "batch-m3-aa", "rho-strong"):
        pairs = []
        for candidate in by_arm["batch-m3"]:
            key = (baseline, candidate["workload"]["workload_id"], candidate["round"])
            reference = keyed[key]
            pairs.append((candidate["workload"]["workload_id"], candidate["time"]["process_wall_ns"], reference["time"]["process_wall_ns"]))
        target_ratios = {}
        for workload in workloads:
            group = [pair for pair in pairs if pair[0] == workload]
            assert len(group) == 4
            target_ratios[workload] = sum(p[1] for p in group) / sum(p[2] for p in group)
        ratios = [p[1] / p[2] for p in pairs]
        comparisons[f"batch-m3/{baseline}"] = {
            "n_pairs": len(pairs),
            "sum_process_wall_ratio": sum(p[1] for p in pairs) / sum(p[2] for p in pairs),
            "pair_ratio_min": min(ratios),
            "pair_ratio_median": med(ratios),
            "pair_ratio_max": max(ratios),
            "pairs_below_one": sum(x < 1 for x in ratios),
            "target_sum_ratios": target_ratios,
        }

    result = {
        "schema": "rootfind-k13-local-diagnostic/v1",
        "scope": "fresh-public-target local L0 engineering diagnostic; not the EKS confirmatory panel",
        "session_id": session["session_id"],
        "spec_id": session["spec_id"],
        "git_commit": session["git_commit"],
        "binary_sha256": session["binary_sha256"],
        "records_sha256": hashlib.sha256(records_path.read_bytes()).hexdigest(),
        "audit_sha256": hashlib.sha256(AUDIT.read_bytes()).hexdigest(),
        "records_total": len(records),
        "measured_total": len(measured),
        "verified_total": len(records),
        "isolation": "L0",
        "arms": arms,
        "comparisons": comparisons,
    }
    OUT.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    draw_svg(result, workloads)
    print(json.dumps({"session_id": result["session_id"], "comparisons": comparisons}, indent=2))


def draw_svg(result, workloads):
    colors = {"f6-ic": "#2766a5", "batch-m3-aa": "#b06d19", "rho-strong": "#6845a5"}
    x0, x1, y0, y1 = 88, 760, 55, 335
    width = x1 - x0
    def y(value):
        return y1 - min(value, 1.4) / 1.4 * (y1 - y0)
    lines = [
        '<svg xmlns="http://www.w3.org/2000/svg" width="850" height="430" viewBox="0 0 850 430">',
        '<rect width="850" height="430" fill="white"/>',
        '<text x="88" y="24" font-family="sans-serif" font-size="17" font-weight="bold">K13 fresh targets: batch / reference complete cold process wall</text>',
        f'<text x="88" y="44" font-family="sans-serif" font-size="12">{result["session_id"]} · L0 local · 4 matched rounds per target · ratio below 1 favors batch</text>',
    ]
    for tick in (0, 0.25, 0.5, 0.75, 1.0, 1.25):
        yy = y(tick)
        lines.append(f'<line x1="{x0}" y1="{yy:.1f}" x2="{x1}" y2="{yy:.1f}" stroke="#d9dde5"/>')
        lines.append(f'<text x="{x0-10}" y="{yy+4:.1f}" text-anchor="end" font-family="sans-serif" font-size="11">{tick:g}</text>')
    lines.append(f'<line x1="{x0}" y1="{y(1):.1f}" x2="{x1}" y2="{y(1):.1f}" stroke="#333" stroke-dasharray="5,4"/>')
    for baseline, color in colors.items():
        ratios = result["comparisons"][f"batch-m3/{baseline}"]["target_sum_ratios"]
        pts = []
        for i, workload in enumerate(workloads):
            xx = x0 + i * width / (len(workloads) - 1)
            yy = y(ratios[workload])
            pts.append(f"{xx:.1f},{yy:.1f}")
            lines.append(f'<circle cx="{xx:.1f}" cy="{yy:.1f}" r="4" fill="{color}"/>')
        lines.append(f'<polyline points="{" ".join(pts)}" fill="none" stroke="{color}" stroke-width="2"/>')
    for i, workload in enumerate(workloads):
        xx = x0 + i * width / (len(workloads) - 1)
        lines.append(f'<text x="{xx:.1f}" y="{y1+20}" text-anchor="middle" font-family="sans-serif" font-size="10">{workload[1:5]}</text>')
    for i, baseline in enumerate(colors):
        xx = 100 + 240 * i
        lines.append(f'<line x1="{xx}" y1="386" x2="{xx+28}" y2="386" stroke="{colors[baseline]}" stroke-width="3"/>')
        lines.append(f'<text x="{xx+35}" y="390" font-family="sans-serif" font-size="12">batch / {baseline}</text>')
    lines.append('<text x="420" y="423" text-anchor="middle" font-family="sans-serif" font-size="11">Public target workload ID prefix</text>')
    lines.append('</svg>')
    SVG.write_text("\n".join(lines) + "\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--epoch", choices=("first", "rerun"), default="first")
    args = parser.parse_args()
    if args.epoch == "rerun":
        SESSION = ROOT / "sessions/local-custody-rerun-202611160401"
        WORK = ROOT / "reports/custody-rerun/work-records.jsonl"
        AUDIT = ROOT / "audits/custody-rerun/replay-all.json"
        OUT = ROOT / "reports/custody-rerun/analysis.json"
        SVG = ROOT / "reports/custody-rerun/paired-ratios.svg"
    main()
