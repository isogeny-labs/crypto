"""Nonpromoted native joint-moment search diagnostic; no original-root output."""
from pathlib import Path
import gzip,hashlib,itertools,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.advance.field_circuit import BinaryFieldCircuit
from research.alternating_rank_v1.cuts import IncrementalCircuitSync,compile_symplectic_factor,symplectic_gram_rows,alternating_elimination
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
sys.path.insert(0,str(R/'research/advance/deps_5_11_21'))
import pycryptosat
assert pycryptosat.__version__=='5.11.21'
def numeric(f,H):
    e=[0]+list(reversed(H[:5]));s=[1]
    for k in range(1,10):
        v=0
        for j in range(1,min(5,k-1)+1):v^=f.mul(e[j],s[k-j])
        if k<=5 and k%2:v^=e[k]
        s.append(v)
    return [s[k]^f.pow(e[1],k)for k in[3,5,9]]
def profile(f,d,m):
    piv={}
    for i,j in itertools.combinations(range(d),2):
        v=sum((f.mul(f.pow(1<<i,q),1<<j)^f.mul(1<<i,f.pow(1<<j,q)))<<(f.degree*t)for t,q in enumerate([2,4,8][:m]))
        while v:
            p=v.bit_length()-1
            if p in piv:v^=piv[p]
            else:piv[p]=v;break
    return piv
def residue(v,piv):
    for p in sorted(piv,reverse=True):
        if v>>p&1:v^=piv[p]
    return v
def moments(c,H):
    a,b,e,d,z=reversed(H[:5]);a2=c.square(a);a4=c.square(a2)
    g3=c.add(c.mul(a,b),e);p3=c.add(c.mul(a,a2),g3)
    g5=c.add(c.add(c.mul(b,p3),c.mul(e,a2)),c.add(c.mul(d,a),z))
    return [g3,g5],(a,b,e,d,z,a2,a4,p3,g3,g5)
def ninth(c,state):
    a,b,e,d,z,a2,a4,p3,g3,g5=state
    return c.add(c.add(c.mul(g3,c.square(g3)),c.mul(c.add(c.add(a4,c.square(b)),d),g5)),c.mul(b,c.add(c.mul(d,p3),c.mul(z,a2))))
def assert_image(c,words,piv):
    bits=[bit for w in words for bit in w]
    for p in sorted(piv,reverse=True):
        expr=bits[p];v=piv[p]
        while v:
            low=v&-v;j=low.bit_length()-1;bits[j]=c.xor(bits[j],expr);v^=low
    for j,bit in enumerate(bits):
        if j not in piv:c.assert_zero(bit)
def build(f,base,cols,d,full=True):
    c=BinaryFieldCircuit(f,141 if len(cols)==69 else len(cols),max_gates=200000,max_variables=300000,max_xor_literals=50000000)
    H=[tuple(c.affine(frozenset(i+1 for i,col in enumerate(cols)if col[j]>>b&1),bool(base[j]>>b&1))for b in range(f.degree))for j in range(6)]
    w,state=moments(c,H);assert_image(c,w,profile(f,d,2))
    if full:w.append(ninth(c,state));assert_image(c,w,profile(f,d,3))
    return c,H,w,state
def value(base,cols,t):
    out=list(base)
    for i,col in enumerate(cols):
        if t>>i&1:out=[a^b for a,b in zip(out,col)]
    return out
def controls():
    out=[]
    for n,mod in[(2,7),(3,11),(4,19)]:
        f=BinaryField(n,mod);base=[0,0,0,0,0,1]
        cols=[[((i+1)*(j+2))%f.order if j<4 else 0 for j in range(6)]for i in range(4)]
        c,H,w,_=build(f,base,cols,min(n,2));piv=profile(f,min(n,2),3);expected=[]
        for t in range(16):
            h=value(base,cols,t);nw=numeric(f,h);wires=c.evaluate_wires(t)
            assert [c.evaluate_element(x,wires)for x in w]==nw
            if residue(sum(v<<(n*i)for i,v in enumerate(nw)),piv)==0:expected.append(t)
        solver=pycryptosat.Solver(threads=1);IncrementalCircuitSync(solver,c).sync();seen=[]
        while True:
            sat,model=solver.solve(time_limit=1)
            assert sat is not None,'tiny control timeout'
            if not sat:break
            t=sum(int(model[i+1])<<i for i in range(4));assert t not in seen;seen.append(t)
            solver.add_clause([-(i+1)if t>>i&1 else i+1 for i in range(4)])
        assert sorted(seen)==expected and 0 in seen
        out.append(dict(n=n,assignments=16,models=len(seen),complete=True))
    return out
