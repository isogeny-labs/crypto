#!/usr/bin/env python3
"""Same runner as run_all.py, but first CLAIMS its cells by writing placeholder res.json files
(so the concurrently running run_all.py process skips them), then runs them, overwriting the
placeholders.  Same command line, caps and thread rule as run_all.py.
Usage: run_claim.py OUTDIR NAME [NAME ...]"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import run_all as RA

def main():
    out = sys.argv[1]
    names = sys.argv[2:]
    metas = {json.loads(l)["name"]: json.loads(l) for l in open(os.path.join(out, "index.jsonl"))}
    rd = os.path.join(out, "runs"); os.makedirs(rd, exist_ok=True)
    todo = []
    for n in names:
        resf = os.path.join(rd, n + ".res.json")
        if os.path.exists(resf) and not json.load(open(resf)).get("placeholder"):
            continue
        json.dump({"name": n, "placeholder": True, "D": None, "W": None, "stop": "claimed", "wall_s": None,
                   "exit": None}, open(resf, "w"))
        todo.append(n)
    for n in todo:
        r = RA.run_one(out, metas[n], force=True)  # placeholder stays until overwritten
        print(r["name"], "D", r["D"], "W", r["W"], "unit", r.get("unit"), "stop", r["stop"], "wall", r["wall_s"], flush=True)

main()
