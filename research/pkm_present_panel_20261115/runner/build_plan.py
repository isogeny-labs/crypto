#!/usr/bin/env python3
"""Build the frozen call plan of panel P (PROTOCOL.md v1, sections 3, 4, 8) from cells/ and protocol.json.

Usage:  build_plan.py [--study DIR] [--cells DIR] [--protocol FILE] [--out plan.json]
                      [--native-deadline-s 86400] [--allow-missing-curves]

Writes plan.json (one row per call, frozen order) and plan.sha256 ("<hex>  plan.json").
Order (section 3): cells in table order (m = 3 block, m = 4 block, m4t4 last); within a cell the curves in
protocol.json order; within a curve the instances in draw order (planted last); within an instance the arms in
table order, then `aa`; for each arm the native call, then the msolve call.
Symmetric arms on a (cell, curve) with B_real < m + 1 get pre-written rows with censored = "not_applicable".
Paths in the plan are relative to the directory that holds plan.json (the study directory by default).
Everything here is [pcc:unverified] bookkeeping; nothing is computed from solver output.
"""
import argparse, hashlib, json, sys
from pathlib import Path

ARM_ORDER = ["chain-tower", "tree-tower", "chain-direct", "mono-direct", "sym-tower", "sym-direct"]
SYM_ARMS = ("sym-tower", "sym-direct")
ENGINES = ("native", "msolve")
PLAN_SCHEMA = "pkm.present-panel.plan/v1"


class PlanError(Exception):
    pass


def arms_for(m):
    """Arm list of a cell with arity m, in table order (section 3 table). Symmetric arms are always listed; whether
    they run is decided per (cell, curve) by B_real >= m + 1."""
    out = []
    for a in ARM_ORDER:
        if a == "tree-tower" and m != 4:
            continue
        if a == "mono-direct" and m != 3:
            continue
        out.append(a)
    return out


