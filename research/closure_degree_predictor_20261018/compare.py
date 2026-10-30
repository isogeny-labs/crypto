"""Compare the frozen predictor with the 148 committed native closure draws.

Reads research/dreg_closure_*/runs/*.jsonl of this repository.  Refuses to run
if predictor.py differs from FROZEN.sha256 (the rule is frozen before the
comparison).  Writes table.md, table.csv, results.json.
"""
from __future__ import annotations
import hashlib, json, sys
from collections import Counter, defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE))
import predictor as P

STUDIES = ["dreg_closure_mutantxl_20261016", "dreg_closure_scale_20261016",
           "dreg_closure_driver_20261016", "dreg_closure_specialise_20261017"]


def check_frozen():
    want = (HERE / "FROZEN.sha256").read_text().split()[0]
    got = hashlib.sha256((HERE / "predictor.py").read_bytes()).hexdigest()
    if want != got:
        sys.exit(f"predictor.py changed after freeze: {got} != {want}")
    return got


def load_native():
    """Native = unsatisfiable chained draws (no 'fixed', no solutions); same
    selection as ml-cryptanalysis csd/closure_rows.py.  Label: exact closure
    degree, or '>4' when degree 4 was built, saturated and did not refute."""
    out = []
    for st in STUDIES:
        for f in sorted((ROOT / "research" / st / "runs").glob("*.jsonl")):
            for line in f.read_text().splitlines():
                if not line.strip():
                    continue
                r = json.loads(line)
                if r.get("fixed"):
                    continue
                if (r.get("committed_outcome") or {}).get("kind") == "satisfiable" or "solutions" in r:
                    continue
                cd = r.get("closure_degree")
                scan = r.get("scan") or []
                if cd is None:
                    last = scan[-1] if scan else None
                    if last and last["degree"] >= 4 and last["saturated"] and not last["closure_refuted"]:
                        cd = ">4"
                    else:
                        continue
                out.append(dict(m=int(r["m"]), n=int(r["n"]), ell=int(r["ell"]),
                                N=int(r["n_vars"]), obs=cd, file=f"{st}/runs/{f.name}",
                                draw=int(r["draw"])))
    return out


def rank(v):
    idx = sorted(range(len(v)), key=lambda i: v[i])
    r = [0.0] * len(v)
    i = 0
    while i < len(idx):
        j = i
        while j + 1 < len(idx) and v[idx[j + 1]] == v[idx[i]]:
            j += 1
        for k in range(i, j + 1):
            r[idx[k]] = (i + j) / 2 + 1
        i = j + 1
    return r


def spearman(a, b):
    ra, rb = rank(a), rank(b)
    n = len(a)
    ma, mb = sum(ra) / n, sum(rb) / n
    num = sum((x - ma) * (y - mb) for x, y in zip(ra, rb))
    da = sum((x - ma) ** 2 for x in ra) ** .5
    db = sum((y - mb) ** 2 for y in rb) ** .5
    return num / (da * db) if da and db else float("nan")


def perm_p(a, b, rho, reps=20000, seed=20261018):
    import random
    rng = random.Random(seed)
    b = list(b)
    c = 0
    for _ in range(reps):
        rng.shuffle(b)
        r = spearman(a, b)
        if r == r and abs(r) >= abs(rho) - 1e-12:
            c += 1
    return (c + 1) / (reps + 1)


def numeric(o):
    return 5 if o == ">4" else int(o)


def hit(pred, o):
    return pred >= 5 if o == ">4" else pred == int(o)


