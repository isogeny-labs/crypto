#!/usr/bin/env python3
"""Descriptive K13 L0 screen; retain failed/censored rows and exact pairs."""
import argparse
import hashlib
import json
import statistics
from collections import Counter, defaultdict
from pathlib import Path

ARMS = ("rho-strong", "matrix-f4", "matrix-f5", "inherited-f4", "f6-ic", "rootfind-m3", "batch-m3", "batch-m3-aa")
BASELINES = ("matrix-f4", "matrix-f5", "inherited-f4", "f6-ic", "rootfind-m3", "batch-m3-aa", "rho-strong")
SPEC_ID = "ECS1h326f1843d687"
TARGET_SEED = 202611160511
EXPECTED = 48


def read_jsonl(path):
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()] if path.exists() else []


def median(values):
    return statistics.median(values) if values else None


def valid(row):
    outcome = row.get("outcome") or {}
    wall = (row.get("time") or {}).get("process_wall_ns")
    return outcome.get("status") == "verified" and outcome.get("matches_target") is True and isinstance(wall, (int, float)) and wall > 0


def key(row):
    return row.get("arm"), (row.get("workload") or {}).get("workload_id"), row.get("round")


def analyze(session_dir, replay_path, work_path):
    session_path = session_dir / "session.json"
    records_path = session_dir / "records.jsonl"
    session = json.loads(session_path.read_text())
    records = read_jsonl(records_path)
    replay = json.loads(replay_path.read_text()) if replay_path.exists() else None
    work = read_jsonl(work_path)
    ids = [r.get("record_id") for r in records]
    duplicates = {k: v for k, v in Counter(ids).items() if v > 1}
    measured = [r for r in records if not r.get("warmup")]
    keyed = {key(r): r for r in measured}
    work_by_id = {r.get("record_id"): r for r in work}
    arms = {}
    for arm in ARMS:
        rows = [r for r in measured if r.get("arm") == arm]
        good = [r for r in rows if valid(r)]
        phases = defaultdict(list)
        for r in good:
            for phase in r.get("phases") or []:
                value = phase.get("wall_ns")
                if isinstance(value, (int, float)):
                    phases[phase.get("name", "unknown")].append(value)
        work_rows = [work_by_id[r.get("record_id")] for r in rows if r.get("record_id") in work_by_id]
        arms[arm] = {
            "measured_rows": len(rows),
            "valid_rows": len(good),
            "outcome_statuses": dict(Counter((r.get("outcome") or {}).get("status", "missing") for r in rows)),
            "process_wall_ns_median_valid": median([(r.get("time") or {}).get("process_wall_ns") for r in good]),
            "method_call_wall_ns_median_valid": median([(r.get("time") or {}).get("method_call_wall_ns") for r in good if isinstance((r.get("time") or {}).get("method_call_wall_ns"), (int, float))]),
            "max_rss_kib_median_valid": median([(r.get("time") or {}).get("max_rss_kib") for r in good if isinstance((r.get("time") or {}).get("max_rss_kib"), (int, float))]),
            "factor_base_sha256s_valid": sorted({(r.get("factor_base") or {}).get("fb_sha256") for r in good if (r.get("factor_base") or {}).get("fb_sha256")}),
            "counter_median_valid": {name: median([(r.get("counters") or {}).get(name) for r in good if isinstance((r.get("counters") or {}).get(name), (int, float))]) for name in ("targets_tried", "relations_found", "matrix_rows", "matrix_rank", "matrix_dependent")},
            "phase_wall_ns_median_valid": {name: median(vals) for name, vals in sorted(phases.items())},
            "work_rows": len(work_rows),
            "work_complete_rows": sum(bool((x.get("work") or {}).get("Ok", {}).get("complete")) for x in work_rows),
        }
    comparisons = {}
    candidate = [r for r in measured if r.get("arm") == "batch-m3"]
    for baseline in BASELINES:
        pairs = []
        omitted = []
        for c in candidate:
            b = keyed.get((baseline, (c.get("workload") or {}).get("workload_id"), c.get("round")))
            if b is None:
                omitted.append({"candidate_record_id": c.get("record_id"), "reason": "missing_baseline"})
            elif not valid(c) or not valid(b):
                omitted.append({"candidate_record_id": c.get("record_id"), "baseline_record_id": b.get("record_id"), "reason": "unverified_or_missing_wall"})
            else:
                pairs.append({"workload_id": c["workload"]["workload_id"], "round": c["round"], "candidate_record_id": c["record_id"], "baseline_record_id": b["record_id"], "candidate_ns": c["time"]["process_wall_ns"], "baseline_ns": b["time"]["process_wall_ns"]})
        comparison = {"valid_pairs": len(pairs), "omitted_pairs": omitted, "sum_process_wall_ratio_valid": sum(x["candidate_ns"] for x in pairs)/sum(x["baseline_ns"] for x in pairs) if pairs else None, "median_pair_ratio_valid": median([x["candidate_ns"]/x["baseline_ns"] for x in pairs]), "pairs_below_one": sum(x["candidate_ns"] < x["baseline_ns"] for x in pairs), "pairs": pairs}
        comparisons[f"batch-m3/{baseline}"] = comparison
    output = {
        "schema": "rootfind-k13-local-f456-screen/v1",
        "scope": "separate fresh-public-target L0 engineering screen; descriptive only",
        "session_id": session.get("session_id"), "session_status": session.get("status"),
        "spec_id": session.get("spec_id"), "spec_matches_frozen": session.get("spec_id") == SPEC_ID,
        "git_commit": session.get("git_commit"), "binary_sha256": session.get("binary_sha256"),
        "records_sha256": hashlib.sha256(records_path.read_bytes()).hexdigest(),
        "replay_sha256": hashlib.sha256(replay_path.read_bytes()).hexdigest() if replay_path.exists() else None,
        "replay_ok": replay.get("ok") if replay else None,
        "record_count": len(records), "expected_record_count": EXPECTED,
        "measured_count": len(measured), "duplicate_record_ids": duplicates,
        "foreign_target_seed_count": sum((r.get("workload") or {}).get("target_seed") != TARGET_SEED for r in records),
        "isolation_levels": dict(Counter((r.get("isolation") or {}).get("level", "missing") for r in records)),
        "arms": arms, "comparisons": comparisons,
    }
    return output


