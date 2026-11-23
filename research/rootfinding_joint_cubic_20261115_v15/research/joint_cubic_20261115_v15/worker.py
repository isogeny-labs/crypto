"""Exact bounded Boolean moment compilation on the recorded69-bit family."""
from pathlib import Path
import gzip,hashlib,itertools,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.quartic_cell_20261114_v1 import worker as norm
from research.gold_projection_20261115_v14 import worker as linear

P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();CHECKS=0;PAIRS=0
MAX_TERMS=54810
def check(value,why):
    global CHECKS
    CHECKS+=1
    assert CHECKS<=200000 and time.monotonic()-START<80,'work cap'
    assert value,why

class Ring:
    def __init__(self,f):self.f=f
    def add(self,*polys):
        out={}
        for p in polys:
            for m,c in p.items():
                v=out.get(m,0)^c
                if v:out[m]=v
                else:out.pop(m,None)
        check(len(out)<=MAX_TERMS,'live term cap');return out
    def scale(self,p,c):return {m:self.f.mul(v,c)for m,v in p.items()}if c else {}
    def square(self,p):return {m:self.f.square(c)for m,c in p.items()}
    def mul(self,a,b):
        global PAIRS
        out={}
        for m,c in a.items():
            for n,d in b.items():
                PAIRS+=1
                assert PAIRS<=1000000,'pair-work cap'
                key=m|n;value=out.get(key,0)^self.f.mul(c,d)
                if value:out[key]=value
                else:out.pop(key,None)
            check(len(out)<=MAX_TERMS,'live multiplication terms')
        return out

def polynomial_hash(p):
    h=hashlib.sha256()
    for m,c in sorted(p.items()):h.update(m.to_bytes(9,'little'));h.update(c.to_bytes(32,'little'))
    return h.hexdigest()
def evaluate(p,t):return linear.xor(c for m,c in p.items()if m&t==m)
def info(p):return dict(terms=len(p),degree=max((m.bit_count()for m in p),default=0),sha256=polynomial_hash(p))

def compile_moments(f,H):
    R=Ring(f);a=H[4].get(0,0)
    check(all(m==0 for m in H[4]),'centroid fixed')
    e2,e3,e4,e5=H[3],H[2],H[1],H[0];a2=f.square(a);a4=f.square(a2)
    g3=R.add(R.scale(e2,a),e3);p3=R.add(g3,{0:f.mul(a,a2)})
    g5=R.add(R.mul(e2,p3),R.scale(e3,a2),R.scale(e4,a),e5)
    g9=R.add(R.mul(R.square(g3),g3),
             R.mul(R.add({0:a4},R.square(e2),e4),g5),
             R.mul(e2,R.add(R.mul(e4,p3),R.scale(e5,a2))))
    check(info(g3)['degree']<=1 and info(g5)['degree']<=2 and info(g9)['degree']<=3,'formal degree consequences')
    return [g3,g5,g9]

def newton(f,H):
    e=[0]+list(reversed(H[:5]));powers=[1]
    for k in range(1,10):
        value=linear.xor(f.mul(e[j],powers[k-j])for j in range(1,min(5,k-1)+1))
        if k<=5 and k%2:value^=e[k]
        powers.append(value)
    return [powers[k]^f.pow(e[1],k)for k in[3,5,9]]

def controls():
    cases=0
    for n,mod in[(2,7),(3,11),(4,19)]:
        f=BinaryField(n,mod)
        # Independent Newton recursion, optimized straight-line expression,
        # and literal sums of planted root powers.
        for bag in itertools.combinations_with_replacement(range(1<<n),5):
            H=[1]
            for root in bag:H=norm.pmul(H,[root,1],f)
            e1,e2,e3,e4,e5=reversed(H[:5]);g3=f.mul(e1,e2)^e3;p3=f.pow(e1,3)^g3
            g5=f.mul(e2,p3)^f.mul(e3,f.square(e1))^f.mul(e4,e1)^e5
            g9=f.pow(g3,3)^f.mul(f.square(f.square(e1))^f.square(e2)^e4,g5)^f.mul(e2,f.mul(e4,p3)^f.mul(e5,f.square(e1)))
            values=[g3,g5,g9];expected=[linear.xor(f.pow(x,k)for x in bag)^f.pow(e1,k)for k in[3,5,9]]
            check(values==expected==newton(f,H),'independent full tiny source moments');cases+=1
        # Complete truth-table oracle for an affine four-selector family.
        H=[{0:(j+1)%f.order,1:1,2:(j+2)%f.order,4:(2*j+1)%f.order,8:(j*j+1)%f.order}for j in range(4)]
        H+=[{0:1},{0:1}];polys=compile_moments(f,H)
        for t in range(16):check([evaluate(p,t)for p in polys]==newton(f,[evaluate(p,t)for p in H]),'tiny Boolean idempotent polynomial evaluation')
    check(cases==16352,'all tiny multiset corpus');return dict(source_multisets=cases,complete_four_selector_assignments=48)

