#!/usr/bin/env python3
"""Independent Python BigInt point arithmetic and paired original-Prime31 target replay.
No import from Rust candidate. Raw producer measurements remain [pcc:unverified].
"""
import json, sys
P=2147483647
N=2147446831
A=847514846
B=2087444724
G=(625037715,2100627251)
MASK=(1<<64)-1
SEED=0x7738423442383231
TARGET_XOR=0x5441524745545338

def add(u,v):
    if u is None: return v
    if v is None: return u
    x,y=u; z,w=v
    if x==z and (y+w)%P==0: return None
    if x==z:
        top=(3*x*x+A)%P
        bot=(2*y)%P
    else:
        top=(w-y)%P
        bot=(z-x)%P
    assert bot, (u,v)
    s=(top*pow(bot,P-2,P))%P
    newx=(s*s-x-z)%P
    return newx, (s*(x-newx)-y)%P

def mul(k,q):
    res=None
    while k:
        if k&1: res=add(res,q)
        q=add(q,q)
        k>>=1
    return res

def splitmix64(state):
    state=(state+0x9e3779b97f4a7c15)&MASK
    z=state
    z=((z^(z>>30))*0xbf58476d1ce4e5b9)&MASK
    z=((z^(z>>27))*0x94d049bb133111eb)&MASK
    return state,z^(z>>31)

def target_scalar(t):
    state=(SEED^TARGET_XOR^((t*0xa0761d6478bd642f)&MASK)^0x7365637265740001)
    threshold=(((-N)&MASK)%N)
    while True:
        state,x=splitmix64(state)
        if x>=threshold and x%N!=0:
            return x%N

def verify(bsgs,old,tern):
    assert bsgs['schema']=='prime31.shared-signed-bsgs.v1'
    assert bsgs['generic_algorithm'] is True
    assert bsgs['genuine_nongeneric_rr9_extractors']==0
    assert bsgs['curve']==dict(p=P,n=N,a=A,b=B,g=list(G),cofactor=1)
    assert bsgs['m']==32768 and bsgs['step']==65537
    assert bsgs['table_entries']==32768 and bsgs['last_giant_index']==32767
    n=bsgs['target_count']
    assert n==1024 and bsgs['verified_target_count']==1024
    solutions=bsgs['recovered_targets']
    assert len(solutions)==n
    total_probes=0
    for i,s in enumerate(solutions):
        assert s['target_index']==i
        q=tuple(s['public_q'])
        d=s['log']
        assert 0<=d<N
        assert d==target_scalar(i)==s['test_secret']
        assert (q[1]*q[1] - (q[0]**3+A*q[0]+B))%P==0
        assert mul(d,G)==q, ('invalid recovered log',i)
        gp=s['giant_probes']
        assert 1<=gp<=32768
        total_probes+=gp
    assert total_probes==bsgs['total_giant_probes']
    counts=bsgs['counts']
    assert counts['total_ec_additions']==counts['setup_ec_additions']+counts['online_ec_additions']
    assert counts['total_fp_multiplies']>counts['total_fp_inversions']>0
    for name,w in [('old',old),('ternary',tern)]:
        assert w['verified_target_count']==n and w['rank']==31
        assert w['generic_four_plus_four_grader_used'] and w['genuine_nongeneric_rr9_extractors']==0
        assert len(w['recovered_targets'])==n
        for i,(bs,ws) in enumerate(zip(solutions,w['recovered_targets'])):
            assert bs['public_q']==ws['public_q'],('different Q',name,i)
            assert bs['log']==ws['log'],('different log',name,i)
    print(json.dumps({'independent_python_verified':n,'target_stream_identical_across_all_arms':True,
       'signed_bsgs_table_entries':bsgs['table_entries'],
       'total_giant_probes':total_probes,
       'status':'original-curve BigInt replay, pcc unverified'},sort_keys=True))

if __name__=='__main__':
    if len(sys.argv)!=4:
        sys.exit('usage: verify_prime31_shared_signed_bsgs.py bsgs.json weight-old.json weight-ternary.json')
    bsgs,old,tern=[json.load(open(path,encoding='utf8')) for path in sys.argv[1:]]
    verify(bsgs,old,tern)