def actual():
    cert=json.loads(gzip.decompress((R/'research/joint_cubic_20261115_v15/raw.json.gz').read_bytes()))['out']['actual']['family_certificate']
    f=BinaryField(83,0x800000000200000000007);base=cert['base'];cols=cert['columns'];assert len(cols)==69
    begin=time.monotonic();c,H,w,state=build(f,base,cols,18,False);solver=pycryptosat.Solver(threads=1);sync=IncrementalCircuitSync(solver,c);sync.sync()
    out=dict(parameter_bits=69,factor_witness_bits=72,stages=[],setup_seconds=time.monotonic()-begin)
    for full,cap in[(False,1),(True,10)]:
        begin=time.monotonic()
        if full:
            w.append(ninth(c,state));assert_image(c,w,profile(f,18,3))
            # Track generator coordinates during elimination, then invert the injective image.
            piv={}
            for index,(i,j) in enumerate(itertools.combinations(range(18),2)):
                v=sum((f.mul(f.pow(1<<i,q),1<<j)^f.mul(1<<i,f.pow(1<<j,q)))<<(83*t)for t,q in enumerate([2,4,8]));coord=1<<index
                while v:
                    p=v.bit_length()-1
                    if p in piv:v^=piv[p][0];coord^=piv[p][1]
                    else:piv[p]=(v,coord);break
            assert len(piv)==153
            packed=[bit for word in w for bit in word];inverse=[]
            for bit in range(249):
                v=1<<bit;coord=0
                for p in sorted(piv,reverse=True):
                    if v>>p&1:v^=piv[p][0];coord^=piv[p][1]
                inverse.append(coord)
            pairbits=[c.xor_many(packed[j]for j,q in enumerate(inverse)if q>>i&1)for i in range(153)]
            compile_symplectic_factor(c,pairbits,18,offset=69,assert_zero=True);sync.sync()
        setup=time.monotonic()-begin;piv=profile(f,18,3 if full else 2);samples=[];deadline=time.monotonic()+cap;result='model_cap'
        for _ in range(8):
            left=deadline-time.monotonic()
            if left<=0:result='time_cap';break
            sat,model=solver.solve(time_limit=left)
            if sat is None:result='native_unknown';break
            if not sat:result='conditional_moment_unsat';break
            t=sum(int(model[i+1])<<i for i in range(69));h=value(base,cols,t);nw=numeric(f,h);wires=c.evaluate_wires(sum(int(model[i+1])<<i for i in range(c.n_primary)))
            assert [c.evaluate_element(x,wires)for x in w]==nw[:len(w)]
            assert residue(sum(v<<(83*i)for i,v in enumerate(nw[:len(w)])),piv)==0
            samples.append(dict(parameter=t,H=h,moments=nw))
            # Stage-local diagnostic: do not block preliminary models before adding g9.
            if not full:result='first_model';break
            solver.add_clause([-(i+1)if t>>i&1 else i+1 for i in range(69)])
        out['stages'].append(dict(full=full,cap_seconds=cap,compile_seconds=setup,elapsed_seconds=time.monotonic()-begin,result=result,models=samples,circuit=c.statistics()))
    return out
def rank_controls():
    # All 4-row factors: independent binary Gaussian rank oracle versus alternating elimination.
    count=0
    for factor in itertools.product(range(16),repeat=4):
        rows=symplectic_gram_rows(factor);piv={}
        for row in rows:
            while row:
                p=row.bit_length()-1
                if p in piv:row^=piv[p]
                else:piv[p]=row;break
        assert len(piv)==alternating_elimination(rows).rank and len(piv)<=4;count+=1
    return dict(all_four_row_factors=count)
def main():
    out={};error=None
    try:out['controls']=controls();out['rank_controls']=rank_controls();out['actual']=actual()
    except Exception as exc:error=type(exc).__name__+': '+str(exc)
    raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();blob=gzip.compress(raw,mtime=0)
    assert len(raw)<128*1024 and len(blob)<32*1024
    with(P/'raw.json.gz').open('xb')as stream:stream.write(blob)
    ev=dict(out=out,error=error,peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,worker_seconds=time.monotonic()-START,raw_sha256=hashlib.sha256(blob).hexdigest(),external_reviewer=False,complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False)
    result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='inconclusive',summary='Changed rank-factor native moment diagnostic; no original-root completion.',evidence=ev,next_actions=['If native search stalls, change to explicit rank factor or a quadratic-only solver with certified degree-specific constraints; do not repeat the identical capped search.'])
    text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84
    print(text)
if __name__=='__main__':main()
