"""Independent integer-only source-Jacobian check of the p31 graph kernel."""
from pathlib import Path
import argparse,hashlib,json,time,struct
parser=argparse.ArgumentParser();parser.add_argument('data_root',type=Path);parser.add_argument('--kernel',type=Path);parser.add_argument('--output',type=Path);args=parser.parse_args()
ROOT=Path(__file__).resolve().parent;DATA=args.data_root;GRAPH=DATA/'Picard full graph 25k boundary';st=time.perf_counter()
source_bytes=(DATA/'Picard checkpoint collector/public_source_instance.json').read_bytes();source=json.loads(source_bytes);p=source['p'];ell=source['ell'];f=source['source_polynomial'];operations=0
def trim(a):
    a=[x%p for x in a]
    while a and not a[-1]:a.pop()
    return a
def add(a,b):return trim([(a[i] if i<len(a) else 0)+(b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])
def neg(a):return trim([-x for x in a])
def sub(a,b):return add(a,neg(b))
def scale(a,c):return trim([c*x for x in a])
def mul(a,b):
    if not a or not b:return []
    out=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):out[i+j]=(out[i+j]+x*y)%p
    return trim(out)
def divmod_poly(a,b):
    a=trim(a);b=trim(b);assert b;q=[0]*max(0,len(a)-len(b)+1);ib=pow(b[-1],-1,p)
    while a and len(a)>=len(b):
        k=len(a)-len(b);c=a[-1]*ib%p;q[k]=c
        for j,x in enumerate(b):a[j+k]=(a[j+k]-c*x)%p
        a=trim(a)
    return trim(q),a
def mod(a,b):return divmod_poly(a,b)[1]
def exact(a,b):
    q,r=divmod_poly(a,b);assert not r;return q
def monic(a):
    a=trim(a);assert a;return scale(a,pow(a[-1],-1,p))
def xgcd(a,b):
    r0,r1=trim(a),trim(b);s0,s1=[1],[];t0,t1=[],[1]
    while r1:
        q,r=divmod_poly(r0,r1);r0,r1=r1,r;s0,s1=s1,sub(s0,mul(q,s1));t0,t1=t1,sub(t0,mul(q,t1))
    assert r0;c=pow(r0[-1],-1,p);return scale(r0,c),scale(s0,c),scale(t0,c)
O=([1],[])
def checked(D):
    u,v=D;assert u==monic(u) and len(u)<=5 and len(v)<len(u) and not mod(sub(f,mul(v,v)),u);return D
def inverse(D):return (D[0],mod(neg(D[1]),D[0]))
def plus(D,E):
    global operations
    operations+=1;u1,v1=D;u2,v2=E;d1,e1,e2=xgcd(u1,u2);d,c1,c2=xgcd(d1,add(v1,v2))
    u=monic(exact(mul(u1,u2),mul(d,d)))
    numerator=add(add(mul(mul(c1,e1),mul(u1,v2)),mul(mul(c1,e2),mul(u2,v1))),mul(c2,add(mul(v1,v2),f)))
    v=mod(exact(numerator,d),u)
    while len(u)>5:
        newu=monic(exact(sub(f,mul(v,v)),u));v=mod(neg(v),newu);u=newu
    return checked((u,v))
def times(n,D):
    out=O
    for bit in bin(n)[2:]:
        out=plus(out,out)
        if bit=='1':out=plus(out,D)
    return out
B=checked((trim(source['base'][0]),trim(source['base'][1])));Q=checked((trim(source['target'][0]),trim(source['target'][1])));assert B!=O and Q!=O
for D in [B,Q]:assert plus(D,O)==D and plus(D,inverse(D))==O
assert times(ell,B)==O and times(ell,Q)==O
cfg_bytes=(DATA/'Picard native collector/input_2147483647.json').read_bytes();assert hashlib.sha256(cfg_bytes).hexdigest()=='935a53638bd9533bef255951836fd5ca3149f0a4f2a9d88ce2b4bbb4f06b7588';cfg=json.loads(cfg_bytes)
kernel_path=args.kernel or DATA/'Picard graph rank completion 20261123/graph_kernel_24999.json';kernel_bytes=kernel_path.read_bytes();kernel=json.loads(kernel_bytes);logs=kernel['kernel'];assert len(logs)==25000 and logs[0]==1
setup=json.loads((GRAPH/'setup.json').read_bytes());assert hashlib.sha256((GRAPH/'fb25k.bin').read_bytes()).hexdigest()==setup['fb25k_sha256']
wanted=[]
for group in cfg['split_input_points']:
    wanted.append([(x,min(y*pow(cfg['zeta'],i,p)%p for i in range(3)),y) for x,y in group])
lookup={}
with (GRAPH/'fb25k.bin').open('rb') as fh:
    for i in range(25000):lookup[struct.unpack('<II',fh.read(8))]=i
scores=[]
for group in wanted:
    value=0
    for x,canonical_y,y in group:
        j=next(j for j in range(3) if canonical_y*pow(cfg['zeta'],j,p)%p==y)
        value=(value+pow(cfg['mu'],j,ell)*logs[lookup[x,canonical_y]])%ell
    scores.append(value)
assert scores[0];raw=cfg['alpha']*scores[1]*pow(cfg['beta']*scores[0]%ell,-1,ell)%ell
attempts=[];answer=None;computed=None
for candidate in [raw,(-raw)%ell]:
    point=times(candidate,B);ok=point==Q;attempts.append(dict(scalar=candidate,matches_original_Q=ok))
    if ok:answer=candidate;computed=point;break
assert answer is not None
out=dict(p=p,ell=ell,recovered_scalar=answer,source_verified=True,method='pure Python integer-polynomial Cantor composition and reduction',
    Sage_imported=False,fixture_scalar_read=False,scalar_derived_from_returned_kernel=True,source_group_order_checks=True,
    computed_kB=[computed[0],computed[1]],public_Q=source['target'],candidate_checks=attempts,Cantor_additions=operations,
    public_source_sha256=hashlib.sha256(source_bytes).hexdigest(),kernel_result_sha256=hashlib.sha256(kernel_bytes).hexdigest(),
    source_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),seconds=time.perf_counter()-st,novelty_claim=False,full50x_claim=False)
if args.output:args.output.write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(out),flush=True)
