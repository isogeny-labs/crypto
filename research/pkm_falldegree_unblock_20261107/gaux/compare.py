"""G-AUX check 3 (Amendment R4 draft): the --aux-order mestre rerun of stage A's P0 isogeny draws
must equal the admitted values in results/R9/analysis/pfdA_summary.json, field by field."""
import json, sys
here = sys.argv[1]
adm = {(r["t"], r["target_index"]): r for r in json.load(open(f"{here}/../results/R9/analysis/pfdA_summary.json"))["draws"]
       if r["kind"] == "isogeny" and r["p"] == 786433 and r["m"] == 2 and r["control"] == "tower"}
A = {(r["t"], r["target_index"]): r for r in map(json.loads, open(f"{here}/A-m2-iso-p0-mestre.jsonl")) if "t" in r}
DA = {(r["t"], r["target_index"]): r for r in map(json.loads, open(f"{here}/DA-m2-iso-p0-mestre.jsonl")) if "t" in r}
rows, bad = [], 0
for k in sorted(adm):
    s, a, d = adm[k], A.get(k), DA.get(k)
    got = {"x_r": a and a["x_r"], "closure_dF": a and str(a["last_fall_degree"]), "falls": a and a["falls"],
           "ffd": a and a["first_fall_degree"], "closure_muladds": a and a["muladds"],
           "closure_max_cols": a and max(c["cols"] for c in a["closures"]),
           "D": d and d.get("solving_degree_max"), "f4_muladds": d and d.get("muladds"),
           "aux_order_method": a and a["tower"].get("aux_order_method")}
    diff = {f: [s[f], got[f]] for f in ["x_r", "closure_dF", "falls", "ffd", "closure_muladds", "closure_max_cols", "D", "f4_muladds"]
            if s[f] != got[f]}
    bad += bool(diff) or got["aux_order_method"] != "mestre"
    rows.append({"t": k[0], "target_index": k[1], "equal": not diff, "diff": diff, "aux_order_method": got["aux_order_method"]})
out = {"schema": "pfd.gaux-check/v1", "draws_admitted": len(adm), "draws_compared": len(rows), "mismatched": bad,
       "verdict": "pass" if bad == 0 and len(rows) == 16 else "FAIL", "rows": rows}
json.dump(out, open(f"{here}/gaux_check.json", "w"), indent=1)
print(out["verdict"], f"{len(rows)} draws, {bad} mismatched")
