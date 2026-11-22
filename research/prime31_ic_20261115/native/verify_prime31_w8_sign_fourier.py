#!/usr/bin/env python3
"""Independent BigInt original-curve replay of all 8192 Prime31 sign determinants."""
import json,sys
from pathlib import Path
P=2147483647
A=847514846
B=2087444724
N=2147446831
W=(1,3,5,7,11,13,17,19)
G=(625037715,2100627251)
def inverse(a):return pow(a,P-2,P)
def add(p,q):
    if p is None:return q
    if q is None:return p
    x,y=p;xx,yy=q
    if x==xx and (y+yy)%P==0:return None
    if p==q:m=(3*x*x+A)*inverse(2*y)%P
    else:m=(yy-y)*inverse(xx-x)%P
    xr=(m*m-x-xx)%P
    return xr,(m*(x-xr)-y)%P
def mult(k,q):
    ans=None
    while k:
        if k&1:ans=add(ans,q)
        q=add(q,q);k>>=1
    return ans
def find_base():
    ans=[]
    for x in range(10000):
        rhs=(x*x*x+A*x+B)%P
        y=pow(rhs,(P+1)//4,P)
        if y and y*y%P==rhs:ans.append((x,min(y,P-y)))
        if len(ans)==31:break
    assert len(ans)==31
    return ans
def summation(pts,mask):
    out=None
    for i,(x,y) in enumerate(pts):
        if mask>>i&1:y=-y%P
        out=add(out,(x,y))
    return out
def determinant(pts,target,mask):
    r,s=target;matrix=[]
    for i,(x,y) in enumerate(pts):
        if mask>>i&1:y=-y%P
        matrix.append([((x-r)*pow(x,j,P))%P for j in range(4)]+
                      [((y+s)*pow(x,j,P))%P for j in range(4)])
    result=1
    for j in range(8):
        pivot=next((k for k in range(j,8) if matrix[k][j]),None)
        if pivot is None:return 0
        if pivot!=j:
            matrix[pivot],matrix[j]=matrix[j],matrix[pivot]
            result=-result
        v=matrix[j][j];result=result*v%P
        vinv=inverse(v)
        for k in range(j+1,8):
            scale=matrix[k][j]*vinv%P
            for h in range(j+1,8):
                matrix[k][h]=(matrix[k][h]-scale*matrix[j][h])%P
    return result%P
def verify(doc):
    assert doc['schema']=='prime31.w8.sign-fourier.v1'
    assert [doc[k] for k in ['p','a','b','n','cofactor']]==[P,A,B,N,1]
    assert tuple(doc['weights'])==W
    assert doc['source_crate_sha']=='1df714645aa683cf975bc76fcacb9842cb4780fd'
    assert doc['given_abscissae'] is True
    assert doc['generic_ec_tables_in_candidate'] is False
    assert doc['unknown_x_index_extractions']==0 and doc['unknown_public_ecdlp_solves']==0
    base=find_base()
    assert doc['base_x']==[x for x,y in base]
    slots=[[mult(w,q) for q in base] for w in W]
    assert len({q[0] for slot in slots for q in slot})==248
    assert len(doc['records'])==32
    totals={mode+'_fp_'+unit:0 for mode in ('direct','fourier') for unit in ('mul','inv','add')}
    total=0;zeros_total=0
    for t in range(16):
        pts=[slots[j][(3*t+7*j+j*j)%31] for j in range(8)]
        planted=sum(1<<j for j in range(8) if (t+3*j)%4>=2)
        r=summation(pts,planted)
        assert r is not None
        rr=add(r,G)
        assert rr is not None
        for arm,target in [('planted',r),('shifted_negative',rr)]:
            rec=doc['records'][2*t+int(arm=='shifted_negative')]
            assert rec['fixture']==t and rec['arm']==arm
            assert rec['points']==[list(p) for p in pts]
            assert rec['target']==list(target)
            assert rec['planted_sign_mask']==(planted if arm=='planted' else None)
            zs=[];checksum=0;samples={}
            for signs in range(256):
                residue=determinant(pts,target,signs)
                checksum=(checksum+(signs+1)*residue)%P
                if signs in (0,1,63,127,255):samples[signs]=residue
                group_hit=summation(pts,signs)==target
                assert (residue==0)==group_hit,(t,arm,signs)
                if residue==0:zs.append(signs)
                total+=1
            assert zs==([planted] if arm=='planted' else [])
            assert rec['zero_sign_masks']==zs
            assert rec['residue_checksum']==checksum
            assert rec['sample_residues']==[samples[i] for i in (0,1,63,127,255)]
            zeros_total+=len(zs)
            for key in totals:
                assert isinstance(rec[key],int) and rec[key]>=0
                totals[key]+=rec[key]
            assert rec['fourier_fp_inv']==0
            for mode in ('direct','fourier'):assert int(rec[mode+'_ns'])>=0
    assert doc['total_predicates_replayed']==total==8192
    assert doc['positive_sign_vectors']==zeros_total==16
    for key,val in totals.items():
        name=key.replace('_fp_','_total_fp_')
        assert doc[name]==val,(name,val)
    assert totals['direct_fp_inv']>0
    assert totals['fourier_fp_inv']==0
    assert totals['fourier_fp_mul']<totals['direct_fp_mul']
    for mode in ('direct','fourier'):
        assert int(doc[mode+'_total_ns'])==sum(int(r[mode+'_ns']) for r in doc['records'])
    print('PASS: 8192 exact prime31 determinant residues/truth checks, 16 planted positives, 16 shifted negatives')
    print('counted Fp multiply direct/fourier',totals['direct_fp_mul'],totals['fourier_fp_mul'])
    print('counted Fp inverse direct/fourier',totals['direct_fp_inv'],totals['fourier_fp_inv'])
if __name__=='__main__':
    if len(sys.argv)!=2:raise SystemExit('usage: verifier.py record.json')
    verify(json.loads(Path(sys.argv[1]).read_text()))