def svg(output):
    lines = ['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="360" viewBox="0 0 900 360">', '<rect width="900" height="360" fill="white"/>', '<text x="35" y="30" font-family="sans-serif" font-size="18">K13 local complete-cold wall: batch / reference</text>', '<text x="35" y="52" font-family="sans-serif" font-size="12">Only verified exact target-round pairs; L0 diagnostic, no qualified claim</text>']
    for i, baseline in enumerate(BASELINES):
        row = output["comparisons"][f"batch-m3/{baseline}"]
        ratio = row["sum_process_wall_ratio_valid"]
        y = 85 + i*36
        label = f"{baseline}: {ratio:.4g} ({row['valid_pairs']} pairs)" if ratio is not None else f"{baseline}: censored (0 pairs)"
        lines.append(f'<text x="35" y="{y+16}" font-family="sans-serif" font-size="14">{label}</text>')
        if ratio is not None:
            width = min(450, round(250*ratio))
            lines.append(f'<rect x="430" y="{y}" width="{width}" height="20" fill="#28649a"/>')
    lines.extend(['<line x1="680" y1="74" x2="680" y2="340" stroke="#8a271c" stroke-dasharray="4 3"/>', '<text x="685" y="345" font-family="sans-serif" font-size="11">ratio 1</text>', '</svg>'])
    return "\n".join(lines)+"\n"


def main():
    p=argparse.ArgumentParser()
    p.add_argument("--session", type=Path, required=True)
    p.add_argument("--replay", type=Path, required=True)
    p.add_argument("--work", type=Path, required=True)
    p.add_argument("--out", type=Path, required=True)
    p.add_argument("--svg", type=Path, required=True)
    args=p.parse_args()
    result=analyze(args.session,args.replay,args.work)
    args.out.parent.mkdir(parents=True,exist_ok=True)
    args.out.write_text(json.dumps(result,indent=2,sort_keys=True)+"\n")
    args.svg.write_text(svg(result))
    print(json.dumps({"session_id":result["session_id"],"record_count":result["record_count"],"replay_ok":result["replay_ok"],"ratios":{k:v["sum_process_wall_ratio_valid"] for k,v in result["comparisons"].items()}},indent=2))

if __name__ == "__main__":
    main()
