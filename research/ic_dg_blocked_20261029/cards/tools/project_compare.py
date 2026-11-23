"""Prints a fixed projection of every compare-*.json in an analysis directory. Reads and prints; computes nothing but equality of the wall blocks across weights.
usage: project_compare.py ANALYSIS_DIR KIND   (KIND is wall or counted)
For KIND counted, no field of any `wall` block is printed. Files are listed by pair name, then weight 0, 0.1, 1. The full JSON of every comparison stays in ANALYSIS_DIR."""
import glob, json, os, sys
an, kind = sys.argv[1:3]
if kind not in ("wall", "counted"):
    print("STOP: KIND is not wall or counted: " + kind)
    sys.exit(1)
def order_key(p):
    base = os.path.basename(p)[len("compare-"):-len(".json")]
    pair, w = base.rsplit("-w", 1)
    return (pair, float(w))


files = sorted(glob.glob(os.path.join(an, "compare-*.json")), key=order_key)
if not files:
    print("STOP: no compare-*.json in " + an)
    sys.exit(1)

def j(x):
    return json.dumps(x, sort_keys=True, separators=(",", ":"))

docs = {}
for p in files:
    try:
        docs[os.path.basename(p)] = json.load(open(p))
    except Exception as e:
        print("STOP: not valid JSON: " + p)
        sys.exit(1)

OPS_KEYS = ["status", "ratio_b_over_a", "ci95", "ci_method", "pairs", "unpaired_runs", "same_seeds", "same_session", "workloads", "dof", "bounded", "censored_pairs", "band"]
CURVE_KEYS = ["slug", "pairs", "ratio_b_over_a", "ci95", "ci_method"]


def w0_name(name):
    return name.rsplit("-w", 1)[0] + "-w0.json"

print("PROJECTION_KIND " + kind + " FILES " + str(len(files)))
for name, d in docs.items():
    print("COMPARE_FILE " + name)
    print("  schema=" + str(d.get("schema")) + " pricing_weight=" + j(d.get("pricing_weight")) + " resample_seed=" + j(d.get("resample_seed")) + " a_arm=" + str((d.get("a") or {}).get("arm")) + " b_arm=" + str((d.get("b") or {}).get("arm")) + " admissible=" + j(d.get("admissible")) + " a_session_status=" + str(d.get("a_session_status")) + " b_session_status=" + str(d.get("b_session_status")))
    ops = d.get("ops") or {}
    print("  OPS " + j({k: ops.get(k) for k in OPS_KEYS}))
    for row in d.get("curves") or []:
        print("  CURVE " + j({k: row.get(k) for k in CURVE_KEYS}))
    if name.endswith("-w0.json"):
        print("  A_UNPRICED_COUNTS " + j((d.get("a") or {}).get("unpriced_counts")))
        print("  B_UNPRICED_COUNTS " + j((d.get("b") or {}).get("unpriced_counts")))
    if kind == "wall":
        if name.endswith("-w0.json"):
            print("  WALL " + j(d.get("wall")))
        else:
            base = docs.get(w0_name(name))
            same = (base is not None) and (base.get("wall") == d.get("wall"))
            print("  WALL_BLOCK_IDENTICAL_TO_W0_FILE " + ("true" if same else "false"))
    else:
        print("  WALL_FIELDS_NOT_PRINTED (counted session)")
print("PROJECT_COMPARE_OK")
