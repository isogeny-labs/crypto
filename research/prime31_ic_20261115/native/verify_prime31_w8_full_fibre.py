#!/usr/bin/env python3
"""Independent affine replay of the W8 full-fibre Rust census.
Uses raw published point coordinates and word witnesses, not Rust arithmetic.
The checker cannot establish nonenumerative extraction or a whole DLP solve.
"""
import json
import sys

P = 2147483647
A = 847514846
B = 2087444724
N = 2147446831
WEIGHTS = [
    [1,3,5,7,11,13,17,19],
    [1,2,4,8,16,32,64,128],
    [1,2,4,8,16,32,64,127],
    [1,3,9,27,81,243,729,2187],
]
LABELS = ["old","pow2","near127","ternary"]

def add(x, y):
    if x is None: return y
    if y is None: return x
    x1,y1 = x
    x2,y2 = y
    if x1 == x2 and (y1 + y2) % P == 0: return None
    if x1 == x2:
        lam = ((3*x1*x1+A)*pow(2*y1, -1, P))%P
    else:
        lam = ((y2-y1)*pow(x2-x1, -1, P))%P
    x3 = (lam*lam-x1-x2)%P
    return (x3,(lam*(x1-x3)-y1)%P)

def scalar(q,k):
    r=None
    while k:
        if k&1: r=add(r,q)
        q=add(q,q); k>>=1
    return r

def on_curve(q):
    if q is None: return True
    x,y=q
    return 0 <= x < P and 0 <= y < P and (y*y-x*x*x-A*x-B)%P == 0

def word_sum(word, begin, win, w, base):
    acc=None
    for j in range(begin,begin+4):
        d=word & 15
        word >>= 4
        pt=base[win[d&7]]
        if d>=8: pt=(pt[0],(-pt[1])%P)
        acc=add(acc,scalar(pt,w[j]))
    assert word==0
    return acc

def stats(vals):
    hist=[0]*7
    for m in vals:
        assert 0<=m<2**32
        hist[0 if m==0 else 1 if m==1 else 2 if m==2 else 3 if m==3
             else 4 if m<=7 else 5 if m<=15 else 6]+=1
    return dict(positive=len(vals)-hist[0],zero=hist[0],s=sum(vals),
                ss=sum(m*m for m in vals),max=max(vals),hist=hist)

def main(path):
    with open(path) as f: j=json.load(f)
    assert j['schema']=='prime31.w8.full-fibre.v1'
    c=j['curve']
    assert [c[k] for k in ('p','a','b','n','cofactor')]==[P,A,B,N,1]
    assert j['labels']==LABELS and j['weights']==WEIGHTS
    assert j['seed']=='5748385f48455432'
    per=j['samples_per_window']
    assert 1<=per<=256
    base=[None if p is None else tuple(p) for p in j['base']]
    assert len(base)==31 and all(on_curve(p) for p in base)
    pooled=[[] for _ in range(4)]
    pair=[[0]*4 for _ in range(4)]
    for wi, win in enumerate(j['per_window']):
        assert win['window']==wi
        assert [a['name'] for a in win['arms']]==LABELS
        assert len(win['multiplicity_per_target'])==4
        for a in range(4):
            vals=win['multiplicity_per_target'][a]
            assert len(vals)==per
            pooled[a].extend(vals)
            s=stats(vals); src=win['arms'][a]['stats']
            assert src['positive']==s['positive'] and src['zero']==s['zero']
            assert src['sum_ordered_witnesses']==s['s']
            assert src['sum_witness_squares']==str(s['ss'])
            assert src['max']==s['max']
            assert src['hist_0_1_2_3_4to7_8to15_16plus']==s['hist']
            assert win['arms'][a]['left_words']==65536
            assert win['arms'][a]['right_words']==65536
        for a in range(4):
            for b in range(4):
                pair[a][b]+=sum(x==0 and y>0 for x,y in zip(
                    win['multiplicity_per_target'][a],
                    win['multiplicity_per_target'][b]))
    assert j['pairwise_a_miss_b_hit']==pair
    for a,entry in enumerate(j['pooled']):
        assert entry['arm']==LABELS[a]
        s=stats(pooled[a])
        assert entry['stats']['positive']==s['positive']
        assert entry['stats']['sum_ordered_witnesses']==s['s']
        assert entry['stats']['sum_witness_squares']==str(s['ss'])
    certs=j['certificates']
    if per == 256: assert len(certs)>=16, f"only {len(certs)} certificates"
    for cert in certs:
        w=WEIGHTS[LABELS.index(cert['arm'])]
        assert cert['weights']==w
        win=cert['window_indices']
        assert win==j['per_window'][cert['window']]['indices']
        q=tuple(cert['target']) if cert['target'] is not None else None
        assert on_curve(q)
        l=word_sum(cert['left_word'],0,win,w,base)
        r=word_sum(cert['right_word'],4,win,w,base)
        assert add(l,r)==q, cert
        assert j['per_window'][cert['window']]['multiplicity_per_target'][LABELS.index(cert['arm'])][cert['q_index']]>0
    print(json.dumps({"validation":"independent-affine-witness-replay",
                      "certificates":len(certs),"curve":"prime31",
                      "total_targets":4*per,
                      "hits_by_arm":{LABELS[a]:stats(pooled[a])['positive'] for a in range(4)},
                      "ordered_witnesses_by_arm":{LABELS[a]:sum(pooled[a]) for a in range(4)}},
                     sort_keys=True))

if __name__=="__main__":
    if len(sys.argv)!=2: raise SystemExit("usage: checker.py result.json")
    main(sys.argv[1])
