#!/usr/bin/env python3
"""CT242 independent Q37/Q51 source target checks on F103 CT240 norm-2LP model.

The two known discrete logs appear ONLY in final post-solve assertions.
No native timing, ECDLP lookup, independent external run, or PCC claim.
"""
import json
import ct240_f103_reduced_norm_lp as D

T, C = D.T, D.C
G, R, P = D.G, D.R, D.P
TESTS = (("Q37",(21,6),37),("Q51",(73,17),51))
BASES = (12,22)

def req(cond,msg):
    if not cond:raise ValueError(msg)

def get_logs(base_size,xs,points,stream):
    bp=[None]*base_size
    lp=[None]*(22-base_size)
    rank=0;used=0;accepted=0
    for cand,off,k,row,rhs in stream:
        outside=sum(v%R!=0 for v in row[base_size:])
        if outside>2:continue
        accepted+=1
        b_row=D.T.lp_insert(lp,row[:base_size],row[base_size:],rhs)
        if b_row is None:continue
        if D.T.reduce_row(bp,*b_row):
            rank+=1
            if rank==base_size:
                used=cand
                break
    req(rank==base_size,"incomplete base rank")
    fb=D.T.solve(bp)
    extra=D.T.recover_lp(lp,fb)
    req(all(x is not None for x in extra),"unrecovered excluded norm points")
    for x,log in zip(xs,fb+extra):
        req(T.emul(log,G)==points[x],"source norm factor logarithm wrong")
    return fb+extra,used,accepted

def left_norm(coeff,xs,points):
    out=None
    for x,k in zip(xs,coeff):
        if k%R:
            out=T.eadd(out,T.emul(k%R,points[x]))
    return out

def solve_public_target(q,xs,points,offsets,all_u,logs):
    req(T.emul(73,G)!=q,"fixed inverse3 masquerading as a target log")
    qdiv=T.dimage(q)
    gdiv=T.dimage(G)
    count=0
    identities=0
    for off_id,pts in enumerate(offsets):
        cur=qdiv
        off=dict.fromkeys(xs,0)
        for x,y in pts:
            cur=C.cantor(cur,T.dpoint((x,y)))
            off[pow(x,3,P)]+=1 if y<P-y else -1
        for a in range(R):
            if count==D.CAP:
                return None,count,identities,None
            count+=1
            signed=D.N.support(cur,all_u)
            if signed is not None:
                row=dict.fromkeys(xs,0)
                for u,sign in signed.items():
                    row[pow(u,3,P)]+=sign
                coeff=[row[x]-off[x] for x in xs]
                # Independently check the public-point norm identity;
                # this deliberately does NOT use the unknown log(Q).
                rhs=T.eadd(T.emul(3,q),T.emul(3*a,G))
                req(left_norm(coeff,xs,points)==rhs,
                    "public-Q elliptic norm relation failed")
                identities+=1
                total=sum(v*log for v,log in zip(coeff,logs))
                answer=(total*pow(3,-1,R)-a)%R
                req(T.emul(answer,G)==q,
                    "source E target log does not verify")
                req(C.sd(gdiv,answer)==qdiv,
                    "target Jacobian pullback does not verify")
                return answer,count,identities,off_id
            if a<R-1 and count<D.CAP:
                cur=C.cantor(cur,gdiv)
    return None,count,identities,None

def run():
    all_u,xs,points,offsets,stream,_=D.make_stream()
    panel=[]
    for base in BASES:
        logs,used,accepted=get_logs(base,xs,points,stream)
        for label,q,expected in TESTS:
            # No 'expected' value enters get_logs or solve_public_target.
            actual,tries,checks,off_id=solve_public_target(
                q,xs,points,offsets,all_u,logs)
            req(actual==expected,"post-solve known-answer failure")
            row=dict(schema="ct242-f103-noninverse-target-v1",target=label,
                public_point=list(q),base=base,relation_candidates=used,
                accepted_lp_relations=accepted,target_trials=tries,
                norm_point_identities=checks,target_offset=off_id,
                recovered_log=actual,
                complete=True,model="python_public_toy_no_target_log_oracle",
                cold_wall="not_measured",pcc="unverified")
            print("CT242_TARGET",json.dumps(row,sort_keys=True),flush=True)
            panel.append(row)
    req(len(panel)==4 and all(row["complete"] for row in panel),
        "incomplete target panel")
    print("CT242_COMPLETE 4 public source known-answer targets verified; no native cost",flush=True)

if __name__=="__main__":
    run()
