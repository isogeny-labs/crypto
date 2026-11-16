"""Verify the committed receipts and recompute every reported number.

    python3 verify.py        # exit 0 only if every check passes; writes results/verify_report.json

Checks
 1. receipts parse; no child ended in an unclassified exception; manifest source hashes match files on disk
 2. summary.json equals a fresh recomputation from the receipts (analysis.py)
 3. headline T_core / T_solver totals recomputed by separate straight-line code
 4. every completed W1 cell: input hash, equation hash, algebraic roots, group verification equal the FES
    receipt of the same repetition AND the archived m83 receipt of the same cell (m83 raw archive, in memory)
 5. stage identity: sum of measured stages <= child-internal wall <= parent wall
 6. relation rows: every natural row re-reduced with independent elimination, claimed ranks match
 7. RESULTS.md == render(RESULTS.template.md, summary); tables.md matches; figures built from this summary
"""
import hashlib
import json
from pathlib import Path
import statistics
import sys

import analysis
import tcommon as tc

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[3]
failures, notes = [], []


def check(cond, msg):
    if not cond:
        failures.append(msg)


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def indep_rank(rows, p):
    """Independent rank over Z/p by dense Gauss-Jordan (separate code path from tcommon.rank_add)."""
    m = [[x % p for x in r] for r in rows]
    rank, ncols = 0, len(m[0]) if m else 0
    for col in range(ncols):
        piv = next((i for i in range(rank, len(m)) if m[i][col]), None)
        if piv is None:
            continue
        m[rank], m[piv] = m[piv], m[rank]
        inv = pow(m[rank][col], -1, p)
        m[rank] = [x * inv % p for x in m[rank]]
        for i in range(len(m)):
            if i != rank and m[i][col]:
                f = m[i][col]
                m[i] = [(a - f * b) % p for a, b in zip(m[i], m[rank])]
        rank += 1
    return rank


def archive_name(c):
    s, seed, i, e = c["payload_bits"], c["seed"], c["index"], c["engine"]
    if c["workload"] == "W1a":
        return (f"extension_20260927/quotient_s2/s2-seed{seed}-i{i}-{e}.json" if e == "sat-cms-xor"
                else f"run_20260927/fixed-s{seed}-i{i}-{e}.json")
    return f"extension_20260927/plain_subspace/s{s}-seed{seed}-i{i}-{e}.json"