def affine_family(f,data):
    oldpath=ROOT/'research/full_quintic_block_20261115_v9/raw.json.gz';goldpath=ROOT/'research/gold_projection_20261115_v14/raw.json.gz'
    check(hashlib.sha256(oldpath.read_bytes()).hexdigest()=='e79604203f3ca5956d55b71d3dcb16d269b8514cd24e32cba8d75fcd914de7bb','old evidence identity')
    check(hashlib.sha256(goldpath.read_bytes()).hexdigest()=='26e5f78fab406df805c958f7be13a3c7a9527dfedc26524504c5c93855a17f83','Gold69 identity')
    base=json.loads(gzip.decompress(oldpath.read_bytes()))['out']['profile']['proofs']['baseline166']
    gold=json.loads(gzip.decompress(goldpath.read_bytes()))['result']['actual']['candidate']
    check(gold['rank']==4 and gold['consistent']and len(gold['basis'])==69,'certified complete69 coordinates')
    lift=lambda bits:linear.xor(v for i,v in enumerate(base['basis'])if bits>>i&1)
    raw0=base['particular']^lift(gold['particular']);directions=[lift(v)for v in gold['basis']]
    A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400
    literal=lambda raw:norm.literal(f,A,B,z,y,h,0,raw&((1<<83)-1),raw>>83)
    H0=literal(raw0);zero=literal(0);columns=[norm.padd(literal(v),zero)for v in directions]
    check(len(linear.pivots(directions))==69,'mapped independent directions')
    coeff=[{0:H0[j]}if H0[j]else {}for j in range(6)]
    for i,col in enumerate(columns):
        for j,c in enumerate(col):
            if c:coeff[j][1<<i]=c
    return coeff,dict(base=H0,columns=columns,raw_particular=raw0,raw_basis=directions),literal