def sha256_file(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()


def load_json(p):
    try:
        return json.loads(Path(p).read_text())
    except Exception as e:  # loud, never silent
        raise PlanError(f"cannot read {p}: {e}")


def cell_json_path(cells, curve, t):
    cands = [cells / f"{curve}t{t}" / "cell.json"]
    if t == 4:  # section 3: the t = 4 cell is built as cells/<curve> (slug c<k>)
        cands.append(cells / curve / "cell.json")
    for c in cands:
        if c.is_file():
            return c
    raise PlanError(f"no cell.json for curve {curve} t={t}; tried {[str(c) for c in cands]}")


def targets_path(cells, curve, t, m):
    dirs = [cells / f"{curve}t{t}"]
    if t == 4:
        dirs.append(cells / curve)
    cands = []
    for d in dirs:
        cands += [d / f"targets_m{m}" / "targets.json", d / f"m{m}" / "targets.json", d / f"targets_m{m}.json",
                  d / "targets.json"]
    for c in cands:
        if c.is_file():
            doc = load_json(c)
            if doc.get("m") == m:
                return c, doc
    raise PlanError(f"no targets.json with m={m} for curve {curve} t={t}; tried {[str(c) for c in cands]}")


def instance_order(targets):
    """Draw order, planted last (section 3). Stable for equal keys."""
    def key(t):
        planted = t.get("label") == "planted" or bool(t.get("planted"))
        di = t.get("draw_index")
        return (1 if planted else 0, di if isinstance(di, (int, float)) else 0)
    return sorted(targets, key=key)


def build(study, cells, protocol_path, native_deadline_s=86400, allow_missing=False):
    prot = load_json(protocol_path)
    known = set(ARM_ORDER) | {"aa"}
    ids = {a["id"] for a in prot["arms"]}
    if ids != known:
        raise PlanError(f"protocol.json arms {sorted(ids)} differ from the planner's {sorted(known)}")
    caps = prot["caps"]
    # cell order: protocol.json order, with the exploratory runs_last block moved to the end (stable)
    cl = prot["cells"]
    cl = [c for c in cl if not c.get("runs_last")] + [c for c in cl if c.get("runs_last")]
    rows, summary, dropped = [], [], []
    seq = 0
    for c in cl:
        cid, m, t = c["id"], c["m"], c["t"]
        csum = {"cell": cid, "m": m, "t": t, "curves": []}
        for curve in c["curves"]:
            try:
                cp = cell_json_path(cells, curve, t)
                tp, tdoc = targets_path(cells, curve, t, m)
            except PlanError as e:
                if allow_missing:
                    dropped.append({"cell": cid, "curve": curve, "reason": str(e)})
                    continue
                raise
            cdoc = load_json(cp)
            if cdoc.get("T") != t:
                raise PlanError(f"{cp}: T={cdoc.get('T')} but cell {cid} needs t={t}")
            b_real = cdoc.get("B_real")
            if not isinstance(b_real, int):
                raise PlanError(f"{cp}: B_real missing")
            sym_ok = b_real >= m + 1
            targets = instance_order(tdoc["targets"])
            csum["curves"].append({"curve": curve, "p": cdoc.get("p"), "B": cdoc.get("B"), "B_real": b_real,
                                   "sym_applicable": sym_ok, "n_instances": len(targets),
                                   "cell_json": str(cp.relative_to(study)) if cp.is_relative_to(study) else str(cp),
                                   "targets_json": str(tp.relative_to(study)) if tp.is_relative_to(study) else str(tp)})
            for tg in targets:
                name = tg["name"]
                for arm in arms_for(m) + ["aa"]:
                    src_arm = "chain-tower" if arm == "aa" else arm
                    na = arm in SYM_ARMS and not sym_ok
                    for eng in ENGINES:
                        base = f"calls/{cid}/{curve}/{name}/{arm}.{eng}"
                        sysd = f"systems/{cid}/{curve}/{name}/{src_arm}"
                        row = {
                            "seq": seq, "cell": cid, "m": m, "t": t, "curve": curve, "p": cdoc.get("p"),
                            "B": cdoc.get("B"), "B_real": b_real, "sym_applicable": sym_ok,
                            "instance": name, "label": tg["label"], "planted": tg["label"] == "planted",
                            "draw_index": tg.get("draw_index"), "algebraic_count": tg.get("algebraic_count"),
                            "real_count": tg.get("real_count"), "real_count_distinct": tg.get("real_count_distinct"),
                            "arm": arm, "engine": eng,
                            "system": None if na else sysd + (".system.json" if eng == "native" else ".ms"),
                            "out": base + (".json" if eng == "native" else ".out"),
                            "cap_wall_s": caps[eng]["wall_s"], "as_gb": caps[eng]["as_gb"],
                        }
                        for opt in ("ordered_algebraic_solutions", "algebraic_count_distinct", "predicted_fp_rational"):
                            if opt in tg:
                                row[opt] = tg[opt]
                        if eng == "native":
                            row["max_degree"] = prot["instruments"]["native"]["max_degree"]
                            row["deadline_s"] = native_deadline_s
                        if na:
                            row["censored"] = "not_applicable"
                            row["status"] = "not_applicable"
                            row["not_applicable_reason"] = f"B_real={b_real} < m+1={m + 1}"
                        rows.append(row)
                        seq += 1
        summary.append(csum)
    plan = {
        "schema": PLAN_SCHEMA, "protocol_id": prot.get("protocol_id"),
        "protocol_json_sha256": sha256_file(protocol_path),
        "root": ".", "n_rows": len(rows),
        "order": "cells in table order (m4t4 last); curves in protocol order; instances in draw order (planted last); "
                 "arms in table order then aa; native then msolve",
        "stop_rules": {"chain_tower_native_censored_fraction_gt": 0.5,
                       "action": "later cells of the same m: not_run rows", "m4t4": "last"},
        "cells": summary, "dropped_curves": dropped, "rows": rows,
    }
    return plan


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    here = Path(__file__).resolve().parent.parent
    ap.add_argument("--study", type=Path, default=here)
    ap.add_argument("--cells", type=Path)
    ap.add_argument("--protocol", type=Path)
    ap.add_argument("--out", type=Path)
    ap.add_argument("--native-deadline-s", type=float, default=86400,
                    help="--deadline passed to `solve`; non-binding by design (B4): the external timeout is the only stop")
    ap.add_argument("--allow-missing-curves", action="store_true",
                    help="drop (and record) curves without cell/targets files (M8 draw-cap exhaustion)")
    a = ap.parse_args(argv)
    study = a.study.resolve()
    cells = (a.cells or study / "cells").resolve()
    proto = (a.protocol or study / "protocol.json").resolve()
    out = (a.out or study / "plan.json").resolve()
    plan = build(study, cells, proto, a.native_deadline_s, a.allow_missing_curves)
    out.write_text(json.dumps(plan, indent=1) + "\n")
    h = sha256_file(out)
    (out.parent / "plan.sha256").write_text(f"{h}  {out.name}\n")
    print(f"plan: {plan['n_rows']} rows, {len(plan['cells'])} cells, sha256 {h}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except PlanError as e:
        print(f"build_plan: {e}", file=sys.stderr)
        sys.exit(2)
