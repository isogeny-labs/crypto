"""Counting check for component (3): relations needed vs B for random and
forced-column ('rank-aware steered') relation collection, m=2 and m=3, B=256,
exact elimination over F_r, r ~ 2^20 prime.  Two stopping rules:
 (a) first linear dependency among rows  (= left-kernel vector = DLP solved for one target)
 (b) rank == B                            (= all base logs, preprocessing variant)
"""
import random, math, json, sys
r = 1048583  # prime ~ 2^20
B = 256

def run(m, steered, seed):
    rng = random.Random(seed)
    piv = {}           # pivot column -> reduced row (dict col->coef)
    deg = [0]*B
    rank = 0; rows = 0; first_dep = None
    while rank < B:
        cols = rng.sample(range(B), m)
        if steered:
            # force the lowest-degree column into the relation (R = P_i + aP + bQ)
            i = min(range(B), key=lambda c: deg[c])
            cols[0] = i
            if len(set(cols)) < m:
                cols = [i] + rng.sample([c for c in range(B) if c != i], m-1)
        row = {}
        for c in cols:
            row[c] = (row.get(c,0) + rng.choice((1, r-1))) % r
        row = {c:v for c,v in row.items() if v}
        for c in cols: deg[c] += 1
        rows += 1
        # reduce
        while row:
            c = min(row)
            if c in piv:
                pr = piv[c]; f = row[c]
                for cc, vv in pr.items():
                    nv = (row.get(cc,0) - f*vv) % r
                    if nv: row[cc] = nv
                    else: row.pop(cc, None)
            else:
                inv = pow(row[c], r-2, r)
                row = {cc:(vv*inv)%r for cc,vv in row.items()}
                piv[c] = row; rank += 1; row = None; break
        if row is not None and first_dep is None:
            first_dep = rows
        if rows > 20*B: break
    return first_dep, rows, rank

out = {}
for m in (2,3):
    for steered in (False, True):
        fd=[]; fr=[]
        for s in range(30):
            a,b,rk = run(m, steered, 1000*m+s)
            fd.append(a if a else float('nan')); fr.append(b)
        key=f"m={m},{'steered' if steered else 'random'}"
        out[key] = {"rows_to_first_dependency_over_B": sum(fd)/len(fd)/B,
                    "rows_to_full_rank_over_B": sum(fr)/len(fr)/B,
                    "min_full": min(fr)/B, "max_full": max(fr)/B}
        print(key, json.dumps(out[key]))
Hb = sum(1/k for k in range(1,B+1))
print("coupon-collector prediction rows_to_full_rank/B (random):", {m: round(Hb/m,3) for m in (2,3)})
print("k-XORSAT full-row-rank threshold k=3: 0.9179 ; even-cycle first dependency m=2: ~0.5")
json.dump(out, open(sys.argv[1],'w'), indent=1)