def main():
    sha = check_frozen()
    rows = load_native()
    assert len(rows) == 148, len(rows)
    cells = defaultdict(list)
    for r in rows:
        cells[(r["m"], r["ell"], r["n"])].append(r)
    assert len(cells) == 37, len(cells)
    arms = {"multi": P.predict_multi, "semireg": P.predict_semiregular,
            "const4": lambda m, l, n: 4}
    table = []
    for (m, ell, n), rs in sorted(cells.items()):
        N = rs[0]["N"]
        assert N == m * ell + (m - 2) * n
        cnt = Counter(r["obs"] for r in rs)
        pred = {k: f(m, ell, n) for k, f in arms.items()}
        fss = P.fss_bound(m, ell, n)
        table.append(dict(m=m, ell=ell, n=n, N=N, counts=dict(sorted(cnt.items())),
                          pred=pred, fss=fss, draws=len(rs),
                          hits={k: sum(c for o, c in cnt.items() if hit(pred[k], o)) for k in arms}))
    res = {"predictor_sha256": sha, "draws": 148, "cells": 37, "arms": {}}
    for k in arms:
        h = sum(t["hits"][k] for t in table)
        # draw-level rank correlation (observed numeric, '>4' -> 5)
        pa = [t["pred"][k] for t in table for o, c in t["counts"].items() for _ in range(c)]
        ob = [numeric(o) for t in table for o, c in t["counts"].items() for _ in range(c)]
        rho = spearman(pa, ob)
        # cell-level: modal observed vs prediction; mean observed
        cm = [sum(numeric(o) * c for o, c in t["counts"].items()) / t["draws"] for t in table]
        cp = [t["pred"][k] for t in table]
        rho_c = spearman(cp, cm)
        modal_hits = sum(1 for t in table if hit(t["pred"][k], max(t["counts"], key=t["counts"].get)))
        rise_cells = [t for t in table if ">4" in t["counts"]]
        res["arms"][k] = dict(
            exact_match_draws=h, exact_match_rate=h / 148,
            spearman_draw=rho, spearman_draw_perm_p=perm_p(pa, ob, rho) if rho == rho else None,
            spearman_cell_mean=rho_c, modal_cell_matches=modal_hits,
            predicted_range=[min(cp), max(cp)],
            rising_cells_predicted=[(t["m"], t["ell"], t["n"], t["pred"][k]) for t in rise_cells])
    # ell = 7 rise: label-wise separation by the predicted degree
    def sep(k):
        pos = [t["pred"][k] for t in table if t["ell"] >= 7]
        neg = [t["pred"][k] for t in table if t["ell"] < 7]
        return dict(min_pred_ell_ge7=min(pos), max_pred_ell_lt7=max(neg),
                    separates=min(pos) > max(neg))
    res["ell7_rise"] = {k: sep(k) for k in ("multi", "semireg")}
    # m = 3 profile: predicted degree against ell at the observed n
    prof = {}
    for t in table:
        if t["m"] == 3:
            prof.setdefault(t["ell"], []).append((t["n"], t["pred"]["multi"], t["pred"]["semireg"]))
    res["m3_prediction_by_ell"] = {str(k): v for k, v in sorted(prof.items())}
    res["fss_applicable_cells"] = sum(1 for t in table if t["fss"] is not None)
    (HERE / "results.json").write_text(json.dumps(res, indent=1, default=str))
    with open(HERE / "table.csv", "w") as f:
        f.write("m,ell,n,N,observed_counts,pred_multi,pred_semireg,pred_const4,fss,hits_multi,hits_semireg,draws\n")
        for t in table:
            oc = ";".join(f"{o}x{c}" for o, c in t["counts"].items())
            f.write(f"{t['m']},{t['ell']},{t['n']},{t['N']},{oc},{t['pred']['multi']},{t['pred']['semireg']},4,{t['fss'] if t['fss'] is not None else 'NA'},{t['hits']['multi']},{t['hits']['semireg']},{t['draws']}\n")
    L = ["| cell (m,ell,n) | N | observed (count) | multi | semireg | FSS | hits multi/semireg/const4 |", "|:--|--:|:--|--:|--:|:--|:--|"]
    for t in table:
        oc = ", ".join(f"{o}x{c}" for o, c in t["counts"].items())
        mk = lambda v: ">=15" if v > P.DMAX else str(v)
        L.append(f"| ({t['m']},{t['ell']},{t['n']}) | {t['N']} | {oc} | {mk(t['pred']['multi'])} | {mk(t['pred']['semireg'])} | {t['fss'] if t['fss'] is not None else 'n/a'} | {t['hits']['multi']}/{t['hits']['semireg']}/{t['hits']['const4']} of {t['draws']} |")
    (HERE / "table.md").write_text("\n".join(L) + "\n")
    print("\n".join(L))
    print(json.dumps(res, indent=1, default=str))


if __name__ == "__main__":
    main()
