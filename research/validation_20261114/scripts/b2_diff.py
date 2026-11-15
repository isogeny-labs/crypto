#!/usr/bin/env python3
"""Byte comparison of the B2 outputs of two ecbench binaries, and of the
re-derived bound/verdict files against the committed ones (Validator V).

Usage: b2_diff.py PRE_DIR POST_DIR ROOT OUT
"""
import hashlib, json, os, sys

pre, post, root, out = sys.argv[1:5]
h = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest() if os.path.exists(p) else None
names = sorted(set(os.listdir(pre)) | set(os.listdir(post)))
rows = []
for n in names:
    a, b = os.path.join(pre, n), os.path.join(post, n)
    ha, hb = h(a), h(b)
    row = {"file": n, "pre_sha256": ha, "post_sha256": hb, "identical": ha == hb and ha is not None}
    if not row["identical"] and ha and hb and n.endswith((".json", ".stdout")):
        try:
            ja, jb = json.load(open(a)), json.load(open(b))
            def diff(x, y, p=""):
                if isinstance(x, dict) and isinstance(y, dict):
                    return [d for k in sorted(set(x) | set(y)) for d in (diff(x.get(k, "<absent>"), y.get(k, "<absent>"), p + "/" + k))]
                if isinstance(x, list) and isinstance(y, list) and len(x) == len(y):
                    return [d for i, (u, v) in enumerate(zip(x, y)) for d in diff(u, v, f"{p}[{i}]")]
                return [] if x == y else [(p, str(x)[:200], str(y)[:200])]
            row["json_diffs"] = diff(ja, jb)[:40]
        except Exception as e:
            row["note"] = f"not JSON: {e}"
    rows.append(row)
committed = {
    "calibration-prime-rho-neg-v2.json": "docs/bounds/records/calibration-prime-rho-neg-v2.json",
    "session-prime-toy-reference-e1-incumbent-v2.json": "docs/bounds/records/session-prime-toy-reference-e1-incumbent-v2.json",
    "calibration-koblitz-toy-rho-frob-v2.json": "docs/bounds/records/calibration-koblitz-toy-rho-frob-v2.json",
    "verdict-prime-toy-reference-e1.json": "research/ecbench_bounds_challenges_20261006/verdict-prime-toy-reference-e1.json",
    "challenge-prime-toy-reference-e1-bsgs-negation.json": "docs/bounds/records/challenge-prime-toy-reference-e1-bsgs-negation.json",
}
vs = []
for n, c in committed.items():
    hc = h(os.path.join(root, c))
    vs.append({"file": n, "committed": c, "committed_sha256": hc, "pre_equal": h(os.path.join(pre, n)) == hc,
               "post_equal": h(os.path.join(post, n)) == hc})
res = {"pre": pre, "post": post, "files": rows, "vs_committed": vs,
       "all_identical": all(r["identical"] for r in rows if not r["file"].endswith(".stderr")),
       "stderr_identical": all(r["identical"] for r in rows if r["file"].endswith(".stderr"))}
json.dump(res, open(out, "w"), indent=1)
print("files", len(rows), "identical (non-stderr):", sum(r["identical"] for r in rows if not r["file"].endswith(".stderr")),
      "/", sum(1 for r in rows if not r["file"].endswith(".stderr")))
for r in rows:
    if not r["identical"]: print("DIFFERS", r["file"], r.get("json_diffs", r.get("note", ""))[:5] if isinstance(r.get("json_diffs"), list) else r.get("note"))
for v in vs: print("vs committed", v["file"], "pre", v["pre_equal"], "post", v["post_equal"])
