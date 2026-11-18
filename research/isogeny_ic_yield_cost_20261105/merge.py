import sys; ROOT = sys.argv[1] if len(sys.argv) > 1 else "results"; BASEMODE = sys.argv[2] if len(sys.argv) > 2 else "smallx"
import csv, json, glob, math
rows = []; meta = None; metas = []; bits = {}
for d in sorted(glob.glob(f"{ROOT}/shard_*")):
    rows += list(csv.DictReader(open(f"{d}/per_scalar.csv")))
    mj = json.load(open(f"{d}/meta.json")); metas += mj["meta"]; meta = meta or mj
    bits.update(json.load(open(f"{d}/bits.json")))
meta["meta"] = metas
json.dump(meta, open(f"{ROOT}/meta.json", "w"), indent=1)
with open(f"{ROOT}/per_scalar.csv", "w", newline="") as f:
    w = csv.DictWriter(f, fieldnames=list(rows[0].keys())); w.writeheader(); w.writerows(rows)
def phi_coef(u, v):
    n = len(u); a = sum(x*y for x, y in zip(u, v)); su = sum(u); sv = sum(v)
    den = math.sqrt(su*(n-su)*sv*(n-sv)) or float("nan"); return (n*a - su*sv)/den, n
pairs = []
for key, bl in bits.items():
    m, name, mode = key.split("|"); base = bits[f"{m}|E0|{BASEMODE}"]; u = []; v = []
    for b0, b1 in zip(base, bl):
        L = min(len(b0), len(b1)); u += b0[:L]; v += b1[:L]
    pc, n = phi_coef(u, v)
    pairs.append(dict(m=m, curve=name, fbmode=mode, n_trials=n, p_E0=sum(u)/n, p_curve=sum(v)/n, both=sum(x*y for x, y in zip(u, v))/n, phi=pc))
with open(f"{ROOT}/paired.csv", "w", newline="") as f:
    w = csv.DictWriter(f, fieldnames=list(pairs[0].keys())); w.writeheader(); w.writerows(pairs)
print(len(rows), "rows,", len(pairs), "pairs")
