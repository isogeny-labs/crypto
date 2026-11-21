#!/usr/bin/env python3
"""Validator data layer (follow-up study): load the re-derived raw records
written by rederive.py and map every row to its census vertex with this
file's own rules. Never reads analysis/*.py or runs/slugmap (producer output).

Mapping:
* D, Ua, Ub, S, N, ctl: vertex = the census vertex whose j-invariant equals
  j(a, b) = 1728 * 4a^3 / (4a^3 + 27b^2) mod p (computed here, and checked
  against the j recorded in census/<V>/vertices.jsonl). D rows must carry the
  vertex's own (a, b); ctl rows one of the vertex's two re-scalings.
* M: vertex = the spec's c<id>; j(a, b) must equal that vertex's j and (a, b)
  must be one of the 8 models of census/<V>/M.json (model index m kept).
* Tcr: vertex = the spec's c<id> (the source); the curve must be the crater
  model and the generator must equal census/V5/tcr/<id>.json "g".
The level is the census vertex's level and must equal the spec's L<level>.
"""
import glob
import gzip
import json
import os

import numpy as np
import pandas as pd


def jinv(a, b, p):
    a, b = a % p, b % p
    num = 4 * pow(a, 3, p) % p
    den = (num + 27 * b * b) % p
    return 1728 * num * pow(den, p - 2, p) % p


def load(study, rowsdir):
    checks = {}
    verts = {}
    for V in ("V5", "V6"):
        vs = [json.loads(l) for l in open(os.path.join(study, f"census/{V}/vertices.jsonl"))]
        verts[V] = {v["id"]: v for v in vs}
        js = {}
        bad_j = 0
        for v in vs:
            j = jinv(v["a"], v["b"], v["p"])
            if j != v["j"] % v["p"]:
                bad_j += 1
            js.setdefault(j, []).append(v["id"])
        checks[f"{V}_vertices"] = len(vs)
        checks[f"{V}_j_recompute_mismatch"] = bad_j
        checks[f"{V}_j_unique"] = all(len(x) == 1 for x in js.values())
        verts[V + "_jmap"] = {j: ids[0] for j, ids in js.items()}
        lv = {}
        for v in vs:
            lv[v["level"]] = lv.get(v["level"], 0) + 1
        checks[f"{V}_vertices_per_level"] = dict(sorted(lv.items()))
        checks[f"{V}_j0_or_1728_vertices"] = sorted(v["id"] for v in vs if v["j"] % v["p"] in (0, 1728 % v["p"]))
    mjson = {}
    for V in ("V5", "V6"):
        m = json.load(open(os.path.join(study, f"census/{V}/M.json")))
        for c in m["curves"]:
            for md in c["models"]:
                mjson[(V, c["id"], md["a"], md["b"])] = md["m"]
    tcr = {}
    for f in glob.glob(os.path.join(study, "census/V5/tcr/*.json")):
        d = json.load(open(f))
        tcr[d["id"]] = d
    usecols = ["session", "spec", "V", "fam", "lvl", "cid", "round", "warmup", "arm", "wid", "slug",
               "p", "a", "b", "gx", "gy", "r", "tseed", "tidx", "planted", "recovered", "status",
               "total_gae", "s", "unc", "solve_wall_ns", "iso", "scalar_ok"]
    frames = []
    for f in sorted(glob.glob(os.path.join(rowsdir, "*.csv.gz"))):
        df = pd.read_csv(f, usecols=usecols, dtype={"recovered": "Int64"})
        frames.append(df)
    df = pd.concat(frames, ignore_index=True)
    checks["rows_all"] = int(len(df))
    checks["rows_warmup"] = int(df["warmup"].sum())
    df = df[df["warmup"] == 0].copy()
    checks["rows_measured"] = int(len(df))
    # vertex mapping
    vert = np.full(len(df), -1, dtype=np.int64)
    model = np.full(len(df), -1, dtype=np.int64)
    bad = {"D_not_vertex_model": 0, "ctl_not_a_control": 0, "M_not_a_model": 0, "M_j_mismatch": 0,
           "tcr_not_crater": 0, "tcr_generator_mismatch": 0, "no_vertex": 0, "level_mismatch": 0}
    cache = {}
    for i, (V, fam, lvl, cid, p, a, b, gx, gy) in enumerate(zip(df["V"], df["fam"], df["lvl"], df["cid"], df["p"],
                                                                 df["a"], df["b"], df["gx"], df["gy"])):
        key = (V, fam, cid, a, b, gx, gy)
        if key in cache:
            vert[i], model[i], flag = cache[key]
            if flag:
                bad[flag] += 1
            continue
        flag = None
        m = -1
        if fam == "Tcr":
            v = cid
            vx = verts["V5"][0]
            if (a, b) != (vx["a"], vx["b"]):
                flag = "tcr_not_crater"
            elif "%d,%d" % (gx, gy) != tcr[cid]["g"]:
                flag = "tcr_generator_mismatch"
        elif fam == "M":
            v = cid
            if jinv(a, b, p) != verts[V][v]["j"] % p:
                flag = "M_j_mismatch"
            m = mjson.get((V, v, a, b), -1)
            if m < 0:
                flag = "M_not_a_model"
        else:
            v = verts[V + "_jmap"].get(jinv(a, b, p), -1)
            if v < 0:
                flag = "no_vertex"
            elif fam == "D" and (a, b) != (verts[V][v]["a"], verts[V][v]["b"]):
                flag = "D_not_vertex_model"
            elif fam == "ctl" and (a, b) not in [(c["a"], c["b"]) for c in verts[V][v]["controls"]]:
                flag = "ctl_not_a_control"
        if flag is None and v >= 0 and verts[V][v]["level"] != lvl:
            flag = "level_mismatch"
        cache[key] = (v, m, flag)
        vert[i], model[i] = v, m
        if flag:
            bad[flag] += 1
    df["vertex"] = vert
    df["model"] = model
    df["level"] = [verts[V][v]["level"] if v >= 0 else -1 for V, v in zip(df["V"], df["vertex"])]
    checks["mapping_failures"] = bad
    checks["distinct_models_per_vertex_isogenous"] = {}
    for V in ("V5", "V6"):
        sub = df[(df["V"] == V) & df["fam"].isin(["D", "Ua", "Ub", "S", "N"])]
        n = sub.groupby("vertex")["slug"].nunique()
        checks["distinct_models_per_vertex_isogenous"][V] = [int(n.min()), int(n.max())]
    return df, verts, tcr, checks


if __name__ == "__main__":
    import sys
    df, verts, tcr, checks = load(sys.argv[1], sys.argv[2])
    print(json.dumps(checks, indent=1))
    df.to_pickle(sys.argv[3])
