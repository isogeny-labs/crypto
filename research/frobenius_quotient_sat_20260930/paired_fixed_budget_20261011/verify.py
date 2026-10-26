#!/usr/bin/env python3
"""Audit a receipts file after the run.  Exit status 0 only if every check holds.

Checks (fail closed):
  manifest      header SHA-256 equals manifest.json, manifest.sha256, and a fresh
                regeneration (freeze_targets.build_artifacts)
  sources       header source hashes equal the files on disk (drift is reported)
  coverage      every (target, arm, form) appears exactly once, in ABBA order,
                as verified / unsat / timeout, or as an explicit skipped receipt
  budget        solver CPU never exceeds the budget by more than 1 s; every
                timeout used at least 90% of it
  relations     every verified attempt yields a relation that passes rank.py's
                four checks, and the solver's own (v, k) agrees with the
                orbit-derived (representative, phase)
  sealed        every UNSAT agrees with the sealed brute force (the one place the
                sealed file is read, after the run); every verified answer is on
                a target the brute force calls decomposable
"""
import argparse
import json
import os
import sys

import paired_common as pc
from rank import build, check_relation, load_attempts, relation_from_points


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--receipts", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--no-regen", action="store_true",
                    help="skip regenerating the manifest (about 6 s)")
    ap.add_argument("--manifest", default=pc.MANIFEST)
    ap.add_argument("--sealed", default=pc.SEALED)
    ap.add_argument("--manifest-sha", default=pc.MANIFEST_SHA)
    a = ap.parse_args(argv)
    problems, notes = [], []
    hdr, atts = load_attempts(a.receipts)
    base = os.path.dirname(os.path.abspath(__file__))

    # manifest
    msha = pc.sha256_file(a.manifest)
    if hdr["manifest_sha256"] != msha:
        problems.append("header manifest_sha256 differs from manifest.json")
    with open(a.manifest_sha) as fh:
        if fh.read().split()[0] != msha:
            problems.append("manifest sha256 file differs from the manifest")
    if not a.no_regen:
        import freeze_targets as ft
        mbytes, sbytes = ft.build_artifacts()
        if pc.sha256_bytes(mbytes) != msha:
            problems.append("regenerating the manifest gives different bytes")
        with open(a.sealed, "rb") as fh:
            if fh.read() != sbytes:
                problems.append("regenerating the sealed file gives different bytes")
    # sources
    drift = [s for s, h in hdr["source_sha256"].items()
             if pc.sha256_file(os.path.join(base, s)) != h]
    if drift:
        notes.append("source drift since the run: " + ", ".join(drift))

    manifest = pc.load_manifest(a.manifest)
    ctx = pc.Ctx(manifest)
    with open(a.sealed) as fh:
        sealed = json.load(fh)
    targets = pc.target_list(manifest)
    forms = hdr["forms"]

    # coverage and ABBA order
    by_key = {}
    for r in atts:
        key = (r["target_id"], r["arm"], r["form"])
        if r["status"] == "skipped" and key in by_key:
            continue
        if key in by_key and by_key[key]["status"] != "skipped":
            problems.append(f"duplicate attempt {key}")
        by_key[key] = r
    expect = []
    for f in forms:
        for item in pc.abba_schedule(targets):
            expect.append((item["target"]["id"], item["arm"], f, item["pair"], item["position"]))
    seqs = []
    for tid, arm, f, pair, pos in expect:
        r = by_key.get((tid, arm, f))
        if r is None:
            problems.append(f"missing attempt {(tid, arm, f)}")
            continue
        if (r["pair"], r["position"]) != (pair, pos):
            problems.append(f"order differs from ABBA at {(tid, arm, f)}")
        seqs.append(r["seq"])
    if seqs != sorted(seqs):
        problems.append("attempts are not in schedule order")

    # budget
    budget = hdr["budget_cpu_s"]
    for r in by_key.values():
        if r["status"] == "skipped":
            continue
        if r["solve_cpu_s"] > budget + 1.0:
            problems.append(f"budget exceeded {r['target_id']} {r['arm']} {r['form']}: {r['solve_cpu_s']}")
        if r["status"] == "timeout" and r["solve_cpu_s"] < 0.9 * budget:
            problems.append(f"early timeout {r['target_id']} {r['arm']} {r['form']}: {r['solve_cpu_s']}")

    # relations and sealed cross-check
    n_checked = n_unsat = n_bad = 0
    for r in by_key.values():
        t = sealed["targets"][r["target_id"]]["brute_force"]
        dec = t["decomposable_phases_free"] if r["arm"] == "B" else t["decomposable_reps_only"]
        R = (pc.unhex(r["target"][0]), pc.unhex(r["target"][1]))
        if r["status"] == "unsat":
            n_unsat += 1
            if dec:
                n_bad += 1
                problems.append(f"UNSAT contradicted by brute force: {r['target_id']} {r['arm']} {r['form']}")
        elif r["status"] == "verified":
            if not dec:
                n_bad += 1
                problems.append(f"verified on a target brute force calls undecomposable: {r['target_id']} {r['arm']}")
            pts = [(pc.unhex(p[0]), pc.unhex(p[1])) for p in r["found_points"]]
            try:
                rel = relation_from_points(ctx, pts)
            except ValueError as exc:
                problems.append(f"relation not buildable {r['target_id']} {r['arm']} {r['form']}: {exc}")
                continue
            chk = check_relation(ctx, rel, R, pts)
            n_checked += 1
            if not chk["all"]:
                problems.append(f"relation check failed {r['target_id']} {r['arm']} {r['form']}: {chk}")
            if len({p[0] for p in pts}) != 4:
                problems.append(f"summand x-coordinates not distinct {r['target_id']}")
            if r["arm"] == "A" and any(t_["phase"] != 0 for t_ in rel["terms"]):
                problems.append(f"arm A answer uses a Frobenius phase {r['target_id']}")
            for t_, s_ in zip(rel["terms"], r["solver_summands"]):
                v = pc.unhex(s_["v"])
                x = v
                for _ in range(s_["k"]):
                    x = ctx.E.f.sq(x)
                if hex(x) != t_["x"] or ctx.orbit.get(x) != (t_["rep"], t_["phase"]):
                    problems.append(f"solver (v,k) disagrees with orbit map {r['target_id']} {r['arm']}")
                    break
        n_bad += 0
    _, summary = build(ctx, atts)
    out = {
        "receipts": os.path.basename(a.receipts),
        "manifest_sha256": msha,
        "attempts_expected": len(expect),
        "attempts_present": len(by_key),
        "attempts_skipped": sum(1 for r in by_key.values() if r["status"] == "skipped"),
        "relations_checked": n_checked,
        "unsat_checked_against_brute_force": n_unsat,
        "rank_summary_relations_failed_check": sum(
            d["relations_failed_check"] for d in summary.values()),
        "notes": notes,
        "problems": problems,
        "pass": not problems,
    }
    with open(a.out, "w") as fh:
        json.dump(out, fh, indent=1, sort_keys=True)
        fh.write("\n")
    print(json.dumps({k: v for k, v in out.items() if k not in ("problems", "notes")}))
    for p in problems:
        print("PROBLEM:", p)
    for p in notes:
        print("NOTE:", p)
    return 0 if out["pass"] else 1


if __name__ == "__main__":
    sys.exit(main())
