"""Copy compact outputs into the repo mirror: per-instance summary + per_t (rows omitted; raw files listed in
MANIFEST.sha256 for S3 upload by SHA-256).  [pcc:unverified]"""
import json, glob, os, hashlib, shutil
H = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
R = os.path.join(H, "repo", "research", "prime_field_ic_closure_20261112", "finish", "theorem", "hypG")
os.makedirs(os.path.join(R, "out", "exh_summaries"), exist_ok=True)
man = []
for f in sorted(glob.glob(os.path.join(H, "out", "exh", "G_*.json"))):
    raw = open(f, "rb").read()
    man.append(f"{hashlib.sha256(raw).hexdigest()}  out/exh/{os.path.basename(f)}")
    try: D = json.loads(raw)
    except Exception: continue
    json.dump({"summary": D["summary"], "per_t": D["per_t"]}, open(os.path.join(R, "out", "exh_summaries", os.path.basename(f)), "w"))
for f in ["S5_generic.pkl"]:
    man.append(f"{hashlib.sha256(open(os.path.join(H, 'out', f), 'rb').read()).hexdigest()}  out/{f}")
open(os.path.join(R, "out", "RAW_MANIFEST.sha256"), "w").write("\n".join(man) + "\n")
for f in ["line_lemma.jsonl", "thresholds_hypG.jsonl", "exponent_floor.jsonl", "delta_R.jsonl", "delta_R2.jsonl", "exh_summary.json", "rm_analogue.jsonl",
          "rm_structured.jsonl", "check_dec.jsonl", "PRESET_hypG.txt", "exh_table.md", "exh_stats.json"]:
    if os.path.exists(os.path.join(H, "out", f)): shutil.copy(os.path.join(H, "out", f), os.path.join(R, "out", f))
os.makedirs(os.path.join(R, "code"), exist_ok=True)
for f in ["line_lemma.py", "exhaustive_G.py", "runjob.sh", "jobs.txt", "jobs_rest.txt", "aggregate.py", "stats.py", "delta_R.py",
          "thresholds_hypG.py", "exponent_floor.py", "rm_analogue.py", "rm_structured.py", "check_dec.py", "show.py", "export_repo.py"]:
    shutil.copy(os.path.join(H, "code", f), os.path.join(R, "code", f))
os.makedirs(os.path.join(R, "lean"), exist_ok=True)
for f in ["LineLemma.lean", "check.log"]:
    shutil.copy(os.path.join(H, "lean", f), os.path.join(R, "lean", f))
print("exported", len(man), "manifest entries to", R)
