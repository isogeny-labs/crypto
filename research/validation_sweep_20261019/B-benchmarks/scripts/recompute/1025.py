import json, os, collections, statistics as st, math
BASE = "/home/user/crypto/research/fiber_ic_scaling_20261011/sessions"
SESS = ["scaling_h4", "scaling_h72", "scaling_h72_small_b"]
recs = []
for s in SESS:
    rs = [json.loads(l) for l in open(os.path.join(BASE, s, "records.jsonl"))]
    print(s, "records:", len(rs))
    recs += [(s, r) for r in rs]
print("total records:", len(recs))
def env_table(session_set, closed_filter):
    g = collections.defaultdict(list)
    for s, r in recs:
        if s not in session_set or r["warmup"]:
            continue
        g[(r["workload"]["curve"]["slug"], r["arm"])].append(r["cost"]["s"])
    curves = sorted({k[0] for k in g if k[0].startswith("icv1")})
    out = {}
    for c in curves:
        rho = st.mean(g[(c, "rho-neg")])
        bits = int(c.split("-fp")[1].split("-")[0])
        arms = [a for (cc, a) in g if cc == c and closed_filter(a)]
        best = min(arms, key=lambda a: st.mean(g[(c, a)]))
        out[c] = (bits, best, st.mean(g[(c, best)]) / rho)
    return out
def show(label, out):
    for bits in (18, 30):
        vals = sorted(v[2] for v in out.values() if v[0] == bits)
        arms = sorted({v[1] for v in out.values() if v[0] == bits})
        print(f"{label} bits={bits}: envelope/rho per curve = {[round(v,2) for v in vals]} arms={arms}")
h4 = env_table({"scaling_h4"}, lambda a: a.startswith("closed-b"))
show("h=4 (closed-b16..b256)", h4)
h72_all = env_table({"scaling_h72", "scaling_h72_small_b"}, lambda a: a.startswith("closed-b"))
show("h=72 (closed-b4..b256 pooled, stated rule B=4..256)", h72_all)
h72_b16 = env_table({"scaling_h72"}, lambda a: a.startswith("closed-b"))
show("h=72 (scaling_h72 only, B>=16) [informational]", h72_b16)
print("h=72 pooled sessions: arms present:", sorted({a for (s,r) in recs if s in ("scaling_h72","scaling_h72_small_b") for a in [r['arm']]}))
