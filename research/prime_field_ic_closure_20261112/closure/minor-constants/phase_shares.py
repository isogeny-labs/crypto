#!/usr/bin/env python3
"""Phase shares of the prime-field incumbent IC (ic.pipeline, oracle
mitm:negation_folded=1, prime-abscissa base, targets=walk) from local
ecbench records.  Read-only; [pcc:unverified].

Per record:
  la_share_w      = w*row_ops / (total_gae + w*sum(all native ops))   (band price w)
  dbl_share       = doubles(all phases) / total_gae    (upper bound on a-dependent GAE)
  setup_share     = oracle_setup gae / total_gae
  la_wall_share   = linear_algebra wall_ns / sum phase wall_ns
  lookups_per_gae = relation lookups / total_gae
"""
import json, sys, statistics as st
from collections import defaultdict

root = "/home/user/crypto/research/"
files = [l.strip() for l in open(sys.argv[1]) if l.strip()]
rows = defaultdict(list)
seen = set()
for f in files:
    try:
        fh = open(root + f[2:])
    except OSError:
        continue
    for line in fh:
        if '"ic.pipeline"' not in line or "negation_folded=1" not in line:
            continue
        r = json.loads(line)
        if r.get("warmup") or r["outcome"]["status"] != "verified":
            continue
        p = r["method"]["params"]
        if p.get("oracle") != "mitm:negation_folded=1" or not p.get("factor_base", "").startswith("prime-abscissa"):
            continue
        if p.get("targets") != "walk" or not p.get("linalg", "").startswith("incremental-gauss"):
            continue
        c = r["workload"]["curve"]
        if c.get("family") != "prime":
            continue
        bits = c["field_bits"]
        if not (20 <= bits <= 32):
            continue
        if r["record_id"] in seen:
            continue
        seen.add(r["record_id"])
        ph = {x["name"]: x for x in r["phases"]}
        tot = r["cost"]["total_gae"]
        native = sum(v for x in r["phases"] for v in x["native"].values()
                     if isinstance(v, (int, float)))
        la = ph.get("linear_algebra", {"native": {}, "wall_ns": 0})
        rowops = la["native"].get("row_ops", 0)
        dbl = sum(x["doubles"] for x in r["phases"])
        walls = [x["wall_ns"] or 0 for x in r["phases"]]
        rows[bits].append(dict(
            fb=p["factor_base"],
            la0=0.0,
            la01=0.1 * rowops / (tot + 0.1 * native),
            la1=rowops / (tot + native),
            dbl=dbl / tot,
            setup=ph.get("oracle_setup", {"gae": 0})["gae"] / tot,
            rel=ph.get("relations", {"gae": 0})["gae"] / tot,
            la_wall=(la["wall_ns"] or 0) / sum(walls) if sum(walls) else float("nan"),
            look=ph.get("relations", {"native": {}})["native"].get("lookups", 0) / tot,
            invs=(r.get("field_ops") or {}).get("invs"),
            gae=tot,
            src=f.split("/sessions")[0][2:],
            rowops=rowops, native=native, dbls=dbl,
        ))

def q(xs):
    xs = sorted(xs)
    return f"{xs[0]:.4f} / {st.median(xs):.4f} / {xs[-1]:.4f}"

print("bits | n | LA share w=0.1 (min/med/max) | LA share w=1 | LA wall share | doubles/GAE | oracle_setup/GAE | relations/GAE | lookups/GAE")
for b in sorted(rows):
    R = rows[b]
    print(f"{b} | {len(R)} | {q([x['la01'] for x in R])} | {q([x['la1'] for x in R])} | "
          f"{q([x['la_wall'] for x in R])} | {q([x['dbl'] for x in R])} | {q([x['setup'] for x in R])} | "
          f"{q([x['rel'] for x in R])} | {q([x['look'] for x in R])}")
print()
print("sources:", sorted({x['src'] for R in rows.values() for x in R}))
print("records with field_ops.invs:", sum(1 for R in rows.values() for x in R if x['invs'] is not None))

# Pooled (sum over records) shares: what an LA or doubling change could move in Sum S.
print()
print("bits | pooled LA share w=0.1 | pooled LA share w=1 | pooled doubles/GAE")
for b in sorted(rows):
    R = rows[b]
    G = sum(x['gae'] for x in R); N = sum(x['native'] for x in R); L = sum(x['rowops'] for x in R); D = sum(x['dbls'] for x in R)
    print(f"{b} | {0.1*L/(G+0.1*N):.5f} | {L/(G+N):.5f} | {D/G:.5f}")
