#!/usr/bin/env python3
"""Convert results/run.log (the per-instance lines pq2_rank prints) into
results/pq2_rank.json and results/pq2_rank.csv. The sweep was stopped during
n = 47 (Z enumeration ~6 min per instance) before the binary wrote its own JSON;
every number here is copied from the log, nothing is recomputed."""
import csv, json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
LOG = os.path.join(HERE, "results", "run.log")
PAT = re.compile(
    r"n=\s*(\d+) n'=\s*(\d+) N=\s*(\d+) inst=(\d+) \|Z\|=\s*(\d+) affdim=(\d+) k=\s*(\d+) rho=\s*(\d+) "
    r"low2=\s*(\S+) low3=\s*(\S+) low4=(\S+) d_lin=(\S+) \| iter3=(\S+) \((\d+) rounds\) "
    r"iter4=(\S+) \((\d+) rounds\) d_iter=(\S+) \((\d+) ms Z, (\d+) ms deg2, (\d+) ms deg3, (\d+) ms deg4, "
    r"(\d+) ms iter3, (\d+) ms iter4\)")
def num(s):
    return None if s == "-" else int(s)
rows = []
header = []
for line in open(LOG):
    m = PAT.search(line)
    if not m:
        header.append(line.rstrip("\n"))
        continue
    g = m.groups()
    rows.append(dict(n=int(g[0]), n_prime=int(g[1]), N=int(g[2]), instance=int(g[3]), Z=int(g[4]),
                     affdim=int(g[5]), k=int(g[6]), rho=int(g[7]), low2=num(g[8]), low3=num(g[9]),
                     low4=num(g[10]), d_lin=g[11], iter3=num(g[12]), iter3_rounds=int(g[13]),
                     iter4=num(g[14]), iter4_rounds=int(g[15]), d_iter=g[16], ms_Z=int(g[17]),
                     ms_deg2=int(g[18]), ms_deg3=int(g[19]), ms_deg4=int(g[20]), ms_iter3=int(g[21]),
                     ms_iter4=int(g[22])))
if len(rows) + len([h for h in header if h]) != sum(1 for l in open(LOG) if l.strip()):
    sys.exit("unparsed lines")
out = os.path.join(HERE, "results", "pq2_rank")
json.dump({"source": "results/run.log", "log_header": header, "complete": False,
           "note": "sweep stopped during n = 47; n = 48..56 not reached", "rows": rows},
          open(out + ".json", "w"), indent=1)
with open(out + ".csv", "w", newline="") as f:
    w = csv.DictWriter(f, fieldnames=list(rows[0]))
    w.writeheader(); w.writerows(rows)
print(len(rows), "rows")
