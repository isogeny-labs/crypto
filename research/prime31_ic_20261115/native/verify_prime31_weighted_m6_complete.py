#!/usr/bin/env python3
"""Independently authored whole-transcript group-law replay. Unsigned PCC."""
import json
import sys

P = 2**31 - 1
N = 2147446831
A = 847514846
B = 2087444724
G = (625037715, 2100627251)
W = (1, 3, 5, 7, 11, 13)

def pt(obj):
    return None if obj is None else tuple(obj)

def add(p, q):
    if p is None: return q
    if q is None: return p
    x, y = p
    z, w = q
    if x == z and (y + w) % P == 0:
        return None
    if x == z:
        num, den = (3*x*x + A) % P, (2*y) % P
    else:
        num, den = (w-y) % P, (z-x) % P
    lam = num * pow(den, P-2, P) % P
    xx = (lam*lam-x-z) % P
    return xx, (lam*(x-xx)-y) % P

def mul(k, p):
    out = None
    while k:
        if k & 1: out = add(out, p)
        p = add(p, p)
        k >>= 1
    return out

def is_on(p):
    return p is None or (
        0 <= p[0] < P and 0 <= p[1] < P and
        (p[1]*p[1] - p[0]**3 - A*p[0]-B) % P == 0
    )

def weighted(base, word):
    assert len(word) == 6
    acc = None
    for weight, index in zip(W, word):
        assert 0 <= index < len(base)
        acc = add(acc, mul(weight, base[index]))
    return acc

def coeffs(word):
    r = [0]*31
    for weight, index in zip(W, word):
        r[index] = (r[index]+weight)%N
    return r

def rank(rows):
    pivots = {}
    for rr in rows:
        row = rr[:]
        for col in range(31):
            if row[col] == 0: continue
            if col in pivots:
                factor = row[col]
                prev = pivots[col]
                row = [(v-factor*w)%N for v,w in zip(row,prev)]
            else:
                inv = pow(row[col],N-2,N)
                pivots[col] = [(v*inv)%N for v in row]
                break
        else:
            assert row[31] == 0, "training equations inconsistent"
    return len(pivots)

def main(filename):
    with open(filename,encoding="utf-8") as f: r=json.load(f)
    assert r["protocol"]=="prime31-weighted-m6-complete-v1"
    assert (r["source_p"],r["source_a"],r["source_b"],r["source_n"])==(P,A,B,N)
    assert tuple(r["generator"])==G and tuple(r["weights"])==W
    assert r["generic_three_plus_three_enumeration"] is True
    assert r["nonenumerative_algebraic_extractor"] is False
    assert r["pcc"]=="unverified"
    assert is_on(G) and mul(N,G) is None
    base = list(map(pt,r["base"]))
    assert len(base)==31 and len(set(base))==31 and all(is_on(q) for q in base)
    assert all(q[1]<=P-q[1] for q in base)
    assert r["train_calls"]<=512 and r["target_limit"]==1024 and r["shift_cap"]==64
    rows=[]
    for entry in r["training"]:
        q=pt(entry["public_residual"])
        a=entry["known_query_coefficient"]
        word=entry["word"]
        assert is_on(q) and mul(a,G)==q and weighted(base,word)==q
        rows.append(coeffs(word)+[a])
    assert len(rows)==r["train_odd"] and rank(rows)==r["train_rank"]
    if r["train_rank"]!=31:
        assert r["completed"]==0 and r["target_attempts"]==0
        print(json.dumps({"status":"CENSORED_RANK","training":len(rows),"rank":r["train_rank"]}))
        return 0
    logs=r["base_logs"]
    assert len(logs)==31
    for ell,p in zip(logs,base):
        assert 0<=ell<N and mul(ell,G)==p
    for row in rows:
        assert sum(x*y for x,y in zip(row[:31],logs))%N==row[31]
    assert len(r["targets"])==1024
    solved=0
    shifts=0
    seen=set()
    for idx,entry in enumerate(r["targets"]):
        assert entry["index"]==idx
        q=pt(entry["public_point"])
        assert is_on(q)
        assert q not in seen, "target corpus must be distinct"
        seen.add(q)
        if entry.get("censored",False):
            shifts += 64
            continue
        ell=entry["recovered"]
        assert 0<=entry["shift"]<64
        assert ell==entry["known_scalar_verifier_only"]
        assert mul(ell,G)==q
        word=entry["word"]
        s=entry["shift"]
        assert weighted(base,word)==add(q,mul(s,G))
        assert (sum(W[i]*logs[word[i]] for i in range(6))-s)%N==ell
        shifts += s+1
        solved += 1
    assert solved==r["completed"] and shifts==r["target_attempts"]
    print(json.dumps({
        "status":"PRODUCER_INDEPENDENT_ARITHMETIC_REPLAY_PASS",
        "pcc":"unverified",
        "curve":"Prime31 original cofactor-one",
        "training_rows":len(rows),"rank":rank(rows),
        "factor_base_logs":31,"targets_solved":solved,
        "targets_total":1024,"attempts":shifts,
        "limits":"no same-host mature IC/BSGS/rho comparison, no nonenumerative extractor"
    },sort_keys=True))
    return 0

if __name__=="__main__":
    try:
        sys.exit(main(sys.argv[1]))
    except (IndexError, AssertionError, ValueError, KeyError) as exc:
        print("FAIL:",repr(exc),file=sys.stderr)
        sys.exit(1)
