# Row 48 | 20261015-p61-adjacency-pilot: recompute headline numbers from committed run metrics.
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common as C

STUDY = "research/p61_isogeny_class_20261014"
RUN = f"{STUDY}/reports/run-20261015T0436Z-adjacency-pilot"
MET = f"{RUN}/metrics.json"
m = json.load(open(os.path.join(C.REPO, MET)))
cal = {c["label"]: c for c in m["calibrations"]}
prod = cal["production window"]
proj = m["projection_from_production_window"]
class_vertices = m["field"]["class_vertices"]

full_logical_tb = class_vertices * (prod["logical_uncompressed_bytes"] / prod["vertices"]) / 1e12
full_stored_tb = class_vertices * (prod["stored_gzip_bytes"] / prod["vertices"]) / 1e12
claims = [
    {"claimed": "4,096 vertices", "computed": str(prod["vertices"]), "definition": "production window vertices", "files": [MET],
     "status": "match" if prod["vertices"] == 4096 else "mismatch"},
    {"claimed": "28,672 edges", "computed": str(prod["directed_edges"]), "definition": "production window directed_edges", "files": [MET],
     "status": "match" if prod["directed_edges"] == 28672 else "mismatch"},
    {"claimed": "471,040 kernel coefficients", "computed": str(prod["kernel_coefficients"]), "definition": "production window kernel_coefficients", "files": [MET],
     "status": "match" if prod["kernel_coefficients"] == 471040 else "mismatch"},
    {"claimed": "191.551 s generation", "computed": f"{prod['generation_elapsed_ms']/1000:.3f} s", "definition": "generation_elapsed_ms / 1000", "files": [MET],
     "status": "match" if round(prod["generation_elapsed_ms"]/1000, 3) == 191.551 else "mismatch"},
    {"claimed": "296.931 s replay", "computed": f"{prod['verification_elapsed_ms']/1000:.3f} s", "definition": "verification_elapsed_ms / 1000", "files": [MET],
     "status": "match" if round(prod["verification_elapsed_ms"]/1000, 3) == 296.931 else "mismatch"},
    {"claimed": "41,543,871 logical bytes", "computed": str(prod["logical_uncompressed_bytes"]), "definition": "logical_uncompressed_bytes", "files": [MET],
     "status": "match" if prod["logical_uncompressed_bytes"] == 41543871 else "mismatch"},
    {"claimed": "8,552,556 gzip bytes", "computed": str(prod["stored_gzip_bytes"]), "definition": "stored_gzip_bytes", "files": [MET],
     "status": "match" if prod["stored_gzip_bytes"] == 8552556 else "mismatch"},
    {"claimed": "full-class projection 3.632 TB logical", "computed": f"{full_logical_tb:.4f} TB (class_vertices {class_vertices} x logical per vertex); stored projection {proj['complete_graph_logical_bytes']} bytes = {proj['complete_graph_logical_bytes']/1e12:.4f} TB",
     "definition": "class_vertices x (production logical bytes / production vertices), decimal TB; study's own projection field also given",
     "files": [MET], "status": "match" if abs(full_logical_tb - 3.632) / 3.632 <= 0.001 else "mismatch"},
    {"claimed": "full-class projection 0.748 TB gzip", "computed": f"{full_stored_tb:.4f} TB; stored projection {proj['complete_graph_stored_bytes']/1e12:.4f} TB",
     "definition": "class_vertices x (production gzip bytes / production vertices), decimal TB",
     "files": [MET], "status": "match" if abs(full_stored_tb - 0.748) / 0.748 <= 0.001 else "mismatch"},
]
# identical / exact replay: the calibration and production records carry status pass; shard/chain sha256 are recorded
rows = [c for c in m["calibrations"]]
n_cmp = len(rows)
n_eq = sum(1 for c in rows if c.get("status") == "pass")
ident = {"claimed": True, "n_compared": n_cmp, "n_equal": n_eq, "n_differ": n_cmp - n_eq, "files": [MET],
         "status": "partial"}
# isolation and offrepo
offrepo = C.grep_lines([RUN, f"{STUDY}/README.md", f"{STUDY}/launch_adjacency_campaign.sh"], C.OFFREPO_PAT)
store_lines = [l for l in offrepo if "store:" in l]
aa = C.grep_lines([RUN], C.AA_PAT, cap=20)
iso = [f"{MET}: isolation: {m['isolation']}", f"{MET}: host: {m['host']}"] + C.grep_lines([RUN + "/REPORT.md"], C.ISO_PAT, cap=20)
row = {
    "row_no": 48,
    "slug": "20261015-p61-adjacency-pilot",
    "claims": claims,
    "identical": ident,
    "offrepo_refs": offrepo,
    "aa_lines": aa,
    "isolation_lines": iso,
    "protocol_git_date": "no PROTOCOL file in " + RUN + " (campaign-release-manifest.json and the calibration receipts are the protocol record)",
    "first_raw_git_date": C.git_first_add(MET) + " | " + C.git_first_add(f"{STUDY}/reports/run-20261014T0902Z/timings.json"),
    "script": "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/48.py",
    "notes": (f"Store references in off-repo lines: {len(store_lines)} (store:<name> check: none present, store/index.json not consulted). "
              "Identical/exact replay: the three calibration rows carry status pass; identical is marked partial because shard and "
              "chain sha256 values are recorded but no second copy is present to compare. "
              "The production window is 1 of 4096 vertices of the class (metrics.json snapshot). "
              "Host: macOS arm64 shared interactive host, unisolated (metrics.json). The REPORT summary says L0 descriptive."),
}
print(json.dumps(row, ensure_ascii=False))
