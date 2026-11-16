"""Inject final pilot summaries into ideas/alg_geom.json (raw numbers from RESULTS_*.json)."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
J = os.path.join(HERE, "..", "alg_geom.json")
doc = json.load(open(J))

def load(fn):
    p = os.path.join(HERE, fn)
    return json.load(open(p)) if os.path.exists(p) else []

pres = load("RESULTS_presentations.json")
rr = load("RESULTS_rr_only.json")
digit = load("RESULTS_digit_rr.json")
ng = load("RESULTS_nongalois.json")
ht = load("RESULTS_honda_tate.json")

def cell_rows():
    rows = []
    for r in pres:
        rows.append({"n": r["n"], "m": r["m"], "l": r["l"], "want": r["want"], "seed": r["seed"],
                     "xonly": {"maxdeg": r["xonly"]["maxdeg"], "time_s": r["xonly"]["time"], "nvars": r["xonly"]["nvars"]},
                     "rr": {"maxdeg": r["rr"]["maxdeg"], "time_s": r["rr"]["time"], "status": r["rr"]["status"], "nvars": r["rr"]["nvars"]},
                     "xonly_nofield": {"maxdeg": r["xonly_nofield"]["maxdeg"], "time_s": r["xonly_nofield"]["time"],
                                       "status": r["xonly_nofield"]["status"], "gb_len": r["xonly_nofield"]["gb_len"]}})
    for r in rr:
        rows.append({"n": r["n"], "m": r["m"], "l": r["l"], "want": r["want"], "seed": 1, "threads": 3,
                     "rr": {"maxdeg": r["maxdeg"], "time_s": r["time"], "status": r["status"], "nvars": r["nvars"], "gb_len": r["gb_len"]}})
    return rows

pilot = {
    "binary_presentations_m3": cell_rows(),
    "digit_presentations_m2": [{"p": r["p"], "s": r["s"], "want": r["want"],
                                "rr_collinear": {"maxdeg": r["rr"]["maxdeg"], "time_s": r["rr"]["time"], "status": r["rr"]["status"]},
                                "semaev_S3": {"maxdeg": r["semaev"]["maxdeg"], "time_s": r["semaev"]["time"], "status": r["semaev"]["status"]}} for r in digit],
    "nongalois_filter": ng,
    "honda_tate_hasse": ht,
    "lmfdb_av_fq_isog_census": {"query": "g=2 classes over q in {61,127} containing an elliptic factor E with #E > 4 sqrt(q)+5 (all 31 resp. 45 elliptic classes per q); for each E: number of surfaces containing E, number with #E^2 | #A, number of those equal to E x E, number simple",
                                 "result": "76/76 rows: n_with_r2 = 1 = n_with_r2_and_ExE, n_simple = 0 (the only surface with a rational r-torsion plane is E x E); q=251,509,1021 not in the table"},
}
doc["pilot_results"] = pilot
json.dump(doc, open(J, "w"), indent=1)
print("patched", J, {k: (len(v) if isinstance(v, list) else 'ok') for k, v in pilot.items()})
