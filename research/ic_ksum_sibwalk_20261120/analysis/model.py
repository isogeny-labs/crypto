#!/usr/bin/env python3
"""Count model for walk_sib=m (PROTOCOL.md section 2), fed by incumbent (`inc`) records only.
usage: python3 analysis/model.py <ecbench> <sessions dir> <out json> [arm=inc]
For each incumbent record it takes the relation phase's stride steps L, stride segments (renewals + restarts + 1) and
renewals, and models the candidate's relation phase on the same targets (the walk presents the same progression):
segment lengths geometric with the record's mean; per segment of length s (relation at position s), blocks of 2m+1 after
the anchor: centres ceil(s/(2m+1)), sibling pairs b*m + min(q+1, m) (b full blocks, q the last slot), a lift when the
last slot is on the minus side (q < m), and m multiple operations per stride that starts a block.
  dG = -L + centres + pairs + lifts + multiples          (group additions + doublings, GAE)
  dN = pairs (sibling_second_points, priced at w) - (pairs - lifts) (guard probes not made on missed abscissae)
       + lifts (the second lookup of a lifted hit)  = 2 lifts
(lookups and guard probes are calibration-priced native work, which the band reprices at w with the table inserts).
Predicted ratio at w = (P_inc(w) + mean dG + w mean dN) / P_inc(w), P_inc(w) = the incumbent's band price per run
(`ecbench band`, mean S(w) * sqrt(r)) on the same records."""
import json, os, sys, math

import subprocess, tempfile
BIN, D, OUT = sys.argv[1], sys.argv[2], sys.argv[3]
ARM = sys.argv[4] if len(sys.argv) > 4 else "inc"
MS = [1, 2, 3, 4, 6, 8, 10, 12, 16, 20, 24, 32]
WS = [0.0, 0.1, 1.0]


def seg_terms(mu, m, smax=None):
    """Expected (centres, pairs, lifts) per segment, lengths geometric on 1.. with mean mu."""
    p = 1.0 / mu
    smax = smax or int(mu * 40) + 50
    c = pr = lf = 0.0
    B = 2 * m + 1
    for s in range(1, smax):
        ps = p * (1 - p) ** (s - 1)
        if ps < 1e-14:
            break
        b, q = divmod(s - 1, B)
        c += ps * (b + 1)
        pr += ps * (b * m + min(q + 1, m))
        lf += ps * (1 if q < m else 0)
    return c, pr, lf


def band_price(d):
    """Mean band price per run of ARM at each w (GAE), and the run count."""
    with tempfile.NamedTemporaryFile(suffix=".json") as t:
        subprocess.run([BIN, "band", "--dir", d, "--json", t.name], capture_output=True, check=True)
        row = next(x for x in json.load(open(t.name))["rows"] if x["arm"] == ARM)
    return [s_ * 2 ** row["log2_r"] / 2 ** (row["log2_r"] / 2) for s_ in row["mean_s"]], row["runs"]


rows = {}
for n in sorted(os.listdir(D)):
    f = os.path.join(D, n, "records.jsonl")
    if not os.path.exists(f):
        continue
    recs = [json.loads(l) for l in open(f)]
    recs = [r for r in recs if r["arm"] == ARM and not r["warmup"] and r["outcome"]["status"] == "verified"]
    P, runs = band_price(os.path.join(D, n))
    pred = {}
    for m in MS:
        dG = dU = 0.0
        for r in recs:
            rel = next(ph for ph in r["phases"] if ph["name"] == "relations")["native"]
            L, ren, rst = rel["stride_steps"], rel["stride_renewals"], rel["stride_restarts"]
            segs = ren + rst  # restarts include the first start
            mu = max(L / max(segs, 1), 1.0)
            c, pr, lf = seg_terms(mu, m)
            c, pr, lf = c * segs, pr * segs, lf * segs
            mult = m * ren  # strides that start a block: first + renewals - the last one (approx.)
            dG += -L + c + pr + lf + mult
            dU += 2 * lf
        k = len(recs)
        pred[str(m)] = {str(w): (P[i] + (dG + w * dU) / k) / P[i] for i, w in enumerate(WS)}
    best = {str(w): min(MS, key=lambda m: pred[str(m)][str(w)]) for w in WS}
    rows[n] = {"records": len(recs), "band_price_inc": dict(zip(map(str, WS), P)), "pred": pred, "best_m": best}
json.dump(rows, open(OUT, "w"), indent=1)
for n, v in rows.items():
    print(n, f"recs={v['records']}", "P_inc", "/".join(f"{x:.0f}" for x in v["band_price_inc"].values()),
          "best m", v["best_m"], *(f"m{m}: " + "/".join(f"{v['pred'][str(m)][str(w)]:.3f}" for w in WS) for m in (4, 8, 16)))
