"""Facts of one session's result copy. Reads files and records them; stops only when a file name is ambiguous or the binary is wrong.
usage: dgb_facts.py EKS_DIR ANALYSIS_DIR WANT_REV FB_PATH FB_SHA
Writes ANALYSIS_DIR/facts.json and ANALYSIS_DIR/binary-path.txt (the keys project_facts.py and receipt.py read)."""
import collections, hashlib, json, os, sys
root, outdir, want_rev, fb_path, fb_sha = sys.argv[1:6]

def sha(p):
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for c in iter(lambda: f.read(1 << 20), b""):
            h.update(c)
    return h.hexdigest()

def find(name):
    hits = sorted(os.path.join(d, f) for d, _, fs in os.walk(root) for f in fs if f == name)
    if len(hits) > 1:
        print("STOP: more than one", name, hits)
        sys.exit(1)
    return hits[0] if hits else None

def load(name):
    p = find(name)
    if p is None:
        return (None, None)
    try:
        return (os.path.relpath(p, root), json.load(open(p)))
    except Exception as e:
        print("FACT_UNREADABLE " + name + " " + type(e).__name__)
        return (os.path.relpath(p, root), None)

out = {}
bp, bi = load("build-info.json")
out["build_info_path"] = bp
out["build_info"] = bi
out["source_revision_equals_want"] = (bi or {}).get("source_revision") == want_rev
bsp = find("binary.sha256")
out["binary_sha256_file"] = open(bsp).read().strip() if bsp else None

ap, au = load("audit.json")
out["audit_path"] = ap
if au is not None:
    keys = ["schema", "session_id", "session_status", "records_bound", "ok", "problems", "checks", "records", "verified_records", "regraded"]
    out["audit"] = {k: au.get(k) for k in keys}
    rp = au.get("replays") or []
    out["audit"]["replays_total"] = len(rp)
    out["audit"]["replays_reproduced_true"] = sum(1 for r in rp if r.get("reproduced") is True)
    out["audit"]["replay_detail_counts"] = dict(collections.Counter(str(r.get("detail")) for r in rp))
else:
    out["audit"] = None
lp, lg = load("l2-gate.json")
out["l2_gate_path"] = lp
out["l2_gate"] = lg
jp, je = load("job-exit.json")
out["job_exit_path"] = jp
out["job_exit"] = je
ccp = find("control-comparison.json")
out["control_comparison_present"] = ccp is not None

ip = find("input-spec.json")
sp = find("spec.json")
out["session_spec_equals_input_spec"] = (json.load(open(ip)) == json.load(open(sp))) if ip and sp else None

want = (bi or {}).get("ecbench_sha256")
if not os.path.isfile(fb_path) or sha(fb_path) != fb_sha:
    print("STOP: binary", fb_path, "is absent or its sha256 is not", fb_sha)
    sys.exit(1)
out["binary_used"] = fb_path
out["binary_used_source"] = "dev-shm-build"
out["binary_used_sha256"] = sha(fb_path)
out["binary_used_sha256_equals_build_info_ecbench_sha256"] = (sha(fb_path) == want)
out["binary_sha256_file_first_field_equals_build_info_ecbench_sha256"] = (bool(out["binary_sha256_file"]) and out["binary_sha256_file"].split()[0] == want)
json.dump(out, open(os.path.join(outdir, "facts.json"), "w"), indent=2, sort_keys=True)
open(os.path.join(outdir, "binary-path.txt"), "w").write(fb_path + "\n")
print("FACTS_OK source_revision=" + str((bi or {}).get("source_revision")) + " source_revision_equals_want=" + json.dumps(out["source_revision_equals_want"]) + " binary_used=" + fb_path)
print("audit.ok=" + json.dumps((au or {}).get("ok")) + " audit.session_status=" + json.dumps((au or {}).get("session_status")) + " l2_gate.accepted=" + json.dumps((lg or {}).get("accepted")) + " job_exit.exit_code=" + json.dumps((je or {}).get("exit_code")) + " control_comparison_present=" + json.dumps(out["control_comparison_present"]))
