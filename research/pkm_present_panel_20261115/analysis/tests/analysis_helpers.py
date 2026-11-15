import json, sys
from pathlib import Path
HERE = Path(__file__).resolve().parent
ANALYSIS = HERE.parent
sys.path.insert(0, str(ANALYSIS))


def mk_rows(cells, curves=("c1", "c2"), n_per=5, dec_fn=None, rc=(1, 1)):
    """cells: list of (cell, m, t, B_real).  arms_fn(cell, arm_list) not used; see callers.
    Returns the list of rows in plan order.  Each arm spec: dict arm -> f(idx, label, curve) ->
    dict(solve, extract, construction, proxy, cens) ; cens in no|timeout|not_applicable; w_lo for censored."""
    rows, seq = [], 0
    for cell, m, t, breal, spec in cells:
        for curve in curves:
            insts = [(f"d{i:06d}", "sat", i) for i in range(n_per)] + \
                    [(f"d{n_per + i:06d}", "unsat", i) for i in range(n_per)] + [("dplant", "planted", 0)]
            for name, label, idx in insts:
                for arm, f in list(spec.items()) + [("aa", spec["chain-tower"])]:
                    for eng in ("native", "msolve"):
                        d = f(idx, label, curve)
                        r = {"seq": seq, "cell": cell, "m": m, "t": t, "curve": curve, "p": 262153, "B": 2 ** t,
                             "B_real": breal, "sym_applicable": breal >= m + 1, "instance": name, "label": label,
                             "planted": label == "planted", "draw_index": None if label == "planted" else
                             (idx if label == "sat" else n_per + idx),
                             "algebraic_count": 0 if label == "unsat" else 2, "real_count": 0 if label == "unsat" else rc[0],
                             "real_count_distinct": 0 if label == "unsat" else rc[1], "arm": arm, "engine": eng,
                             "censored": d.get("cens", "no"), "status": "ok"}
                        seq += 1
                        if r["censored"] == "not_applicable":
                            rows.append(r)
                            continue
                        if eng == "native":
                            r["construction_field_multiplications"] = d.get("construction", 5)
                            if r["censored"] == "no":
                                r.update(solve_field_multiplications=d["solve"], extract_field_multiplications=d.get("extract", 0),
                                         verified_real_decompositions=(dec_fn(arm, label, name) if dec_fn else
                                                                       ([] if label == "unsat" else [[[1, 1], [2, 1], [3, -1]]])),
                                         unlifted_solutions=0, fp_rational_solutions=d.get("fp", 3))
                            else:
                                r["w_lo"] = d.get("w_lo")
                        else:
                            if r["censored"] == "no":
                                r["work_proxy"] = d["proxy"]
                            else:
                                r["work_proxy"] = None
                                r["work_proxy_lo"] = d.get("proxy_lo")
                        rows.append(r)
    return rows


def write_results(path, rows):
    path.mkdir(parents=True, exist_ok=True)
    (path / "rows.jsonl").write_text("\n".join(json.dumps(r) for r in rows) + "\n")
