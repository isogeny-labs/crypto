import sys, json, random, time
import model, analysis

def run(mul, b, m, policy, ks, rule="deg", seed=0, tlimit=900):
    e = model.build(b, m, mul)
    free = list(e.free)
    rows = []
    for k in ks:
        if policy == "bits_seq":
            pins = set(free[:k])
        elif policy == "bits_rand":
            r = random.Random(seed); fr = free[:]; r.shuffle(fr); pins = set(fr[:k])
        elif policy == "twords":
            pins = set(); i = 0
            while len(pins) < 10**9 and i < len(e.twords) and k > 0 and i < k:
                pins.update(e.twords[i]); i += 1
        elif policy == "slopes":
            pins = set()
            for w_ in e.slopes[:k]: pins.update(w_)
        elif policy == "sums":
            pins = set()
            for w_ in e.sums[:k]: pins.update(w_)
        elif policy == "cwords":
            pins = set(); i = 0
            while i < len(e.cwords) and i < k:
                pins.update(e.cwords[i]); i += 1
        sc, es = analysis.build_graph(e, frozenset(pins))
        ar = max(len(s) for s in sc)
        t0 = time.time()
        r = analysis.eliminate(sc, rule, tlimit=tlimit) if rule != "none" else dict(width=-1, partial=False)
        adj = analysis.primal(sc); lb = analysis.mmd_plus(adj); dg = analysis.degeneracy(adj)
        rows.append(dict(mul=mul, b=b, m=m, policy=policy, k=k, pinned_signals=len(pins), max_arity=ar,
                         width=r["width"], mmd_plus=lb, degeneracy=dg, partial=r["partial"], secs=round(time.time()-t0,1),
                         enum_exp=(m-1)*b/m))
        print(json.dumps(rows[-1]), flush=True)
    return rows

if __name__ == "__main__":
    mul, b, m, policy = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
    ks = [int(x) for x in sys.argv[5].split(",")]
    rule = sys.argv[6] if len(sys.argv) > 6 else "fill"
    run(mul, b, m, policy, ks, rule)