def actual():
    begin=time.monotonic();data=json.loads((ROOT/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text())
    n=data['extension_field']['degree'];mod=int(data['extension_field']['modulus_hex'],16);check(n==83 and mod==0x800000000200000000007,'actual field')
    f=BinaryField(n,mod);H,certificate,literal=affine_family(f,data);family_seconds=time.monotonic()-begin
    begin=time.monotonic();moments=compile_moments(f,H);compile_seconds=time.monotonic()-begin
    basis=[1<<i for i in range(18)];generators=[]
    for i in range(18):
        for j in range(i+1,18):generators.append(sum((f.mul(f.pow(basis[i],q),basis[j])^f.mul(basis[i],f.pow(basis[j],q)))<<(83*t)for t,q in enumerate([2,4,8])))
    image=linear.pivots(generators);pair_rank=len(linear.pivots(v&((1<<166)-1)for v in generators));g3rank=len(linear.pivots(v&((1<<83)-1)for v in generators))
    check(len(image)==153 and g3rank==48,'actual joint image injective andGold3 rank')
    free=[i for i in range(249)if i not in image]
    def quotient(v):
        r=linear.residue(v,image);return sum(((r>>j)&1)<<i for i,j in enumerate(free))
    begin=time.monotonic();syndrome={}
    for mask in set().union(*(p.keys()for p in moments)):
        value=quotient(sum(p.get(mask,0)<<(83*i)for i,p in enumerate(moments)))
        if value:syndrome[mask]=value
    projection_seconds=time.monotonic()-begin
    allspan=linear.pivots(syndrome.values());nonlinear=linear.pivots(v for m,v in syndrome.items()if m.bit_count()>=2);cubic=linear.pivots(v for m,v in syndrome.items()if m.bit_count()>=3)
    check(len(allspan)<=61,'Gold3 already removed35joint conditions')
    def annihilator(span):return linear.row_oracle([(v,0)for v in span.values()],96)['basis']
    affine_duals=annihilator(nonlinear);quadratic_duals=annihilator(cubic)
    affine_rows=[(sum(((lam&syndrome.get(1<<i,0)).bit_count()&1)<<i for i in range(69)),(lam&syndrome.get(0,0)).bit_count()&1)for lam in affine_duals]
    affine_sol=linear.row_oracle(affine_rows,69)
    state=0x67723539616e6631;mask64=(1<<64)-1;tests=[0]+[1<<i for i in range(69)]
    def word():
        nonlocal state
        state^=(state<<13)&mask64;state^=state>>7;state^=(state<<17)&mask64;state &=mask64;return state
    while len(tests)<128:tests.append((word()|(word()<<64))&((1<<69)-1))
    for index,t in enumerate(tests):
        polynomial=[evaluate(p,t)for p in H];values=[evaluate(p,t)for p in moments]
        check(values==newton(f,polynomial),'actual ANF/direct Newton crosscheck')
        check(evaluate(syndrome,t)==quotient(sum(v<<(83*i)for i,v in enumerate(values))),'actual syndrome evaluation')
        if index<16:check(polynomial==literal(certificate['raw_particular']^linear.xor(v for i,v in enumerate(certificate['raw_basis'])if t>>i&1)),'mixed literal norm composition')
    return dict(parameter_bits=69,family_certificate=certificate,joint_generators=generators,image_rank=len(image),pair35_image_rank=pair_rank,g3_image_rank=g3rank,quotient_free_bits=free,
                moment_polynomials=[info(p)for p in moments],syndrome=info(syndrome),coefficient_span_rank=len(allspan),nonlinear_span_rank=len(nonlinear),cubic_span_rank=len(cubic),
                nonzero_affine_consequence_dimension=len(allspan)-len(nonlinear),nonzero_quadratic_consequence_dimension=len(allspan)-len(cubic),
                affine_duals=affine_duals,quadratic_duals=quadratic_duals,affine_rows=affine_rows,affine_solution=affine_sol,nonlinear_span=list(nonlinear.values()),cubic_span=list(cubic.values()),
                evaluation_points=tests,family_seconds=family_seconds,compile_seconds=compile_seconds,projection_seconds=projection_seconds,
                reconstruction='Exact complete ANF defined by affineHcertificate, frozenNewtonstraightlineformulas andjointgenerators; hashescommitexpandedpolys; no omittedconstraints or candidate-root enumeration.')

def compact(actual):return {k:v for k,v in actual.items()if k in['parameter_bits','image_rank','pair35_image_rank','g3_image_rank','moment_polynomials','syndrome','coefficient_span_rank','nonlinear_span_rank','cubic_span_rank','nonzero_affine_consequence_dimension','nonzero_quadratic_consequence_dimension','family_seconds','compile_seconds','projection_seconds']}
def main():
    out={};status='passed';error=None
    try:out['controls']=controls();out['actual']=actual()
    except Exception as exc:status='inconclusive';error=type(exc).__name__+': '+str(exc)
    encoded=json.dumps(dict(out=out,error=error,checks=CHECKS,multiply_pairs=PAIRS),sort_keys=True,separators=(',',':')).encode();blob=gzip.compress(encoded,mtime=0)
    if len(encoded)>128*1024 or len(blob)>32*1024:status='inconclusive';error='raw cap';out={};encoded=json.dumps(dict(error=error,complete=False)).encode();blob=gzip.compress(encoded,mtime=0)
    with(P/'raw.json.gz').open('xb')as stream:stream.write(blob)
    evidence=dict(controls=out.get('controls'),actual=compact(out.get('actual',{})),error=error,multiply_pairs=PAIRS,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),raw_payload_bytes=len(encoded),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_symbolic_diagnostic=status=='passed',external_reviewer=False,scope='sole-model preparation, independent Newton/source-sum oracle algorithms; no root search or solver promotion',complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False)
    result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status=status,summary='Exact bounded cubic joint-moment compilation on the certified69-bit family; not a root solver or speed claim.',evidence=evidence,next_actions=['Use exactdegree/span result to choose SAT/degree-specific solve or a changed invariant; no assumption that fewer coordinates or a cubic representation implies a fast all-roots algorithm.'])
    text=json.dumps(result,separators=(',',':'))
    if time.monotonic()-START>=84:result['status']='inconclusive';evidence['complete_symbolic_diagnostic']=False;evidence['actual']={};evidence['error']='late report';text=json.dumps(result,separators=(',',':'))
    assert len(text.encode())<=8192 and time.monotonic()-START<88,'final report cap';print(text)
if __name__=='__main__':main()
