#!/usr/bin/env python3
"""Paired fixed-budget runner: arm A (representatives only) against arm B
(one-hot Frobenius phases) on the SAME frozen targets.

One process, one CryptoMiniSat thread, attempts strictly sequential, ABBA
interleaving (pair i runs A,B when i is even and B,A when odd).  Every
attempt writes one JSON line.  Every unsuccessful search is kept.  Every
proposed tuple is checked on the curve (encode.verify); a rejected tuple is
blocked and the search continues inside the same CPU budget (the solver is
rebuilt with the remaining budget, because pycryptosat's time_limit is a
fresh CPU allowance on every solve() call).

This script reads manifest.json only.  It never opens the sealed witness file.
"""
import argparse
import datetime
import json
import os
import platform
import resource
import sys
import time

import pycryptosat

from paired_common import (ARM_PHASES, HERE, MANIFEST, MANIFEST_SHA,
                           abba_schedule, hx, load_manifest, sha256_file,
                           target_list, unhex, Curve)

from encode import build, decode, verify  # parent harness
from run import FORMULATIONS  # parent harness

SCHEMA = "frobenius_quotient_sat/paired_fixed_budget/receipt/v1"
SOURCES = ["../encode.py", "../run.py", "../brute_check.py",
           "../../frobenius_quotient_followup_20260926/verify_claims.py",
           "paired_common.py", "run_paired.py", "freeze_targets.py"]


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds")


def cpu_model():
    try:
        with open("/proc/cpuinfo") as fh:
            for line in fh:
                if line.startswith("model name"):
                    return line.split(":", 1)[1].strip()
    except OSError:
        pass
    return platform.processor() or "unknown"


def clock():
    return time.perf_counter(), time.process_time()


def detail(sol, cvars, evars, basis, f):
    """Per summand: representative-space element v and Frobenius phase k."""
    out = []
    for i, ci in enumerate(cvars):
        v = 0
        for j, var in enumerate(ci):
            if sol[var]:
                v ^= basis[j]
        k = 0
        if evars is not None:
            k = next(kk for kk, e in enumerate(evars[i]) if sol[e])
        out.append({"v": hx(v), "k": k})
    return out


def run_attempt(E, basis, R, phases, form, budget, assumptions=None):
    """One fresh attempt.  Returns a dict of measurements (no target info).

    ``assumptions`` is a test-only hook: a function (cvars, evars) -> literals
    passed to solve().  The production runner never sets it."""
    n = E.f.n
    final, sub, nx = FORMULATIONS[form]
    la0 = os.getloadavg()[0]
    w0, c0 = clock()
    M, cvars, evars = build(n, basis, R, final, sub, nx, phases)
    w1, c1 = clock()
    build_w, build_c = w1 - w0, c1 - c0
    load_w = load_c = solve_w = solve_c = ver_w = ver_c = 0.0
    blocks, rejected, instances = [], 0, 0
    status, found, solver_summands = None, None, None
    while True:
        remaining = budget - solve_c
        if remaining <= 0:
            status = "timeout"
            break
        a0, b0 = clock()
        solver = pycryptosat.Solver(threads=1, time_limit=remaining)
        M.load(solver)
        for cl in blocks:
            solver.add_clause(cl)
        a1, b1 = clock()
        load_w += a1 - a0
        load_c += b1 - b0
        instances += 1
        sat, sol = solver.solve(assumptions(cvars, evars) if assumptions else [])
        a2, b2 = clock()
        solve_w += a2 - a1
        solve_c += b2 - b1
        if sat is None:
            status = "timeout"
            break
        if not sat:
            status = "unsat"
            break
        xs = decode(sol, cvars, basis, evars, E.f)
        pts = verify(E, xs, R)
        a3, b3 = clock()
        ver_w += a3 - a2
        ver_c += b3 - b2
        if pts:
            status = "verified"
            found = [[hx(P[0]), hx(P[1])] for P in pts]
            solver_summands = detail(sol, cvars, evars, basis, E.f)
            break
        rejected += 1
        blocking = [v for ci in cvars for v in ci]
        if evars is not None:
            blocking += [e for ev in evars for e in ev]
        blocks.append([-v if sol[v] else v for v in blocking])
    wE, cE = clock()
    return {
        "status": status,
        "budget_cpu_s": budget,
        "build_wall_s": round(build_w, 4), "build_cpu_s": round(build_c, 4),
        "load_wall_s": round(load_w, 4), "load_cpu_s": round(load_c, 4),
        "solve_wall_s": round(solve_w, 4), "solve_cpu_s": round(solve_c, 4),
        "verify_wall_s": round(ver_w, 4), "verify_cpu_s": round(ver_c, 4),
        "total_wall_s": round(wE - w0, 4), "total_cpu_s": round(cE - c0, 4),
        "rejected_tuples": rejected,
        "solver_instances": instances,
        "found_points": found,
        "solver_summands": solver_summands,
        "size": M.size(),
        "loadavg_start": round(la0, 2),
        "loadavg_end": round(os.getloadavg()[0], 2),
        "maxrss_kb_process": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
    }


