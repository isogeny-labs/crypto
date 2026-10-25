#!/usr/bin/env python3
"""Progress-metrics exporter for the `crypto` repo (Rust ECDLP / index-calculus lab).

Sources (all committed, read at --ref unless noted):
  docs/ic/leaderboard.json        IC leaders per regime (S, ratio_rho, largest log2 r); history via git log
  docs/ic/progress-timeline.json  dated headline series (ec-cold etc.); points carry their own dates
  docs/perf/history.json          on branch origin/perf-history: chained Ir performance index per push
  benchmarks/README.md            append-only benchmark entries (one row per study)
  docs/ic/ci/runs/*.json          ic e2e history records
  research/nist_skipped_curve_dissection_20261021/factorization/progress.json
                                certified factorization status and recorded raw relation counts
Run: python3 tools/progress_metrics.py [--ref origin/main] [--since YYYY-MM-DD]
"""
# ---- progress-metrics exporter boilerplate (copied verbatim into each repo's tools/progress_metrics.py) ----
# Contract: print one JSON object per line to stdout. Fields: metric, value, date (YYYY-MM-DD),
# problem, unit, better (higher|lower|neutral), kind (gauge|event), label, source.
# Reads committed state at --ref via `git show`; never the working tree. stdlib only.
import argparse
import datetime as dt
import json
import os
import subprocess
import sys
from pathlib import Path

REPO = Path(os.environ.get("PM_REPO_PATH") or Path(__file__).resolve().parents[1])
TODAY = dt.date.today().isoformat()


def _git(*args, check=True):
    p = subprocess.run(["git", "-C", str(REPO), *args], capture_output=True, text=True)
    if check and p.returncode != 0:
        raise RuntimeError(f"git {' '.join(args)}: {p.stderr.strip()[:200]}")
    return p.stdout


def show(ref, path):
    p = subprocess.run(["git", "-C", str(REPO), "show", f"{ref}:{path}"], capture_output=True, text=True)
    return p.stdout if p.returncode == 0 else None


def ls(ref, path=""):
    return [l for l in _git("ls-tree", "-r", "--name-only", ref, "--", path, check=False).splitlines() if l]


def file_history(ref, path, since):
    """Yield (date, commit, text) for the newest commit per day touching `path`, newest first."""
    out = _git("log", "--first-parent", ref, f"--since={since}", "--format=%H %cs", "--", path, check=False)
    seen = set()
    for line in out.splitlines():
        h, d = line.split()
        if d in seen:
            continue
        seen.add(d)
        txt = show(h, path)
        if txt is not None:
            yield d, h[:12], txt


def emit(metric, value, *, date=None, problem="misc", unit="", better="neutral", kind="gauge", label="", source=""):
    if value is None:
        return
    try:
        v = float(value)
    except (TypeError, ValueError):
        return
    if v != v:
        return
    print(json.dumps({"metric": metric, "value": v, "date": date or TODAY, "problem": problem, "unit": unit,
                      "better": better, "kind": kind, "label": label or metric, "source": source}))


def args():
    ap = argparse.ArgumentParser()
    ap.add_argument("--ref", default=os.environ.get("PM_REF", "origin/main"))
    ap.add_argument("--since", default=os.environ.get("PM_SINCE") or (dt.date.today() - dt.timedelta(days=90)).isoformat())
    return ap.parse_args()
# ---- end boilerplate ----

P_IC = "ecdlp-index-calculus"
P_PERF = "crypto-performance"


def leaderboard(ref, since):
    for date, commit, txt in file_history(ref, "docs/ic/leaderboard.json", since):
        try:
            d = json.loads(txt)
        except json.JSONDecodeError:
            continue
        src = f"docs/ic/leaderboard.json@{commit}"
        for regime, L in (d.get("leaders") or {}).items():
            if not isinstance(L, dict):
                continue
            emit(f"ic.leader.{regime}.ratio_rho", L.get("ratio_rho"), date=date, problem=P_IC, unit="IC/rho",
                 better="lower", label=f"IC leader {regime}: cost ratio to rho (whole pipeline)", source=src)
            emit(f"ic.leader.{regime}.s", L.get("s"), date=date, problem=P_IC, unit="S",
                 better="lower", label=f"IC leader {regime}: S = ops/sqrt(n)", source=src)
            emit(f"ic.leader.{regime}.log2_r", L.get("log2_r"), date=date, problem=P_IC, unit="bits",
                 better="higher", label=f"IC leader {regime}: curve size of best ratio", source=src)
            big = L.get("largest") or {}
            emit(f"ic.leader.{regime}.largest_log2_r", big.get("log2_r"), date=date, problem=P_IC, unit="bits",
                 better="higher", label=f"IC {regime}: largest curve priced end to end", source=src)
            emit(f"ic.leader.{regime}.largest_ratio_rho", big.get("ratio_rho"), date=date, problem=P_IC, unit="IC/rho",
                 better="lower", label=f"IC {regime}: ratio to rho on the largest curve", source=src)
            if L.get("online_speedup") is not None:
                emit(f"ic.leader.{regime}.online_speedup", L.get("online_speedup"), date=date, problem=P_IC, unit="x",
                     better="higher", label=f"IC leader {regime}: online speedup vs rho", source=src)
        emit("ic.leaderboard.board_rows", len(d.get("board") or []), date=date, problem=P_IC, unit="count",
             better="higher", label="IC leaderboard: priced board rows", source=src)
        emit("ic.leaderboard.roster", len(d.get("roster") or []), date=date, problem=P_IC, unit="count",
             better="higher", label="IC leaderboard: curves in roster", source=src)


