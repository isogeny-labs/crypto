#!/usr/bin/env python3
"""Independent point-law and direct-polynomial audits for the portable controls."""
import hashlib
import itertools
import json
import os
from pathlib import Path
import random
import time
from controls import add, scalar, neg, quotient_pairs, s3_scalar, solve_pair, solve_pair_cpp, verify_witness

ROOT = Path(__file__).resolve().parent


def source_dickson_base(depth):
    """Deterministic Base::build algorithm from captured native_lib.rs."""
    p,a,b=2147483647,847514846,2087444724
    for seed in range(2,10000):
        c=seed
        for _ in range(depth):
            c=(c*c-2)%p
        if c in (2,p-2):
            continue
        xs=[c]
        for _ in range(depth):
            nxt=[]
            for y in xs:
                x=pow((y+2)%p,(p+1)//4,p)
                if x*x%p==(y+2)%p:
                    nxt.extend([x,-x%p])
            xs=sorted(set(nxt))
        if len(xs)!=1<<depth:
            continue
        points=[]
        for x in xs:
            y=pow((x*x*x+a*x+b)%p,(p+1)//4,p)
            if y and y*y%p==(x*x*x+a*x+b)%p:
                points.append((x,min(y,p-y)))
        if len(points)>=2:
            return p,a,b,points,c
    raise AssertionError("no base")


def actual_pairs(p,a,points0,points1,target):
    """Unbatched affine group-law comparison, independent of S3 algebra."""
    out=[]
    for i,u in enumerate(points0):
        for j,v in enumerate(points1):
            for si,sj in itertools.product((1,-1),repeat=2):
                if add(p,a,u if si==1 else neg(p,u),v if sj==1 else neg(p,v))==target:
                    out.append((i,j,si,sj))
    return out


def direct_coordinate_pairs(p,a,b,xs,ys,t):
    return [(i,j) for i,x in enumerate(xs) for j,y in enumerate(ys)
            if int(s3_scalar(p,a,b,x,t)(y))==0]


def replay_m2():
    rows=[]
    for depth in (6,8):
        p,a,b,points,terminal=source_dickson_base(depth)
        xs=[q[0] for q in points]
        G=(625037715,2100627251)
        targets=[("planted_sum",add(p,a,points[3],points[9])),
                 ("planted_double",add(p,a,points[4],points[4])),
                 ("planted_difference",add(p,a,points[0],neg(p,points[7])))]
        if depth==8:
            targets.append(("source_B135_sum_P0_P14",add(p,a,points[0],points[14])))
        targets += [(f"public_scalar_{k}",scalar(p,a,k,G)) for k in (1,17,137,20261115)]
        targets += [(f"exceptional_x_target_{j}",points[j]) for j in range(3)]
        for name,target in targets:
            assert target is not None
            q=quotient_pairs(p,a,b,xs,xs,target[0])
            algebra=direct_coordinate_pairs(p,a,b,xs,xs,target[0])
            assert q["pairs"]==algebra,(depth,name,q["pairs"],algebra)
            lifts=actual_pairs(p,a,points,points,target)
            assert sorted({(i,j) for i,j,_,_ in lifts})==algebra,(depth,name,lifts,algebra)
            rows.append({"depth":depth,"B":len(points),"terminal":terminal,
                "name":name,"target":target,"coordinate_pairs":q["pairs"],
                "signed_lifts":lifts,"quotient":q})
    return rows


def random_rectangular_audit():
    rng=random.Random(9157309)
    count=0
    for p,a,b in ((101,2,3),(103,2,7),(127,1,0),(131,0,1)):
        assert (4*a*a*a+27*b*b)%p
        points=[(x,y) for x in range(p) for y in range(p)
                if y*y%p==(x*x*x+a*x+b)%p]
        byx={x:(x,y) for x,y in points}
        coords=list(byx)
        for case in range(50):
            nx,ny=rng.randrange(1,9),rng.randrange(1,9)
            xs=rng.sample(coords,nx);ys=rng.sample(coords,ny)
            t=rng.choice(coords)
            if case%3==0:
                t=rng.choice(xs)
            target=byx[t]
            q=quotient_pairs(p,a,b,xs,ys,t)
            direct=direct_coordinate_pairs(p,a,b,xs,ys,t)
            assert q["pairs"]==direct,(p,xs,ys,t,q,direct)
            group=actual_pairs(p,a,[byx[x] for x in xs],[byx[y] for y in ys],target)
            assert sorted({(i,j) for i,j,_,_ in group})==direct
            count+=1
    return {"cases":count,"coordinate_pool_shapes":"rectangular; overlap allowed",
            "exceptions":"x=t, ordinates zero/two-torsion, group infinity tested",
            "result":"all quotient/direct polynomial/actual point-law candidates identical"}


def audit_pair_controls():
    fixture_dir=Path("/workspace/ecdlp_research/prime_fixtures")
    rows=[]
    for path in sorted(fixture_dir.glob("b*_natural_s*.json")):
        if path.name.endswith(".validation.json"):
            continue
        fixture=json.loads(path.read_text())
        got=solve_pair(fixture)
        native=solve_pair_cpp(fixture)
        assert native["status"]==got["status"] and native["witness"]==got["witness"],path.name
        assert native["recovery"]==got["recovery"],path.name
        for key in ("pair_positions_scanned","signed_pair_candidates","x_hits","batch_rows",
                    "batch_inputs","fp_multiplications_explicit_search","modular_inversions_search"):
            assert native["counters"][key]==got["counters"][key],(path.name,key)
        validation=json.loads(path.with_suffix(".validation.json").read_text())
        assert (got["status"]=="relation")==validation["signed_zero_sum_exists"],path.name
        if got["status"]=="relation":
            check=verify_witness(fixture,got["witness"])
            assert check["recovered_scalar_verified"] and check["k"]==validation["secret_k"]
        # Independent unbatched group pair enumeration for every B<=16.
        if fixture["B"]<=16:
            c=fixture["curve"];p,a=c["p"],c["a"]
            expected=False
            for e0,e1,e2 in itertools.product(*fixture["pools"]):
                u=(e0["x"],e0["y"]);v=(e1["x"],e1["y"]);w=(e2["x"],e2["y"])
                for sj,sk in itertools.product((1,-1),repeat=2):
                    if add(p,a,add(p,a,u,v if sj==1 else neg(p,v)),w if sk==1 else neg(p,w)) is None:
                        expected=True
            assert expected==(got["status"]=="relation")
        rows.append({"id":fixture["id"],"B":fixture["B"],"status":got["status"],
                     "scalar_verified":got["recovery"] is None or got["recovery"]["recovered_scalar_verified"]})
    return rows


def audit_pair_exceptions():
    """Cross-pool equal x, doubling, infinity, and colliding lookup entries.

    These are correctness controls; one alternative representation uses the
    validation logarithm to force an informative coefficient. They are excluded
    from all natural-pool timings and scaling claims.
    """
    path=Path('/workspace/ecdlp_research/prime_fixtures/b008_c0_natural_s0.json')
    f=json.loads(path.read_text());v=json.loads(path.with_suffix('.validation.json').read_text())
    c=f['curve'];p,a,r=c['p'],c['a'],c['r'];k=v['secret_k']
    e=f['pools'][0][0];u=(e['x'],e['y']);u2=add(p,a,u,u);assert u2 is not None
    e2={'x':u2[0],'y':(-u2[1])%p,'alpha':(-2*e['alpha'])%r,'beta':(-2*e['beta'])%r}
    informative=dict(e2,alpha=(e2['alpha']-k)%r,beta=(e2['beta']+1)%r)
    tests=[('doubling_then_infinity_and_x_collision',[[e],[e],[e2,informative]]),
           ('opposites_and_doubling',[[e],[dict(e,y=-e['y']%p,alpha=-e['alpha']%r,beta=-e['beta']%r)],[informative]])]
    out=[]
    for name,pools in tests:
        fixture={'id':name,'B':1,'curve':c,'pools':pools}
        py=solve_pair(fixture);cpp=solve_pair_cpp(fixture)
        assert py['status']==cpp['status'] and py['witness']==cpp['witness']
        assert py['status']=='relation' and py['recovery']['recovered_scalar_verified']
        assert cpp['counters']['exceptional_denominators']>0
        if name.startswith('opposites'):
            assert cpp['counters']['pair_sums_at_infinity']>0
        else:
            assert cpp['counters']['uninformative_relations']>0
        out.append({'name':name,'status':cpp['status'],'counters':cpp['counters'],
                    'validation_secret_used_only_for_correctness_fixture':True})
    return out


def main():
    os.sched_setaffinity(0,{2})
    begin=time.process_time()
    m2=replay_m2()
    rectangular=random_rectangular_audit()
    pools=audit_pair_controls()
    pair_exceptions=audit_pair_exceptions()
    record={"schema":"repo-controls-audit/v1",
       "native_replay":False,"reason":"cargo/rustc unavailable; portable algorithm-level replay only",
       "m2":m2,"rectangular":rectangular,"natural_pair_control":pools,
       "pair_exception_controls":pair_exceptions,
       "cpu_affinity":sorted(os.sched_getaffinity(0)),"cpu_seconds":time.process_time()-begin,
       "source_sha256":{name:hashlib.sha256((ROOT/name).read_bytes()).hexdigest()
                        for name in ("controls.py","check_controls.py","pair_control.cpp")}}
    (ROOT/"correctness_audit.json").write_text(json.dumps(record,indent=2)+"\n")
    print(json.dumps({"m2_targets":len(m2),"rectangular_cases":rectangular["cases"],
                      "natural_fixtures":len(pools),"cpu_seconds":record["cpu_seconds"]}))


if __name__=="__main__":
    main()
