#!/usr/bin/env python3
"""Validator: runs.json is an exact projection of rows.jsonl; the grid matches PROTOCOL.md.
usage: projection_grid.py <study dir> <out json>"""
import json, sys, os, hashlib, collections, itertools
D, OUT = sys.argv[1], sys.argv[2]
H = os.path.join(D, "results", "holdout")
raw = open(os.path.join(H, "rows.jsonl"), "rb").read()
rows = [json.loads(l) for l in raw.decode().splitlines()]
rj = json.load(open(os.path.join(H, "runs.json")))
res = {}
res["rows_sha256"] = hashlib.sha256(raw).hexdigest()
res["runs_json_sha256"] = hashlib.sha256(open(os.path.join(H, "runs.json"), "rb").read()).hexdigest()
res["runs_source_sha_matches"] = rj["source_sha256"] == res["rows_sha256"]
res["runs_commit"] = rj["commit"]; res["runs_bin"] = rj["bin_sha256"]
fields = rj["fields"]
res["fields"] = fields
proj = [[r.get(k) if k != "g" else (r.get("g") if r["arm"] == "ic4" else None) for k in fields] for r in rows]
res["n_rows"] = len(rows); res["n_runs_json"] = len(rj["rows"])
res["exact_projection_in_order"] = proj == rj["rows"]
# type-exact comparison too (1.0 vs 1, etc.)
res["exact_projection_json_text"] = json.dumps(proj) == json.dumps(rj["rows"])
diff = [i for i, (a, b) in enumerate(zip(proj, rj["rows"])) if a != b]
res["first_diffs"] = diff[:5]
res["row_commit_set"] = sorted({r["commit"] for r in rows})
res["row_bin_set"] = sorted({r["bin_sha256"] for r in rows})
res["runs_json_keys"] = sorted(rj.keys())
# grid
cells = [(22, 1024), (24, 1024), (24, 4096), (24, 16384), (25, 4096), (25, 16384)]
expect = collections.Counter()
for (pb, T), idx, seed in itertools.product(cells, range(11), [11, 12]):
    expect[("ic4", pb, T, idx, seed, 1.0, 2.0)] += 1
    for k in [0.5, 0.7, 1.0]:
        for a in ["rho", "rho2", "rho2x", "rhoseq"]:
            expect[(a, pb, T, idx, seed, k, None)] += 1
    for mf in [0.7, 1.0, 1.4]:
        expect[("bsgs2", pb, T, idx, seed, mf, None)] += 1
got = collections.Counter((r["arm"], r["pb"], r["T"], r["idx"], r["seed"], r["param"], r.get("g") if r["arm"] == "ic4" else None) for r in rows)
res["grid_exact"] = got == expect
res["grid_missing"] = [list(k) for k in (expect - got)][:10]
res["grid_extra"] = [list(k) for k in (got - expect)][:10]
# argv consistency and instance binding
inst = {}
for fn in os.listdir(os.path.join(D, "instances")):
    if fn.startswith("W"):
        d = json.load(open(os.path.join(D, "instances", fn)))
        inst[fn] = (int(d["field"]["modulus"]), int(d["subgroup_order"]))
bad_argv = []
for i, r in enumerate(rows):
    a = r["argv"]
    want = [r["arm"], f"W{r['pb']}-{r['idx']}.json", str(r["T"]), str(r["seed"]), str(r["param"])] + ([str(r["g"])] if r["arm"] == "ic4" else [])
    got_a = [a[0], os.path.basename(a[1])] + a[2:]
    if got_a != want or r["targets"] != r["T"] or (r["p"], r["n"]) != inst[os.path.basename(a[1])] or r["seed"] != int(a[3]):
        bad_argv.append(i)
res["argv_or_instance_mismatch"] = bad_argv[:10]; res["n_argv_mismatch"] = len(bad_argv)
res["errors_in_rows"] = sum("error" in r for r in rows)
res["unverified_rows"] = sum(r.get("ok") != r.get("targets") for r in rows)
res["total_targets"] = sum(r["targets"] for r in rows)
json.dump(res, open(OUT, "w"), indent=1)
print(json.dumps(res, indent=1))
