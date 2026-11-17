#!/usr/bin/env python3
"""Infrastructure loss ledger (schema crypto.infra-loss/v1).

One JSON file per loss under benchmarks/losses/<YYYYMMDD>-<slug>.json.

  loss_ledger.py add --slug S --route R --cause C --what "..." --source "path#heading" ...
  loss_ledger.py report [--since YYYY-MM-DD] [--json]
  loss_ledger.py top-cause [--since YYYY-MM-DD]

Hours and USD are recorded only when a log gives the numbers; otherwise null
with derivation "not recorded". Every record is [pcc:unverified] by default.
"""
import argparse
import datetime as dt
import glob
import json
import os
import re
import sys

SCHEMA = "crypto.infra-loss/v1"
ROUTES = ("isolab-eks", "runpod", "local", "ci", "s3", "other")
CAUSES = ("spot_preemption", "balance_exhausted", "shell_time_limit", "pool_blind_spot",
          "access_denied", "container_restart", "quota", "other")
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LOSS_DIR = os.path.join(ROOT, "benchmarks", "losses")


def _num(s):
    if s is None or str(s).lower() in ("null", "none", ""):
        return None
    return float(s)


def load(loss_dir=None):
    recs = []
    for p in sorted(glob.glob(os.path.join(loss_dir or LOSS_DIR, "*.json"))):
        with open(p) as f:
            r = json.load(f)
        if r.get("schema") == SCHEMA:
            recs.append(r)
    return recs


def validate(r):
    errs = []
    if r.get("cause") not in CAUSES:
        errs.append(f"unknown cause {r.get('cause')!r}; allowed: {', '.join(CAUSES)}")
    if r.get("route") not in ROUTES:
        errs.append(f"unknown route {r.get('route')!r}; allowed: {', '.join(ROUTES)}")
    for k in ("lost_cpu_hours", "lost_usd"):
        v = r.get(k)
        if v is not None and (isinstance(v, bool) or not isinstance(v, (int, float)) or v < 0):
            errs.append(f"{k} must be a non-negative number or null")
    if not r.get("what") or "\n" in r["what"]:
        errs.append("what must be one non-empty line")
    if not re.match(r"^\d{4}-\d{2}-\d{2}", str(r.get("observed_at", ""))):
        errs.append("observed_at must start with YYYY-MM-DD")
    if not r.get("source"):
        errs.append("source (session-log path + heading) is required")
    return errs


def report(recs, since=None):
    if since:
        recs = [r for r in recs if r["observed_at"][:10] >= since]
    rows = {}
    for r in recs:
        row = rows.setdefault(r["cause"], {"cause": r["cause"], "count": 0, "lost_cpu_hours": 0.0,
                                            "lost_usd": 0.0, "unquantified": 0})
        row["count"] += 1
        if r.get("lost_cpu_hours") is None and r.get("lost_usd") is None:
            row["unquantified"] += 1
        row["lost_cpu_hours"] += r.get("lost_cpu_hours") or 0.0
        row["lost_usd"] += r.get("lost_usd") or 0.0
    return sorted(rows.values(), key=lambda x: (-x["count"], -x["lost_usd"], x["cause"]))


def top_cause(rows):
    """Most losses; ties broken by lost USD, then lost hours."""
    if not rows:
        return None
    return sorted(rows, key=lambda x: (-x["count"], -x["lost_usd"], -x["lost_cpu_hours"], x["cause"]))[0]


def _table(rows):
    out = [f"{'cause':<20}{'count':>6}{'lost cpu-h':>12}{'lost USD':>10}{'unquantified':>14}"]
    for r in rows:
        out.append(f"{r['cause']:<20}{r['count']:>6}{r['lost_cpu_hours']:>12.1f}"
                   f"{r['lost_usd']:>10.2f}{r['unquantified']:>14}")
    out.append(f"{'total':<20}{sum(r['count'] for r in rows):>6}"
               f"{sum(r['lost_cpu_hours'] for r in rows):>12.1f}{sum(r['lost_usd'] for r in rows):>10.2f}"
               f"{sum(r['unquantified'] for r in rows):>14}")
    return "\n".join(out)


def cmd_add(a, loss_dir=None):
    day = (a.observed_at or dt.date.today().isoformat())
    rec = {
        "schema": SCHEMA,
        "id": f"{day.replace('-', '')[:8]}-{a.slug}",
        "observed_at": day,
        "route": a.route, "cause": a.cause, "what": a.what,
        "lost_cpu_hours": _num(a.lost_cpu_hours),
        "lost_cpu_hours_derivation": a.hours_derivation or ("not recorded" if _num(a.lost_cpu_hours) is None else ""),
        "lost_usd": _num(a.lost_usd),
        "lost_usd_derivation": a.usd_derivation or ("not recorded" if _num(a.lost_usd) is None else ""),
        "relaunched": a.relaunched, "relaunch_ref": a.relaunch_ref,
        "source": a.source, "pcc": a.pcc,
    }
    errs = validate(rec)
    if errs:
        print("refused: " + "; ".join(errs), file=sys.stderr)
        return 2
    d = loss_dir or LOSS_DIR
    os.makedirs(d, exist_ok=True)
    path = os.path.join(d, rec["id"] + ".json")
    if os.path.exists(path):
        print(f"refused: {path} exists", file=sys.stderr)
        return 2
    with open(path, "w") as f:
        json.dump(rec, f, indent=2)
        f.write("\n")
    print(path)
    return 0


def main(argv=None, loss_dir=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("add")
    p.add_argument("--slug", required=True)
    p.add_argument("--observed-at", help="YYYY-MM-DD[THH:MMZ] (default today)")
    p.add_argument("--route", required=True)
    p.add_argument("--cause", required=True)
    p.add_argument("--what", required=True)
    p.add_argument("--lost-cpu-hours")
    p.add_argument("--hours-derivation")
    p.add_argument("--lost-usd")
    p.add_argument("--usd-derivation")
    p.add_argument("--relaunched", action="store_true")
    p.add_argument("--relaunch-ref")
    p.add_argument("--source", required=True)
    p.add_argument("--pcc", default="[pcc:unverified]")
    p = sub.add_parser("report")
    p.add_argument("--since")
    p.add_argument("--json", action="store_true")
    p = sub.add_parser("top-cause")
    p.add_argument("--since")
    a = ap.parse_args(argv)
    if a.cmd == "add":
        return cmd_add(a, loss_dir)
    rows = report(load(loss_dir), a.since)
    if a.cmd == "report":
        print(json.dumps(rows, indent=2) if a.json else _table(rows))
        return 0
    t = top_cause(rows)
    print(f"{t['cause']}\tcount={t['count']}\tlost_cpu_hours={t['lost_cpu_hours']:.1f}\t"
          f"lost_usd={t['lost_usd']:.2f}" if t else "none")
    return 0


if __name__ == "__main__":
    sys.exit(main())
