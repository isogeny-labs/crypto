import json, sys
o = json.load(open(sys.argv[1]))["items"]; r = json.load(open(sys.argv[2]))["items"]
def key(x): return (x.get("arm", x.get("comparison", x.get("group", "?"))), x.get("s"), x.get("group"))
def canon(d):
    if isinstance(d, dict): return {k: canon(v) for k, v in d.items() if not any(t in k.lower() for t in ("wall", "_ms", "time"))}
    if isinstance(d, list): return [canon(v) for v in d]
    return d
om = {}
for x in o: om.setdefault(json.dumps(key(x)), []).append(x)
matched = eq = 0; diffs = []
for x in r:
    k = json.dumps(key(x)); cands = om.get(k, [])
    if not cands: diffs.append(("no_orig", k)); continue
    matched += 1
    ok = any(canon(c) == canon(x) for c in cands)
    if ok: eq += 1
    else: diffs.append(("differs", k, x.get("s")))
print({"replicated_items": len(r), "matched_to_orig": matched, "identical_ignoring_timing": eq, "n_diff": len(diffs)})
for d in diffs[:12]: print(d)