def read_receipts(path):
    lines = []
    if os.path.exists(path):
        with open(path) as fh:
            for ln in fh:
                if ln.strip():
                    lines.append(json.loads(ln))
    return lines


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--out", required=True, help="receipts JSONL (append-only)")
    ap.add_argument("--budget", type=float, default=120.0,
                    help="CPU seconds of solving per attempt")
    ap.add_argument("--forms", default="subgroup_xor,line_subgroup")
    ap.add_argument("--cpu-cap", type=float, default=4500.0,
                    help="stop at a pair boundary once total attempt CPU reaches this")
    ap.add_argument("--targets", default="",
                    help="comma-separated target ids to run (default: all in manifest order)")
    ap.add_argument("--resume", action="store_true")
    ap.add_argument("--manifest", default=MANIFEST)
    a = ap.parse_args(argv)

    msha = sha256_file(a.manifest)
    if a.manifest == MANIFEST and os.path.exists(MANIFEST_SHA):
        with open(MANIFEST_SHA) as fh:
            frozen = fh.read().split()[0]
        if frozen != msha:
            sys.exit(f"manifest changed after freeze: {msha} != {frozen}")
    manifest = load_manifest(a.manifest)
    n = manifest["curve"]["n"]
    E = Curve(n)
    basis = [unhex(b) for b in manifest["factor_space"]["basis"]]
    targets = target_list(manifest)
    if a.targets:
        keep = a.targets.split(",")
        targets = [t for t in targets if t["id"] in keep]
    forms = a.forms.split(",")
    for f in forms:
        assert f in FORMULATIONS, f

    prior = read_receipts(a.out)
    done, spent = set(), 0.0
    if prior:
        if not a.resume:
            sys.exit(f"{a.out} exists; pass --resume to append")
        hdr = prior[0]
        assert hdr["type"] == "header" and hdr["manifest_sha256"] == msha, "different manifest"
        assert hdr["budget_cpu_s"] == a.budget and hdr["forms"] == forms, "different setup"
        for r in prior:
            if r["type"] == "attempt" and r["status"] != "skipped":
                done.add((r["target_id"], r["arm"], r["form"]))
                spent += r["total_cpu_s"]
    seq = sum(1 for r in prior if r["type"] == "attempt")
    cmd = " ".join([sys.executable] + sys.argv)
    base = os.path.dirname(os.path.abspath(__file__))
    with open(a.out, "a") as out:
        def emit(obj):
            out.write(json.dumps(obj, sort_keys=True) + "\n")
            out.flush()
            os.fsync(out.fileno())

        meta = {
            "created_utc": now(), "command": cmd,
            "platform": platform.platform(), "machine": platform.machine(),
            "cpu_model": cpu_model(), "cpu_count": os.cpu_count(),
            "python": sys.version.split()[0], "pycryptosat": pycryptosat.__version__,
            "loadavg_start": os.getloadavg(),
        }
        if not prior:
            emit({"type": "header", "schema": SCHEMA, "manifest_sha256": msha,
                  "budget_cpu_s": a.budget, "forms": forms, "cpu_cap_s": a.cpu_cap,
                  "solver_threads": 1,
                  "budget_clock": "solver CPU seconds (pycryptosat time_limit)",
                  "source_sha256": {s: sha256_file(os.path.join(base, s)) for s in SOURCES},
                  "arms": {"A": "representatives only", "B": "one-hot Frobenius phases"},
                  "schedule": "formulation-major; pair i runs A,B if i even else B,A",
                  **meta})
        else:
            emit({"type": "resume", **meta})
        capped = False
        for form in forms:
            sched = abba_schedule(targets)
            for item in sched:
                tid, arm = item["target"]["id"], item["arm"]
                if item["position"] == 0 and not capped and spent >= a.cpu_cap:
                    capped = True
                if (tid, arm, form) in done:
                    continue
                rec = {"type": "attempt", "seq": seq, "pair": item["pair"],
                       "position": item["position"], "arm": arm,
                       "phases": ARM_PHASES[arm], "form": form, "n": n,
                       "set": item["target"]["set"], "target_id": tid,
                       "target": [hx(item["target"]["R"][0]), hx(item["target"]["R"][1])]}
                seq += 1
                if capped:
                    rec.update({"status": "skipped", "reason": f"cpu_cap {a.cpu_cap} s reached"})
                    emit(rec)
                    continue
                rec["started_utc"] = now()
                rec.update(run_attempt(E, basis, item["target"]["R"],
                                       ARM_PHASES[arm], form, a.budget))
                spent += rec["total_cpu_s"]
                emit(rec)
                print(f"{form:14s} pair {item['pair']:2d} {arm} {tid} {rec['status']:8s} "
                      f"cpu {rec['total_cpu_s']:7.2f}s wall {rec['total_wall_s']:7.2f}s "
                      f"cum_cpu {spent:7.1f}s", flush=True)
        emit({"type": "end", "finished_utc": now(), "total_attempt_cpu_s": round(spent, 3),
              "capped": capped})
    return 0


if __name__ == "__main__":
    sys.exit(main())
