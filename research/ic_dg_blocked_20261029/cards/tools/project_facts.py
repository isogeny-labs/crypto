"""Prints a fixed small projection of analysis/facts.json and its sha256. usage: project_facts.py ANALYSIS_DIR"""
import hashlib, json, os, sys
p = os.path.join(sys.argv[1], "facts.json")
try:
    raw = open(p, "rb").read()
    f = json.loads(raw)
except Exception as e:
    print("STOP: cannot read " + p + ": " + type(e).__name__)
    sys.exit(1)
print("FACTS_JSON_SHA256 " + hashlib.sha256(raw).hexdigest() + " BYTES " + str(len(raw)))
g = f.get("l2_gate") or {}
bi = f.get("build_info") or {}
proj = {
    "audit": f.get("audit"),
    "binary_sha256_file": f.get("binary_sha256_file"),
    "binary_used": f.get("binary_used"),
    "binary_used_sha256": f.get("binary_used_sha256"),
    "binary_used_sha256_equals_build_info_ecbench_sha256": f.get("binary_used_sha256_equals_build_info_ecbench_sha256"),
    "binary_sha256_file_first_field_equals_build_info_ecbench_sha256": f.get("binary_sha256_file_first_field_equals_build_info_ecbench_sha256"),
    "build_info": {k: bi.get(k) for k in ["architecture", "ecbench_sha256", "source_revision", "target", "built_at"]},
    "job_exit": f.get("job_exit"),
    "l2_gate": {
        "required": g.get("required"),
        "executions": g.get("executions"),
        "grades": g.get("grades"),
        "accepted": g.get("accepted"),
        "invalid_outcomes_len": len(g.get("invalid_outcomes") or []),
        "below_l2_len": len(g.get("below_l2") or []),
    },
    "session_spec_equals_input_spec": f.get("session_spec_equals_input_spec"),
}
print(json.dumps(proj, indent=1, sort_keys=True))
print("FACTS_PROJECTION_OK")
