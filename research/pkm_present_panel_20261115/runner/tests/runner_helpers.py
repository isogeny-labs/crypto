import json, os, sys
from pathlib import Path
HERE = Path(__file__).resolve().parent
RUNNER = HERE.parent
STUDY = RUNNER.parent
sys.path.insert(0, str(RUNNER))


def write_protocol(path, cells, wall=1):
    prot = json.loads((STUDY / "protocol.json").read_text())
    prot["cells"] = cells
    prot["caps"] = {"native": {"wall_s": wall, "as_gb": 10}, "msolve": {"wall_s": wall, "as_gb": 10}}
    Path(path).write_text(json.dumps(prot))


def make_cells(root, specs, n_per_label=1, b_real=None):
    """specs: list of (curve, t, m, B_real). Layout of PROTOCOL.md section 8; targets in cells/<curve>t<t>/targets_m<m>/."""
    cells = Path(root) / "cells"
    for curve, t, m, breal in specs:
        d = cells / (f"{curve}t{t}")
        d.mkdir(parents=True, exist_ok=True)
        (d / "cell.json").write_text(json.dumps({"T": t, "B": 2 ** t, "B_real": breal, "p": 262153}))
        td = d / f"targets_m{m}"
        td.mkdir(exist_ok=True)
        tg = []
        di = 0
        # written out of draw order on purpose: planted first, then unsat, then sat
        tg.append({"name": "dplant", "label": "planted", "draw_index": None, "algebraic_count": 1, "real_count": 1,
                   "real_count_distinct": 1})
        for lab in ("unsat", "sat"):
            for i in range(n_per_label):
                tg.append({"name": f"d{di:06d}", "label": lab, "draw_index": di, "algebraic_count": 0 if lab == "unsat" else 2,
                           "real_count": 0 if lab == "unsat" else 1, "real_count_distinct": 0 if lab == "unsat" else 1})
                di += 1
        tg.sort(key=lambda x: -1 if x["draw_index"] is None else x["draw_index"])
        (td / "targets.json").write_text(json.dumps({"m": m, "targets": tg}))
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
