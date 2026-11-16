#!/usr/bin/env sage -python
"""Resumable geometric classification of every quintic-node N4 survivor."""
from sage.all import GF, PolynomialRing, ProjectiveSpace, Curve, matrix
from pathlib import Path
import hashlib
import json
import sys
import time

output=Path(sys.argv[1])
limit=int(sys.argv[2]) if len(sys.argv)>2 else None
here=Path(__file__).parent
source_hash=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
inputs=here.parent/'ghs_trace_minus14_quintic_nodes_20261116'
input_paths=[inputs/('FULL_%d.json'%c) for c in (0,1)]
input_hashes=[hashlib.sha256(p.read_bytes()).hexdigest() for p in input_paths]
data=[json.loads(p.read_text()) for p in input_paths]
assert [d['counts']['N4'] for d in data]==[947,1024]
assert [d['counts']['ordinary_N4'] for d in data]==[916,994]

F3=GF(3)
K=GF(3**5,name="zeta");zeta=K.gen()
R=PolynomialRing(F3,names=("x","y","z"));x,y,z=R.gens()
P=ProjectiveSpace(F3,2,names=("x","y","z"))
mons=[x**i*y**j*z**(6-i-j) for i in range(7) for j in range(7-i)]
ext2=GF(9,name="u");ext3=GF(27,name="v")

def coords(a):
    w=[int(v) for v in a.polynomial()]
    return w+[0]*(5-len(w))

def basis_for(case):
    p=(K(1),zeta**(127 if case==0 else 218),zeta**(143 if case==0 else 72))
    rows=[]
    for polys in (mons,[m.derivative(y) for m in mons],[m.derivative(z) for m in mons]):
        w=[coords(q.change_ring(K)(*p)) for q in polys]
        rows.extend([[v[j] for v in w] for j in range(5)])
    ker=matrix(F3,rows).right_kernel().basis()
    assert len(ker)==13
    return [sum((v[i]*mons[i] for i in range(28)),R(0)) for v in ker]

bases=[basis_for(0),basis_for(1)]
already={}
if output.exists():
    for line in output.read_text().splitlines():
        rec=json.loads(line)
        assert rec['source_sha256']==source_hash
        assert rec['input_sha256']==input_hashes[rec['case']]
        key=(rec['case'],rec['index'])
        assert key not in already
        already[key]=rec

start=time.perf_counter();processed=0
with output.open('a') as out:
    for case in (0,1):
        basis=bases[case]
        for index,candidate in enumerate(data[case]['n4_candidates']):
            if limit is not None and processed>=limit:break
            processed+=1
            if (case,index) in already:continue
            rec={'schema':'ghs.quintic-nodes-all-traces-classification/v1',
                 'source_sha256':source_hash,'input_sha256':input_hashes[case],
                 'case':case,'index':index,'coefficients':candidate['coefficients'],
                 'N5':candidate.get('N5'),'status':None}
            if 'N5' not in candidate:
                rec['status']='nonordinary_intended_node'
            elif (244-candidate['N5'])%5!=0 or abs((244-candidate['N5'])//5)>31:
                rec['status']='outside_genus5_weil_bound'
            else:
                t=(244-candidate['N5'])//5
                rec['trace_t']=t
                c=candidate['coefficients']
                f=sum((F3(c[i])*basis[i] for i in range(13)),R(0))
                C=Curve(P.subscheme([f]))
                irreducible=bool(C.is_irreducible())
                rec['irreducible_F3']=irreducible
                if irreducible:
                    genus=int(C.geometric_genus())
                    rec['geometric_genus']=genus
                    if genus==5:
                        fact2=f.change_ring(ext2).factor()
                        fact3=f.change_ring(ext3).factor()
                        absolute=(len(fact2)==len(fact3)==1 and
                                  fact2[0][1]==fact3[0][1]==1)
                        rec['absolutely_irreducible']=bool(absolute)
                        if absolute:
                            rec['status']='genus5_carrier'
                            rec['sextic']=str(f)
                        else:
                            rec['status']='geometrically_reducible'
                    else:rec['status']='different_genus'
                else:rec['status']='reducible_F3'
            out.write(json.dumps(rec,sort_keys=True)+'\n')
            out.flush()
            if len(already)+processed<=20 or (len(already)+processed)%100==0:
                print('classified=%d case=%d index=%d status=%s elapsed=%.1f'%
                      (len(already)+processed,case,index,rec['status'],time.perf_counter()-start),
                      file=sys.stderr,flush=True)
        if limit is not None and processed>=limit:break
