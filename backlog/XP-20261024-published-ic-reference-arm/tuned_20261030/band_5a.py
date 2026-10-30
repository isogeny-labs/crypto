"""AGENTS.md 5a band for follow-up A: S_w = (counted GAE less any solver GAE priced through the pinned word-XOR
calibration) / sqrt(r) + w * (solver native ops) / sqrt(r), w in {0, 0.1, 1}. Under the corrected rule (section 9.3)
the lab's word-XOR counter is partial, so it is unpriced at every size, like msolve's instructions. Median per arm."""
import json, math, statistics as st
for sess in ("T9", "T13", "T13b"):
    recs = [json.loads(l) for l in open(f"runs/{sess}/records.jsonl")]
    for arm in dict.fromkeys(r["arm"] for r in recs):
        vals = {w: [] for w in (0, 0.1, 1)}
        for r in recs:
            if r["arm"] != arm or r["outcome"]["status"] != "verified": continue
            rr = math.sqrt(r["workload"]["curve"]["r"]); sol = r.get("solver") or {}
            dsol = (r["detail"].get("decomposition") or {}).get("solver", {})
            priced = dsol.get("gae", 0) if dsol.get("priced_by") == "pinned" else 0
            base = (r["cost"]["total_gae"] - priced) / rr
            for w in vals: vals[w].append(base + w * sol.get("ops", 0) / rr)
        if not vals[0]: print(sess, arm, "no verified run"); continue
        print(sess, arm, " ".join(f"w={w}:{st.median(v):.3g}" for w, v in vals.items()))
