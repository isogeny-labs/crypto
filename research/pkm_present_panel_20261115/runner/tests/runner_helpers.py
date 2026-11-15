import json, os, sys
from pathlib import Path
HERE = Path(__file__).resolve().parent
RUNNER = HERE.parent
STUDY = RUNNER.parent
sys.path.insert(0, str(RUNNER))


def write_protocol(path, cells, wall=1, m4wall=None):
    prot = json.loads((STUDY / "protocol.json").read_text())
    prot["cells"] = cells
    w4 = wall if m4wall is None else m4wall
    prot["caps"] = {"m3": {"native": {"wall_s": wall, "as_gb": 10}, "msolve": {"wall_s": wall, "as_gb": 10}},
                    "m4": {"native": {"wall_s": w4, "as_gb": 10}, "msolve": {"wall_s": w4, "as_gb": 10}},
                    "native_deadline_s": 86400, "retries": 0}
    Path(path).write_text(json.dumps(prot))


def make_cells(root, specs, n_per_label=1, legacy=False):
    """specs: list of (curve, t, m, B_real).  Layout of the instrument (cell --nested-ts) and PROTOCOL.md section 8:
    cells/<curve>/cell.json (pinned t = the largest t of the curve), cells/<curve>/nested_t<t>/cell.json,
    cells/<curve>/cell.present.json, targets in cells/<curve>t<t>/m<M>/targets.json."""
    cells = Path(root) / "cells"
    by_curve = {}
    for curve, t, m, breal in specs:
        by_curve.setdefault(curve, {})[t] = breal
    for curve, ts in by_curve.items():
        pinned = max(ts)
        entries = []
        for t, breal in sorted(ts.items(), reverse=True):
            d = cells / curve if t == pinned else cells / curve / f"nested_t{t}"
            if legacy:
                d = cells / f"{curve}t{t}"
            d.mkdir(parents=True, exist_ok=True)
            (d / "cell.json").write_text(json.dumps({"T": t, "B": 2 ** t, "B_real": breal, "p": 262153}))
            entries.append({"t": t, "path": ("cell.json" if t == pinned else f"nested_t{t}/cell.json"), "B_real": breal})
        if not legacy:
            (cells / curve / "cell.present.json").write_text(json.dumps({"cells": entries}))
    for curve, t, m, breal in specs:
        td = cells / f"{curve}t{t}" / f"m{m}"
        td.mkdir(parents=True, exist_ok=True)
        tg, di = [], 0
        tg.append({"name": "dplant", "label": "planted", "draw_index": None, "algebraic_count": 1, "real_count": 1,
                   "real_count_distinct": 1, "planted_abscissae": [1, 2, 3]})
        for lab in ("unsat", "sat"):
            for i in range(n_per_label):
                tg.append({"name": f"d{di:06d}", "label": lab, "draw_index": di, "algebraic_count": 0 if lab == "unsat" else 2,
                           "real_count": 0 if lab == "unsat" else 1, "real_count_distinct": 0 if lab == "unsat" else 1})
                di += 1
        tg.sort(key=lambda x: -1 if x["draw_index"] is None else x["draw_index"])
        (td / "targets.json").write_text(json.dumps({"m": m, "b_real": breal, "sym_applicable": breal >= m + 1,
                                                     "slug": f"{curve}t{t}", "targets": tg}))
    return cells


def make_systems(root, plan, fake_for=lambda row: {}):
    root = Path(root)
    for r in plan["rows"]:
        if not r.get("system") or r["arm"] == "aa":  # aa re-uses the chain-tower files
            continue
        p = root / r["system"]
        p.parent.mkdir(parents=True, exist_ok=True)
        fk = fake_for(r)
        if r["engine"] == "native":
            p.write_text(json.dumps({"construction_field_multiplications": 5, "nterms_total": 9, "max_total_degree": 4,
                                     "solution_semantics": {"ordered": True}, "nvars": 7, "nequations": 8, "fake": fk}))
        else:
            p.write_text(f"# stub ms\nsleep={fk.get('msleep', 0)}\nwork={fk.get('mwork', 10)}\n" if fk.get("msleep") else
                         f"# stub ms\nwork={fk.get('mwork', 10)}\n")
