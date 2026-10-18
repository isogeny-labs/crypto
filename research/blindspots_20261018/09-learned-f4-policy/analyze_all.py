#!/usr/bin/env python3
"""Item 09 assessment (third pass): frozen decision rule of PROTOCOL.md applied to the committed r2/r2b eval records.
Cells 17, 19, 21 (cell 13 has training records only). Bootstrap: 2000 resamples of targets, seed 1 (assessor's choice)."""
import json, glob, collections, random
RAW = "raw/"
CELLS = {
 "17": sorted(glob.glob(RAW + "eval-17-4-3-test-00-16-r2.jsonl") + glob.glob(RAW + "eval-17-4-3-test-16-32-r2.jsonl")),
 "19": sorted(glob.glob(RAW + "eval-19-4-3-test-*-r2.jsonl")),
 "21": sorted(glob.glob(RAW + "eval-21-5-3-test-*-r2.jsonl") + glob.glob(RAW + "eval-21-5-3-test-*-r2b.jsonl")),
}
def cost(r, w): return r["touches"] + w * r["policy_ops"]
out, summary = {}, {}
for cell, fs in CELLS.items():
    rs = [json.loads(l) for f in fs for l in open(f)]
    by = collections.defaultdict(dict)
    for r in rs: by[r["idx"]][r["arm"]] = r
    idxs = sorted(by)
    usable = [i for i in idxs if by[i]["a"]["outcome"] == "unsat"]
    cens_inc = [i for i in idxs if by[i]["a"]["outcome"] != "unsat"]
    arms = [x for x in ["b50", "b25", "c", "d", "p", "r50", "r25"] if all(x in by[i] for i in usable)]
    def ratio(sel, num, den, w): return sum(cost(by[i][num], w) for i in sel) / sum(cost(by[i][den], w) for i in sel)
    def boot(num, den, w):
        rng = random.Random(1); v = []
        for _ in range(2000):
            s = [rng.choice(usable) for _ in usable]; v.append(ratio(s, num, den, w))
        v.sort(); return v[49], v[1949]
    rows = lambda x: sum(by[i]["a"]["rows"] for i in usable) / sum(by[i][x]["rows"] for i in usable)
    print(f"=== cell {cell}: targets {len(idxs)}, usable {len(usable)}, censored incumbent {cens_inc} "
          f"{[by[i]['a']['outcome'] for i in cens_inc]}, checks false {sum(not r['check'] for r in rs)}, files {len(fs)}")
    cd = {"n_targets": len(idxs), "n_usable": len(usable), "censored_incumbent": cens_inc, "arms": {}, "null": {}}
    for x in arms:
        cens = sum(by[i][x]["outcome"] == "cap_touch" for i in usable)
        both = all(by[i][x]["basis_hash"] == by[i]["a"]["basis_hash"] for i in usable if by[i][x]["outcome"] == "unsat")
        ent = {"censored": cens, "rows_ratio": rows(x), "bases_equal": both, "w": {}}
        win = True
        for w in (0, 0.1, 1):
            rt = ratio(usable, "a", x, w); lo, hi = boot("a", x, w)
            med = sorted(cost(by[i]["a"], w) / cost(by[i][x], w) for i in usable); n = len(med)
            med = med[n // 2] if n % 2 else (med[n // 2 - 1] + med[n // 2]) / 2
            ent["w"][str(w)] = [rt, lo, hi, med]
            win &= (rt >= 1.3 and lo > 1)
            print(f"{x:4s} w={w:<4} {rt:7.3f} [{lo:.3f},{hi:.3f}] med {med:.3f} rows_a/rows_X {rows(x):.3f} censored {cens} bases_equal {both}")
        ent["touch_win"] = win; ent["row_ok"] = rows(x) >= 1.0; ent["cell_win"] = bool(win and both)
        ent["cell_win_with_rows"] = bool(win and both and rows(x) >= 1.0)
        cd["arms"][x] = ent
    for x, nl in (("b50", "r50"), ("b25", "r25"), ("c", "p"), ("d", "r50")):
        if x in arms and nl in arms:
            ent = {}
            for w in (0, 0.1, 1):
                rt = ratio(usable, nl, x, w); lo, hi = boot(nl, x, w); ent[str(w)] = [rt, lo, hi]
                print(f"null {nl}/{x} w={w:<4} {rt:7.3f} [{lo:.3f},{hi:.3f}]")
            cd["null"][x] = {"null": nl, "w": ent, "beats_null": all(v[1] > 1 for v in ent.values())}
    # random arms vs a (the 'measured improvement of selection' clause)
    for x in ("r50", "p"):
        if x in arms:
            cd["arms"][x]["improves_a_1.3"] = cd["arms"][x]["cell_win"]
    cd["sum_touches"] = {x: sum(by[i][x]["touches"] for i in usable) for x in ["a"] + arms}
    cd["sum_wall_ms"] = {x: sum(by[i][x]["wall_ms"] for i in usable) for x in ["a"] + arms}
    cd["sum_policy_ops"] = {x: sum(by[i][x]["policy_ops"] for i in usable) for x in ["a"] + arms}
    print("sum touches", cd["sum_touches"]); print("sum policy_ops", cd["sum_policy_ops"]); print("sum wall ms", cd["sum_wall_ms"])
    print("b50 per-target a/b50 min,max", min(by[i]["a"]["touches"]/by[i]["b50"]["touches"] for i in usable), max(by[i]["a"]["touches"]/by[i]["b50"]["touches"] for i in usable))
    out[cell] = cd
json.dump(out, open("figures/cells.json", "w"), indent=1)