def timeline(ref):
    txt = show(ref, "docs/ic/progress-timeline.json")
    if not txt:
        return
    d = json.loads(txt)
    for s in d.get("series", []):
        sid = s.get("id", "").replace("_", "-")
        if not sid:
            continue
        last_per_day = {}
        for p in s.get("points", []):
            if p.get("date") and p.get("value") is not None:
                last_per_day[p["date"]] = p
        for date, p in last_per_day.items():
            emit(f"ic.timeline.{sid}", p["value"], date=date, problem=P_IC, unit=p.get("unit") or "IC/rho",
                 better="lower", label=f"timeline {s.get('label', sid)} ({s.get('role','')})",
                 source=f"docs/ic/progress-timeline.json point {p.get('label','')[:40]} [{p.get('class','')}]")


def perf_history(since):
    txt = show("origin/perf-history", "docs/perf/history.json")
    if not txt:
        return
    d = json.loads(txt)
    last = {}
    for snap in d.get("snapshots", []):
        day = (snap.get("committed_at") or "")[:10]
        if day >= since:
            last[day] = snap
    for day, snap in last.items():
        src = f"perf-history docs/perf/history.json rev {str(snap.get('rev',''))[:12]}"
        emit("perf.index.overall", snap.get("overall"), date=day, problem=P_PERF, unit="x",
             better="higher", label="performance index, overall (chained callgrind Ir; <5% is noise)", source=src)
        for area, v in (snap.get("areas") or {}).items():
            emit(f"perf.index.{area}", v, date=day, problem=P_PERF, unit="x", better="higher",
                 label=f"performance index: {area}", source=src)


def benchmarks(ref, since):
    for date, commit, txt in file_history(ref, "benchmarks/README.md", since):
        n = sum(1 for l in txt.splitlines() if l.startswith("| 20"))
        emit("benchmarks.entries", n, date=date, problem=P_IC, unit="count", better="higher",
             label="benchmark studies recorded (benchmarks/README.md rows)", source=f"benchmarks/README.md@{commit}")


def e2e_runs(ref):
    n = len([p for p in ls(ref, "docs/ic/ci/runs") if p.endswith(".json")])
    emit("ic.e2e.history_runs", n, problem=P_IC, unit="count", better="higher",
         label="ic e2e gate history records", source="docs/ic/ci/runs/")


def nist_factorization(ref, since):
    path = "research/nist_skipped_curve_dissection_20261021/factorization/progress.json"
    txt = show(ref, path)
    if not txt:
        return
    data = json.loads(txt)
    for row in data.get("observations", []):
        date = row.get("date")
        if not date or date < since:
            continue
        source = f"{path}: {row.get('source', '')}"
        for field, better, unit, label in (
            ("complete_targets", "higher", "count", "NIST timeout integers completely prime-certified"),
            ("resolved_timeout_fields", "higher", "count", "NIST timeout fields resolved"),
            ("largest_residual_bits", "lower", "bits", "Remaining skipped-P256 cofactor size"),
            ("main_raw_relation_lines", "neutral", "lines", "Skipped-P256 raw relation lines; no dedup/rank claim"),
            ("filter_unique_relations", "neutral", "relations", "Native unique relations, including generated free relations"),
            ("filter_singleton_survivors", "neutral", "relations", "Native relations surviving singleton removal; no rank claim"),
        ):
            emit(f"nist.factorization.{field}", row.get(field), date=date,
                 problem="nist-skipped-curve-factorization", unit=unit, better=better,
                 label=label, source=source)


def main():
    a = args()
    leaderboard(a.ref, a.since)
    timeline(a.ref)
    perf_history(a.since)
    benchmarks(a.ref, a.since)
    e2e_runs(a.ref)
    nist_factorization(a.ref, a.since)


if __name__ == "__main__":
    main()
