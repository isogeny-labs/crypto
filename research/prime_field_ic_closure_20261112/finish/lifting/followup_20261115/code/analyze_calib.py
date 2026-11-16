#!/usr/bin/env python3
"""Tables for lambda_calib.jsonl.  [pcc:unverified]"""
import sys, json, collections
import numpy as np

recs = [json.loads(l) for l in open(sys.argv[1])]
cells = collections.defaultdict(list)
for r in recs:
    cells[(r["r"], r["e"])].append(r)
out = []
allratio = []
print("r e n ok rank_det dep rank>r  lamMW/lamG[min,med]  rhoMW/rhoG[max,med]  rhoMW[med,max]  lang=lamMW/logDmin[min,med]  index>1")
for k in sorted(cells):
    rs = cells[k]
    ok = [x for x in rs if x.get("status") == "ok"]
    det = [x for x in ok if "lambda_MW" in x]
    dep = [x for x in ok if x.get("dependent")]
    both = [x for x in det if x.get("lambda_G")]
    ratio = [x["lambda_MW"] / x["lambda_G"] for x in both]
    allratio += ratio
    rr = [x["rho_MW"] / x["rho_G"] for x in both]
    rho = [x["rho_MW"] for x in det]
    lang = [x["lambda_MW"] / x["log_absDmin"] for x in det]
    gt = [x for x in det if x["rank_lo"] > k[0]]
    idx = [x for x in det if x.get("index") and x["index"] > 1.01]
    row = dict(r=k[0], e=k[1], n=len(rs), ok=len(ok), rank_determined=len(det), dependent=len(dep),
               rank_gt_r=len(gt), status=dict(collections.Counter(x.get("status", "?")[:20] for x in rs)),
               ratio_min=min(ratio) if ratio else None, ratio_med=float(np.median(ratio)) if ratio else None,
               frac_ratio_lt_1=(sum(1 for v in ratio if v < 0.999) / len(ratio)) if ratio else None,
               rho_ratio_max=max(rr) if rr else None, rho_ratio_med=float(np.median(rr)) if rr else None,
               rhoMW_med=float(np.median(rho)) if rho else None, rhoMW_max=max(rho) if rho else None,
               lang_min=min(lang) if lang else None, lang_med=float(np.median(lang)) if lang else None,
               index_gt1=len(idx))
    out.append(row)
    f = lambda v: "-" if v is None else (f"{v:.3f}" if isinstance(v, float) else str(v))
    print(k[0], k[1], row["n"], row["ok"], row["rank_determined"], row["dependent"], row["rank_gt_r"],
          f(row["ratio_min"]), f(row["ratio_med"]), f(row["rho_ratio_max"]), f(row["rho_ratio_med"]),
          f(row["rhoMW_med"]), f(row["rhoMW_max"]), f(row["lang_min"]), f(row["lang_med"]), row["index_gt1"], row["status"])
summ = dict(n_pairs=len(allratio), ratio_min=min(allratio), ratio_med=float(np.median(allratio)),
            frac_lt_1=sum(1 for v in allratio if v < 0.999) / len(allratio),
            p10=float(np.percentile(allratio, 10)))
print("all:", summ)
json.dump(dict(cells=out, summary=summ), open(sys.argv[2], "w"), indent=1)