def main():
    w1_path, w2_path = HERE / "results/w1/receipts.jsonl", HERE / "results/w2/receipts.jsonl"
    manifest, children = analysis.load_receipts(w1_path)
    # 1 ---------------------------------------------------------------
    check(manifest["schema"] == "solver_total_cost/w1/v1", "w1 manifest schema")
    statuses = [analysis.status_of(c) for c in children]
    check(all(s in ("complete", "timeout", "setup_failed", "oom") for s in statuses),
          f"unexpected child statuses: {sorted(set(statuses) - {'complete', 'timeout', 'setup_failed', 'oom'})}")
    check(len(children) == 360, f"expected 360 W1 children, found {len(children)}")
    for name, digest in manifest["source_sha256"].items():
        p = REPO / name
        check(p.exists() and sha(p) == digest, f"source hash mismatch (edited after the run?): {name}")
    # 2 ---------------------------------------------------------------
    fresh = analysis.build_summary(w1_path, w2_path if w2_path.exists() else None)
    committed = json.loads((HERE / "results/summary.json").read_text())
    check(json.dumps(fresh, sort_keys=True) == json.dumps(committed, sort_keys=True),
          "results/summary.json differs from a fresh recomputation")
    # 3 ---------------------------------------------------------------
    for engine in analysis.ENGINES:
        per_rep_core, per_rep_solver = [], []
        for rep in sorted({c["rep"] for c in children}):
            core = solver = 0.0
            for c in children:
                if c["rep"] != rep or c["workload"] != "W1a" or c["engine"] != engine:
                    continue
                st = c["result"]["stages"]
                core += sum(st.get(k, {}).get("cpu_s", 0.0) for k in
                            ("fb_setup", "eq_build", "solver_setup", "solve", "model_extraction",
                             "lifting", "verification", "rank_update"))
                solver += sum(st.get(k, {}).get("cpu_s", 0.0) for k in ("solver_setup", "solve", "model_extraction"))
            per_rep_core.append(core)
            per_rep_solver.append(solver)
        got = committed["w1"]["groups"]["W1a"]["engines"][engine]["median"]
        check(abs(statistics.median(per_rep_core) - got["cpu_s.T_A"]) < 1e-9, f"T_core mismatch {engine}")
        check(abs(statistics.median(per_rep_solver) - got["cpu_s.T_solver"]) < 1e-9, f"T_solver mismatch {engine}")
    # 4 ---------------------------------------------------------------
    files = analysis.m83_archive_files()
    fes = {(c["rep"], c["workload"], c["payload_bits"], c["seed"], c["index"]): c["result"]
           for c in children if c["engine"] == "fes" and c["result"]}
    compared_fes = compared_m83 = status_diff = 0
    diffs = []
    for c in children:
        r = c["result"]
        orig = analysis.m83_receipt(files, archive_name(c))
        if r["status"] != orig["status"]:
            status_diff += 1
            diffs.append({"cell": f"{c['workload']}-r{c['rep']}-s{c['payload_bits']}-{c['seed']}-i{c['index']}-{c['engine']}",
                          "this_run": r["status"], "m83": orig["status"]})
        if r["status"] == "setup_failed":
            check(orig["status"] == "setup_failed", f"setup_failed not reproduced for {c['key']}")
            continue
        if r["status"] != "complete":
            continue
        ref = fes.get((c["rep"], c["workload"], c["payload_bits"], c["seed"], c["index"]))
        check(ref is not None and ref["status"] == "complete", f"no FES reference for {c['key']}")
        for key in ("input_sha256", "equation_sha256", "algebraic_roots", "group_verification"):
            check(r[key] == ref[key], f"{key} differs from FES receipt for {c['key']}")
        compared_fes += 1
        if orig["status"] == "complete":
            for key in ("input_sha256", "equation_sha256", "algebraic_roots", "group_verification"):
                check(r[key] == orig[key], f"{key} differs from archived m83 receipt for {c['key']}")
            compared_m83 += 1
    notes.append({"cells_checked_against_fes": compared_fes, "cells_checked_against_m83_archive": compared_m83,
                  "cells_whose_status_differs_from_m83": status_diff, "status_differences": diffs})
    # 5 ---------------------------------------------------------------
    for c in children:
        r = c["result"]
        total = sum(v["wall_s"] for v in r["stages"].values())
        check(total <= r["child_main_wall_s"] + 1e-3 and r["child_main_wall_s"] <= c["parent_wall_s"] + 1e-3,
              f"stage identity broken for {c['key']}: {total} / {r['child_main_wall_s']} / {c['parent_wall_s']}")
    # 6 ---------------------------------------------------------------
    for path, wl in ((w1_path, "w1"), (w2_path, "w2")):
        if not path.exists():
            continue
        _, kids = analysis.load_receipts(path)
        by_key = {}
        for c in kids:
            r = c["result"]
            if not r or r.get("status") != "complete":
                continue
            rel = r["relation"]
            rows = [x["row"] for x in rel["relation_rows"] if x["independent_of_natural_matrix"]]
            if rel["new_independent_natural"]:
                by_key.setdefault((c.get("rep"), c["engine"], c.get("seed"), c.get("payload_bits")), []).append(
                    (c["index"], rows, rel["rank_after"], rel["modulus_r"]))
        for key, entries in by_key.items():
            acc = []
            for index, rows, rank_after, p in sorted(entries):
                acc += rows
                check(indep_rank(acc, p) == rank_after, f"rank mismatch {wl} {key} at index {index}")
    # 7 ---------------------------------------------------------------
    template = HERE / "RESULTS.template.md"
    if template.exists():
        check((HERE / "RESULTS.md").read_text() == analysis.render(template.read_text(), committed),
              "RESULTS.md is not the rendering of RESULTS.template.md from summary.json")
    if template.exists():      # hand-typed counts in the prose must match what this script computed
        text = (HERE / "RESULTS.md").read_text()
        check(f"({notes[0]['cells_checked_against_fes']} cells)" in text, "FES cross-check count in RESULTS.md is stale")
        check(f"({notes[0]['cells_checked_against_m83_archive']} cells)" in text, "m83 cross-check count in RESULTS.md is stale")
        check(f"({notes[0]['cells_whose_status_differs_from_m83']} cells over 3 repetitions" in text,
              "m83 status-difference count in RESULTS.md is stale")
    tabs = analysis.tables(committed)
    check((HERE / "results/tables.md").read_text() ==
          "\n\n".join(f"<!-- {k} -->\n{v}" for k, v in tabs.items()) + "\n", "results/tables.md stale")
    fm = HERE / "figures/figures_manifest.json"
    if fm.exists():
        check(json.loads(fm.read_text())["summary_sha256"] == sha(HERE / "results/summary.json"),
              "figures were built from a different summary.json")
    report = {"ok": not failures, "failures": failures, "notes": notes,
              "children": len(children), "w2_present": w2_path.exists()}
    (HERE / "results/verify_report.json").write_text(json.dumps(report, indent=1, sort_keys=True) + "\n")
    print(json.dumps({k: report[k] for k in ("ok", "failures", "children", "w2_present")}, indent=1))
    print(json.dumps(notes[0] | {"status_differences": len(notes[0]["status_differences"])}))
    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main())
