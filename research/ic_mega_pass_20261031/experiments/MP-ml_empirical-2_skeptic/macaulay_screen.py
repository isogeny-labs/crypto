"""Degree-<=d F_2 Macaulay statistics of `ic corpus` S4 Weil-descent instances, SAT vs UNSAT.
Columns = squarefree monomials of degree <= d, indexed so that HIGHER degree = HIGHER bit;
Gaussian elimination by leading bit, so a pivot whose leading monomial has degree k < d is a
'fall' element (W_d cap B_{<=k}).  Reports, per degree d: rank(W_d), dim(W_d cap B_{<=d-1}),
dim(W_d cap B_{<=2}), dim(W_d cap B_{<=1}), 1 in W_d (refutation), and the dimension of the
cokernel (truncated Hilbert function HF_d = #mon_{<=d} - rank).
"""
import sys, os, re, json, itertools, time, glob

def parse_anf(path):
    eqs=[]; names={}
    with open(path) as fh:
        for line in fh:
            line=line.strip()
            if not line or line[0] in 'cp': continue
            poly=set()
            for term in line.split('+'):
                term=term.strip()
                if term=='1': m=0
                else:
                    m=0
                    for v in term.split('*'):
                        v=v.strip()
                        if v not in names: names[v]=len(names)
                        m|=1<<names[v]
                if m in poly: poly.remove(m)
                else: poly.add(m)
            eqs.append(frozenset(poly))
    return eqs, len(names)

def monomials_upto(N, d):
    out=[]
    for k in range(d+1):
        for comb in itertools.combinations(range(N), k):
            m=0
            for p in comb: m|=1<<p
            out.append(m)
    return out   # already degree-ascending

def macaulay_stats(eqs, N, d):
    monos=monomials_upto(N,d)
    col={m:i for i,m in enumerate(monos)}            # degree ascending => high degree = high bit
    deg_of_col=[bin(m).count('1') for m in monos]
    # degree boundaries
    first_col_of_deg=[0]*(d+2)
    for i,m in enumerate(monos):
        k=deg_of_col[i]
        if first_col_of_deg[k]==0 and k>0: first_col_of_deg[k]=i
    first_col_of_deg[d+1]=len(monos)
    rows=[]
    for f in eqs:
        df=max(bin(m).count('1') for m in f)
        for s in monos:
            if bin(s).count('1')>d-df: continue
            prod=set()
            for mm in f:
                x=mm|s
                if x in prod: prod.remove(x)
                else: prod.add(x)
            if not prod: continue
            r=0
            for x in prod: r|=1<<col[x]
            rows.append(r)
    piv={}
    for r in rows:
        while r:
            lb=r.bit_length()-1
            p=piv.get(lb)
            if p is None:
                piv[lb]=r; break
            r^=p
    rank=len(piv)
    lead_deg=[deg_of_col[lb] for lb in piv]
    stats=dict(d=d, ncols=len(monos), nrows=len(rows), rank=rank, HF=len(monos)-rank,
               fall=sum(1 for k in lead_deg if k<d),
               le2=sum(1 for k in lead_deg if k<=2),
               le1=sum(1 for k in lead_deg if k<=1),
               const=int(0 in piv))
    return stats

def main():
    corpus_dir=sys.argv[1]; dmax=int(sys.argv[2]); out=sys.argv[3]
    res=[]
    for anf in sorted(glob.glob(os.path.join(corpus_dir,'*.anf'))):
        name=os.path.basename(anf)[:-4]
        eqs,N=parse_anf(anf)
        sat=name.endswith('-S')
        rec=dict(name=name, sat=sat, N=N, neqs=len(eqs))
        t0=time.time()
        for d in range(2,dmax+1):
            rec['d%d'%d]=macaulay_stats(eqs,N,d)
        rec['secs']=round(time.time()-t0,2)
        res.append(rec)
        print(json.dumps(rec), flush=True)
    json.dump(res, open(out,'w'), indent=1)

if __name__=='__main__': main()
