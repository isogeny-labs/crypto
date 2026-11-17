#!/usr/bin/env python3
"""Bit-identity check: K1120/root11 current producer vs KIC_RANK_SOLVE=padd.

Runs both binaries (or the same binary with the switch) on one sealed point
and compares every non-timing field of the summary and result rows.
Usage: identity_check.py <base_bin> <padd_bin> <point_index 1-5> <out_dir> [solve]
"""
import importlib.util, json, os, subprocess, sys, time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location(
    "holdout", ROOT / "research/n83_e2e_holdout_20261016/run.py")
holdout = importlib.util.module_from_spec(spec); spec.loader.exec_module(holdout)

TIMING = ("timing_ms", "peak_rss_bytes", "target_online", "elapsed", "_ms", "_ns")


def strip(value):
    if isinstance(value, dict):
        return {k: strip(v) for k, v in value.items()
                if not any(t in k for t in TIMING)}
    if isinstance(value, list):
        return [strip(v) for v in value]
    return value


def run(binary, folder, point_file, extra):
    folder.mkdir(parents=True, exist_ok=True)
    env = holdout.dynamic.environment(folder, "root11_dynamic")
    env.update(extra)
    out = folder / "targets_result.jsonl"
    t = time.time()
    done = subprocess.run([str(binary), "construct:83:1:1120", str(point_file), "29", str(out)],
                          env=env, check=True, capture_output=True, text=True)
    (folder / "stdout.jsonl").write_text(done.stdout)
    rows = [json.loads(l) for l in out.read_text().splitlines() if l.strip()]
    rows += [json.loads(l) for l in done.stdout.splitlines() if l.startswith("{")]
    return rows, time.time() - t


def main():
    base, padd, idx, out = sys.argv[1:5]
    solve = sys.argv[5] if len(sys.argv) > 5 else "padd"
    out = Path(out); out.mkdir(parents=True, exist_ok=True)
    line = holdout.TARGETS.read_text().splitlines()[int(idx) - 1]
    point = out / "point.jsonl"; point.write_text(line + "\n")
    a, ta = run(base, out / "base", point, {})
    b, tb = run(padd, out / "padd", point, {"KIC_RANK_SOLVE": solve})
    sa, sb = strip(a), strip(b)
    ignore = ("rank_solve", "rank_padd")
    def norm(rows):
        return [{k: v for k, v in r.items() if not k.startswith(ignore)} for r in rows]
    same = norm(sa) == norm(sb)
    summary = next(r for r in b if r.get("kind") == "compact_orbit_dlp_summary")
    print(json.dumps({"identical_non_timing": same, "wall_base_s": round(ta, 3),
                      "wall_padd_s": round(tb, 3),
                      "rank_stage_ms_base": next(r for r in a if r.get("kind") == "compact_orbit_dlp_summary")["timing_ms"]["rank_stage"],
                      "rank_stage_ms_padd": summary["timing_ms"]["rank_stage"],
                      "rank_solve": summary.get("rank_solve"),
                      "padd_counters": {k: v for k, v in summary.items() if k.startswith("rank_padd")}}, indent=1))
    sys.exit(0 if same else 1)

main()
