"""Symbolic window-{0..3} closure over F2(z,c) with PARI (cypari2). z_j=z^(2^j), c_j=c^(2^j).
Outputs the K-RREF (L9 cubic-first order) of W3∩Win and W2∩Win and the 4 new relations, factored."""
import sys,itertools,json,time; sys.path.insert(0,'.')
import cypari2; pari=cypari2.Pari(); pari.allocatemem(2*10**9)
from zmodel import wt,inwin,pmono,vars_
lo,hi=int(sys.argv[2]),int(sys.argv[3]); win=range(int(sys.argv[4]),int(sys.argv[5]))
CERT=len(sys.argv)>6
one=pari('Mod(1,2)'); Z=pari('Mod(1,2)*z'); C=pari('Mod(1,2)*c')
def Fj(j):
    zj=Z**(2**j); cj=C**(2**j); A=lambda p:1<<(p-lo); P={}
    def add(k,v):
        P[k]=P.get(k,0)+v
        if P[k]==0: del P[k]
    add((A(j+1),A(j+1)),one); add((A(j),A(j)),zj); add((A(j+1),0),zj**2); add((0,A(j+1)),zj**2); add((0,0),cj); return P
def pm(P,e,f):
    Q={}
    for (e1,f1),v in P.items():
        k=(e1+e,f1+f); Q[k]=Q.get(k,0)+v
        if Q[k]==0: del Q[k]
    return Q
V=vars_(lo,hi); mults=[(0,0)]+V+[(a[0]+b[0],a[1]+b[1]) for a,b in itertools.combinations(V,2)]
gens=[pm(Fj(j),*mu) for j in range(lo,hi-1) for mu in mults]
def mat(rows,cols):
    ci={k:i for i,k in enumerate(cols)}
    M=pari.matrix(len(cols),len(rows))   # columns = rows (transpose)
    for r,P in enumerate(rows):
        for k,v in P.items():
            if k in ci: M[ci[k],r]=v
    return M
def comb(lam,rows):
    out={}
    for i in range(len(rows)):
        l=lam[i]
        if l==0: continue
        for k,v in rows[i].items():
            out[k]=out.get(k,0)+l*v
    return {k:v for k,v in out.items() if v!=0}
def intersect(rows,keep,retlam=False):
    """basis of span(rows) ∩ span(monomials satisfying keep)"""
    keys=sorted(set(k for P in rows for k in P)); bad=[k for k in keys if not keep(k)]
    t=time.time(); Ker=pari.matker(mat(rows,bad)); print('matker',len(rows),len(bad),'->',len(Ker),round(time.time()-t,1),file=sys.stderr,flush=True)
    out=[comb(Ker[i],rows) for i in range(len(Ker))]
    if retlam: return [P for P in out if P],Ker
    return [P for P in out if P]
W2,LAM2=intersect(gens,lambda k: wt(*k)<=3,True)
print('W2 elems',len(W2),file=sys.stderr)
prods=[pm(P,*v) for P in W2 for v in V]
W3w=intersect(gens+prods,lambda k: wt(*k)<=3 and inwin(k,lo,win))
W2w=[P for P in W2 if all(inwin(k,lo,win) for k in P)]
W2w=intersect(gens,lambda k: wt(*k)<=3 and inwin(k,lo,win))
# K-RREF in L9 order
def lab(key):
    e,f=key; return tuple(i+lo for i in range(12) if e>>i&1),tuple(i+lo for i in range(12) if f>>i&1)
def fmt(m): return '*'.join(['a%d'%i for i in m[0]]+['b%d'%j for j in m[1]]) or '1'
keys=sorted(set(lab(k) for P in W3w+W2w for k in P),key=lambda m:(-(len(m[0])+len(m[1])),m)); keys_l=keys
pos={m:i for i,m in enumerate(keys)}
def M_(rows):
    M=pari.matrix(len(rows),len(keys))
    for r,P in enumerate(rows):
        for k,v in P.items(): M[r,pos[lab(k)]]=v
    return M
def rref(M):
    # row-reduce over rational functions: use matimage of transpose then manual RREF
    rows=[[M[i,j] for j in range(len(keys))] for i in range(M.nrows())]
    out=[];piv=[]
    for v in rows:
        for pc,rv in zip(piv,out):
            if v[pc]!=0: f=v[pc]; v=[a-f*b for a,b in zip(v,rv)]
        nz=[i for i in range(len(v)) if v[i]!=0]
        if not nz: continue
        pc=nz[0]; iv=1/v[pc]; v=[x*iv for x in v]
        for j in range(len(out)):
            if out[j][pc]!=0: f=out[j][pc]; out[j]=[a-f*b for a,b in zip(out[j],v)]
        out.append(v); piv.append(pc)
    return out,piv
B2,p2=rref(M_(W2w)); B3,p3=rref(M_(W2w+W3w))
new=[v for v,p in zip(B3,p3) if p not in p2]
res=dict(dimK_W2win=len(B2),dimK_W3win=len(B3),
  new_relations=[{fmt(keys[i]):str(pari.factor(pari.lift(x))) if pari.type(pari.lift(x))!='t_RFRAC' else str(pari.factor(pari.lift(x))) for i,x in enumerate(v) if x!=0} for v in new],
  new_raw=[{fmt(keys[i]):str(pari.lift(x)) for i,x in enumerate(v) if x!=0} for v in new],
  W3_rref=[{fmt(keys[i]):str(pari.lift(x)) for i,x in enumerate(v) if x!=0} for v in B3])
json.dump(res,open(sys.argv[1],'w'),indent=1)
print(len(B2),len(B3),len(new))
for r in res['new_raw']: print(r)

if CERT:
    # certificate denominators: R_i = sum lam_g g + sum mu (v*h), h = W2 basis (= LAM2 combos of gens)
    rows=gens+prods; keys=sorted(set(k for P in rows for k in P)); ci={k:i for i,k in enumerate(keys)}
    A=mat(rows,keys); dens=[]
    for P in W2: pass
    for l in LAM2:
        for x in l:
            if x!=0: dens.append(pari.denominator(pari.content(pari.lift(x)))); dens.append(pari.denominator(pari.lift(x)))
    inv={tuple(sorted(lab(k))):k for k in keys}
    labkey={}
    for k in keys: labkey[(lambda m: '*'.join(['a%d'%i for i in m[0]]+['b%d'%j for j in m[1]]) or '1')(lab(k))]=k
    certs=[]
    for v in new:
        t=pari.matrix(len(keys),1)
        for i,x in enumerate(v):
            if x!=0: t[ci[labkey[fmt(keys_l[i])]],0]=x
        sol=pari.matinverseimage(A,t)
        lam=[sol[i,0] for i in range(len(rows))]
        used=[i for i,x in enumerate(lam) if x!=0]
        for x in lam+v:
            if x!=0: dens.append(pari.denominator(pari.content(pari.lift(x)))); dens.append(pari.denominator(pari.lift(x)))
        pos_used=sorted(set(p for i in used for kk in rows[i] for p in [q for q in lab(kk)[0]+lab(kk)[1]]))
        certs.append(dict(nterms=len(used),positions=pos_used,ok=bool(len(sol)>0)))
    D=pari.lcm(dens); fz=pari.factor(D)
    res['certificate']=dict(certs=certs,lcm_denominators=str(D),factored=str(fz))
    json.dump(res,open(sys.argv[1],'w'),indent=1)
    print(res['certificate'])
